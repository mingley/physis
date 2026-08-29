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

/// Matrix rows for the lattice-gauge lab.
pub fn gauge_rows() -> [&'static str; 3] {
    [GAUGE_INVARIANT, GAUGE_LOCAL, CONFINING]
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
    fn world(&self) -> World {
        let space = self.dimension.saturating_sub(1);
        World {
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
        }
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
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id.0.as_str() {
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

/// The lattice-gauge experiment: a compact U(1) gauge field on links.
pub fn gauge_lattice() -> ExperimentReport {
    let theories: Vec<Box<dyn Theory>> = vec![Box::new(WilsonU1::default())];
    report_from_rows(
        "gauge-lattice",
        "Lattice gauge lab",
        "Can a gauge field live on the links of a lattice with gauge invariance \
         and locality as structural theorems, and does compact U(1) confine in a \
         way that flips with the coupling in 4D?",
        "Gauge invariance and locality are theorems of the Wilson construction. \
         Confinement is a lattice-gauge result: a theorem in 2D/3D (encoded), and \
         a knob-sensitive heuristic across the 4D transition near β ≈ 1.01.",
        vec![
            "`holds` / `fails` are internal to the encoding.".into(),
            "The gauge field lives on links; the action sums 1 − cos(θ) over plaquettes.".into(),
            "`set wilson-u1 beta 2` deconfines the 4D theory (Coulomb phase); `set wilson-u1 dimension 3` confines at any β.".into(),
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
    fn gauge_experiment_builds_a_matrix() {
        let r = gauge_lattice();
        assert_eq!(r.id, "gauge-lattice");
        assert_eq!(r.theories.len(), 1);
        assert_eq!(
            r.matrix
                .get(GAUGE_INVARIANT)
                .and_then(|m| m.get("wilson-u1"))
                .copied(),
            Some(VerdictKind::Holds)
        );
    }
}
