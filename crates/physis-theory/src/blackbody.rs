//! Blackbody radiation: Rayleigh–Jeans on trial, Planck as the resolution.
//!
//! Classical statistical mechanics applied to the electromagnetic modes of a
//! cavity is a long-standing 19th-century theory (Rayleigh 1900, Jeans 1905).
//! Equipartition assigns every mode energy `kT`. The spectral density
//! `u(ν) = 8πν² kT / c³` then has no peak, the energy in a finite cavity
//! diverges as the ultraviolet cutoff is removed (**ultraviolet catastrophe**),
//! and the integrated energy at fixed bandwidth is linear in `T` rather than
//! `T⁴`.
//!
//! Planck's law replaces the mode energy with the Bose mean
//! `hν / (e^{hν/kT} − 1)`. The same typed integrals then yield a finite energy
//! density `u = a T⁴`, Wien's displacement peak, and the Rayleigh–Jeans
//! spectrum as the *infrared* limit `hν ≪ kT`.
//!
//! Two lab objects share this encoding:
//! - [`Blackbody::rayleigh_jeans`] — the standing classical theory
//! - [`Blackbody::planck`] — the 1900 resolution
//!
//! The Planck–Bose occupation lives on the IR package of `planck`.
//! Wien's 1896 law (`add-wien`) is a package mutation, not the
//! `quantum` knob: the infrared Rayleigh–Jeans correspondence fails
//! on that fork. `quantum` still switches Bose occupation to
//! classical `kT`. `rayleigh-jeans` has no package.
//!
//! A `quantum` knob on either object is the revolution as a mechanical turn:
//! `set planck quantum false` restores the catastrophe.

use std::f64::consts::PI;

use physis_core::assumption::DomainOfValidity;
use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::qty::{hertz, kelvin, Qty};
use physis_core::ParameterOrigin;
use physis_core::{Energy, EnergyDensity, Frequency, LengthTemperature, SpectralEnergyDensity};
use physis_ir::{apply_mutation, parse_package, render_package, PackageMutation, TheoryPackage};
use physis_model::constants::{k_boltzmann, planck_energy_density, planck_h, C};
use physis_model::{GaugeGroup, Manifold, SimpleGroup, Species, Spectrum, World};

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;

/// Every cavity mode has mean energy kT (classical equipartition).
pub const MODE_EQUIPARTITION: &str = "thermo.mode-equipartition";
/// The cavity energy density stays finite as the ultraviolet cutoff is raised.
pub const UV_FINITE: &str = "thermo.uv-finite";
/// Integrated energy density scales as T⁴ (Stefan–Boltzmann).
pub const STEFAN_BOLTZMANN: &str = "thermo.stefan-boltzmann";
/// The spectrum has a finite peak and λ_max T is constant (Wien).
pub const WIEN_DISPLACEMENT: &str = "thermo.wien-displacement";
/// In the infrared (hν ≪ kT) the spectrum agrees with Rayleigh–Jeans.
pub const RJ_IR_LIMIT: &str = "thermo.rj-ir-limit";

/// Matrix rows for the blackbody lab.
pub fn blackbody_rows() -> [&'static str; 5] {
    [
        MODE_EQUIPARTITION,
        UV_FINITE,
        STEFAN_BOLTZMANN,
        WIEN_DISPLACEMENT,
        RJ_IR_LIMIT,
    ]
}

/// Default cavity temperature (kelvin): a hot laboratory blackbody.
const DEFAULT_T_K: f64 = 5000.0;
/// Default ultraviolet cutoff (Hz). At 5000 K, kT/h ≈ 1.04×10¹⁴ Hz, so this
/// is ~100 thermal frequencies — enough for Planck to converge.
const DEFAULT_CUTOFF_HZ: f64 = 1.0e16;
/// Dimensionless frequency beyond which the Bose tail is negligible.
const PLANCK_X_TAIL: f64 = 40.0;
/// High-frequency probe of equipartition, in units of kT/h.
const UV_MODE_X: f64 = 8.0;
/// Infrared probe of the Rayleigh–Jeans correspondence, in units of kT/h.
const IR_MODE_X: f64 = 0.01;
/// Exact Bose occupation on the live Planck package.
const MODE_PLANCK_BOSE: &str = "mode planck-bose";
/// Wien 1896 occupation ⟨E⟩ = hν e^{−hν/kT}.
const MODE_WIEN: &str = "mode wien";

fn rj_ir_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["hν = 0.01 kT infrared probe".into()],
        vec!["|u − u_RJ|/u_RJ < 1%".into()],
        "Correspondence is the infrared, not the ultraviolet catastrophe. \
         Using Planck at hν ≫ kT as if it were Rayleigh–Jeans is a new claim.",
    )
}

fn parse_planck_mode(pkg: &TheoryPackage) -> Result<bool, String> {
    let mut bose = false;
    let mut wien = false;
    for eq in &pkg.equations {
        match eq.trim() {
            MODE_PLANCK_BOSE => bose = true,
            MODE_WIEN => wien = true,
            _ => {}
        }
    }
    if !bose {
        return Err(format!("{} package has no Planck-Bose occupation", pkg.id));
    }
    Ok(wien)
}

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "quantum",
        layer: LayerId::Quantum,
        doc: "If true, cavity modes are Bose-occupied (Planck). If false, every mode has energy kT (Rayleigh–Jeans). Turning this off is the 19th-century theory, and it produces the ultraviolet catastrophe.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Bool,
    },
    KnobSpec {
        name: "temperature",
        layer: LayerId::Statistical,
        doc: "Cavity temperature in kelvin.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 1.0,
            max: 1.0e7,
        },
    },
    KnobSpec {
        name: "cutoff_hz",
        layer: LayerId::Effective,
        doc: "Ultraviolet frequency cutoff of the cavity, in hertz. Classical energy density grows as ν_max³; Planck saturates once ν_max ≫ kT/h.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 1.0e8,
            max: 1.0e20,
        },
    },
];

/// Electromagnetic cavity radiation: Rayleigh–Jeans or Planck.
///
/// Bose occupation lives on the `planck` IR package. Truncated Wien
/// occupation (`add-wien`) is a package mutation, not the `quantum`
/// knob: the infrared Rayleigh–Jeans correspondence fails.
#[derive(Clone, Debug, PartialEq)]
pub struct Blackbody {
    /// Lab id. Fixed at construction; not a knob.
    id: &'static str,
    /// Planck (true) or Rayleigh–Jeans (false).
    quantum: bool,
    temperature_k: f64,
    cutoff_hz: f64,
    /// Whether the encoding uses Wien ⟨E⟩ = hν e^{−x} instead of Bose.
    wien: bool,
}

impl Default for Blackbody {
    fn default() -> Self {
        Self::planck()
    }
}

impl Blackbody {
    /// Planck's law: Bose occupation of cavity modes. The 1900 resolution.
    pub fn planck() -> Self {
        Self {
            id: "planck",
            quantum: true,
            temperature_k: DEFAULT_T_K,
            cutoff_hz: DEFAULT_CUTOFF_HZ,
            wien: false,
        }
    }

    /// Rayleigh–Jeans: classical equipartition of cavity modes.
    ///
    /// The standing 19th-century theory. It holds `thermo.mode-equipartition`
    /// by construction and fails the observational claims (finite energy,
    /// Stefan–Boltzmann T⁴, Wien peak).
    pub fn rayleigh_jeans() -> Self {
        Self {
            id: "rayleigh-jeans",
            quantum: false,
            temperature_k: DEFAULT_T_K,
            cutoff_hz: DEFAULT_CUTOFF_HZ,
            wien: false,
        }
    }

    fn thermal_frequency(&self) -> Qty<Frequency> {
        k_boltzmann() * kelvin(self.temperature_k) / planck_h()
    }

    fn wien_occupation(&self) -> bool {
        self.wien && self.quantum
    }

    /// IR package for the Planck object. Equations are `mode planck-bose`
    /// and, when forked, `mode wien`. `quantum` stays on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![MODE_PLANCK_BOSE.to_string()];
        if self.wien {
            equations.push(MODE_WIEN.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["cavity-modes".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: RJ_IR_LIMIT.into(),
                statement: "In the infrared hν ≪ kT the spectrum agrees with Rayleigh–Jeans."
                    .into(),
                layer: "statistical".into(),
                class: "model-internal".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load a Bose encoding from a package. Knobs default; overlay them
    /// from a live Planck object when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        if pkg.id != "planck" {
            return Err(format!("planck package id '{}' is not planck", pkg.id));
        }
        let wien = parse_planck_mode(pkg)?;
        Ok(Self {
            wien,
            ..Self::planck()
        })
    }

    fn wien_equation() -> String {
        MODE_WIEN.to_string()
    }

    fn mean_mode_energy(&self, nu: Qty<Frequency>) -> Qty<Energy> {
        let kt: Qty<Energy> = k_boltzmann() * kelvin(self.temperature_k);
        if !self.quantum {
            return kt;
        }
        let hnu: Qty<Energy> = planck_h() * nu;
        let x = hnu.value() / kt.value();
        if self.wien {
            Qty::new(hnu.value() * (-x).exp())
        } else if x < 1.0e-10 {
            kt
        } else {
            Qty::new(hnu.value() / (x.exp() - 1.0))
        }
    }

    /// Spectral energy density u(ν) (J m⁻³ Hz⁻¹), typed.
    fn spectral_u_nu(&self, nu: Qty<Frequency>) -> Qty<SpectralEnergyDensity> {
        let mode_density = (nu * nu) / (C * C * C) * (8.0 * PI);
        mode_density * self.mean_mode_energy(nu)
    }

    /// Wavelength-basis spectral density u(λ) = u(ν)·c/λ², untyped payload
    /// used only to locate the Wien peak.
    fn u_lambda(&self, lambda_m: f64) -> f64 {
        if lambda_m <= 0.0 {
            return 0.0;
        }
        let nu = C.value() / lambda_m;
        self.spectral_u_nu(hertz(nu)).value() * C.value() / (lambda_m * lambda_m)
    }

    fn energy_density_to(&self, nu_max: f64) -> Qty<EnergyDensity> {
        if self.wien_occupation() {
            self.wien_u_to(nu_max)
        } else if self.quantum {
            self.planck_u_to(nu_max)
        } else {
            self.rayleigh_jeans_u_to(nu_max)
        }
    }

    fn rayleigh_jeans_u_to(&self, nu_max: f64) -> Qty<EnergyDensity> {
        let kt: Qty<Energy> = k_boltzmann() * kelvin(self.temperature_k);
        let nu = hertz(nu_max);
        let nu3 = nu * nu * nu;
        kt * nu3 / (C * C * C) * (8.0 * PI / 3.0)
    }

    fn planck_u_to(&self, nu_max: f64) -> Qty<EnergyDensity> {
        let nu_t = self.thermal_frequency().value();
        let xmax = (nu_max / nu_t).min(PLANCK_X_TAIL);
        let integral = integrate_bose_x3(xmax);
        let nu = hertz(nu_t);
        let nu4 = nu * nu * nu * nu;
        planck_h() * nu4 / (C * C * C) * (8.0 * PI * integral)
    }

    fn wien_u_to(&self, nu_max: f64) -> Qty<EnergyDensity> {
        let nu_t = self.thermal_frequency().value();
        let xmax = (nu_max / nu_t).min(PLANCK_X_TAIL);
        let integral = integrate_wien_x3(xmax);
        let nu = hertz(nu_t);
        let nu4 = nu * nu * nu * nu;
        planck_h() * nu4 / (C * C * C) * (8.0 * PI * integral)
    }

    /// Full Planck energy density (improper integral, independent of cutoff).
    fn full_planck_u(&self) -> Qty<EnergyDensity> {
        self.planck_u_to(f64::INFINITY)
    }

    fn full_wien_u(&self) -> Qty<EnergyDensity> {
        self.wien_u_to(f64::INFINITY)
    }

    /// Log-spaced samples of u(λ) from the deep UV to the IR, in metres.
    fn u_lambda_samples(&self) -> (f64, f64, Vec<(f64, f64)>) {
        let kt = k_boltzmann().value() * self.temperature_k;
        let hc = planck_h().value() * C.value();
        let lam_uv = hc / (30.0 * kt);
        let lam_ir = hc / (0.2 * kt);
        let n = 48_usize;
        let mut pts = Vec::with_capacity(n + 1);
        for i in 0..=n {
            let t = i as f64 / n as f64;
            let l = (lam_uv.ln() + t * (lam_ir.ln() - lam_uv.ln())).exp();
            pts.push((l, self.u_lambda(l)));
        }
        (lam_uv, lam_ir, pts)
    }

    /// Wavelength of an *interior* u(λ) peak, if the sampled spectrum has one.
    ///
    /// Classical Rayleigh–Jeans `u(λ) ∝ λ⁻⁴` is monotonic and maxes at the UV
    /// endpoint of any finite window — that is a computed absence of a peak,
    /// not a prose assertion.
    fn wien_peak_lambda_m(&self) -> Option<f64> {
        let (lam_uv, lam_ir, pts) = self.u_lambda_samples();
        let (best_l, _) = pts
            .iter()
            .copied()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))?;
        // A peak at either endpoint is not Wien's displacement: it is a
        // window artefact (or a UV divergence).
        if (best_l - lam_uv).abs() / lam_uv < 0.25 || (best_l - lam_ir).abs() / lam_ir < 0.25 {
            return None;
        }
        let mut lo = (best_l * 0.5).max(lam_uv).ln();
        let mut hi = (best_l * 2.0).min(lam_ir).ln();
        for _ in 0..60 {
            let m1 = lo + (hi - lo) / 3.0;
            let m2 = hi - (hi - lo) / 3.0;
            if self.u_lambda(m1.exp()) < self.u_lambda(m2.exp()) {
                lo = m1;
            } else {
                hi = m2;
            }
        }
        Some(((lo + hi) * 0.5).exp())
    }
}

/// ∫_0^xmax x³ / (e^x − 1) dx by trapezoidal rule.
fn integrate_bose_x3(xmax: f64) -> f64 {
    if xmax <= 0.0 {
        return 0.0;
    }
    let n = 4000_usize;
    let dx = xmax / n as f64;
    let mut s = 0.0;
    for i in 0..=n {
        let x = i as f64 * dx;
        let w = if i == 0 || i == n { 0.5 } else { 1.0 };
        s += w * bose_x3(x);
    }
    s * dx
}

fn bose_x3(x: f64) -> f64 {
    if x <= 0.0 {
        0.0
    } else if x < 1.0e-8 {
        x * x
    } else {
        let ex = x.exp();
        if !ex.is_finite() {
            0.0
        } else {
            x * x * x / (ex - 1.0)
        }
    }
}

/// ∫_0^xmax x³ e^{−x} dx by trapezoidal rule. The improper integral is 6.
fn integrate_wien_x3(xmax: f64) -> f64 {
    if xmax <= 0.0 {
        return 0.0;
    }
    let n = 4000_usize;
    let dx = xmax / n as f64;
    let mut s = 0.0;
    for i in 0..=n {
        let x = i as f64 * dx;
        let w = if i == 0 || i == n { 0.5 } else { 1.0 };
        s += w * wien_x3(x);
    }
    s * dx
}

fn wien_x3(x: f64) -> f64 {
    if x <= 0.0 {
        0.0
    } else {
        x * x * x * (-x).exp()
    }
}

/// Wien root of `x = 5 (1 − e^{-x})`, the peak of u(λ).
fn wien_x_wavelength() -> f64 {
    let mut x = 5.0_f64;
    for _ in 0..40 {
        x = 5.0 * (1.0 - (-x).exp());
    }
    x
}

/// Analytic Wien constant `λ_max T = hc / (k x)`.
fn wien_constant() -> Qty<LengthTemperature> {
    planck_h() * C / (k_boltzmann() * wien_x_wavelength())
}

fn cavity_world(note: String) -> World {
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
        free_parameter_count: 0,
        landscape_log10: 0.0,
        note,
    }
}

impl Knobbed for Blackbody {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "quantum" => Ok(KnobValue::Bool(self.quantum)),
            "temperature" => Ok(KnobValue::Float(self.temperature_k)),
            "cutoff_hz" => Ok(KnobValue::Float(self.cutoff_hz)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("quantum", KnobValue::Bool(v)) => self.quantum = v,
            ("temperature", KnobValue::Float(v)) => self.temperature_k = v,
            ("cutoff_hz", KnobValue::Float(v)) => self.cutoff_hz = v,
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

impl Theory for Blackbody {
    fn id(&self) -> &'static str {
        self.id
    }
    fn name(&self) -> &'static str {
        if self.wien_occupation() {
            "Wien's law (cavity radiation)"
        } else if self.quantum {
            "Planck's law (cavity radiation)"
        } else {
            "Rayleigh–Jeans law (cavity radiation)"
        }
    }
    fn summary(&self) -> &'static str {
        "Electromagnetic energy in a thermal cavity. Classical equipartition of \
         modes (Rayleigh–Jeans) produces the ultraviolet catastrophe and the \
         wrong T-dependence; Planck's Bose occupation yields finite u = a T⁴ \
         and Wien's peak. The quantum knob is the 1900 revolution, mechanized. \
         Wien 1896 occupation is an IR mutation on planck, not that knob."
    }
    fn world(&self) -> Option<World> {
        Some(cavity_world(format!(
            "{} cavity at T = {} K, ν_max = {:.3e} Hz",
            if self.wien_occupation() {
                "Wien"
            } else if self.quantum {
                "Planck"
            } else {
                "Rayleigh–Jeans"
            },
            self.temperature_k,
            self.cutoff_hz
        )))
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                MODE_EQUIPARTITION,
                "Every cavity mode has mean energy kT (classical equipartition).",
                LayerId::Statistical,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                UV_FINITE,
                "The improper integral ∫_0^∞ u(ν) dν converges to a finite energy density.",
                LayerId::Statistical,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                STEFAN_BOLTZMANN,
                "The integrated energy density scales as T⁴ (Stefan–Boltzmann).",
                LayerId::Statistical,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                WIEN_DISPLACEMENT,
                "The spectrum has a finite peak and λ_max T is constant (Wien's displacement law).",
                LayerId::Statistical,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                RJ_IR_LIMIT,
                "In the infrared hν ≪ kT the spectrum agrees with Rayleigh–Jeans.",
                LayerId::Statistical,
                ClaimClass::ModelInternal,
            )
            .with_domain(rj_ir_domain()),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id_str() {
            MODE_EQUIPARTITION => {
                let nu = hertz(self.thermal_frequency().value() * UV_MODE_X);
                let ratio = self.mean_mode_energy(nu).value()
                    / (k_boltzmann().value() * self.temperature_k);
                if (ratio - 1.0).abs() < 0.05 {
                    Verdict::holds(
                        claim,
                        "a UV mode still carries kT (classical equipartition)",
                    )
                    .with_evidence([format!("<E>/(kT) = {ratio:.4} at hν = {UV_MODE_X} kT")])
                } else {
                    let reason = if self.wien_occupation() {
                        "UV modes freeze out: <E> = hν e^{-hν/kT} ≪ kT"
                    } else {
                        "UV modes freeze out: <E> = hν/(e^{hν/kT}−1) ≪ kT"
                    };
                    Verdict::fails(claim, reason).with_evidence([format!(
                        "<E>/(kT) = {ratio:.4e} at hν = {UV_MODE_X} kT (not 1)"
                    )])
                }
            }
            UV_FINITE => {
                let u1 = self.energy_density_to(self.cutoff_hz);
                let u2 = self.energy_density_to(self.cutoff_hz * 2.0);
                let doubling = u2.value() / u1.value();
                if self.wien_occupation() {
                    let full = self.full_wien_u();
                    let nu_t = self.thermal_frequency();
                    let nu4 = nu_t * nu_t * nu_t * nu_t;
                    let analytic = planck_h() * nu4 / (C * C * C) * (8.0 * PI * 6.0);
                    let rel = (full.value() - analytic.value()).abs() / analytic.value();
                    if rel < 1e-3 {
                        Verdict::holds(
                            claim,
                            "∫ u(ν) dν is finite (Wien occupation, independent of cutoff)",
                        )
                        .with_evidence([format!(
                            "u_∞ = {:.4e} J/m³ vs 6·8πh(kT/h)⁴/c³ = {:.4e} (rel {rel:.2e})",
                            full.value(),
                            analytic.value()
                        )])
                    } else {
                        Verdict::fails(claim, "Wien integral disagrees with Γ(4) = 6")
                            .with_evidence([format!(
                                "u_∞ = {:.4e}, analytic = {:.4e}",
                                full.value(),
                                analytic.value()
                            )])
                    }
                } else if self.quantum {
                    // The theorem is convergence of the improper integral, not
                    // saturation of the current cutoff (which may sit in the
                    // Rayleigh–Jeans infrared at high T / low ν_max).
                    let full = self.full_planck_u();
                    let analytic = planck_energy_density(kelvin(self.temperature_k));
                    let rel = (full.value() - analytic.value()).abs() / analytic.value();
                    if rel < 1e-3 {
                        Verdict::holds(claim,
                            "∫ u(ν) dν = a T⁴ is finite (Planck, independent of cutoff)",
                        )
                        .with_evidence([format!(
                            "u_∞ = {:.4e} J/m³ vs aT⁴ = {:.4e} (rel {rel:.2e}); ν_max/(kT/h) = {:.3e}",
                            full.value(),
                            analytic.value(),
                            self.cutoff_hz / self.thermal_frequency().value()
                        )])
                    } else {
                        Verdict::fails(claim, "Planck integral disagrees with analytic a T⁴")
                            .with_evidence([format!(
                                "u_∞ = {:.4e}, aT⁴ = {:.4e}",
                                full.value(),
                                analytic.value()
                            )])
                    }
                } else {
                    Verdict::fails(
                        claim,
                        "ultraviolet catastrophe: the Rayleigh–Jeans integral diverges as ν_max³",
                    )
                    .with_evidence([format!(
                        "u(2ν_max)/u(ν_max) = {doubling:.3} (equipartition predicts 8)"
                    )])
                }
            }
            STEFAN_BOLTZMANN => {
                if self.wien_occupation() {
                    let u_t = self.full_wien_u();
                    let mut hot = self.clone();
                    hot.temperature_k *= 2.0;
                    let u_2t = hot.full_wien_u();
                    let ratio = u_2t.value() / u_t.value();
                    if (ratio - 16.0).abs() / 16.0 < 0.03 {
                        Verdict::holds(claim, "u_∞(2T)/u_∞(T) = 16 = 2⁴ (Wien occupation is T⁴)")
                            .with_evidence([format!("u_∞(2T)/u_∞(T) = {ratio:.3}")])
                    } else {
                        Verdict::fails(claim, "Wien energy density does not scale as T⁴")
                            .with_evidence([format!("u_∞(2T)/u_∞(T) = {ratio:.3} (expected 16)")])
                    }
                } else if self.quantum {
                    let u_t = self.full_planck_u();
                    let mut hot = self.clone();
                    hot.temperature_k *= 2.0;
                    let u_2t = hot.full_planck_u();
                    let ratio = u_2t.value() / u_t.value();
                    let analytic = planck_energy_density(kelvin(self.temperature_k));
                    if (ratio - 16.0).abs() / 16.0 < 0.03 {
                        Verdict::holds(claim, "u_∞(2T)/u_∞(T) = 16 = 2⁴ (Stefan–Boltzmann)")
                            .with_evidence([
                                format!("u_∞(2T)/u_∞(T) = {ratio:.3}"),
                                format!(
                                    "u_∞(T) = {:.4e} J/m³ (analytic aT⁴ = {:.4e})",
                                    u_t.value(),
                                    analytic.value()
                                ),
                            ])
                    } else {
                        Verdict::fails(claim, "Planck energy density does not scale as T⁴")
                            .with_evidence([format!("u_∞(2T)/u_∞(T) = {ratio:.3} (expected 16)")])
                    }
                } else {
                    let u_t = self.energy_density_to(self.cutoff_hz);
                    let mut hot = self.clone();
                    hot.temperature_k *= 2.0;
                    let u_2t = hot.energy_density_to(self.cutoff_hz);
                    let ratio = u_2t.value() / u_t.value();
                    Verdict::fails(
                        claim,
                        "at fixed bandwidth classical u is linear in T, not T⁴",
                    )
                    .with_evidence([format!(
                        "u(2T)/u(T) = {ratio:.3} at fixed ν_max (Stefan–Boltzmann requires 16)"
                    )])
                }
            }
            WIEN_DISPLACEMENT => {
                if self.wien_occupation() {
                    match self.wien_peak_lambda_m() {
                        None => Verdict::fails(
                            claim,
                            "Wien occupation sampled u(λ) has no interior peak",
                        ),
                        Some(lambda) => {
                            let product = lambda * self.temperature_k;
                            Verdict::holds(claim, "Wien occupation still has a spectral peak")
                                .with_evidence([format!(
                                    "λ_max T = {product:.6e} m·K (not the Planck Wien constant)"
                                )])
                        }
                    }
                } else {
                    match self.wien_peak_lambda_m() {
                        None => {
                            let (lam_uv, _, pts) = self.u_lambda_samples();
                            let u_uv = pts.first().map(|p| p.1).unwrap_or(0.0);
                            let u_mid = pts.get(pts.len() / 2).map(|p| p.1).unwrap_or(0.0);
                            Verdict::fails(
                                claim,
                                "sampled u(λ) has no interior peak; the maximum is at the UV endpoint",
                            )
                            .with_evidence([format!(
                                "u(λ_UV = {lam_uv:.3e} m) = {u_uv:.3e} > u(mid-window) = {u_mid:.3e} (monotonic UV rise)"
                            )])
                        }
                        Some(lambda) => {
                            let product = lambda * self.temperature_k;
                            let analytic = wien_constant().value();
                            if (product - analytic).abs() / analytic < 0.02 {
                                Verdict::holds(claim, "λ_max T matches the computed Wien constant")
                                    .with_evidence([format!(
                                        "λ_max T = {product:.6e} m·K (analytic hc/(k x) = {analytic:.6e})"
                                    )])
                            } else {
                                Verdict::fails(
                                    claim,
                                    "computed peak does not match Wien's constant",
                                )
                                .with_evidence([format!(
                                    "λ_max T = {product:.6e}, analytic {analytic:.6e}"
                                )])
                            }
                        }
                    }
                }
            }
            RJ_IR_LIMIT => {
                let nu = hertz(self.thermal_frequency().value() * IR_MODE_X);
                let u = self.spectral_u_nu(nu).value();
                let mut classical = self.clone();
                classical.quantum = false;
                let u_rj = classical.spectral_u_nu(nu).value();
                let rel = (u - u_rj).abs() / u_rj;
                if self.wien_occupation() {
                    Verdict::fails(claim, "Wien occupation: the infrared is not Rayleigh–Jeans")
                        .with_evidence([format!(
                            "|u − u_RJ|/u_RJ = {rel:.3} at hν = {IR_MODE_X} kT"
                        )])
                } else if rel < 0.01 {
                    Verdict::holds(claim, "hν ≪ kT: the spectrum agrees with Rayleigh–Jeans")
                        .with_evidence([format!(
                            "|u − u_RJ|/u_RJ = {rel:.2e} at hν = {IR_MODE_X} kT"
                        )])
                } else {
                    Verdict::fails(claim, "infrared spectrum disagrees with Rayleigh–Jeans")
                        .with_evidence([format!("|u − u_RJ|/u_RJ = {rel:.3e}")])
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by a cavity-radiation object"),
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        if self.id != "planck" {
            return None;
        }
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.wien = parsed.wien;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        if self.id != "planck" || self.wien {
            return Vec::new();
        }
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(Self::wien_equation()),
        );
        if let Ok(parsed) = Self::from_package(&mutated) {
            if parsed.wien {
                let mut fork = self.clone();
                fork.wien = true;
                return vec![("add-wien".into(), Box::new(fork))];
            }
        }
        Vec::new()
    }
}

/// Rayleigh–Jeans vs Planck: classical cavity radiation on trial.
pub fn blackbody() -> ExperimentReport {
    report_from_rows(
        "blackbody",
        "Blackbody radiation lab",
        "Does classical equipartition of electromagnetic cavity modes survive \
         contact with a finite energy density, the T⁴ law, and Wien's peak — \
         or does the standing Rayleigh–Jeans theory fail those theorems, while \
         Planck's Bose occupation holds them?",
        "Verdicts are internal to the encoding. The ultraviolet catastrophe is \
         a computed divergence of the Rayleigh–Jeans integral, not a slogan. \
         Planck's T⁴ law is the same integral with Bose occupation, checked \
         against the typed Stefan–Boltzmann constant derived from h, k_B, c.",
        vec![
            "`thermo.mode-equipartition` is the standing 19th-century claim: it holds for Rayleigh–Jeans and fails for Planck (UV modes freeze out).".into(),
            "`thermo.uv-finite` / `stefan-boltzmann` / `wien-displacement` are the observational theorems classical theory fails.".into(),
            "`thermo.rj-ir-limit` is the correspondence: Planck *contains* Rayleigh–Jeans at hν ≪ kT.".into(),
            "`set planck quantum false` restores the catastrophe — the 1900 revolution as a knob turn.".into(),
            "`add-wien` is an IR mutation on `planck`: truncated Wien occupation fails the infrared Rayleigh–Jeans correspondence.".into(),
        ],
        &blackbody_rows(),
        vec![
            Box::new(Blackbody::rayleigh_jeans()),
            Box::new(Blackbody::planck()),
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
    fn bose_integral_matches_pi4_over_15() {
        let i = integrate_bose_x3(PLANCK_X_TAIL);
        let exact = PI.powi(4) / 15.0;
        assert!(
            (i - exact).abs() / exact < 1e-5,
            "∫ x³/(e^x−1) dx = {i}, π⁴/15 = {exact}"
        );
    }

    #[test]
    fn planck_numeric_u_matches_analytic_a_t4() {
        let b = Blackbody::planck();
        let numeric = b.energy_density_to(b.cutoff_hz);
        let analytic = planck_energy_density(kelvin(b.temperature_k));
        let rel = (numeric.value() - analytic.value()).abs() / analytic.value();
        assert!(
            rel < 1e-4,
            "numeric {} vs aT⁴ {} (rel {rel})",
            numeric.value(),
            analytic.value()
        );
        // Typed: a T⁴ has energy-density dimension.
        let _u: Qty<EnergyDensity> = analytic;
        let _a = physis_model::constants::radiation_density_constant();
    }

    #[test]
    fn rayleigh_jeans_fails_the_observational_theorems() {
        let rj = Blackbody::rayleigh_jeans();
        assert_eq!(verdict(&rj, MODE_EQUIPARTITION), VerdictKind::Holds);
        assert_eq!(verdict(&rj, UV_FINITE), VerdictKind::Fails);
        assert_eq!(verdict(&rj, STEFAN_BOLTZMANN), VerdictKind::Fails);
        assert_eq!(verdict(&rj, WIEN_DISPLACEMENT), VerdictKind::Fails);
        assert_eq!(verdict(&rj, RJ_IR_LIMIT), VerdictKind::Holds);
    }

    #[test]
    fn planck_holds_observations_and_rejects_equipartition() {
        let p = Blackbody::planck();
        assert_eq!(verdict(&p, MODE_EQUIPARTITION), VerdictKind::Fails);
        assert_eq!(verdict(&p, UV_FINITE), VerdictKind::Holds);
        assert_eq!(verdict(&p, STEFAN_BOLTZMANN), VerdictKind::Holds);
        assert_eq!(verdict(&p, WIEN_DISPLACEMENT), VerdictKind::Holds);
        assert_eq!(verdict(&p, RJ_IR_LIMIT), VerdictKind::Holds);
        let ir = p
            .claims()
            .into_iter()
            .find(|c| c.id_str() == RJ_IR_LIMIT)
            .unwrap();
        assert!(
            !ir.domain().is_encoding_wide(),
            "Rayleigh–Jeans correspondence must name hν ≪ kT: {:?}",
            ir.domain()
        );
        assert!(
            ir.domain().regimes.iter().any(|r| r.contains("0.01 kT")),
            "IR regime: {:?}",
            ir.domain()
        );
        let uv = p
            .claims()
            .into_iter()
            .find(|c| c.id_str() == UV_FINITE)
            .unwrap();
        assert!(
            uv.domain().is_encoding_wide(),
            "UV finiteness stays encoding-wide (improper integral, not the IR probe)"
        );
    }

    #[test]
    fn quantum_knob_is_the_1900_revolution() {
        let mut p = Blackbody::planck();
        assert_eq!(verdict(&p, UV_FINITE), VerdictKind::Holds);
        p.set("quantum", KnobValue::Bool(false)).unwrap();
        assert_eq!(verdict(&p, UV_FINITE), VerdictKind::Fails);
        assert_eq!(verdict(&p, STEFAN_BOLTZMANN), VerdictKind::Fails);
        assert_eq!(verdict(&p, WIEN_DISPLACEMENT), VerdictKind::Fails);
        assert_eq!(verdict(&p, MODE_EQUIPARTITION), VerdictKind::Holds);
        // Identity is stable: the object is still `planck` in the lab.
        assert_eq!(p.id(), "planck");
    }

    #[test]
    fn planck_observables_hold_when_cutoff_is_in_the_infrared() {
        // High T and the domain-minimum cutoff put ν_max ≪ kT/h, so a
        // cutoff-doubling test would look classical. The improper-integral
        // statements must still hold.
        let mut p = Blackbody::planck();
        p.set("temperature", KnobValue::Float(1.0e6)).unwrap();
        p.set("cutoff_hz", KnobValue::Float(1.0e8)).unwrap();
        assert_eq!(verdict(&p, UV_FINITE), VerdictKind::Holds);
        assert_eq!(verdict(&p, STEFAN_BOLTZMANN), VerdictKind::Holds);
        assert_eq!(verdict(&p, WIEN_DISPLACEMENT), VerdictKind::Holds);
        assert_eq!(verdict(&p, MODE_EQUIPARTITION), VerdictKind::Fails);
    }

    #[test]
    fn doubling_rj_cutoff_octuples_energy() {
        let rj = Blackbody::rayleigh_jeans();
        let u1 = rj.energy_density_to(rj.cutoff_hz).value();
        let u2 = rj.energy_density_to(rj.cutoff_hz * 2.0).value();
        assert!((u2 / u1 - 8.0).abs() < 1e-9, "ratio = {}", u2 / u1);
    }

    #[test]
    fn wien_product_is_temperature_independent() {
        let mut p = Blackbody::planck();
        let l1 = p.wien_peak_lambda_m().unwrap() * p.temperature_k;
        p.set("temperature", KnobValue::Float(2500.0)).unwrap();
        let l2 = p.wien_peak_lambda_m().unwrap() * p.temperature_k;
        assert!(
            (l1 - l2).abs() / l1 < 0.02,
            "λT at 5000 K = {l1:.6e}, at 2500 K = {l2:.6e}"
        );
        assert_eq!(verdict(&p, WIEN_DISPLACEMENT), VerdictKind::Holds);
    }

    #[test]
    fn blackbody_experiment_puts_the_standing_theory_on_trial() {
        let r = blackbody();
        assert_eq!(r.id, "blackbody");
        assert_eq!(r.theories.len(), 2);
        let cell =
            |claim: &str, theory: &str| r.matrix.get(claim).and_then(|m| m.get(theory)).copied();
        assert_eq!(cell(UV_FINITE, "rayleigh-jeans"), Some(VerdictKind::Fails));
        assert_eq!(cell(UV_FINITE, "planck"), Some(VerdictKind::Holds));
        assert_eq!(
            cell(MODE_EQUIPARTITION, "rayleigh-jeans"),
            Some(VerdictKind::Holds)
        );
        assert_eq!(cell(MODE_EQUIPARTITION, "planck"), Some(VerdictKind::Fails));
    }

    #[test]
    fn h_times_frequency_is_energy() {
        let e: Qty<Energy> = planck_h() * hertz(1.0e14);
        assert!(e.value() > 0.0);
    }

    #[test]
    fn wien_integral_matches_gamma_4() {
        let i = integrate_wien_x3(PLANCK_X_TAIL);
        assert!(
            (i - 6.0).abs() / 6.0 < 1e-4,
            "∫ x³ exp(-x) dx = {i}, expected 6"
        );
    }

    #[test]
    fn wien_occupation_is_ir_not_a_knob() {
        let t = Blackbody::planck();
        assert!(
            Blackbody::planck()
                .set("wien", KnobValue::Bool(true))
                .is_err(),
            "Wien occupation is an IR mutation, not a knob"
        );
        assert!(Blackbody::planck()
            .set("occupation", KnobValue::Bool(true))
            .is_err());
        assert_eq!(
            t.get("quantum").unwrap(),
            KnobValue::Bool(true),
            "quantum stays a knob"
        );
        let src = render_package(&t.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            Blackbody::from_package(&pkg).unwrap(),
            t,
            "IR round-trip must preserve Planck-Bose occupation"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(Blackbody::wien_equation()),
        );
        let parsed = Blackbody::from_package(&mutated).unwrap();
        assert!(parsed.wien);
        let mut fork = t.clone();
        fork.wien = true;
        assert_eq!(fork.id(), "planck");
        assert_eq!(verdict(&fork, RJ_IR_LIMIT), VerdictKind::Fails);
        assert_eq!(verdict(&fork, UV_FINITE), VerdictKind::Holds);
        assert_eq!(verdict(&fork, STEFAN_BOLTZMANN), VerdictKind::Holds);
        assert_eq!(verdict(&fork, WIEN_DISPLACEMENT), VerdictKind::Holds);
        assert_eq!(verdict(&fork, MODE_EQUIPARTITION), VerdictKind::Fails);
        assert_eq!(
            verdict(&t, RJ_IR_LIMIT),
            VerdictKind::Holds,
            "live Planck still contains Rayleigh–Jeans in the IR"
        );
        let ir = fork
            .claims()
            .into_iter()
            .find(|c| c.id_str() == RJ_IR_LIMIT)
            .unwrap();
        let v = fork.evaluate(&ir);
        assert!(
            !v.summary.contains("catastrophe")
                && !v.summary.contains("quantum")
                && !v.summary.contains("Galilean"),
            "Wien occupation is not the quantum knob: {}",
            v.summary
        );
        assert!(
            v.evidence.iter().any(|e| e.contains("|u − u_RJ|/u_RJ")),
            "got {:?}",
            v.evidence
        );
        let residual = v
            .evidence
            .iter()
            .find(|e| e.contains("|u − u_RJ|/u_RJ"))
            .unwrap();
        assert!(
            residual.contains("0.99") || residual.contains("0.9"),
            "residual must be the IR mismatch, not a unit flag: {residual}"
        );
        let mut classical = Blackbody::planck();
        classical.set("quantum", KnobValue::Bool(false)).unwrap();
        assert_eq!(verdict(&classical, UV_FINITE), VerdictKind::Fails);
        assert_eq!(verdict(&classical, RJ_IR_LIMIT), VerdictKind::Holds);
        let probes = Blackbody::planck().structural_mutations();
        assert!(
            probes.iter().any(|(label, _)| label == "add-wien"),
            "live Planck must offer add-wien: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let probe = probes
            .iter()
            .find(|(label, _)| label == "add-wien")
            .expect("add-wien");
        assert_eq!(verdict(probe.1.as_ref(), RJ_IR_LIMIT), VerdictKind::Fails);
        let fork_probes = fork.structural_mutations();
        assert!(
            fork_probes.iter().all(|(label, _)| label != "add-wien"),
            "Wien fork must not re-offer add-wien"
        );
        let live = Blackbody::planck();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let mut abs = Blackbody::planck();
        abs.set("quantum", KnobValue::Bool(false)).unwrap();
        let rebuilt = abs.reparse_package(&parsed).unwrap();
        assert_eq!(
            rebuilt.get("quantum").unwrap(),
            KnobValue::Bool(false),
            "reparse must overlay Wien IR onto live knobs"
        );
        assert_eq!(
            verdict(rebuilt.as_ref(), UV_FINITE),
            VerdictKind::Fails,
            "quantum still Fails UV-finite on the live Bose encoding"
        );
        let live_rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(
            verdict(live_rebuilt.as_ref(), RJ_IR_LIMIT),
            VerdictKind::Holds
        );
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == RJ_IR_LIMIT)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "IR correspondence must keep the catalog 0.01 kT domain: {:?}",
            cell.domain()
        );
        assert!(
            Blackbody::rayleigh_jeans()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-wien"),
            "rayleigh-jeans must not grow add-wien"
        );
        assert!(
            crate::relativity::GeneralRelativity::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-wien"),
            "general-relativity must not grow add-wien"
        );
        assert!(
            crate::computation::TuringMachine::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-wien"),
            "turing-machine must not grow add-wien"
        );
    }
}
