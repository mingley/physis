//! Computation: the workspace's third scientific domain.
//!
//! Like electromagnetism, this reuses the whole substrate (knobs, claims,
//! verdicts, the `Theory` trait, the experiment matrix) for a domain that is
//! not physics. Its payoff is the **halting problem**: an unbounded-tape Turing
//! machine's `comp.halts` claim is genuinely `Undecidable` — a verdict kind the
//! lab already had. Bounding the tape turns the machine into a finite automaton
//! and halting becomes decidable, a clean knob → verdict diff.
//!
//! `World` is a physics-shaped projection, so computational objects use a
//! degenerate placeholder world (documented in `specs/009-computation.md`);
//! only their claims carry meaning here.

use physis_core::claim::{Claim, Epistemic, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_model::{GaugeGroup, Manifold, Spectrum, World};

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;

/// The machine halts on every input.
pub const HALTS: &str = "comp.halts";
/// The model is Turing complete.
pub const TURING_COMPLETE: &str = "comp.turing-complete";
/// Evolution is deterministic.
pub const DETERMINISTIC: &str = "comp.deterministic";
/// Equivalence of two instances is decidable.
pub const DECIDABLE_EQUIVALENCE: &str = "comp.decidable-equivalence";
/// The computation runs within an a priori resource bound.
pub const RESOURCE_BOUNDED: &str = "comp.resource-bounded";

/// Matrix rows for the computation lab.
pub fn computation_rows() -> [&'static str; 5] {
    [
        HALTS,
        TURING_COMPLETE,
        DETERMINISTIC,
        DECIDABLE_EQUIVALENCE,
        RESOURCE_BOUNDED,
    ]
}

fn comp_claims() -> Vec<Claim> {
    vec![
        Claim::new(
            HALTS,
            "The machine halts on every input.",
            LayerId::Information,
            Epistemic::Theorem,
        ),
        Claim::new(
            TURING_COMPLETE,
            "The model is Turing complete.",
            LayerId::Information,
            Epistemic::EncodedFact,
        ),
        Claim::new(
            DETERMINISTIC,
            "The transition function is single-valued.",
            LayerId::Information,
            Epistemic::Theorem,
        ),
        Claim::new(
            DECIDABLE_EQUIVALENCE,
            "Equivalence of two instances is decidable.",
            LayerId::Mathematical,
            Epistemic::EncodedFact,
        ),
        Claim::new(
            RESOURCE_BOUNDED,
            "The computation runs within an a priori resource bound.",
            LayerId::Information,
            Epistemic::Theorem,
        ),
    ]
}

/// Degenerate physics-shaped world; computation lives on the information and
/// mathematical layers, so only the note is meaningful here.
fn comp_world(note: String) -> World {
    World {
        spacetime: Manifold::observed_4d(),
        gauge: GaugeGroup::trivial(),
        spectrum: Spectrum::empty(),
        has_gravity: false,
        supersymmetric: false,
        free_parameter_count: 0,
        landscape_log10: 0.0,
        note,
    }
}

/// A finite, acyclic boolean circuit (combinational logic).
#[derive(Clone, Debug, Default)]
pub struct CombinationalCircuit;

impl Knobbed for CombinationalCircuit {
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

impl Theory for CombinationalCircuit {
    fn id(&self) -> &'static str {
        "combinational-circuit"
    }
    fn name(&self) -> &'static str {
        "Combinational circuit"
    }
    fn summary(&self) -> &'static str {
        "A finite, acyclic boolean circuit. It always halts and its equivalence \
         is decidable, but with no memory or feedback it is not Turing complete."
    }
    fn world(&self) -> World {
        comp_world("combinational boolean circuit (acyclic, finite)".to_string())
    }
    fn claims(&self) -> Vec<Claim> {
        comp_claims()
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id.0.as_str() {
            HALTS => Verdict::holds(
                Epistemic::Theorem,
                "an acyclic combinational circuit always terminates",
            ),
            TURING_COMPLETE => Verdict::fails(
                Epistemic::Theorem,
                "no memory or feedback: combinational logic is not Turing complete",
            ),
            DETERMINISTIC => {
                Verdict::holds(Epistemic::Theorem, "boolean functions are deterministic")
            }
            DECIDABLE_EQUIVALENCE => Verdict::holds(
                Epistemic::EncodedFact,
                "circuit equivalence is decidable (coNP-complete, but decidable)",
            ),
            RESOURCE_BOUNDED => {
                Verdict::holds(Epistemic::Theorem, "bounded by gate count and depth")
            }
            _ => Verdict::inapplicable("claim not made by a computational object"),
        }
    }
}

const TM_SPECS: &[KnobSpec] = &[KnobSpec {
    name: "tape_bound",
    layer: LayerId::Information,
    doc: "Tape length bound in cells; 0 means an unbounded tape. A finite bound makes the machine a finite automaton.",
    domain: KnobDomain::UInt { min: 0, max: 1_000_000 },
}];

/// A deterministic Turing machine with an optional tape bound.
#[derive(Clone, Debug, Default)]
pub struct TuringMachine {
    /// Tape bound in cells; 0 means unbounded.
    tape_bound: u64,
}

impl TuringMachine {
    fn unbounded(&self) -> bool {
        self.tape_bound == 0
    }
}

impl Knobbed for TuringMachine {
    fn specs(&self) -> &'static [KnobSpec] {
        TM_SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "tape_bound" => Ok(KnobValue::UInt(self.tape_bound)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("tape_bound", KnobValue::UInt(v)) => self.tape_bound = v,
            _ => {
                return Err(CoreError::TypeMismatch {
                    name: name.into(),
                    expected: spec.domain.kind_name().into(),
                    got: old.kind_name().into(),
                });
            }
        }
        Ok(old)
    }
}

impl Theory for TuringMachine {
    fn id(&self) -> &'static str {
        "turing-machine"
    }
    fn name(&self) -> &'static str {
        "Turing machine"
    }
    fn summary(&self) -> &'static str {
        "A deterministic Turing machine. With an unbounded tape it is Turing \
         complete and its halting is undecidable; bounding the tape makes it a \
         finite automaton whose halting and equivalence are decidable."
    }
    fn world(&self) -> World {
        comp_world(format!(
            "Turing machine, tape_bound={} ({})",
            self.tape_bound,
            if self.unbounded() {
                "unbounded"
            } else {
                "finite"
            }
        ))
    }
    fn claims(&self) -> Vec<Claim> {
        comp_claims()
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id.0.as_str() {
            HALTS => {
                if self.unbounded() {
                    Verdict::undecidable(
                        Epistemic::Open,
                        "the halting problem: no algorithm decides halting for an unbounded-tape machine",
                    )
                    .with_evidence(["Turing 1936; this is the point of the Undecidable verdict".to_string()])
                } else {
                    Verdict::holds(
                        Epistemic::Theorem,
                        format!(
                            "a {}-cell tape has finitely many configurations; halting is decidable by cycle detection",
                            self.tape_bound
                        ),
                    )
                }
            }
            TURING_COMPLETE => {
                if self.unbounded() {
                    Verdict::holds(
                        Epistemic::EncodedFact,
                        "an unbounded-tape Turing machine is Turing complete",
                    )
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        "a bounded-tape machine is a finite automaton, not Turing complete",
                    )
                }
            }
            DETERMINISTIC => {
                Verdict::holds(Epistemic::Theorem, "single-valued transition function")
            }
            DECIDABLE_EQUIVALENCE => {
                if self.unbounded() {
                    Verdict::undecidable(
                        Epistemic::Open,
                        "equivalence of Turing machines is undecidable (Rice's theorem)",
                    )
                } else {
                    Verdict::holds(
                        Epistemic::EncodedFact,
                        "equivalence of finite automata is decidable",
                    )
                }
            }
            RESOURCE_BOUNDED => {
                if self.unbounded() {
                    Verdict::fails(
                        Epistemic::Theorem,
                        "no a priori resource bound on an unbounded-tape machine",
                    )
                } else {
                    Verdict::holds(
                        Epistemic::Theorem,
                        format!("bounded by {} tape cells", self.tape_bound),
                    )
                }
            }
            _ => Verdict::inapplicable("claim not made by a computational object"),
        }
    }
}

/// The computation experiment: a combinational circuit vs a Turing machine.
pub fn computation() -> ExperimentReport {
    let theories: Vec<Box<dyn Theory>> = vec![
        Box::new(CombinationalCircuit),
        Box::new(TuringMachine::default()),
    ];
    report_from_rows(
        "computation",
        "Computation lab",
        "Does the same typed substrate host computation — and does the halting \
         problem come out as an honest `undecidable`, with a knob (the tape \
         bound) that mechanically restores decidability?",
        "These are structural facts about models of computation (Turing 1936, \
         Rice's theorem), encoded as claims. The halting/undecidability verdicts \
         are theorems of computability, not opinions.",
        vec![
            "`holds` / `fails` are internal to the encoding.".into(),
            "The unbounded Turing machine's `comp.halts` is genuinely `undecidable` — that is the point.".into(),
            "Bounding the tape turns the machine into a finite automaton: halting and equivalence become decidable, but it is no longer Turing complete.".into(),
        ],
        &computation_rows(),
        theories,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;

    fn verdict(t: &dyn Theory, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id.0 == id).unwrap();
        t.evaluate(&c).kind
    }

    #[test]
    fn halting_problem_is_undecidable() {
        let tm = TuringMachine::default();
        assert!(tm.unbounded());
        assert_eq!(verdict(&tm, HALTS), VerdictKind::Undecidable);
        assert_eq!(verdict(&tm, TURING_COMPLETE), VerdictKind::Holds);
    }

    #[test]
    fn bounding_the_tape_restores_decidability() {
        // The computation knob → verdict diff: a finite tape decides halting.
        let mut tm = TuringMachine::default();
        assert_eq!(verdict(&tm, HALTS), VerdictKind::Undecidable);
        tm.set("tape_bound", KnobValue::UInt(1000)).unwrap();
        assert_eq!(verdict(&tm, HALTS), VerdictKind::Holds);
        // ...but a bounded machine is no longer Turing complete.
        assert_eq!(verdict(&tm, TURING_COMPLETE), VerdictKind::Fails);
        assert_eq!(verdict(&tm, DECIDABLE_EQUIVALENCE), VerdictKind::Holds);
    }

    #[test]
    fn combinational_halts_but_is_not_turing_complete() {
        let c = CombinationalCircuit;
        assert_eq!(verdict(&c, HALTS), VerdictKind::Holds);
        assert_eq!(verdict(&c, TURING_COMPLETE), VerdictKind::Fails);
        assert_eq!(verdict(&c, DECIDABLE_EQUIVALENCE), VerdictKind::Holds);
    }

    #[test]
    fn computation_experiment_builds_a_matrix() {
        let r = computation();
        assert_eq!(r.id, "computation");
        assert_eq!(r.theories.len(), 2);
        let halts = r.matrix.get(HALTS).expect("row");
        assert_eq!(
            halts.get("turing-machine").copied(),
            Some(VerdictKind::Undecidable)
        );
        assert_eq!(
            halts.get("combinational-circuit").copied(),
            Some(VerdictKind::Holds)
        );
    }
}
