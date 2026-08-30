//! Thermodynamics: a fourth domain, populating the `statistical` layer.
//!
//! [`IdealGas`] is a monatomic classical ideal gas. It exists to (a) show the
//! typed substrate reaching the statistical layer that earlier milestones left
//! empty, (b) exercise `Qty<Temperature>` and `Qty<Energy>` so the compiler
//! keeps kelvin and joules apart, and (c) record an honest failure: the
//! *classical* ideal gas violates the third law of thermodynamics.
//!
//! Cavity radiation — the ultraviolet catastrophe that killed Rayleigh–Jeans —
//! lives in [`crate::blackbody`]: same statistical layer, a different standing
//! theory on trial.
//!
//! Like computation, the ideal gas has no spacetime/gauge/spectrum, so it
//! returns `None` from `Theory::world()` and describes itself via `note`.

use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::qty::kelvin;
use physis_core::ParameterOrigin;
use physis_core::{Energy, Qty};
use physis_model::constants::k_boltzmann;
use physis_model::World;

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;
use crate::solid::{EinsteinSolid, DEBYE_T3, DULONG_PETIT, HIGH_T_CLASSICAL};

/// Energy is equipartitioned: U = (3/2) N k T, so C_v = (3/2) N k.
pub const EQUIPARTITION: &str = "thermo.equipartition";
/// The second law: a spontaneous (free) expansion does not decrease entropy.
pub const SECOND_LAW: &str = "thermo.second-law";
/// The third law: entropy → 0 as T → 0.
pub const THIRD_LAW: &str = "thermo.third-law";

/// Matrix rows for the thermodynamics lab.
pub fn thermo_rows() -> [&'static str; 6] {
    [
        EQUIPARTITION,
        SECOND_LAW,
        THIRD_LAW,
        DULONG_PETIT,
        HIGH_T_CLASSICAL,
        DEBYE_T3,
    ]
}

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "temperature",
        layer: LayerId::Statistical,
        doc: "Temperature in kelvin.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 1.0e-6,
            max: 1.0e9,
        },
    },
    KnobSpec {
        name: "volume_ratio",
        layer: LayerId::Statistical,
        doc: "Final/initial volume V_f/V_i for an isothermal expansion. > 1 is a spontaneous free expansion.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 1.0e-3,
            max: 1.0e6,
        },
    },
    KnobSpec {
        name: "particles",
        layer: LayerId::Statistical,
        doc: "Number of gas particles N.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 1.0,
            max: 1.0e30,
        },
    },
];

/// A monatomic classical ideal gas.
#[derive(Clone, Debug)]
pub struct IdealGas {
    temperature_k: f64,
    volume_ratio: f64,
    particles: f64,
}

impl Default for IdealGas {
    fn default() -> Self {
        Self {
            temperature_k: 300.0,
            volume_ratio: 2.0,
            particles: 1.0e23,
        }
    }
}

impl IdealGas {
    /// Internal energy U = (3/2) N k T, as a typed energy (kelvin × k_B × N).
    fn internal_energy(&self) -> Qty<Energy> {
        k_boltzmann() * kelvin(self.temperature_k) * (1.5 * self.particles)
    }

    /// Heat capacity C_v = dU/dT, computed by finite difference (should be
    /// (3/2) N k for equipartition).
    fn heat_capacity(&self) -> f64 {
        let h = self.temperature_k * 1e-6;
        let mut hot = self.clone();
        hot.temperature_k += h;
        (hot.internal_energy().value() - self.internal_energy().value()) / h
    }

    /// Entropy change of an isothermal volume change, in units of k: N ln(V_f/V_i).
    fn entropy_change_over_k(&self) -> f64 {
        self.particles * self.volume_ratio.ln()
    }
}

impl Knobbed for IdealGas {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "temperature" => Ok(KnobValue::Float(self.temperature_k)),
            "volume_ratio" => Ok(KnobValue::Float(self.volume_ratio)),
            "particles" => Ok(KnobValue::Float(self.particles)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("temperature", KnobValue::Float(v)) => self.temperature_k = v,
            ("volume_ratio", KnobValue::Float(v)) => self.volume_ratio = v,
            ("particles", KnobValue::Float(v)) => self.particles = v,
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

impl Theory for IdealGas {
    fn id(&self) -> &'static str {
        "ideal-gas"
    }
    fn name(&self) -> &'static str {
        "Ideal gas (monatomic)"
    }
    fn summary(&self) -> &'static str {
        "A monatomic classical ideal gas on the statistical layer. Equipartition \
         and the second law hold; the third law fails, because a classical ideal \
         gas has unbounded-below entropy (S ∝ ln T) — quantum statistics are \
         needed. Temperatures and energies are kept apart by the type system."
    }
    fn world(&self) -> Option<World> {
        None // thermodynamics lives on the statistical layer, not spacetime
    }
    fn note(&self) -> String {
        format!(
            "ideal gas: T = {} K, N = {:.2e}, V_f/V_i = {}",
            self.temperature_k, self.particles, self.volume_ratio
        )
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                EQUIPARTITION,
                "Energy is equipartitioned: C_v = (3/2) N k.",
                LayerId::Statistical,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                SECOND_LAW,
                "A spontaneous free expansion does not decrease entropy.",
                LayerId::Statistical,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                THIRD_LAW,
                "Entropy tends to zero as temperature tends to zero.",
                LayerId::Statistical,
                ClaimClass::ModelInternal,
            ),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id_str() {
            EQUIPARTITION => {
                let cv = self.heat_capacity();
                let expected = 1.5 * self.particles * k_boltzmann().value();
                if (cv - expected).abs() <= 1e-6 * expected.abs() {
                    Verdict::holds(claim, "C_v = dU/dT = (3/2) N k, verified numerically")
                        .with_evidence([format!(
                            "computed C_v/(Nk) = {:.4}",
                            cv / (self.particles * k_boltzmann().value())
                        )])
                } else {
                    Verdict::fails(claim, "heat capacity is not (3/2) N k")
                }
            }
            SECOND_LAW => {
                let ds = self.entropy_change_over_k();
                if self.volume_ratio >= 1.0 {
                    Verdict::holds(claim, "ΔS = N k ln(V_f/V_i) ≥ 0 for a free expansion")
                        .with_evidence([format!("ΔS/k = {ds:.3e}")])
                } else {
                    Verdict::fails(claim,
                        format!(
                            "ΔS/k = {ds:.3e} < 0: a spontaneous compression would violate the second law"
                        ),
                    )
                }
            }
            THIRD_LAW => {
                // Classical S has a (3/2)Nk·ln T term with no lower bound.
                let s_term = 1.5 * self.temperature_k.ln();
                Verdict::fails(claim,
                    "classical ideal-gas entropy S ∝ (3/2) ln T → −∞ as T → 0; the third law needs quantum statistics",
                )
                .with_evidence([format!(
                    "the ln-T term is {s_term:.2} and diverges as T → 0"
                )])
            }
            _ => Verdict::inapplicable(claim, "claim not made by a thermodynamic object"),
        }
    }
}

/// The thermodynamics experiment: a classical ideal gas and its three laws.
pub fn thermodynamics() -> ExperimentReport {
    report_from_rows(
        "thermo",
        "Thermodynamics lab",
        "Does the typed substrate reach the statistical layer, keeping kelvin and \
         joules apart — and do classical theories (ideal gas, Dulong–Petit) honestly \
         fail the third law while Einstein and Debye hold it, with only Debye holding T³?",
        "Equipartition and the second law are computed; third-law failures are \
         real properties of the classical encodings, not modelling shortcuts. \
         Einstein's exponential freeze-out and Debye's T³ both hold the third-law \
         row; only Debye holds thermo.debye-t3.",
        vec![
            "`holds` / `fails` are internal to the encoding.".into(),
            "U, C_v and ΔS are computed; T and energy are distinct types.".into(),
            "The classical ideal gas and Dulong–Petit both fail the third law; Einstein and Debye hold it.".into(),
            "`thermo.debye-t3` fails for Einstein (exponential) and holds for Debye (phonon continuum).".into(),
        ],
        &thermo_rows(),
        vec![
            Box::new(IdealGas::default()),
            Box::new(EinsteinSolid::dulong_petit()),
            Box::new(EinsteinSolid::einstein()),
            Box::new(EinsteinSolid::debye()),
        ],
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
    fn equipartition_gives_three_halves_nk() {
        let g = IdealGas::default();
        let cv_over_nk = g.heat_capacity() / (g.particles * k_boltzmann().value());
        assert!((cv_over_nk - 1.5).abs() < 1e-6);
        assert_eq!(verdict(&g, EQUIPARTITION), VerdictKind::Holds);
    }

    #[test]
    fn second_law_is_knob_sensitive() {
        let mut g = IdealGas::default(); // V_f/V_i = 2 (expansion)
        assert_eq!(verdict(&g, SECOND_LAW), VerdictKind::Holds);
        g.set("volume_ratio", KnobValue::Float(0.5)).unwrap();
        assert_eq!(verdict(&g, SECOND_LAW), VerdictKind::Fails);
    }

    #[test]
    fn classical_ideal_gas_violates_the_third_law() {
        let g = IdealGas::default();
        assert_eq!(verdict(&g, THIRD_LAW), VerdictKind::Fails);
    }

    #[test]
    fn thermo_experiment_builds_a_matrix() {
        let r = thermodynamics();
        assert_eq!(r.id, "thermo");
        assert_eq!(r.theories.len(), 4);
        assert_eq!(
            r.matrix
                .get(THIRD_LAW)
                .and_then(|m| m.get("ideal-gas"))
                .copied(),
            Some(VerdictKind::Fails)
        );
        assert_eq!(
            r.matrix
                .get(THIRD_LAW)
                .and_then(|m| m.get("einstein-solid"))
                .copied(),
            Some(VerdictKind::Holds)
        );
        assert_eq!(
            r.matrix
                .get(THIRD_LAW)
                .and_then(|m| m.get("debye-solid"))
                .copied(),
            Some(VerdictKind::Holds)
        );
        assert_eq!(
            r.matrix
                .get(DULONG_PETIT)
                .and_then(|m| m.get("dulong-petit"))
                .copied(),
            Some(VerdictKind::Holds)
        );
        assert_eq!(
            r.matrix
                .get(DEBYE_T3)
                .and_then(|m| m.get("debye-solid"))
                .copied(),
            Some(VerdictKind::Holds)
        );
        assert_eq!(
            r.matrix
                .get(DEBYE_T3)
                .and_then(|m| m.get("einstein-solid"))
                .copied(),
            Some(VerdictKind::Fails)
        );
    }
}
