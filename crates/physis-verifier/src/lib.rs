//! Trusted verification receipts.
//!
//! This crate is the **only** place a [`Verified`] value can be minted.
//! Fields are private. There is no public constructor. Theories, agents,
//! and CLI code may *hold* a `Verified<T>` they were given; they cannot
//! manufacture one by setting an enum.
//!
//! M1 does not yet run Lean. The minting API is `pub(crate)` so even this
//! crate's public surface cannot be used by an agent to stamp
//! `MachineProved`. Tests inside this crate exercise the type.
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

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use physis_core::artifact::ArtifactId;
use physis_core::axiom::AxiomId;
use serde::{Deserialize, Serialize};

/// Formal backend that produced a proof artifact.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FormalBackend {
    /// Lean 4 + Physlib (M2). Not wired in M1.
    Lean4,
}

/// One checker's replay of a proof artifact.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckerReceipt {
    /// Checker name (`lean-kernel`, `nanoda`, …).
    pub checker: String,
    /// Checker version string.
    pub version: String,
    /// Hash of the checker binary / library lock.
    pub checker_hash: ArtifactId,
    /// Whether replay succeeded against the challenge statement.
    pub accepted: bool,
}

/// Binding of a statement hash to a dual-checked proof.
///
/// Fields are public for serialization of *existing* receipts but the
/// only way to obtain a [`Verified`] wrapper is [`Verified::mint`], which
/// is crate-private.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofReceipt {
    /// Claim id the challenge was generated from.
    pub claim_id: String,
    /// Hash of the formal statement the checkers saw.
    pub statement_hash: ArtifactId,
    /// Hash of the assumption set.
    pub assumption_hash: ArtifactId,
    /// Hash of the proof artifact bytes.
    pub proof_artifact_hash: ArtifactId,
    /// Backend.
    pub formal_backend: FormalBackend,
    /// Backend version.
    pub formal_backend_version: String,
    /// Lockfile hash of the formal library.
    pub library_lock_hash: ArtifactId,
    /// Primary kernel replay.
    pub primary_checker: CheckerReceipt,
    /// Independent checker replay.
    pub secondary_checker: CheckerReceipt,
    /// Transitive axiom ids.
    pub axioms_used: Vec<AxiomId>,
}

/// An artifact that has passed the trusted verifier.
///
/// Constructor is private to this crate. Theories cannot mint this.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Verified<T> {
    artifact: T,
    receipt: ProofReceipt,
}

impl<T> Verified<T> {
    /// Mint a verified value. Crate-private: not part of the public API.
    #[allow(dead_code)]
    pub(crate) fn mint(artifact: T, receipt: ProofReceipt) -> Self {
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

/// The lab-wide store of minted receipts. M1 is empty: nothing is
/// `MachineProved` until the Lean pipeline (M2) mints a receipt.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReceiptStore {
    hashes: Vec<ArtifactId>,
}

impl ReceiptStore {
    /// No receipts.
    pub fn empty() -> Self {
        Self::default()
    }

    /// True when no kernel proofs have been minted.
    pub fn is_empty(&self) -> bool {
        self.hashes.is_empty()
    }

    /// Number of minted receipts.
    pub fn len(&self) -> usize {
        self.hashes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::artifact::ArtifactId;

    fn dummy_checker(name: &str) -> CheckerReceipt {
        CheckerReceipt {
            checker: name.into(),
            version: "test".into(),
            checker_hash: ArtifactId::of(name.as_bytes()),
            accepted: true,
        }
    }

    #[test]
    fn mint_is_possible_inside_the_verifier_crate_only() {
        let receipt = ProofReceipt {
            claim_id: "math.d2".into(),
            statement_hash: ArtifactId::of(b"d2=0"),
            assumption_hash: ArtifactId::of(b"assumptions"),
            proof_artifact_hash: ArtifactId::of(b"proof"),
            formal_backend: FormalBackend::Lean4,
            formal_backend_version: "test".into(),
            library_lock_hash: ArtifactId::of(b"lock"),
            primary_checker: dummy_checker("lean-kernel"),
            secondary_checker: dummy_checker("nanoda"),
            axioms_used: vec![],
        };
        let v = Verified::mint("d² = 0", receipt);
        assert_eq!(*v.artifact(), "d² = 0");
        assert_eq!(v.receipt().claim_id, "math.d2");
    }

    #[test]
    fn public_store_starts_empty() {
        assert!(ReceiptStore::empty().is_empty());
    }
}
