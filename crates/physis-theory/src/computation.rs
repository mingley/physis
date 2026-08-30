//! Computation: the workspace's third scientific domain.
//!
//! Like electromagnetism, this reuses the whole substrate (knobs, claims,
//! verdicts, the `Theory` trait, the experiment matrix) for a domain that is
//! not physics. Its payoff is the **halting problem**: an unbounded-tape Turing
//! machine's `comp.halts` claim is genuinely `Undecidable` — a verdict kind the
//! lab already had. Bounding the tape turns the machine into a finite automaton
//! and halting becomes decidable, a clean knob → verdict diff. Decidable is
//! not feasible: `comp.feasible-decision` is a resource gap (coNP-complete
//! circuit equivalence; exponential configuration graphs), not the
//! halting problem.
//!
//! Computation has no spacetime, gauge, or spectrum, so these theories return
//! `None` from `Theory::world()` and describe themselves via `Theory::note()`
//! instead of borrowing a physics-shaped placeholder.
//!
//! The Landauer bound lives on the IR package. Dropping `ln2` (`E = N kT`
//! instead of `N kT ln2`) is a package mutation (`add-kt`), not a
//! `reversible` knob: the encoding energy is no longer the Landauer floor
//! and `info.landauer-cost` fails. `temperature_k` / `bits_erased` /
//! `reversible` still scale the bath and Bennett reversibility. That fork
//! is still this object, not a silent Turing-machine install.

use physis_core::assumption::DomainOfValidity;
use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::qty::kelvin;
use physis_core::ParameterOrigin;
use physis_core::{Energy, Qty};
use physis_ir::{apply_mutation, parse_package, render_package, PackageMutation, TheoryPackage};
use physis_model::constants::k_boltzmann;
use physis_model::World;

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
/// A resource-feasible procedure in this lab decides the instance.
pub const FEASIBLE_DECISION: &str = "comp.feasible-decision";
/// Whether P = NP in this model.
pub const P_EQUALS_NP: &str = "comp.p-equals-np";
/// The gate graph of a NAND netlist is acyclic.
pub const ACYCLIC: &str = "comp.acyclic";

/// Matrix rows for the computation lab.
pub fn computation_rows() -> [&'static str; 7] {
    [
        HALTS,
        TURING_COMPLETE,
        DETERMINISTIC,
        DECIDABLE_EQUIVALENCE,
        RESOURCE_BOUNDED,
        FEASIBLE_DECISION,
        P_EQUALS_NP,
    ]
}

fn comp_claims() -> Vec<Claim> {
    vec![
        Claim::new(
            HALTS,
            "The machine halts on every input.",
            LayerId::Information,
            ClaimClass::ModelInternal,
        ),
        Claim::new(
            TURING_COMPLETE,
            "The model is Turing complete.",
            LayerId::Information,
            ClaimClass::Phenomenological,
        ),
        Claim::new(
            DETERMINISTIC,
            "The transition function is single-valued.",
            LayerId::Information,
            ClaimClass::ModelInternal,
        ),
        Claim::new(
            DECIDABLE_EQUIVALENCE,
            "Equivalence of two instances is decidable.",
            LayerId::Information,
            ClaimClass::Phenomenological,
        ),
        Claim::new(
            RESOURCE_BOUNDED,
            "The computation runs within an a priori resource bound.",
            LayerId::Information,
            ClaimClass::ModelInternal,
        ),
        Claim::new(
            FEASIBLE_DECISION,
            "A resource-feasible procedure in this lab decides the instance.",
            LayerId::Information,
            ClaimClass::Phenomenological,
        ),
        Claim::new(
            P_EQUALS_NP,
            "Polynomial time equals nondeterministic polynomial time (P = NP).",
            LayerId::Mathematical,
            ClaimClass::OpenProblem,
        ),
    ]
}

/// A finite NAND netlist. Structure lives on the IR package (gate equations),
/// not on knobs. A feedback wire is a package mutation, not `set`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CombinationalCircuit {
    gates: Vec<NandGate>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct NandGate {
    a: usize,
    b: usize,
    out: usize,
}

impl Default for CombinationalCircuit {
    fn default() -> Self {
        Self {
            gates: vec![NandGate { a: 0, b: 1, out: 2 }],
        }
    }
}

impl CombinationalCircuit {
    fn is_acyclic(&self) -> bool {
        use std::collections::BTreeMap;
        let mut adj: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for g in &self.gates {
            adj.entry(g.out).or_default().push(g.a);
            adj.entry(g.out).or_default().push(g.b);
        }
        #[derive(Clone, Copy, PartialEq, Eq)]
        enum Color {
            White,
            Gray,
            Black,
        }
        let mut color: BTreeMap<usize, Color> = BTreeMap::new();
        fn dfs(
            n: usize,
            adj: &BTreeMap<usize, Vec<usize>>,
            color: &mut BTreeMap<usize, Color>,
        ) -> bool {
            color.insert(n, Color::Gray);
            if let Some(next) = adj.get(&n) {
                for &m in next {
                    match color.get(&m).copied().unwrap_or(Color::White) {
                        Color::Gray => return false,
                        Color::White => {
                            if !dfs(m, adj, color) {
                                return false;
                            }
                        }
                        Color::Black => {}
                    }
                }
            }
            color.insert(n, Color::Black);
            true
        }
        for &n in adj.keys() {
            if color.get(&n).copied().unwrap_or(Color::White) == Color::White
                && !dfs(n, &adj, &mut color)
            {
                return false;
            }
        }
        true
    }

    /// IR package for this netlist. Equations are `nand a b -> out`.
    pub fn package(&self) -> TheoryPackage {
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["finite-nand-netlist".into()],
            equations: self
                .gates
                .iter()
                .map(|g| format!("nand {} {} -> {}", g.a, g.b, g.out))
                .collect(),
            claims: vec![physis_ir::ClaimDecl {
                id: ACYCLIC.into(),
                statement: "The gate graph is acyclic.".into(),
                layer: "information".into(),
                class: "model-internal".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load a NAND netlist from a package. Non-nand equations are ignored.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        let mut gates = Vec::new();
        for eq in &pkg.equations {
            if let Some(g) = parse_nand(eq) {
                gates.push(g);
            }
        }
        if gates.is_empty() {
            return Err("combinational-circuit package has no nand equations".into());
        }
        Ok(Self { gates })
    }

    fn feedback_equation(&self) -> String {
        let sink = self.gates.last().map(|g| g.out).unwrap_or(0);
        let src = self.gates.first().map(|g| g.a).unwrap_or(0);
        format!("nand {sink} {sink} -> {src}")
    }
}

fn parse_nand(eq: &str) -> Option<NandGate> {
    let rest = eq.trim().strip_prefix("nand ")?;
    let (ins, out) = rest.split_once("->")?;
    let mut ws = ins.split_whitespace();
    let a = ws.next()?.parse().ok()?;
    let b = ws.next()?.parse().ok()?;
    if ws.next().is_some() {
        return None;
    }
    let out = out.trim().parse().ok()?;
    Some(NandGate { a, b, out })
}

fn nand_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["finite NAND netlist".into()],
        vec![
            "cycle detection on the gate graph".into(),
            "no SAT solver".into(),
            "no tape or circuit simulator".into(),
        ],
        "Acyclicity is a graph property of this netlist. A cyclic encoding \
         is a new claim, not a silent combinational circuit.",
    )
}

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
        "A finite NAND netlist. Acyclicity is a graph property of the IR \
         package. Feedback is a package mutation, not a knob. No SAT solver \
         and no tape simulator are run."
    }
    fn world(&self) -> Option<World> {
        None // computation has no spacetime/gauge/spectrum projection
    }
    fn note(&self) -> String {
        format!(
            "NAND netlist, {} gate(s), {}",
            self.gates.len(),
            if self.is_acyclic() {
                "acyclic"
            } else {
                "cyclic"
            }
        )
    }
    fn claims(&self) -> Vec<Claim> {
        let mut c = comp_claims();
        c.push(
            Claim::new(
                ACYCLIC,
                "The gate graph of this NAND netlist is acyclic.",
                LayerId::Information,
                ClaimClass::ModelInternal,
            )
            .with_domain(nand_domain()),
        );
        c
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id_str() {
            HALTS => {
                if self.is_acyclic() {
                    Verdict::holds(claim, "an acyclic NAND netlist always terminates")
                } else {
                    Verdict::inapplicable(
                        claim,
                        "halts is for acyclic combinational netlists; this encoding has a cycle",
                    )
                }
            }
            TURING_COMPLETE => Verdict::fails(
                claim,
                "no memory or unbounded tape: a NAND netlist is not Turing complete",
            ),
            DETERMINISTIC => Verdict::holds(claim, "boolean functions are deterministic"),
            DECIDABLE_EQUIVALENCE => Verdict::holds(
                claim,
                "circuit equivalence is decidable (coNP-complete, but decidable)",
            ),
            RESOURCE_BOUNDED => Verdict::holds(claim, "bounded by gate count and depth"),
            FEASIBLE_DECISION => Verdict::undecidable(
                claim,
                "circuit equivalence is coNP-complete; this lab does not brute-force SAT",
            )
            .with_intractable()
            .with_evidence([
                "decidable (see comp.decidable-equivalence) is not feasible; no circuit simulator is run"
                    .to_string(),
            ]),
            P_EQUALS_NP => Verdict::inapplicable(
                claim,
                "P vs NP concerns uniform machine models, not a single fixed circuit",
            ),
            ACYCLIC => {
                if self.is_acyclic() {
                    Verdict::holds(claim, "the NAND gate graph has no cycle")
                } else {
                    Verdict::fails(claim, "a feedback equation closed a cycle on the gate graph")
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by a computational object"),
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        Ok(Box::new(Self::from_package(pkg)?))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(self.feedback_equation()),
        );
        match Self::from_package(&mutated) {
            Ok(fork) if fork != *self => vec![("add-feedback".into(), Box::new(fork))],
            _ => Vec::new(),
        }
    }
}

const TM_SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "tape_bound",
        layer: LayerId::Information,
        doc: "Tape length bound in cells; 0 means an unbounded tape. A finite bound makes the machine a finite automaton.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 0, max: 1_000_000 },
    },
    KnobSpec {
        name: "nondeterministic",
        layer: LayerId::Information,
        doc: "Whether the transition relation allows nondeterministic branching.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Bool,
    },
];

/// A Turing machine with an optional tape bound and (non)determinism.
#[derive(Clone, Debug, Default)]
pub struct TuringMachine {
    /// Tape bound in cells; 0 means unbounded.
    tape_bound: u64,
    /// Whether the transition relation is nondeterministic.
    nondeterministic: bool,
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
            "nondeterministic" => Ok(KnobValue::Bool(self.nondeterministic)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("tape_bound", KnobValue::UInt(v)) => self.tape_bound = v,
            ("nondeterministic", KnobValue::Bool(v)) => self.nondeterministic = v,
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
    fn world(&self) -> Option<World> {
        None // computation has no spacetime/gauge/spectrum projection
    }
    fn note(&self) -> String {
        format!(
            "Turing machine, tape_bound={} ({})",
            self.tape_bound,
            if self.unbounded() {
                "unbounded"
            } else {
                "finite"
            }
        )
    }
    fn claims(&self) -> Vec<Claim> {
        comp_claims()
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id_str() {
            HALTS => {
                if self.unbounded() {
                    Verdict::undecidable(claim,
                        "the halting problem: no algorithm decides halting for an unbounded-tape machine",
                    )
                    .with_evidence(["Turing 1936; this is the point of the Undecidable verdict".to_string()])
                } else {
                    Verdict::holds(claim,
                        format!(
                            "a {}-cell tape has finitely many configurations; halting is decidable by cycle detection",
                            self.tape_bound
                        ),
                    )
                }
            }
            TURING_COMPLETE => {
                if self.unbounded() {
                    Verdict::holds(claim, "an unbounded-tape Turing machine is Turing complete")
                } else {
                    Verdict::fails(
                        claim,
                        "a bounded-tape machine is a finite automaton, not Turing complete",
                    )
                }
            }
            DETERMINISTIC => {
                if self.nondeterministic {
                    Verdict::fails(
                        claim,
                        "nondeterministic transition relation: multiple next configurations",
                    )
                } else {
                    Verdict::holds(claim, "single-valued transition function")
                }
            }
            P_EQUALS_NP => Verdict::undecidable(
                claim,
                "P vs NP is an open problem; this encoding does not decide it",
            )
            .with_evidence([
                "one of the Clay Millenium Problems; honestly Open, not Holds or Fails".to_string(),
            ]),
            DECIDABLE_EQUIVALENCE => {
                if self.unbounded() {
                    Verdict::undecidable(
                        claim,
                        "equivalence of Turing machines is undecidable (Rice's theorem)",
                    )
                } else {
                    Verdict::holds(claim, "equivalence of finite automata is decidable")
                }
            }
            RESOURCE_BOUNDED => {
                if self.unbounded() {
                    Verdict::fails(
                        claim,
                        "no a priori resource bound on an unbounded-tape machine",
                    )
                } else {
                    Verdict::holds(claim, format!("bounded by {} tape cells", self.tape_bound))
                }
            }
            FEASIBLE_DECISION => {
                if self.unbounded() {
                    Verdict::inapplicable(
                        claim,
                        "no finite configuration graph; the obstruction is computability (comp.halts), not cost",
                    )
                } else {
                    Verdict::undecidable(
                        claim,
                        format!(
                            "a {}-cell tape is finite so the problem is decidable, but this lab does not enumerate the configuration graph",
                            self.tape_bound
                        ),
                    )
                    .with_intractable()
                    .with_evidence([
                        "decidable (see comp.halts / comp.decidable-equivalence) is not feasible; no tape simulator is run"
                            .to_string(),
                    ])
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by a computational object"),
        }
    }
}

/// Landauer's principle: erasing one bit dissipates at least `k_B·T·ln2`.
pub const INFO_LANDAUER_COST: &str = "info.landauer-cost";
/// The process erases no information, so it can be thermodynamically free.
pub const INFO_THERMO_FREE: &str = "info.thermodynamically-free";

/// Landauer bound on the live package: `E = N kT ln2`.
const LANDAUER_LN2_EQ: &str = "erase kT ln2";
/// Dropped-ln2 encoding: `E = N kT`.
const KT_EQ: &str = "erase kT";

fn parse_landauer_bound(pkg: &TheoryPackage) -> Result<bool, String> {
    let mut ln2 = false;
    let mut drop_ln2 = false;
    for eq in &pkg.equations {
        match eq.trim() {
            LANDAUER_LN2_EQ => ln2 = true,
            KT_EQ => drop_ln2 = true,
            _ => {}
        }
    }
    if !ln2 {
        return Err(format!("{} package has no kT ln2 Landauer bound", pkg.id));
    }
    Ok(drop_ln2)
}

fn landauer_cost_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["kT ln2 Landauer bound".into()],
        vec!["E = N k_B T ln2 for irreversible erasure".into()],
        "The cost cell is the kT ln2 encoding. Dropping ln2 is a new encoding, \
         not a silent reversible knob.",
    )
}

fn landauer_bound_joules(bits: u64, temperature_k: f64) -> f64 {
    bits as f64 * k_boltzmann().value() * temperature_k * std::f64::consts::LN_2
}

fn encoding_matches_ln2_bound(encoding: f64, bound: f64) -> bool {
    let scale = encoding.abs().max(bound.abs()).max(1e-40);
    (encoding - bound).abs() <= 1e-12 * scale
}

const LANDAUER_SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "temperature_k",
        layer: LayerId::Statistical,
        doc: "Bath temperature in kelvin; sets the Landauer energy scale k_B·T·ln2.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 0.0,
            max: 1.0e9,
        },
    },
    KnobSpec {
        name: "bits_erased",
        layer: LayerId::Information,
        doc: "Number of logical bits irreversibly erased by the computation.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt {
            min: 0,
            max: 1_000_000,
        },
    },
    KnobSpec {
        name: "reversible",
        layer: LayerId::Information,
        doc: "Whether the computation is logically reversible (Bennett): no bits are erased.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Bool,
    },
];

/// A computation coupled to a heat bath, judged by Landauer's principle.
///
/// This theory sits on the **information** and **statistical** layers at once:
/// it turns a fact about information (bits erased) into a typed thermodynamic
/// energy (`Qty<Energy>`), the bridge Landauer 1961 discovered. It is the first
/// object that reuses substrate from two domains — computation and
/// thermodynamics — in a single claim.
///
/// The `kT ln2` bound lives on the IR package. Dropping `ln2` is a package
/// mutation (`add-kt`), not a knob: the encoding energy is `N kT` and
/// `info.landauer-cost` fails. That fork is still this object, not a silent
/// Turing-machine install. `temperature_k` / `bits_erased` / `reversible`
/// stay knobs.
#[derive(Clone, Debug, PartialEq)]
pub struct LandauerEngine {
    /// Bath temperature in kelvin.
    temperature_k: f64,
    /// Logical bits irreversibly erased.
    bits_erased: u64,
    /// Whether the computation is logically reversible.
    reversible: bool,
    /// Whether the encoding dropped `ln2` (`E = N kT`).
    drop_ln2: bool,
}

impl Default for LandauerEngine {
    fn default() -> Self {
        // The canonical irreversible eraser: one bit at room temperature.
        Self {
            temperature_k: 300.0,
            bits_erased: 1,
            reversible: false,
            drop_ln2: false,
        }
    }
}

impl LandauerEngine {
    /// A logically reversible computer (Bennett): erases nothing, dissipates nothing.
    pub fn reversible() -> Self {
        Self {
            temperature_k: 300.0,
            bits_erased: 0,
            reversible: true,
            drop_ln2: false,
        }
    }

    /// Effective number of bits erased: zero if the computation is reversible.
    fn effective_bits(&self) -> u64 {
        if self.reversible {
            0
        } else {
            self.bits_erased
        }
    }

    /// Dissipated energy this encoding computes, as a typed quantity.
    ///
    /// Live: `E = N · k_B · T · ln2`. Mutant (`add-kt`): `E = N · k_B · T`.
    /// The units fall out of the type system: `k_B` carries J/K, `kelvin(T)`
    /// carries K, so the product is an energy.
    pub fn landauer_energy(&self) -> Qty<Energy> {
        let n = self.effective_bits() as f64;
        let kt_n = k_boltzmann() * kelvin(self.temperature_k) * n;
        if self.drop_ln2 {
            kt_n
        } else {
            kt_n * std::f64::consts::LN_2
        }
    }

    /// IR package for this bound encoding. Equations are `erase kT ln2`
    /// and, when forked, `erase kT`. Temperature, bits, and reversibility
    /// stay on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![LANDAUER_LN2_EQ.to_string()];
        if self.drop_ln2 {
            equations.push(KT_EQ.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["kt-ln2-landauer-bound".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: INFO_LANDAUER_COST.into(),
                statement: "Erasing a logical bit dissipates at least k_B·T·ln2 of energy.".into(),
                layer: "statistical".into(),
                class: "model-internal".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load a bound encoding from a package. Knobs default; overlay them
    /// from a live engine when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        if pkg.id != "landauer-engine" {
            return Err(format!(
                "landauer-engine package id '{}' is not landauer-engine",
                pkg.id
            ));
        }
        Ok(Self {
            drop_ln2: parse_landauer_bound(pkg)?,
            ..Self::default()
        })
    }

    fn kt_equation() -> String {
        KT_EQ.to_string()
    }
}

impl Knobbed for LandauerEngine {
    fn specs(&self) -> &'static [KnobSpec] {
        LANDAUER_SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "temperature_k" => Ok(KnobValue::Float(self.temperature_k)),
            "bits_erased" => Ok(KnobValue::UInt(self.bits_erased)),
            "reversible" => Ok(KnobValue::Bool(self.reversible)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("temperature_k", KnobValue::Float(v)) => self.temperature_k = v,
            ("bits_erased", KnobValue::UInt(v)) => self.bits_erased = v,
            ("reversible", KnobValue::Bool(v)) => self.reversible = v,
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

impl Theory for LandauerEngine {
    fn id(&self) -> &'static str {
        "landauer-engine"
    }
    fn name(&self) -> &'static str {
        "Landauer engine"
    }
    fn summary(&self) -> &'static str {
        "A computation coupled to a heat bath. Erasing a logical bit costs at \
         least k_B·T·ln2 of energy (Landauer) on the live encoding; dropping \
         ln2 is an IR mutation, not a reversible knob. A logically reversible \
         computation erases nothing and can be thermodynamically free (Bennett)."
    }
    fn world(&self) -> Option<World> {
        None // information/statistical content, no spacetime/gauge projection
    }
    fn note(&self) -> String {
        format!(
            "Landauer engine: {} bit(s) erased at {} K ({})",
            self.effective_bits(),
            self.temperature_k,
            if self.reversible {
                "reversible"
            } else {
                "irreversible"
            }
        )
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                INFO_LANDAUER_COST,
                "Erasing a logical bit dissipates at least k_B·T·ln2 of energy.",
                LayerId::Statistical,
                ClaimClass::ModelInternal,
            )
            .with_domain(landauer_cost_domain()),
            Claim::new(
                INFO_THERMO_FREE,
                "The computation erases no information and can dissipate no heat.",
                LayerId::Information,
                ClaimClass::ModelInternal,
            ),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        let e = self.landauer_energy().value();
        match claim.id_str() {
            INFO_LANDAUER_COST => {
                let n = self.effective_bits();
                let bound = landauer_bound_joules(n, self.temperature_k);
                if encoding_matches_ln2_bound(e, bound) {
                    Verdict::holds(
                        claim,
                        format!(
                            "erasing {n} bit(s) at {} K costs at least {e:.3e} J",
                            self.temperature_k
                        ),
                    )
                    .with_evidence([
                        format!(
                            "k_B·T·ln2 = {:.3e} J/bit",
                            landauer_bound_joules(1, self.temperature_k)
                        ),
                        format!("E = {e:.3e} J matches N·k_B·T·ln2 for N = {n}"),
                    ])
                } else {
                    Verdict::fails(
                        claim,
                        format!("encoding dissipates {e:.3e} J, not N kT ln2 = {bound:.3e} J"),
                    )
                    .with_evidence([format!(
                        "N kT without ln2 is not the Landauer floor (N = {n})"
                    )])
                }
            }
            INFO_THERMO_FREE => {
                if self.effective_bits() == 0 {
                    Verdict::holds(
                        claim,
                        "no bits erased: the process can be run with zero dissipation",
                    )
                    .with_evidence([format!("Landauer floor E_min = {e:.3e} J")])
                } else {
                    Verdict::fails(claim,
                        format!(
                            "erasing {} bit(s) forces at least {e:.3e} J of dissipation",
                            self.effective_bits()
                        ),
                    )
                    .with_evidence([
                        "reversible computation (reversible=true / bits_erased=0) would make this holds"
                            .to_string(),
                    ])
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by a Landauer engine"),
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.drop_ln2 = parsed.drop_ln2;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        if self.drop_ln2 {
            return Vec::new();
        }
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mutated = apply_mutation(&pkg, &PackageMutation::AppendEquation(Self::kt_equation()));
        match Self::from_package(&mutated) {
            Ok(parsed) if parsed.drop_ln2 => {
                let mut fork = self.clone();
                fork.drop_ln2 = true;
                vec![("add-kt".into(), Box::new(fork))]
            }
            _ => Vec::new(),
        }
    }
}

/// The computation experiment: a combinational circuit vs a Turing machine.
pub fn computation() -> ExperimentReport {
    let theories: Vec<Box<dyn Theory>> = vec![
        Box::new(CombinationalCircuit::default()),
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
            "`comp.feasible-decision` is coNP-complete (circuits) / exponential (bounded tape): decidable is not feasible. No simulator is run.".into(),
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
        let c = t.claims().into_iter().find(|c| c.id_str() == id).unwrap();
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
        // Decidable is not feasible: this lab does not enumerate the graph.
        assert_eq!(verdict(&tm, FEASIBLE_DECISION), VerdictKind::Undecidable);
        let claim = tm
            .claims()
            .into_iter()
            .find(|c| c.id_str() == FEASIBLE_DECISION)
            .unwrap();
        assert!(tm.evaluate(&claim).intractable());
    }

    #[test]
    fn combinational_halts_but_is_not_turing_complete() {
        let mut c = CombinationalCircuit::default();
        assert_eq!(verdict(&c, HALTS), VerdictKind::Holds);
        assert_eq!(verdict(&c, TURING_COMPLETE), VerdictKind::Fails);
        assert_eq!(verdict(&c, DECIDABLE_EQUIVALENCE), VerdictKind::Holds);
        assert_eq!(verdict(&c, FEASIBLE_DECISION), VerdictKind::Undecidable);
        assert_eq!(verdict(&c, ACYCLIC), VerdictKind::Holds);
        let claim = c
            .claims()
            .into_iter()
            .find(|cl| cl.id_str() == FEASIBLE_DECISION)
            .unwrap();
        assert!(c.evaluate(&claim).intractable());
        assert!(
            c.set("feedback", KnobValue::Bool(true)).is_err(),
            "feedback is an IR mutation, not a knob"
        );
    }

    #[test]
    fn ir_feedback_mutation_closes_a_cycle() {
        let c = CombinationalCircuit::default();
        assert!(c.is_acyclic());
        let src = render_package(&c.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            CombinationalCircuit::from_package(&pkg).unwrap(),
            c,
            "IR round-trip must preserve the netlist"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(c.feedback_equation()),
        );
        let fork = CombinationalCircuit::from_package(&mutated).unwrap();
        assert!(!fork.is_acyclic());
        assert_eq!(verdict(&fork, ACYCLIC), VerdictKind::Fails);
        assert_eq!(verdict(&fork, HALTS), VerdictKind::Inapplicable);
        assert_eq!(verdict(&c, HALTS), VerdictKind::Holds);
        let probes = c.structural_mutations();
        assert_eq!(probes.len(), 1);
        assert_eq!(probes[0].0, "add-feedback");
        let canonical = physis_ir::certify_round_trip(&c.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = c.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), c.package());
        assert!(rebuilt.ir_package().unwrap() != probes[0].1.ir_package().unwrap());
    }

    #[test]
    fn unbounded_machine_feasible_decision_is_inapplicable() {
        let tm = TuringMachine::default();
        assert_eq!(verdict(&tm, FEASIBLE_DECISION), VerdictKind::Inapplicable);
        let claim = tm
            .claims()
            .into_iter()
            .find(|c| c.id_str() == FEASIBLE_DECISION)
            .unwrap();
        assert!(!tm.evaluate(&claim).intractable());
    }

    #[test]
    fn p_vs_np_is_honestly_open() {
        // The lab refuses to pretend it knows: P vs NP is Undecidable/Open.
        let tm = TuringMachine::default();
        let c = tm
            .claims()
            .into_iter()
            .find(|c| c.id_str() == P_EQUALS_NP)
            .unwrap();
        let v = tm.evaluate(&c);
        assert_eq!(v.kind, VerdictKind::Undecidable);
        assert_eq!(v.class, ClaimClass::OpenProblem);
        // It does not apply to a single fixed circuit.
        assert_eq!(
            verdict(&CombinationalCircuit::default(), P_EQUALS_NP),
            VerdictKind::Inapplicable
        );
    }

    #[test]
    fn nondeterminism_knob_flips_determinism() {
        let mut tm = TuringMachine::default();
        assert_eq!(verdict(&tm, DETERMINISTIC), VerdictKind::Holds);
        tm.set("nondeterministic", KnobValue::Bool(true)).unwrap();
        assert_eq!(verdict(&tm, DETERMINISTIC), VerdictKind::Fails);
    }

    #[test]
    fn landauer_bound_is_computed_from_typed_constants() {
        // One bit at 300 K: k_B·T·ln2 ≈ 2.87e-21 J. The value comes from the
        // typed Boltzmann constant, so its units are checked at compile time.
        let e = LandauerEngine::default();
        let joules = e.landauer_energy().value();
        let expected = 1.380_649e-23 * 300.0 * std::f64::consts::LN_2;
        assert!((joules - expected).abs() < 1e-30, "got {joules}");
        assert!((2.5e-21..3.2e-21).contains(&joules));
    }

    #[test]
    fn erasing_bits_forces_dissipation() {
        let e = LandauerEngine::default();
        assert_eq!(verdict(&e, INFO_THERMO_FREE), VerdictKind::Fails);
        assert_eq!(verdict(&e, INFO_LANDAUER_COST), VerdictKind::Holds);
    }

    #[test]
    fn reversibility_knob_removes_the_cost() {
        // The cross-domain knob → verdict diff: reversible computation is free.
        let mut e = LandauerEngine::default();
        assert_eq!(verdict(&e, INFO_THERMO_FREE), VerdictKind::Fails);
        assert!(e.landauer_energy().value() > 0.0);

        e.set("reversible", KnobValue::Bool(true)).unwrap();
        assert_eq!(verdict(&e, INFO_THERMO_FREE), VerdictKind::Holds);
        assert_eq!(e.landauer_energy().value(), 0.0);

        // Setting bits_erased to 0 is the other route to a free process.
        let mut e2 = LandauerEngine::default();
        e2.set("bits_erased", KnobValue::UInt(0)).unwrap();
        assert_eq!(verdict(&e2, INFO_THERMO_FREE), VerdictKind::Holds);
    }

    #[test]
    fn cost_scales_with_bits_and_temperature() {
        let one = LandauerEngine::default();
        let mut ten = LandauerEngine::default();
        ten.set("bits_erased", KnobValue::UInt(10)).unwrap();
        let r = ten.landauer_energy().value() / one.landauer_energy().value();
        assert!((r - 10.0).abs() < 1e-9, "ratio {r}");

        let mut hot = LandauerEngine::default();
        hot.set("temperature_k", KnobValue::Float(600.0)).unwrap();
        let r2 = hot.landauer_energy().value() / one.landauer_energy().value();
        assert!((r2 - 2.0).abs() < 1e-9, "ratio {r2}");
    }

    #[test]
    fn kt_without_ln2_is_ir_not_a_knob() {
        let mut e = LandauerEngine::default();
        assert!(
            LandauerEngine::default()
                .set("kt", KnobValue::Bool(true))
                .is_err(),
            "dropping ln2 is an IR mutation, not a knob"
        );
        assert!(
            LandauerEngine::default()
                .set("drop_ln2", KnobValue::Bool(true))
                .is_err(),
            "drop_ln2 is not a knob"
        );
        assert!(
            LandauerEngine::default()
                .set("ln2", KnobValue::Bool(false))
                .is_err(),
            "ln2 is not a knob"
        );
        assert!(
            LandauerEngine::default()
                .set("tape_bound", KnobValue::UInt(1000))
                .is_err(),
            "landauer-engine must not grow a tape_bound knob; that stays on turing-machine"
        );
        let src = render_package(&e.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            LandauerEngine::from_package(&pkg).unwrap(),
            e,
            "IR round-trip must preserve the kT ln2 bound"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(LandauerEngine::kt_equation()),
        );
        let parsed = LandauerEngine::from_package(&mutated).unwrap();
        assert!(parsed.drop_ln2);
        let mut fork = e.clone();
        fork.drop_ln2 = true;
        assert_eq!(verdict(&fork, INFO_LANDAUER_COST), VerdictKind::Fails);
        assert_eq!(verdict(&e, INFO_LANDAUER_COST), VerdictKind::Holds);
        assert_eq!(verdict(&fork, INFO_THERMO_FREE), VerdictKind::Fails);
        let live_e = e.landauer_energy().value();
        let mutant_e = fork.landauer_energy().value();
        let bound = landauer_bound_joules(1, 300.0);
        assert!(
            encoding_matches_ln2_bound(live_e, bound),
            "live energy must be N kT ln2, got {live_e}"
        );
        assert!(
            (mutant_e / live_e - 1.0 / std::f64::consts::LN_2).abs() < 1e-9,
            "mutant energy must be N kT = (N kT ln2)/ln2, got {mutant_e} / {live_e}"
        );
        e.set("reversible", KnobValue::Bool(true)).unwrap();
        assert_eq!(verdict(&e, INFO_THERMO_FREE), VerdictKind::Holds);
        assert_eq!(verdict(&e, INFO_LANDAUER_COST), VerdictKind::Holds);
        let mut free_fork = fork.clone();
        free_fork.set("reversible", KnobValue::Bool(true)).unwrap();
        assert_eq!(verdict(&free_fork, INFO_LANDAUER_COST), VerdictKind::Holds);
        assert_eq!(verdict(&free_fork, INFO_THERMO_FREE), VerdictKind::Holds);
        let probes = LandauerEngine::default().structural_mutations();
        assert_eq!(probes.len(), 1);
        assert_eq!(probes[0].0, "add-kt");
        assert_eq!(
            verdict(probes[0].1.as_ref(), INFO_LANDAUER_COST),
            VerdictKind::Fails
        );
        assert!(fork.structural_mutations().is_empty());
        let live = LandauerEngine::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), live.package());
        assert_eq!(
            rebuilt.get("temperature_k").unwrap(),
            KnobValue::Float(300.0),
            "reparse must overlay bound IR onto live knobs"
        );
        assert_eq!(
            rebuilt.get("bits_erased").unwrap(),
            KnobValue::UInt(1),
            "reparse must overlay bound IR onto live knobs"
        );
        assert_eq!(
            rebuilt.get("reversible").unwrap(),
            KnobValue::Bool(false),
            "reparse must overlay bound IR onto live knobs"
        );
        assert_eq!(
            verdict(rebuilt.as_ref(), INFO_LANDAUER_COST),
            VerdictKind::Holds
        );
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == INFO_LANDAUER_COST)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "landauer-cost must name kT ln2: {:?}",
            cell.domain()
        );
        assert!(
            TuringMachine::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-kt"),
            "turing-machine must not grow add-kt"
        );
        assert!(
            TuringMachine::default()
                .set("tape_bound", KnobValue::UInt(1000))
                .is_ok(),
            "turing-machine keeps the tape_bound knob"
        );
        assert!(
            TuringMachine::default()
                .set("nondeterministic", KnobValue::Bool(true))
                .is_ok(),
            "turing-machine keeps the nondeterministic knob"
        );
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
