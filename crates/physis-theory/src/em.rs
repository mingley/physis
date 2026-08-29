//! Electromagnetism: the workspace's second scientific domain.
//!
//! This exists to prove `physis` is not a string-theory toy — the same typed
//! substrate (quantities, layers, knobs, claims, verdicts, the `Theory` trait,
//! and the experiment machinery) hosts classical electromagnetism without
//! forking the core. See `specs/007-reuse-domains.md` and
//! `specs/008-electromagnetism.md`.
//!
//! The flagship claim is mechanical, not a table: in vacuum the wave speed is
//! `1/√(ε₀μ₀) = c` (checked in `physis_model::constants`). In a linear medium
//! the refractive index `n = √(ε_r μ_r)` slows light below `c` and selects a
//! rest frame, so the wave-speed and Lorentz-invariance claims flip.

use physis_core::claim::{Claim, Epistemic, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_model::constants::{epsilon0, mu0, C};
use physis_model::{GaugeGroup, Manifold, SimpleGroup, Species, Spectrum, World};

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;

/// EM waves in vacuum travel at c (`1/√(ε₀μ₀)`).
pub const WAVE_SPEED_C: &str = "em.wave-speed-c";
/// Gauss's law for the electric field.
pub const GAUSS: &str = "em.gauss";
/// Faraday's law of induction.
pub const FARADAY: &str = "em.faraday";
/// Ampère–Maxwell law.
pub const AMPERE: &str = "em.ampere";
/// Local charge conservation (the continuity equation).
pub const CHARGE_CONSERVATION: &str = "em.charge-conservation";
/// Lorentz (boost) invariance of the field equations.
pub const LORENTZ_INVARIANCE: &str = "em.lorentz-invariance";

/// Matrix rows for the electromagnetism lab.
pub fn em_rows() -> [&'static str; 6] {
    [
        WAVE_SPEED_C,
        GAUSS,
        FARADAY,
        AMPERE,
        CHARGE_CONSERVATION,
        LORENTZ_INVARIANCE,
    ]
}

fn refractive_index(epsilon_r: f64, mu_r: f64) -> f64 {
    (epsilon_r * mu_r).sqrt()
}

fn is_vacuum(epsilon_r: f64, mu_r: f64) -> bool {
    (refractive_index(epsilon_r, mu_r) - 1.0).abs() < 1e-9
}

fn em_claims() -> Vec<Claim> {
    vec![
        Claim::new(
            WAVE_SPEED_C,
            "Electromagnetic waves propagate at c.",
            LayerId::Field,
            Epistemic::Theorem,
        ),
        Claim::new(
            GAUSS,
            "Gauss's law relates flux to enclosed charge.",
            LayerId::Field,
            Epistemic::EncodedFact,
        ),
        Claim::new(
            FARADAY,
            "A changing magnetic field induces an electric field.",
            LayerId::Field,
            Epistemic::EncodedFact,
        ),
        Claim::new(
            AMPERE,
            "Currents and changing electric fields source the magnetic field.",
            LayerId::Field,
            Epistemic::EncodedFact,
        ),
        Claim::new(
            CHARGE_CONSERVATION,
            "Electric charge is locally conserved.",
            LayerId::Field,
            Epistemic::Theorem,
        ),
        Claim::new(
            LORENTZ_INVARIANCE,
            "The field equations are invariant under Lorentz boosts.",
            LayerId::Spacetime,
            Epistemic::Theorem,
        ),
    ]
}

fn eval_em(epsilon_r: f64, mu_r: f64, claim: &Claim) -> Verdict {
    let n = refractive_index(epsilon_r, mu_r);
    match claim.id.0.as_str() {
        WAVE_SPEED_C => {
            if is_vacuum(epsilon_r, mu_r) {
                Verdict::holds(Epistemic::Theorem, "wave speed is 1/√(ε₀μ₀) = c").with_evidence([
                    format!(
                        "ε₀·μ₀·c² = {:.6} (dimensionless, = 1)",
                        epsilon0().value() * mu0().value() * C.value() * C.value()
                    ),
                ])
            } else {
                Verdict::fails(
                    Epistemic::EncodedFact,
                    format!("v = c/n with n = {n:.3}; light is slower than c in the medium"),
                )
            }
        }
        GAUSS => Verdict::holds(Epistemic::EncodedFact, "∇·D = ρ_free"),
        FARADAY => Verdict::holds(Epistemic::EncodedFact, "∇×E = −∂B/∂t"),
        AMPERE => Verdict::holds(Epistemic::EncodedFact, "∇×H = J_free + ∂D/∂t"),
        CHARGE_CONSERVATION => Verdict::holds(
            Epistemic::Theorem,
            "∂ρ/∂t + ∇·J = 0 follows from Gauss + Ampère (divergence of the curl)",
        ),
        LORENTZ_INVARIANCE => {
            if is_vacuum(epsilon_r, mu_r) {
                Verdict::holds(
                    Epistemic::Theorem,
                    "vacuum Maxwell equations are invariant under Lorentz boosts",
                )
            } else {
                Verdict::fails(
                    Epistemic::EncodedFact,
                    "a material medium selects a rest frame, breaking boost invariance",
                )
            }
        }
        _ => Verdict::inapplicable("claim not made by an electromagnetism object"),
    }
}

fn em_world(epsilon_r: f64, mu_r: f64, note: String) -> World {
    let mut spectrum = Spectrum::empty();
    spectrum.species.push(Species::photon());
    World {
        spacetime: Manifold::observed_4d(),
        gauge: GaugeGroup {
            factors: vec![SimpleGroup::U1],
        },
        spectrum,
        has_gravity: false,
        supersymmetric: false,
        free_parameter_count: 2,
        landscape_log10: 0.0,
        note: format!("{note} (n = {:.3})", refractive_index(epsilon_r, mu_r)),
    }
}

/// Classical electromagnetism in vacuum (the U(1) gauge theory of light).
#[derive(Clone, Debug, Default)]
pub struct MaxwellVacuum;

impl Knobbed for MaxwellVacuum {
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

impl Theory for MaxwellVacuum {
    fn id(&self) -> &'static str {
        "maxwell-vacuum"
    }
    fn name(&self) -> &'static str {
        "Maxwell (vacuum)"
    }
    fn summary(&self) -> &'static str {
        "Classical electromagnetism in vacuum: a U(1) gauge field whose waves \
         travel at 1/√(ε₀μ₀) = c. The workspace's second domain, sharing the \
         same typed substrate as the physics lab."
    }
    fn world(&self) -> World {
        em_world(1.0, 1.0, "Maxwell vacuum".to_string())
    }
    fn claims(&self) -> Vec<Claim> {
        em_claims()
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        eval_em(1.0, 1.0, claim)
    }
}

const MEDIUM_SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "epsilon_r",
        layer: LayerId::Effective,
        doc: "Relative permittivity ε_r of the linear medium (vacuum = 1). Raises the refractive index.",
        domain: KnobDomain::Float {
            min: 1.0,
            max: 1.0e6,
        },
    },
    KnobSpec {
        name: "mu_r",
        layer: LayerId::Effective,
        doc: "Relative permeability μ_r of the linear medium (vacuum = 1). Raises the refractive index.",
        domain: KnobDomain::Float {
            min: 1.0,
            max: 1.0e6,
        },
    },
];

/// Classical electromagnetism in a linear medium (ε_r, μ_r knobs).
#[derive(Clone, Debug)]
pub struct LinearMedium {
    epsilon_r: f64,
    mu_r: f64,
}

impl Default for LinearMedium {
    fn default() -> Self {
        // A glass-like dielectric: n = 1.5, so light is slower than c.
        Self {
            epsilon_r: 2.25,
            mu_r: 1.0,
        }
    }
}

impl Knobbed for LinearMedium {
    fn specs(&self) -> &'static [KnobSpec] {
        MEDIUM_SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "epsilon_r" => Ok(KnobValue::Float(self.epsilon_r)),
            "mu_r" => Ok(KnobValue::Float(self.mu_r)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("epsilon_r", KnobValue::Float(v)) => self.epsilon_r = v,
            ("mu_r", KnobValue::Float(v)) => self.mu_r = v,
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

impl Theory for LinearMedium {
    fn id(&self) -> &'static str {
        "linear-medium"
    }
    fn name(&self) -> &'static str {
        "Maxwell (linear medium)"
    }
    fn summary(&self) -> &'static str {
        "Classical electromagnetism in a linear dielectric/magnetic medium. The \
         refractive index n = √(ε_r μ_r) slows light below c and picks a rest \
         frame, so the wave-speed and Lorentz-invariance claims fail unless the \
         medium is trivial."
    }
    fn world(&self) -> World {
        em_world(
            self.epsilon_r,
            self.mu_r,
            format!("linear medium ε_r={} μ_r={}", self.epsilon_r, self.mu_r),
        )
    }
    fn claims(&self) -> Vec<Claim> {
        em_claims()
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        eval_em(self.epsilon_r, self.mu_r, claim)
    }
}

/// The electromagnetism experiment: Maxwell in vacuum vs a linear medium.
pub fn em_vacuum() -> ExperimentReport {
    let theories: Vec<Box<dyn Theory>> =
        vec![Box::new(MaxwellVacuum), Box::new(LinearMedium::default())];
    report_from_rows(
        "em-vacuum",
        "Electromagnetism lab",
        "Does the same typed substrate that hosts the string-critique lab also \
         host classical electromagnetism — and does the vacuum wave speed come \
         out as a theorem (1/√(ε₀μ₀) = c) that a medium can mechanically break?",
        "Maxwell's equations are encoded facts here; the vacuum wave speed and \
         charge conservation are theorems of the encoding. A linear medium is a \
         knob-controlled effective description, not new fundamental physics.",
        vec![
            "`holds` / `fails` are internal to the encoding.".into(),
            "Vacuum wave speed is a theorem: ε₀·μ₀·c² = 1 (typed, checked).".into(),
            "A medium with n > 1 slows light and selects a rest frame, so wave-speed and Lorentz-invariance fail.".into(),
        ],
        &em_rows(),
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
    fn vacuum_wave_speed_is_a_theorem() {
        let v = MaxwellVacuum;
        assert_eq!(verdict(&v, WAVE_SPEED_C), VerdictKind::Holds);
        assert_eq!(verdict(&v, LORENTZ_INVARIANCE), VerdictKind::Holds);
        assert_eq!(verdict(&v, CHARGE_CONSERVATION), VerdictKind::Holds);
    }

    #[test]
    fn a_medium_slows_light_and_breaks_boosts() {
        let glass = LinearMedium::default();
        assert_eq!(verdict(&glass, WAVE_SPEED_C), VerdictKind::Fails);
        assert_eq!(verdict(&glass, LORENTZ_INVARIANCE), VerdictKind::Fails);
        // Maxwell's laws themselves still hold in the medium.
        assert_eq!(verdict(&glass, GAUSS), VerdictKind::Holds);
    }

    #[test]
    fn permittivity_knob_flips_the_wave_speed_verdict() {
        // The electricity knob → verdict diff required by M3.
        let mut m = LinearMedium::default();
        assert_eq!(verdict(&m, WAVE_SPEED_C), VerdictKind::Fails);
        m.set("epsilon_r", KnobValue::Float(1.0)).unwrap();
        assert_eq!(verdict(&m, WAVE_SPEED_C), VerdictKind::Holds);
        assert_eq!(verdict(&m, LORENTZ_INVARIANCE), VerdictKind::Holds);
    }

    #[test]
    fn em_experiment_builds_a_matrix() {
        let r = em_vacuum();
        assert_eq!(r.id, "em-vacuum");
        assert_eq!(r.theories.len(), 2);
        let wave = r.matrix.get(WAVE_SPEED_C).expect("row");
        assert_eq!(
            wave.get("maxwell-vacuum").copied(),
            Some(VerdictKind::Holds)
        );
        assert_eq!(wave.get("linear-medium").copied(), Some(VerdictKind::Fails));
    }
}
