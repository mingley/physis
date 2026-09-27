//! Versioned campaign manifests and dry-run planning (R1, spec 022).
//!
//! A manifest is typed, versioned data — not a shell script. [`plan_text`]
//! parses a manifest, refuses schema violations (unknown version or fields,
//! duplicate keys, unbounded budgets, unregistered operations, missing
//! identities), validates live references against the lab (theories, claim
//! ids, knob domains, structural-mutation labels, role permissions), and
//! renders the dry-run plan. Planning executes nothing and grants no
//! authority; dispatching a step is R4.

use std::collections::{BTreeMap, HashSet};

use physis_core::judgment::ParameterOrigin;
use physis_core::knob::{KnobDomain, KnobValue};
use physis_core::ArtifactId;
use serde::{Deserialize, Serialize};

use crate::lab::Lab;
use crate::protocol::Command;
use crate::role::Role;

/// Manifest schema tag. Anything else is refused before any other check.
pub const MANIFEST_SCHEMA: &str = "physis-campaign-manifest";
/// The only manifest version R1 understands.
pub const MANIFEST_VERSION: u32 = 1;

/// Minimal valid manifest, kept in sync with [`Manifest`] by
/// `accepts_minimal_valid_manifest`. Also serves as user documentation.
pub const EXAMPLE_MANIFEST: &str = r#"{
  "schema": "physis-campaign-manifest",
  "version": 1,
  "objective": {
    "question": "does total_dim 10 keep the critical-dimension claim holding",
    "theories": ["type-iib"],
    "statements": ["consistency.critical-dimension"],
    "observable": "critical-dimension verdict",
    "units": "verdict-kind",
    "domain": "total_dim in [2, 32]",
    "success": "critical-dimension holds at total_dim 10",
    "failure": "critical-dimension fails at total_dim 10",
    "inconclusive": "verdict is undecidable or inapplicable"
  },
  "baseline": {
    "source_revision": "working-tree",
    "theory_package": "type-iib",
    "knobs": [{"theory": "type-iib", "knob": "total_dim", "value": "10"}],
    "assumptions": ["live lab definitions"],
    "constants": ["c"],
    "datasets": [],
    "evidence": []
  },
  "candidate_space": {
    "operations": ["hypothesize"],
    "parameter_ranges": [{"theory": "type-iib", "knob": "total_dim", "min": 2.0, "max": 32.0}],
    "structural_mutations": [],
    "max_candidates": 8,
    "seed": 7
  },
  "evaluation": {
    "obligations": ["consistency.critical-dimension"],
    "numeric_tolerance": 0.000001,
    "tolerance_justification": "verdict flips are exact; tolerance is a formality",
    "comparison": "diff verdicts against the retained baseline",
    "controls": ["unmutated baseline"],
    "evidence_required": ["verdict diffs"]
  },
  "data": {
    "sources": [{"id": "none", "locator": "synthetic dry run", "license": "n/a"}],
    "calibration": [],
    "evaluation_inputs": [],
    "uncertainty": "no empirical inputs",
    "access_approval": "none-required: synthetic dry run",
    "adaptive_use": []
  },
  "authority": {"hypothesize": "explorer"},
  "budget": {
    "actions": 4,
    "candidates": 8,
    "attempts": 8,
    "wall_time_secs": 60,
    "memory_mb": 512,
    "output_bytes": 65536,
    "external_cost": 0
  },
  "stop": {"no_progress_limit": 3, "human_pause": false}
}"#;

/// Closed set of operations a campaign may dispatch. Anything else is a
/// schema refusal, never a shell-out.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CampaignOp {
    /// Request a dual-check mint through the verifier path.
    Prove,
    /// Cite a registered source for a missing dataset.
    Cite,
    /// Bounded numeric enclosure without trust upgrade.
    Enclose,
    /// Emit an untrusted encoding of a catalog identity.
    Formalize,
    /// Bounded fork-only hypothesis search (diagnostic).
    Hypothesize,
}

/// Typed research objective (spec 022 manifest row 1).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Objective {
    /// Research question in one sentence.
    pub question: String,
    /// Target theory ids (lab registry).
    pub theories: Vec<String>,
    /// Target statement identities (claim ids).
    pub statements: Vec<String>,
    /// What is observed.
    pub observable: String,
    /// Units of the observable.
    pub units: String,
    /// Reachable domain of the observable.
    pub domain: String,
    /// Predeclared success condition.
    pub success: String,
    /// Predeclared failure condition.
    pub failure: String,
    /// Predeclared inconclusive condition.
    pub inconclusive: String,
}

/// One pinned knob value in the campaign baseline.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KnobPin {
    /// Theory id.
    pub theory: String,
    /// Knob name.
    pub knob: String,
    /// Raw value, parsed against the live knob domain.
    pub value: String,
}

/// Baseline the campaign is diffed against (spec 022 manifest row 2).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Baseline {
    /// Source revision the baseline was taken from.
    pub source_revision: String,
    /// Primary theory package id.
    pub theory_package: String,
    /// Pinned knob state.
    pub knobs: Vec<KnobPin>,
    /// Named assumptions.
    pub assumptions: Vec<String>,
    /// Constant identities.
    pub constants: Vec<String>,
    /// Dataset identities.
    pub datasets: Vec<String>,
    /// Prior evidence references.
    pub evidence: Vec<String>,
}

/// Searchable range over one chosen/fitted knob.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParamRange {
    /// Theory id.
    pub theory: String,
    /// Knob name (chosen/fitted only; measured inputs stay frozen).
    pub knob: String,
    /// Inclusive lower bound.
    pub min: f64,
    /// Inclusive upper bound.
    pub max: f64,
}

/// One permitted structural mutation, by live label.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructMutation {
    /// Theory id.
    pub theory: String,
    /// Mutation label from `Theory::structural_mutations`.
    pub label: String,
}

/// Bounded candidate space (spec 022 manifest row 3).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateSpace {
    /// Registered operations, no duplicates.
    pub operations: Vec<CampaignOp>,
    /// Chosen/fitted parameter ranges within live domains.
    pub parameter_ranges: Vec<ParamRange>,
    /// Permitted structural mutations.
    pub structural_mutations: Vec<StructMutation>,
    /// Maximum candidates (finite, at least 1).
    pub max_candidates: u64,
    /// Deterministic ordering seed.
    pub seed: u64,
}

/// How candidates are judged (spec 022 manifest row 4).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvalPolicy {
    /// Exact obligations (claim ids).
    pub obligations: Vec<String>,
    /// Numeric tolerance, finite and positive.
    pub numeric_tolerance: f64,
    /// Why this tolerance is the right one.
    pub tolerance_justification: String,
    /// Empirical comparison method.
    pub comparison: String,
    /// Controls.
    pub controls: Vec<String>,
    /// Required evidence per candidate.
    pub evidence_required: Vec<String>,
}

/// One data source with locator and license.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataSource {
    /// Source identity.
    pub id: String,
    /// Where the bytes live.
    pub locator: String,
    /// License.
    pub license: String,
}

/// Data handling (spec 022 manifest row 5): calibration and evaluation
/// inputs are separate lists, and an input in both is leakage.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DataPolicy {
    /// Sources with locators and licenses.
    pub sources: Vec<DataSource>,
    /// Calibration input identities.
    pub calibration: Vec<String>,
    /// Evaluation input identities (disjoint from calibration).
    pub evaluation_inputs: Vec<String>,
    /// Uncertainty/covariance interpretation.
    pub uncertainty: String,
    /// Data-access approval reference.
    pub access_approval: String,
    /// Adaptive-use history.
    pub adaptive_use: Vec<String>,
}

/// Finite budgets (spec 022 manifest row 7). Every field is required:
/// a missing budget is unbounded, and unbounded is refused.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Budget {
    /// Maximum actions.
    pub actions: u64,
    /// Maximum candidates.
    pub candidates: u64,
    /// Maximum attempts (an attempt is charged even when it fails).
    pub attempts: u64,
    /// Maximum wall time in seconds.
    pub wall_time_secs: u64,
    /// Maximum per-worker memory in megabytes.
    pub memory_mb: u64,
    /// Maximum per-worker output in bytes.
    pub output_bytes: u64,
    /// Approval-scoped external cost. R1 allows zero only.
    pub external_cost: u64,
}

/// Stop conditions (spec 022 manifest row 8).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StopPolicy {
    /// Stop after this many steps without progress (at least 1).
    pub no_progress_limit: u64,
    /// Human pause requested.
    pub human_pause: bool,
}

/// Versioned campaign manifest (R1 implements every spec 022 row except
/// dispatch, which is R4).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    /// Must be [`MANIFEST_SCHEMA`].
    pub schema: String,
    /// Must be [`MANIFEST_VERSION`].
    pub version: u32,
    /// Research objective.
    pub objective: Objective,
    /// Baseline.
    pub baseline: Baseline,
    /// Candidate space.
    pub candidate_space: CandidateSpace,
    /// Evaluation policy.
    pub evaluation: EvalPolicy,
    /// Data policy.
    pub data: DataPolicy,
    /// Required role per operation (op name to kebab-case role).
    pub authority: BTreeMap<String, String>,
    /// Finite budgets.
    pub budget: Budget,
    /// Stop conditions.
    pub stop: StopPolicy,
}

/// Dry-run plan for a manifest: parse, refuse schema violations, validate
/// live references against `lab`, and render the plan. Executes nothing
/// and grants no authority.
pub fn plan_text(source: &str, text: &str, lab: &Lab) -> Result<String, String> {
    let (manifest, id) = parse_manifest(text)?;
    validate(&manifest, lab)?;
    Ok(render(source, &id, &manifest))
}

/// Parse plus schema gate: duplicate keys, serde shape (unknown fields and
/// operations are refused by `deny_unknown_fields` and the closed
/// [`CampaignOp`] enum), then schema tag and version.
pub(crate) fn parse_manifest(text: &str) -> Result<(Manifest, ArtifactId), String> {
    reject_duplicate_keys(text)?;
    let manifest: Manifest =
        serde_json::from_str(text).map_err(|e| format!("manifest schema: {e}"))?;
    if manifest.schema != MANIFEST_SCHEMA {
        return Err(format!(
            "manifest schema is '{}', want '{MANIFEST_SCHEMA}'",
            manifest.schema
        ));
    }
    if manifest.version != MANIFEST_VERSION {
        return Err(format!(
            "unknown manifest version {}, want {MANIFEST_VERSION}",
            manifest.version
        ));
    }
    Ok((manifest, ArtifactId::of(text.as_bytes())))
}

/// Refuse duplicate object keys before serde (which would silently keep
/// the last one). Compares unescaped key text per object frame; keys that
/// merely look alike inside string values are ignored. Malformed JSON is
/// left for serde to refuse with its own error.
fn reject_duplicate_keys(text: &str) -> Result<(), String> {
    enum Frame {
        Object(HashSet<String>),
        Array,
    }
    let bytes = text.as_bytes();
    let mut stack: Vec<Frame> = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => {
                let (key, next) = scan_string(bytes, i)?;
                i = next;
                let mut j = i;
                while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                    j += 1;
                }
                if j < bytes.len() && bytes[j] == b':' {
                    if let Some(Frame::Object(seen)) = stack.last_mut() {
                        if !seen.insert(key.clone()) {
                            return Err(format!("manifest has duplicate key \"{key}\""));
                        }
                    }
                    i = j + 1;
                }
            }
            b'{' => {
                stack.push(Frame::Object(HashSet::new()));
                i += 1;
            }
            b'[' => {
                stack.push(Frame::Array);
                i += 1;
            }
            b'}' | b']' => {
                stack.pop();
                i += 1;
            }
            _ => {
                i += 1;
            }
        }
    }
    Ok(())
}

/// Scan one JSON string starting at the opening quote, returning the
/// unescaped value and the index just past the closing quote. An
/// unterminated string returns what was scanned: serde reports the real
/// syntax error.
fn scan_string(bytes: &[u8], start: usize) -> Result<(String, usize), String> {
    let mut raw: Vec<u8> = Vec::new();
    let mut i = start + 1;
    while i < bytes.len() {
        match bytes[i] {
            b'"' => {
                let key = String::from_utf8(raw)
                    .map_err(|_| "manifest has invalid string bytes".to_string())?;
                return Ok((key, i + 1));
            }
            b'\\' => {
                i += 1;
                if i >= bytes.len() {
                    break;
                }
                match bytes[i] {
                    b'"' => raw.push(b'"'),
                    b'\\' => raw.push(b'\\'),
                    b'/' => raw.push(b'/'),
                    b'b' => raw.push(0x08),
                    b'f' => raw.push(0x0C),
                    b'n' => raw.push(b'\n'),
                    b'r' => raw.push(b'\r'),
                    b't' => raw.push(b'\t'),
                    b'u' => {
                        if i + 4 >= bytes.len() {
                            return Err("manifest has truncated unicode escape".into());
                        }
                        let hex = std::str::from_utf8(&bytes[i + 1..i + 5])
                            .map_err(|_| "manifest has invalid unicode escape".to_string())?;
                        let cp = u32::from_str_radix(hex, 16)
                            .map_err(|_| "manifest has invalid unicode escape".to_string())?;
                        let mut buf = [0u8; 4];
                        raw.extend_from_slice(
                            char::from_u32(cp)
                                .unwrap_or('\u{FFFD}')
                                .encode_utf8(&mut buf)
                                .as_bytes(),
                        );
                        i += 4;
                    }
                    other => {
                        return Err(format!("manifest has invalid string escape 0x{other:02x}"));
                    }
                }
                i += 1;
            }
            b => {
                raw.push(b);
                i += 1;
            }
        }
    }
    let key =
        String::from_utf8(raw).map_err(|_| "manifest has invalid string bytes".to_string())?;
    Ok((key, i))
}

/// Kebab-case operation name, matching the manifest wire form.
pub(crate) fn op_name(op: CampaignOp) -> &'static str {
    match op {
        CampaignOp::Prove => "prove",
        CampaignOp::Cite => "cite",
        CampaignOp::Enclose => "enclose",
        CampaignOp::Formalize => "formalize",
        CampaignOp::Hypothesize => "hypothesize",
    }
}

/// Dummy command per operation, for the role-permission check. Permission
/// is variant-level, so empty fields are fine.
pub(crate) fn op_command(op: CampaignOp) -> Command {
    match op {
        CampaignOp::Prove => Command::Prove {
            claim: String::new(),
        },
        CampaignOp::Cite => Command::Cite {
            claim: String::new(),
        },
        CampaignOp::Enclose => Command::Enclose {
            claim: String::new(),
        },
        CampaignOp::Formalize => Command::Formalize {
            claim: String::new(),
        },
        CampaignOp::Hypothesize => Command::Hypothesize { theory: None },
    }
}

/// Live validation against the lab: theories, statement identities, knob
/// domains and origins, structural-mutation labels, role permissions, and
/// finite budgets. The first violation is refused with its reason.
pub(crate) fn validate(manifest: &Manifest, lab: &Lab) -> Result<(), String> {
    validate_objective(&manifest.objective, lab)?;
    validate_baseline(&manifest.baseline, lab)?;
    validate_candidate_space(&manifest.candidate_space, lab)?;
    validate_evaluation(&manifest.evaluation, lab)?;
    validate_data(&manifest.data)?;
    validate_authority(&manifest.authority, &manifest.candidate_space.operations)?;
    validate_budget(&manifest.budget)?;
    if manifest.stop.no_progress_limit == 0 {
        return Err("stop no_progress_limit must be at least 1, got 0".into());
    }
    Ok(())
}

fn nonempty(field: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{field} must not be empty"));
    }
    Ok(())
}

fn validate_objective(objective: &Objective, lab: &Lab) -> Result<(), String> {
    nonempty("objective question", &objective.question)?;
    nonempty("objective observable", &objective.observable)?;
    nonempty("objective units", &objective.units)?;
    nonempty("objective domain", &objective.domain)?;
    nonempty("objective success", &objective.success)?;
    nonempty("objective failure", &objective.failure)?;
    nonempty("objective inconclusive", &objective.inconclusive)?;
    if objective.theories.is_empty() {
        return Err("objective theories must list at least one theory".into());
    }
    for theory in &objective.theories {
        lab.theory(theory)
            .map_err(|_| format!("unknown theory '{theory}'"))?;
    }
    for statement in &objective.statements {
        if lab.find_claim(statement).is_none() {
            return Err(format!("unknown statement '{statement}'"));
        }
    }
    Ok(())
}

fn validate_baseline(baseline: &Baseline, lab: &Lab) -> Result<(), String> {
    nonempty("baseline source_revision", &baseline.source_revision)?;
    lab.theory(&baseline.theory_package).map_err(|_| {
        format!(
            "baseline theory_package: unknown theory '{}'",
            baseline.theory_package
        )
    })?;
    for pin in &baseline.knobs {
        let theory = lab
            .theory(&pin.theory)
            .map_err(|_| format!("unknown theory '{}'", pin.theory))?;
        let spec = theory
            .spec(&pin.knob)
            .map_err(|e| format!("baseline knob pin {}.{}: {e}", pin.theory, pin.knob))?;
        KnobValue::parse_in_domain(&pin.value, &spec.domain).map_err(|e| {
            format!(
                "baseline knob pin {}.{} = {:?}: {e}",
                pin.theory, pin.knob, pin.value
            )
        })?;
    }
    for (field, entries) in [
        ("baseline assumptions", &baseline.assumptions),
        ("baseline constants", &baseline.constants),
        ("baseline datasets", &baseline.datasets),
        ("baseline evidence", &baseline.evidence),
    ] {
        for entry in entries {
            nonempty(&format!("{field} entry"), entry)?;
        }
    }
    Ok(())
}

fn validate_candidate_space(space: &CandidateSpace, lab: &Lab) -> Result<(), String> {
    if space.operations.is_empty() {
        return Err("candidate_space operations must list at least one operation".into());
    }
    let mut seen = HashSet::new();
    for op in &space.operations {
        if !seen.insert(op_name(*op)) {
            return Err(format!("duplicate operation '{}'", op_name(*op)));
        }
    }
    for range in &space.parameter_ranges {
        validate_range(range, lab)?;
    }
    for mutation in &space.structural_mutations {
        let theory = lab
            .theory(&mutation.theory)
            .map_err(|_| format!("unknown theory '{}'", mutation.theory))?;
        let labels: Vec<String> = theory
            .structural_mutations()
            .into_iter()
            .map(|(label, _)| label)
            .collect();
        if !labels.contains(&mutation.label) {
            return Err(format!(
                "unknown structural mutation '{}' for theory '{}'",
                mutation.label, mutation.theory
            ));
        }
    }
    if space.max_candidates == 0 {
        return Err("candidate_space max_candidates must be at least 1, got 0".into());
    }
    Ok(())
}

/// One parameter range: chosen/fitted knob only (measured inputs stay
/// frozen), finite ordered bounds inside the live domain.
fn validate_range(range: &ParamRange, lab: &Lab) -> Result<(), String> {
    let theory = lab
        .theory(&range.theory)
        .map_err(|_| format!("unknown theory '{}'", range.theory))?;
    let spec = theory
        .spec(&range.knob)
        .map_err(|e| format!("parameter range {}.{}: {e}", range.theory, range.knob))?;
    if !matches!(
        spec.origin,
        ParameterOrigin::Chosen | ParameterOrigin::Fitted
    ) {
        return Err(format!(
            "parameter range on frozen knob '{}' ({:?}): only chosen/fitted knobs; measured inputs stay frozen",
            range.knob, spec.origin
        ));
    }
    if !range.min.is_finite() || !range.max.is_finite() {
        return Err(format!(
            "parameter range {}.{} bounds must be finite, got [{}, {}]",
            range.theory, range.knob, range.min, range.max
        ));
    }
    if range.min > range.max {
        return Err(format!(
            "parameter range {}.{} min {} exceeds max {}",
            range.theory, range.knob, range.min, range.max
        ));
    }
    match &spec.domain {
        KnobDomain::Int { min, max } => {
            if range.min.fract() != 0.0 || range.max.fract() != 0.0 {
                return Err(format!(
                    "parameter range {}.{} bounds must be integers for an int domain, got [{}, {}]",
                    range.theory, range.knob, range.min, range.max
                ));
            }
            if range.min < *min as f64 || range.max > *max as f64 {
                return Err(format!(
                    "parameter range {}.{} [{}, {}] outside knob domain int [{min}, {max}]",
                    range.theory, range.knob, range.min, range.max
                ));
            }
        }
        KnobDomain::UInt { min, max } => {
            if range.min.fract() != 0.0 || range.max.fract() != 0.0 {
                return Err(format!(
                    "parameter range {}.{} bounds must be integers for a uint domain, got [{}, {}]",
                    range.theory, range.knob, range.min, range.max
                ));
            }
            if range.min < *min as f64 || range.max > *max as f64 {
                return Err(format!(
                    "parameter range {}.{} [{}, {}] outside knob domain uint [{min}, {max}]",
                    range.theory, range.knob, range.min, range.max
                ));
            }
        }
        KnobDomain::Float { min, max } => {
            if range.min < *min || range.max > *max {
                return Err(format!(
                    "parameter range {}.{} [{}, {}] outside knob domain float [{min}, {max}]",
                    range.theory, range.knob, range.min, range.max
                ));
            }
        }
        KnobDomain::Bool | KnobDomain::Choice(_) => {
            return Err(format!(
                "parameter range on non-numeric knob '{}.{}' ({} domain)",
                range.theory,
                range.knob,
                spec.domain.kind_name()
            ));
        }
    }
    Ok(())
}

fn validate_evaluation(policy: &EvalPolicy, lab: &Lab) -> Result<(), String> {
    for obligation in &policy.obligations {
        if lab.find_claim(obligation).is_none() {
            return Err(format!("unknown obligation '{obligation}'"));
        }
    }
    if !policy.numeric_tolerance.is_finite() || policy.numeric_tolerance <= 0.0 {
        return Err(format!(
            "evaluation numeric_tolerance must be finite and positive, got {}",
            policy.numeric_tolerance
        ));
    }
    nonempty(
        "evaluation tolerance_justification",
        &policy.tolerance_justification,
    )?;
    nonempty("evaluation comparison", &policy.comparison)?;
    for entry in policy
        .controls
        .iter()
        .chain(policy.evidence_required.iter())
    {
        nonempty("evaluation controls/evidence entry", entry)?;
    }
    Ok(())
}

fn validate_data(policy: &DataPolicy) -> Result<(), String> {
    for source in &policy.sources {
        nonempty("data source id", &source.id)?;
        nonempty("data source locator", &source.locator)?;
        nonempty("data source license", &source.license)?;
    }
    for entry in policy
        .calibration
        .iter()
        .chain(policy.evaluation_inputs.iter())
        .chain(policy.adaptive_use.iter())
    {
        nonempty("data input entry", entry)?;
    }
    let calibration: HashSet<&str> = policy.calibration.iter().map(String::as_str).collect();
    for input in &policy.evaluation_inputs {
        if calibration.contains(input.as_str()) {
            return Err(format!(
                "input '{input}' is both calibration and evaluation input"
            ));
        }
    }
    nonempty("data uncertainty", &policy.uncertainty)?;
    nonempty("data access_approval", &policy.access_approval)?;
    Ok(())
}

fn validate_authority(
    authority: &BTreeMap<String, String>,
    operations: &[CampaignOp],
) -> Result<(), String> {
    for op in operations {
        let name = op_name(*op);
        let role_name = authority
            .get(name)
            .ok_or_else(|| format!("missing authority entry for operation '{name}'"))?;
        let role = Role::parse(role_name)
            .ok_or_else(|| format!("unknown role '{role_name}' for operation '{name}'"))?;
        if !role.permits(&op_command(*op)) {
            return Err(format!("role '{role_name}' cannot run operation '{name}'"));
        }
    }
    let listed: HashSet<&str> = operations.iter().map(|op| op_name(*op)).collect();
    for name in authority.keys() {
        if !listed.contains(name.as_str()) {
            return Err(format!("authority entry for unlisted operation '{name}'"));
        }
    }
    Ok(())
}

fn validate_budget(budget: &Budget) -> Result<(), String> {
    for (field, value) in [
        ("actions", budget.actions),
        ("candidates", budget.candidates),
        ("attempts", budget.attempts),
        ("wall_time_secs", budget.wall_time_secs),
        ("memory_mb", budget.memory_mb),
        ("output_bytes", budget.output_bytes),
    ] {
        if value == 0 {
            return Err(format!("budget {field} must be at least 1, got 0"));
        }
    }
    if budget.external_cost != 0 {
        return Err(format!(
            "budget external_cost must be 0 in R1 (got {}): external spend needs approval",
            budget.external_cost
        ));
    }
    Ok(())
}

/// Render the validated plan. Deterministic text from the manifest plus its
/// content id; the dry-run marker is part of the contract.
fn render(source: &str, id: &ArtifactId, manifest: &Manifest) -> String {
    let mut out = format!("research plan  {source}\n  manifest {}\n", id.to_hex());
    out.push_str(&format!("  question: {}\n", manifest.objective.question));
    out.push_str(&format!(
        "  theories: {}\n",
        manifest.objective.theories.join(", ")
    ));
    out.push_str(&format!(
        "  statements: {}\n",
        if manifest.objective.statements.is_empty() {
            "none".into()
        } else {
            manifest.objective.statements.join(", ")
        }
    ));
    let ops: Vec<String> = manifest
        .candidate_space
        .operations
        .iter()
        .map(|op| {
            let name = op_name(*op);
            let role = manifest
                .authority
                .get(name)
                .map(String::as_str)
                .unwrap_or("?");
            format!("{name} ({role})")
        })
        .collect();
    out.push_str(&format!("  operations: {}\n", ops.join(", ")));
    out.push_str(&format!(
        "  candidates: max {} seed {} ranges {} mutations {}\n",
        manifest.candidate_space.max_candidates,
        manifest.candidate_space.seed,
        manifest.candidate_space.parameter_ranges.len(),
        manifest.candidate_space.structural_mutations.len()
    ));
    let b = &manifest.budget;
    out.push_str(&format!(
        "  budget: actions={} candidates={} attempts={} wall={}s mem={}MB out={}B external={}\n",
        b.actions,
        b.candidates,
        b.attempts,
        b.wall_time_secs,
        b.memory_mb,
        b.output_bytes,
        b.external_cost
    ));
    out.push_str(&format!(
        "  stop: no-progress<={} human-pause={}\n",
        manifest.stop.no_progress_limit, manifest.stop.human_pause
    ));
    out.push_str("  dry-run: no action executed, no authority granted\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lab() -> Lab {
        Lab::standard()
    }

    fn patched(f: impl FnOnce(&mut serde_json::Value)) -> String {
        let mut v: serde_json::Value = serde_json::from_str(EXAMPLE_MANIFEST).unwrap();
        f(&mut v);
        serde_json::to_string(&v).unwrap()
    }

    fn plan(text: &str) -> Result<String, String> {
        plan_text("test", text, &lab())
    }

    #[test]
    fn accepts_minimal_valid_manifest() {
        let out = plan(EXAMPLE_MANIFEST).expect("example must plan");
        assert!(out.contains("dry-run"), "{out}");
        assert!(out.contains("no action executed"), "{out}");
        assert!(out.contains("no authority granted"), "{out}");
        let id = physis_core::ArtifactId::of(EXAMPLE_MANIFEST.as_bytes()).to_hex();
        assert!(out.contains(&id), "{out}");
    }

    #[test]
    fn refuses_unknown_version() {
        let text = patched(|v| v["version"] = serde_json::json!(2));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("version"), "{err}");
    }

    #[test]
    fn refuses_unknown_schema() {
        let text = patched(|v| v["schema"] = serde_json::json!("nope"));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("schema"), "{err}");
    }

    #[test]
    fn refuses_duplicate_top_key() {
        let body = &EXAMPLE_MANIFEST[..EXAMPLE_MANIFEST.len() - 1];
        let text = format!("{body}, \"version\": 1}}");
        let err = plan(&text).unwrap_err();
        assert!(err.contains("duplicate"), "{err}");
    }

    #[test]
    fn refuses_duplicate_nested_key() {
        let text = EXAMPLE_MANIFEST.replace(
            "\"theory_package\": \"type-iib\"",
            "\"theory_package\": \"type-iib\", \"theory_package\": \"x\"",
        );
        assert_ne!(text, EXAMPLE_MANIFEST);
        let err = plan(&text).unwrap_err();
        assert!(err.contains("duplicate"), "{err}");
    }

    #[test]
    fn refuses_unknown_field() {
        let text = patched(|v| v["zzz"] = serde_json::json!(1));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("unknown field"), "{err}");
    }

    #[test]
    fn refuses_vacuous_budgets() {
        for key in ["actions", "candidates", "attempts"] {
            let text = patched(|v| v["budget"][key] = serde_json::json!(0));
            let err = plan(&text).unwrap_err();
            assert!(err.contains("budget"), "{key}: {err}");
        }
    }

    #[test]
    fn r7_refuses_negative_and_zero_resource_budgets() {
        // Unit/sign attack: negative budgets are a serde type error and
        // zero wall/memory/output starve the campaign; both refuse.
        for key in ["wall_time_secs", "memory_mb", "output_bytes"] {
            let text = patched(|v| v["budget"][key] = serde_json::json!(0));
            let err = plan(&text).unwrap_err();
            assert!(err.contains("budget"), "{key}: {err}");
        }
        let text = patched(|v| v["budget"]["actions"] = serde_json::json!(-1));
        let err = plan(&text).unwrap_err();
        assert!(
            err.contains("invalid type") || err.contains("expected u64"),
            "{err}"
        );
    }

    #[test]
    fn refuses_external_cost() {
        let text = patched(|v| v["budget"]["external_cost"] = serde_json::json!(5));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("external"), "{err}");
    }

    #[test]
    fn refuses_unknown_operation() {
        let text =
            patched(|v| v["candidate_space"]["operations"] = serde_json::json!(["teleport"]));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("teleport"), "{err}");
    }

    #[test]
    fn refuses_duplicate_operation() {
        let text = patched(|v| {
            v["candidate_space"]["operations"] = serde_json::json!(["hypothesize", "hypothesize"]);
        });
        let err = plan(&text).unwrap_err();
        assert!(err.contains("duplicate"), "{err}");
    }

    #[test]
    fn refuses_empty_operations() {
        let text = patched(|v| v["candidate_space"]["operations"] = serde_json::json!([]));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("operations"), "{err}");
    }

    #[test]
    fn refuses_unknown_theory() {
        let text = patched(|v| v["objective"]["theories"] = serde_json::json!(["nope"]));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("theory"), "{err}");
    }

    #[test]
    fn refuses_empty_theories() {
        let text = patched(|v| v["objective"]["theories"] = serde_json::json!([]));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("theories"), "{err}");
    }

    #[test]
    fn refuses_unknown_statement() {
        let text = patched(|v| v["objective"]["statements"] = serde_json::json!(["no.such-claim"]));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("statement"), "{err}");
    }

    #[test]
    fn refuses_unknown_knob_pin() {
        let text = patched(|v| v["baseline"]["knobs"][0]["knob"] = serde_json::json!("nope"));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("knob"), "{err}");
    }

    #[test]
    fn refuses_bad_knob_pin_value() {
        let text = patched(|v| v["baseline"]["knobs"][0]["value"] = serde_json::json!("999"));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("not in"), "{err}");
    }

    #[test]
    fn refuses_range_on_measured_knob() {
        let text = patched(|v| {
            v["candidate_space"]["parameter_ranges"][0]["knob"] = serde_json::json!("observed_dim");
        });
        let err = plan(&text).unwrap_err();
        assert!(err.contains("frozen"), "{err}");
    }

    #[test]
    fn refuses_inverted_range() {
        let text = patched(|v| {
            v["candidate_space"]["parameter_ranges"][0]["min"] = serde_json::json!(30.0);
            v["candidate_space"]["parameter_ranges"][0]["max"] = serde_json::json!(3.0);
        });
        let err = plan(&text).unwrap_err();
        assert!(err.contains("min"), "{err}");
    }

    #[test]
    fn refuses_range_outside_domain() {
        let text = patched(|v| {
            v["candidate_space"]["parameter_ranges"][0]["min"] = serde_json::json!(0.0);
        });
        let err = plan(&text).unwrap_err();
        assert!(err.contains("domain"), "{err}");
    }

    #[test]
    fn refuses_unknown_structural_label() {
        let text = patched(|v| {
            v["candidate_space"]["structural_mutations"] =
                serde_json::json!([{"theory": "maxwell-vacuum", "label": "nope"}]);
        });
        let err = plan(&text).unwrap_err();
        assert!(err.contains("structural"), "{err}");
    }

    #[test]
    fn accepts_known_structural_label() {
        let text = patched(|v| {
            v["candidate_space"]["structural_mutations"] =
                serde_json::json!([{"theory": "maxwell-vacuum", "label": "add-monopole"}]);
        });
        plan(&text).expect("known label must plan");
    }

    #[test]
    fn refuses_unknown_role() {
        let text = patched(|v| v["authority"]["hypothesize"] = serde_json::json!("emperor"));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("role"), "{err}");
    }

    #[test]
    fn refuses_missing_authority() {
        let text = patched(|v| {
            v["candidate_space"]["operations"] = serde_json::json!(["hypothesize", "prove"]);
        });
        let err = plan(&text).unwrap_err();
        assert!(err.contains("authority"), "{err}");
    }

    #[test]
    fn refuses_extra_authority() {
        let text = patched(|v| v["authority"]["prove"] = serde_json::json!("lab"));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("authority"), "{err}");
    }

    #[test]
    fn refuses_role_that_cannot_run_operation() {
        let text = patched(|v| {
            v["candidate_space"]["operations"] = serde_json::json!(["prove"]);
            v["authority"] = serde_json::json!({"prove": "explorer"});
        });
        let err = plan(&text).unwrap_err();
        assert!(err.contains("cannot run"), "{err}");
    }

    #[test]
    fn refuses_calibration_evaluation_overlap() {
        let text = patched(|v| {
            v["data"]["calibration"] = serde_json::json!(["d1"]);
            v["data"]["evaluation_inputs"] = serde_json::json!(["d1"]);
        });
        let err = plan(&text).unwrap_err();
        assert!(err.contains("both"), "{err}");
    }

    #[test]
    fn refuses_empty_question() {
        let text = patched(|v| v["objective"]["question"] = serde_json::json!(""));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("question"), "{err}");
    }

    #[test]
    fn refuses_zero_no_progress_limit() {
        let text = patched(|v| v["stop"]["no_progress_limit"] = serde_json::json!(0));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("progress"), "{err}");
    }

    #[test]
    fn refuses_nonpositive_tolerance() {
        let text = patched(|v| v["evaluation"]["numeric_tolerance"] = serde_json::json!(0.0));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("tolerance"), "{err}");
    }

    #[test]
    fn refuses_zero_max_candidates() {
        let text = patched(|v| v["candidate_space"]["max_candidates"] = serde_json::json!(0));
        let err = plan(&text).unwrap_err();
        assert!(err.contains("max_candidates"), "{err}");
    }

    #[test]
    fn refuses_overflowing_tolerance() {
        let raw = EXAMPLE_MANIFEST.replace("0.000001", "1e999");
        assert_ne!(raw, EXAMPLE_MANIFEST);
        assert!(plan(&raw).is_err());
    }

    #[test]
    fn scanner_accepts_nested_objects() {
        assert!(reject_duplicate_keys(r#"{"a": {"b": 1, "c": [1, {"d": 2}]}}"#).is_ok());
    }

    #[test]
    fn scanner_refuses_top_dup() {
        let err = reject_duplicate_keys(r#"{"a": 1, "a": 2}"#).unwrap_err();
        assert!(err.contains("\"a\""), "{err}");
    }

    #[test]
    fn scanner_refuses_nested_dup() {
        let err = reject_duplicate_keys(r#"{"a": {"b": 1, "b": 2}}"#).unwrap_err();
        assert!(err.contains("\"b\""), "{err}");
    }

    #[test]
    fn scanner_ignores_dup_like_text_inside_strings() {
        assert!(reject_duplicate_keys(r#"{"a": "b: 1, \"b\": 2", "c": 1}"#).is_ok());
    }

    #[test]
    fn scanner_treats_escaped_keys_by_value() {
        let err = reject_duplicate_keys("{\"a\": 1, \"\\u0061\": 2}").unwrap_err();
        assert!(err.contains('a'), "{err}");
    }

    #[test]
    fn scanner_keeps_array_elements_separate() {
        assert!(reject_duplicate_keys(r#"[{"k": 1}, {"k": 2}]"#).is_ok());
    }
}
