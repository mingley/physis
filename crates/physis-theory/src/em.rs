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
/// The lumped-element (quasi-static) approximation is valid.
pub const QUASI_STATIC_VALID: &str = "em.quasi-static-valid";

/// Matrix rows for the electromagnetism lab.
pub fn em_rows() -> [&'static str; 7] {
    [
        WAVE_SPEED_C,
        GAUSS,
        FARADAY,
        AMPERE,
        CHARGE_CONSERVATION,
        LORENTZ_INVARIANCE,
        QUASI_STATIC_VALID,
    ]
}

fn refractive_index(epsilon_r: f64, mu_r: f64) -> f64 {
    (epsilon_r * mu_r).sqrt()
}

// A vacuum plane wave travelling along +x in natural units (c = k = ω = 1):
// E = ŷ cos(x − t), B = ẑ cos(x − t). We verify numerically that it solves
// the source-free Maxwell equations, promoting Faraday/Ampère from encoded
// facts to computed theorems for the vacuum.
fn wave_ey(t: f64, x: f64) -> f64 {
    (x - t).cos()
}
fn wave_bz(t: f64, x: f64) -> f64 {
    (x - t).cos()
}

/// Max residual of Faraday's law `∂B/∂t + (∇×E) = 0` over sample points,
/// evaluated by central finite differences on the plane-wave fields.
fn plane_wave_faraday_residual() -> f64 {
    let h = 1e-4;
    let mut max = 0.0_f64;
    for i in 0..8 {
        let t = 0.13 * i as f64;
        let x = 0.29 * i as f64;
        let dbz_dt = (wave_bz(t + h, x) - wave_bz(t - h, x)) / (2.0 * h);
        let dey_dx = (wave_ey(t, x + h) - wave_ey(t, x - h)) / (2.0 * h);
        // (∇×E)_z = ∂E_y/∂x (E_x = 0); Faraday: ∂B_z/∂t + (∇×E)_z = 0.
        max = max.max((dbz_dt + dey_dx).abs());
    }
    max
}

/// Max residual of Gauss's law `∇·E = 0` for a Coulomb field away from its
/// source, by central finite differences. E = r̂/r² = r⃗/r³; its divergence
/// vanishes for r > 0 (all the charge is the delta function at the origin).
fn coulomb_gauss_residual() -> f64 {
    let h = 1e-4;
    let e = |x: f64, y: f64, z: f64| -> [f64; 3] {
        let r3 = (x * x + y * y + z * z).powf(1.5);
        [x / r3, y / r3, z / r3]
    };
    let mut max = 0.0_f64;
    for &(x, y, z) in &[
        (1.0, 2.0, 3.0),
        (2.0, -1.0, 1.0),
        (-1.5, 0.5, 2.0),
        (3.0, 3.0, -2.0),
    ] {
        let dex_dx = (e(x + h, y, z)[0] - e(x - h, y, z)[0]) / (2.0 * h);
        let dey_dy = (e(x, y + h, z)[1] - e(x, y - h, z)[1]) / (2.0 * h);
        let dez_dz = (e(x, y, z + h)[2] - e(x, y, z - h)[2]) / (2.0 * h);
        max = max.max((dex_dx + dey_dy + dez_dz).abs());
    }
    max
}

/// Max residual of Ampère's law `∂E/∂t − (∇×B) = 0` (sourceless vacuum) over
/// sample points, by central finite differences on the plane-wave fields.
fn plane_wave_ampere_residual() -> f64 {
    let h = 1e-4;
    let mut max = 0.0_f64;
    for i in 0..8 {
        let t = 0.13 * i as f64;
        let x = 0.29 * i as f64;
        let dey_dt = (wave_ey(t + h, x) - wave_ey(t - h, x)) / (2.0 * h);
        let dbz_dx = (wave_bz(t, x + h) - wave_bz(t, x - h)) / (2.0 * h);
        // (∇×B)_y = −∂B_z/∂x; Ampère: ∂E_y/∂t − (∇×B)_y = ∂E_y/∂t + ∂B_z/∂x = 0.
        max = max.max((dey_dt + dbz_dx).abs());
    }
    max
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
        Claim::new(
            QUASI_STATIC_VALID,
            "The lumped-element (quasi-static) approximation is valid.",
            LayerId::Effective,
            Epistemic::EncodedFact,
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
        GAUSS => {
            if is_vacuum(epsilon_r, mu_r) {
                let r = coulomb_gauss_residual();
                Verdict::holds(Epistemic::Theorem, "∇·E = 0 in vacuum away from charges")
                    .with_evidence([format!(
                        "verified numerically on a Coulomb field: max |∇·E| = {r:.1e}"
                    )])
            } else {
                Verdict::holds(Epistemic::EncodedFact, "∇·D = ρ_free (macroscopic form)")
            }
        }
        FARADAY => {
            if is_vacuum(epsilon_r, mu_r) {
                let r = plane_wave_faraday_residual();
                Verdict::holds(Epistemic::Theorem, "∇×E = −∂B/∂t").with_evidence([format!(
                    "verified numerically on a vacuum plane wave: max residual {r:.1e}"
                )])
            } else {
                Verdict::holds(
                    Epistemic::EncodedFact,
                    "∇×E = −∂B/∂t (macroscopic form in the medium)",
                )
            }
        }
        AMPERE => {
            if is_vacuum(epsilon_r, mu_r) {
                let r = plane_wave_ampere_residual();
                Verdict::holds(Epistemic::Theorem, "∇×B = ∂E/∂t (sourceless)").with_evidence([
                    format!("verified numerically on a vacuum plane wave: max residual {r:.1e}"),
                ])
            } else {
                Verdict::holds(
                    Epistemic::EncodedFact,
                    "∇×H = J_free + ∂D/∂t (macroscopic form in the medium)",
                )
            }
        }
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
        QUASI_STATIC_VALID => {
            Verdict::inapplicable("full Maxwell theory, not a lumped-element approximation")
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
    fn world(&self) -> Option<World> {
        Some(em_world(1.0, 1.0, "Maxwell vacuum".to_string()))
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
    fn world(&self) -> Option<World> {
        Some(em_world(
            self.epsilon_r,
            self.mu_r,
            format!("linear medium ε_r={} μ_r={}", self.epsilon_r, self.mu_r),
        ))
    }
    fn claims(&self) -> Vec<Claim> {
        em_claims()
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        eval_em(self.epsilon_r, self.mu_r, claim)
    }
}

/// Characteristic size of the modelled circuit, in metres.
const CIRCUIT_SIZE_M: f64 = 0.1;
/// The lumped approximation needs the wavelength to dwarf the circuit.
const QUASI_STATIC_MARGIN: f64 = 100.0;

const OHM_SPECS: &[KnobSpec] = &[KnobSpec {
    name: "frequency_hz",
    layer: LayerId::Effective,
    doc: "Operating frequency in Hz. The lumped approximation holds while the wavelength c/f dwarfs the circuit; at high frequency it breaks down.",
    domain: KnobDomain::Float {
        min: 0.0,
        max: 1.0e18,
    },
}];

/// Ohm's-law lumped circuit theory: the quasi-static effective limit of Maxwell.
#[derive(Clone, Debug)]
pub struct OhmCircuit {
    frequency_hz: f64,
}

impl Default for OhmCircuit {
    fn default() -> Self {
        // 1 kHz: comfortably quasi-static for a 0.1 m circuit.
        Self {
            frequency_hz: 1.0e3,
        }
    }
}

impl OhmCircuit {
    /// Wavelength c/f as a typed length (infinite at DC).
    fn wavelength(&self) -> physis_core::Qty<physis_core::Length> {
        let lambda = if self.frequency_hz <= 0.0 {
            f64::INFINITY
        } else {
            C.value() / self.frequency_hz
        };
        physis_core::qty::meters(lambda)
    }

    fn quasi_static_valid(&self) -> bool {
        self.wavelength().value() > QUASI_STATIC_MARGIN * CIRCUIT_SIZE_M
    }
}

impl Knobbed for OhmCircuit {
    fn specs(&self) -> &'static [KnobSpec] {
        OHM_SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "frequency_hz" => Ok(KnobValue::Float(self.frequency_hz)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("frequency_hz", KnobValue::Float(v)) => self.frequency_hz = v,
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

impl Theory for OhmCircuit {
    fn id(&self) -> &'static str {
        "ohm-circuit"
    }
    fn name(&self) -> &'static str {
        "Ohm circuit (lumped)"
    }
    fn summary(&self) -> &'static str {
        "Lumped-element circuit theory: the quasi-static, long-wavelength limit \
         of Maxwell. Kirchhoff's laws are charge conservation and Faraday's law \
         in disguise; wave propagation is dropped and there is a preferred rest \
         frame. Valid only while the wavelength dwarfs the circuit."
    }
    fn world(&self) -> Option<World> {
        Some(World {
            spacetime: Manifold::observed_4d(),
            gauge: GaugeGroup {
                factors: vec![SimpleGroup::U1],
            },
            spectrum: Spectrum::empty(),
            has_gravity: false,
            supersymmetric: false,
            free_parameter_count: 3,
            landscape_log10: 0.0,
            note: format!(
                "lumped circuit at {:.3e} Hz, wavelength {}",
                self.frequency_hz,
                self.wavelength()
            ),
        })
    }
    fn claims(&self) -> Vec<Claim> {
        em_claims()
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id.0.as_str() {
            WAVE_SPEED_C => Verdict::inapplicable(
                "lumped circuits are the quasi-static limit; wave propagation is dropped",
            ),
            GAUSS => Verdict::holds(
                Epistemic::EncodedFact,
                "capacitor charge Q = CV is Gauss's law in the lumped limit",
            ),
            FARADAY => Verdict::holds(
                Epistemic::EncodedFact,
                "inductor EMF / Kirchhoff's voltage law is Faraday's law",
            ),
            AMPERE => Verdict::holds(
                Epistemic::EncodedFact,
                "displacement current shows up as capacitor current",
            ),
            CHARGE_CONSERVATION => Verdict::holds(
                Epistemic::Theorem,
                "Kirchhoff's current law is exactly charge conservation",
            ),
            LORENTZ_INVARIANCE => Verdict::fails(
                Epistemic::EncodedFact,
                "quasi-static circuit theory has a preferred (lab) rest frame",
            ),
            QUASI_STATIC_VALID => {
                if self.quasi_static_valid() {
                    Verdict::holds(
                        Epistemic::EncodedFact,
                        format!(
                            "wavelength {} dwarfs the {CIRCUIT_SIZE_M} m circuit",
                            self.wavelength()
                        ),
                    )
                } else {
                    Verdict::fails(
                        Epistemic::EncodedFact,
                        format!(
                            "wavelength {} is comparable to the {CIRCUIT_SIZE_M} m circuit; lumped model breaks down",
                            self.wavelength()
                        ),
                    )
                }
            }
            _ => Verdict::inapplicable("claim not made by an electromagnetism object"),
        }
    }
}

/// The electromagnetism experiment: Maxwell in vacuum vs a linear medium vs the
/// lumped-circuit effective theory.
pub fn em_vacuum() -> ExperimentReport {
    let theories: Vec<Box<dyn Theory>> = vec![
        Box::new(MaxwellVacuum),
        Box::new(LinearMedium::default()),
        Box::new(OhmCircuit::default()),
    ];
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
    fn plane_wave_solves_vacuum_maxwell() {
        // The homogeneous Maxwell equations are verified numerically, so
        // Faraday/Ampère are computed theorems in vacuum.
        assert!(
            plane_wave_faraday_residual() < 1e-6,
            "faraday residual {}",
            plane_wave_faraday_residual()
        );
        assert!(
            plane_wave_ampere_residual() < 1e-6,
            "ampere residual {}",
            plane_wave_ampere_residual()
        );
        let v = MaxwellVacuum;
        let faraday = v.claims().into_iter().find(|c| c.id.0 == FARADAY).unwrap();
        assert_eq!(v.evaluate(&faraday).epistemic, Epistemic::Theorem);
    }

    #[test]
    fn gauss_law_verified_on_a_coulomb_field() {
        assert!(
            coulomb_gauss_residual() < 1e-4,
            "gauss residual {}",
            coulomb_gauss_residual()
        );
        let v = MaxwellVacuum;
        let gauss = v.claims().into_iter().find(|c| c.id.0 == GAUSS).unwrap();
        assert_eq!(v.evaluate(&gauss).epistemic, Epistemic::Theorem);
        // In a medium, Gauss stays an encoded fact (macroscopic form).
        let glass = LinearMedium::default();
        let gauss_m = glass
            .claims()
            .into_iter()
            .find(|c| c.id.0 == GAUSS)
            .unwrap();
        assert_eq!(glass.evaluate(&gauss_m).epistemic, Epistemic::EncodedFact);
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
        assert_eq!(r.theories.len(), 3);
        let wave = r.matrix.get(WAVE_SPEED_C).expect("row");
        assert_eq!(
            wave.get("maxwell-vacuum").copied(),
            Some(VerdictKind::Holds)
        );
        assert_eq!(wave.get("linear-medium").copied(), Some(VerdictKind::Fails));
        // The lumped circuit is the quasi-static limit: no wave propagation.
        assert_eq!(
            wave.get("ohm-circuit").copied(),
            Some(VerdictKind::Inapplicable)
        );
    }

    #[test]
    fn ohm_circuit_is_the_quasi_static_limit() {
        let c = OhmCircuit::default();
        assert_eq!(verdict(&c, WAVE_SPEED_C), VerdictKind::Inapplicable);
        assert_eq!(verdict(&c, LORENTZ_INVARIANCE), VerdictKind::Fails);
        assert_eq!(verdict(&c, CHARGE_CONSERVATION), VerdictKind::Holds);
        assert_eq!(verdict(&c, QUASI_STATIC_VALID), VerdictKind::Holds);
    }

    #[test]
    fn high_frequency_breaks_the_lumped_approximation() {
        // The circuit knob → verdict diff.
        let mut c = OhmCircuit::default();
        assert_eq!(verdict(&c, QUASI_STATIC_VALID), VerdictKind::Holds);
        c.set("frequency_hz", KnobValue::Float(1.0e10)).unwrap();
        assert_eq!(verdict(&c, QUASI_STATIC_VALID), VerdictKind::Fails);
    }
}
