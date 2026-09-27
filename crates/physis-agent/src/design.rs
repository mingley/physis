//! Empirical experiment design (R5, spec 022).
//!
//! Ranking of discriminating observables is pure analysis over live
//! verdict views plus the registered dataset inventory: no downloads,
//! no invented costs, no trust minted. [`rank_observables`] contrasts
//! each theory pair per observable (verdict-kind split first, then
//! empirical-status contrast, then Gaussian NLL gap), freezes one
//! comparison rule per observable from its likelihood model, and
//! discloses fitted inputs, covariance assumptions, and adaptive data
//! reuse. The lab collects the [`DesignInput`] from live state,
//! journals the frozen design id, and renders the report.

use std::collections::{BTreeMap, BTreeSet};

use physis_core::artifact::ArtifactId;
use physis_data::{dataset_for_claim, LikelihoodModel};
use physis_numeric::Ratio;

/// Maximum ranked observables per theory pair. The registry holds
/// fewer today; the bound keeps the report finite as it grows.
pub const MAX_DESIGN_OBSERVABLES: usize = 16;

/// Frozen comparison rule for Gaussian two-sided measurements.
pub const RULE_TWO_SIGMA_NLL: &str = "two-sigma + exact-nll";
/// Frozen comparison rule for one-sided limits and hull-only datasets.
pub const RULE_INTERVAL_SUBSET: &str = "interval-subset";

/// One theory's live verdict on one claim, as plain data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerdictView {
    /// Theory id.
    pub theory: String,
    /// Claim id.
    pub claim: String,
    /// Verdict kind (`holds`, `fails`, …).
    pub kind: String,
    /// Empirical status (`excluded`, `compatible`, …).
    pub empirical: String,
    /// Exact Gaussian NLL overlay, when the theory reports one.
    pub nll: Option<String>,
}

/// Everything the pure ranking needs. The lab collects this from live
/// evaluations, knob origins, and the journal; nothing here is trusted.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DesignInput {
    /// Theories to compare pairwise, in caller order.
    pub theories: Vec<String>,
    /// Live verdict views for the compared theories.
    pub verdicts: Vec<VerdictView>,
    /// `theory/knob` entries with fitted origin.
    pub fitted: Vec<String>,
    /// `theory/knob` entries with nuisance origin.
    pub nuisance: Vec<String>,
    /// Journaled cite uses per dataset id.
    pub cite_uses: BTreeMap<String, u64>,
    /// Verdict evaluations charged while collecting.
    pub compute_evaluations: u64,
}

/// One ranked observable for one theory pair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RankedObservable {
    /// Claim id.
    pub claim: String,
    /// Registered dataset id.
    pub dataset: String,
    /// Observable name.
    pub observable: String,
    /// Unit string.
    pub unit: String,
    /// Reachable measurement domain (combined hull).
    pub domain: String,
    /// Likelihood model (`gaussian` or `interval-subset`).
    pub likelihood: String,
    /// Frozen comparison rule for this observable.
    pub rule: String,
    /// Contrast class (`decisive`, `strong`, `weak`, `none`, `unknown`).
    pub contrast: String,
    /// Gaussian NLL gap in milli-NLL, for tie-breaking within a class.
    /// Zero when either theory reports no parseable NLL.
    pub nll_gap_milli: u64,
    /// Per-theory `kind/empirical` plus NLL, in pair order.
    pub stances: Vec<String>,
    /// Journaled cite uses of this dataset.
    pub prior_uses: u64,
}

/// Ranked observables for one theory pair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PairDesign {
    /// First theory id.
    pub a: String,
    /// Second theory id.
    pub b: String,
    /// Observables, best contrast first, bounded by
    /// [`MAX_DESIGN_OBSERVABLES`].
    pub rows: Vec<RankedObservable>,
}

/// A frozen experiment design over the requested theories.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExperimentDesign {
    /// Content id of the frozen design (theories, observables, rules,
    /// thresholds, fitted inputs).
    pub design_id: String,
    /// One ranking per theory pair.
    pub pairs: Vec<PairDesign>,
    /// Fitted `theory/knob` inputs disclosed by the design.
    pub fitted: Vec<String>,
    /// Nuisance `theory/knob` inputs disclosed by the design.
    pub nuisance: Vec<String>,
    /// Verdict evaluations charged while collecting.
    pub compute_evaluations: u64,
    /// Journaled cite uses per dataset id.
    pub cite_uses: BTreeMap<String, u64>,
}

/// Contrast class rank: lower discriminates better.
fn contrast_rank(contrast: &str) -> u8 {
    match contrast {
        "decisive" => 0,
        "strong" => 1,
        "weak" => 2,
        "none" => 3,
        _ => 4,
    }
}

/// Order rows by contrast (best first), NLL gap (widest first), then
/// claim id, and truncate to [`MAX_DESIGN_OBSERVABLES`]. Pure.
pub(crate) fn order_rows(mut rows: Vec<RankedObservable>) -> Vec<RankedObservable> {
    rows.sort_by(|a, b| {
        contrast_rank(&a.contrast)
            .cmp(&contrast_rank(&b.contrast))
            .then_with(|| b.nll_gap_milli.cmp(&a.nll_gap_milli))
            .then_with(|| a.claim.cmp(&b.claim))
    });
    rows.truncate(MAX_DESIGN_OBSERVABLES);
    rows
}

/// Empirical tier for contrast: excluded and compatible-like verdicts
/// decide; inconclusive is weak; anything else is unassessed.
fn status_tier(status: &str) -> Option<u8> {
    match status {
        "excluded" => Some(0),
        "compatible" | "supported" => Some(1),
        "inconclusive" => Some(2),
        _ => None,
    }
}

/// Contrast class for one pair of stances. A verdict-kind split is
/// decisive whatever the empirical axis says; otherwise the ordered
/// empirical pair decides; unassessed statuses are unknown, never
/// evidence of agreement.
fn contrast_class(a_kind: &str, a_emp: &str, b_kind: &str, b_emp: &str) -> &'static str {
    if a_kind != b_kind {
        return "decisive";
    }
    match (status_tier(a_emp), status_tier(b_emp)) {
        (Some(0), Some(1)) | (Some(1), Some(0)) => "strong",
        (Some(0), Some(2)) | (Some(2), Some(0)) | (Some(1), Some(2)) | (Some(2), Some(1)) => "weak",
        (Some(_), Some(_)) => "none",
        _ => "unknown",
    }
}

/// Exact NLL overlay value, when it parses as a reduced ratio.
fn nll_value(nll: Option<&str>) -> Option<f64> {
    Ratio::parse_display(nll?).map(|r| r.to_f64())
}

/// NLL gap in milli-NLL. Zero unless both sides parse; ranking-only
/// arithmetic, disclosed on the row.
fn nll_gap_milli(a: Option<&str>, b: Option<&str>) -> u64 {
    match (nll_value(a), nll_value(b)) {
        (Some(x), Some(y)) => ((x - y).abs() * 1000.0).round().max(0.0) as u64,
        _ => 0,
    }
}

/// One theory's stance line on a row.
fn stance_line(v: &VerdictView) -> String {
    match &v.nll {
        Some(nll) => format!("{}/{}/{} nll={nll}", v.theory, v.kind, v.empirical),
        None => format!("{}/{}/{}", v.theory, v.kind, v.empirical),
    }
}

/// Frozen rule for one dataset's likelihood model.
fn frozen_rule(likelihood: LikelihoodModel) -> (&'static str, &'static str) {
    match likelihood {
        LikelihoodModel::Gaussian { .. } => ("gaussian", RULE_TWO_SIGMA_NLL),
        LikelihoodModel::IntervalSubset => ("interval-subset", RULE_INTERVAL_SUBSET),
    }
}

/// Rank discriminating observables per theory pair and freeze the
/// comparison rules. Pure: same input, same design id, same order.
/// Only claims with a registered dataset and both theories' verdicts
/// are ranked; everything else is out of scope, not evidence.
pub fn rank_observables(input: &DesignInput) -> ExperimentDesign {
    let mut pairs = Vec::new();
    for (i, a) in input.theories.iter().enumerate() {
        for b in input.theories.iter().skip(i + 1) {
            let mut claims = BTreeSet::new();
            for v in &input.verdicts {
                if &v.theory == a || &v.theory == b {
                    claims.insert(v.claim.clone());
                }
            }
            let mut rows = Vec::new();
            for claim in claims {
                let va = input
                    .verdicts
                    .iter()
                    .find(|v| &v.theory == a && v.claim == claim);
                let vb = input
                    .verdicts
                    .iter()
                    .find(|v| &v.theory == b && v.claim == claim);
                let (Some(va), Some(vb)) = (va, vb) else {
                    continue;
                };
                let Some(dataset) = dataset_for_claim(&claim) else {
                    continue;
                };
                let (likelihood, rule) = frozen_rule(dataset.likelihood);
                rows.push(RankedObservable {
                    claim: claim.clone(),
                    dataset: dataset.id.clone(),
                    observable: dataset.observable.clone(),
                    unit: dataset.unit.clone(),
                    domain: format!("[{}, {}]", dataset.combined().lo, dataset.combined().hi),
                    likelihood: likelihood.into(),
                    rule: rule.into(),
                    contrast: contrast_class(&va.kind, &va.empirical, &vb.kind, &vb.empirical)
                        .into(),
                    nll_gap_milli: nll_gap_milli(va.nll.as_deref(), vb.nll.as_deref()),
                    stances: vec![stance_line(va), stance_line(vb)],
                    prior_uses: input.cite_uses.get(&dataset.id).copied().unwrap_or(0),
                });
            }
            pairs.push(PairDesign {
                a: a.clone(),
                b: b.clone(),
                rows: order_rows(rows),
            });
        }
    }
    let mut fitted = input.fitted.clone();
    fitted.sort();
    let mut body = String::new();
    let mut theories = input.theories.clone();
    theories.sort();
    body.push_str(&theories.join(","));
    body.push('\n');
    for pair in &pairs {
        for row in &pair.rows {
            body.push_str(&format!("{}:{}:{}\n", row.claim, row.dataset, row.rule));
        }
    }
    body.push_str("two-sigma;interval-subset;nll-comparative\n");
    body.push_str(&fitted.join(","));
    ExperimentDesign {
        design_id: ArtifactId::of(body.as_bytes()).to_hex(),
        pairs,
        fitted,
        nuisance: {
            let mut n = input.nuisance.clone();
            n.sort();
            n
        },
        compute_evaluations: input.compute_evaluations,
        cite_uses: input.cite_uses.clone(),
    }
}

/// Render the frozen design: ranked observables with units, domains,
/// rules and costs, then the frozen-rule ledger and disclosures.
pub fn render(design: &ExperimentDesign) -> String {
    let mut out = format!("experiments (frozen design {})\n", design.design_id);
    for pair in &design.pairs {
        out.push_str(&format!("  {} vs {}:\n", pair.a, pair.b));
        if pair.rows.is_empty() {
            out.push_str("    no registered-dataset observable has both verdicts\n");
        }
        for row in &pair.rows {
            out.push_str(&format!(
                "    {} [{}] {} ({})\n",
                row.claim, row.dataset, row.observable, row.unit
            ));
            out.push_str(&format!(
                "      domain {}  rule {}  likelihood {}\n",
                row.domain, row.rule, row.likelihood
            ));
            out.push_str(
                "      acquisition registered-archival (no download; compiled-in listing)\n",
            );
            for stance in &row.stances {
                out.push_str(&format!("      stance {stance}\n"));
            }
            out.push_str(&format!(
                "      contrast {}  nll-gap-milli {}  prior-cite-uses {}\n",
                row.contrast, row.nll_gap_milli, row.prior_uses
            ));
        }
    }
    out.push_str(
        "comparison rules frozen: two-sigma + exact-nll (Gaussian datasets); \
         interval-subset (one-sided limits and hull-only datasets); NLL is comparative only, \
         no exclusion threshold\n",
    );
    if design.fitted.is_empty() {
        out.push_str("fitted inputs: none declared\n");
    } else {
        out.push_str(&format!("fitted inputs: {}\n", design.fitted.join(", ")));
    }
    if design.nuisance.is_empty() {
        out.push_str("nuisance inputs: none declared\n");
    } else {
        out.push_str(&format!(
            "nuisance inputs: {}\n",
            design.nuisance.join(", ")
        ));
    }
    out.push_str("covariance: no covariance records; independence assumed across datasets\n");
    let total_uses: u64 = design.cite_uses.values().sum();
    out.push_str(&format!(
        "adaptive use: {total_uses} journaled cite uses ({}); live evaluations reuse data \
         every run; score does not touch datasets\n",
        if design.cite_uses.is_empty() {
            "none".to_string()
        } else {
            design
                .cite_uses
                .iter()
                .map(|(id, n)| format!("{id}={n}"))
                .collect::<Vec<_>>()
                .join(", ")
        }
    ));
    out.push_str(&format!(
        "compute: {} verdict evaluations charged\n",
        design.compute_evaluations
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn view(theory: &str, claim: &str, kind: &str, empirical: &str) -> VerdictView {
        VerdictView {
            theory: theory.into(),
            claim: claim.into(),
            kind: kind.into(),
            empirical: empirical.into(),
            nll: None,
        }
    }

    fn nll_view(theory: &str, claim: &str, nll: &str) -> VerdictView {
        VerdictView {
            theory: theory.into(),
            claim: claim.into(),
            kind: "holds".into(),
            empirical: "compatible".into(),
            nll: Some(nll.into()),
        }
    }

    fn input(theories: &[&str], verdicts: Vec<VerdictView>) -> DesignInput {
        DesignInput {
            theories: theories.iter().map(|t| t.to_string()).collect(),
            verdicts,
            fitted: vec!["gr/bias".into()],
            nuisance: vec![],
            cite_uses: BTreeMap::new(),
            compute_evaluations: 4,
        }
    }

    #[test]
    fn decisive_kind_split_outranks_status_contrast() {
        // Eddington: kinds differ (holds vs fails). Mercury: same kind,
        // empirical contrast only. Eddington must rank first.
        let design = rank_observables(&input(
            &["general-relativity", "newtonian-gravity"],
            vec![
                view(
                    "general-relativity",
                    "gr.eddington-deflection",
                    "holds",
                    "compatible",
                ),
                view(
                    "newtonian-gravity",
                    "gr.eddington-deflection",
                    "fails",
                    "excluded",
                ),
                view(
                    "general-relativity",
                    "gr.mercury-perihelion",
                    "holds",
                    "compatible",
                ),
                view(
                    "newtonian-gravity",
                    "gr.mercury-perihelion",
                    "holds",
                    "inconclusive",
                ),
            ],
        ));
        assert_eq!(design.pairs.len(), 1);
        let rows = &design.pairs[0].rows;
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].claim, "gr.eddington-deflection");
        assert_eq!(rows[0].contrast, "decisive");
        assert_eq!(rows[1].claim, "gr.mercury-perihelion");
    }

    #[test]
    fn nll_gap_breaks_status_ties() {
        // Same contrast class; the wider NLL gap ranks first.
        // Unparseable NLL counts as zero gap, never an error.
        let design = rank_observables(&input(
            &["general-relativity", "newtonian-gravity"],
            vec![
                nll_view("general-relativity", "gr.eddington-deflection", "1/100"),
                nll_view("newtonian-gravity", "gr.eddington-deflection", "50"),
                nll_view("general-relativity", "gr.mercury-perihelion", "1/100"),
                nll_view("newtonian-gravity", "gr.mercury-perihelion", "not-a-number"),
            ],
        ));
        let rows = &design.pairs[0].rows;
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].claim, "gr.eddington-deflection");
        assert_eq!(rows[1].claim, "gr.mercury-perihelion");
    }

    fn row(claim: &str, contrast: &str) -> RankedObservable {
        RankedObservable {
            claim: claim.into(),
            dataset: "d".into(),
            observable: "o".into(),
            unit: "u".into(),
            domain: "hull".into(),
            likelihood: "gaussian".into(),
            rule: RULE_TWO_SIGMA_NLL.into(),
            contrast: contrast.into(),
            nll_gap_milli: 0,
            stances: Vec::new(),
            prior_uses: 0,
        }
    }

    #[test]
    fn ordering_truncates_to_the_bound() {
        let rows: Vec<RankedObservable> = (0..20)
            .map(|i| row(&format!("claim-{i:02}"), "decisive"))
            .collect();
        let ordered = order_rows(rows);
        assert_eq!(ordered.len(), MAX_DESIGN_OBSERVABLES);
        // Best contrast first, claim id tie-break.
        assert_eq!(ordered[0].claim, "claim-00");
    }

    #[test]
    fn r7_adaptive_reuse_is_disclosed() {
        // Dataset reuse is never silent: journaled cite uses are pinned
        // to each row and rendered in the adaptive-use disclosure.
        let mut uses = BTreeMap::new();
        uses.insert("dyson-1920-sobral-deflection".to_string(), 5u64);
        let design = rank_observables(&DesignInput {
            theories: vec![
                "general-relativity".to_string(),
                "newtonian-gravity".to_string(),
            ],
            verdicts: vec![
                view(
                    "general-relativity",
                    "gr.eddington-deflection",
                    "holds",
                    "compatible",
                ),
                view(
                    "newtonian-gravity",
                    "gr.eddington-deflection",
                    "fails",
                    "excluded",
                ),
            ],
            fitted: vec![],
            nuisance: vec![],
            cite_uses: uses,
            compute_evaluations: 2,
        });
        let rows = &design.pairs[0].rows;
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].prior_uses, 5);
        let text = render(&design);
        assert!(text.contains("dyson-1920-sobral-deflection=5"), "{text}");
    }

    #[test]
    fn ranking_is_deterministic_with_a_stable_id() {
        let verdicts = vec![
            view(
                "general-relativity",
                "gr.eddington-deflection",
                "holds",
                "compatible",
            ),
            view(
                "newtonian-gravity",
                "gr.eddington-deflection",
                "fails",
                "excluded",
            ),
        ];
        let first = rank_observables(&input(
            &["general-relativity", "newtonian-gravity"],
            verdicts.clone(),
        ));
        let second = rank_observables(&input(
            &["general-relativity", "newtonian-gravity"],
            verdicts,
        ));
        assert_eq!(first, second);
        assert!(!first.design_id.is_empty());
    }

    #[test]
    fn rules_freeze_by_likelihood_model() {
        // Mercury (Gaussian) freezes two-sigma + NLL; the Super-K limit
        // (interval-subset) freezes hull comparison.
        let design = rank_observables(&input(
            &["general-relativity", "newtonian-gravity"],
            vec![
                view(
                    "general-relativity",
                    "gr.mercury-perihelion",
                    "holds",
                    "compatible",
                ),
                view(
                    "newtonian-gravity",
                    "gr.mercury-perihelion",
                    "holds",
                    "inconclusive",
                ),
                view(
                    "general-relativity",
                    "gut.proton-lifetime-sk",
                    "holds",
                    "compatible",
                ),
                view(
                    "newtonian-gravity",
                    "gut.proton-lifetime-sk",
                    "holds",
                    "compatible",
                ),
            ],
        ));
        let rows = &design.pairs[0].rows;
        let mercury = rows
            .iter()
            .find(|r| r.claim == "gr.mercury-perihelion")
            .unwrap();
        assert_eq!(mercury.rule, RULE_TWO_SIGMA_NLL);
        assert_eq!(mercury.likelihood, "gaussian");
        let sk = rows
            .iter()
            .find(|r| r.claim == "gut.proton-lifetime-sk")
            .unwrap();
        assert_eq!(sk.rule, RULE_INTERVAL_SUBSET);
        assert_eq!(sk.likelihood, "interval-subset");
    }

    #[test]
    fn unregistered_claims_are_not_ranked() {
        let design = rank_observables(&input(
            &["de-rham", "general-relativity"],
            vec![
                view("de-rham", "dec.d-squared-zero", "holds", "not-applicable"),
                view(
                    "general-relativity",
                    "dec.d-squared-zero",
                    "holds",
                    "not-applicable",
                ),
            ],
        ));
        assert!(design.pairs[0].rows.is_empty());
    }

    #[test]
    fn render_freezes_rules_and_discloses() {
        let design = rank_observables(&input(
            &["general-relativity", "newtonian-gravity"],
            vec![
                view(
                    "general-relativity",
                    "gr.eddington-deflection",
                    "holds",
                    "compatible",
                ),
                view(
                    "newtonian-gravity",
                    "gr.eddington-deflection",
                    "fails",
                    "excluded",
                ),
            ],
        ));
        let text = render(&design);
        assert!(text.contains("frozen design"), "{text}");
        assert!(text.contains(&design.design_id), "{text}");
        assert!(text.contains("gr.eddington-deflection"), "{text}");
        assert!(text.contains("arcsec"), "{text}");
        assert!(text.contains(RULE_TWO_SIGMA_NLL), "{text}");
        assert!(text.contains("no download"), "{text}");
        assert!(text.contains("fitted inputs"), "{text}");
        assert!(text.contains("gr/bias"), "{text}");
        assert!(text.contains("independence assumed"), "{text}");
        assert!(text.contains("adaptive use"), "{text}");
    }
}
