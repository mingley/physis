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
    /// Evidence graph: competing encodings and evaluations of a lab slug.
    /// Groups by statement hash. Confidence is a derived TrustProfile, not
    /// a numeric score. Does not mint.
    Evidence {
        /// Claim id (lab slug).
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
    /// Search chosen/fitted knob probes for scientific-axis diffs.
    /// Measured, derived, and fundamental-input knobs are frozen: they are
    /// not hypotheses about the encoding. Does not persist and does not mint.
    Hypothesize {
        /// Restrict to one theory. `None` searches the whole lab.
        theory: Option<String>,
    },
    /// Raise semantic assurance from a trusted dossier (not an agent-set tag).
    Review {
        /// Claim id.
        claim: String,
    },
    /// One scheduled research cycle: observe, hypothesize, prove, falsify,
    /// replicate, design, audit, review.
    Loop,
    /// Inverse query: list claims or knobs matching a trust/class/origin/gap.
    Inspect {
        /// Axis (`trust`, `class`, `origin`, `gap`). None lists the axes.
        axis: Option<String>,
        /// Value to match (`P0`, `conjecture`, `chosen`, `missing-theorem`).
        value: Option<String>,
    },
    /// Emit the catalog encoding of a claim as untrusted bytes. Does not mint.
    Formalize {
        /// Claim id.
        claim: String,
    },
    /// Re-run the dual checkers against a stored receipt. Same-process remint
    /// is not P4 independent reproduction.
    Reproduce {
        /// Claim id.
        claim: String,
    },
    /// Rebuild the knowledge-gap graph from live verdicts and receipts.
    Gaps,
}

impl Command {
    /// Protocol verb, for role/budget errors.
    pub fn verb(&self) -> &'static str {
        match self {
            Command::Layers => "layers",
            Command::Theories => "theories",
            Command::Knobs { .. } => "knobs",
            Command::Run { .. } => "run",
            Command::Set { .. } => "set",
            Command::Epistemics => "epistemics",
            Command::Why { .. } => "why",
            Command::Evidence { .. } => "evidence",
            Command::Experiments => "experiments",
            Command::Experiment { .. } => "experiment",
            Command::Journal => "journal",
            Command::Replay { .. } => "replay",
            Command::Score { .. } => "score",
            Command::Prove { .. } => "prove",
            Command::Falsify { .. } => "falsify",
            Command::Sweep { .. } => "sweep",
            Command::Branch { .. } => "branch",
            Command::Checkout { .. } => "checkout",
            Command::Compare { .. } => "compare",
            Command::Audit => "audit",
            Command::Design { .. } => "design",
            Command::Sensitivity { .. } => "sensitivity",
            Command::Hypothesize { .. } => "hypothesize",
            Command::Review { .. } => "review",
            Command::Loop => "loop",
            Command::Inspect { .. } => "inspect",
            Command::Formalize { .. } => "formalize",
            Command::Reproduce { .. } => "reproduce",
            Command::Gaps => "gaps",
        }
    }
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
