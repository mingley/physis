//! Continuum (M4), gauge sector: a gauge field as **link objects**, not a flag.
//!
//! [`WilsonU1`] is a compact U(1) lattice gauge theory. The degrees of freedom
//! live on the links between sites; the action is the sum over plaquettes of
//! `1 - cos(θ_plaquette)`. Gauge invariance and locality are structural
//! theorems of that construction. The confinement/deconfinement behaviour is a
//! knob-sensitive, honestly-labelled result of lattice gauge theory:
//!
//! - compact U(1) confines at all couplings in 2D and 3D;
//! - in 4D it has a phase transition near `β ≈ 1.01`: confining below, a
//!   Coulomb (free-photon) phase above.

use physis_core::claim::{Claim, Epistemic, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_model::{GaugeGroup, Manifold, SimpleGroup, Spectrum, World};

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;

/// The action is invariant under local gauge transformations of the links.
pub const GAUGE_INVARIANT: &str = "gauge.invariant";
/// The action couples only neighbouring links (plaquettes).
pub const GAUGE_LOCAL: &str = "gauge.local";
/// Static charges are confined (area law / linear potential).
pub const CONFINING: &str = "gauge.confining";
/// The coupling runs to zero at high energy (asymptotic freedom).
pub const ASYMPTOTIC_FREEDOM: &str = "gauge.asymptotic-freedom";

/// Matrix rows for the lattice-gauge lab.
pub fn gauge_rows() -> [&'static str; 4] {
    [GAUGE_INVARIANT, GAUGE_LOCAL, CONFINING, ASYMPTOTIC_FREEDOM]
}

/// Approximate 4D compact-U(1) deconfinement coupling (β = 1/g²).
const BETA_C_4D: f64 = 1.01;

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "dimension",
        layer: LayerId::Spacetime,
        doc: "Lattice spacetime dimension (2–4). Compact U(1) confines at all β in 2D/3D.",
        domain: KnobDomain::UInt { min: 2, max: 4 },
    },
    KnobSpec {
        name: "beta",
        layer: LayerId::Interaction,
        doc: "Inverse coupling β = 1/g². In 4D, β below ~1.01 confines; above it is the Coulomb phase.",
        domain: KnobDomain::Float {
            min: 0.0,
            max: 100.0,
        },
    },
    KnobSpec {
        name: "sites_per_side",
        layer: LayerId::Spacetime,
        doc: "Linear lattice size L (the lattice has L^dimension sites).",
        domain: KnobDomain::UInt { min: 2, max: 256 },
    },
];

/// Compact U(1) lattice gauge theory (Wilson action).
#[derive(Clone, Debug)]
pub struct WilsonU1 {
    dimension: u8,
    beta: f64,
    sites_per_side: u32,
}

impl Default for WilsonU1 {
    fn default() -> Self {
        // 4D at β = 1.0: just inside the confining phase.
        Self {
            dimension: 4,
            beta: 1.0,
            sites_per_side: 8,
        }
    }
}

impl WilsonU1 {
    fn is_confining(&self) -> bool {
        match self.dimension {
            2 | 3 => true,              // compact U(1) always confines here
            _ => self.beta < BETA_C_4D, // 4D: confining below the transition
        }
    }
}

impl Knobbed for WilsonU1 {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "dimension" => Ok(KnobValue::UInt(self.dimension as u64)),
            "beta" => Ok(KnobValue::Float(self.beta)),
            "sites_per_side" => Ok(KnobValue::UInt(self.sites_per_side as u64)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("dimension", KnobValue::UInt(v)) => self.dimension = v as u8,
            ("beta", KnobValue::Float(v)) => self.beta = v,
            ("sites_per_side", KnobValue::UInt(v)) => self.sites_per_side = v as u32,
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

impl Theory for WilsonU1 {
    fn id(&self) -> &'static str {
        "wilson-u1"
    }
    fn name(&self) -> &'static str {
        "Wilson U(1) lattice gauge"
    }
    fn summary(&self) -> &'static str {
        "Compact U(1) lattice gauge theory: the gauge field lives on links and \
         the action sums 1 − cos(θ) over plaquettes. Gauge invariance and \
         locality are structural theorems; confinement is a knob-sensitive \
         lattice result (all β in 2D/3D; a transition near β ≈ 1.01 in 4D)."
    }
    fn world(&self) -> Option<World> {
        let space = self.dimension.saturating_sub(1);
        Some(World {
            spacetime: Manifold {
                dim: self.dimension,
                signature: physis_model::Signature { time: 1, space },
                compact_extra: 0,
                compact_radius_planck: 0.0,
                topology: physis_model::Topology::Minkowski,
                convention: physis_model::SignConvention::MostlyPlus,
            },
            gauge: GaugeGroup {
                factors: vec![SimpleGroup::U1],
            },
            spectrum: Spectrum::empty(),
            has_gravity: false,
            supersymmetric: false,
            free_parameter_count: 1,
            landscape_log10: 0.0,
            note: format!(
                "compact U(1) on a {}^{} lattice, β={}, {}",
                self.sites_per_side,
                self.dimension,
                self.beta,
                if self.is_confining() {
                    "confining"
                } else {
                    "Coulomb phase"
                }
            ),
        })
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                GAUGE_INVARIANT,
                "The action is invariant under local gauge transformations of the links.",
                LayerId::Interaction,
                Epistemic::Theorem,
            ),
            Claim::new(
                GAUGE_LOCAL,
                "The action couples only neighbouring links (plaquettes).",
                LayerId::Interaction,
                Epistemic::Theorem,
            ),
            Claim::new(
                CONFINING,
                "Static charges are confined.",
                LayerId::Interaction,
                Epistemic::Heuristic,
            ),
            Claim::new(
                ASYMPTOTIC_FREEDOM,
                "The coupling runs to zero at high energy.",
                LayerId::Interaction,
                Epistemic::EncodedFact,
            ),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id.0.as_str() {
            ASYMPTOTIC_FREEDOM => Verdict::fails(
                Epistemic::EncodedFact,
                "abelian U(1) is not asymptotically free: the coupling grows with energy (Landau pole)",
            ),
            GAUGE_INVARIANT => Verdict::holds(
                Epistemic::Theorem,
                "plaquette action is invariant under U_μ(x) → g(x) U_μ(x) g(x+μ̂)†",
            ),
            GAUGE_LOCAL => Verdict::holds(
                Epistemic::Theorem,
                "the action sums over plaquettes: only neighbouring links couple",
            ),
            CONFINING => match self.dimension {
                2 | 3 => Verdict::holds(
                    Epistemic::EncodedFact,
                    format!(
                        "compact U(1) confines at all β in {}D (Polyakov)",
                        self.dimension
                    ),
                ),
                _ => {
                    if self.beta < BETA_C_4D {
                        Verdict::holds(
                            Epistemic::Heuristic,
                            format!(
                                "4D strong coupling β={} < β_c≈{BETA_C_4D}: confining",
                                self.beta
                            ),
                        )
                    } else {
                        Verdict::fails(
                            Epistemic::Heuristic,
                            format!(
                                "4D weak coupling β={} ≥ β_c≈{BETA_C_4D}: Coulomb (deconfined) phase",
                                self.beta
                            ),
                        )
                        .with_evidence([
                            "compact U(1) in 4D has a phase transition; the continuum limit here is free Maxwell".to_string(),
                        ])
                    }
                }
            },
            _ => Verdict::inapplicable("claim not made by a lattice gauge object"),
        }
    }
}

const SUN_SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "dimension",
        layer: LayerId::Spacetime,
        doc: "Lattice spacetime dimension (2–4).",
        domain: KnobDomain::UInt { min: 2, max: 4 },
    },
    KnobSpec {
        name: "beta",
        layer: LayerId::Interaction,
        doc: "Inverse coupling β = 2N/g². Non-abelian gauge theories confine at all β in this encoding.",
        domain: KnobDomain::Float {
            min: 0.0,
            max: 100.0,
        },
    },
    KnobSpec {
        name: "sites_per_side",
        layer: LayerId::Spacetime,
        doc: "Linear lattice size L.",
        domain: KnobDomain::UInt { min: 2, max: 256 },
    },
];

/// Non-abelian SU(N) Wilson lattice gauge theory (N = 2 or 3).
#[derive(Clone, Debug)]
pub struct WilsonSun {
    n: u8,
    dimension: u8,
    beta: f64,
    sites_per_side: u32,
}

impl WilsonSun {
    /// SU(2) Yang–Mills.
    pub fn su2() -> Self {
        Self {
            n: 2,
            dimension: 4,
            beta: 2.3,
            sites_per_side: 8,
        }
    }

    /// SU(3) Yang–Mills (the gauge group of QCD).
    pub fn su3() -> Self {
        Self {
            n: 3,
            dimension: 4,
            beta: 6.0,
            sites_per_side: 8,
        }
    }
}

impl Knobbed for WilsonSun {
    fn specs(&self) -> &'static [KnobSpec] {
        SUN_SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "dimension" => Ok(KnobValue::UInt(self.dimension as u64)),
            "beta" => Ok(KnobValue::Float(self.beta)),
            "sites_per_side" => Ok(KnobValue::UInt(self.sites_per_side as u64)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("dimension", KnobValue::UInt(v)) => self.dimension = v as u8,
            ("beta", KnobValue::Float(v)) => self.beta = v,
            ("sites_per_side", KnobValue::UInt(v)) => self.sites_per_side = v as u32,
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

impl Theory for WilsonSun {
    fn id(&self) -> &'static str {
        match self.n {
            2 => "wilson-su2",
            _ => "wilson-su3",
        }
    }
    fn name(&self) -> &'static str {
        match self.n {
            2 => "Wilson SU(2) lattice gauge",
            _ => "Wilson SU(3) lattice gauge",
        }
    }
    fn summary(&self) -> &'static str {
        "Non-abelian Wilson lattice gauge theory. Unlike compact U(1), SU(N) is \
         asymptotically free and is expected to confine at all couplings in 4D — \
         but 4D confinement / the Yang–Mills mass gap is unproven (a Millennium \
         Problem), so that verdict is honestly a conjecture."
    }
    fn world(&self) -> Option<World> {
        let space = self.dimension.saturating_sub(1);
        Some(World {
            spacetime: Manifold {
                dim: self.dimension,
                signature: physis_model::Signature { time: 1, space },
                compact_extra: 0,
                compact_radius_planck: 0.0,
                topology: physis_model::Topology::Minkowski,
                convention: physis_model::SignConvention::MostlyPlus,
            },
            gauge: GaugeGroup {
                factors: vec![SimpleGroup::Su(self.n)],
            },
            spectrum: Spectrum::empty(),
            has_gravity: false,
            supersymmetric: false,
            free_parameter_count: 1,
            landscape_log10: 0.0,
            note: format!(
                "SU({}) on a {}^{} lattice, β={}",
                self.n, self.sites_per_side, self.dimension, self.beta
            ),
        })
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                GAUGE_INVARIANT,
                "The action is invariant under local gauge transformations of the links.",
                LayerId::Interaction,
                Epistemic::Theorem,
            ),
            Claim::new(
                GAUGE_LOCAL,
                "The action couples only neighbouring links (plaquettes).",
                LayerId::Interaction,
                Epistemic::Theorem,
            ),
            Claim::new(
                CONFINING,
                "Static charges are confined.",
                LayerId::Interaction,
                Epistemic::Conjecture,
            ),
            Claim::new(
                ASYMPTOTIC_FREEDOM,
                "The coupling runs to zero at high energy.",
                LayerId::Interaction,
                Epistemic::EncodedFact,
            ),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id.0.as_str() {
            GAUGE_INVARIANT => Verdict::holds(
                Epistemic::Theorem,
                "non-abelian plaquette action is gauge invariant by construction",
            ),
            GAUGE_LOCAL => Verdict::holds(
                Epistemic::Theorem,
                "the action sums over plaquettes: only neighbouring links couple",
            ),
            ASYMPTOTIC_FREEDOM => Verdict::holds(
                Epistemic::EncodedFact,
                "non-abelian SU(N) is asymptotically free (Gross–Wilczek–Politzer 1973)",
            ),
            CONFINING => match self.dimension {
                2 | 3 => Verdict::holds(
                    Epistemic::EncodedFact,
                    format!("SU({}) confines in {}D", self.n, self.dimension),
                ),
                _ => Verdict::holds(
                    Epistemic::Conjecture,
                    format!(
                        "SU({}) is expected to confine in 4D at all β, but the mass gap is unproven",
                        self.n
                    ),
                )
                .with_evidence([
                    "4D Yang–Mills existence and mass gap is a Clay Millennium Problem".to_string(),
                ]),
            },
            _ => Verdict::inapplicable("claim not made by a lattice gauge object"),
        }
    }
}

/// The lattice-gauge experiment: compact U(1) vs non-abelian SU(2)/SU(3).
pub fn gauge_lattice() -> ExperimentReport {
    let theories: Vec<Box<dyn Theory>> = vec![
        Box::new(WilsonU1::default()),
        Box::new(WilsonSun::su2()),
        Box::new(WilsonSun::su3()),
    ];
    report_from_rows(
        "gauge-lattice",
        "Lattice gauge lab",
        "How do abelian and non-abelian gauge fields on a lattice differ? Compact \
         U(1) (QED-like) vs SU(2)/SU(3) (Yang–Mills): which confine, which are \
         asymptotically free, and which claims are theorems vs conjectures?",
        "Gauge invariance and locality are theorems of the Wilson construction. \
         U(1) is not asymptotically free and deconfines in 4D above β≈1.01; SU(N) \
         is asymptotically free and is *expected* to confine in 4D — but that is \
         the unproven Yang–Mills mass gap, so it is honestly a conjecture.",
        vec![
            "`holds` / `fails` are internal to the encoding; read the `epistemic` tag.".into(),
            "The gauge field lives on links; the action sums over plaquettes.".into(),
            "U(1): `set wilson-u1 beta 2` deconfines the 4D theory (Coulomb phase).".into(),
            "SU(N): 4D confinement holds as a *conjecture* — the Millennium mass-gap problem."
                .into(),
        ],
        &gauge_rows(),
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
    fn gauge_invariance_and_locality_are_theorems() {
        let w = WilsonU1::default();
        assert_eq!(verdict(&w, GAUGE_INVARIANT), VerdictKind::Holds);
        assert_eq!(verdict(&w, GAUGE_LOCAL), VerdictKind::Holds);
    }

    #[test]
    fn four_d_coupling_flips_confinement() {
        // The gauge knob → verdict diff: 4D compact U(1) deconfines at weak coupling.
        let mut w = WilsonU1::default(); // 4D, β=1.0
        assert_eq!(verdict(&w, CONFINING), VerdictKind::Holds);
        w.set("beta", KnobValue::Float(2.0)).unwrap();
        assert_eq!(verdict(&w, CONFINING), VerdictKind::Fails);
    }

    #[test]
    fn low_dimensions_always_confine() {
        let mut w = WilsonU1::default();
        w.set("dimension", KnobValue::UInt(3)).unwrap();
        w.set("beta", KnobValue::Float(50.0)).unwrap();
        assert_eq!(verdict(&w, CONFINING), VerdictKind::Holds);
    }

    #[test]
    fn qed_and_qcd_differ_on_asymptotic_freedom() {
        // U(1) is not asymptotically free; SU(N) is.
        assert_eq!(
            verdict(&WilsonU1::default(), ASYMPTOTIC_FREEDOM),
            VerdictKind::Fails
        );
        assert_eq!(
            verdict(&WilsonSun::su3(), ASYMPTOTIC_FREEDOM),
            VerdictKind::Holds
        );
    }

    #[test]
    fn four_d_su3_confinement_is_a_conjecture() {
        // SU(3) confines in 4D (holds) but only as a conjecture (mass gap unproven).
        let qcd = WilsonSun::su3();
        let c = qcd
            .claims()
            .into_iter()
            .find(|c| c.id.0 == CONFINING)
            .unwrap();
        let v = qcd.evaluate(&c);
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_eq!(v.epistemic, Epistemic::Conjecture);
        // Unlike U(1), it stays confining at weak coupling.
        let mut qcd_weak = WilsonSun::su3();
        qcd_weak.set("beta", KnobValue::Float(50.0)).unwrap();
        assert_eq!(verdict(&qcd_weak, CONFINING), VerdictKind::Holds);
    }

    #[test]
    fn gauge_experiment_builds_a_matrix() {
        let r = gauge_lattice();
        assert_eq!(r.id, "gauge-lattice");
        assert_eq!(r.theories.len(), 3);
        let af = r.matrix.get(ASYMPTOTIC_FREEDOM).expect("row");
        assert_eq!(af.get("wilson-u1").copied(), Some(VerdictKind::Fails));
        assert_eq!(af.get("wilson-su3").copied(), Some(VerdictKind::Holds));
    }
}
