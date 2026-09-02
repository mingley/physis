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
//!
//! [`Claim::statement_hash`] is derived from the live sentence. JSON cannot
//! mint a stored hash, and a public field cannot rebind a kernel receipt:
//!
//! ```compile_fail
//! fn needs_deserialize<'de, T: serde::Deserialize<'de>>() {}
//! fn _blocked() {
//!     needs_deserialize::<physis_core::claim::Claim>();
//! }
//! ```
//!
//! ```compile_fail
//! let mut c = physis_core::claim::Claim::new(
//!     "x",
//!     "y",
//!     physis_core::LayerId::Mathematical,
//!     physis_core::ClaimClass::Mathematical,
//! );
//! c.statement_hash = physis_core::ArtifactId::of(b"forged");
//! ```
//!
//! ```compile_fail
//! let mut c = physis_core::claim::Claim::new(
//!     "x",
//!     "y",
//!     physis_core::LayerId::Mathematical,
//!     physis_core::ClaimClass::Mathematical,
//! );
//! c.statement.push_str(" forged");
//! ```
//!
//! JSON cannot mint a [`Verdict`] either (`certified-numeric` Holds is not
//! a deserializable overlay):
//!
//! ```compile_fail
//! fn needs_deserialize<'de, T: serde::Deserialize<'de>>() {}
//! fn _blocked() {
//!     needs_deserialize::<physis_core::claim::Verdict>();
//! }
//! ```

use serde::{Deserialize, Serialize};

use crate::artifact::ArtifactId;
use crate::assumption::{AssumptionSet, DomainOfValidity};
use crate::formal::{ClaimCommitments, FormalClaim};
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
///
/// There is no [`serde::Deserialize`] impl: JSON cannot mint a
/// `certified-numeric` overlay or an `adversarially-reviewed` tag.
/// Theories construct verdicts with [`Verdict::from_claim`] and the
/// overlay builders (`with_certified_numeric`, `with_interval_enclosure`,
/// `with_cross_checked`, `with_empirical`, `with_statistical_nll`).
/// Derivation, empirical, semantic, enclosure, and NLL fields are private:
/// a public assignment cannot mint those overlays.
///
/// ```compile_fail
/// let c = physis_core::claim::Claim::new(
///     "x",
///     "y",
///     physis_core::LayerId::Mathematical,
///     physis_core::ClaimClass::Mathematical,
/// );
/// let mut v = physis_core::claim::Verdict::holds(&c, "ran");
/// v.derivation = physis_core::DerivationAssurance::CertifiedNumeric;
/// ```
///
/// ```compile_fail
/// let c = physis_core::claim::Claim::new(
///     "x",
///     "y",
///     physis_core::LayerId::Mathematical,
///     physis_core::ClaimClass::Mathematical,
/// );
/// let mut v = physis_core::claim::Verdict::holds(&c, "ran");
/// v.statistical_nll = Some("1".into());
/// ```
///
/// ```compile_fail
/// let c = physis_core::claim::Claim::new(
///     "x",
///     "y",
///     physis_core::LayerId::Mathematical,
///     physis_core::ClaimClass::Mathematical,
/// );
/// let mut v = physis_core::claim::Verdict::holds(&c, "ran");
/// v.numeric_lo = Some("1".into());
/// ```
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Verdict {
    /// Holds / fails / …
    pub kind: VerdictKind,
    /// What kind of sentence this was.
    pub class: ClaimClass,
    derivation: DerivationAssurance,
    empirical: EmpiricalStatus,
    semantic: SemanticAssurance,
    /// One-line reason.
    pub summary: String,
    /// Structured notes (numbers, mismatched knobs, citations).
    pub evidence: Vec<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    intractable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    numeric_lo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    numeric_hi: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    statistical_nll: Option<String>,
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
            class: claim.class(),
            derivation: claim.derivation(),
            empirical: claim.empirical(),
            semantic: claim.semantic(),
            summary: summary.into(),
            evidence,
            intractable: false,
            numeric_lo: None,
            numeric_hi: None,
            statistical_nll: None,
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

    /// How the deduction was checked. Never a kernel proof.
    pub const fn derivation(&self) -> DerivationAssurance {
        self.derivation
    }

    /// Empirical axis after any dataset overlay.
    pub const fn empirical(&self) -> EmpiricalStatus {
        self.empirical
    }

    /// Encoding-review tag copied from the claim at construction. P3S is a
    /// review-store tag, not this field.
    pub const fn semantic(&self) -> SemanticAssurance {
        self.semantic
    }

    /// True when an undecidable evaluation is a resource bound.
    pub const fn intractable(&self) -> bool {
        self.intractable
    }

    /// Display lower bound of a numeric certificate, when present.
    pub fn numeric_lo(&self) -> Option<&str> {
        self.numeric_lo.as_deref()
    }

    /// Display upper bound of a numeric certificate, when present.
    pub fn numeric_hi(&self) -> Option<&str> {
        self.numeric_hi.as_deref()
    }

    /// Exact Gaussian NLL overlay, when present. Not a kernel proof.
    pub fn statistical_nll(&self) -> Option<&str> {
        self.statistical_nll.as_deref()
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

    /// Overlay the empirical axis after a dataset receipt. Cannot mint a
    /// kernel proof.
    pub fn with_empirical(mut self, empirical: EmpiricalStatus) -> Self {
        self.empirical = empirical;
        self
    }

    /// Mark an undecidable evaluation as a resource bound, not a missing
    /// algorithm. Does not mint a kernel proof.
    pub fn with_intractable(mut self) -> Self {
        self.intractable = true;
        self
    }

    /// Overlay exact-ratio / interval-certificate assurance, with the
    /// certified enclosure as display strings (`lo == hi` for a `Ratio`
    /// identity). Does not mint a kernel proof and is not P4.
    pub fn with_certified_numeric(mut self, lo: impl Into<String>, hi: impl Into<String>) -> Self {
        debug_assert_eq!(self.kind, VerdictKind::Holds);
        self.derivation = DerivationAssurance::CertifiedNumeric;
        self.numeric_lo = Some(lo.into());
        self.numeric_hi = Some(hi.into());
        self
    }

    /// Overlay independently parseable interval endpoints without
    /// assigning [`DerivationAssurance::CertifiedNumeric`]. Does not mint
    /// P3N, a kernel proof, Canonical, or P4. The overlay is not the
    /// certificate: an independent `[lo, hi]` Ratio parse is the check.
    pub fn with_interval_enclosure(mut self, lo: impl Into<String>, hi: impl Into<String>) -> Self {
        self.numeric_lo = Some(lo.into());
        self.numeric_hi = Some(hi.into());
        self
    }

    /// Overlay two-path agreement. Does not mint a kernel proof and is not P4.
    pub fn with_cross_checked(mut self) -> Self {
        debug_assert_eq!(self.kind, VerdictKind::Holds);
        self.derivation = DerivationAssurance::CrossChecked;
        self
    }

    /// Overlay an exact Gaussian NLL. Does not mint a kernel proof, is not
    /// P3N, and is not P4. Only [`crate::judgment::Judgment::from_lab`]
    /// projects `statistical computed`, and only for empirical/measurement
    /// classes.
    pub fn with_statistical_nll(mut self, nll: impl std::fmt::Display) -> Self {
        self.statistical_nll = Some(nll.to_string());
        self
    }
}

/// A sentence a theory is willing to be judged on.
///
/// [`Self::statement_hash`] is derived from the live sentence, class, layer,
/// assumptions, domain, and commitments. There is no stored hash field and
/// no [`serde::Deserialize`] impl: JSON cannot mint a catalog identity.
/// The English statement is private: a public assignment cannot rebind a
/// kernel receipt, and same-module mutation still cannot keep a stale hash.
/// Class, layer, assumptions, domain, and commitments are private too:
/// overlays are [`Self::with_commitments`], [`Self::with_domain`], and
/// [`Self::with_assumptions`]. The slug is private ([`Self::id`] /
/// [`Self::id_str`]); lemma edges stay public.
///
/// Derivation, empirical, and semantic axes are private: a public field
/// cannot mint [`DerivationAssurance::CertifiedNumeric`] or an
/// encoding-review tag. Those overlays live on [`Verdict`].
///
/// ```compile_fail
/// let mut c = physis_core::claim::Claim::new(
///     "x",
///     "y",
///     physis_core::LayerId::Mathematical,
///     physis_core::ClaimClass::Mathematical,
/// );
/// c.derivation = physis_core::DerivationAssurance::CertifiedNumeric;
/// ```
///
/// ```compile_fail
/// let mut c = physis_core::claim::Claim::new(
///     "x",
///     "y",
///     physis_core::LayerId::Mathematical,
///     physis_core::ClaimClass::Mathematical,
/// );
/// c.semantic = physis_core::SemanticAssurance::AdversariallyReviewed;
/// ```
///
/// ```compile_fail
/// let mut c = physis_core::claim::Claim::new(
///     "x",
///     "y",
///     physis_core::LayerId::Mathematical,
///     physis_core::ClaimClass::Mathematical,
/// );
/// c.statement.push_str(" forged");
/// ```
///
/// ```compile_fail
/// let mut c = physis_core::claim::Claim::new(
///     "x",
///     "y",
///     physis_core::LayerId::Mathematical,
///     physis_core::ClaimClass::Mathematical,
/// );
/// c.class = physis_core::ClaimClass::Conjecture;
/// ```
///
/// ```compile_fail
/// let mut c = physis_core::claim::Claim::new(
///     "x",
///     "y",
///     physis_core::LayerId::Mathematical,
///     physis_core::ClaimClass::Mathematical,
/// );
/// c.id = physis_core::ClaimId::new("forged");
/// ```
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Claim {
    id: ClaimId,
    statement: String,
    layer: LayerId,
    class: ClaimClass,
    /// Derivation assurance. Constructors never produce a kernel proof.
    derivation: DerivationAssurance,
    /// Empirical axis.
    empirical: EmpiricalStatus,
    /// Encoding-review axis. Defaults to [`SemanticAssurance::Unreviewed`].
    semantic: SemanticAssurance,
    assumptions: AssumptionSet,
    domain: DomainOfValidity,
    commitments: ClaimCommitments,
    /// Lemma ids this claim uses. Not part of [`Self::statement_hash`]: a lab
    /// encoding of "this uses that" is not a change to the sentence.
    pub depends_on: Vec<ClaimId>,
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
        let commitments = ClaimCommitments::unspecified();
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
            commitments,
            depends_on: Vec::new(),
        }
    }

    /// Content address of the live formal identity. Always hashed from
    /// current fields: mutating the sentence cannot keep a stale catalog
    /// hash attached to a kernel receipt.
    pub fn statement_hash(&self) -> ArtifactId {
        ArtifactId::of(FormalClaim::canonical_bytes(
            self.id_str(),
            self.statement(),
            self.class(),
            self.layer(),
            self.assumptions(),
            self.domain(),
            self.commitments(),
        ))
    }

    /// The sentence as encoded today. Private so a public assignment cannot
    /// rebind a kernel receipt to different English.
    pub fn statement(&self) -> &str {
        &self.statement
    }

    /// Stable lab slug (`theory.slug`). Private so a public assignment cannot
    /// rebind the hashed identity to a different name.
    pub fn id(&self) -> &ClaimId {
        &self.id
    }

    /// Borrow the slug as a string.
    pub fn id_str(&self) -> &str {
        &self.id.0
    }

    /// Layer the statement is about.
    pub const fn layer(&self) -> LayerId {
        self.layer
    }

    /// What kind of sentence this is.
    pub const fn class(&self) -> ClaimClass {
        self.class
    }

    /// Explicit assumptions (never empty: the encoding-internal default).
    pub fn assumptions(&self) -> &AssumptionSet {
        &self.assumptions
    }

    /// Domain of validity. Extrapolating outside it is a new claim.
    pub fn domain(&self) -> &DomainOfValidity {
        &self.domain
    }

    /// First-class identity fields committed in [`Self::statement_hash`].
    pub fn commitments(&self) -> &ClaimCommitments {
        &self.commitments
    }

    /// How the deduction was tagged at construction. Never a kernel proof.
    /// [`DerivationAssurance::CertifiedNumeric`] is a [`Verdict`] overlay.
    pub const fn derivation(&self) -> DerivationAssurance {
        self.derivation
    }

    /// Empirical axis at construction. Dataset overlays live on [`Verdict`].
    pub const fn empirical(&self) -> EmpiricalStatus {
        self.empirical
    }

    /// Encoding-review axis at construction. Always
    /// [`SemanticAssurance::Unreviewed`]; P3S is a review-store tag.
    pub const fn semantic(&self) -> SemanticAssurance {
        self.semantic
    }

    /// Record lemma ids this claim uses. Does not change [`Self::statement_hash`].
    pub fn with_dependencies(mut self, lemmas: &[&str]) -> Self {
        self.depends_on = lemmas.iter().copied().map(ClaimId::new).collect();
        self
    }

    /// Overlay first-class identity fields. The slug [`ClaimId`] is unchanged;
    /// [`Self::statement_hash`] follows the new commitments.
    pub fn with_commitments(mut self, commitments: ClaimCommitments) -> Self {
        self.commitments = commitments;
        self
    }

    /// Overlay a domain of validity. Extrapolating outside it is a new claim,
    /// not a silent reuse; [`Self::statement_hash`] follows the new domain.
    pub fn with_domain(mut self, domain: DomainOfValidity) -> Self {
        self.domain = domain;
        self
    }

    /// Overlay the assumption set. Hidden hypotheses are a new identity;
    /// [`Self::statement_hash`] follows the new set.
    pub fn with_assumptions(mut self, assumptions: AssumptionSet) -> Self {
        self.assumptions = assumptions;
        self
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
        assert_eq!(c.derivation(), DerivationAssurance::Executed);
        assert_eq!(c.semantic(), SemanticAssurance::Unreviewed);
        assert!(!c.assumptions().items.is_empty());
        let v = Verdict::holds(&c, "evaluator ran");
        assert_eq!(v.derivation(), DerivationAssurance::Executed);
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
        assert_ne!(a.statement_hash(), b.statement_hash());
    }

    #[test]
    fn mutating_the_sentence_cannot_keep_a_stale_hash() {
        let mut c = Claim::new(
            "dec.d-squared-zero",
            "The exterior derivative is nilpotent: d ∘ d = 0.",
            LayerId::Mathematical,
            ClaimClass::Mathematical,
        );
        let honest = c.statement_hash();
        c.statement.push_str(" forged");
        assert_ne!(c.statement_hash(), honest);
        let formal = FormalClaim::from_claim(&c);
        assert_eq!(formal.statement_hash(), c.statement_hash());
        assert_eq!(formal.statement(), c.statement());
        assert_eq!(
            c.statement(),
            "The exterior derivative is nilpotent: d ∘ d = 0. forged"
        );
    }

    #[test]
    fn lemma_dependencies_are_not_statement_identity() {
        let a = Claim::new(
            "dec.closed-equals-exact",
            "Every closed 1-form is exact (the Poincaré lemma).",
            LayerId::Mathematical,
            ClaimClass::ModelInternal,
        );
        let b = a.clone().with_dependencies(&["dec.d-squared-zero"]);
        assert_eq!(a.statement_hash(), b.statement_hash());
        assert!(a.depends_on.is_empty());
        assert_eq!(b.depends_on[0].0, "dec.d-squared-zero");
    }

    #[test]
    fn commitments_change_the_hash_not_the_slug() {
        let a = Claim::new(
            "dec.d-squared-zero",
            "The exterior derivative is nilpotent: d ∘ d = 0.",
            LayerId::Mathematical,
            ClaimClass::Mathematical,
        );
        let b = a
            .clone()
            .with_commitments(ClaimCommitments::physlib_forall());
        assert_eq!(a.id(), b.id());
        assert_ne!(a.statement_hash(), b.statement_hash());
        assert_eq!(
            b.commitments().quantifier,
            crate::formal::Quantifier::ForAll
        );
    }

    #[test]
    fn domain_overlay_changes_the_hash() {
        let a = Claim::new(
            "field.second-order-accurate",
            "The discretization is second-order accurate (error ∝ a²).",
            LayerId::Field,
            ClaimClass::ModelInternal,
        );
        let b = a.clone().with_domain(DomainOfValidity::new(
            vec!["|k a| < 1 at the Richardson probe".into()],
            vec!["O(a^2) stencil at long wavelength".into()],
            "Outside |k a| < 1 the Richardson order is not a stencil verdict.",
        ));
        assert_eq!(a.id(), b.id());
        assert_ne!(a.statement_hash(), b.statement_hash());
        assert!(b.domain().regimes.iter().any(|r| r.contains("|k a| < 1")));
    }

    #[test]
    fn assumption_overlay_changes_the_hash() {
        let a = Claim::new(
            "dec.d-squared-zero",
            "The exterior derivative is nilpotent: d ∘ d = 0.",
            LayerId::Mathematical,
            ClaimClass::Mathematical,
        );
        let mut items = a.assumptions().items.clone();
        items.push(crate::assumption::Assumption {
            id: "discrete-coboundary".into(),
            statement: "Oriented simplex coboundary".into(),
            class: crate::AxiomClass::ModelAssumption,
        });
        let b = a.clone().with_assumptions(AssumptionSet::new(items));
        assert_eq!(a.id(), b.id());
        assert_ne!(a.statement_hash(), b.statement_hash());
        assert!(b
            .assumptions()
            .items
            .iter()
            .any(|x| x.id == "discrete-coboundary"));
    }

    #[test]
    fn conjecture_stays_asserted() {
        let c = Claim::new(
            "predictivity.unique-vacuum",
            "the vacuum is unique",
            LayerId::Effective,
            ClaimClass::Conjecture,
        );
        assert_eq!(c.derivation(), DerivationAssurance::Asserted);
    }

    #[test]
    fn empirical_overlay_does_not_change_class_or_derivation() {
        let c = Claim::new(
            "gut.sin2",
            "sin²θ_W(M_Z) lies in the PDG hull",
            LayerId::Effective,
            ClaimClass::EmpiricalPrediction,
        );
        assert_eq!(c.empirical(), EmpiricalStatus::Untested);
        let v = Verdict::undecidable(&c, "overlap is not containment")
            .with_empirical(EmpiricalStatus::Inconclusive);
        assert_eq!(v.class, ClaimClass::EmpiricalPrediction);
        assert_eq!(v.derivation(), DerivationAssurance::Executed);
        assert_eq!(v.empirical(), EmpiricalStatus::Inconclusive);
        assert_eq!(v.kind, VerdictKind::Undecidable);
    }

    #[test]
    fn intractable_flag_does_not_mint_a_kernel_proof() {
        let c = Claim::new(
            "comp.feasible-decision",
            "A feasible procedure decides the instance.",
            LayerId::Information,
            ClaimClass::Phenomenological,
        );
        let v = Verdict::undecidable(&c, "coNP-complete; no brute-force search").with_intractable();
        assert!(v.intractable());
        assert_eq!(v.kind, VerdictKind::Undecidable);
        assert_eq!(v.derivation(), DerivationAssurance::Executed);
    }

    #[test]
    fn certified_numeric_is_not_a_kernel_proof() {
        let c = Claim::new(
            "consistency.anomaly-cancellation",
            "Chiral gauge anomalies cancel within each generation.",
            LayerId::Interaction,
            ClaimClass::ModelInternal,
        );
        let v = Verdict::holds(&c, "exact Ratio sums vanish").with_certified_numeric("0", "0");
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_eq!(v.derivation(), DerivationAssurance::CertifiedNumeric);
        assert_eq!(v.numeric_lo(), Some("0"));
        assert_eq!(v.numeric_hi(), Some("0"));
        assert_ne!(v.derivation(), DerivationAssurance::Asserted);
    }

    #[test]
    fn statistical_nll_is_not_a_kernel_proof() {
        let c = Claim::new(
            "gut.weinberg-angle-mz-interval",
            "GQW centre vs PDG Gaussian.",
            LayerId::Effective,
            ClaimClass::EmpiricalPrediction,
        );
        let v = Verdict::fails(&c, "disjoint").with_statistical_nll("2933042");
        assert_eq!(v.statistical_nll(), Some("2933042"));
        assert_eq!(v.derivation(), DerivationAssurance::Executed);
        assert_ne!(v.derivation(), DerivationAssurance::CertifiedNumeric);
    }

    #[test]
    fn interval_enclosure_is_not_certified_numeric() {
        let c = Claim::new(
            "gut.weinberg-angle-mz-interval",
            "GQW centre vs PDG Gaussian.",
            LayerId::Effective,
            ClaimClass::EmpiricalPrediction,
        );
        let v = Verdict::fails(&c, "disjoint").with_interval_enclosure("1/5", "1/4");
        assert_eq!(v.numeric_lo(), Some("1/5"));
        assert_eq!(v.numeric_hi(), Some("1/4"));
        assert_eq!(v.derivation(), DerivationAssurance::Executed);
        assert_ne!(v.derivation(), DerivationAssurance::CertifiedNumeric);
        assert_eq!(v.kind, VerdictKind::Fails);
    }

    #[test]
    fn cross_checked_is_not_a_kernel_proof() {
        let c = Claim::new(
            "dec.euler-poincare",
            "The Euler characteristic V−E+F equals b₀−b₁+b₂.",
            LayerId::Mathematical,
            ClaimClass::ModelInternal,
        );
        let v = Verdict::holds(&c, "cell count matches Betti alternating sum").with_cross_checked();
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_eq!(v.derivation(), DerivationAssurance::CrossChecked);
        assert_ne!(v.derivation(), DerivationAssurance::CertifiedNumeric);
    }
}
