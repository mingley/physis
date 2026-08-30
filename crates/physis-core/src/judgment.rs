//! Typed scientific judgments. `Holds` is too broad: a proved lemma and a
//! compatible dataset are different Rust types.

use serde::{Deserialize, Serialize};

use crate::artifact::ArtifactId;

/// Top-level judgment. Distinct from [`crate::claim::VerdictKind`], which is
/// the Level-2 evaluator result. A Level-3 claim carries one of these.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StatisticalJudgment {
    /// No formal statistical object exists.
    Unquantified,
    /// A defined procedure produced a result (see evidence / receipt).
    Computed,
}

/// Heuristic judgment — explicitly not a proof.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
}

/// Why a claim is not yet established. The knowledge-gap graph is these
/// reasons plus the claim's dependency list.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GapReason {
    /// A needed lemma has no receipt.
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
}
