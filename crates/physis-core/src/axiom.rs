//! Axiom ledger: every trusted deduction must name its transitive hypotheses.
//!
//! An agent must not be able to insert `axiom answer_is_true` as an
//! implementation detail. A new physical postulate is a scientific change.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::artifact::ArtifactId;

/// Stable axiom identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AxiomId(pub String);

impl AxiomId {
    /// Construct from a stable name.
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

/// Kind of hypothesis. Physical postulates are not “code.”
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AxiomClass {
    /// Logical / inference rules of the kernel.
    Logical,
    /// Foundational mathematics (not physics).
    MathematicalFoundational,
    /// A law of nature taken as primitive in this encoding.
    PhysicalPostulate,
    /// An assumption of the model (flat space, equilibrium, …).
    ModelAssumption,
    /// An effective-theory restriction (cutoff, leading order, …).
    EffectiveAssumption,
    /// An empirical input treated as a boundary condition.
    EmpiricalInput,
}

impl AxiomClass {
    /// Stable kebab-case name.
    pub const fn as_str(self) -> &'static str {
        match self {
            AxiomClass::Logical => "logical",
            AxiomClass::MathematicalFoundational => "mathematical-foundational",
            AxiomClass::PhysicalPostulate => "physical-postulate",
            AxiomClass::ModelAssumption => "model-assumption",
            AxiomClass::EffectiveAssumption => "effective-assumption",
            AxiomClass::EmpiricalInput => "empirical-input",
        }
    }
}

/// Review state of an axiom record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReviewStatus {
    /// Proposed; not accepted into trusted modules.
    Unreviewed,
    /// Accepted for use in this encoding, with provenance.
    Accepted,
    /// Explicitly rejected; must not appear in trusted closures.
    Rejected,
}

impl ReviewStatus {
    /// Stable kebab-case name.
    pub const fn as_str(self) -> &'static str {
        match self {
            ReviewStatus::Unreviewed => "unreviewed",
            ReviewStatus::Accepted => "accepted",
            ReviewStatus::Rejected => "rejected",
        }
    }
}

/// One ledger entry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AxiomRecord {
    /// Stable id.
    pub id: AxiomId,
    /// Classification.
    pub class: AxiomClass,
    /// Hash of the formal statement bytes (when a formalization exists).
    pub formal_statement_hash: Option<ArtifactId>,
    /// Human provenance (citation class, not a proof).
    pub provenance: String,
    /// Review.
    pub review_status: ReviewStatus,
}

/// Global-shaped map of axioms. M1 starts empty of kernel axioms; model
/// assumptions live on claims as [`crate::assumption::AssumptionSet`].
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AxiomLedger {
    records: BTreeMap<AxiomId, AxiomRecord>,
}

impl AxiomLedger {
    /// No axioms registered.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Lookup.
    pub fn get(&self, id: &AxiomId) -> Option<&AxiomRecord> {
        self.records.get(id)
    }

    /// All records, ordered by id.
    pub fn iter(&self) -> impl Iterator<Item = &AxiomRecord> {
        self.records.values()
    }

    /// Propose an axiom as unreviewed. This is a scientific change, not a
    /// silent insert of `axiom answer_is_true`.
    pub fn propose(&mut self, mut record: AxiomRecord) {
        record.review_status = ReviewStatus::Unreviewed;
        self.records.insert(record.id.clone(), record);
    }

    /// Number of registered axioms.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// True when nothing is registered.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// In-tree Lean kernel axioms and catalog postulates. Agents cannot
    /// mark these Accepted through [`Self::propose`].
    pub fn physis_defaults() -> Self {
        let mut l = Self::empty();
        let builtins: &[(&str, AxiomClass, &str)] = &[
            (
                "propext",
                AxiomClass::Logical,
                "Lean kernel: propositional extensionality",
            ),
            (
                "Quot.sound",
                AxiomClass::Logical,
                "Lean kernel: quotient soundness",
            ),
            (
                "Classical.choice",
                AxiomClass::Logical,
                "Lean kernel: choice",
            ),
            (
                "integer-arithmetic",
                AxiomClass::MathematicalFoundational,
                "Integer ring axioms used by the exact and Lean backends",
            ),
            (
                "discrete-coboundary",
                AxiomClass::ModelAssumption,
                "Oriented simplex coboundary in the discrete exterior calculus encoding",
            ),
            (
                "minkowski-interval-signature",
                AxiomClass::PhysicalPostulate,
                "Minkowski signature (+,-,-,-) at c = 1, polynomial form",
            ),
        ];
        for (id, class, provenance) in builtins {
            l.records.insert(
                AxiomId::new(*id),
                AxiomRecord {
                    id: AxiomId::new(*id),
                    class: *class,
                    formal_statement_hash: None,
                    provenance: (*provenance).into(),
                    review_status: ReviewStatus::Accepted,
                },
            );
        }
        l
    }

    /// Look up each id. Missing entries stay visible; they are not invented.
    pub fn closure<'a>(
        &'a self,
        ids: &'a [AxiomId],
    ) -> Vec<(&'a AxiomId, Option<&'a AxiomRecord>)> {
        ids.iter().map(|id| (id, self.records.get(id))).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn propose_is_always_unreviewed() {
        let mut ledger = AxiomLedger::empty();
        ledger.propose(AxiomRecord {
            id: AxiomId::new("answer-is-true"),
            class: AxiomClass::PhysicalPostulate,
            formal_statement_hash: None,
            provenance: "agent wish".into(),
            review_status: ReviewStatus::Accepted,
        });
        let rec = ledger.get(&AxiomId::new("answer-is-true")).unwrap();
        assert_eq!(rec.review_status, ReviewStatus::Unreviewed);
        ledger.propose(AxiomRecord {
            id: AxiomId::new("propext"),
            class: AxiomClass::Logical,
            formal_statement_hash: None,
            provenance: "agent overwrite".into(),
            review_status: ReviewStatus::Accepted,
        });
        assert_eq!(
            ledger.get(&AxiomId::new("propext")).unwrap().review_status,
            ReviewStatus::Unreviewed
        );
    }

    #[test]
    fn defaults_name_lean_and_catalog_axioms() {
        let l = AxiomLedger::physis_defaults();
        assert_eq!(
            l.get(&AxiomId::new("propext")).unwrap().class,
            AxiomClass::Logical
        );
        assert_eq!(
            l.get(&AxiomId::new("discrete-coboundary"))
                .unwrap()
                .review_status,
            ReviewStatus::Accepted
        );
        let missing = AxiomId::new("not-a-real-axiom");
        let c = l.closure(std::slice::from_ref(&missing));
        assert!(c[0].1.is_none());
    }
}
