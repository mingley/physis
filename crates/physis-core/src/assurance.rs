//! Orthogonal assurance axes for a scientific claim.
//!
//! Level-2 physis mixed “this is a theorem” into a single enum. Level-3
//! splits three questions that must not be conflated:
//!
//! 1. What kind of claim is this? ([`ClaimClass`])
//! 2. How strongly has the deduction been verified? ([`DerivationAssurance`])
//! 3. What does observation say? ([`EmpiricalStatus`])
//!
//! A fourth axis, [`SemanticAssurance`], records whether the *encoding*
//! has been checked against the intended physics. A machine-checked
//! deduction from the wrong formalization is still dangerous.
//!
//! There is **no** [`DerivationAssurance`] variant for a kernel proof.
//! `MachineProved` exists only as `physis_verifier::Verified<T>` minted
//! from a dual-checker receipt. Setting an enum cannot create it.

use serde::{Deserialize, Serialize};

/// What kind of scientific sentence this is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClaimClass {
    /// A statement of mathematics (e.g. `d² = 0`).
    Mathematical,
    /// A consequence of the encoded model, not yet an observation.
    ModelInternal,
    /// An effective / descriptive relation stored or checked in the model.
    Phenomenological,
    /// A prediction that could be compared to data.
    EmpiricalPrediction,
    /// A report of a measurement (requires a dataset artifact to be trusted).
    Measurement,
    /// The theory claims it; we have not derived it.
    Conjecture,
    /// Order-of-magnitude / folklore.
    Heuristic,
    /// The encoding cannot decide.
    OpenProblem,
}

impl ClaimClass {
    /// Stable kebab-case name.
    pub const fn as_str(self) -> &'static str {
        match self {
            ClaimClass::Mathematical => "mathematical",
            ClaimClass::ModelInternal => "model-internal",
            ClaimClass::Phenomenological => "phenomenological",
            ClaimClass::EmpiricalPrediction => "empirical-prediction",
            ClaimClass::Measurement => "measurement",
            ClaimClass::Conjecture => "conjecture",
            ClaimClass::Heuristic => "heuristic",
            ClaimClass::OpenProblem => "open-problem",
        }
    }

    /// Every class, for inverse queries (`physis inspect class …`).
    pub const ALL: [ClaimClass; 8] = [
        ClaimClass::Mathematical,
        ClaimClass::ModelInternal,
        ClaimClass::Phenomenological,
        ClaimClass::EmpiricalPrediction,
        ClaimClass::Measurement,
        ClaimClass::Conjecture,
        ClaimClass::Heuristic,
        ClaimClass::OpenProblem,
    ];

    /// Default derivation tag for a newly encoded claim of this class.
    ///
    /// Model-internal and mathematical claims that the lab *evaluates* are
    /// [`DerivationAssurance::Executed`]. Conjecture / heuristic / open stay
    /// [`DerivationAssurance::Asserted`]. This is not `MachineProved`.
    pub const fn default_derivation(self) -> DerivationAssurance {
        match self {
            ClaimClass::Mathematical
            | ClaimClass::ModelInternal
            | ClaimClass::Phenomenological
            | ClaimClass::EmpiricalPrediction
            | ClaimClass::Measurement => DerivationAssurance::Executed,
            ClaimClass::Conjecture | ClaimClass::Heuristic | ClaimClass::OpenProblem => {
                DerivationAssurance::Asserted
            }
        }
    }

    /// Default empirical tag. Predictions start untested until a dataset
    /// receipt exists; purely mathematical / model-internal claims are
    /// not applicable as observations.
    pub const fn default_empirical(self) -> EmpiricalStatus {
        match self {
            ClaimClass::EmpiricalPrediction | ClaimClass::Measurement => EmpiricalStatus::Untested,
            ClaimClass::Conjecture | ClaimClass::Heuristic | ClaimClass::OpenProblem => {
                EmpiricalStatus::Untested
            }
            ClaimClass::Mathematical | ClaimClass::ModelInternal | ClaimClass::Phenomenological => {
                EmpiricalStatus::NotApplicable
            }
        }
    }
}

/// How strongly a deduction has been verified **inside Physis**.
///
/// There is deliberately no `MachineProved` variant. A kernel-checked proof
/// is a `physis_verifier::Verified<T>` value, not a tag an agent can set.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DerivationAssurance {
    /// Stated, not mechanically checked.
    Asserted,
    /// A deterministic evaluator in this encoding returned a verdict.
    Executed,
    /// Two independent executable paths agreed (not yet a kernel proof).
    CrossChecked,
    /// A numeric certificate (interval / residual) exists. Not a Lean proof.
    CertifiedNumeric,
}

impl DerivationAssurance {
    /// Stable kebab-case name.
    pub const fn as_str(self) -> &'static str {
        match self {
            DerivationAssurance::Asserted => "asserted",
            DerivationAssurance::Executed => "executed",
            DerivationAssurance::CrossChecked => "cross-checked",
            DerivationAssurance::CertifiedNumeric => "certified-numeric",
        }
    }
}

/// What observation currently says, independently of derivation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EmpiricalStatus {
    /// Not a claim about data.
    NotApplicable,
    /// Could be tested; no dataset receipt yet.
    Untested,
    /// Compatible with registered data under stated assumptions.
    Compatible,
    /// Positively supported (stronger than compatible; requires a receipt).
    Supported,
    /// Tension with data; not yet an exclusion receipt.
    Tension,
    /// Excluded by a registered empirical analysis. A theory may still
    /// *derive* the prediction (`Executed` / later `MachineProved`).
    Excluded,
    /// Data exist but do not decide.
    Inconclusive,
}

impl EmpiricalStatus {
    /// Stable kebab-case name.
    pub const fn as_str(self) -> &'static str {
        match self {
            EmpiricalStatus::NotApplicable => "not-applicable",
            EmpiricalStatus::Untested => "untested",
            EmpiricalStatus::Compatible => "compatible",
            EmpiricalStatus::Supported => "supported",
            EmpiricalStatus::Tension => "tension",
            EmpiricalStatus::Excluded => "excluded",
            EmpiricalStatus::Inconclusive => "inconclusive",
        }
    }
}

/// Whether the formalization is known to represent the intended physics.
///
/// Formal verification proves the conclusion follows from the assumptions.
/// It does not prove the assumptions encode the right physics. A
/// `MachineProved` result with [`SemanticAssurance::Unreviewed`] is
/// dangerous and must be presented as such.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SemanticAssurance {
    /// No independent encoding review.
    Unreviewed,
    /// Points at a locked source record (page/equation). Not yet dual-encoded.
    SourceAnchored,
    /// Encoded independently of the first formalization.
    IndependentlyEncoded,
    /// An encoding auditor has challenged and not found a fatal hole.
    AdversariallyReviewed,
    /// Community-canonical encoding of a named result.
    Canonical,
}

impl SemanticAssurance {
    /// Stable kebab-case name.
    pub const fn as_str(self) -> &'static str {
        match self {
            SemanticAssurance::Unreviewed => "unreviewed",
            SemanticAssurance::SourceAnchored => "source-anchored",
            SemanticAssurance::IndependentlyEncoded => "independently-encoded",
            SemanticAssurance::AdversariallyReviewed => "adversarially-reviewed",
            SemanticAssurance::Canonical => "canonical",
        }
    }
}
