//! The string-critique experiment: one matrix of claims, several theories.
//!
//! This lab does **not** decide whether string theory is false. It makes
//! the distinctive structural claims of string constructions and of a
//! unique-geometry program *mechanically comparable*, including the
//! landscape / uniqueness objection popularized in public physics
//! arguments (Weinstein and others).

use std::collections::BTreeMap;

use physis_core::claim::{Claim, Verdict, VerdictKind};
use serde::{Deserialize, Serialize};

use crate::claims;
use crate::framework::Theory;
use crate::geometry::ObserverGeometry;
use crate::relativity::GeneralRelativity;
use crate::standard_model::StandardModel;
use crate::strings::StringTheory;

/// One theory's full evaluation.
#[derive(Clone, Debug, Serialize)]
pub struct TheoryReport {
    /// Lab id.
    pub id: String,
    /// Title.
    pub name: String,
    /// What the object is.
    pub summary: String,
    /// World projection note.
    pub world_note: String,
    /// Landscape log10.
    pub landscape_log10: f64,
    /// Free parameter count (heuristic).
    pub free_parameter_count: u32,
    /// Claim evaluations.
    pub verdicts: Vec<ClaimVerdict>,
}

/// A claim plus its verdict.
#[derive(Clone, Debug, Serialize)]
pub struct ClaimVerdict {
    /// Claim id.
    pub id: String,
    /// Statement.
    pub statement: String,
    /// Layer.
    pub layer: String,
    /// Verdict kind.
    pub kind: VerdictKind,
    /// Claim class (mathematical, model-internal, conjecture, …).
    pub class: physis_core::ClaimClass,
    /// Derivation assurance (executed, asserted, …). Never a kernel proof.
    pub derivation: physis_core::DerivationAssurance,
    /// Summary.
    pub summary: String,
    /// Evidence lines.
    pub evidence: Vec<String>,
}

/// Comparison matrix: claim id → theory id → verdict kind.
#[derive(Clone, Debug, Serialize)]
pub struct ExperimentReport {
    /// Experiment id.
    pub id: &'static str,
    /// Title.
    pub title: String,
    /// What is being asked.
    pub question: String,
    /// Scientific honesty note.
    pub honesty: String,
    /// Per-theory reports.
    pub theories: Vec<TheoryReport>,
    /// Claim ids used as matrix rows, in display order.
    pub rows: Vec<String>,
    /// Honesty / reading notes rendered under the matrix.
    pub notes: Vec<String>,
    /// Row-major matrix using shared claim ids.
    pub matrix: BTreeMap<String, BTreeMap<String, VerdictKind>>,
}

/// A before/after verdict change after a knob turn.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerdictDiff {
    /// Claim id.
    pub claim: String,
    /// Previous kind.
    pub from: VerdictKind,
    /// New kind.
    pub to: VerdictKind,
}

impl ExperimentReport {
    /// Pretty text for CLI and journals.
    pub fn render(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# {}\n\n", self.title));
        out.push_str(&format!("{}\n\n", self.question));
        out.push_str(&format!("Honesty: {}\n\n", self.honesty));

        out.push_str("## Theories\n\n");
        for t in &self.theories {
            out.push_str(&format!(
                "- **{}** (`{}`): landscape ~10^{:.1}, free params {}, {}\n",
                t.name, t.id, t.landscape_log10, t.free_parameter_count, t.world_note
            ));
        }

        out.push_str("\n## Claim matrix\n\n");
        let ids: Vec<&str> = self.theories.iter().map(|t| t.id.as_str()).collect();
        out.push_str("| claim |");
        for id in &ids {
            out.push_str(&format!(" {id} |"));
        }
        out.push('\n');
        out.push_str("|---|");
        for _ in &ids {
            out.push_str("---|");
        }
        out.push('\n');

        for row in &self.rows {
            out.push_str(&format!("| `{row}` |"));
            for id in &ids {
                let cell = self
                    .matrix
                    .get(row)
                    .and_then(|m| m.get(*id))
                    .map(|k| k.as_str())
                    .unwrap_or("—");
                out.push_str(&format!(" {cell} |"));
            }
            out.push('\n');
        }

        out.push_str("\n## Notes\n\n");
        for note in &self.notes {
            out.push_str(&format!("- {note}\n"));
        }
        out
    }
}

fn report_of(t: &dyn Theory) -> TheoryReport {
    let world = t.world();
    let landscape_log10 = world.as_ref().map(|w| w.landscape_log10).unwrap_or(0.0);
    let free_parameter_count = world.as_ref().map(|w| w.free_parameter_count).unwrap_or(0);
    let verdicts = t
        .evaluate_all()
        .into_iter()
        .map(|(c, v)| ClaimVerdict {
            id: c.id.0,
            statement: c.statement,
            layer: c.layer.as_str().into(),
            kind: v.kind,
            class: v.class,
            derivation: v.derivation(),
            summary: v.summary,
            evidence: v.evidence,
        })
        .collect();
    TheoryReport {
        id: t.id().into(),
        name: t.name().into(),
        summary: t.summary().into(),
        world_note: t.note(),
        landscape_log10,
        free_parameter_count,
        verdicts,
    }
}

fn evaluate_id(t: &dyn Theory, id: &str) -> VerdictKind {
    let claim = Claim::new(
        id,
        "",
        physis_core::LayerId::Mathematical,
        physis_core::ClaimClass::OpenProblem,
    );
    t.evaluate(&claim).kind
}

/// Run the default string-critique lab.
pub fn string_critique() -> ExperimentReport {
    let theories: Vec<Box<dyn Theory>> = vec![
        Box::new(StandardModel::default()),
        Box::new(GeneralRelativity::default()),
        Box::new(StringTheory::type_iib()),
        Box::new(StringTheory::type_iia()),
        Box::new(StringTheory::type_i()),
        Box::new(StringTheory::heterotic_e8()),
        Box::new(StringTheory::heterotic_so32()),
        Box::new(StringTheory::bosonic()),
        Box::new(StringTheory::m_theory()),
        Box::new(ObserverGeometry::default()),
    ];
    report_from(theories)
}

/// Build a report from a list of theories (used by the agent lab).
pub fn report_from(theories: Vec<Box<dyn Theory>>) -> ExperimentReport {
    report_from_rows(
        "string-critique",
        "String critique lab",
        "Which structural claims of string constructions, the Standard Model, GR, \
         and a unique-geometry scaffold hold under their default knobs — and which \
         flip when knobs move? In particular: does uniqueness/predictivity fail for \
         strings in a way that is *mechanical* in this encoding, and does an \
         alternative program actually *earn* empirical contact, or only assert it?",
        "This experiment compares encoded structures. It cannot settle whether \
         nature is a string, a geometry, or something else. Landscape counts are \
         heuristics. Observer-geometry gauge assignment is a conjecture. Critical \
         dimensions of strings are executed model-internal claims, not kernel proofs.",
        vec![
            "`holds` / `fails` are *internal to the encoding*.".into(),
            "Read `class` and `derivation` before treating a cell as physics. `executed` is not a kernel proof.".into(),
            "Type IIB uniqueness failing under a landscape heuristic is not a disproof of string theory; it is the predictivity objection made inspectable.".into(),
            "Observer-geometry uniqueness holding as a *conjecture/axiom* is not a proof that geometry succeeds.".into(),
        ],
        &claims::critique_rows(),
        theories,
    )
}

/// Build an experiment report over an explicit set of claim-id rows.
///
/// This is the domain-agnostic core: string-critique and the electromagnetism
/// lab both use it, so a new domain adds a theory list and a row list without
/// forking the report machinery.
#[allow(clippy::too_many_arguments)]
pub fn report_from_rows(
    id: &'static str,
    title: impl Into<String>,
    question: impl Into<String>,
    honesty: impl Into<String>,
    notes: Vec<String>,
    rows: &[&str],
    theories: Vec<Box<dyn Theory>>,
) -> ExperimentReport {
    let reports: Vec<TheoryReport> = theories.iter().map(|t| report_of(t.as_ref())).collect();
    let mut matrix: BTreeMap<String, BTreeMap<String, VerdictKind>> = BTreeMap::new();
    for row in rows {
        let mut cells = BTreeMap::new();
        for t in &theories {
            cells.insert(t.id().to_string(), evaluate_id(t.as_ref(), row));
        }
        matrix.insert((*row).to_string(), cells);
    }
    ExperimentReport {
        id,
        title: title.into(),
        question: question.into(),
        honesty: honesty.into(),
        theories: reports,
        rows: rows.iter().map(|s| s.to_string()).collect(),
        notes,
        matrix,
    }
}

/// Diff two evaluations of the same theory (after a knob turn).
pub fn diff_verdicts(before: &[(Claim, Verdict)], after: &[(Claim, Verdict)]) -> Vec<VerdictDiff> {
    let mut diffs = Vec::new();
    for (c, vb) in before {
        if let Some((_, va)) = after.iter().find(|(ca, _)| ca.id == c.id) {
            if va.kind != vb.kind {
                diffs.push(VerdictDiff {
                    claim: c.id.0.clone(),
                    from: vb.kind,
                    to: va.kind,
                });
            }
        }
    }
    diffs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_has_all_theories() {
        let r = string_critique();
        assert!(r.theories.len() >= 5);
        let uniq = r.matrix.get(claims::UNIQUE_VACUUM).expect("row");
        assert_eq!(uniq.get("type-iib").copied(), Some(VerdictKind::Fails));
        assert_eq!(
            uniq.get("observer-geometry").copied(),
            Some(VerdictKind::Holds)
        );
        let crit = r.matrix.get(claims::CRITICAL_DIMENSION).unwrap();
        assert_eq!(crit.get("type-iib").copied(), Some(VerdictKind::Holds));
        assert_eq!(
            crit.get("observer-geometry").copied(),
            Some(VerdictKind::Inapplicable)
        );
        assert_eq!(
            crit.get("standard-model").copied(),
            Some(VerdictKind::Inapplicable)
        );
    }
}
