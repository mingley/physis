//! Deterministic replay of a recorded journal onto a fresh [`Lab::standard`].
//!
//! The bet behind physis is that an agent can work here over long time
//! horizons *because* the state is small, typed, and diffable. That only pays
//! off if a later agent can reconstruct **what was tried** from a JSONL
//! journal alone, without the original process or session.
//!
//! Replay re-applies each recorded `set-knob` event, in order, onto a fresh
//! `Lab::standard()` and recomputes the verdict diffs. It then checks that the
//! diffs it recomputes are exactly the ones the journal recorded. Because knob
//! turns are deterministic functions of the lab state, a faithful replay is a
//! *mechanical proof* that the recorded session is reproducible — and a
//! mismatch is a mechanical proof that the journal (or the encoding) drifted.
//!
//! ```
//! use physis_agent::{Journal, Lab};
//! use physis_agent::replay::replay_journal;
//!
//! let mut lab = Lab::standard();
//! lab.set_knob("type-iib", "total_dim", "9").unwrap();
//!
//! let jsonl = lab.journal().to_string();
//! let journal = Journal::from_jsonl(&jsonl);
//! let report = replay_journal(&journal);
//! assert!(report.faithful());
//! ```

use physis_theory::VerdictDiff;

use crate::journal::{Journal, JournalEvent};
use crate::lab::Lab;

fn diffs_replay_match(recorded: &[VerdictDiff], recomputed: &[VerdictDiff]) -> bool {
    recorded.len() == recomputed.len()
        && recorded
            .iter()
            .zip(recomputed)
            .all(|(r, c)| r.replay_matches(c))
}

/// Result of replaying one recorded `set-knob` event.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayStep {
    /// Theory id the knob turn targeted.
    pub theory: String,
    /// Knob name.
    pub knob: String,
    /// Raw value applied on replay (the recorded `to`, rendered as a token).
    pub value: String,
    /// Verdict diffs the journal recorded for this turn.
    pub recorded: Vec<VerdictDiff>,
    /// Verdict diffs recomputed by replaying the turn.
    pub recomputed: Vec<VerdictDiff>,
    /// Error message if the knob turn failed on replay (e.g. unknown theory).
    pub error: Option<String>,
}

impl ReplayStep {
    /// True when replay reproduced the recorded diffs with no error.
    ///
    /// Kind triples always compare. Extra scientific axes compare only
    /// when the journal record carries them, so a pre-axis JSONL still
    /// certifies against a live recompute that now emits those fields.
    pub fn faithful(&self) -> bool {
        self.error.is_none() && diffs_replay_match(&self.recorded, &self.recomputed)
    }
}

/// Outcome of replaying a whole journal.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReplayReport {
    /// One entry per replayed `set-knob` event, in journal order.
    pub steps: Vec<ReplayStep>,
}

impl ReplayReport {
    /// True when every replayed step reproduced its recorded diffs.
    pub fn faithful(&self) -> bool {
        self.steps.iter().all(ReplayStep::faithful)
    }

    /// Number of `set-knob` steps replayed.
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Whether any `set-knob` steps were replayed.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Count of steps that reproduced faithfully.
    pub fn faithful_count(&self) -> usize {
        self.steps.iter().filter(|s| s.faithful()).count()
    }

    /// Human-readable report, suitable for the CLI and journals.
    pub fn render(&self) -> String {
        let mut out = String::from("replay\n");
        if self.steps.is_empty() {
            out.push_str("  (no set-knob events to replay)\n");
            return out;
        }
        for (i, s) in self.steps.iter().enumerate() {
            let status = if s.faithful() { "ok" } else { "MISMATCH" };
            out.push_str(&format!(
                "  [{i:>3}] {:<18} {:<18} = {:<8} {status}\n",
                s.theory, s.knob, s.value
            ));
            if let Some(e) = &s.error {
                out.push_str(&format!("        error: {e}\n"));
            } else if !s.faithful() {
                out.push_str(&format!("        recorded:   {:?}\n", s.recorded));
                out.push_str(&format!("        recomputed: {:?}\n", s.recomputed));
            }
        }
        out.push_str(&format!(
            "\n{}/{} steps reproduced; journal is {}\n",
            self.faithful_count(),
            self.steps.len(),
            if self.faithful() {
                "faithful"
            } else {
                "NOT faithful"
            }
        ));
        out
    }
}

/// Replay a journal's `set-knob` events onto a fresh [`Lab::standard`].
///
/// Non-`set-knob` events (boot, run, experiment, prove, review, loop,
/// evidence, enclose, cite) are ignored. File restore ([`Lab::restore_from_journal`]) is
/// what reconstitutes receipts, reviews, and evidence graphs. The returned
/// [`ReplayReport`] pairs each recorded diff with the diff recomputed on a
/// clean lab so callers can prove — or disprove — reproducibility.
pub fn replay_journal(journal: &Journal) -> ReplayReport {
    let mut lab = Lab::standard();
    let mut steps = Vec::new();
    for ev in journal.events() {
        if let JournalEvent::SetKnob {
            theory,
            knob,
            to,
            diffs,
            ..
        } = ev
        {
            let value = to.display();
            let (recomputed, error) = match lab.set_knob(theory, knob, &value) {
                Ok((_, _, d)) => (d, None),
                Err(e) => (Vec::new(), Some(e.to_string())),
            };
            steps.push(ReplayStep {
                theory: theory.clone(),
                knob: knob.clone(),
                value,
                recorded: diffs.clone(),
                recomputed,
                error,
            });
        }
    }
    ReplayReport { steps }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn recorded_session() -> Lab {
        let mut lab = Lab::standard();
        lab.set_knob("type-iib", "total_dim", "9").unwrap();
        lab.set_knob("standard-model", "generations", "2").unwrap();
        lab.set_knob("observer-geometry", "derive_gauge", "false")
            .unwrap();
        lab.set_knob("type-iib", "supersymmetry", "false").unwrap();
        lab
    }

    #[test]
    fn recorded_session_replays_faithfully() {
        let lab = recorded_session();

        // Round-trip through JSONL exactly as a later agent would from disk.
        let jsonl = lab.journal().to_string();
        let journal = Journal::from_jsonl(&jsonl);
        let report = replay_journal(&journal);

        assert_eq!(report.len(), 4, "one step per recorded set-knob");
        assert!(report.faithful(), "{}", report.render());
        for step in &report.steps {
            assert!(
                diffs_replay_match(&step.recorded, &step.recomputed),
                "{:?}",
                step
            );
        }

        // The flagship theorem flip survives the round-trip.
        assert!(report.steps[0]
            .recomputed
            .iter()
            .any(|d| d.claim == "consistency.critical-dimension"));
    }

    #[test]
    fn replay_catches_a_tampered_journal() {
        // A journal that claims a diff the mechanics do not produce must fail.
        let mut lab = Lab::standard();
        lab.set_knob("type-iib", "total_dim", "9").unwrap();
        let tampered = lab.journal().to_string().replace(
            "consistency.critical-dimension",
            "consistency.spacetime-structure",
        );

        let journal = Journal::from_jsonl(&tampered);
        let report = replay_journal(&journal);

        assert!(!report.faithful(), "tampered claim id must not reproduce");
    }

    #[test]
    fn replay_reports_a_failed_knob_turn() {
        // A recorded turn against a theory absent from the standard lab errors.
        let jsonl = r#"{"event":"set-knob","t":1,"theory":"no-such-theory","knob":"x","from":{"kind":"u-int","value":1},"to":{"kind":"u-int","value":2},"diffs":[]}"#;
        let journal = Journal::from_jsonl(jsonl);
        let report = replay_journal(&journal);
        assert_eq!(report.len(), 1);
        assert!(report.steps[0].error.is_some());
        assert!(!report.faithful());
    }

    #[test]
    fn resumed_multi_run_session_replays_faithfully() {
        // Run 1: a fresh process turns one knob and persists the journal.
        let mut run1 = Lab::standard();
        run1.set_knob("type-iib", "total_dim", "9").unwrap();
        let jsonl1 = run1.journal().to_string();

        // Run 2: a new process loads that journal, restores state, then turns
        // another knob on the *same* theory. Without restore, this second turn
        // would be computed against fresh defaults and would not replay.
        let mut run2 = Lab::standard();
        *run2.journal_mut() = Journal::from_jsonl(&jsonl1);
        run2.restore_from_journal();
        run2.set_knob("type-iib", "supersymmetry", "false").unwrap();
        let jsonl2 = run2.journal().to_string();

        // The accumulated session must replay faithfully on a fresh lab.
        let report = replay_journal(&Journal::from_jsonl(&jsonl2));
        assert_eq!(report.len(), 2);
        assert!(report.faithful(), "{}", report.render());
    }

    #[test]
    fn empty_journal_is_trivially_faithful() {
        let report = replay_journal(&Journal::memory());
        assert!(report.is_empty());
        assert!(report.faithful());
    }

    fn strip_axis_fields(jsonl: &str) -> String {
        jsonl
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|line| {
                let mut v: serde_json::Value = serde_json::from_str(line).unwrap();
                if let Some(diffs) = v.get_mut("diffs").and_then(|d| d.as_array_mut()) {
                    for d in diffs {
                        if let Some(obj) = d.as_object_mut() {
                            for k in [
                                "statement_hash",
                                "from_derivation",
                                "to_derivation",
                                "from_empirical",
                                "to_empirical",
                                "from_judgment",
                                "to_judgment",
                            ] {
                                obj.remove(k);
                            }
                        }
                    }
                }
                serde_json::to_string(&v).unwrap()
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n"
    }

    #[test]
    fn legacy_kind_only_journal_replays_faithfully() {
        let mut lab = Lab::standard();
        lab.set_knob("type-iib", "total_dim", "9").unwrap();
        lab.set_knob("klein-gordon", "spacing", "100").unwrap();
        let jsonl = lab.journal().to_string();
        let stripped = strip_axis_fields(&jsonl);
        assert!(
            !stripped.contains("from_judgment"),
            "strip must drop axis fields: {stripped}"
        );
        let report = replay_journal(&Journal::from_jsonl(&stripped));
        assert!(report.faithful(), "{}", report.render());
        assert_eq!(report.len(), 2);
        assert_ne!(
            report.steps[0].recorded, report.steps[0].recomputed,
            "live recompute must still carry scientific axes"
        );
        assert!(report.steps[1]
            .recomputed
            .iter()
            .any(|d| d.claim == "field.second-order-accurate"
                && d.to_judgment.as_deref() == Some("numeric unresolved")));
    }

    #[test]
    fn tampered_empirical_axis_is_not_faithful() {
        let mut lab = Lab::standard();
        lab.set_knob("klein-gordon", "spacing", "100").unwrap();
        let jsonl = lab.journal().to_string();
        assert!(
            jsonl.contains("\"to_empirical\":\"inconclusive\""),
            "journal must record the empirical axis: {jsonl}"
        );
        let tampered = jsonl.replace(
            "\"to_empirical\":\"inconclusive\"",
            "\"to_empirical\":\"compatible\"",
        );
        let report = replay_journal(&Journal::from_jsonl(&tampered));
        assert!(
            !report.faithful(),
            "forged empirical axis must not certify: {}",
            report.render()
        );
    }
}
