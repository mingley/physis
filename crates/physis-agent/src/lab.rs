//! The laboratory: theories, knobs, experiments, journal.

use std::collections::BTreeMap;

use physis_core::claim::VerdictKind;
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::KnobValue;
use physis_theory::critique::diff_verdicts;
use physis_theory::{
    string_critique, ExperimentReport, GeneralRelativity, ObserverGeometry, StandardModel,
    StringTheory, Theory, VerdictDiff,
};

use crate::journal::{Journal, JournalEvent};
use crate::protocol::{Command, Response};

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
        lab.insert(Box::new(StringTheory::heterotic_e8()));
        lab.insert(Box::new(StringTheory::bosonic()));
        lab.insert(Box::new(ObserverGeometry::default()));
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
        self.journal.record(JournalEvent::SetKnob {
            t: 0,
            theory: theory.into(),
            knob: knob.into(),
            from: old.clone(),
            to: value.clone(),
            diffs: diffs.clone(),
        });
        Ok((old, value, diffs))
    }

    /// Canonical experiment (fresh default knobs).
    pub fn experiment_canonical(&mut self, id: &str) -> Result<ExperimentReport, CoreError> {
        match id {
            "string-critique" => {
                let report = string_critique();
                self.journal.record(JournalEvent::Experiment {
                    t: 0,
                    id: id.into(),
                });
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
                self.journal.record(JournalEvent::Run {
                    t: 0,
                    theory: theory.clone(),
                    holds,
                    fails,
                    other,
                });
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
                    report: Some(report),
                    diffs: None,
                },
                Err(e) => Response::err(e.to_string()),
            },
            Command::Journal => Response::ok(self.journal.to_string()),
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
}
