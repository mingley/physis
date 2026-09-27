//! Red-team corpus: intentionally wrong encodings that must not promote.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::path::PathBuf;

use physis_core::formal::FormalClaim;
use physis_core::{ArtifactId, ClaimClass, LayerId};
use physis_proof::expr::{add, mul, pow, sub, Expr};
use physis_proof::{identity_is_zero, lookup, Challenge, UntrustedProof};
use physis_verifier::reproduce::{
    export_obligation, reproduce, run_checker, ApprovedChecker, ReproDisposition, ReproLimits,
    ReproductionStore,
};
use physis_verifier::{verify, ReceiptStore, VerifyError};

/// A named corruption and whether verify/identity must reject it.
pub struct Mutation {
    /// Short name.
    pub name: &'static str,
    /// How to attack.
    pub kind: MutationKind,
}

/// Kind of attack.
pub enum MutationKind {
    /// A polynomial that must *not* be identically zero.
    NonZeroIdentity(fn() -> Expr),
    /// Lean source that must be rejected as unauthorized.
    HostileLean(&'static str),
}

/// The standing corpus (spec §46, first slice).
pub fn corpus() -> Vec<Mutation> {
    vec![
        Mutation {
            name: "wrong Maxwell / coboundary sign",
            kind: MutationKind::NonZeroIdentity(|| {
                let a = Expr::var("a");
                let b = Expr::var("b");
                let c = Expr::var("c");
                // last plus flipped
                sub(sub(sub(b.clone(), a.clone()), sub(c.clone(), a)), sub(c, b))
            }),
        },
        Mutation {
            name: "metric signature / Galilean interval",
            kind: MutationKind::NonZeroIdentity(|| {
                let t = Expr::var("t");
                let x = Expr::var("x");
                let b = Expr::var("beta");
                sub(
                    sub(pow(t.clone(), 2), pow(sub(x.clone(), mul(b, t.clone())), 2)),
                    sub(pow(t, 2), pow(x, 2)),
                )
            }),
        },
        Mutation {
            name: "c instead of c^2 (drop a square)",
            kind: MutationKind::NonZeroIdentity(|| {
                let t = Expr::var("t");
                let x = Expr::var("x");
                let b = Expr::var("beta");
                // drop the square on the boosted time term
                sub(
                    sub(
                        sub(t.clone(), mul(b.clone(), x.clone())),
                        pow(sub(x.clone(), mul(b.clone(), t.clone())), 2),
                    ),
                    mul(sub(Expr::c(1), pow(b, 2)), sub(pow(t, 2), pow(x, 2))),
                )
            }),
        },
        Mutation {
            name: "forall silently rewritten by sorry",
            kind: MutationKind::HostileLean("theorem T : Desired := sorry\n"),
        },
        Mutation {
            name: "hidden axiom answer_is_true",
            kind: MutationKind::HostileLean("axiom answer_is_true : DesiredTheorem\n"),
        },
        Mutation {
            name: "vacuous True as a stand-in for d^2 = 0",
            kind: MutationKind::NonZeroIdentity(|| add(Expr::var("a"), Expr::c(1))),
        },
        Mutation {
            name: "factor 2 deletion on (b-a)-(c-a)+2(c-b)",
            kind: MutationKind::NonZeroIdentity(|| {
                let a = Expr::var("a");
                let b = Expr::var("b");
                let c = Expr::var("c");
                add(
                    sub(sub(b.clone(), a.clone()), sub(c.clone(), a)),
                    mul(Expr::c(2), sub(c, b)),
                )
            }),
        },
    ]
}

/// Run the corpus. Every mutation must fail to promote.
pub fn attack() -> Result<(), String> {
    let claim = lookup("dec.d-squared-zero").unwrap().lab_claim();
    let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
    for m in corpus() {
        match m.kind {
            MutationKind::NonZeroIdentity(f) => {
                if identity_is_zero(&f()).is_ok() {
                    return Err(format!("{}: identity was zero; coverage hole", m.name));
                }
            }
            MutationKind::HostileLean(src) => {
                match verify(
                    &challenge,
                    &UntrustedProof::LeanSource { source: src.into() },
                ) {
                    Err(VerifyError::UnauthorizedAxiom(_)) => {}
                    other => {
                        return Err(format!(
                            "{}: expected unauthorized axiom, got {other:?}",
                            m.name
                        ))
                    }
                }
            }
        }
    }
    Ok(())
}

/// A named attack on the campaign substrate (R7) and the refusal it
/// must meet. Every attack must fail to promote trust.
pub struct CampaignAttack {
    /// Short name.
    pub name: &'static str,
    /// How to attack.
    pub kind: CampaignKind,
}

/// Kind of campaign-substrate attack.
pub enum CampaignKind {
    /// A receipt minted for one statement hash must miss under another.
    StaleReceiptBinding,
    /// A sealed reproduce record must miss under another statement hash.
    StaleReproBinding,
    /// Obligation export must refuse an empty run id and a claim with
    /// no catalogued exact identity.
    MalformedObligationExport,
    /// An approved executable changed before spawn must be refused.
    TamperedExecutable,
    /// Zero wall time and empty run id are refused before spawn.
    RefusedSpawnParams,
    /// `run_checker` alone never mints, even on honest agree bytes.
    CallerBytesNeverMint,
    /// `run_checker` on tampered bytes never mints.
    TamperedBytesNeverMint,
}

/// The standing campaign corpus (R7). Unit/sign, disclosure, manifest,
/// budget, and bundle attacks live as regression tests beside the code
/// they attack (`physis-agent`); this corpus covers the verifier-level
/// campaign substrate: receipt binding, obligation export, checker
/// approval, and the sealed mint boundary.
pub fn campaign_corpus() -> Vec<CampaignAttack> {
    vec![
        CampaignAttack {
            name: "stale receipt presented under a rotated statement hash",
            kind: CampaignKind::StaleReceiptBinding,
        },
        CampaignAttack {
            name: "stale sealed reproduction under a rotated statement hash",
            kind: CampaignKind::StaleReproBinding,
        },
        CampaignAttack {
            name: "malformed obligation export (empty run, uncatalogued claim)",
            kind: CampaignKind::MalformedObligationExport,
        },
        CampaignAttack {
            name: "approved checker executable tampered before spawn",
            kind: CampaignKind::TamperedExecutable,
        },
        CampaignAttack {
            name: "refused spawn parameters (zero wall, empty run)",
            kind: CampaignKind::RefusedSpawnParams,
        },
        CampaignAttack {
            name: "caller-supplied honest bytes presented as agreement",
            kind: CampaignKind::CallerBytesNeverMint,
        },
        CampaignAttack {
            name: "tampered obligation bytes presented as agreement",
            kind: CampaignKind::TamperedBytesNeverMint,
        },
    ]
}

/// Locate the independently built checker binary, building it first.
/// Shared by the campaign corpus and the agent regression tests so the
/// red-team always runs the same reviewed bytes the lab approves.
pub fn checker_binary() -> Result<PathBuf, String> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = manifest
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| "audit crate lives two levels under the workspace root".to_string())?;
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
        let status = std::process::Command::new(cargo)
            .args([
                "build",
                "-p",
                "physis-repro-check",
                "--bin",
                "physis-repro-check",
            ])
            .current_dir(workspace)
            .status()
            .map_err(|e| format!("checker build would not start: {e}"))?;
        if !status.success() {
            return Err("checker binary must build".into());
        }
    }
    Ok(bin)
}

/// Approve one checker binary by hashing its bytes now.
fn approve_checker(bin: &std::path::Path) -> Result<ApprovedChecker, String> {
    let bytes =
        std::fs::read(bin).map_err(|e| format!("cannot read checker '{}': {e}", bin.display()))?;
    let digest = ArtifactId::of(&bytes).to_hex();
    ApprovedChecker::approve(bin, &digest).map_err(|e| e.to_string())
}

/// The d-squared-zero challenge: catalogued exact identity, mints.
fn d2_challenge() -> Challenge {
    let claim = lookup("dec.d-squared-zero").unwrap().lab_claim();
    Challenge::generate(&FormalClaim::from_claim(&claim))
}

/// Run the campaign corpus. Every attack must fail to promote.
pub fn attack_campaign() -> Result<(), String> {
    for a in campaign_corpus() {
        match a.kind {
            CampaignKind::StaleReceiptBinding => {
                let challenge = d2_challenge();
                let v = verify(&challenge, &UntrustedProof::ExactIdentity)
                    .map_err(|e| format!("{}: honest mint failed: {e}", a.name))?;
                let mut store = ReceiptStore::empty();
                store.record(&v);
                let live = v.receipt().statement_hash;
                if store.by_statement(live).is_none() {
                    return Err(format!("{}: live binding missed; corpus broken", a.name));
                }
                let rotated = ArtifactId::of(b"r7-rotated-statement");
                if rotated == live {
                    return Err(format!("{}: rotated hash collided; corpus broken", a.name));
                }
                if store.by_statement(rotated).is_some() {
                    return Err(format!(
                        "{}: stale receipt satisfied a rotated hash; promotion",
                        a.name
                    ));
                }
            }
            CampaignKind::StaleReproBinding => {
                let bin = checker_binary().map_err(|e| format!("{}: {e}", a.name))?;
                let checker = approve_checker(&bin).map_err(|e| format!("{}: {e}", a.name))?;
                let challenge = d2_challenge();
                let attempt = reproduce(
                    &challenge,
                    &UntrustedProof::ExactIdentity,
                    &checker,
                    "r7-stale-repro-1",
                    &ReproLimits::default(),
                )
                .map_err(|e| format!("{}: honest reproduce failed: {e}", a.name))?;
                let verified = attempt.verified().ok_or_else(|| {
                    format!("{}: honest reproduce did not mint; corpus broken", a.name)
                })?;
                let mut store = ReproductionStore::empty();
                store.record(verified);
                let live = verified.artifact().statement_hash();
                if store.by_statement(live).is_none() {
                    return Err(format!("{}: live binding missed; corpus broken", a.name));
                }
                let rotated = ArtifactId::of(b"r7-rotated-repro-statement");
                if store.by_statement(rotated).is_some() {
                    return Err(format!(
                        "{}: stale reproduction satisfied a rotated hash; promotion",
                        a.name
                    ));
                }
            }
            CampaignKind::MalformedObligationExport => {
                let challenge = d2_challenge();
                match export_obligation(&challenge, "") {
                    Err(_) => {}
                    Ok(_) => {
                        return Err(format!("{}: empty run id exported; promotion", a.name));
                    }
                }
                let stray = physis_core::claim::Claim::new(
                    "r7.uncatalogued-claim",
                    "R7 uncatalogued sentence",
                    LayerId::Mathematical,
                    ClaimClass::Mathematical,
                );
                let stray_challenge = Challenge::generate(&FormalClaim::from_claim(&stray));
                if stray_challenge.identity().is_some() {
                    return Err(format!("{}: stray claim catalogued; corpus broken", a.name));
                }
                match export_obligation(&stray_challenge, "r7-stray-1") {
                    Err(e) if e.to_string().contains("no exact identity") => {}
                    other => {
                        return Err(format!(
                            "{}: expected no-identity refusal, got {other:?}",
                            a.name
                        ));
                    }
                }
            }
            CampaignKind::TamperedExecutable => {
                let dir = std::env::temp_dir();
                let path = dir.join(format!("physis-r7-tamper-{}", std::process::id()));
                std::fs::write(&path, b"r7-approved-bytes").map_err(|e| e.to_string())?;
                let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
                let digest = ArtifactId::of(&bytes).to_hex();
                let checker =
                    ApprovedChecker::approve(&path, &digest).map_err(|e| e.to_string())?;
                std::fs::write(&path, b"r7-tampered-bytes").map_err(|e| e.to_string())?;
                let challenge = d2_challenge();
                let bytes = export_obligation(&challenge, "r7-tamper-exe-1")
                    .map_err(|e| format!("{}: honest export failed: {e}", a.name))?;
                match run_checker(&checker, &bytes, "r7-tamper-exe-1", &ReproLimits::default()) {
                    Err(e) if e.to_string().contains("changed since approval") => {}
                    other => {
                        std::fs::remove_file(&path).ok();
                        return Err(format!(
                            "{}: expected changed-since-approval refusal, got {other:?}",
                            a.name
                        ));
                    }
                }
                std::fs::remove_file(&path).ok();
            }
            CampaignKind::RefusedSpawnParams => {
                let dir = std::env::temp_dir();
                let path = dir.join(format!("physis-r7-params-{}", std::process::id()));
                std::fs::write(&path, b"r7-param-bytes").map_err(|e| e.to_string())?;
                let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
                let digest = ArtifactId::of(&bytes).to_hex();
                let checker =
                    ApprovedChecker::approve(&path, &digest).map_err(|e| e.to_string())?;
                let challenge = d2_challenge();
                let bytes = export_obligation(&challenge, "r7-params-1")
                    .map_err(|e| format!("{}: honest export failed: {e}", a.name))?;
                let zero_wall = ReproLimits {
                    wall_secs: 0,
                    output_bytes: ReproLimits::default().output_bytes,
                };
                match run_checker(&checker, &bytes, "r7-params-1", &zero_wall) {
                    Err(e) if e.to_string().contains("must be positive") => {}
                    other => {
                        std::fs::remove_file(&path).ok();
                        return Err(format!(
                            "{}: expected positive-wall refusal, got {other:?}",
                            a.name
                        ));
                    }
                }
                match run_checker(&checker, &bytes, "", &ReproLimits::default()) {
                    Err(e) if e.to_string().contains("non-empty") => {}
                    other => {
                        std::fs::remove_file(&path).ok();
                        return Err(format!(
                            "{}: expected non-empty-run refusal, got {other:?}",
                            a.name
                        ));
                    }
                }
                std::fs::remove_file(&path).ok();
            }
            CampaignKind::CallerBytesNeverMint => {
                let bin = checker_binary().map_err(|e| format!("{}: {e}", a.name))?;
                let checker = approve_checker(&bin).map_err(|e| format!("{}: {e}", a.name))?;
                let challenge = d2_challenge();
                let bytes = export_obligation(&challenge, "r7-caller-bytes-1")
                    .map_err(|e| format!("{}: honest export failed: {e}", a.name))?;
                let attempt = run_checker(
                    &checker,
                    &bytes,
                    "r7-caller-bytes-1",
                    &ReproLimits::default(),
                )
                .map_err(|e| format!("{}: honest run failed: {e}", a.name))?;
                // Teeth: the honest checker really agreed; agreement alone
                // still seals nothing.
                if attempt.disposition() != ReproDisposition::Agree {
                    return Err(format!(
                        "{}: honest run did not agree ({:?}); corpus broken",
                        a.name,
                        attempt.disposition()
                    ));
                }
                if attempt.verified().is_some() {
                    return Err(format!(
                        "{}: caller-supplied bytes minted; promotion",
                        a.name
                    ));
                }
            }
            CampaignKind::TamperedBytesNeverMint => {
                let bin = checker_binary().map_err(|e| format!("{}: {e}", a.name))?;
                let checker = approve_checker(&bin).map_err(|e| format!("{}: {e}", a.name))?;
                let challenge = d2_challenge();
                let mut bytes = export_obligation(&challenge, "r7-tampered-bytes-1")
                    .map_err(|e| format!("{}: honest export failed: {e}", a.name))?;
                let honest = ArtifactId::of(&bytes).to_hex();
                let mid = bytes.len() / 2;
                bytes[mid] ^= 0x01;
                if ArtifactId::of(&bytes).to_hex() == honest {
                    return Err(format!(
                        "{}: tamper did not change bytes; corpus broken",
                        a.name
                    ));
                }
                let attempt = run_checker(
                    &checker,
                    &bytes,
                    "r7-tampered-bytes-1",
                    &ReproLimits::default(),
                )
                .map_err(|e| format!("{}: tampered run failed to execute: {e}", a.name))?;
                if attempt.verified().is_some() {
                    return Err(format!("{}: tampered bytes minted; promotion", a.name));
                }
                if attempt.obligation_sha256() == honest {
                    return Err(format!(
                        "{}: tampered run reported the honest hash; binding broken",
                        a.name
                    ));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_team_corpus_is_caught() {
        attack().unwrap();
    }

    #[test]
    fn campaign_corpus_is_caught() {
        attack_campaign().unwrap();
    }
}
