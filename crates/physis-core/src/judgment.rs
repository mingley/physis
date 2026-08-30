//! Typed scientific judgments. `Holds` is too broad: a proved lemma and a
//! compatible dataset are different Rust types.
//!
//! [`Judgment::from_lab`] projects evaluator + receipts. JSON cannot mint
//! [`LogicalJudgment::Proved`].
//!
//! ```compile_fail
//! fn needs_deserialize<'de, T: serde::Deserialize<'de>>() {}
//! fn _blocked() {
//!     needs_deserialize::<physis_core::judgment::Judgment>();
//! }
//! ```

use serde::{Deserialize, Serialize};

use crate::artifact::ArtifactId;

/// Top-level judgment. Distinct from [`crate::claim::VerdictKind`], which is
/// the Level-2 evaluator result. A Level-3 claim carries one of these.
///
/// Constructed by [`Judgment::from_lab`]. There is no [`serde::Deserialize`]
/// impl: JSON cannot mint [`LogicalJudgment::Proved`].
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Judgment {
    /// Deductive / formal.
    Logical(LogicalJudgment),
    /// Validated numeric.
    Numeric(NumericJudgment),
    /// Comparison with data.
    Empirical(EmpiricalJudgment),
    /// Statistical procedure.
    Statistical(StatisticalJudgment),
    /// Heuristic / order-of-magnitude.
    Heuristic(HeuristicJudgment),
}

/// Outcome of a logical claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogicalJudgment {
    /// Independently checked proof of the challenge statement.
    Proved,
    /// Counterexample or refutation of the challenge statement.
    Disproved,
    /// Neither.
    Undetermined,
}

/// Outcome of a numeric claim.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NumericJudgment {
    /// Rigorous enclosure.
    Certified {
        /// Inclusive bounds as decimal strings (exact display, not authority).
        lo: String,
        /// Inclusive upper bound.
        hi: String,
    },
    /// A concrete witness that the claim fails.
    Counterexample {
        /// Artifact id of the witness.
        witness: ArtifactId,
    },
    /// No certificate and no witness.
    Unresolved,
}

/// Outcome of an empirical claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EmpiricalJudgment {
    /// Compatible with registered data under stated assumptions.
    Compatible,
    /// Excluded by a registered analysis.
    Excluded,
    /// Data do not decide.
    Inconclusive,
}

/// Outcome of a statistical procedure (never an LLM-invented confidence).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum StatisticalJudgment {
    /// No formal statistical object exists.
    Unquantified,
    /// A defined procedure produced a result (see evidence / receipt).
    Computed,
}

/// Heuristic judgment — explicitly not a proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum HeuristicJudgment {
    /// Order-of-magnitude / folklore, labelled as such.
    Suggestive,
    /// Heuristic that failed its own encoded check.
    Failed,
}

/// Origin of a theory parameter. Distinguishes derived predictions from
/// numbers chosen to match observation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ParameterOrigin {
    /// Input of the theory, not fitted.
    FundamentalInput,
    /// Follows from other parameters.
    Derived,
    /// Taken from a measurement (requires a dataset id to be trusted).
    Measured,
    /// Adjusted to data.
    Fitted,
    /// Chosen by the encoder / agent.
    Chosen,
    /// Nuisance parameter of an analysis.
    Nuisance,
}

impl ParameterOrigin {
    /// Stable kebab-case name.
    pub const fn as_str(self) -> &'static str {
        match self {
            ParameterOrigin::FundamentalInput => "fundamental-input",
            ParameterOrigin::Derived => "derived",
            ParameterOrigin::Measured => "measured",
            ParameterOrigin::Fitted => "fitted",
            ParameterOrigin::Chosen => "chosen",
            ParameterOrigin::Nuisance => "nuisance",
        }
    }

    /// Every origin, for inverse queries (`physis inspect origin …`).
    pub const ALL: [ParameterOrigin; 6] = [
        ParameterOrigin::FundamentalInput,
        ParameterOrigin::Derived,
        ParameterOrigin::Measured,
        ParameterOrigin::Fitted,
        ParameterOrigin::Chosen,
        ParameterOrigin::Nuisance,
    ];
}

/// Why a claim is not yet established. The knowledge-gap graph is these
/// reasons plus live [`crate::claim::Claim::depends_on`] edges, rebuilt
/// from the encoding rather than deserialized.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GapReason {
    /// An evaluator-Holds lemma has no dual-checked receipt.
    MissingTheorem,
    /// No registered dataset.
    MissingDataset,
    /// Numerics too coarse to decide.
    InsufficientPrecision,
    /// The formal backend cannot express the statement yet.
    UnsupportedFormalPrimitive,
    /// In range but too expensive.
    ComputationallyIntractable,
    /// Known undecidable.
    LogicallyUndecidable,
    /// Named open problem in the science, not a lab limitation.
    ScientificOpenProblem,
}

impl GapReason {
    /// Stable kebab-case name.
    pub const fn as_str(self) -> &'static str {
        match self {
            GapReason::MissingTheorem => "missing-theorem",
            GapReason::MissingDataset => "missing-dataset",
            GapReason::InsufficientPrecision => "insufficient-precision",
            GapReason::UnsupportedFormalPrimitive => "unsupported-formal-primitive",
            GapReason::ComputationallyIntractable => "computationally-intractable",
            GapReason::LogicallyUndecidable => "logically-undecidable",
            GapReason::ScientificOpenProblem => "scientific-open-problem",
        }
    }

    /// Every gap reason, for inverse queries (`physis inspect gap …`).
    pub const ALL: [GapReason; 7] = [
        GapReason::MissingTheorem,
        GapReason::MissingDataset,
        GapReason::InsufficientPrecision,
        GapReason::UnsupportedFormalPrimitive,
        GapReason::ComputationallyIntractable,
        GapReason::LogicallyUndecidable,
        GapReason::ScientificOpenProblem,
    ];
}

/// Visible global assurance level. A high-value result aims at
/// P3F + P3S + P4, not a single `theorem` tag.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrustTier {
    /// Agent-generated assertion.
    P0,
    /// Typed and deterministically evaluated.
    P1,
    /// Independent implementation or reference comparison.
    P2,
    /// Rigorous numerical certificate.
    P3N,
    /// Independently verified machine proof.
    P3F,
    /// Formal statement independently checked against sources/encoding.
    P3S,
    /// Independent reproduction of the complete result.
    P4,
}

impl TrustTier {
    /// Stable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            TrustTier::P0 => "P0",
            TrustTier::P1 => "P1",
            TrustTier::P2 => "P2",
            TrustTier::P3N => "P3N",
            TrustTier::P3F => "P3F",
            TrustTier::P3S => "P3S",
            TrustTier::P4 => "P4",
        }
    }

    /// Every named tier, low to high. A profile may hold several at once.
    pub const ALL: [TrustTier; 7] = [
        TrustTier::P0,
        TrustTier::P1,
        TrustTier::P2,
        TrustTier::P3N,
        TrustTier::P3F,
        TrustTier::P3S,
        TrustTier::P4,
    ];
}

/// Inputs from which a [`TrustProfile`] is *derived*. Setting a tier enum
/// is not an input; a dual-checked receipt is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrustEvidence {
    /// Evaluator / numeric assurance on the claim.
    pub derivation: crate::assurance::DerivationAssurance,
    /// Encoding-review tag (never Canonical from an agent).
    pub semantic: crate::assurance::SemanticAssurance,
    /// True only when `physis_verifier::verify` minted a receipt.
    pub dual_checked_receipt: bool,
    /// True when a numeric certificate (interval / ratio) backs a threshold.
    pub numeric_certificate: bool,
}

/// Derived trust. Fields are private: P3F cannot be manufactured by a
/// struct literal. There is no [`serde::Deserialize`] impl.
///
/// ```compile_fail
/// use physis_core::judgment::TrustProfile;
/// let _ = TrustProfile {
///     p0: false,
///     p1: false,
///     p2: false,
///     p3n: false,
///     p3f: true,
///     p3s: false,
///     p4: false,
/// };
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct TrustProfile {
    p0: bool,
    p1: bool,
    p2: bool,
    p3n: bool,
    p3f: bool,
    p3s: bool,
    p4: bool,
}

impl TrustProfile {
    /// Compute earned tiers from evidence. P4 is not assigned from a
    /// single in-process remint (that is not independent reproduction).
    pub fn derive(ev: TrustEvidence) -> Self {
        use crate::assurance::{DerivationAssurance, SemanticAssurance};
        let mut s = Self {
            p0: false,
            p1: false,
            p2: false,
            p3n: false,
            p3f: false,
            p3s: false,
            p4: false,
        };
        match ev.derivation {
            DerivationAssurance::Asserted => s.p0 = true,
            DerivationAssurance::Executed => s.p1 = true,
            DerivationAssurance::CrossChecked => {
                s.p1 = true;
                s.p2 = true;
            }
            DerivationAssurance::CertifiedNumeric => {
                s.p1 = true;
                s.p3n = true;
            }
        }
        if ev.numeric_certificate {
            s.p3n = true;
        }
        if ev.dual_checked_receipt {
            s.p3f = true;
            s.p1 = true;
            s.p0 = false;
        }
        match ev.semantic {
            SemanticAssurance::Unreviewed | SemanticAssurance::SourceAnchored => {}
            SemanticAssurance::IndependentlyEncoded
            | SemanticAssurance::AdversariallyReviewed
            | SemanticAssurance::Canonical => {
                s.p2 = true;
                s.p3s = true;
            }
        }
        s
    }

    /// Whether this profile earned `tier`.
    pub const fn has(self, tier: TrustTier) -> bool {
        match tier {
            TrustTier::P0 => self.p0,
            TrustTier::P1 => self.p1,
            TrustTier::P2 => self.p2,
            TrustTier::P3N => self.p3n,
            TrustTier::P3F => self.p3f,
            TrustTier::P3S => self.p3s,
            TrustTier::P4 => self.p4,
        }
    }

    /// Compact `P1+P3F` display. Empty is `none`.
    pub fn display(self) -> String {
        let parts: Vec<&str> = TrustTier::ALL
            .iter()
            .copied()
            .filter(|t| self.has(*t))
            .map(TrustTier::as_str)
            .collect();
        if parts.is_empty() {
            "none".into()
        } else {
            parts.join("+")
        }
    }

    /// Kernel proof of an unreviewed encoding is not physics.
    pub const fn unreviewed_proof_is_dangerous(
        self,
        semantic: crate::assurance::SemanticAssurance,
    ) -> bool {
        self.p3f && matches!(semantic, crate::assurance::SemanticAssurance::Unreviewed)
    }
}

impl Judgment {
    /// Project a lab evaluation into a typed judgment. Evaluator `holds`
    /// is not [`LogicalJudgment::Proved`]. A model-internal evaluation
    /// that overlays [`crate::assurance::EmpiricalStatus::Inconclusive`]
    /// is [`NumericJudgment::Unresolved`] (too coarse to decide), not a
    /// failed theorem. [`crate::assurance::DerivationAssurance::CertifiedNumeric`]
    /// Holds is [`NumericJudgment::Certified`], not logical undetermined.
    pub fn from_lab(
        class: crate::assurance::ClaimClass,
        kind: crate::claim::VerdictKind,
        empirical: crate::assurance::EmpiricalStatus,
        derivation: crate::assurance::DerivationAssurance,
        dual_checked: bool,
        numeric_lo: Option<&str>,
        numeric_hi: Option<&str>,
    ) -> Self {
        use crate::assurance::{ClaimClass, DerivationAssurance, EmpiricalStatus};
        use crate::claim::VerdictKind;
        if empirical == EmpiricalStatus::Inconclusive
            && matches!(
                class,
                ClaimClass::Mathematical | ClaimClass::ModelInternal | ClaimClass::Phenomenological
            )
        {
            return Judgment::Numeric(NumericJudgment::Unresolved);
        }
        if derivation == DerivationAssurance::CertifiedNumeric && kind == VerdictKind::Holds {
            return Judgment::Numeric(NumericJudgment::Certified {
                lo: numeric_lo.unwrap_or("").to_string(),
                hi: numeric_hi.unwrap_or("").to_string(),
            });
        }
        match class {
            ClaimClass::Mathematical | ClaimClass::ModelInternal | ClaimClass::Phenomenological => {
                let j = if dual_checked && kind == VerdictKind::Holds {
                    LogicalJudgment::Proved
                } else if kind == VerdictKind::Fails {
                    LogicalJudgment::Disproved
                } else {
                    LogicalJudgment::Undetermined
                };
                Judgment::Logical(j)
            }
            ClaimClass::Heuristic => {
                let j = if kind == VerdictKind::Fails {
                    HeuristicJudgment::Failed
                } else {
                    HeuristicJudgment::Suggestive
                };
                Judgment::Heuristic(j)
            }
            ClaimClass::EmpiricalPrediction | ClaimClass::Measurement => {
                let j = match empirical {
                    EmpiricalStatus::Compatible | EmpiricalStatus::Supported => {
                        EmpiricalJudgment::Compatible
                    }
                    EmpiricalStatus::Excluded => EmpiricalJudgment::Excluded,
                    _ => EmpiricalJudgment::Inconclusive,
                };
                Judgment::Empirical(j)
            }
            ClaimClass::Conjecture | ClaimClass::OpenProblem => {
                Judgment::Logical(LogicalJudgment::Undetermined)
            }
        }
    }

    /// Stable two-token label (`logical proved`).
    pub fn label(&self) -> String {
        match self {
            Judgment::Logical(j) => format!(
                "logical {}",
                match j {
                    LogicalJudgment::Proved => "proved",
                    LogicalJudgment::Disproved => "disproved",
                    LogicalJudgment::Undetermined => "undetermined",
                }
            ),
            Judgment::Numeric(NumericJudgment::Certified { .. }) => "numeric certified".into(),
            Judgment::Numeric(NumericJudgment::Counterexample { .. }) => {
                "numeric counterexample".into()
            }
            Judgment::Numeric(NumericJudgment::Unresolved) => "numeric unresolved".into(),
            Judgment::Empirical(j) => format!(
                "empirical {}",
                match j {
                    EmpiricalJudgment::Compatible => "compatible",
                    EmpiricalJudgment::Excluded => "excluded",
                    EmpiricalJudgment::Inconclusive => "inconclusive",
                }
            ),
            Judgment::Statistical(j) => format!(
                "statistical {}",
                match j {
                    StatisticalJudgment::Unquantified => "unquantified",
                    StatisticalJudgment::Computed => "computed",
                }
            ),
            Judgment::Heuristic(j) => format!(
                "heuristic {}",
                match j {
                    HeuristicJudgment::Suggestive => "suggestive",
                    HeuristicJudgment::Failed => "failed",
                }
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assurance::{DerivationAssurance, SemanticAssurance};

    #[test]
    fn p3f_requires_a_receipt() {
        let no = TrustProfile::derive(TrustEvidence {
            derivation: DerivationAssurance::Executed,
            semantic: SemanticAssurance::Unreviewed,
            dual_checked_receipt: false,
            numeric_certificate: false,
        });
        assert!(no.has(TrustTier::P1));
        assert!(!no.has(TrustTier::P3F));
        assert!(!no.has(TrustTier::P4));

        let yes = TrustProfile::derive(TrustEvidence {
            derivation: DerivationAssurance::Executed,
            semantic: SemanticAssurance::Unreviewed,
            dual_checked_receipt: true,
            numeric_certificate: false,
        });
        assert!(yes.has(TrustTier::P3F));
        assert!(yes.unreviewed_proof_is_dangerous(SemanticAssurance::Unreviewed));
        assert!(!yes.has(TrustTier::P4));
    }

    #[test]
    fn review_earns_p3s_not_p3f() {
        let p = TrustProfile::derive(TrustEvidence {
            derivation: DerivationAssurance::Executed,
            semantic: SemanticAssurance::AdversariallyReviewed,
            dual_checked_receipt: false,
            numeric_certificate: false,
        });
        assert!(p.has(TrustTier::P3S));
        assert!(!p.has(TrustTier::P3F));
    }

    #[test]
    fn asserted_is_p0_not_proved() {
        let p = TrustProfile::derive(TrustEvidence {
            derivation: DerivationAssurance::Asserted,
            semantic: SemanticAssurance::Unreviewed,
            dual_checked_receipt: false,
            numeric_certificate: false,
        });
        assert!(p.has(TrustTier::P0));
        assert!(!p.has(TrustTier::P1));
        let j = Judgment::from_lab(
            crate::assurance::ClaimClass::Conjecture,
            crate::claim::VerdictKind::Holds,
            crate::assurance::EmpiricalStatus::Untested,
            crate::assurance::DerivationAssurance::Asserted,
            false,
            None,
            None,
        );
        assert_eq!(j.label(), "logical undetermined");
    }

    #[test]
    fn evaluator_holds_is_not_proved_without_a_receipt() {
        let open = Judgment::from_lab(
            crate::assurance::ClaimClass::Mathematical,
            crate::claim::VerdictKind::Holds,
            crate::assurance::EmpiricalStatus::NotApplicable,
            crate::assurance::DerivationAssurance::Executed,
            false,
            None,
            None,
        );
        assert_eq!(open.label(), "logical undetermined");
        assert_ne!(open, Judgment::Logical(LogicalJudgment::Proved));
        let proved = Judgment::from_lab(
            crate::assurance::ClaimClass::Mathematical,
            crate::claim::VerdictKind::Holds,
            crate::assurance::EmpiricalStatus::NotApplicable,
            crate::assurance::DerivationAssurance::Executed,
            true,
            None,
            None,
        );
        assert_eq!(proved.label(), "logical proved");
        assert_eq!(proved, Judgment::Logical(LogicalJudgment::Proved));
    }

    #[test]
    fn inconclusive_model_internal_is_numeric_unresolved() {
        let j = Judgment::from_lab(
            crate::assurance::ClaimClass::ModelInternal,
            crate::claim::VerdictKind::Undecidable,
            crate::assurance::EmpiricalStatus::Inconclusive,
            crate::assurance::DerivationAssurance::Executed,
            false,
            None,
            None,
        );
        assert_eq!(j.label(), "numeric unresolved");
    }

    #[test]
    fn certified_numeric_holds_is_numeric_certified() {
        let j = Judgment::from_lab(
            crate::assurance::ClaimClass::ModelInternal,
            crate::claim::VerdictKind::Holds,
            crate::assurance::EmpiricalStatus::NotApplicable,
            crate::assurance::DerivationAssurance::CertifiedNumeric,
            false,
            Some("3/8"),
            Some("3/8"),
        );
        assert_eq!(j.label(), "numeric certified");
        match j {
            Judgment::Numeric(NumericJudgment::Certified { lo, hi }) => {
                assert_eq!(lo, "3/8");
                assert_eq!(hi, "3/8");
            }
            other => panic!("expected numeric certified, got {other:?}"),
        }
    }

    #[test]
    fn inconclusive_beats_certified_numeric() {
        let j = Judgment::from_lab(
            crate::assurance::ClaimClass::ModelInternal,
            crate::claim::VerdictKind::Holds,
            crate::assurance::EmpiricalStatus::Inconclusive,
            crate::assurance::DerivationAssurance::CertifiedNumeric,
            false,
            Some("3/8"),
            Some("3/8"),
        );
        assert_eq!(j.label(), "numeric unresolved");
    }

    #[test]
    fn certified_numeric_earns_p3n_not_p3f() {
        let p = TrustProfile::derive(TrustEvidence {
            derivation: DerivationAssurance::CertifiedNumeric,
            semantic: SemanticAssurance::Unreviewed,
            dual_checked_receipt: false,
            numeric_certificate: true,
        });
        assert!(p.has(TrustTier::P1));
        assert!(p.has(TrustTier::P3N));
        assert!(!p.has(TrustTier::P3F));
        assert!(!p.has(TrustTier::P4));
    }

    #[test]
    fn cross_checked_earns_p2_not_p3f() {
        let p = TrustProfile::derive(TrustEvidence {
            derivation: DerivationAssurance::CrossChecked,
            semantic: SemanticAssurance::Unreviewed,
            dual_checked_receipt: false,
            numeric_certificate: false,
        });
        assert!(p.has(TrustTier::P1));
        assert!(p.has(TrustTier::P2));
        assert!(!p.has(TrustTier::P3N));
        assert!(!p.has(TrustTier::P3F));
        assert!(!p.has(TrustTier::P4));
    }

    #[test]
    fn parameter_origin_names_are_stable_and_unique() {
        let names: Vec<_> = ParameterOrigin::ALL.iter().map(|o| o.as_str()).collect();
        assert_eq!(names.len(), 6);
        for (i, a) in names.iter().enumerate() {
            for b in names.iter().skip(i + 1) {
                assert_ne!(a, b);
            }
        }
        assert_eq!(ParameterOrigin::Fitted.as_str(), "fitted");
        assert_eq!(ParameterOrigin::Chosen.as_str(), "chosen");
        assert_eq!(ParameterOrigin::Measured.as_str(), "measured");
    }
}
