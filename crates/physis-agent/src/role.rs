//! Agent roles and research budgets.
//!
//! Roles propose protocol commands. They do not mint [`physis_verifier::Verified`].
//! The lab still routes `prove` through `physis_verifier::verify`. A role gate
//! cannot be bypassed by setting an enum on a claim.

use serde::{Deserialize, Serialize};

use crate::protocol::Command;

/// Who is issuing a command.
///
/// `Lab` is the full protocol (CLI default). Named roles are the processes
/// in the Level-3 picture: they may observe, and each may run one kind of
/// untrusted work. None of them can deserialize a kernel proof.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Role {
    /// Full lab protocol (human operator / CLI default).
    #[default]
    Lab,
    /// Observe, inspect, compare. Cannot set, prove, review, or audit.
    Explorer,
    /// Emit an untrusted encoding of a catalog identity. Cannot prove.
    Formalizer,
    /// Request a dual-check mint. The verifier still runs the checkers.
    ProofSearcher,
    /// Search for failing evaluations. Cannot prove or review.
    Falsifier,
    /// Request encoding review. Cannot prove.
    Reviewer,
    /// Run the red-team corpus. Cannot prove or review.
    Auditor,
}

impl Role {
    /// Every named role, including the lab.
    pub const ALL: [Role; 7] = [
        Role::Lab,
        Role::Explorer,
        Role::Formalizer,
        Role::ProofSearcher,
        Role::Falsifier,
        Role::Reviewer,
        Role::Auditor,
    ];

    /// Stable kebab-case name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Role::Lab => "lab",
            Role::Explorer => "explorer",
            Role::Formalizer => "formalizer",
            Role::ProofSearcher => "proof-searcher",
            Role::Falsifier => "falsifier",
            Role::Reviewer => "reviewer",
            Role::Auditor => "auditor",
        }
    }

    /// Parse a kebab-case role name.
    pub fn parse(s: &str) -> Option<Self> {
        let want = s.to_ascii_lowercase();
        Self::ALL.iter().copied().find(|r| r.as_str() == want)
    }

    /// Observe-only ops: no knob writes, no mint, no review, no audit.
    fn observe(cmd: &Command) -> bool {
        matches!(
            cmd,
            Command::Layers
                | Command::Theories
                | Command::Knobs { .. }
                | Command::Run { .. }
                | Command::Epistemics
                | Command::Why { .. }
                | Command::Inspect { .. }
                | Command::Experiments
                | Command::Experiment { .. }
                | Command::Journal
                | Command::Score { .. }
                | Command::Compare { .. }
                | Command::Design { .. }
        )
    }

    /// Whether this role may issue `cmd`.
    ///
    /// `loop` and `replay` stay lab-only: they orchestrate or certify, and
    /// must not be a back door for an explorer to mint.
    pub fn permits(self, cmd: &Command) -> bool {
        if matches!(self, Role::Lab) {
            return true;
        }
        if Self::observe(cmd) {
            return true;
        }
        match self {
            Role::Formalizer => matches!(cmd, Command::Formalize { .. }),
            Role::ProofSearcher => matches!(cmd, Command::Prove { .. }),
            Role::Falsifier => matches!(
                cmd,
                Command::Set { .. }
                    | Command::Falsify { .. }
                    | Command::Sweep { .. }
                    | Command::Sensitivity { .. }
                    | Command::Branch { .. }
                    | Command::Checkout { .. }
            ),
            Role::Reviewer => matches!(cmd, Command::Review { .. }),
            Role::Auditor => matches!(cmd, Command::Audit),
            Role::Explorer | Role::Lab => false,
        }
    }
}

/// Remaining research actions. `None` on a slot means unlimited.
///
/// Exhaustion is a lab refusal, not a mint. A zero prove budget cannot
/// be spent around by switching roles: the budget lives on the [`crate::Lab`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ResearchBudget {
    prove: Option<u32>,
    review: Option<u32>,
    set: Option<u32>,
}

impl ResearchBudget {
    /// No caps.
    pub const fn unlimited() -> Self {
        Self {
            prove: None,
            review: None,
            set: None,
        }
    }

    /// Cap prove / review / set independently. Other ops are free.
    pub const fn limited(prove: u32, review: u32, set: u32) -> Self {
        Self {
            prove: Some(prove),
            review: Some(review),
            set: Some(set),
        }
    }

    /// Parse `prove=N,review=N,set=N` (any subset). Unspecified slots stay
    /// unlimited.
    pub fn parse(spec: &str) -> Result<Self, String> {
        let mut b = Self::unlimited();
        if spec.is_empty() {
            return Ok(b);
        }
        for part in spec.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            let Some((k, v)) = part.split_once('=') else {
                return Err(format!(
                    "budget entry '{part}' must be prove=N, review=N, or set=N"
                ));
            };
            let n: u32 = v
                .trim()
                .parse()
                .map_err(|_| format!("budget {k} value '{v}' is not a non-negative integer"))?;
            match k.trim() {
                "prove" => b.prove = Some(n),
                "review" => b.review = Some(n),
                "set" => b.set = Some(n),
                other => return Err(format!("unknown budget slot '{other}' (prove|review|set)")),
            }
        }
        Ok(b)
    }

    /// Spend one slot for a consuming command. Observe is free.
    pub fn try_consume(&mut self, cmd: &Command) -> Result<(), String> {
        match cmd {
            Command::Prove { .. } => dec(&mut self.prove, "prove"),
            Command::Review { .. } => dec(&mut self.review, "review"),
            Command::Set { .. } => dec(&mut self.set, "set"),
            _ => Ok(()),
        }
    }
}

fn dec(slot: &mut Option<u32>, name: &str) -> Result<(), String> {
    match slot {
        None => Ok(()),
        Some(0) => Err(format!("research budget exhausted: {name}")),
        Some(n) => {
            *n -= 1;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prove() -> Command {
        Command::Prove {
            claim: "dec.d-squared-zero".into(),
        }
    }
    fn review() -> Command {
        Command::Review {
            claim: "dec.d-squared-zero".into(),
        }
    }
    fn inspect() -> Command {
        Command::Inspect {
            axis: Some("trust".into()),
            value: Some("P0".into()),
        }
    }

    #[test]
    fn explorer_cannot_prove_or_review() {
        assert!(Role::Explorer.permits(&inspect()));
        assert!(!Role::Explorer.permits(&prove()));
        assert!(!Role::Explorer.permits(&review()));
        assert!(!Role::Explorer.permits(&Command::Loop));
        assert!(!Role::Explorer.permits(&Command::Audit));
        assert!(!Role::Explorer.permits(&Command::Formalize {
            claim: "dec.d-squared-zero".into(),
        }));
    }

    #[test]
    fn formalizer_can_formalize_not_prove() {
        let f = Command::Formalize {
            claim: "dec.d-squared-zero".into(),
        };
        assert!(Role::Formalizer.permits(&f));
        assert!(!Role::Formalizer.permits(&prove()));
        assert!(Role::Lab.permits(&f));
    }

    #[test]
    fn proof_searcher_can_prove_not_review() {
        assert!(Role::ProofSearcher.permits(&prove()));
        assert!(!Role::ProofSearcher.permits(&review()));
        assert!(!Role::ProofSearcher.permits(&Command::Loop));
    }

    #[test]
    fn falsifier_can_set_not_prove() {
        let set = Command::Set {
            theory: "type-iib".into(),
            knob: "total_dim".into(),
            value: "9".into(),
        };
        assert!(Role::Falsifier.permits(&set));
        assert!(Role::Falsifier.permits(&Command::Falsify {
            claim: "consistency.critical-dimension".into(),
        }));
        assert!(!Role::Falsifier.permits(&prove()));
        assert!(!Role::Explorer.permits(&set));
    }

    #[test]
    fn budget_prove_zero_refuses_a_second_mint_request() {
        let mut b = ResearchBudget::limited(1, 0, 0);
        assert!(b.try_consume(&prove()).is_ok());
        let err = b.try_consume(&prove()).unwrap_err();
        assert!(err.contains("exhausted"), "{err}");
        assert!(b.try_consume(&inspect()).is_ok());
    }

    #[test]
    fn budget_parse_subset_leaves_other_slots_unlimited() {
        let mut b = ResearchBudget::parse("prove=0").unwrap();
        let err = b.try_consume(&prove()).unwrap_err();
        assert!(err.contains("exhausted"), "{err}");
        assert!(b.try_consume(&review()).is_ok());
    }
}
