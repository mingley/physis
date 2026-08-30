//! Immutable proof challenge. Generated on the trusted side from a
//! [`physis_core::FormalClaim`]. The solver never chooses the statement.

use physis_core::artifact::ArtifactId;
use physis_core::formal::FormalClaim;
use serde::{Deserialize, Serialize};

use crate::catalog;
use crate::expr::Expr;

/// Trusted challenge a candidate proof is judged against.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Challenge {
    /// Lab claim id.
    pub claim_id: String,
    /// Hash of the formal claim identity (statement, assumptions, domain, …).
    pub statement_hash: ArtifactId,
    /// Hash of the assumption set.
    pub assumption_hash: ArtifactId,
    /// Lean-shaped type of the obligation.
    pub lean_type: String,
    /// Algebraic identity that must be the zero polynomial, when the exact
    /// backend applies.
    pub identity: Option<Expr>,
    /// Axiom ids the receipt must list (and only these).
    pub axioms: Vec<String>,
    /// Hash of the canonical challenge bytes. A one-byte mutation of the
    /// theorem statement changes this.
    pub challenge_hash: ArtifactId,
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

    /// Build a challenge from a formal claim. Exact identities are looked up
    /// in the catalog; other claims still get a Lean type (the English
    /// statement as a Prop) but cannot be promoted by the exact backend.
    pub fn generate(claim: &FormalClaim) -> Self {
        let spec = catalog::lookup(&claim.id.0);
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
        let mut mutated = c.clone();
        mutated.lean_type = c.lean_type.replace("= 0", "= 1");
        let h = ArtifactId::of(Challenge::canonical_bytes(
            &mutated.claim_id,
            mutated.statement_hash,
            mutated.assumption_hash,
            &mutated.lean_type,
            mutated.identity.as_ref(),
            &mutated.axioms,
        ));
        assert_ne!(c.challenge_hash, h);
    }

    #[test]
    fn forall_claim_is_not_the_exists_claim() {
        let a = claim("math.example", "forall x, P(x)");
        let b = claim("math.example", "exists x, P(x)");
        assert_ne!(
            Challenge::generate(&a).statement_hash,
            Challenge::generate(&b).statement_hash
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
        assert_ne!(a.statement_hash, b.statement_hash);
        assert_ne!(a.challenge_hash, b.challenge_hash);
    }
}
