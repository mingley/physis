//! Immutable formal claim identity.
//!
//! A proof generated for one statement must not attach to another.
//! [`FormalClaim::statement_hash`] commits to the sentence, class, layer,
//! assumptions, and domain. Changing ∀ to ∃, a sign, a unit, or a
//! boundary condition yields a new hash.

use serde::{Deserialize, Serialize};

use crate::artifact::ArtifactId;
use crate::assumption::{AssumptionSet, AssumptionSetId, DomainOfValidity};
use crate::assurance::ClaimClass;
use crate::claim::Claim;
use crate::id::{ClaimId, LayerId};

/// Immutable identity of a scientific sentence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormalClaim {
    /// Lab claim id (`astro.sky-finite`, …).
    pub id: ClaimId,
    /// The sentence as encoded today (English until physis-ir exists).
    pub statement: String,
    /// Hash of the canonical identity bytes.
    pub statement_hash: ArtifactId,
    /// Assumption-set id.
    pub assumptions: AssumptionSetId,
    /// Domain of validity.
    pub domain: DomainOfValidity,
    /// Claim class.
    pub class: ClaimClass,
    /// Layer.
    pub layer: LayerId,
}

impl FormalClaim {
    /// Canonical bytes the statement hash commits to.
    pub fn canonical_bytes(
        id: &str,
        statement: &str,
        class: ClaimClass,
        layer: LayerId,
        assumptions: &AssumptionSet,
        domain: &DomainOfValidity,
    ) -> Vec<u8> {
        let mut s = String::new();
        s.push_str("id:");
        s.push_str(id);
        s.push('\n');
        s.push_str("statement:");
        s.push_str(statement);
        s.push('\n');
        s.push_str("class:");
        s.push_str(class.as_str());
        s.push('\n');
        s.push_str("layer:");
        s.push_str(layer.as_str());
        s.push('\n');
        s.push_str("assumptions:");
        s.push_str(&assumptions.id.0.to_hex());
        s.push('\n');
        s.push_str("domain:");
        s.push_str(&domain.id.to_hex());
        s.push('\n');
        s.into_bytes()
    }

    /// Identity of an executable lab claim.
    pub fn from_claim(claim: &Claim) -> Self {
        Self {
            id: claim.id.clone(),
            statement: claim.statement.clone(),
            statement_hash: claim.statement_hash,
            assumptions: claim.assumptions.id.clone(),
            domain: claim.domain.clone(),
            class: claim.class,
            layer: claim.layer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::id::LayerId;

    #[test]
    fn forall_to_exists_is_a_new_identity() {
        let assumptions = AssumptionSet::encoding_internal();
        let domain = DomainOfValidity::encoding_wide();
        let a = ArtifactId::of(FormalClaim::canonical_bytes(
            "math.example",
            "forall x, P(x)",
            ClaimClass::Mathematical,
            LayerId::Mathematical,
            &assumptions,
            &domain,
        ));
        let b = ArtifactId::of(FormalClaim::canonical_bytes(
            "math.example",
            "exists x, P(x)",
            ClaimClass::Mathematical,
            LayerId::Mathematical,
            &assumptions,
            &domain,
        ));
        assert_ne!(a, b);
    }
}
