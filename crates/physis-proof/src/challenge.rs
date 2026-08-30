//! Immutable proof challenge. Generated on the trusted side from a
//! [`physis_core::FormalClaim`]. The solver never chooses the statement,
//! the Lean type, or the polynomial: those are private fields filled only
//! by [`Challenge::generate`]. JSON cannot mint a challenge.
//!
//! ```compile_fail
//! use physis_proof::Challenge;
//! let _ = Challenge {
//!     claim_id: String::new(),
//!     statement_hash: todo!(),
//!     assumption_hash: todo!(),
//!     lean_type: String::new(),
//!     identity: None,
//!     axioms: Vec::new(),
//!     challenge_hash: todo!(),
//! };
//! ```
//!
//! ```compile_fail
//! fn needs_deserialize<'de, T: serde::Deserialize<'de>>() {}
//! fn _blocked() {
//!     needs_deserialize::<physis_proof::Challenge>();
//! }
//! ```

use physis_core::artifact::ArtifactId;
use physis_core::formal::FormalClaim;
use serde::Serialize;

use crate::catalog;
use crate::expr::Expr;

/// Trusted challenge a candidate proof is judged against.
///
/// Constructed only by [`Challenge::generate`]. There is no
/// [`serde::Deserialize`] impl.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Challenge {
    claim_id: String,
    statement_hash: ArtifactId,
    assumption_hash: ArtifactId,
    lean_type: String,
    identity: Option<Expr>,
    axioms: Vec<String>,
    challenge_hash: ArtifactId,
}

impl Challenge {
    /// Canonical bytes the challenge hash commits to.
    pub fn canonical_bytes(
        claim_id: &str,
        statement_hash: ArtifactId,
        assumption_hash: ArtifactId,
        lean_type: &str,
        identity: Option<&Expr>,
        axioms: &[String],
    ) -> Vec<u8> {
        let mut s = String::new();
        s.push_str("claim:");
        s.push_str(claim_id);
        s.push('\n');
        s.push_str("statement:");
        s.push_str(&statement_hash.to_hex());
        s.push('\n');
        s.push_str("assumptions:");
        s.push_str(&assumption_hash.to_hex());
        s.push('\n');
        s.push_str("lean_type:");
        s.push_str(lean_type);
        s.push('\n');
        s.push_str("identity:");
        match identity {
            Some(e) => s.push_str(&e.canonical()),
            None => s.push_str("none"),
        }
        s.push('\n');
        for a in axioms {
            s.push_str("axiom:");
            s.push_str(a);
            s.push('\n');
        }
        s.into_bytes()
    }

    /// True when [`Self::challenge_hash`] matches [`Self::canonical_bytes`].
    pub fn hash_is_consistent(&self) -> bool {
        ArtifactId::of(Self::canonical_bytes(
            &self.claim_id,
            self.statement_hash,
            self.assumption_hash,
            &self.lean_type,
            self.identity.as_ref(),
            &self.axioms,
        )) == self.challenge_hash
    }

    /// Lab claim id.
    pub fn claim_id(&self) -> &str {
        &self.claim_id
    }

    /// Hash of the formal claim identity.
    pub fn statement_hash(&self) -> ArtifactId {
        self.statement_hash
    }

    /// Hash of the assumption set.
    pub fn assumption_hash(&self) -> ArtifactId {
        self.assumption_hash
    }

    /// Lean-shaped type of the obligation.
    pub fn lean_type(&self) -> &str {
        &self.lean_type
    }

    /// Algebraic identity that must be the zero polynomial, when the exact
    /// backend applies.
    pub fn identity(&self) -> Option<&Expr> {
        self.identity.as_ref()
    }

    /// Axiom ids the receipt must list (and only these).
    pub fn axioms(&self) -> &[String] {
        &self.axioms
    }

    /// Hash of the canonical challenge bytes.
    pub fn challenge_hash(&self) -> ArtifactId {
        self.challenge_hash
    }

    /// Build a challenge from a formal claim. Exact identities are looked up
    /// by FormalClaim identity, not by slug. A matching slug with different
    /// commitments is not a catalog obligation and cannot be promoted by
    /// the exact backend.
    pub fn generate(claim: &FormalClaim) -> Self {
        let spec = catalog::lookup_matching(claim);
        let (lean_type, identity, axioms) = match spec {
            Some(s) => (
                s.lean_type.to_string(),
                Some((s.identity)()),
                s.axioms.iter().map(|a| (*a).to_string()).collect(),
            ),
            None => (
                format!("-- uninterpreted Prop\n-- {}", claim.statement),
                None,
                Vec::new(),
            ),
        };
        let statement_hash = claim.statement_hash;
        let assumption_hash = claim.assumptions.0;
        let challenge_hash = ArtifactId::of(Self::canonical_bytes(
            &claim.id.0,
            statement_hash,
            assumption_hash,
            &lean_type,
            identity.as_ref(),
            &axioms,
        ));
        Self {
            claim_id: claim.id.0.clone(),
            statement_hash,
            assumption_hash,
            lean_type,
            identity,
            axioms,
            challenge_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use physis_core::assumption::{AssumptionSet, DomainOfValidity};
    use physis_core::assurance::ClaimClass;
    use physis_core::formal::FormalClaim;
    use physis_core::id::{ClaimId, LayerId};

    use super::*;

    fn claim_with(id: &str, stmt: &str, commitments: physis_core::ClaimCommitments) -> FormalClaim {
        let assumptions = AssumptionSet::encoding_internal();
        let domain = DomainOfValidity::encoding_wide();
        let statement_hash = ArtifactId::of(FormalClaim::canonical_bytes(
            id,
            stmt,
            ClaimClass::Mathematical,
            LayerId::Mathematical,
            &assumptions,
            &domain,
            &commitments,
        ));
        FormalClaim {
            id: ClaimId::new(id),
            statement: stmt.into(),
            statement_hash,
            assumptions: assumptions.id,
            domain,
            class: ClaimClass::Mathematical,
            layer: LayerId::Mathematical,
            commitments,
        }
    }

    fn claim(id: &str, stmt: &str) -> FormalClaim {
        claim_with(id, stmt, physis_core::ClaimCommitments::unspecified())
    }

    #[test]
    fn mutating_the_lean_type_changes_the_challenge_hash() {
        let c = Challenge::generate(&claim(
            "dec.d-squared-zero",
            "The exterior derivative is nilpotent: d ∘ d = 0.",
        ));
        assert!(c.hash_is_consistent());
        let mut mutated = c.clone();
        mutated.lean_type.push('x');
        assert!(!mutated.hash_is_consistent());
    }

    #[test]
    fn forall_claim_is_not_the_exists_claim() {
        let a = claim("math.example", "forall x, P(x)");
        let b = claim("math.example", "exists x, P(x)");
        assert_ne!(
            Challenge::generate(&a).statement_hash(),
            Challenge::generate(&b).statement_hash()
        );
    }

    #[test]
    fn physlib_forall_is_not_the_unspecified_d2_challenge() {
        let stmt = "The exterior derivative is nilpotent: d ∘ d = 0.";
        let a = Challenge::generate(&claim("dec.d-squared-zero", stmt));
        let b = Challenge::generate(&claim_with(
            "dec.d-squared-zero",
            stmt,
            physis_core::ClaimCommitments::physlib_forall(),
        ));
        assert_ne!(a.statement_hash(), b.statement_hash());
        assert_ne!(a.challenge_hash(), b.challenge_hash());
        assert!(
            a.identity().is_none(),
            "unspecified d² is not the catalog obligation"
        );
        assert!(
            b.identity().is_some(),
            "physlib forall d² is the catalog obligation"
        );
    }
}
