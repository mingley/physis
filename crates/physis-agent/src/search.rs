//! Bounded typed hypothesis search (C7.2).
//!
//! The search explores a bounded family per theory: the structural-mutation
//! closure to [`SEARCH_MAX_DEPTH`], plus chosen/fitted knob probes evaluated
//! on owned forks. The engine takes `&dyn Theory` only, so candidate
//! evaluation cannot modify the trusted baseline: there is no `&mut` path
//! from the search to lab state, and no restore step to forget.
//!
//! Every candidate passes a type gate before evaluation: the knob value must
//! satisfy [`KnobDomain::check`] and [`Knobbed::set`] on a fresh fork, and
//! the fork's frozen (measured / derived / fundamental-input / nuisance)
//! knobs must still match the baseline. Rejected candidates are reported
//! with reasons, never silently dropped. Evaluated candidates keep the
//! baseline verdicts as their reference: a candidate that flips a
//! baseline-`Holds` claim to anything else is retained as refuted, with its
//! counterexample diffs, rather than promoted or deleted.

use std::collections::{HashSet, VecDeque};

use physis_core::claim::VerdictKind;
use physis_core::judgment::ParameterOrigin;
use physis_core::knob::{KnobDomain, KnobValue};
use physis_theory::critique::diff_verdicts;
use physis_theory::{Theory, VerdictDiff};

/// Maximum structural-mutation depth below the baseline.
///
/// Depth 1 is today's single mutation; depth 2 adds pairwise combinations
/// (`add-monopole+add-proca`). Deeper closures explode combinatorially and
/// stop being legible hypotheses.
pub const SEARCH_MAX_DEPTH: usize = 2;

/// Maximum evaluated candidates per theory per search.
///
/// Structural nodes and knob probes share one budget; when it runs out the
/// report says so instead of silently truncating the family.
pub const SEARCH_MAX_CANDIDATES: usize = 32;

/// One evaluated candidate that moved at least one baseline verdict.
pub struct SearchHit {
    /// Knob name, or `+`-joined structural path for IR candidates.
    pub knob: String,
    /// Knob origin (`Chosen` for IR candidates, which are not knobs).
    pub origin: ParameterOrigin,
    /// Baseline value display (`package` for IR candidates).
    pub from: String,
    /// Candidate value display (structural path for IR candidates).
    pub to: String,
    /// Verdict moves against the retained baseline.
    pub diffs: Vec<VerdictDiff>,
    /// True for structural (IR package) candidates.
    pub ir: bool,
    /// True when a baseline-`Holds` claim no longer holds: the candidate is
    /// a retained counterexample, not a promotion.
    pub refuted: bool,
}

/// One candidate the type gate refused before evaluation.
pub struct SearchRejection {
    /// Knob name.
    pub knob: String,
    /// Rejected value display.
    pub value: String,
    /// Why the gate refused it (domain, kind, fork `set`, frozen drift).
    pub reason: String,
}

/// Outcome of searching one theory.
pub struct SearchReport {
    /// Evaluated candidates that moved at least one verdict.
    pub hits: Vec<SearchHit>,
    /// Candidates refused before evaluation, with reasons.
    pub rejected: Vec<SearchRejection>,
    /// Knobs the search would not touch (not chosen/fitted).
    pub frozen_knobs: usize,
    /// Candidates evaluated (structural nodes plus knob probes).
    pub explored: usize,
    /// Evaluated candidates that broke a baseline-`Holds` claim.
    pub refuted: usize,
    /// Deepest structural path evaluated (0 when the theory has no IR
    /// mutations).
    pub depth: usize,
    /// True when [`SEARCH_MAX_CANDIDATES`] stopped the family early.
    pub budget_exhausted: bool,
}

/// Search the bounded hypothesis family of one theory.
///
/// The baseline is `theory` as observed: its snapshot and verdicts are the
/// reference every candidate is diffed against, and the search holds only a
/// shared reference, so the baseline cannot be modified.
pub fn search_theory(theory: &dyn Theory) -> SearchReport {
    let baseline = theory.evaluate_all();
    let mut report = SearchReport {
        hits: Vec::new(),
        rejected: Vec::new(),
        frozen_knobs: 0,
        explored: 0,
        refuted: 0,
        depth: 0,
        budget_exhausted: false,
    };
    search_structural(theory, &baseline, &mut report);
    if !report.budget_exhausted {
        search_knobs(theory, &baseline, &mut report);
    }
    report
}

/// Candidate identity for [`record_eval`]: everything about a hit except the
/// verdicts, which arrive separately from the fork evaluation.
struct HitMeta {
    knob: String,
    origin: ParameterOrigin,
    from: String,
    to: String,
    ir: bool,
}

/// Record one evaluated candidate against the retained baseline.
fn record_eval(
    baseline: &[(physis_core::claim::Claim, physis_core::claim::Verdict)],
    after: Vec<(physis_core::claim::Claim, physis_core::claim::Verdict)>,
    meta: HitMeta,
    report: &mut SearchReport,
) {
    let diffs = diff_verdicts(baseline, &after);
    let refuted = diffs
        .iter()
        .any(|d| d.from == VerdictKind::Holds && d.to != VerdictKind::Holds);
    if refuted {
        report.refuted += 1;
    }
    if !diffs.is_empty() {
        report.hits.push(SearchHit {
            knob: meta.knob,
            origin: meta.origin,
            from: meta.from,
            to: meta.to,
            diffs,
            ir: meta.ir,
            refuted,
        });
    }
    report.explored += 1;
}

/// Breadth-first walk of the structural-mutation closure to
/// [`SEARCH_MAX_DEPTH`], deduplicated by sorted label set so the same flag
/// combination reached via two orders counts once.
fn search_structural(
    theory: &dyn Theory,
    baseline: &[(physis_core::claim::Claim, physis_core::claim::Verdict)],
    report: &mut SearchReport,
) {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(Vec<String>, Box<dyn Theory>)> = theory
        .structural_mutations()
        .into_iter()
        .map(|(label, mutant)| (vec![label], mutant))
        .collect();
    while let Some((path, node)) = queue.pop_front() {
        if report.explored >= SEARCH_MAX_CANDIDATES {
            report.budget_exhausted = true;
            return;
        }
        let mut key = path.clone();
        key.sort();
        if !visited.insert(key.join("+")) {
            continue;
        }
        let after = node.evaluate_all();
        let label = path.join("+");
        record_eval(
            baseline,
            after,
            HitMeta {
                knob: label.clone(),
                origin: ParameterOrigin::Chosen,
                from: "package".into(),
                to: label,
                ir: true,
            },
            report,
        );
        report.depth = report.depth.max(path.len());
        if path.len() < SEARCH_MAX_DEPTH {
            for (label, child) in node.structural_mutations() {
                let mut child_path = path.clone();
                child_path.push(label);
                queue.push_back((child_path, child));
            }
        }
    }
}

/// Knob probes on fresh forks of the baseline: valid domain probes are
/// evaluated, boundary/type probes go through the same gate and are usually
/// rejected with reasons. Only chosen/fitted knobs are probed; everything
/// else counts as frozen and is re-verified on every fork.
fn search_knobs(
    theory: &dyn Theory,
    baseline: &[(physis_core::claim::Claim, physis_core::claim::Verdict)],
    report: &mut SearchReport,
) {
    let knobs: Vec<(String, ParameterOrigin, KnobDomain, KnobValue)> = theory
        .snapshot()
        .into_iter()
        .map(|(s, v)| (s.name.to_string(), s.origin, s.domain.clone(), v))
        .collect();
    for (name, origin, domain, current) in knobs {
        if !matches!(origin, ParameterOrigin::Chosen | ParameterOrigin::Fitted) {
            report.frozen_knobs += 1;
            continue;
        }
        let mut tried: HashSet<String> = HashSet::new();
        tried.insert(current.display());
        let mut probes = domain_probes(&domain, &current);
        probes.extend(invalid_probes(&domain, &current));
        for cand in probes {
            if report.explored >= SEARCH_MAX_CANDIDATES {
                report.budget_exhausted = true;
                return;
            }
            if !tried.insert(cand.display()) {
                continue;
            }
            if let Err(e) = domain.check(&name, &cand) {
                report.rejected.push(SearchRejection {
                    knob: name.clone(),
                    value: cand.display(),
                    reason: e.to_string(),
                });
                continue;
            }
            let mut fork = theory.fork();
            if let Err(e) = fork.set(&name, cand.clone()) {
                report.rejected.push(SearchRejection {
                    knob: name.clone(),
                    value: cand.display(),
                    reason: e.to_string(),
                });
                continue;
            }
            if frozen_drifted(theory, fork.as_ref(), &name) {
                report.rejected.push(SearchRejection {
                    knob: name.clone(),
                    value: cand.display(),
                    reason: "frozen knob drifted on fork; candidate discarded".into(),
                });
                continue;
            }
            let after = fork.evaluate_all();
            record_eval(
                baseline,
                after,
                HitMeta {
                    knob: name.clone(),
                    origin,
                    from: current.display(),
                    to: cand.display(),
                    ir: false,
                },
                report,
            );
        }
    }
}

/// Frozen-knob guard: every knob except the probed one must still match the
/// baseline on the candidate fork. Forks inherit state and the search only
/// sets the probed knob, so drift means a coupled `set` implementation, and
/// the candidate is discarded rather than evaluated against a moved
/// baseline.
fn frozen_drifted(baseline: &dyn Theory, fork: &dyn Theory, probed: &str) -> bool {
    let before = baseline.snapshot();
    let after = fork.snapshot();
    if before.len() != after.len() {
        return true;
    }
    before
        .iter()
        .zip(after.iter())
        .any(|((spec, a), (fspec, b))| spec.name != fspec.name || (spec.name != probed && a != b))
}

/// Valid probes for a domain: boundaries plus the current value (the caller
/// skips the current value).
pub fn domain_probes(domain: &KnobDomain, current: &KnobValue) -> Vec<KnobValue> {
    match domain {
        KnobDomain::Bool => vec![KnobValue::Bool(true), KnobValue::Bool(false)],
        KnobDomain::Int { min, max } => {
            vec![KnobValue::Int(*min), KnobValue::Int(*max), current.clone()]
        }
        KnobDomain::UInt { min, max } => {
            let mut v = vec![KnobValue::UInt(*min), KnobValue::UInt(*max)];
            if let KnobValue::UInt(x) = current {
                if *x > *min {
                    v.push(KnobValue::UInt(*x - 1));
                }
                if *x < *max {
                    v.push(KnobValue::UInt(*x + 1));
                }
            }
            v
        }
        KnobDomain::Float { min, max } => vec![
            KnobValue::Float(*min),
            KnobValue::Float(*max),
            current.clone(),
        ],
        KnobDomain::Choice(opts) => opts
            .iter()
            .map(|s| KnobValue::Choice((*s).into()))
            .collect(),
    }
}

/// Boundary/type probes that the gate is expected to reject: just outside
/// each range, plus wrong-kind values. A probe that passes the gate anyway
/// (saturating range edge, float precision collapse) is evaluated as the
/// valid candidate it turned out to be.
pub fn invalid_probes(domain: &KnobDomain, current: &KnobValue) -> Vec<KnobValue> {
    let mut v = match domain {
        KnobDomain::Bool => vec![
            KnobValue::Int(0),
            KnobValue::Choice("no-such-choice".into()),
        ],
        KnobDomain::Int { min, max } => vec![
            KnobValue::Int(min.saturating_sub(1)),
            KnobValue::Int(max.saturating_add(1)),
            KnobValue::Bool(true),
        ],
        KnobDomain::UInt { min, max } => vec![
            KnobValue::UInt(min.saturating_sub(1)),
            KnobValue::UInt(max.saturating_add(1)),
            KnobValue::Bool(false),
        ],
        KnobDomain::Float { min, max } => vec![
            KnobValue::Float(min - 1.0),
            KnobValue::Float(max + 1.0),
            KnobValue::Bool(true),
        ],
        KnobDomain::Choice(_) => vec![
            KnobValue::Choice("no-such-choice".into()),
            KnobValue::Int(0),
        ],
    };
    v.retain(|c| c != current);
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::{Claim, Verdict};
    use physis_core::error::CoreError;
    use physis_core::knob::{KnobSpec, Knobbed};

    /// Stub theory offering `fanout` mutations at every node, forever. The
    /// engine's depth and budget caps are the only thing standing between
    /// this stub and an infinite family.
    #[derive(Clone)]
    struct Fanout {
        fanout: usize,
    }

    impl Knobbed for Fanout {
        fn specs(&self) -> &'static [KnobSpec] {
            &[]
        }
        fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
            Err(CoreError::UnknownKnob { name: name.into() })
        }
        fn set(&mut self, name: &str, _value: KnobValue) -> Result<KnobValue, CoreError> {
            Err(CoreError::UnknownKnob { name: name.into() })
        }
    }

    impl Theory for Fanout {
        fn fork(&self) -> Box<dyn Theory> {
            Box::new(self.clone())
        }
        fn id(&self) -> &'static str {
            "stub-fanout"
        }
        fn name(&self) -> &'static str {
            "stub fanout"
        }
        fn summary(&self) -> &'static str {
            "test stub"
        }
        fn world(&self) -> Option<physis_model::World> {
            None
        }
        fn claims(&self) -> Vec<Claim> {
            Vec::new()
        }
        fn evaluate(&self, _claim: &Claim) -> Verdict {
            unreachable!("stub has no claims")
        }
        fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
            (0..self.fanout)
                .map(|i| {
                    (
                        format!("m{i}"),
                        Box::new(Self {
                            fanout: self.fanout,
                        }) as Box<dyn Theory>,
                    )
                })
                .collect()
        }
    }

    #[test]
    fn search_stops_at_max_depth() {
        let stub = Fanout { fanout: 3 };
        let report = search_theory(&stub);
        // 3 singles + 6 unordered pairs; depth 3 never explored.
        assert_eq!(report.explored, 9);
        assert_eq!(report.depth, SEARCH_MAX_DEPTH);
        assert!(!report.budget_exhausted);
        assert!(report.hits.is_empty());
    }

    #[test]
    fn search_stops_at_candidate_budget() {
        let stub = Fanout { fanout: 8 };
        let report = search_theory(&stub);
        // 8 singles + 36 pairs would exceed the budget; the report says so.
        assert_eq!(report.explored, SEARCH_MAX_CANDIDATES);
        assert_eq!(report.depth, SEARCH_MAX_DEPTH);
        assert!(report.budget_exhausted);
    }
}
