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
    /// Groups by statement hash. Inserts a rebuilt snapshot into the
    /// artifact DAG (not deserialized as authority). Confidence is a
    /// derived TrustProfile, not a numeric score. Does not mint.
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
    /// Search chosen/fitted knob probes and IR package forks for
    /// scientific-axis diffs. Measured, derived, and fundamental-input
    /// knobs are frozen. Package mutations are not knobs and are not
    /// installed. Does not persist and does not mint.
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
    /// Independently parse a `CertifiedNumeric` enclosure as `Ratio`.
    /// Stores a content-addressed numeric certificate. Not a kernel
    /// receipt, not Canonical, and not P4.
    Enclose {
        /// Claim id.
        claim: String,
    },
    /// Independently rebuild a live [`physis_provenance::SourceRecord`].
    /// Datasets and catalog dossiers. Not P3S, not a kernel receipt,
    /// not Canonical, and not P4.
    Cite {
        /// Claim id.
        claim: String,
    },
    /// Independently rebuild a versioned physical constant from live
    /// constructors. Not P3N, not P3S, not a kernel receipt, not
    /// Canonical, and not P4. `name: None` rebuilds the full ledger.
    Constant {
        /// Ledger name (`c`, `G`, `h`, …). `None` rebuilds every
        /// [`physis_constants::LEDGER`] entry into one bundle node.
        #[serde(default)]
        name: Option<String>,
    },
    /// Independently parse, round-trip, and reconstruct a live theory
    /// IR package. Stores a content-addressed EncodingPackage. Not P3S,
    /// not a kernel receipt, not Canonical, and not P4.
    Encode {
        /// Theory id.
        theory: String,
    },
    /// Independently rebuild [`physis_core::judgment::Judgment::from_lab`]
    /// from live evaluator axes and receipts. Stores a content-addressed
    /// JudgmentProjection. Not P3S, not a kernel receipt, not Canonical,
    /// and not P4. JSON cannot mint `logical proved`.
    Judge {
        /// Claim id (lab slug).
        claim: String,
    },
    /// One gap-driven research step (C7.1): select one eligible gap in
    /// scope by documented priority plus stable statement-hash
    /// tie-break, then run at most one allowed existing operation
    /// (prove, cite, or enclose) or emit an explicit blocked work item.
    /// Lab-only: selection never lends a proposer Lab authority.
    Step {
        /// Theory id or claim id to scope the scan. `None` scans the lab.
        #[serde(default)]
        scope: Option<String>,
        /// Finite step cap for this call. One action consumes one step.
        #[serde(default = "default_step_cap")]
        max_steps: u32,
    },
    /// Dry-run a versioned campaign manifest (R1): parse, refuse schema
    /// violations, validate live references, and render the plan.
    /// Executes nothing and grants no authority.
    ResearchPlan {
        /// Manifest file path.
        path: String,
    },
    /// Run at most one coordinated campaign action (R4): reconcile,
    /// select, reserve, execute, commit, checkpoint.
    ResearchStep {
        /// Manifest file path (revision-pinned to the bundle).
        manifest: String,
        /// Campaign bundle directory.
        campaign: String,
    },
    /// Render campaign status from the bundle alone (R4). Read-only.
    ResearchStatus {
        /// Campaign bundle directory.
        campaign: String,
    },
    /// Export the versioned campaign report (R6). Read-only projection.
    ResearchExport {
        /// Campaign bundle directory.
        campaign: String,
        /// `json` (default) or `markdown`.
        format: Option<String>,
    },
    /// Write a child manifest from a stopped campaign. Does not reset
    /// the parent budget or erase its attempts.
    ResearchFollow {
        /// Parent manifest path.
        manifest: String,
        /// Parent campaign directory.
        campaign: String,
        /// Path of the new manifest. Must sit outside the parent directory.
        child: String,
    },
    /// Submit a dimensioned polynomial. Dual-expand it, store the record
    /// in the campaign bundle, and leave the baseline theory installed
    /// as it was. Not a receipt.
    Propose {
        /// Theory id.
        theory: String,
        /// Observable unit. Refused when it is not that theory's unit.
        unit: String,
        /// Polynomial source.
        expression: String,
        /// Campaign bundle that stores the untrusted record.
        campaign: String,
    },
}

/// Default cap for [`Command::Step`]: exactly one action.
fn default_step_cap() -> u32 {
    1
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
            Command::Enclose { .. } => "enclose",
            Command::Cite { .. } => "cite",
            Command::Constant { .. } => "constant",
            Command::Encode { .. } => "encode",
            Command::Judge { .. } => "judge",
            Command::Step { .. } => "step",
            Command::ResearchPlan { .. } => "research-plan",
            Command::ResearchStep { .. } => "research-step",
            Command::ResearchStatus { .. } => "research-status",
            Command::ResearchExport { .. } => "research-export",
            Command::ResearchFollow { .. } => "research-follow",
            Command::Propose { .. } => "propose",
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
