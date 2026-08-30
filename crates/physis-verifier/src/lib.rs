//! Trusted verification receipts.
//!
//! This crate is the **only** place a [`Verified`] value can be minted.
//! Fields are private. There is no public constructor and **no
//! `Deserialize` impl** — JSON cannot manufacture a kernel proof.
//!
//! The public entry point is [`verify`]: it generates nothing the caller
//! did not already have as a [`physis_proof::Challenge`]. It *runs* two
//! independent checkers. Callers cannot pass a homemade `accepted: true`
//! receipt.
//!
//! External crates cannot construct [`Verified`] by struct literal:
//!
//! ```compile_fail
//! use physis_verifier::Verified;
//! let _ = Verified { artifact: (), receipt: todo!() };
//! ```
//!
//! And they cannot call the crate-private mint:
//!
//! ```compile_fail
//! use physis_verifier::Verified;
//! let _ = Verified::mint((), unimplemented!());
//! ```
//!
//! Serde cannot mint one either:
//!
//! ```compile_fail
//! fn needs_deserialize<'de, T: serde::Deserialize<'de>>() {}
//! fn _blocked() {
//!     needs_deserialize::<physis_verifier::Verified<()>>();
//! }
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use physis_core::artifact::ArtifactId;
use physis_core::axiom::AxiomId;
use physis_proof::{identity_is_zero, scan_lean_source, Challenge, UntrustedProof, CATALOG};
use serde::Serialize;

mod lean;

pub use lean::discover_tools;

/// Formal backend that produced a proof artifact.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FormalBackend {
    /// Lean 4 kernel compile plus nanoda replay of the `lean4export`.
    /// Both checkers must run or `verify` refuses to mint.
    Lean4,
    /// Dual-expanded exact polynomial identity. Not a Lean kernel proof.
    /// The receipt says so.
    ExactCertificate,
}

/// One checker's replay of a proof artifact.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CheckerReceipt {
    /// Checker name (`expand-recursive`, `expand-postfix`, `nanoda`, …).
    pub checker: String,
    /// Checker version string.
    pub version: String,
    /// Hash of the checker identity (name+version).
    pub checker_hash: ArtifactId,
    /// Whether replay succeeded against the challenge statement.
    pub accepted: bool,
}

impl CheckerReceipt {
    pub(crate) fn ran(name: &str, version: &str, accepted: bool) -> Self {
        Self {
            checker: name.into(),
            version: version.into(),
            checker_hash: ArtifactId::of(format!("{name}:{version}").as_bytes()),
            accepted,
        }
    }
}

/// Binding of a statement hash to a dual-checked proof.
///
/// Constructing this struct is not enough to create a [`Verified`] value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ProofReceipt {
    /// Claim id the challenge was generated from.
    pub claim_id: String,
    /// Hash of the formal statement the checkers saw.
    pub statement_hash: ArtifactId,
    /// Hash of the assumption set.
    pub assumption_hash: ArtifactId,
    /// Hash of the challenge (statement + identity + lean type).
    pub challenge_hash: ArtifactId,
    /// Hash of the proof artifact bytes (canonical identity / export).
    pub proof_artifact_hash: ArtifactId,
    /// Backend.
    pub formal_backend: FormalBackend,
    /// Backend version.
    pub formal_backend_version: String,
    /// Lockfile hash of the formal library / expander pair.
    pub library_lock_hash: ArtifactId,
    /// Primary replay.
    pub primary_checker: CheckerReceipt,
    /// Independent replay.
    pub secondary_checker: CheckerReceipt,
    /// Transitive axiom ids.
    pub axioms_used: Vec<AxiomId>,
}

/// An artifact that has passed the trusted verifier.
///
/// Constructor is private to this crate. There is no [`serde::Deserialize`]
/// impl: a forged JSON document is not a kernel proof.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Verified<T> {
    artifact: T,
    receipt: ProofReceipt,
}

impl<T> Verified<T> {
    fn mint(artifact: T, receipt: ProofReceipt) -> Self {
        Self { artifact, receipt }
    }

    /// Borrow the artifact.
    pub fn artifact(&self) -> &T {
        &self.artifact
    }

    /// Borrow the receipt.
    pub fn receipt(&self) -> &ProofReceipt {
        &self.receipt
    }
}

/// Why verification refused to mint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VerifyError {
    /// No exact identity is catalogued for this claim.
    NoExactIdentity,
    /// Dual expanders did not both see the zero polynomial.
    IdentityFailed(String),
    /// Lean source contained `axiom` / `sorry` / `admit`.
    UnauthorizedAxiom(String),
    /// Lean kernel + independent checker are not both available.
    LeanPipelineNotWired,
    /// Challenge hash did not match a recomputation of the canonical bytes.
    ChallengeTampered,
    /// Lean source compiled, but no theorem matched the challenge type.
    StatementMismatch,
    /// The Lean compiler / lake / exporter rejected the source.
    LeanKernelRejected(String),
    /// nanoda refused the export (panic, axiom, or type error).
    NanodaRejected(String),
}

impl std::fmt::Display for VerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyError::NoExactIdentity => {
                write!(f, "no exact identity catalogued; Lean pipeline required")
            }
            VerifyError::IdentityFailed(s) => write!(f, "identity failed: {s}"),
            VerifyError::UnauthorizedAxiom(s) => write!(f, "unauthorized axiom or hole: {s}"),
            VerifyError::LeanPipelineNotWired => write!(
                f,
                "Lean kernel + independent checker are not both wired; refusing to mint"
            ),
            VerifyError::ChallengeTampered => {
                write!(f, "challenge hash does not match canonical bytes")
            }
            VerifyError::StatementMismatch => {
                write!(f, "Lean source has no theorem matching the challenge type")
            }
            VerifyError::LeanKernelRejected(s) => write!(f, "Lean kernel rejected source: {s}"),
            VerifyError::NanodaRejected(s) => write!(f, "nanoda rejected export: {s}"),
        }
    }
}

impl std::error::Error for VerifyError {}

/// Dual-check an untrusted artifact against a trusted challenge.
///
/// This is the only public function that returns [`Verified`].
pub fn verify(
    challenge: &Challenge,
    artifact: &UntrustedProof,
) -> Result<Verified<CheckedProof>, VerifyError> {
    if !challenge.hash_is_consistent() {
        return Err(VerifyError::ChallengeTampered);
    }
    bind_catalog(challenge)?;

    match artifact {
        UntrustedProof::ExactIdentity => verify_exact(challenge),
        UntrustedProof::LeanSource { source } => {
            let scan = scan_lean_source(source);
            if !scan.clean() {
                let mut parts = scan.axioms.clone();
                parts.extend(scan.holes);
                return Err(VerifyError::UnauthorizedAxiom(parts.join("; ")));
            }
            verify_lean(challenge, source)
        }
        UntrustedProof::LeanExport { .. } => Err(VerifyError::LeanPipelineNotWired),
    }
}

/// Catalog backends (exact polynomial, catalog Lean type) bind to the
/// catalog FormalClaim, not the slug. A matching slug with a different
/// identity cannot borrow the Physlib obligation.
fn bind_catalog(challenge: &Challenge) -> Result<(), VerifyError> {
    if let Some(spec) = CATALOG
        .iter()
        .find(|s| s.formal_claim().statement_hash() == challenge.statement_hash())
    {
        if spec.claim_id != challenge.claim_id() {
            return Err(VerifyError::NoExactIdentity);
        }
        if challenge.lean_type() != spec.lean_type {
            return Err(VerifyError::StatementMismatch);
        }
        match challenge.identity() {
            Some(id) if id.canonical() == (spec.identity)().canonical() => {}
            Some(_) => {
                return Err(VerifyError::IdentityFailed(
                    "not the catalog identity tree".into(),
                ));
            }
            None => {}
        }
        return Ok(());
    }
    for spec in CATALOG {
        if challenge.lean_type() == spec.lean_type {
            return Err(VerifyError::NoExactIdentity);
        }
        if let Some(id) = challenge.identity() {
            if id.canonical() == (spec.identity)().canonical() {
                return Err(VerifyError::NoExactIdentity);
            }
        }
    }
    Ok(())
}

fn verify_exact(challenge: &Challenge) -> Result<Verified<CheckedProof>, VerifyError> {
    let identity = challenge.identity().ok_or(VerifyError::NoExactIdentity)?;
    identity_is_zero(identity).map_err(VerifyError::IdentityFailed)?;

    let primary = CheckerReceipt::ran("expand-recursive", "physis-exact-0", true);
    let secondary = CheckerReceipt::ran("expand-postfix", "physis-exact-0", true);
    if !primary.accepted || !secondary.accepted {
        return Err(VerifyError::IdentityFailed(
            "a checker refused the identity".into(),
        ));
    }

    let receipt = ProofReceipt {
        claim_id: challenge.claim_id().to_string(),
        statement_hash: challenge.statement_hash(),
        assumption_hash: challenge.assumption_hash(),
        challenge_hash: challenge.challenge_hash(),
        proof_artifact_hash: ArtifactId::of(identity.canonical().as_bytes()),
        formal_backend: FormalBackend::ExactCertificate,
        formal_backend_version: "physis-exact-0".into(),
        library_lock_hash: ArtifactId::of(b"expand-recursive+expand-postfix"),
        primary_checker: primary,
        secondary_checker: secondary,
        axioms_used: challenge
            .axioms()
            .iter()
            .cloned()
            .map(AxiomId::new)
            .collect(),
    };
    Ok(Verified::mint(
        CheckedProof {
            challenge_hash: challenge.challenge_hash(),
            backend: FormalBackend::ExactCertificate,
        },
        receipt,
    ))
}

fn verify_lean(challenge: &Challenge, source: &str) -> Result<Verified<CheckedProof>, VerifyError> {
    let (primary, secondary) = lean::check_source(challenge, source)?;
    if !primary.accepted || !secondary.accepted {
        return Err(VerifyError::LeanKernelRejected(
            "a checker refused the Lean artifact".into(),
        ));
    }
    let mut axioms: Vec<AxiomId> = ["propext", "Quot.sound", "Classical.choice"]
        .into_iter()
        .map(AxiomId::new)
        .collect();
    axioms.extend(challenge.axioms().iter().cloned().map(AxiomId::new));
    let receipt = ProofReceipt {
        claim_id: challenge.claim_id().to_string(),
        statement_hash: challenge.statement_hash(),
        assumption_hash: challenge.assumption_hash(),
        challenge_hash: challenge.challenge_hash(),
        proof_artifact_hash: ArtifactId::of(source.as_bytes()),
        formal_backend: FormalBackend::Lean4,
        formal_backend_version: "lean-4.34.0-rc2+nanoda-0.4.16".into(),
        library_lock_hash: ArtifactId::of(b"physlib+lean-kernel+nanoda"),
        primary_checker: primary,
        secondary_checker: secondary,
        axioms_used: axioms,
    };
    Ok(Verified::mint(
        CheckedProof {
            challenge_hash: challenge.challenge_hash(),
            backend: FormalBackend::Lean4,
        },
        receipt,
    ))
}

/// What a successful verify returns as the artifact payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct CheckedProof {
    /// Challenge this proof was judged against.
    pub challenge_hash: ArtifactId,
    /// Backend that checked it.
    pub backend: FormalBackend,
}

/// The lab-wide store of minted receipts. Inserts only through
/// [`ReceiptStore::record`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReceiptStore {
    receipts: Vec<ProofReceipt>,
}

impl ReceiptStore {
    /// No receipts.
    pub fn empty() -> Self {
        Self::default()
    }

    /// True when no kernel proofs have been minted.
    pub fn is_empty(&self) -> bool {
        self.receipts.is_empty()
    }

    /// Number of minted receipts.
    pub fn len(&self) -> usize {
        self.receipts.len()
    }

    /// Record a minted value. The only way a receipt enters the store.
    pub fn record<T>(&mut self, verified: &Verified<T>) {
        self.receipts.push(verified.receipt().clone());
    }

    /// Lookup by statement hash.
    pub fn by_statement(&self, statement_hash: ArtifactId) -> Option<&ProofReceipt> {
        self.receipts
            .iter()
            .find(|r| r.statement_hash == statement_hash)
    }

    /// Lookup by claim slug (last receipt wins). Not P3F: a changed
    /// statement identity keeps the slug and must use [`Self::by_statement`].
    pub fn by_claim(&self, claim_id: &str) -> Option<&ProofReceipt> {
        self.receipts.iter().rev().find(|r| r.claim_id == claim_id)
    }
}

#[cfg(test)]
mod tests {
    use physis_core::assurance::ClaimClass;
    use physis_core::claim::Claim;
    use physis_core::formal::FormalClaim;
    use physis_core::id::LayerId;
    use physis_proof::catalog::discrete_d2;
    use physis_proof::expr::{add, Expr};
    use physis_proof::lookup;

    use super::*;

    fn d2_claim() -> Claim {
        lookup("dec.d-squared-zero").unwrap().lab_claim()
    }

    fn interval_claim() -> Claim {
        lookup("sr.invariant-interval").unwrap().lab_claim()
    }

    fn composition_claim() -> Claim {
        lookup("sr.subluminal-composition").unwrap().lab_claim()
    }

    fn mass_shell_claim() -> Claim {
        lookup("sr.energy-momentum-invariant").unwrap().lab_claim()
    }

    #[test]
    fn exact_identity_mints_a_receipt() {
        let claim = d2_claim();
        let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
        let v = verify(&challenge, &UntrustedProof::ExactIdentity).unwrap();
        assert_eq!(v.receipt().claim_id, "dec.d-squared-zero");
        assert!(v.receipt().primary_checker.accepted);
        assert!(v.receipt().secondary_checker.accepted);
        assert!(matches!(
            v.receipt().formal_backend,
            FormalBackend::ExactCertificate
        ));
    }

    #[test]
    fn generated_challenge_is_consistent() {
        let claim = d2_claim();
        let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
        assert!(challenge.hash_is_consistent());
        verify(&challenge, &UntrustedProof::ExactIdentity).unwrap();
    }

    #[test]
    fn sorry_blocks_promotion() {
        let claim = d2_claim();
        let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
        let err = verify(
            &challenge,
            &UntrustedProof::LeanSource {
                source: "theorem T : True := sorry\n".into(),
            },
        )
        .unwrap_err();
        assert!(matches!(err, VerifyError::UnauthorizedAxiom(_)));
    }

    #[test]
    fn unauthorized_axiom_blocks_promotion() {
        let claim = d2_claim();
        let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
        let err = verify(
            &challenge,
            &UntrustedProof::LeanSource {
                source: "axiom answer_is_true : Desired\n".into(),
            },
        )
        .unwrap_err();
        assert!(matches!(err, VerifyError::UnauthorizedAxiom(_)));
    }

    #[test]
    fn clean_lean_true_is_not_the_d2_challenge() {
        let claim = d2_claim();
        let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
        let err = verify(
            &challenge,
            &UntrustedProof::LeanSource {
                source: "theorem T : True := trivial\n".into(),
            },
        )
        .unwrap_err();
        assert_eq!(err, VerifyError::StatementMismatch);
    }

    #[test]
    fn lean_export_bytes_alone_do_not_mint() {
        let claim = d2_claim();
        let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
        let err = verify(
            &challenge,
            &UntrustedProof::LeanExport {
                bytes: b"not a kernel export".to_vec(),
            },
        )
        .unwrap_err();
        assert_eq!(err, VerifyError::LeanPipelineNotWired);
    }

    #[test]
    fn physlib_dual_kernel_mints_when_pipeline_is_wired() {
        if discover_tools().is_none() {
            if std::env::var("CI").is_ok() {
                panic!("CI must install Lean 4.34 and lean4export (LEAN4EXPORT)");
            }
            return;
        }
        let claim = d2_claim();
        let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
        let v = verify(
            &challenge,
            &UntrustedProof::LeanSource {
                source: physis_proof::PHYSLIB_SOURCE.into(),
            },
        )
        .expect("Lean kernel + nanoda must mint for Physlib d² = 0");
        assert!(matches!(v.receipt().formal_backend, FormalBackend::Lean4));
        assert_eq!(v.receipt().primary_checker.checker, "lean-kernel");
        assert_eq!(v.receipt().secondary_checker.checker, "nanoda");
        assert!(v.receipt().axioms_used.iter().any(|a| a.0 == "propext"));
    }

    #[test]
    fn physlib_lorentz_dual_kernel_mints_when_pipeline_is_wired() {
        if discover_tools().is_none() {
            if std::env::var("CI").is_ok() {
                panic!("CI must install Lean 4.34 and lean4export (LEAN4EXPORT)");
            }
            return;
        }
        let claim = interval_claim();
        let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
        let v = verify(
            &challenge,
            &UntrustedProof::LeanSource {
                source: physis_proof::PHYSLIB_SOURCE.into(),
            },
        )
        .expect("Lean kernel + nanoda must mint for Physlib interval identity");
        assert!(matches!(v.receipt().formal_backend, FormalBackend::Lean4));
        assert_eq!(v.receipt().primary_checker.checker, "lean-kernel");
        assert_eq!(v.receipt().secondary_checker.checker, "nanoda");
    }

    #[test]
    fn store_only_grows_via_record() {
        let mut store = ReceiptStore::empty();
        assert!(store.is_empty());
        let claim = d2_claim();
        let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
        let v = verify(&challenge, &UntrustedProof::ExactIdentity).unwrap();
        store.record(&v);
        assert_eq!(store.len(), 1);
        assert!(store.by_claim("dec.d-squared-zero").is_some());
    }

    #[test]
    fn vacuous_true_is_not_the_catalog_identity() {
        // 0 = 0 is a different identity; attaching it requires a different
        // challenge hash, which will not match the d² claim.
        let zero = crate::VerifyError::NoExactIdentity; // type smoke
        let _ = zero;
        let tautology = add(Expr::c(0), Expr::c(0));
        // The catalog identity is discrete_d2, not 0.
        assert_ne!(tautology.canonical(), discrete_d2().canonical());
    }

    #[test]
    fn unspecified_slug_cannot_borrow_the_catalog_certificate() {
        let stale = Claim::new(
            "dec.d-squared-zero",
            "The exterior derivative is nilpotent: d ∘ d = 0.",
            LayerId::Mathematical,
            ClaimClass::Mathematical,
        );
        let challenge = Challenge::generate(&FormalClaim::from_claim(&stale));
        let err = verify(&challenge, &UntrustedProof::ExactIdentity).unwrap_err();
        assert_eq!(err, VerifyError::NoExactIdentity);
    }

    #[test]
    fn lorentz_catalog_mints() {
        let claim = interval_claim();
        let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
        verify(&challenge, &UntrustedProof::ExactIdentity).unwrap();
    }

    #[test]
    fn composition_catalog_mints() {
        let claim = composition_claim();
        let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
        verify(&challenge, &UntrustedProof::ExactIdentity).unwrap();
    }

    #[test]
    fn physlib_composition_dual_kernel_mints_when_pipeline_is_wired() {
        if discover_tools().is_none() {
            if std::env::var("CI").is_ok() {
                panic!("CI must install Lean 4.34 and lean4export (LEAN4EXPORT)");
            }
            return;
        }
        let claim = composition_claim();
        let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
        let v = verify(
            &challenge,
            &UntrustedProof::LeanSource {
                source: physis_proof::PHYSLIB_SOURCE.into(),
            },
        )
        .expect("Lean kernel + nanoda must mint for Physlib composition identity");
        assert!(matches!(v.receipt().formal_backend, FormalBackend::Lean4));
        assert_eq!(v.receipt().primary_checker.checker, "lean-kernel");
        assert_eq!(v.receipt().secondary_checker.checker, "nanoda");
    }

    #[test]
    fn mass_shell_catalog_mints() {
        let challenge = Challenge::generate(&FormalClaim::from_claim(&mass_shell_claim()));
        verify(&challenge, &UntrustedProof::ExactIdentity).unwrap();
    }

    #[test]
    fn physlib_mass_shell_dual_kernel_mints_when_pipeline_is_wired() {
        if discover_tools().is_none() {
            if std::env::var("CI").is_ok() {
                panic!("CI must install Lean 4.34 and lean4export (LEAN4EXPORT)");
            }
            return;
        }
        let challenge = Challenge::generate(&FormalClaim::from_claim(&mass_shell_claim()));
        let v = verify(
            &challenge,
            &UntrustedProof::LeanSource {
                source: physis_proof::PHYSLIB_SOURCE.into(),
            },
        )
        .expect("Lean kernel + nanoda must mint for Physlib mass-shell identity");
        assert!(matches!(v.receipt().formal_backend, FormalBackend::Lean4));
        assert_eq!(v.receipt().primary_checker.checker, "lean-kernel");
        assert_eq!(v.receipt().secondary_checker.checker, "nanoda");
        assert!(v
            .receipt()
            .axioms_used
            .iter()
            .any(|a| a.0 == "minkowski-interval-signature"));
    }
}
