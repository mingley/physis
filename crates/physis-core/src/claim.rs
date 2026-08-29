//! Claims, epistemic status, and verdicts.
//!
//! A claim is a sentence a theory makes. A verdict is what happens when
//! that sentence is checked against the theory's current knobs and the
//! mechanical consequences they induce.
//!
//! Verdicts are not "truth of the universe". They are *internal*:
//! consistency with the model as encoded. Empirical contact is a family
//! of claims, not a side channel.

use serde::{Deserialize, Serialize};

use crate::id::{ClaimId, LayerId};

/// How much the codebase claims to *know* about a claim.
///
/// This is as important as the verdict. A `Holds` that is a `Heuristic`
/// is not the same object as a `Holds` that is a `Theorem`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Epistemic {
    /// Proven inside this model (or a standard theorem encoded as such).
    Theorem,
    /// Standard mathematical fact encoded as a table, not re-derived here.
    EncodedFact,
    /// Plausible, not proven, in this model.
    Conjecture,
    /// Rule of thumb (landscape counts, naturalness, …).
    Heuristic,
    /// Not decided by the current encoding.
    Open,
}

impl Epistemic {
    /// Stable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Epistemic::Theorem => "theorem",
            Epistemic::EncodedFact => "encoded-fact",
            Epistemic::Conjecture => "conjecture",
            Epistemic::Heuristic => "heuristic",
            Epistemic::Open => "open",
        }
    }
}

/// Four-way judgment.
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
    /// How seriously to take this verdict.
    pub epistemic: Epistemic,
    /// One-line reason.
    pub summary: String,
    /// Structured notes (numbers, mismatched knobs, citations).
    pub evidence: Vec<String>,
}

impl Verdict {
    /// Convenience constructor.
    pub fn new(
        kind: VerdictKind,
        epistemic: Epistemic,
        summary: impl Into<String>,
        evidence: Vec<String>,
    ) -> Self {
        Self {
            kind,
            epistemic,
            summary: summary.into(),
            evidence,
        }
    }

    /// Holds.
    pub fn holds(epistemic: Epistemic, summary: impl Into<String>) -> Self {
        Self::new(VerdictKind::Holds, epistemic, summary, Vec::new())
    }

    /// Fails.
    pub fn fails(epistemic: Epistemic, summary: impl Into<String>) -> Self {
        Self::new(VerdictKind::Fails, epistemic, summary, Vec::new())
    }

    /// Does not apply.
    pub fn inapplicable(summary: impl Into<String>) -> Self {
        Self::new(
            VerdictKind::Inapplicable,
            Epistemic::EncodedFact,
            summary,
            Vec::new(),
        )
    }

    /// Cannot decide.
    pub fn undecidable(epistemic: Epistemic, summary: impl Into<String>) -> Self {
        Self::new(VerdictKind::Undecidable, epistemic, summary, Vec::new())
    }

    /// Attach evidence lines.
    pub fn with_evidence(mut self, lines: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.evidence.extend(lines.into_iter().map(Into::into));
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
    /// Default epistemic status (verdicts may refine).
    pub epistemic: Epistemic,
}

impl Claim {
    /// Constructor.
    pub fn new(
        id: impl Into<String>,
        statement: impl Into<String>,
        layer: LayerId,
        epistemic: Epistemic,
    ) -> Self {
        Self {
            id: ClaimId::new(id),
            statement: statement.into(),
            layer,
            epistemic,
        }
    }
}
