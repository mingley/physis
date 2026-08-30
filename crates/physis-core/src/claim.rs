//! Claims, orthogonal assurance, and executable verdicts.
//!
//! A claim is a sentence a theory makes. A verdict is what happens when
//! that sentence is checked against the theory's current knobs.
//!
//! Verdicts are not "truth of the universe". They are *internal*:
//! consistency with the model as encoded. Empirical contact is a separate
//! axis ([`EmpiricalStatus`]) and requires a dataset receipt to rise
//! above `Untested`.
//!
//! [`DerivationAssurance::Executed`] means the evaluator ran. It is **not**
//! a kernel proof. There is no `Epistemic::Theorem` tag: that name mixed
//! “proven in this model” with “a standard theorem encoded as such,” and
//! it was forgeable by setting an enum.

use serde::{Deserialize, Serialize};

use crate::artifact::ArtifactId;
use crate::assumption::{AssumptionSet, DomainOfValidity};
use crate::formal::FormalClaim;
use crate::id::{ClaimId, LayerId};

pub use crate::assurance::{ClaimClass, DerivationAssurance, EmpiricalStatus, SemanticAssurance};

/// Four-way executable judgment (Level-2).
///
/// Typed Level-3 judgments (`Logical`, `Numeric`, `Empirical`, …) arrive
/// with the proof and data pipelines. Until then this is the deterministic
/// evaluator result, *not* a theorem status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VerdictKind {
    /// The claim is satisfied by the current knobs.
    Holds,
    /// The claim is violated; evidence should contain a counterexample.
    Fails,
    /// The encoding cannot decide (missing math, missing data).
    Undecidable,
    /// The claim does not apply to this theory.
    Inapplicable,
}

impl VerdictKind {
    /// Stable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            VerdictKind::Holds => "holds",
            VerdictKind::Fails => "fails",
            VerdictKind::Undecidable => "undecidable",
            VerdictKind::Inapplicable => "inapplicable",
        }
    }
}

/// Result of evaluating a claim.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Verdict {
    /// Holds / fails / …
    pub kind: VerdictKind,
    /// What kind of sentence this was.
    pub class: ClaimClass,
    /// How the deduction was checked. Never a kernel proof.
    pub derivation: DerivationAssurance,
    /// What observation says (usually `NotApplicable` / `Untested` in M1).
    pub empirical: EmpiricalStatus,
    /// Whether the encoding has been reviewed.
    pub semantic: SemanticAssurance,
    /// One-line reason.
    pub summary: String,
    /// Structured notes (numbers, mismatched knobs, citations).
    pub evidence: Vec<String>,
}

impl Verdict {
    /// Copy assurance fields from the claim being evaluated.
    pub fn from_claim(
        kind: VerdictKind,
        claim: &Claim,
        summary: impl Into<String>,
        evidence: Vec<String>,
    ) -> Self {
        Self {
            kind,
            class: claim.class,
            derivation: claim.derivation,
            empirical: claim.empirical,
            semantic: claim.semantic,
            summary: summary.into(),
            evidence,
        }
    }

    /// Holds under the claim's own assurance tags.
    pub fn holds(claim: &Claim, summary: impl Into<String>) -> Self {
        Self::from_claim(VerdictKind::Holds, claim, summary, Vec::new())
    }

    /// Fails under the claim's own assurance tags.
    pub fn fails(claim: &Claim, summary: impl Into<String>) -> Self {
        Self::from_claim(VerdictKind::Fails, claim, summary, Vec::new())
    }

    /// Does not apply.
    pub fn inapplicable(claim: &Claim, summary: impl Into<String>) -> Self {
        Self::from_claim(VerdictKind::Inapplicable, claim, summary, Vec::new())
    }

    /// Cannot decide.
    pub fn undecidable(claim: &Claim, summary: impl Into<String>) -> Self {
        Self::from_claim(VerdictKind::Undecidable, claim, summary, Vec::new())
    }

    /// Attach evidence lines.
    pub fn with_evidence(mut self, lines: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.evidence.extend(lines.into_iter().map(Into::into));
        self
    }

    /// Refine the claim class of this verdict. Cannot mint a kernel proof.
    pub fn with_class(mut self, class: ClaimClass) -> Self {
        self.class = class;
        self.derivation = class.default_derivation();
        self.empirical = class.default_empirical();
        self
    }
}

/// A sentence a theory is willing to be judged on.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Claim {
    /// Stable id, `theory.slug`.
    pub id: ClaimId,
    /// English statement.
    pub statement: String,
    /// Layer the statement is about.
    pub layer: LayerId,
    /// What kind of sentence this is.
    pub class: ClaimClass,
    /// Derivation assurance. Constructors never produce a kernel proof.
    pub derivation: DerivationAssurance,
    /// Empirical axis.
    pub empirical: EmpiricalStatus,
    /// Encoding-review axis. Defaults to [`SemanticAssurance::Unreviewed`].
    pub semantic: SemanticAssurance,
    /// Explicit assumptions (never empty: the encoding-internal default).
    pub assumptions: AssumptionSet,
    /// Domain of validity.
    pub domain: DomainOfValidity,
    /// Content address of the formal identity.
    pub statement_hash: ArtifactId,
}

impl Claim {
    /// Construct a claim. Derivation/empirical default from `class`.
    /// Semantic assurance is [`SemanticAssurance::Unreviewed`].
    /// Assumptions are [`AssumptionSet::encoding_internal`].
    pub fn new(
        id: impl Into<String>,
        statement: impl Into<String>,
        layer: LayerId,
        class: ClaimClass,
    ) -> Self {
        let id = ClaimId::new(id);
        let statement = statement.into();
        let assumptions = AssumptionSet::encoding_internal();
        let domain = DomainOfValidity::encoding_wide();
        let statement_hash = ArtifactId::of(FormalClaim::canonical_bytes(
            &id.0,
            &statement,
            class,
            layer,
            &assumptions,
            &domain,
        ));
        Self {
            id,
            statement,
            layer,
            class,
            derivation: class.default_derivation(),
            empirical: class.default_empirical(),
            semantic: SemanticAssurance::Unreviewed,
            assumptions,
            domain,
            statement_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_internal_is_executed_not_a_kernel_proof() {
        let c = Claim::new(
            "math.d2",
            "d² = 0",
            LayerId::Mathematical,
            ClaimClass::Mathematical,
        );
        assert_eq!(c.derivation, DerivationAssurance::Executed);
        assert_eq!(c.semantic, SemanticAssurance::Unreviewed);
        assert!(!c.assumptions.items.is_empty());
        let v = Verdict::holds(&c, "evaluator ran");
        assert_eq!(v.derivation, DerivationAssurance::Executed);
        assert_eq!(v.class, ClaimClass::Mathematical);
    }

    #[test]
    fn changing_the_statement_changes_the_hash() {
        let a = Claim::new(
            "rel.interval",
            "Lorentz boosts preserve s²",
            LayerId::Spacetime,
            ClaimClass::ModelInternal,
        );
        let b = Claim::new(
            "rel.interval",
            "Lorentz boosts preserve s² up to a sign",
            LayerId::Spacetime,
            ClaimClass::ModelInternal,
        );
        assert_ne!(a.statement_hash, b.statement_hash);
    }

    #[test]
    fn conjecture_stays_asserted() {
        let c = Claim::new(
            "predictivity.unique-vacuum",
            "the vacuum is unique",
            LayerId::Effective,
            ClaimClass::Conjecture,
        );
        assert_eq!(c.derivation, DerivationAssurance::Asserted);
    }
}
