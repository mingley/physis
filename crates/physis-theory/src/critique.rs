//! The string-critique experiment: one matrix of claims, several theories.
//!
//! This lab does **not** decide whether string theory is false. It makes
//! the distinctive structural claims of string constructions and of a
//! unique-geometry program *mechanically comparable*, including the
//! landscape / uniqueness objection popularized in public physics
//! arguments (Weinstein and others).

use std::collections::BTreeMap;

use physis_core::claim::{Claim, Verdict, VerdictKind};
use physis_core::Judgment;
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

/// A before/after scientific change after a knob turn.
///
/// `from` / `to` remain the Level-2 evaluator kinds so a pre-axis JSONL
/// journal still deserializes. Orthogonal axes (derivation, empirical,
/// projected judgment) are optional strings: absent on legacy records,
/// always present on diffs this lab now emits. Replay compares kind for
/// every record and compares extra axes only when the record carries them.
/// Judgment labels are kebab phrases from [`Judgment::from_lab`] with
/// `dual_checked = false` — a knob turn does not mint a kernel proof.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerdictDiff {
    /// Claim slug (lab id). Not the statement hash.
    pub claim: String,
    /// Previous kind.
    pub from: VerdictKind,
    /// New kind.
    pub to: VerdictKind,
    /// Content-addressed identity of the sentence that moved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_hash: Option<String>,
    /// Previous derivation assurance (`executed`, `certified-numeric`, …).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_derivation: Option<String>,
    /// New derivation assurance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_derivation: Option<String>,
    /// Previous empirical status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_empirical: Option<String>,
    /// New empirical status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_empirical: Option<String>,
    /// Previous projected judgment label (`logical undetermined`, …).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_judgment: Option<String>,
    /// New projected judgment label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_judgment: Option<String>,
}

impl VerdictDiff {
    /// Diff of one claim that moved on a scientific axis.
    pub fn from_pair(claim: &Claim, before: &Verdict, after: &Verdict) -> Self {
        Self {
            claim: claim.id_str().to_string(),
            from: before.kind,
            to: after.kind,
            statement_hash: Some(claim.statement_hash().to_hex()),
            from_derivation: Some(before.derivation().as_str().into()),
            to_derivation: Some(after.derivation().as_str().into()),
            from_empirical: Some(before.empirical().as_str().into()),
            to_empirical: Some(after.empirical().as_str().into()),
            from_judgment: Some(evaluator_judgment(before)),
            to_judgment: Some(evaluator_judgment(after)),
        }
    }

    /// True when `live` reproduces this recorded diff.
    ///
    /// Kind and slug always compare. Axis fields compare only when this
    /// record carries them, so a pre-axis journal remains faithful.
    pub fn replay_matches(&self, live: &Self) -> bool {
        self.claim == live.claim
            && self.from == live.from
            && self.to == live.to
            && axis_replay_ok(&self.statement_hash, &live.statement_hash)
            && axis_replay_ok(&self.from_derivation, &live.from_derivation)
            && axis_replay_ok(&self.to_derivation, &live.to_derivation)
            && axis_replay_ok(&self.from_empirical, &live.from_empirical)
            && axis_replay_ok(&self.to_empirical, &live.to_empirical)
            && axis_replay_ok(&self.from_judgment, &live.from_judgment)
            && axis_replay_ok(&self.to_judgment, &live.to_judgment)
    }

    /// CLI lines: kind always, other axes only when they moved.
    pub fn render(&self) -> String {
        let mut out = format!(
            "  {:<32} {} → {}\n",
            self.claim,
            self.from.as_str(),
            self.to.as_str()
        );
        push_axis_line(
            &mut out,
            "derivation:",
            &self.from_derivation,
            &self.to_derivation,
        );
        push_axis_line(
            &mut out,
            "empirical:",
            &self.from_empirical,
            &self.to_empirical,
        );
        push_axis_line(
            &mut out,
            "judgment:",
            &self.from_judgment,
            &self.to_judgment,
        );
        out
    }
}

fn axis_replay_ok(recorded: &Option<String>, live: &Option<String>) -> bool {
    match recorded {
        None => true,
        Some(v) => live.as_deref() == Some(v.as_str()),
    }
}

fn push_axis_line(out: &mut String, name: &str, from: &Option<String>, to: &Option<String>) {
    if let (Some(a), Some(b)) = (from, to) {
        if a != b {
            out.push_str(&format!("    {name:<12} {a} → {b}\n"));
        }
    }
}

/// Project a set-time judgment. Knobs do not mint a dual-checked receipt.
fn evaluator_judgment(v: &Verdict) -> String {
    Judgment::from_lab(
        v.class,
        v.kind,
        v.empirical(),
        v.derivation(),
        false,
        v.numeric_lo(),
        v.numeric_hi(),
        v.statistical_nll(),
    )
    .label()
}

fn scientific_axes_changed(before: &Verdict, after: &Verdict) -> bool {
    before.kind != after.kind
        || before.derivation() != after.derivation()
        || before.empirical() != after.empirical()
        || before.statistical_nll() != after.statistical_nll()
        || evaluator_judgment(before) != evaluator_judgment(after)
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
            id: c.id_str().to_string(),
            statement: c.statement().to_string(),
            layer: c.layer().as_str().into(),
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
///
/// A row is emitted when the evaluator kind **or** derivation **or**
/// empirical status **or** projected judgment label changes. Kind-only
/// silence used to hide a coarse lattice leaving `|k a| < 1` (Holds stays
/// off the page while empirical/judgment move).
pub fn diff_verdicts(before: &[(Claim, Verdict)], after: &[(Claim, Verdict)]) -> Vec<VerdictDiff> {
    let mut diffs = Vec::new();
    for (c, vb) in before {
        if let Some((_, va)) = after.iter().find(|(ca, _)| ca.id() == c.id()) {
            if scientific_axes_changed(vb, va) {
                diffs.push(VerdictDiff::from_pair(c, vb, va));
            }
        }
    }
    diffs
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::knob::Knobbed;

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

    #[test]
    fn unique_vacuum_encodings_name_distinct_regimes() {
        use crate::geometry::ObserverGeometry;
        use crate::relativity::GeneralRelativity;
        use crate::standard_model::StandardModel;
        use crate::strings::StringTheory;

        let iib_t = StringTheory::type_iib();
        let het_t = StringTheory::heterotic_e8();
        let og_t = ObserverGeometry::default();
        let gr_t = GeneralRelativity::default();
        let sm_t = StandardModel::default();
        let claim = |t: &dyn Theory| {
            t.claims()
                .into_iter()
                .find(|c| c.id_str() == claims::UNIQUE_VACUUM)
                .unwrap()
        };
        let iib = claim(&iib_t);
        let het = claim(&het_t);
        let og = claim(&og_t);
        let gr = claim(&gr_t);
        let sm = claim(&sm_t);
        for c in [&iib, &het, &og, &gr, &sm] {
            assert!(
                !c.domain().is_encoding_wide(),
                "{} stays encoding-wide: {:?}",
                c.statement(),
                c.domain()
            );
        }
        assert_eq!(iib.statement_hash(), het.statement_hash());
        assert_ne!(iib.statement_hash(), og.statement_hash());
        assert_ne!(iib.statement_hash(), gr.statement_hash());
        assert_ne!(iib.statement_hash(), sm.statement_hash());
        assert_ne!(og.statement_hash(), gr.statement_hash());
        assert_ne!(og.statement_hash(), sm.statement_hash());
        assert_ne!(gr.statement_hash(), sm.statement_hash());
        assert!(iib
            .domain()
            .regimes
            .iter()
            .any(|r| r.contains("flux/moduli landscape")));
        assert!(og
            .domain()
            .regimes
            .iter()
            .any(|r| r.contains("unique_vacuum program axiom")));
        assert!(gr
            .domain()
            .regimes
            .iter()
            .any(|r| r.contains("Einstein-Hilbert")));
        assert!(sm
            .domain()
            .regimes
            .iter()
            .any(|r| r.contains("Higgs vacuum")));
    }

    #[test]
    fn coarse_lattice_diffs_empirical_and_judgment_not_just_kind() {
        use crate::continuum::{KleinGordonField, SECOND_ORDER};
        use physis_core::knob::KnobValue;

        let mut f = KleinGordonField::default();
        let before = f.evaluate_all();
        f.set("spacing", KnobValue::Float(100.0)).unwrap();
        let after = f.evaluate_all();
        let diffs = diff_verdicts(&before, &after);
        let d = diffs
            .iter()
            .find(|d| d.claim == SECOND_ORDER)
            .expect("second-order row");
        assert_eq!(d.from, VerdictKind::Holds);
        assert_eq!(d.to, VerdictKind::Undecidable);
        assert_eq!(d.from_empirical.as_deref(), Some("not-applicable"));
        assert_eq!(d.to_empirical.as_deref(), Some("inconclusive"));
        assert_eq!(d.from_judgment.as_deref(), Some("logical undetermined"));
        assert_eq!(d.to_judgment.as_deref(), Some("numeric unresolved"));
        assert_eq!(d.from_derivation.as_deref(), Some("executed"));
        assert_eq!(d.to_derivation.as_deref(), Some("executed"));
        assert!(
            d.statement_hash.as_ref().is_some_and(|h| h.len() == 64),
            "identity must be content-addressed: {:?}",
            d.statement_hash
        );
        let rendered = d.render();
        assert!(rendered.contains("holds → undecidable"), "{rendered}");
        assert!(
            rendered.contains("empirical:") && rendered.contains("not-applicable → inconclusive"),
            "{rendered}"
        );
        assert!(
            rendered.contains("judgment:")
                && rendered.contains("logical undetermined → numeric unresolved"),
            "{rendered}"
        );
        assert!(
            !rendered.contains("derivation:"),
            "executed → executed is not a causal axis: {rendered}"
        );
    }

    #[test]
    fn gut_interval_diffs_empirical_excluded_to_inconclusive() {
        use crate::gut::{Su5Gut, GUT_WEINBERG_ANGLE_MZ_INTERVAL};
        use physis_core::knob::KnobValue;

        let mut g = Su5Gut::default();
        let before = g.evaluate_all();
        g.set("supersymmetric", KnobValue::Bool(true)).unwrap();
        let after = g.evaluate_all();
        let diffs = diff_verdicts(&before, &after);
        let d = diffs
            .iter()
            .find(|d| d.claim == GUT_WEINBERG_ANGLE_MZ_INTERVAL)
            .expect("interval row");
        assert_eq!(d.from, VerdictKind::Fails);
        assert_eq!(d.to, VerdictKind::Undecidable);
        assert_eq!(d.from_empirical.as_deref(), Some("excluded"));
        assert_eq!(d.to_empirical.as_deref(), Some("inconclusive"));
        assert_eq!(d.from_judgment.as_deref(), Some("statistical computed"));
        assert_eq!(d.to_judgment.as_deref(), Some("statistical computed"));
    }

    #[test]
    fn gut_alpha3_interval_diffs_empirical_excluded_to_inconclusive() {
        use crate::gut::{Su5Gut, GUT_COUPLING_UNIFICATION_INTERVAL};
        use physis_core::knob::KnobValue;

        let mut g = Su5Gut::default();
        let before = g.evaluate_all();
        g.set("supersymmetric", KnobValue::Bool(true)).unwrap();
        let after = g.evaluate_all();
        let diffs = diff_verdicts(&before, &after);
        let d = diffs
            .iter()
            .find(|d| d.claim == GUT_COUPLING_UNIFICATION_INTERVAL)
            .expect("α_3 interval row");
        assert_eq!(d.from, VerdictKind::Fails);
        assert_eq!(d.to, VerdictKind::Undecidable);
        assert_eq!(d.from_empirical.as_deref(), Some("excluded"));
        assert_eq!(d.to_empirical.as_deref(), Some("inconclusive"));
        assert_eq!(d.from_judgment.as_deref(), Some("statistical computed"));
        assert_eq!(d.to_judgment.as_deref(), Some("statistical computed"));
    }

    #[test]
    fn gut_inv_alpha_interval_stays_excluded_under_susy() {
        use crate::gut::{Su5Gut, GUT_INVERSE_ALPHA_EM_MZ_INTERVAL};
        use physis_core::knob::KnobValue;

        let mut g = Su5Gut::default();
        let before = g.evaluate_all();
        g.set("supersymmetric", KnobValue::Bool(true)).unwrap();
        let after = g.evaluate_all();
        let diffs = diff_verdicts(&before, &after);
        let d = diffs
            .iter()
            .find(|d| d.claim == GUT_INVERSE_ALPHA_EM_MZ_INTERVAL)
            .expect("inv-alpha-em interval row");
        assert_eq!(d.from, VerdictKind::Fails);
        assert_eq!(d.to, VerdictKind::Fails);
        assert_eq!(d.from_empirical.as_deref(), Some("excluded"));
        assert_eq!(d.to_empirical.as_deref(), Some("excluded"));
        assert_eq!(d.from_judgment.as_deref(), Some("statistical computed"));
        assert_eq!(d.to_judgment.as_deref(), Some("statistical computed"));
    }

    #[test]
    fn gut_proton_lifetime_diffs_empirical_excluded_to_compatible() {
        use crate::gut::{Su5Gut, GUT_PROTON_LIFETIME_SK};
        use physis_core::knob::KnobValue;

        let mut g = Su5Gut::default();
        let before = g.evaluate_all();
        g.set("supersymmetric", KnobValue::Bool(true)).unwrap();
        let after = g.evaluate_all();
        let diffs = diff_verdicts(&before, &after);
        let d = diffs
            .iter()
            .find(|d| d.claim == GUT_PROTON_LIFETIME_SK)
            .expect("super-k row");
        assert_eq!(d.from, VerdictKind::Fails);
        assert_eq!(d.to, VerdictKind::Holds);
        assert_eq!(d.from_empirical.as_deref(), Some("excluded"));
        assert_eq!(d.to_empirical.as_deref(), Some("compatible"));
        assert_eq!(d.from_judgment.as_deref(), Some("empirical excluded"));
        assert_eq!(d.to_judgment.as_deref(), Some("empirical compatible"));
    }

    #[test]
    fn legacy_kind_only_record_matches_live_axes() {
        use crate::strings::StringTheory;
        use physis_core::knob::KnobValue;

        let mut t = StringTheory::type_iib();
        let before = t.evaluate_all();
        t.set("total_dim", KnobValue::UInt(9)).unwrap();
        let after = t.evaluate_all();
        let live = diff_verdicts(&before, &after);
        let crit = live
            .iter()
            .find(|d| d.claim == "consistency.critical-dimension")
            .expect("critical dimension");
        let legacy = VerdictDiff {
            claim: "consistency.critical-dimension".into(),
            from: VerdictKind::Holds,
            to: VerdictKind::Fails,
            statement_hash: None,
            from_derivation: None,
            to_derivation: None,
            from_empirical: None,
            to_empirical: None,
            from_judgment: None,
            to_judgment: None,
        };
        assert!(
            legacy.replay_matches(crit),
            "pre-axis journals must still certify: live={crit:?}"
        );
        assert_ne!(
            legacy, *crit,
            "live diffs carry axes the legacy record lacks"
        );
        assert_eq!(crit.from_judgment.as_deref(), Some("logical undetermined"));
        assert_eq!(crit.to_judgment.as_deref(), Some("logical disproved"));
        let rendered = crit.render();
        assert!(
            rendered.contains("logical undetermined → logical disproved"),
            "{rendered}"
        );
    }
}
