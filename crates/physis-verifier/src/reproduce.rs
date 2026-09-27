//! Host side of independent reproduction (C6.2, spec 021).
//!
//! The checker (`physis-repro-check`) is a separately implemented
//! executable with its own parser and grid-evaluation algorithm. This
//! module is the trusted host around it: it exports the obligation from
//! the *live* challenge, approves one executable by digest, runs it
//! through the R3 worker boundary, parses the verdict independently,
//! and binds the verdict to the requested run, bundle, obligation, and
//! actual executable identity before anything is minted.
//!
//! Minting rule: only [`reproduce`] mints [`Verified<ReproductionRecord>`],
//! and only after the primary artifact rechecks through [`verify`], the
//! approved executable's digest re-verifies at spawn time, and an `agree`
//! verdict binds on run id, obligation hash, and executable digest.
//! [`run_checker`] executes and classifies but never mints: bytes the
//! caller supplied are not the live obligation. Exit code zero or a
//! line containing `agree` is not evidence; the parsed bindings are.
//!
//! [`verify`]: crate::verify

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::Duration;

use physis_core::artifact::ArtifactId;
use physis_core::worker::{
    run_worker, WorkerFile, WorkerLimits, WorkerOutcome, WorkerRequest, WorkerTrust,
};
use physis_proof::expr::Expr;
use physis_proof::{Challenge, UntrustedProof};
use serde::Serialize;

use crate::{verify, CheckerReceipt, ProofReceipt, Verified, VerifyError};

/// Wire schema tag the host exports.
pub const OBLIGATION_SCHEMA: &str = "physis-int-obligation";
/// Wire schema version the host exports.
pub const OBLIGATION_VERSION: u64 = 1;
/// Wire schema tag the host accepts back.
pub const VERDICT_SCHEMA: &str = "physis-int-verdict";
/// Wire schema version the host accepts back.
pub const VERDICT_VERSION: u64 = 1;
/// Checker identity the host accepts verdicts from.
pub const CHECKER_ID: &str = "dec-int-checker";
/// Checker version the host accepts verdicts from.
pub const CHECKER_VERSION: u64 = 1;
/// Default wall time for one checker run, in seconds.
pub const REPRO_WALL_SECS: u64 = 30;
/// Default output cap for one checker run, in bytes.
pub const REPRO_OUTPUT_BYTES: u64 = 1_000_000;
/// Deepest expression tree the host exports. Mirrors the wire cap so a
/// live identity that outgrows the checker is a host refusal, not a
/// shipped `invalid`.
pub const MAX_TREE_DEPTH: usize = 32;
/// Most tree nodes the host exports. Mirrors the wire cap.
pub const MAX_TREE_NODES: usize = 100_000;
/// Most symbols the host exports. Mirrors the wire cap.
pub const MAX_SYMBOLS: usize = 16;

/// Scratch-relative obligation input path.
const OBLIGATION_PATH: &str = "inputs/obligation.json";
/// Scratch-relative verdict output path.
const VERDICT_ARG: &str = "outputs/verdict.json";
/// Collected verdict file name (`collect_outputs` strips the directory).
const VERDICT_NAME: &str = "verdict.json";
/// Environment variable carrying the executable digest to the checker.
const EXE_DIGEST_ENV: &str = "WORKER_EXE_DIGEST";

/// Why the host refused before or during a reproduction run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReproError {
    /// The primary artifact did not recheck. The checker never ran.
    PrimaryFailed(VerifyError),
    /// The live challenge could not be exported as a wire obligation,
    /// or the run binding was ill-formed.
    Export(String),
    /// The executable is not the approved one (unknown path, unreadable
    /// file, digest mismatch, or changed since approval).
    Approval(String),
    /// The worker boundary refused to spawn.
    SpawnRefused(String),
}

impl std::fmt::Display for ReproError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReproError::PrimaryFailed(e) => write!(f, "primary evidence recheck failed: {e}"),
            ReproError::Export(e) => write!(f, "obligation export refused: {e}"),
            ReproError::Approval(e) => write!(f, "checker approval refused: {e}"),
            ReproError::SpawnRefused(e) => write!(f, "worker boundary refused spawn: {e}"),
        }
    }
}

impl std::error::Error for ReproError {}

/// Host-approved checker executable: one file, one digest.
///
/// Approval binds the file bytes the host hashed, not the path string:
/// a copy of the approved bytes at another path is the same approved
/// implementation, while any other program under any name is refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApprovedChecker {
    program: PathBuf,
    exe_digest: String,
}

impl ApprovedChecker {
    /// Approve one executable file against the allowlisted digest (hex
    /// SHA-256 of the reviewed bytes). Reads and hashes the file now;
    /// [`run_checker`] and [`reproduce`] re-hash at spawn time and
    /// refuse a file that changed since approval. The stored program is
    /// canonicalized so the worker's scratch cwd cannot redirect it.
    pub fn approve(program: &Path, expected_exe_digest: &str) -> Result<Self, ReproError> {
        if !program.is_file() {
            return Err(ReproError::Approval(format!(
                "unknown program '{}': not a host-selected binary",
                program.display()
            )));
        }
        let bytes = std::fs::read(program).map_err(|e| {
            ReproError::Approval(format!("cannot read '{}': {e}", program.display()))
        })?;
        let digest = ArtifactId::of(&bytes).to_hex();
        if digest != expected_exe_digest {
            return Err(ReproError::Approval(
                "executable digest does not match the allowlisted checker digest".into(),
            ));
        }
        let canonical = std::fs::canonicalize(program).map_err(|e| {
            ReproError::Approval(format!("cannot resolve '{}': {e}", program.display()))
        })?;
        Ok(Self {
            program: canonical,
            exe_digest: digest,
        })
    }

    /// Approved program path.
    pub fn program(&self) -> &Path {
        &self.program
    }

    /// Approved hex SHA-256 digest of the executable bytes.
    pub fn exe_digest(&self) -> &str {
        &self.exe_digest
    }
}

/// Limits for one checker run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReproLimits {
    /// Kill the checker after this many seconds. Must be positive.
    pub wall_secs: u64,
    /// Byte cap per captured stream and for collected output files.
    pub output_bytes: u64,
}

impl Default for ReproLimits {
    fn default() -> Self {
        Self {
            wall_secs: REPRO_WALL_SECS,
            output_bytes: REPRO_OUTPUT_BYTES,
        }
    }
}

/// Classified outcome of one executed checker run. The spec-021
/// dispositions, plus `execution-failed`, which exists only here.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReproDisposition {
    /// Independent check of this exact obligation succeeded.
    Agree,
    /// Well-formed supported obligation was rejected.
    Disagree,
    /// Checker does not implement this schema or obligation.
    Unsupported,
    /// Malformed/tampered bundle or mismatched identity.
    Invalid,
    /// Time, memory, or output cap reached. Unresolved, never disproof.
    ResourceExhausted,
    /// Crash, launch failure, missing output, or invalid worker
    /// response. Operational failure, never a scientific verdict.
    ExecutionFailed,
}

impl ReproDisposition {
    /// Wire spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            ReproDisposition::Agree => "agree",
            ReproDisposition::Disagree => "disagree",
            ReproDisposition::Unsupported => "unsupported",
            ReproDisposition::Invalid => "invalid",
            ReproDisposition::ResourceExhausted => "resource-exhausted",
            ReproDisposition::ExecutionFailed => "execution-failed",
        }
    }
}

/// Sealed reproduction record: an `agree` verdict bound to the live
/// statement, the exact obligation bytes, and the executed bytes.
///
/// Private fields, [`Serialize`] for logging and display, and **no
/// `Deserialize` impl**: imported JSON is raw data, never authority.
/// Only [`reproduce`] mints one, wrapped in [`Verified`].
///
/// ```compile_fail
/// use physis_verifier::reproduce::ReproductionRecord;
/// let _ = ReproductionRecord {
///     claim_id: String::new(),
/// };
/// ```
///
/// ```compile_fail
/// fn needs_deserialize<'de, T: serde::Deserialize<'de>>() {}
/// fn _blocked() {
///     needs_deserialize::<physis_verifier::reproduce::ReproductionRecord>();
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ReproductionRecord {
    claim_id: String,
    statement_hash: ArtifactId,
    challenge_hash: ArtifactId,
    assumption_hash: ArtifactId,
    obligation_sha256: String,
    exe_digest: String,
    checker_id: String,
    checker_version: u64,
    run_id: String,
    evaluated_points: u64,
    grid_degree: Option<u64>,
}

impl ReproductionRecord {
    /// Claim slug the live challenge was generated from.
    pub fn claim_id(&self) -> &str {
        &self.claim_id
    }

    /// Live statement hash this reproduction is bound to.
    pub fn statement_hash(&self) -> ArtifactId {
        self.statement_hash
    }

    /// Live challenge hash this reproduction is bound to.
    pub fn challenge_hash(&self) -> ArtifactId {
        self.challenge_hash
    }

    /// Live assumption-set hash in force for this reproduction.
    pub fn assumption_hash(&self) -> ArtifactId {
        self.assumption_hash
    }

    /// Hex SHA-256 of the exact obligation bytes the checker read.
    pub fn obligation_sha256(&self) -> &str {
        &self.obligation_sha256
    }

    /// Hex SHA-256 of the executed checker bytes.
    pub fn exe_digest(&self) -> &str {
        &self.exe_digest
    }

    /// Checker identity echoed in the bound verdict.
    pub fn checker_id(&self) -> &str {
        &self.checker_id
    }

    /// Checker version echoed in the bound verdict.
    pub fn checker_version(&self) -> u64 {
        self.checker_version
    }

    /// Run id the host issued for this attempt.
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    /// Grid points the checker evaluated.
    pub fn evaluated_points(&self) -> u64 {
        self.evaluated_points
    }

    /// Syntactic degree the checker used, when reported.
    pub fn grid_degree(&self) -> Option<u64> {
        self.grid_degree
    }
}

/// One executed reproduction run: classified disposition, human detail,
/// and the minted record when every binding held. Raw inspectable
/// history — including failures — not authority by itself.
///
/// [`Serialize`] without [`serde::Deserialize`]: attempts can be logged
/// to the artifact graph but never imported as evidence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ReproductionAttempt {
    disposition: ReproDisposition,
    detail: String,
    verified: Option<Verified<ReproductionRecord>>,
    obligation_sha256: String,
    run_id: String,
    exe_digest: String,
    evaluated_points: u64,
    grid_degree: Option<u64>,
}

impl ReproductionAttempt {
    /// Classified outcome.
    pub fn disposition(&self) -> ReproDisposition {
        self.disposition
    }

    /// Human diagnostic (checker detail or host refusal note).
    pub fn detail(&self) -> &str {
        &self.detail
    }

    /// Minted record. `Some` only on a fully bound `agree` through
    /// [`reproduce`]; [`run_checker`] never mints.
    pub fn verified(&self) -> Option<&Verified<ReproductionRecord>> {
        self.verified.as_ref()
    }

    /// Hex SHA-256 of the obligation bytes the host wrote.
    pub fn obligation_sha256(&self) -> &str {
        &self.obligation_sha256
    }

    /// Run id the host issued.
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    /// Hex SHA-256 of the executed bytes.
    pub fn exe_digest(&self) -> &str {
        &self.exe_digest
    }

    /// Grid points evaluated, when the checker reported them.
    pub fn evaluated_points(&self) -> u64 {
        self.evaluated_points
    }

    /// Syntactic degree used, when the checker reported it.
    pub fn grid_degree(&self) -> Option<u64> {
        self.grid_degree
    }
}

/// Build wire-format obligation bytes from the live challenge identity.
///
/// The tree comes from the challenge the host holds, never from
/// proposer bytes. Refuses an inconsistent challenge, a missing exact
/// identity, an empty run id, constants the wire integers cannot spell
/// exactly, ill-formed symbol names, and trees beyond the wire caps.
/// Keys are lexicographic with minimal separators and no trailing
/// newline; the checker hashes the bytes it reads.
pub fn export_obligation(challenge: &Challenge, run_id: &str) -> Result<Vec<u8>, ReproError> {
    if run_id.is_empty() {
        return Err(ReproError::Export("run id must be non-empty".into()));
    }
    if !challenge.hash_is_consistent() {
        return Err(ReproError::Export(
            "challenge hash does not match canonical bytes".into(),
        ));
    }
    let identity = challenge
        .identity()
        .ok_or_else(|| ReproError::Export("no exact identity catalogued for this claim".into()))?;
    let mut symbols = BTreeSet::new();
    let mut nodes = 0usize;
    let tree = export_tree(identity, &mut symbols, &mut nodes, 1)?;
    if symbols.len() > MAX_SYMBOLS {
        return Err(ReproError::Export(format!(
            "identity uses {} symbols, cap is {MAX_SYMBOLS}",
            symbols.len()
        )));
    }
    let mut obj = serde_json::Map::new();
    obj.insert("expect".into(), serde_json::json!("zero"));
    obj.insert("run_id".into(), serde_json::json!(run_id));
    obj.insert("schema".into(), serde_json::json!(OBLIGATION_SCHEMA));
    obj.insert(
        "symbols".into(),
        serde_json::Value::Array(symbols.into_iter().map(serde_json::Value::String).collect()),
    );
    obj.insert("tree".into(), tree);
    obj.insert("version".into(), serde_json::json!(OBLIGATION_VERSION));
    serde_json::to_string(&serde_json::Value::Object(obj))
        .map(String::into_bytes)
        .map_err(|e| ReproError::Export(format!("obligation encoding failed: {e}")))
}

/// Serialize one identity node, enforcing the wire's integer, name, and
/// size caps. `depth` counts tree levels along this path from 1.
fn export_tree(
    expr: &Expr,
    symbols: &mut BTreeSet<String>,
    nodes: &mut usize,
    depth: usize,
) -> Result<serde_json::Value, ReproError> {
    if depth > MAX_TREE_DEPTH {
        return Err(ReproError::Export(format!(
            "identity exceeds {MAX_TREE_DEPTH} tree levels"
        )));
    }
    *nodes += 1;
    if *nodes > MAX_TREE_NODES {
        return Err(ReproError::Export(format!(
            "identity exceeds {MAX_TREE_NODES} nodes"
        )));
    }
    match expr {
        Expr::Var(name) => {
            if !is_wire_name(name) {
                return Err(ReproError::Export(format!(
                    "symbol '{name}' is not a wire name"
                )));
            }
            symbols.insert(name.clone());
            Ok(serde_json::json!({ "var": name }))
        }
        Expr::Const(n) => {
            let literal = if let Ok(v) = i64::try_from(*n) {
                serde_json::json!(v)
            } else if let Ok(v) = u64::try_from(*n) {
                serde_json::json!(v)
            } else {
                return Err(ReproError::Export(format!(
                    "constant {n} is outside the wire integer range"
                )));
            };
            Ok(serde_json::json!({ "const": literal }))
        }
        Expr::Add(a, b) => export_op("add", [&**a, &**b], symbols, nodes, depth),
        Expr::Sub(a, b) => export_op("sub", [&**a, &**b], symbols, nodes, depth),
        Expr::Mul(a, b) => export_op("mul", [&**a, &**b], symbols, nodes, depth),
        Expr::Pow(base, exp) => {
            let mut obj = serde_json::Map::new();
            obj.insert(
                "args".into(),
                serde_json::Value::Array(vec![export_tree(base, symbols, nodes, depth + 1)?]),
            );
            obj.insert("exp".into(), serde_json::json!(*exp));
            obj.insert("op".into(), serde_json::json!("pow"));
            Ok(serde_json::Value::Object(obj))
        }
    }
}

/// Serialize a two-argument operator node.
fn export_op(
    op: &str,
    args: [&Expr; 2],
    symbols: &mut BTreeSet<String>,
    nodes: &mut usize,
    depth: usize,
) -> Result<serde_json::Value, ReproError> {
    let mut rendered = Vec::with_capacity(2);
    for arg in args {
        rendered.push(export_tree(arg, symbols, nodes, depth + 1)?);
    }
    let mut obj = serde_json::Map::new();
    obj.insert("args".into(), serde_json::Value::Array(rendered));
    obj.insert("op".into(), serde_json::json!(op));
    Ok(serde_json::Value::Object(obj))
}

/// Wire symbol names: `[A-Za-z_][A-Za-z0-9_]*`, non-empty.
fn is_wire_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Execute the approved checker on caller-supplied obligation bytes and
/// classify the outcome. Binds the verdict to `run_id`, the exact bytes
/// written, and the executed digest; anything that does not bind is
/// [`ReproDisposition::ExecutionFailed`]. Never mints: without the
/// live-challenge export and primary recheck of [`reproduce`] there is
/// no statement binding to seal.
pub fn run_checker(
    checker: &ApprovedChecker,
    obligation_bytes: &[u8],
    run_id: &str,
    limits: &ReproLimits,
) -> Result<ReproductionAttempt, ReproError> {
    execute(checker, obligation_bytes, run_id, limits)
}

/// Full sealed pipeline: recheck the primary artifact through [`verify`],
/// export the obligation from the live challenge, execute the approved
/// checker, and bind the verdict. Mints [`Verified<ReproductionRecord>`]
/// only on a fully bound `agree`. Every executed run — including
/// disagreements and failures — returns an inspectable
/// [`ReproductionAttempt`]; `Err` means the checker never ran.
pub fn reproduce(
    challenge: &Challenge,
    artifact: &UntrustedProof,
    checker: &ApprovedChecker,
    run_id: &str,
    limits: &ReproLimits,
) -> Result<ReproductionAttempt, ReproError> {
    let primary = verify(challenge, artifact).map_err(ReproError::PrimaryFailed)?;
    let obligation_bytes = export_obligation(challenge, run_id)?;
    let mut attempt = execute(checker, &obligation_bytes, run_id, limits)?;
    if attempt.disposition == ReproDisposition::Agree {
        let primary_receipt = primary.receipt();
        let record = ReproductionRecord {
            claim_id: challenge.claim_id().to_string(),
            statement_hash: challenge.statement_hash(),
            challenge_hash: challenge.challenge_hash(),
            assumption_hash: challenge.assumption_hash(),
            obligation_sha256: attempt.obligation_sha256.clone(),
            exe_digest: checker.exe_digest().to_string(),
            checker_id: CHECKER_ID.to_string(),
            checker_version: CHECKER_VERSION,
            run_id: run_id.to_string(),
            evaluated_points: attempt.evaluated_points,
            grid_degree: attempt.grid_degree,
        };
        let receipt = ProofReceipt {
            claim_id: challenge.claim_id().to_string(),
            statement_hash: challenge.statement_hash(),
            assumption_hash: challenge.assumption_hash(),
            challenge_hash: challenge.challenge_hash(),
            proof_artifact_hash: ArtifactId::of(&obligation_bytes),
            formal_backend: primary_receipt.formal_backend.clone(),
            formal_backend_version: primary_receipt.formal_backend_version.clone(),
            library_lock_hash: ArtifactId::of(checker.exe_digest().as_bytes()),
            primary_checker: primary_receipt.primary_checker.clone(),
            secondary_checker: CheckerReceipt::ran(CHECKER_ID, "1", true),
            axioms_used: primary_receipt.axioms_used.clone(),
        };
        attempt.verified = Some(Verified::mint(record, receipt));
    }
    Ok(attempt)
}

/// Shared execution path: re-verify the executable digest, spawn through
/// the worker boundary, and classify. Never mints.
fn execute(
    checker: &ApprovedChecker,
    obligation_bytes: &[u8],
    run_id: &str,
    limits: &ReproLimits,
) -> Result<ReproductionAttempt, ReproError> {
    if run_id.is_empty() {
        return Err(ReproError::Export("run id must be non-empty".into()));
    }
    if limits.wall_secs == 0 {
        return Err(ReproError::SpawnRefused(
            "reproduction wall time must be positive".into(),
        ));
    }
    let program_bytes = std::fs::read(checker.program()).map_err(|e| {
        ReproError::Approval(format!(
            "cannot re-read approved program '{}': {e}",
            checker.program().display()
        ))
    })?;
    if ArtifactId::of(&program_bytes).to_hex() != checker.exe_digest() {
        return Err(ReproError::Approval(
            "executable changed since approval; refusing to run".into(),
        ));
    }
    let obligation_sha256 = ArtifactId::of(obligation_bytes).to_hex();
    let outcome = run_worker(&WorkerRequest {
        program: checker.program().to_path_buf(),
        args: vec![OBLIGATION_PATH.into(), VERDICT_ARG.into()],
        files: vec![WorkerFile {
            path: OBLIGATION_PATH.into(),
            bytes: obligation_bytes.to_vec(),
        }],
        env_extra: vec![(EXE_DIGEST_ENV.into(), checker.exe_digest().to_string())],
        trust: WorkerTrust::ReviewedLocal,
        limits: WorkerLimits {
            wall_time: Duration::from_secs(limits.wall_secs),
            output_bytes: limits.output_bytes,
            memory_mb: None,
        },
    })
    .map_err(ReproError::SpawnRefused)?;
    Ok(classify(
        &outcome,
        run_id,
        &obligation_sha256,
        checker.exe_digest(),
    ))
}

/// Bindings the host re-verifies against the worker's response.
struct Bindings<'a> {
    run_id: &'a str,
    obligation_sha256: &'a str,
    exe_digest: &'a str,
}

/// Classify one worker outcome. Timeouts, output caps, and checker-side
/// resource verdicts are unresolved, never disproof; everything that
/// does not bind to this run is `execution-failed`, never a verdict.
fn classify(
    outcome: &WorkerOutcome,
    run_id: &str,
    obligation_sha256: &str,
    exe_digest: &str,
) -> ReproductionAttempt {
    let bindings = Bindings {
        run_id,
        obligation_sha256,
        exe_digest,
    };
    let (disposition, detail, points, degree) = match outcome {
        WorkerOutcome::TimedOut { .. } => (
            ReproDisposition::ResourceExhausted,
            "checker exceeded its wall time; unresolved".to_string(),
            0,
            None,
        ),
        WorkerOutcome::OutputLimit { .. } => (
            ReproDisposition::ResourceExhausted,
            "checker exceeded its output cap; unresolved".to_string(),
            0,
            None,
        ),
        WorkerOutcome::InputsMutated { file } => (
            ReproDisposition::ExecutionFailed,
            format!("checker modified its inputs ({file}); result refused"),
            0,
            None,
        ),
        WorkerOutcome::Completed {
            success: false,
            code,
            stderr,
            ..
        } => (
            ReproDisposition::ExecutionFailed,
            format!(
                "checker exited code {}: {}",
                code.map_or("?".into(), |c| c.to_string()),
                head_lossy(stderr, 300)
            ),
            0,
            None,
        ),
        WorkerOutcome::Completed {
            success: true,
            files,
            ..
        } => classify_verdict(files, &bindings),
    };
    ReproductionAttempt {
        disposition,
        detail,
        verified: None,
        obligation_sha256: obligation_sha256.to_string(),
        run_id: run_id.to_string(),
        exe_digest: exe_digest.to_string(),
        evaluated_points: points,
        grid_degree: degree,
    }
}

/// Parse and bind the collected verdict file. Any violation — missing
/// file, bad JSON, duplicate keys, unknown keys, schema mismatch, or a
/// binding that does not match this run — is `execution-failed`.
fn classify_verdict(
    files: &[WorkerFile],
    bindings: &Bindings<'_>,
) -> (ReproDisposition, String, u64, Option<u64>) {
    let fail = |detail: String| (ReproDisposition::ExecutionFailed, detail, 0, None);
    let Some(verdict) = files.iter().find(|f| f.path == VERDICT_NAME) else {
        return fail("checker wrote no verdict file".into());
    };
    let text = match std::str::from_utf8(&verdict.bytes) {
        Ok(t) => t,
        Err(_) => return fail("verdict file is not utf-8".into()),
    };
    if let Err(e) = reject_duplicate_keys(text) {
        return fail(format!("verdict rejected: {e}"));
    }
    let value: serde_json::Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => return fail(format!("verdict is not json: {e}")),
    };
    let obj = match value.as_object() {
        Some(o) => o,
        None => return fail("verdict is not a json object".into()),
    };
    const KNOWN: &[&str] = &[
        "schema",
        "version",
        "run_id",
        "obligation_sha256",
        "exe_sha256",
        "checker_id",
        "checker_version",
        "disposition",
        "detail",
        "evaluated_points",
        "grid_degree",
        "witness",
    ];
    for key in obj.keys() {
        if !KNOWN.contains(&key.as_str()) {
            return fail(format!("verdict has unknown key '{key}'"));
        }
    }
    let get = |key: &str| {
        obj.get(key)
            .ok_or_else(|| format!("verdict is missing '{key}'"))
    };
    let as_str = |key: &str| match get(key)?.as_str() {
        Some(s) => Ok(s),
        None => Err(format!("verdict '{key}' is not a string")),
    };
    let as_u64 = |key: &str| match get(key)?.as_u64() {
        Some(n) => Ok(n),
        None => Err(format!("verdict '{key}' is not a non-negative integer")),
    };
    let checked = (|| -> Result<(ReproDisposition, String, u64, Option<u64>), String> {
        if as_str("schema")? != VERDICT_SCHEMA {
            return Err("verdict schema is not physis-int-verdict".into());
        }
        if as_u64("version")? != VERDICT_VERSION {
            return Err(format!("verdict version is not {VERDICT_VERSION}"));
        }
        if as_str("checker_id")? != CHECKER_ID {
            return Err("verdict is not from the approved checker identity".into());
        }
        if as_u64("checker_version")? != CHECKER_VERSION {
            return Err(format!("verdict checker version is not {CHECKER_VERSION}"));
        }
        if as_str("run_id")? != bindings.run_id {
            return Err("verdict run id does not match this run".into());
        }
        if as_str("obligation_sha256")? != bindings.obligation_sha256 {
            return Err("verdict obligation hash does not match the bytes written".into());
        }
        if as_str("exe_sha256")? != bindings.exe_digest {
            return Err("verdict executable digest does not match the executed bytes".into());
        }
        let disposition = match as_str("disposition")? {
            "agree" => ReproDisposition::Agree,
            "disagree" => ReproDisposition::Disagree,
            "unsupported" => ReproDisposition::Unsupported,
            "invalid" => ReproDisposition::Invalid,
            "resource-exhausted" => ReproDisposition::ResourceExhausted,
            other => return Err(format!("verdict disposition '{other}' is unknown")),
        };
        let detail = as_str("detail")?.to_string();
        let points = as_u64("evaluated_points")?;
        let degree = match get("grid_degree")? {
            serde_json::Value::Null => None,
            v => Some(v.as_u64().ok_or_else(|| {
                "verdict 'grid_degree' is not a non-negative integer or null".to_string()
            })?),
        };
        match get("witness")? {
            serde_json::Value::Null => {}
            serde_json::Value::Object(w) => {
                for (k, v) in w {
                    if !is_wire_name(k) || !(v.is_i64() || v.is_u64()) {
                        return Err("verdict witness is not a name-to-integer map".into());
                    }
                }
            }
            _ => return Err("verdict witness is not an object or null".into()),
        }
        Ok((disposition, detail, points, degree))
    })();
    match checked {
        Ok(ok) => ok,
        Err(e) => fail(format!("verdict rejected: {e}")),
    }
}

/// First 300 characters of a byte stream for diagnostics.
fn head_lossy(bytes: &[u8], max: usize) -> String {
    let text = String::from_utf8_lossy(bytes);
    let mut out: String = text.chars().take(max).collect();
    if text.chars().count() > max {
        out.push('…');
    }
    out
}

/// Reject any JSON object with a duplicate key. String-aware scan over
/// objects; arrays share the stack but contribute no keys.
fn reject_duplicate_keys(text: &str) -> Result<(), String> {
    let bytes = text.as_bytes();
    let mut stack: Vec<Option<BTreeSet<String>>> = Vec::new();
    let mut i = 0;
    let mut in_string = false;
    let mut escaped = false;
    let mut current = String::new();
    while i < bytes.len() {
        let c = bytes[i];
        if in_string {
            if escaped {
                escaped = false;
                current.push(c as char);
            } else if c == b'\\' {
                escaped = true;
                current.push(c as char);
            } else if c == b'"' {
                in_string = false;
            } else {
                current.push(c as char);
            }
            i += 1;
            continue;
        }
        match c {
            b'"' => {
                in_string = true;
                current.clear();
            }
            b'{' => stack.push(Some(BTreeSet::new())),
            b'[' => stack.push(None),
            b'}' | b']' => {
                stack.pop();
            }
            b':' => {
                if let Some(Some(keys)) = stack.last_mut() {
                    if !keys.insert(std::mem::take(&mut current)) {
                        return Err("duplicate object key".into());
                    }
                }
                current.clear();
            }
            b',' | b' ' | b'\t' | b'\n' | b'\r' => {
                current.clear();
            }
            _ => {
                current.clear();
            }
        }
        i += 1;
    }
    Ok(())
}

/// The lab-wide store of minted reproduction records. Inserts only
/// through [`ReproductionStore::record`]; lookups bind the live
/// statement hash, never the slug alone.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReproductionStore {
    records: Vec<ReproductionRecord>,
}

impl ReproductionStore {
    /// No records.
    pub fn empty() -> Self {
        Self::default()
    }

    /// True when no reproduction has been recorded.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Number of recorded reproductions.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Record a minted value. The only way a record enters the store.
    pub fn record(&mut self, verified: &Verified<ReproductionRecord>) {
        self.records.push(verified.artifact().clone());
    }

    /// Lookup by live statement hash. A changed statement keeps the slug
    /// and must miss here: that is the stale-binding refusal.
    pub fn by_statement(&self, statement_hash: ArtifactId) -> Option<&ReproductionRecord> {
        self.records
            .iter()
            .find(|r| r.statement_hash == statement_hash)
    }

    /// Lookup by claim slug (last record wins). Display convenience, not
    /// a binding: trust reads [`Self::by_statement`].
    pub fn by_claim(&self, claim_id: &str) -> Option<&ReproductionRecord> {
        self.records.iter().rev().find(|r| r.claim_id == claim_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn completed(files: Vec<WorkerFile>) -> WorkerOutcome {
        WorkerOutcome::Completed {
            success: true,
            code: Some(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
            files,
        }
    }

    fn verdict_file(body: &serde_json::Value) -> WorkerFile {
        WorkerFile {
            path: VERDICT_NAME.into(),
            bytes: serde_json::to_string(body).unwrap().into_bytes(),
        }
    }

    fn good_verdict() -> serde_json::Value {
        serde_json::json!({
            "schema": VERDICT_SCHEMA,
            "version": VERDICT_VERSION,
            "run_id": "run-1",
            "obligation_sha256": "0".repeat(64),
            "exe_sha256": "1".repeat(64),
            "checker_id": CHECKER_ID,
            "checker_version": CHECKER_VERSION,
            "disposition": "agree",
            "detail": "all 8 grid points vanish (degree 1)",
            "evaluated_points": 8,
            "grid_degree": 1,
            "witness": null,
        })
    }

    fn classify_body(body: &serde_json::Value) -> ReproductionAttempt {
        classify(
            &completed(vec![verdict_file(body)]),
            "run-1",
            &"0".repeat(64),
            &"1".repeat(64),
        )
    }

    #[test]
    fn bound_agree_classifies() {
        let attempt = classify_body(&good_verdict());
        assert_eq!(attempt.disposition(), ReproDisposition::Agree);
        assert_eq!(attempt.evaluated_points(), 8);
        assert_eq!(attempt.grid_degree(), Some(1));
        assert!(attempt.verified().is_none());
    }

    #[test]
    fn checker_resource_verdict_is_unresolved() {
        let mut v = good_verdict();
        v["disposition"] = serde_json::json!("resource-exhausted");
        v["detail"] = serde_json::json!("grid cap reached before evaluation");
        v["evaluated_points"] = serde_json::json!(0);
        v["grid_degree"] = serde_json::Value::Null;
        let attempt = classify_body(&v);
        assert_eq!(attempt.disposition(), ReproDisposition::ResourceExhausted);
        assert!(attempt.verified().is_none());
    }

    #[test]
    fn wrong_run_id_is_execution_failed() {
        let mut v = good_verdict();
        v["run_id"] = serde_json::json!("run-2");
        let attempt = classify_body(&v);
        assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
        assert!(attempt.detail().contains("run id"), "{}", attempt.detail());
    }

    #[test]
    fn wrong_obligation_hash_is_execution_failed() {
        let mut v = good_verdict();
        v["obligation_sha256"] = serde_json::json!("2".repeat(64));
        let attempt = classify_body(&v);
        assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
        assert!(
            attempt.detail().contains("obligation hash"),
            "{}",
            attempt.detail()
        );
    }

    #[test]
    fn wrong_exe_digest_is_execution_failed() {
        let mut v = good_verdict();
        v["exe_sha256"] = serde_json::json!("");
        let attempt = classify_body(&v);
        assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
        assert!(
            attempt.detail().contains("executable digest"),
            "{}",
            attempt.detail()
        );
    }

    #[test]
    fn unknown_schema_version_is_execution_failed() {
        let mut v = good_verdict();
        v["version"] = serde_json::json!(99);
        let attempt = classify_body(&v);
        assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
    }

    #[test]
    fn wrong_checker_identity_is_execution_failed() {
        let mut v = good_verdict();
        v["checker_id"] = serde_json::json!("some-other-checker");
        let attempt = classify_body(&v);
        assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
    }

    #[test]
    fn unknown_disposition_is_execution_failed() {
        let mut v = good_verdict();
        v["disposition"] = serde_json::json!("probably-fine");
        let attempt = classify_body(&v);
        assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
    }

    #[test]
    fn unknown_key_is_execution_failed() {
        let mut v = good_verdict();
        v["p4"] = serde_json::json!(true);
        let attempt = classify_body(&v);
        assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
        assert!(
            attempt.detail().contains("unknown key"),
            "{}",
            attempt.detail()
        );
    }

    #[test]
    fn duplicate_key_is_execution_failed() {
        let raw = r#"{"schema":"physis-int-verdict","version":1,"version":1,"run_id":"run-1","obligation_sha256":"00","exe_sha256":"11","checker_id":"dec-int-checker","checker_version":1,"disposition":"agree","detail":"x","evaluated_points":0,"grid_degree":null,"witness":null}"#;
        let attempt = classify(
            &completed(vec![WorkerFile {
                path: VERDICT_NAME.into(),
                bytes: raw.as_bytes().to_vec(),
            }]),
            "run-1",
            "00",
            "11",
        );
        assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
        assert!(
            attempt.detail().contains("duplicate"),
            "{}",
            attempt.detail()
        );
    }

    #[test]
    fn malformed_verdict_is_execution_failed() {
        let attempt = classify(
            &completed(vec![WorkerFile {
                path: VERDICT_NAME.into(),
                bytes: b"{nope".to_vec(),
            }]),
            "run-1",
            &"0".repeat(64),
            &"1".repeat(64),
        );
        assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
    }

    #[test]
    fn missing_verdict_is_execution_failed() {
        let attempt = classify(
            &completed(vec![]),
            "run-1",
            &"0".repeat(64),
            &"1".repeat(64),
        );
        assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
        assert!(
            attempt.detail().contains("no verdict"),
            "{}",
            attempt.detail()
        );
    }

    #[test]
    fn timeout_is_unresolved_not_disproof() {
        let attempt = classify(
            &WorkerOutcome::TimedOut {
                stdout: Vec::new(),
                stderr: Vec::new(),
            },
            "run-1",
            &"0".repeat(64),
            &"1".repeat(64),
        );
        assert_eq!(attempt.disposition(), ReproDisposition::ResourceExhausted);
        assert!(
            attempt.detail().contains("unresolved"),
            "{}",
            attempt.detail()
        );
        assert!(attempt.verified().is_none());
    }

    #[test]
    fn output_cap_is_unresolved_not_disproof() {
        let attempt = classify(
            &WorkerOutcome::OutputLimit {
                stdout: Vec::new(),
                stderr: Vec::new(),
            },
            "run-1",
            &"0".repeat(64),
            &"1".repeat(64),
        );
        assert_eq!(attempt.disposition(), ReproDisposition::ResourceExhausted);
    }

    #[test]
    fn mutated_inputs_refuse_the_result() {
        let attempt = classify(
            &WorkerOutcome::InputsMutated {
                file: OBLIGATION_PATH.into(),
            },
            "run-1",
            &"0".repeat(64),
            &"1".repeat(64),
        );
        assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
        assert!(
            attempt.detail().contains("modified"),
            "{}",
            attempt.detail()
        );
    }

    #[test]
    fn nonzero_exit_is_execution_failed() {
        let attempt = classify(
            &WorkerOutcome::Completed {
                success: false,
                code: Some(3),
                stdout: Vec::new(),
                stderr: b"boom".to_vec(),
                files: Vec::new(),
            },
            "run-1",
            &"0".repeat(64),
            &"1".repeat(64),
        );
        assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
        assert!(attempt.detail().contains('3'), "{}", attempt.detail());
    }

    #[test]
    fn export_names_symbols_and_caps() {
        // d2 uses exactly a, b, c.
        let claim = physis_proof::lookup("dec.d-squared-zero")
            .unwrap()
            .lab_claim();
        let challenge = Challenge::generate(&physis_core::formal::FormalClaim::from_claim(&claim));
        let bytes = export_obligation(&challenge, "run-x").unwrap();
        let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(v["symbols"], serde_json::json!(["a", "b", "c"]));
    }
}
