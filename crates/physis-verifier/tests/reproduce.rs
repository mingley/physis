//! C6.2 acceptance cases: the documented second checker agrees on the
//! live `dec.d-squared-zero` export, and every tamper, mismatch, and
//! failure shape refuses or classifies without minting.

use std::path::PathBuf;
use std::process::Command;

use physis_core::formal::FormalClaim;
use physis_proof::{lookup, Challenge, UntrustedProof};
use physis_verifier::reproduce::{
    export_obligation, reproduce, run_checker, ApprovedChecker, ReproDisposition, ReproError,
    ReproLimits, ReproductionStore, CHECKER_ID, CHECKER_VERSION, OBLIGATION_SCHEMA,
    OBLIGATION_VERSION, REPRO_OUTPUT_BYTES, REPRO_WALL_SECS,
};

fn d2_challenge() -> Challenge {
    let claim = lookup("dec.d-squared-zero").unwrap().lab_claim();
    Challenge::generate(&FormalClaim::from_claim(&claim))
}

fn other_challenge() -> Challenge {
    let claim = lookup("sr.invariant-interval").unwrap().lab_claim();
    Challenge::generate(&FormalClaim::from_claim(&claim))
}

fn tetrahedron_challenge() -> Challenge {
    let claim = lookup("dec.d-squared-one").unwrap().lab_claim();
    Challenge::generate(&FormalClaim::from_claim(&claim))
}

/// Locate the independently built checker binary, building it first so
/// this suite passes under both `-p` and `--workspace` runs.
fn checker_bin() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = manifest
        .parent()
        .and_then(|p| p.parent())
        .expect("verifier lives two levels under the workspace root");
    let target = std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| workspace.join("target"));
    let exe = if cfg!(windows) {
        "physis-repro-check.exe"
    } else {
        "physis-repro-check"
    };
    let bin = target.join("debug").join(exe);
    if !bin.is_file() {
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
        let status = Command::new(cargo)
            .args([
                "build",
                "-p",
                "physis-repro-check",
                "--bin",
                "physis-repro-check",
            ])
            .current_dir(workspace)
            .status()
            .expect("cargo build runs");
        assert!(status.success(), "checker binary must build");
    }
    assert!(bin.is_file(), "checker binary at {}", bin.display());
    bin
}

fn bin_digest(bin: &std::path::Path) -> String {
    physis_core::artifact::ArtifactId::of(std::fs::read(bin).unwrap()).to_hex()
}

fn approved_bin() -> ApprovedChecker {
    let bin = checker_bin();
    let digest = bin_digest(&bin);
    ApprovedChecker::approve(&bin, &digest).expect("fresh binary approves against its own digest")
}

fn limits() -> ReproLimits {
    ReproLimits {
        wall_secs: REPRO_WALL_SECS,
        output_bytes: REPRO_OUTPUT_BYTES,
    }
}

#[test]
fn export_is_versioned_wire_json() {
    let bytes = export_obligation(&d2_challenge(), "run-wire-1").unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["schema"], OBLIGATION_SCHEMA);
    assert_eq!(v["version"], OBLIGATION_VERSION);
    assert_eq!(v["run_id"], "run-wire-1");
    assert_eq!(v["expect"], "zero");
    assert!(v["symbols"]
        .as_array()
        .unwrap()
        .contains(&serde_json::json!("a")));
    // Lexicographic keys, minimal separators, no trailing newline.
    let text = String::from_utf8(bytes).unwrap();
    assert!(!text.ends_with('\n'));
    assert!(text.find("\"expect\"").unwrap() < text.find("\"run_id\"").unwrap());
    assert!(text.find("\"run_id\"").unwrap() < text.find("\"schema\"").unwrap());
}

#[test]
fn export_refuses_empty_run_id() {
    assert!(matches!(
        export_obligation(&d2_challenge(), ""),
        Err(ReproError::Export(_))
    ));
}

#[test]
fn live_triangle_export_agrees_and_mints() {
    let challenge = d2_challenge();
    let attempt = reproduce(
        &challenge,
        &UntrustedProof::ExactIdentity,
        &approved_bin(),
        "run-agree-1",
        &limits(),
    )
    .unwrap();
    assert_eq!(attempt.disposition(), ReproDisposition::Agree);
    assert!(attempt.evaluated_points() > 0);
    let record = attempt.verified().expect("bound agree mints").artifact();
    assert_eq!(record.claim_id(), "dec.d-squared-zero");
    assert_eq!(record.statement_hash(), challenge.statement_hash());
    assert_eq!(record.challenge_hash(), challenge.challenge_hash());
    assert_eq!(record.run_id(), "run-agree-1");
    assert_eq!(record.checker_id(), CHECKER_ID);
    assert_eq!(record.checker_version(), CHECKER_VERSION);
    assert_eq!(record.obligation_sha256(), attempt.obligation_sha256());
}

#[test]
fn flipped_sign_is_disagreement_without_mint() {
    let bytes = export_obligation(&d2_challenge(), "run-flip-1").unwrap();
    let text = String::from_utf8(bytes).unwrap();
    // Every single sub->add flip in (b-a)-(c-a)+(c-b) leaves a nonzero
    // polynomial, whichever occurrence the replacement hits.
    let flipped = text.replacen("\"op\":\"sub\"", "\"op\":\"add\"", 1);
    assert_ne!(flipped, text);
    let attempt =
        run_checker(&approved_bin(), flipped.as_bytes(), "run-flip-1", &limits()).unwrap();
    assert_eq!(attempt.disposition(), ReproDisposition::Disagree);
    assert!(attempt.verified().is_none());
    assert!(
        attempt.detail().contains("counterexample"),
        "{}",
        attempt.detail()
    );
}

#[test]
fn unknown_expect_kind_is_unsupported() {
    let bytes = export_obligation(&d2_challenge(), "run-unsup-1").unwrap();
    let text = String::from_utf8(bytes).unwrap();
    let other = text.replacen("\"expect\":\"zero\"", "\"expect\":\"normalize\"", 1);
    let attempt = run_checker(&approved_bin(), other.as_bytes(), "run-unsup-1", &limits()).unwrap();
    assert_eq!(attempt.disposition(), ReproDisposition::Unsupported);
    assert!(attempt.verified().is_none());
}

#[test]
fn copied_checker_bytes_are_the_same_implementation() {
    let dir = std::env::temp_dir().join("physis-repro-copy");
    std::fs::create_dir_all(&dir).unwrap();
    let dest = dir.join("same-checker-copy");
    let bin = checker_bin();
    std::fs::copy(&bin, &dest).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perm = std::fs::metadata(&dest).unwrap().permissions();
        perm.set_mode(0o755);
        std::fs::set_permissions(&dest, perm).unwrap();
    }
    let approved = ApprovedChecker::approve(&dest, &bin_digest(&bin)).unwrap();
    let bytes = export_obligation(&d2_challenge(), "run-copy-1").unwrap();
    let attempt = run_checker(&approved, &bytes, "run-copy-1", &limits()).unwrap();
    assert_eq!(attempt.disposition(), ReproDisposition::Agree);
    // Probing execution never mints: only the full pipeline does.
    assert!(attempt.verified().is_none());
    std::fs::remove_file(&dest).ok();
}

#[test]
fn wrong_expected_digest_refuses_approval() {
    let bin = checker_bin();
    assert!(matches!(
        ApprovedChecker::approve(&bin, &"0".repeat(64)),
        Err(ReproError::Approval(_))
    ));
}

#[test]
fn another_program_is_not_the_checker() {
    // The test binary itself, approved against the checker's digest:
    // same harness, different name, refused.
    let me = std::env::current_exe().unwrap();
    let digest = bin_digest(&checker_bin());
    assert!(matches!(
        ApprovedChecker::approve(&me, &digest),
        Err(ReproError::Approval(_))
    ));
}

#[test]
fn missing_program_is_refused_before_spawn() {
    let ghost = std::env::temp_dir().join("physis-no-such-checker");
    let _ = std::fs::remove_file(&ghost);
    assert!(matches!(
        ApprovedChecker::approve(&ghost, &"0".repeat(64)),
        Err(ReproError::Approval(_))
    ));
}

#[test]
fn primary_failure_runs_no_checker() {
    let err = reproduce(
        &d2_challenge(),
        &UntrustedProof::LeanSource {
            source: "theorem T : True := sorry\n".into(),
        },
        &approved_bin(),
        "run-primary-1",
        &limits(),
    )
    .unwrap_err();
    assert!(matches!(err, ReproError::PrimaryFailed(_)));
}

#[test]
fn stale_statement_has_no_primary_to_reproduce() {
    // Same slug, different statement: no catalog identity to borrow, so
    // the primary recheck fails and the checker never runs.
    let stale = physis_core::claim::Claim::new(
        "dec.d-squared-zero",
        "The exterior derivative is nilpotent: d ∘ d = 0.",
        physis_core::id::LayerId::Mathematical,
        physis_core::assurance::ClaimClass::Mathematical,
    );
    let challenge = Challenge::generate(&FormalClaim::from_claim(&stale));
    let err = reproduce(
        &challenge,
        &UntrustedProof::ExactIdentity,
        &approved_bin(),
        "run-primary-2",
        &limits(),
    )
    .unwrap_err();
    assert!(matches!(err, ReproError::PrimaryFailed(_)));
}

#[test]
fn store_binds_live_statement_not_slug() {
    let challenge = d2_challenge();
    let attempt = reproduce(
        &challenge,
        &UntrustedProof::ExactIdentity,
        &approved_bin(),
        "run-store-1",
        &limits(),
    )
    .unwrap();
    let mut store = ReproductionStore::empty();
    assert!(store.is_empty());
    store.record(attempt.verified().unwrap());
    assert_eq!(store.len(), 1);
    assert!(store.by_statement(challenge.statement_hash()).is_some());
    // Same slug family, different live statement: stale binding misses.
    assert!(store
        .by_statement(other_challenge().statement_hash())
        .is_none());
    // Display lookup still finds the slug.
    assert_eq!(
        store.by_claim("dec.d-squared-zero").unwrap().run_id(),
        "run-store-1"
    );
    assert!(store.by_claim("sr.invariant-interval").is_none());
}

#[test]
fn foreign_zero_identity_agrees_but_never_mints() {
    // The tetrahedron identity is genuinely zero, but bytes handed to
    // the probe are not the live obligation: agreement without mint.
    let bytes = export_obligation(&tetrahedron_challenge(), "run-foreign-1").unwrap();
    let attempt = run_checker(&approved_bin(), &bytes, "run-foreign-1", &limits()).unwrap();
    assert_eq!(attempt.disposition(), ReproDisposition::Agree);
    assert!(attempt.verified().is_none());
}

#[test]
fn executable_changed_since_approval_is_refused() {
    let dir = std::env::temp_dir().join("physis-repro-swap");
    std::fs::create_dir_all(&dir).unwrap();
    let dest = dir.join("swapped-checker");
    let bin = checker_bin();
    std::fs::copy(&bin, &dest).unwrap();
    let approved = ApprovedChecker::approve(&dest, &bin_digest(&bin)).unwrap();
    // Tamper after approval: one appended byte changes the digest.
    use std::io::Write;
    std::fs::OpenOptions::new()
        .append(true)
        .open(&dest)
        .unwrap()
        .write_all(b"x")
        .unwrap();
    let bytes = export_obligation(&d2_challenge(), "run-swap-1").unwrap();
    let err = run_checker(&approved, &bytes, "run-swap-1", &limits()).unwrap_err();
    assert!(matches!(err, ReproError::Approval(_)));
    std::fs::remove_file(&dest).ok();
}

#[test]
fn tampered_bytes_never_agree_with_mint() {
    // Valid envelope with the run echoed but a non-tree where the tree
    // belongs: the checker reports invalid, bound to this run.
    let bytes = export_obligation(&d2_challenge(), "run-tamper-1").unwrap();
    let text = String::from_utf8(bytes).unwrap();
    let cut = text.find("\"tree\"").unwrap();
    let tampered = format!("{}\"tree\":42}}", &text[..cut]);
    let attempt = run_checker(
        &approved_bin(),
        tampered.as_bytes(),
        "run-tamper-1",
        &limits(),
    )
    .unwrap();
    assert_eq!(attempt.disposition(), ReproDisposition::Invalid);
    assert!(attempt.verified().is_none());
}

#[test]
fn unparseable_bytes_are_an_unbound_response() {
    // Garbage the checker cannot attribute to the run echoes no run id,
    // so the host refuses the provenance instead of trusting `invalid`.
    let attempt = run_checker(&approved_bin(), b"{nope", "run-tamper-2", &limits()).unwrap();
    assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
    assert!(attempt.detail().contains("run id"), "{}", attempt.detail());
    assert!(attempt.verified().is_none());
}

#[cfg(unix)]
#[test]
fn obligation_copied_as_verdict_is_execution_failed() {
    // `cp in out` writes the obligation where the verdict belongs: a
    // well-formed file that is not a verdict is an invalid response.
    let cp = PathBuf::from("/bin/cp");
    let digest = bin_digest(&cp);
    let approved = ApprovedChecker::approve(&cp, &digest).unwrap();
    let bytes = export_obligation(&d2_challenge(), "run-cp-1").unwrap();
    let attempt = run_checker(&approved, &bytes, "run-cp-1", &limits()).unwrap();
    assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
    assert!(attempt.verified().is_none());
}

#[cfg(unix)]
#[test]
fn program_writing_no_verdict_is_execution_failed() {
    let echo = PathBuf::from("/bin/echo");
    let digest = bin_digest(&echo);
    let approved = ApprovedChecker::approve(&echo, &digest).unwrap();
    let bytes = export_obligation(&d2_challenge(), "run-echo-1").unwrap();
    let attempt = run_checker(&approved, &bytes, "run-echo-1", &limits()).unwrap();
    assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
    assert!(attempt.verified().is_none());
}

#[cfg(unix)]
#[test]
fn crashing_program_is_execution_failed() {
    // `sh` executes the obligation JSON as a script and exits nonzero.
    let sh = PathBuf::from("/bin/sh");
    let digest = bin_digest(&sh);
    let approved = ApprovedChecker::approve(&sh, &digest).unwrap();
    let bytes = export_obligation(&d2_challenge(), "run-sh-1").unwrap();
    let attempt = run_checker(&approved, &bytes, "run-sh-1", &limits()).unwrap();
    assert_eq!(attempt.disposition(), ReproDisposition::ExecutionFailed);
    assert!(attempt.verified().is_none());
}
