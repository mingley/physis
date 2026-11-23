//! Evaluate an explicit grid of joint knob changes against one fixed statement.

use std::collections::BTreeMap;

use physis_core::artifact::ArtifactId;
use physis_core::claim::{Verdict, VerdictKind};
use physis_core::knob::KnobValue;
use physis_core::ParameterOrigin;
use physis_theory::Theory;
use serde::{Deserialize, Serialize};

/// The grid includes the baseline and all single-coordinate reversions.
const MAX_POINTS: usize = 256;

/// Values to probe in addition to a knob's live value.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConditionAxis {
    /// A chosen or fitted knob of the selected theory.
    pub knob: String,
    /// Raw tokens parsed against the live domain.
    pub values: Vec<String>,
}

/// Outcome for an exact assignment on a fresh fork.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "status", rename_all = "kebab-case")]
pub enum ConditionOutcome {
    /// The same statement was evaluated at the requested knob settings.
    Evaluated {
        /// Includes the evidence and assurance of the ordinary theory evaluator.
        verdict: Verdict,
    },
    /// A setter, coupled knob, or changed statement invalidated the comparison.
    Rejected {
        /// The specific reason this row could not be evaluated.
        reason: String,
    },
}

/// One grid point, expressed relative to the retained baseline.
#[derive(Clone, Debug, Serialize)]
pub struct ConditionRow {
    /// Only the knobs that differ from baseline.
    pub changes: BTreeMap<String, KnobValue>,
    /// The observed verdict or a rejection reason.
    pub outcome: ConditionOutcome,
    /// This point holds and every proper subset of these assignments failed.
    pub minimal_repair: bool,
}

impl ConditionRow {
    fn kind(&self) -> Option<VerdictKind> {
        match &self.outcome {
            ConditionOutcome::Evaluated { verdict } => Some(verdict.kind),
            ConditionOutcome::Rejected { .. } => None,
        }
    }
}

/// Finite evidence for one statement. Minimality is local to these assignments.
#[derive(Clone, Debug, Serialize)]
pub struct ConditionGrid {
    /// Stable theory id.
    pub theory: String,
    /// Claim slug.
    pub claim: String,
    /// Original formal statement. Every evaluated row has this identity.
    pub statement_hash: ArtifactId,
    /// All live knob values before the search.
    pub baseline: BTreeMap<String, KnobValue>,
    /// All grid points, including the unchanged baseline in the first row.
    pub rows: Vec<ConditionRow>,
}

impl ConditionGrid {
    pub(crate) fn render(&self) -> String {
        let mut text = format!(
            "conditions {} {}  points={}\n",
            self.theory,
            self.claim,
            self.rows.len()
        );
        for row in &self.rows {
            let settings = if row.changes.is_empty() {
                "baseline".into()
            } else {
                row.changes
                    .iter()
                    .map(|(k, v)| format!("{k}={}", v.display()))
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            let result = match &row.outcome {
                ConditionOutcome::Evaluated { verdict } => verdict.kind.as_str().to_string(),
                ConditionOutcome::Rejected { reason } => format!("rejected ({reason})"),
            };
            let marker = if row.minimal_repair {
                "  minimal repair"
            } else {
                ""
            };
            text.push_str(&format!("  {settings}  {result}{marker}\n"));
        }
        text
    }
}

/// Evaluate the complete requested grid on forks. An oversized or invalid
/// axis is rejected before any evaluation; a rejected fork remains in the grid.
pub fn search_conditions(
    theory: &dyn Theory,
    claim: &str,
    axes: &[ConditionAxis],
) -> Result<ConditionGrid, String> {
    if axes.is_empty() {
        return Err("provide at least one knob and its values".into());
    }
    let original = theory
        .claims()
        .into_iter()
        .find(|c| c.id_str() == claim)
        .ok_or_else(|| format!("unknown claim '{claim}' on '{}'", theory.id()))?;
    let hash = original.statement_hash();
    let mut baseline = BTreeMap::new();
    for spec in theory.specs() {
        baseline.insert(
            spec.name.to_string(),
            theory.get(spec.name).map_err(|e| e.to_string())?,
        );
    }
    let mut coordinates = Vec::new();
    let mut points = 1;
    for axis in axes {
        let name = &axis.knob;
        let spec = theory.spec(name).map_err(|e| e.to_string())?;
        if !matches!(
            spec.origin,
            ParameterOrigin::Chosen | ParameterOrigin::Fitted
        ) {
            return Err(format!("'{name}' is frozen ({})", spec.origin.as_str()));
        }
        if coordinates.iter().any(|(n, _)| n == name) || axis.values.is_empty() {
            return Err(format!("duplicate or empty axis '{name}'"));
        }
        let mut values = vec![baseline[name].clone()];
        for token in &axis.values {
            let v = KnobValue::parse_in_domain(token, &spec.domain)
                .map_err(|e| format!("{name}: {e}"))?;
            spec.domain.check(name, &v).map_err(|e| e.to_string())?;
            if !values.contains(&v) {
                values.push(v);
                if values.len() > MAX_POINTS / points {
                    return Err(format!(
                        "grid exceeds {MAX_POINTS} points including baseline values"
                    ));
                }
            }
        }
        points *= values.len();
        coordinates.push((name.clone(), values));
    }
    let mut rows = Vec::with_capacity(points);
    rows.push(ConditionRow {
        changes: BTreeMap::new(),
        outcome: ConditionOutcome::Evaluated {
            verdict: theory.evaluate(&original),
        },
        minimal_repair: false,
    });
    let mut all_subsets_fail = Vec::with_capacity(points);
    all_subsets_fail.push(rows[0].kind() == Some(VerdictKind::Fails));
    for index in 1..points {
        let mut digits = index;
        let mut changes = BTreeMap::new();
        let mut stride = 1;
        let mut proper_subsets_fail = true;
        for (name, values) in &coordinates {
            let digit = digits % values.len();
            if digit != 0 {
                changes.insert(name.clone(), values[digit].clone());
                // Every proper subset omits at least one changed coordinate.
                proper_subsets_fail &= all_subsets_fail[index - digit * stride];
            }
            digits /= values.len();
            stride *= values.len();
        }
        let outcome = match evaluate_point(theory, claim, hash, &baseline, axes, &changes) {
            Ok(verdict) => ConditionOutcome::Evaluated { verdict },
            Err(reason) => ConditionOutcome::Rejected { reason },
        };
        let mut row = ConditionRow {
            changes,
            outcome,
            minimal_repair: false,
        };
        let kind = row.kind();
        row.minimal_repair = kind == Some(VerdictKind::Holds) && proper_subsets_fail;
        all_subsets_fail.push(kind == Some(VerdictKind::Fails) && proper_subsets_fail);
        rows.push(row);
    }
    Ok(ConditionGrid {
        theory: theory.id().into(),
        claim: claim.into(),
        statement_hash: hash,
        baseline,
        rows,
    })
}

fn evaluate_point(
    theory: &dyn Theory,
    claim: &str,
    hash: ArtifactId,
    baseline: &BTreeMap<String, KnobValue>,
    axes: &[ConditionAxis],
    changes: &BTreeMap<String, KnobValue>,
) -> Result<Verdict, String> {
    let mut fork = theory.fork();
    for axis in axes {
        if let Some(value) = changes.get(&axis.knob) {
            fork.set(&axis.knob, value.clone())
                .map_err(|e| e.to_string())?;
        }
    }
    if fork.specs().len() != baseline.len() {
        return Err("knob catalog changed".into());
    }
    for (name, value) in baseline {
        if &fork.get(name).map_err(|e| e.to_string())? != changes.get(name).unwrap_or(value) {
            return Err(format!("knob '{name}' drifted on the fork"));
        }
    }
    let live = fork
        .claims()
        .into_iter()
        .find(|c| c.id_str() == claim && c.statement_hash() == hash)
        .ok_or_else(|| "original statement absent on the fork".to_string())?;
    Ok(fork.evaluate(&live))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Command, Lab, Response, Role};

    fn axis(knob: &str, values: &[&str]) -> ConditionAxis {
        ConditionAxis {
            knob: knob.into(),
            values: values.iter().map(|v| (*v).into()).collect(),
        }
    }

    #[test]
    fn joint_repairs_retain_failed_subsets_and_the_live_journal() {
        let mut lab = Lab::standard();
        lab.set_role(Role::Explorer);
        let journal_len = lab.journal().len();
        let original = lab.theory("excursion-counts").unwrap().snapshot();
        let response = lab.exec(Command::Conditions {
            theory: "excursion-counts".into(),
            claim: "excursion.floor-attained".into(),
            axes: vec![
                axis("second_mean_jumps", &["1", "01"]),
                axis("second_winding_quarters", &["-2", "2"]),
                axis("first_percent", &["50"]),
            ],
        });
        let Response::Ok {
            conditions: Some(grid),
            ..
        } = response
        else {
            panic!("conditions grid missing")
        };
        assert_eq!(grid.rows.len(), 12);
        assert_eq!(grid.rows[0].kind(), Some(VerdictKind::Fails));
        assert!(grid.rows[0].changes.is_empty());
        let repairs: Vec<_> = grid.rows.iter().filter(|r| r.minimal_repair).collect();
        assert_eq!(repairs.len(), 2);
        for row in repairs {
            assert_eq!(row.changes.len(), 2);
            assert_eq!(row.changes["second_mean_jumps"], KnobValue::UInt(1));
            assert!(matches!(
                row.changes["second_winding_quarters"],
                KnobValue::Int(-2 | 2)
            ));
        }
        assert_eq!(
            grid.rows
                .iter()
                .filter(|r| r.kind() == Some(VerdictKind::Holds))
                .count(),
            4
        );
        assert_eq!(lab.journal().len(), journal_len);
        assert_eq!(lab.theory("excursion-counts").unwrap().snapshot(), original);
    }

    #[test]
    fn grid_minimality_agrees_with_checking_every_proper_assignment_subset() {
        let lab = Lab::standard();
        let theory = lab.theory("excursion-counts").unwrap();
        let axes = [
            axis("first_percent", &["1", "25", "50", "75", "99"]),
            axis("second_mean_jumps", &["1", "2", "3", "5", "100"]),
            axis("second_winding_quarters", &["-2", "-1", "0", "1", "2"]),
        ];
        for claim in theory.claims() {
            let grid = search_conditions(theory, claim.id_str(), &axes).unwrap();
            assert_eq!(grid.rows.len(), 150);
            for row in &grid.rows {
                let expected = !row.changes.is_empty()
                    && row.kind() == Some(VerdictKind::Holds)
                    && grid
                        .rows
                        .iter()
                        .filter(|other| {
                            other.changes.len() < row.changes.len()
                                && other
                                    .changes
                                    .iter()
                                    .all(|(k, v)| row.changes.get(k) == Some(v))
                        })
                        .all(|other| other.kind() == Some(VerdictKind::Fails));
                assert_eq!(row.minimal_repair, expected);
            }
        }
    }

    #[test]
    fn invalid_frozen_and_oversized_grids_report_the_blocking_input() {
        let lab = Lab::standard();
        let t = lab.theory("excursion-counts").unwrap();
        let key = "excursion.floor-attained";
        for axes in [
            vec![],
            vec![axis("first_percent", &[])],
            vec![axis("first_percent", &["0"])],
            vec![
                axis("first_percent", &["50"]),
                axis("first_percent", &["99"]),
            ],
        ] {
            assert!(search_conditions(t, key, &axes).is_err());
        }
        let too_many = ConditionAxis {
            knob: "second_mean_jumps".into(),
            values: (1..=256).map(|n| n.to_string()).collect(),
        };
        assert!(search_conditions(t, key, &[too_many])
            .unwrap_err()
            .contains("256"));
        let sm = lab.theory("standard-model").unwrap();
        assert!(
            search_conditions(sm, sm.claims()[0].id_str(), &[axis("generations", &["2"])])
                .unwrap_err()
                .contains("frozen")
        );
    }

    #[test]
    fn restoring_resolution_on_a_coarse_lattice_does_not_repair_a_failed_assertion() {
        let mut lab = Lab::standard();
        lab.set_knob("klein-gordon", "spacing", "100").unwrap();
        let grid = search_conditions(
            lab.theory("klein-gordon").unwrap(),
            "field.second-order-accurate",
            &[axis("spacing", &["1"])],
        )
        .unwrap();
        assert_eq!(grid.rows[0].kind(), Some(VerdictKind::Undecidable));
        assert_eq!(grid.rows[1].kind(), Some(VerdictKind::Holds));
        assert!(!grid.rows[1].minimal_repair);
    }
}
