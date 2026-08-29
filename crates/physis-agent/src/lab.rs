//! The laboratory: theories, knobs, experiments, journal.

use std::collections::BTreeMap;

use physis_core::claim::VerdictKind;
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::KnobValue;
use physis_theory::computation::{CombinationalCircuit, TuringMachine};
use physis_theory::continuum::KleinGordonField;
use physis_theory::critique::diff_verdicts;
use physis_theory::em::{LinearMedium, MaxwellVacuum, OhmCircuit};
use physis_theory::gauge_field::{WilsonSun, WilsonU1};
use physis_theory::{
    string_critique, ExperimentReport, GeneralRelativity, ObserverGeometry, StandardModel,
    StringTheory, Theory, VerdictDiff,
};

use crate::journal::{Journal, JournalEvent};
use crate::protocol::{Command, Response};
use crate::replay::replay_journal;

/// An agent-operable collection of theories.
pub struct Lab {
    theories: BTreeMap<String, Box<dyn Theory>>,
    journal: Journal,
}

impl Lab {
    /// Empty lab with an in-memory journal.
    pub fn empty() -> Self {
        Self {
            theories: BTreeMap::new(),
            journal: Journal::memory(),
        }
    }

    /// Default lab: SM, GR, Type IIB, heterotic E₈×E₈, bosonic, observer-geometry.
    pub fn standard() -> Self {
        let mut lab = Self::empty();
        lab.insert(Box::new(StandardModel::default()));
        lab.insert(Box::new(GeneralRelativity::default()));
        lab.insert(Box::new(StringTheory::type_iib()));
        lab.insert(Box::new(StringTheory::type_iia()));
        lab.insert(Box::new(StringTheory::type_i()));
        lab.insert(Box::new(StringTheory::heterotic_e8()));
        lab.insert(Box::new(StringTheory::heterotic_so32()));
        lab.insert(Box::new(StringTheory::bosonic()));
        lab.insert(Box::new(StringTheory::m_theory()));
        lab.insert(Box::new(ObserverGeometry::default()));
        // Second domain: electromagnetism shares the same lab and protocol.
        lab.insert(Box::new(MaxwellVacuum));
        lab.insert(Box::new(LinearMedium::default()));
        lab.insert(Box::new(OhmCircuit::default()));
        // Third domain: computation.
        lab.insert(Box::new(CombinationalCircuit));
        lab.insert(Box::new(TuringMachine::default()));
        // M4 continuum: a scalar field and lattice gauge fields as local objects.
        lab.insert(Box::new(KleinGordonField::default()));
        lab.insert(Box::new(WilsonU1::default()));
        lab.insert(Box::new(WilsonSun::su2()));
        lab.insert(Box::new(WilsonSun::su3()));
        let ids = lab.theories.keys().cloned().collect();
        lab.journal.record(JournalEvent::boot(ids));
        lab
    }

    /// Insert a theory under its `id()`.
    pub fn insert(&mut self, theory: Box<dyn Theory>) {
        self.theories.insert(theory.id().to_string(), theory);
    }

    /// Journal.
    pub fn journal(&self) -> &Journal {
        &self.journal
    }

    /// Mutable journal (for file persistence).
    pub fn journal_mut(&mut self) -> &mut Journal {
        &mut self.journal
    }

    /// Theory ids.
    pub fn theory_ids(&self) -> Vec<String> {
        self.theories.keys().cloned().collect()
    }

    /// Borrow a theory.
    pub fn theory(&self, id: &str) -> Result<&dyn Theory, CoreError> {
        self.theories
            .get(id)
            .map(|t| t.as_ref())
            .ok_or_else(|| CoreError::UnknownTheory { id: id.into() })
    }

    /// Borrow a theory mutably.
    pub fn theory_mut(&mut self, id: &str) -> Result<&mut dyn Theory, CoreError> {
        self.theories
            .get_mut(id)
            .map(|t| t.as_mut() as &mut dyn Theory)
            .ok_or_else(|| CoreError::UnknownTheory { id: id.into() })
    }

    /// Turn a knob, returning (old, new, verdict diffs).
    pub fn set_knob(
        &mut self,
        theory: &str,
        knob: &str,
        raw: &str,
    ) -> Result<(KnobValue, KnobValue, Vec<VerdictDiff>), CoreError> {
        if !self.theories.contains_key(theory) {
            return Err(CoreError::UnknownTheory { id: theory.into() });
        }
        let before = self.theories[theory].evaluate_all();
        let spec = self.theories[theory].spec(knob)?;
        let value = KnobValue::parse_in_domain(raw, &spec.domain).map_err(|mut e| {
            match &mut e {
                CoreError::TypeMismatch { name, .. } | CoreError::Domain { name, .. } => {
                    *name = knob.into();
                }
                _ => {}
            }
            e
        })?;
        let old = self
            .theories
            .get_mut(theory)
            .unwrap()
            .set(knob, value.clone())?;
        let after = self.theories[theory].evaluate_all();
        let diffs = diff_verdicts(&before, &after);
        self.journal.record(JournalEvent::set_knob(
            theory,
            knob,
            old.clone(),
            value.clone(),
            diffs.clone(),
        ));
        Ok((old, value, diffs))
    }

    /// Re-apply the `set-knob` events already in the journal to theory state,
    /// **without** recording them again.
    ///
    /// This resumes a persisted session: after loading a journal from a file,
    /// call this so subsequent turns build on the prior ones instead of on
    /// fresh defaults. It is what makes a multi-process `--journal` session a
    /// single coherent, replayable session rather than a bag of independent
    /// one-shot diffs.
    pub fn restore_from_journal(&mut self) {
        for ev in self.journal.events().to_vec() {
            if let JournalEvent::SetKnob {
                theory, knob, to, ..
            } = ev
            {
                if let Ok(t) = self.theory_mut(&theory) {
                    let _ = t.set(&knob, to);
                }
            }
        }
    }

    /// Canonical experiment (fresh default knobs).
    pub fn experiment_canonical(&mut self, id: &str) -> Result<ExperimentReport, CoreError> {
        match id {
            "string-critique" => {
                let report = string_critique();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            "em-vacuum" => {
                let report = physis_theory::em_vacuum();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            "computation" => {
                let report = physis_theory::computation();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            "field-modes" => {
                let report = physis_theory::field_modes();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            "gauge-lattice" => {
                let report = physis_theory::gauge_lattice();
                self.journal.record(JournalEvent::experiment(id));
                Ok(report)
            }
            other => Err(CoreError::UnknownTheory {
                id: format!("experiment:{other}"),
            }),
        }
    }

    /// Dispatch a protocol command.
    pub fn exec(&mut self, cmd: Command) -> Response {
        match cmd {
            Command::Layers => {
                let mut text = String::from("layers (finest → coarsest)\n");
                for layer in LayerId::ALL {
                    text.push_str(&format!("  {:<14} {}\n", layer.as_str(), layer.doc()));
                }
                Response::ok(text)
            }
            Command::Theories => {
                let mut text = String::from("theories\n");
                for (id, t) in &self.theories {
                    text.push_str(&format!("  {id:<22} {}\n", t.name()));
                    text.push_str(&format!("    {}\n", t.summary()));
                }
                Response::ok(text)
            }
            Command::Knobs { theory } => match theory {
                Some(id) => match self.theory(&id) {
                    Ok(t) => Response::ok(render_knobs(t)),
                    Err(e) => Response::err(e.to_string()),
                },
                None => {
                    let mut text = String::new();
                    for t in self.theories.values() {
                        text.push_str(&render_knobs(t.as_ref()));
                        text.push('\n');
                    }
                    Response::ok(text)
                }
            },
            Command::Run { theory } => {
                if !self.theories.contains_key(&theory) {
                    return Response::err(format!("unknown theory '{theory}'"));
                }
                let eval = self.theories[&theory].evaluate_all();
                let mut holds = 0;
                let mut fails = 0;
                let mut other = 0;
                let mut text = format!("run {theory}\n");
                for (c, v) in &eval {
                    match v.kind {
                        VerdictKind::Holds => holds += 1,
                        VerdictKind::Fails => fails += 1,
                        _ => other += 1,
                    }
                    text.push_str(&format!(
                        "  {:<32} {:<13} {:<12} {}\n",
                        c.id.0,
                        v.kind.as_str(),
                        v.epistemic.as_str(),
                        v.summary
                    ));
                }
                text.push_str(&format!("\nholds={holds} fails={fails} other={other}\n"));
                self.journal
                    .record(JournalEvent::run(theory.clone(), holds, fails, other));
                Response::ok(text)
            }
            Command::Set {
                theory,
                knob,
                value,
            } => match self.set_knob(&theory, &knob, &value) {
                Ok((from, to, diffs)) => {
                    let mut text =
                        format!("{theory}  {knob}: {} → {}\n", from.display(), to.display());
                    if diffs.is_empty() {
                        text.push_str("no verdict kinds changed\n");
                    } else {
                        text.push_str("verdict changes:\n");
                        for d in &diffs {
                            text.push_str(&format!(
                                "  {:<32} {} → {}\n",
                                d.claim,
                                d.from.as_str(),
                                d.to.as_str()
                            ));
                        }
                    }
                    Response::Ok {
                        text,
                        report: None,
                        diffs: Some(diffs),
                    }
                }
                Err(e) => Response::err(e.to_string()),
            },
            Command::Experiment { id } => match self.experiment_canonical(&id) {
                Ok(report) => Response::Ok {
                    text: report.render(),
                    report: Some(Box::new(report)),
                    diffs: None,
                },
                Err(e) => Response::err(e.to_string()),
            },
            Command::Journal => Response::ok(self.journal.to_string()),
            Command::Score { theory } => match self.theory(&theory) {
                Ok(t) => {
                    let card = physis_theory::score(&physis_theory::empirical_target(), t);
                    Response::ok(card.render())
                }
                Err(e) => Response::err(e.to_string()),
            },
            Command::Replay { path } => match std::fs::read_to_string(&path) {
                Ok(contents) => {
                    let (journal, malformed) = Journal::from_jsonl_counting(&contents);
                    // Refuse to certify a journal we could not fully parse:
                    // dropped lines would make an incomplete replay look faithful.
                    if malformed > 0 {
                        return Response::err(format!(
                            "journal '{path}': {malformed} malformed line(s); refusing to certify replay"
                        ));
                    }
                    let report = replay_journal(&journal);
                    if report.is_empty() {
                        return Response::err(format!(
                            "journal '{path}': no set-knob events to replay"
                        ));
                    }
                    // A non-faithful replay is a verification failure: exit non-zero.
                    if report.faithful() {
                        Response::ok(report.render())
                    } else {
                        Response::err(report.render())
                    }
                }
                Err(e) => Response::err(format!("cannot read journal '{path}': {e}")),
            },
        }
    }
}

fn render_knobs(t: &dyn Theory) -> String {
    let mut text = format!("knobs  {}\n", t.id());
    for (spec, val) in t.snapshot() {
        text.push_str(&format!(
            "  {:<24} {:<10} {}\n    {}\n",
            spec.name,
            val.display(),
            spec.layer.as_str(),
            spec.doc
        ));
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turning_iib_dimension_flips_critical_claim() {
        let mut lab = Lab::standard();
        let diffs = lab.set_knob("type-iib", "total_dim", "9").unwrap().2;
        assert!(
            diffs
                .iter()
                .any(|d| d.claim == "consistency.critical-dimension"
                    && d.from == VerdictKind::Holds
                    && d.to == VerdictKind::Fails),
            "expected critical-dimension Holds→Fails, got {diffs:?}"
        );
    }

    #[test]
    fn sm_generation_knob_flips_claim() {
        let mut lab = Lab::standard();
        let diffs = lab
            .set_knob("standard-model", "generations", "2")
            .unwrap()
            .2;
        assert!(diffs
            .iter()
            .any(|d| d.claim == "empirical.three-generations" && d.to == VerdictKind::Fails));
    }

    fn temp_path(tag: &str) -> std::path::PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        std::env::temp_dir().join(format!("physis_{tag}_{}_{nanos}.jsonl", std::process::id()))
    }

    #[test]
    fn replay_command_rejects_malformed_journal() {
        let path = temp_path("malformed");
        std::fs::write(
            &path,
            "this is not json\n{\"event\":\"boot\",\"t\":1,\"theories\":[]}\n",
        )
        .unwrap();
        let mut lab = Lab::standard();
        let resp = lab.exec(Command::Replay {
            path: path.to_string_lossy().into_owned(),
        });
        assert_eq!(resp.exit_code(), 1, "malformed journal must not certify");
        assert!(resp.text().contains("malformed"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn replay_command_rejects_journal_with_no_turns() {
        let path = temp_path("noturns");
        // Only a boot event — nothing to verify.
        std::fs::write(&path, "{\"event\":\"boot\",\"t\":1,\"theories\":[]}\n").unwrap();
        let mut lab = Lab::standard();
        let resp = lab.exec(Command::Replay {
            path: path.to_string_lossy().into_owned(),
        });
        assert_eq!(resp.exit_code(), 1, "empty session must not certify");
        assert!(resp.text().contains("no set-knob events"));
        let _ = std::fs::remove_file(&path);
    }
}
