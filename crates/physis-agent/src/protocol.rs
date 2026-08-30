//! Commands an agent can issue, and responses the lab returns.

use physis_theory::{ExperimentReport, VerdictDiff};
use serde::{Deserialize, Serialize};

/// Agent → lab.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "kebab-case")]
pub enum Command {
    /// List description layers.
    Layers,
    /// List theories in the lab.
    Theories,
    /// List knobs of one theory (or all if omitted).
    Knobs {
        /// Theory id.
        theory: Option<String>,
    },
    /// Evaluate one theory.
    Run {
        /// Theory id.
        theory: String,
    },
    /// Turn a knob.
    Set {
        /// Theory id.
        theory: String,
        /// Knob name.
        knob: String,
        /// Raw value (parsed against the knob domain).
        value: String,
    },
    /// Tally every verdict across the lab by class / derivation / semantic axes.
    Epistemics,
    /// Explain a claim's assumptions, identity hash, and assurance (not a proof).
    Why {
        /// Claim id (`astro.sky-finite`, `consistency.critical-dimension`, …).
        claim: String,
    },
    /// List the available experiments.
    Experiments,
    /// Run a named experiment.
    Experiment {
        /// Experiment id (`string-critique`).
        id: String,
    },
    /// Dump the journal.
    Journal,
    /// Replay a recorded JSONL journal and check it reproduces.
    Replay {
        /// Path to a JSONL journal file.
        path: String,
    },
    /// Score a theory against the empirical target fixture.
    Score {
        /// Theory id.
        theory: String,
    },
    /// Dual-check a catalogued identity and mint a receipt (verifier only).
    Prove {
        /// Claim id.
        claim: String,
    },
    /// Search knob space for a failing evaluation of a currently-holding claim.
    Falsify {
        /// Claim id.
        claim: String,
    },
    /// Evaluate a knob at many values; report verdict diffs.
    Sweep {
        /// Theory id.
        theory: String,
        /// Knob name.
        knob: String,
        /// Raw values.
        values: Vec<String>,
    },
    /// Snapshot current knobs under a branch name (content-addressed later).
    Branch {
        /// Branch name.
        name: String,
    },
    /// Restore a named branch.
    Checkout {
        /// Branch name.
        name: String,
    },
    /// Compare two theories' verdict kinds on shared claim ids.
    Compare {
        /// First theory.
        a: String,
        /// Second theory.
        b: String,
    },
    /// Run the red-team corpus. Exit non-zero if a corruption is not caught.
    Audit,
    /// Rank claims that distinguish a list of theories (experiment design).
    Design {
        /// Theory ids.
        theories: Vec<String>,
    },
    /// Perturb one knob and list claims whose kind flipped (sensitivity).
    Sensitivity {
        /// Theory id.
        theory: String,
        /// Knob name.
        knob: String,
    },
}

/// Lab → agent.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum Response {
    /// Success with a text body (CLI-oriented).
    Ok {
        /// Human-readable body.
        text: String,
        /// Optional structured experiment (boxed: it is much larger than the
        /// other response fields).
        #[serde(skip_serializing_if = "Option::is_none")]
        report: Option<Box<ExperimentReport>>,
        /// Optional verdict diffs.
        #[serde(skip_serializing_if = "Option::is_none")]
        diffs: Option<Vec<VerdictDiff>>,
    },
    /// Failure.
    Err {
        /// Message.
        message: String,
    },
}

impl Response {
    /// Convenience.
    pub fn ok(text: impl Into<String>) -> Self {
        Response::Ok {
            text: text.into(),
            report: None,
            diffs: None,
        }
    }

    /// Failure.
    pub fn err(message: impl Into<String>) -> Self {
        Response::Err {
            message: message.into(),
        }
    }

    /// Text body (errors included).
    pub fn text(&self) -> &str {
        match self {
            Response::Ok { text, .. } => text,
            Response::Err { message } => message,
        }
    }

    /// Exit code for the CLI.
    pub fn exit_code(&self) -> i32 {
        match self {
            Response::Ok { .. } => 0,
            Response::Err { .. } => 1,
        }
    }
}
