//! First-class assumption sets and domains of validity.
//!
//! Hidden assumptions are bugs. Every claim carries an [`AssumptionSet`]
//! and a [`DomainOfValidity`]. The M1 defaults are *explicit placeholders*
//! for encodings that have not yet listed every physical hypothesis; they
//! are not a licence to treat those encodings as axiom-free.

use serde::{Deserialize, Serialize};

use crate::artifact::ArtifactId;
use crate::axiom::AxiomClass;

/// Stable id for an assumption set (content-addressed).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssumptionSetId(pub ArtifactId);

/// One explicit hypothesis.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assumption {
    /// Short stable name (`flat-spacetime`, `encoding-is-the-model`, …).
    pub id: String,
    /// What is being assumed.
    pub statement: String,
    /// How this assumption is classified in the axiom ledger.
    pub class: AxiomClass,
}

/// A named bundle of assumptions a claim depends on.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssumptionSet {
    /// Content address of the canonical listing.
    pub id: AssumptionSetId,
    /// Members, in declaration order.
    pub items: Vec<Assumption>,
}

impl AssumptionSet {
    /// Build from items; the id hashes the canonical listing.
    pub fn new(items: Vec<Assumption>) -> Self {
        let mut canonical = String::new();
        for a in &items {
            canonical.push_str(&a.id);
            canonical.push('\n');
            canonical.push_str(&a.statement);
            canonical.push('\n');
            canonical.push_str(a.class.as_str());
            canonical.push('\n');
        }
        Self {
            id: AssumptionSetId(ArtifactId::of(canonical.as_bytes())),
            items,
        }
    }

    /// Honest default: the claim is evaluated inside the encoding.
    ///
    /// This is *not* an empty assumption set. It records that Level-2
    /// execution is the current assurance, not a kernel proof.
    pub fn encoding_internal() -> Self {
        Self::new(vec![Assumption {
            id: "encoding-is-the-model".into(),
            statement: "The claim is judged against this theory's encoded \
                 equations and knobs. That is a model-internal execution, \
                 not a kernel-checked deduction from independently audited \
                 axioms."
                .into(),
            class: AxiomClass::ModelAssumption,
        }])
    }
}

/// Machine-readable scope. Extrapolating outside it is a new claim.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainOfValidity {
    /// Named regime constraints (`v << c`, `T >> Theta_D`, …).
    pub regimes: Vec<String>,
    /// Approximations in force.
    pub approximations: Vec<String>,
    /// Free-text honesty about what is not yet constrained.
    pub notes: String,
    /// Content address of the canonical listing.
    pub id: ArtifactId,
}

impl DomainOfValidity {
    /// Build from parts; id hashes the canonical listing.
    pub fn new(
        regimes: Vec<String>,
        approximations: Vec<String>,
        notes: impl Into<String>,
    ) -> Self {
        let notes = notes.into();
        let mut canonical = String::new();
        for r in &regimes {
            canonical.push_str("regime:");
            canonical.push_str(r);
            canonical.push('\n');
        }
        for a in &approximations {
            canonical.push_str("approx:");
            canonical.push_str(a);
            canonical.push('\n');
        }
        canonical.push_str("notes:");
        canonical.push_str(&notes);
        Self {
            regimes,
            approximations,
            notes,
            id: ArtifactId::of(canonical.as_bytes()),
        }
    }

    /// Placeholder: validity is “whatever the current knobs encode.”
    pub fn encoding_wide() -> Self {
        Self::new(
            vec![],
            vec!["evaluated at the theory's current knobs".into()],
            "Domain of validity is not yet a machine-checked regime. \
             Using a result outside encoded knobs is a new claim, not a \
             silent extrapolation.",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_an_assumption_changes_the_set_id() {
        let a = AssumptionSet::encoding_internal();
        let mut items = a.items.clone();
        items.push(Assumption {
            id: "locality".into(),
            statement: "interactions are local".into(),
            class: AxiomClass::PhysicalPostulate,
        });
        let b = AssumptionSet::new(items);
        assert_ne!(a.id, b.id);
    }
}
