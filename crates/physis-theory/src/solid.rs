//! Dulong–Petit on trial: classical oscillators vs Einstein vs Debye.
//!
//! The 1819 standing theory of solid heat capacity says every atom is three
//! classical oscillators, so `C_V = 3 N k` independent of temperature
//! (Dulong–Petit). That is equipartition applied to a lattice, and it is
//! *wrong* at low T: measured heat capacities vanish as T → 0, as the third
//! law requires.
//!
//! Einstein (1907) gives each oscillator a Bose mean energy
//! `ħω / (e^{ħω/kT} − 1)`. Then `C_V → 0` exponentially as T → 0, while
//! `C_V → 3 N k` when `T ≫ Θ_E` — the standing theory is the high-T limit,
//! not the law. The exponential freeze-out *over-suppresses* `C_V` relative
//! to the observed `T³` phonon law.
//!
//! Debye (1912) replaces the single frequency with an acoustic continuum of
//! density of states `g(ω) ∝ ω²` up to `ω_D`. The low-T heat capacity is
//! then the computed theorem `C_V = (12/5) π⁴ N k (T/Θ_D)³`, sampled as
//! `C_V(2T)/C_V(T) = 8`.
//!
//! Three lab objects share this encoding:
//! - [`EinsteinSolid::dulong_petit`] — the standing classical theory
//! - [`EinsteinSolid::einstein`] — the 1907 resolution (exponential freeze-out)
//! - [`EinsteinSolid::debye`] — the 1912 phonon continuum (`T³`)
//!
//! `set einstein-solid quantum false` restores Dulong–Petit.
//! `set einstein-solid spectrum debye` flips Einstein's exponential into Debye
//! `T³`. Raising `temperature` far above `Θ` makes Dulong–Petit hold again
//! (correspondence), without resurrecting the third-law or `T³` failures —
//! those are statements about T → 0, probed at `Θ/40` and `Θ/20`.

use std::f64::consts::PI;

use physis_core::claim::{Claim, Epistemic, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::qty::kelvin;
use physis_core::{Energy, HeatCapacity, Qty};
use physis_model::constants::k_boltzmann;
use physis_model::World;

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;
use crate::thermo::THIRD_LAW;

/// Heat capacity of a solid is 3 N k, independent of temperature (Dulong–Petit).
pub const DULONG_PETIT: &str = "thermo.dulong-petit";
/// At T ≫ Θ the heat capacity recovers the classical 3 N k (correspondence).
pub const HIGH_T_CLASSICAL: &str = "thermo.high-t-classical";
/// At T ≪ Θ_D the heat capacity of a phonon continuum scales as T³ (Debye).
pub const DEBYE_T3: &str = "thermo.debye-t3";

/// Matrix rows for the solid lab.
pub fn solid_rows() -> [&'static str; 4] {
    [DULONG_PETIT, HIGH_T_CLASSICAL, THIRD_LAW, DEBYE_T3]
}

const DEFAULT_T_K: f64 = 60.0;
const DEFAULT_THETA_K: f64 = 300.0;
const DEFAULT_N: f64 = 1.0e23;
/// Probe temperature for the third law, as a fraction of Θ.
const THIRD_LAW_T_OVER_THETA: f64 = 1.0 / 40.0;
/// High-T correspondence: T / Θ above this counts as classical.
const HIGH_T_RATIO: f64 = 8.0;
/// Low-T T³ probe: T = Θ/20 (and 2T = Θ/10), still deep in the Debye tail.
const T3_T_OVER_THETA: f64 = 1.0 / 20.0;
/// Bose tail beyond which `e^{-x}` is negligible (same cutoff as Planck).
const BOSE_X_TAIL: f64 = 40.0;
/// Doubling ratio `C_V(2T)/C_V(T)` must sit in this window to count as T³.
const T3_RATIO_LO: f64 = 7.0;
const T3_RATIO_HI: f64 = 9.0;
/// Relative match of `C_V(Θ/20)` to the analytic `(4π⁴/5)(T/Θ)³`.
const T3_ANALYTIC_TOL: f64 = 0.08;

const SPECTRUM_OPTIONS: &[&str] = &["einstein", "debye"];

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "quantum",
        layer: LayerId::Quantum,
        doc: "If true, oscillators are Bose-occupied (Einstein or Debye). If false, every oscillator has energy kT (Dulong–Petit). Turning this off is the 1819 standing theory.",
        domain: KnobDomain::Bool,
    },
    KnobSpec {
        name: "spectrum",
        layer: LayerId::Quantum,
        doc: "Phonon spectrum when quantum is true: einstein (single ω, exponential freeze-out) or debye (ω² density of states, T³). Ignored classically.",
        domain: KnobDomain::Choice(SPECTRUM_OPTIONS),
    },
    KnobSpec {
        name: "temperature",
        layer: LayerId::Statistical,
        doc: "Lattice temperature in kelvin.",
        domain: KnobDomain::Float {
            min: 1.0e-3,
            max: 1.0e5,
        },
    },
    KnobSpec {
        name: "einstein_temp",
        layer: LayerId::Statistical,
        doc: "Characteristic temperature Θ in kelvin: Einstein Θ_E = ħω/k, or Debye Θ_D = ħω_D/k. Classical Dulong–Petit ignores this.",
        domain: KnobDomain::Float {
            min: 1.0,
            max: 1.0e5,
        },
    },
    KnobSpec {
        name: "oscillators",
        layer: LayerId::Statistical,
        doc: "Number of atoms N (3N oscillators).",
        domain: KnobDomain::Float {
            min: 1.0,
            max: 1.0e30,
        },
    },
];

/// Phonon spectrum of a quantum solid: one frequency, or a Debye continuum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PhononSpectrum {
    Einstein,
    Debye,
}

impl PhononSpectrum {
    fn name(self) -> &'static str {
        match self {
            Self::Einstein => "einstein",
            Self::Debye => "debye",
        }
    }

    fn parse(name: &str) -> Option<Self> {
        match name {
            "einstein" => Some(Self::Einstein),
            "debye" => Some(Self::Debye),
            _ => None,
        }
    }
}

/// A lattice of 3N oscillators: Dulong–Petit, Einstein, or Debye.
#[derive(Clone, Debug)]
pub struct EinsteinSolid {
    id: &'static str,
    quantum: bool,
    spectrum: PhononSpectrum,
    temperature_k: f64,
    einstein_temp_k: f64,
    oscillators: f64,
}

impl Default for EinsteinSolid {
    fn default() -> Self {
        Self::einstein()
    }
}

impl EinsteinSolid {
    /// Einstein (1907): Bose occupation of 3N identical oscillators.
    pub fn einstein() -> Self {
        Self {
            id: "einstein-solid",
            quantum: true,
            spectrum: PhononSpectrum::Einstein,
            temperature_k: DEFAULT_T_K,
            einstein_temp_k: DEFAULT_THETA_K,
            oscillators: DEFAULT_N,
        }
    }

    /// Debye (1912): acoustic phonon continuum with `g(ω) ∝ ω²` up to `ω_D`.
    pub fn debye() -> Self {
        Self {
            id: "debye-solid",
            quantum: true,
            spectrum: PhononSpectrum::Debye,
            temperature_k: DEFAULT_T_K,
            einstein_temp_k: DEFAULT_THETA_K,
            oscillators: DEFAULT_N,
        }
    }

    /// Dulong–Petit (1819): classical equipartition, C_V = 3 N k.
    pub fn dulong_petit() -> Self {
        Self {
            id: "dulong-petit",
            quantum: false,
            spectrum: PhononSpectrum::Einstein,
            temperature_k: DEFAULT_T_K,
            einstein_temp_k: DEFAULT_THETA_K,
            oscillators: DEFAULT_N,
        }
    }

    fn x_at(&self, t_k: f64) -> f64 {
        self.einstein_temp_k / t_k
    }

    fn n3k(&self) -> Qty<HeatCapacity> {
        k_boltzmann() * (3.0 * self.oscillators)
    }

    fn is_debye(&self) -> bool {
        self.quantum && self.spectrum == PhononSpectrum::Debye
    }

    /// Internal energy of 3N oscillators, typed.
    fn internal_energy_at(&self, t_k: f64) -> Qty<Energy> {
        let n3 = 3.0 * self.oscillators;
        let kt: Qty<Energy> = k_boltzmann() * kelvin(t_k);
        if !self.quantum {
            return kt * n3;
        }
        match self.spectrum {
            PhononSpectrum::Einstein => {
                let x = self.x_at(t_k);
                if x < 1.0e-10 {
                    kt * n3
                } else {
                    // U = 3N · k Θ_E / (e^x − 1) = 3N · kT · x / (e^x − 1)
                    kt * n3 * (x / (x.exp() - 1.0))
                }
            }
            PhononSpectrum::Debye => {
                // U = 9 N k T (T/Θ_D)³ ∫_0^{x_D} x³/(e^x − 1) dx
                let x_d = self.x_at(t_k);
                if x_d < 1.0e-8 {
                    return kt * n3;
                }
                let t_over_theta = t_k / self.einstein_temp_k;
                kt * (9.0 * self.oscillators) * t_over_theta.powi(3) * debye_energy_integral(x_d)
            }
        }
    }

    fn heat_capacity_at(&self, t_k: f64) -> Qty<HeatCapacity> {
        self.n3k() * self.cv_over_3nk_at(t_k)
    }

    fn cv_over_3nk_at(&self, t_k: f64) -> f64 {
        if !self.quantum {
            return 1.0;
        }
        match self.spectrum {
            PhononSpectrum::Einstein => {
                // Analytic: C_V = 3 N k x² e^x / (e^x − 1)²
                let x = self.x_at(t_k);
                if x < 1.0e-8 {
                    1.0
                } else {
                    let ex = x.exp();
                    x * x * ex / ((ex - 1.0) * (ex - 1.0))
                }
            }
            PhononSpectrum::Debye => {
                // C_V = 9 N k (T/Θ_D)³ ∫_0^{x_D} x^4 e^x / (e^x − 1)² dx
                let x_d = self.x_at(t_k);
                if x_d < 1.0e-8 {
                    return 1.0;
                }
                let t_over_theta = t_k / self.einstein_temp_k;
                3.0 * t_over_theta.powi(3) * debye_cv_integral(x_d)
            }
        }
    }

    fn model_label(&self) -> &'static str {
        if !self.quantum {
            "Dulong–Petit"
        } else if self.spectrum == PhononSpectrum::Debye {
            "Debye"
        } else {
            "Einstein"
        }
    }
}

/// Low-T Debye law: `C_V / (3 N k) = (4 π⁴ / 5) (T/Θ_D)³`.
fn debye_t3_cv_over_3nk(t_over_theta: f64) -> f64 {
    (4.0 * PI.powi(4) / 5.0) * t_over_theta.powi(3)
}

/// `x³ / (e^x − 1)`, with the x → 0 limit `x²`.
fn bose_poly3(x: f64) -> f64 {
    if x < 1.0e-8 {
        x * x
    } else if x > BOSE_X_TAIL {
        x.powi(3) * (-x).exp()
    } else {
        x.powi(3) / (x.exp() - 1.0)
    }
}

/// `x^4 e^x / (e^x − 1)²`, with the x → 0 limit `x²`.
fn bose_cv_kernel(x: f64) -> f64 {
    if x < 1.0e-8 {
        x * x
    } else {
        let ex = x.exp();
        if !ex.is_finite() {
            0.0
        } else {
            x.powi(4) * ex / ((ex - 1.0) * (ex - 1.0))
        }
    }
}

fn trapezoid(xmax: f64, f: impl Fn(f64) -> f64) -> f64 {
    let xmax = xmax.clamp(0.0, BOSE_X_TAIL);
    if xmax == 0.0 {
        return 0.0;
    }
    let n = ((xmax * 100.0).ceil() as usize).clamp(400, 8000);
    let dx = xmax / n as f64;
    let mut acc = 0.5 * f(0.0) + 0.5 * f(xmax);
    for i in 1..n {
        acc += f(i as f64 * dx);
    }
    acc * dx
}

fn debye_energy_integral(x_d: f64) -> f64 {
    trapezoid(x_d, bose_poly3)
}

fn debye_cv_integral(x_d: f64) -> f64 {
    trapezoid(x_d, bose_cv_kernel)
}

impl Knobbed for EinsteinSolid {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "quantum" => Ok(KnobValue::Bool(self.quantum)),
            "spectrum" => Ok(KnobValue::Choice(self.spectrum.name().to_string())),
            "temperature" => Ok(KnobValue::Float(self.temperature_k)),
            "einstein_temp" => Ok(KnobValue::Float(self.einstein_temp_k)),
            "oscillators" => Ok(KnobValue::Float(self.oscillators)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("quantum", KnobValue::Bool(v)) => self.quantum = v,
            ("spectrum", KnobValue::Choice(v)) => {
                self.spectrum = PhononSpectrum::parse(&v).ok_or_else(|| CoreError::Domain {
                    name: name.into(),
                    reason: format!("'{v}' not in {SPECTRUM_OPTIONS:?}"),
                })?;
            }
            ("temperature", KnobValue::Float(v)) => self.temperature_k = v,
            ("einstein_temp", KnobValue::Float(v)) => self.einstein_temp_k = v,
            ("oscillators", KnobValue::Float(v)) => self.oscillators = v,
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

impl Theory for EinsteinSolid {
    fn id(&self) -> &'static str {
        self.id
    }
    fn name(&self) -> &'static str {
        match (self.quantum, self.spectrum) {
            (false, _) => "Dulong–Petit (classical solid)",
            (true, PhononSpectrum::Debye) => "Debye solid",
            (true, PhononSpectrum::Einstein) => "Einstein solid",
        }
    }
    fn summary(&self) -> &'static str {
        "A lattice of 3N oscillators. Classical equipartition gives C_V = 3 N k \
         at every T (Dulong–Petit) and fails the third law. Einstein's Bose \
         occupation makes C_V vanish exponentially as T → 0 — over-suppressing \
         the observed T³ phonon law. Debye's ω² density of states holds T³ \
         and recovers 3 N k only for T ≫ Θ_D."
    }
    fn world(&self) -> Option<World> {
        None
    }
    fn note(&self) -> String {
        format!(
            "{} solid: T = {} K, Θ = {} K, N = {:.2e}, C_V = {:.4e} J/K, C_V/(3Nk) = {:.4}",
            self.model_label(),
            self.temperature_k,
            self.einstein_temp_k,
            self.oscillators,
            self.heat_capacity_at(self.temperature_k).value(),
            self.cv_over_3nk_at(self.temperature_k)
        )
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                DULONG_PETIT,
                "The heat capacity is 3 N k, independent of temperature (Dulong–Petit).",
                LayerId::Statistical,
                Epistemic::Theorem,
            ),
            Claim::new(
                HIGH_T_CLASSICAL,
                "At T ≫ Θ the heat capacity recovers the classical 3 N k.",
                LayerId::Statistical,
                Epistemic::Theorem,
            ),
            Claim::new(
                THIRD_LAW,
                "Heat capacity (and therefore entropy) tends to zero as T → 0.",
                LayerId::Statistical,
                Epistemic::Theorem,
            ),
            Claim::new(
                DEBYE_T3,
                "At T ≪ Θ the heat capacity scales as T³ (Debye phonon continuum).",
                LayerId::Statistical,
                Epistemic::Theorem,
            ),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id.0.as_str() {
            DULONG_PETIT => {
                let ratio = self.cv_over_3nk_at(self.temperature_k);
                let u = self.internal_energy_at(self.temperature_k);
                if (ratio - 1.0).abs() < 0.05 {
                    Verdict::holds(
                        Epistemic::Theorem,
                        "C_V = 3 N k at the current temperature (Dulong–Petit)",
                    )
                    .with_evidence([format!(
                        "U = {:.4e} J; C_V/(3Nk) = {ratio:.4} at T/Θ = {:.3}",
                        u.value(),
                        self.temperature_k / self.einstein_temp_k
                    )])
                } else {
                    let why = if self.is_debye() {
                        "C_V is not 3 N k: Debye phonon modes are frozen out"
                    } else {
                        "C_V is not 3 N k: Einstein oscillators are frozen out"
                    };
                    Verdict::fails(Epistemic::Theorem, why).with_evidence([format!(
                        "U = {:.4e} J; C_V/(3Nk) = {ratio:.4} at T/Θ = {:.3} (Dulong–Petit requires 1)",
                        u.value(),
                        self.temperature_k / self.einstein_temp_k
                    )])
                }
            }
            HIGH_T_CLASSICAL => {
                let ratio_t = self.temperature_k / self.einstein_temp_k;
                let cv = self.cv_over_3nk_at(self.temperature_k);
                if ratio_t >= HIGH_T_RATIO && (cv - 1.0).abs() < 0.05 {
                    Verdict::holds(
                        Epistemic::Theorem,
                        "T ≫ Θ: C_V has recovered the classical 3 N k",
                    )
                    .with_evidence([format!("T/Θ = {ratio_t:.2}, C_V/(3Nk) = {cv:.4}")])
                } else if !self.quantum && (cv - 1.0).abs() < 0.05 {
                    Verdict::holds(
                        Epistemic::Theorem,
                        "classical C_V = 3 N k at every T, including T ≫ Θ",
                    )
                    .with_evidence([format!(
                        "C_V/(3Nk) = {cv:.4} (independent of T/Θ = {ratio_t:.3})"
                    )])
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        "not in the high-T regime: C_V has not recovered 3 N k",
                    )
                    .with_evidence([format!(
                        "T/Θ = {ratio_t:.3} (need ≥ {HIGH_T_RATIO}), C_V/(3Nk) = {cv:.4}"
                    )])
                }
            }
            THIRD_LAW => {
                let t_probe = self.einstein_temp_k * THIRD_LAW_T_OVER_THETA;
                let cv = self.cv_over_3nk_at(t_probe);
                if cv < 0.05 {
                    let why = if self.is_debye() {
                        "C_V → 0 as T → 0 (Debye T³ freeze-out)"
                    } else {
                        "C_V → 0 as T → 0 (Einstein freeze-out)"
                    };
                    Verdict::holds(Epistemic::Theorem, why).with_evidence([format!(
                        "C_V/(3Nk) = {cv:.3e} at T = Θ/40 = {t_probe:.3} K"
                    )])
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        "classical C_V = 3 N k down to T → 0; the third law fails",
                    )
                    .with_evidence([format!(
                        "C_V/(3Nk) = {cv:.4} at T = Θ/40 = {t_probe:.3} K (does not vanish)"
                    )])
                }
            }
            DEBYE_T3 => eval_debye_t3(self),
            _ => Verdict::inapplicable("claim not made by a solid-oscillator object"),
        }
    }
}

fn eval_debye_t3(solid: &EinsteinSolid) -> Verdict {
    let t1 = solid.einstein_temp_k * T3_T_OVER_THETA;
    let t2 = 2.0 * t1;
    let cv1 = solid.cv_over_3nk_at(t1);
    let cv2 = solid.cv_over_3nk_at(t2);
    let ratio = if cv1 > 0.0 { cv2 / cv1 } else { f64::INFINITY };
    let analytic = debye_t3_cv_over_3nk(T3_T_OVER_THETA);
    let doubling_ok = (T3_RATIO_LO..=T3_RATIO_HI).contains(&ratio);
    let magnitude_ok = analytic > 0.0 && (cv1 / analytic - 1.0).abs() < T3_ANALYTIC_TOL;
    if solid.is_debye() && doubling_ok && magnitude_ok {
        Verdict::holds(
            Epistemic::Theorem,
            "C_V ∝ T³ at low T (Debye phonon continuum)",
        )
        .with_evidence([format!(
            "C_V(2T)/C_V(T) = {ratio:.3} at T = Θ_D/20 (T³ requires 8); \
             C_V/(3Nk) = {cv1:.4e} vs (4π⁴/5)(T/Θ)³ = {analytic:.4e}"
        )])
    } else if !solid.quantum {
        Verdict::fails(
            Epistemic::Theorem,
            "classical C_V is independent of T, not T³",
        )
        .with_evidence([format!(
            "C_V(2T)/C_V(T) = {ratio:.3} at T = Θ/20 (T³ requires 8)"
        )])
    } else if solid.spectrum == PhononSpectrum::Einstein {
        Verdict::fails(
            Epistemic::Theorem,
            "Einstein freeze-out is exponential, not T³",
        )
        .with_evidence([format!(
            "C_V(2T)/C_V(T) = {ratio:.3e} at T = Θ_E/20 (T³ requires 8); \
             C_V/(3Nk) = {cv1:.3e} vs Debye T³ {analytic:.3e}"
        )])
    } else {
        Verdict::fails(
            Epistemic::Theorem,
            "Debye C_V is not in the T³ window at the low-T probe",
        )
        .with_evidence([format!(
            "C_V(2T)/C_V(T) = {ratio:.3}, C_V/(3Nk) = {cv1:.4e}, \
             (4π⁴/5)(T/Θ)³ = {analytic:.4e}"
        )])
    }
}

/// Dulong–Petit vs Einstein vs Debye: classical solid heat capacity on trial.
pub fn solid() -> ExperimentReport {
    report_from_rows(
        "solid",
        "Einstein–Debye solid lab",
        "Does classical equipartition of lattice oscillators survive contact \
         with the third law — and does Einstein's exponential freeze-out match \
         the observed T³ phonon law, or does only Debye's ω² density of states \
         hold that theorem?",
        "Verdicts are internal to the encoding. Dulong–Petit is a computed \
         C_V = 3 N k at every T, not a slogan. Einstein's C_V is the analytic \
         x² e^x / (e^x − 1)². Debye's T³ is a sampled doubling C_V(2T)/C_V(T) \
         at Θ/20 plus a match to (12/5)π⁴ N k (T/Θ)³, independent of the current T.",
        vec![
            "`thermo.dulong-petit` is the standing 1819 claim: it holds for the classical solid and fails for Einstein and Debye at T ≲ Θ.".into(),
            "`thermo.third-law` fails classically (C_V stays 3 N k) and holds for both Einstein (exponential) and Debye (T³).".into(),
            "`thermo.debye-t3` is the 1912 correction: it holds only for Debye. Einstein over-freezes (exponential, doubling ≫ 8); Dulong–Petit is T-independent (doubling = 1).".into(),
            "`thermo.high-t-classical` is the correspondence: raising T far above Θ on einstein-solid or debye-solid flips dulong-petit fails → holds.".into(),
            "`set einstein-solid spectrum debye` flips thermo.debye-t3 fails → holds. `set einstein-solid quantum false` restores Dulong–Petit.".into(),
        ],
        &solid_rows(),
        vec![
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
        let c = t.claims().into_iter().find(|c| c.id.0 == id).unwrap();
        t.evaluate(&c).kind
    }

    fn finite_diff_cv(s: &EinsteinSolid) -> f64 {
        let h = s.temperature_k * 1e-5;
        (s.internal_energy_at(s.temperature_k + h).value()
            - s.internal_energy_at(s.temperature_k - h).value())
            / (2.0 * h)
    }

    #[test]
    fn einstein_cv_matches_analytic_and_finite_difference() {
        let s = EinsteinSolid::einstein();
        let analytic = s.heat_capacity_at(s.temperature_k).value();
        let du = finite_diff_cv(&s);
        assert!(
            (analytic - du).abs() / analytic < 1e-6,
            "analytic {analytic} vs dU/dT {du}"
        );
        assert!(s.cv_over_3nk_at(s.temperature_k) < 0.5);
    }

    #[test]
    fn debye_cv_matches_finite_difference() {
        let s = EinsteinSolid::debye();
        let analytic = s.heat_capacity_at(s.temperature_k).value();
        let du = finite_diff_cv(&s);
        assert!(
            (analytic - du).abs() / analytic < 1e-4,
            "Debye C_V {analytic} vs dU/dT {du}"
        );
    }

    #[test]
    fn debye_integrals_match_bose_infinity() {
        let energy = debye_energy_integral(BOSE_X_TAIL);
        let pi4_15 = PI.powi(4) / 15.0;
        assert!(
            (energy / pi4_15 - 1.0).abs() < 1e-4,
            "∫ x³/(e^x−1) dx = {energy}, π⁴/15 = {pi4_15}"
        );
        let cv = debye_cv_integral(BOSE_X_TAIL);
        let four_pi4_15 = 4.0 * PI.powi(4) / 15.0;
        assert!(
            (cv / four_pi4_15 - 1.0).abs() < 1e-4,
            "∫ x⁴ e^x/(e^x−1)² dx = {cv}, 4π⁴/15 = {four_pi4_15}"
        );
    }

    #[test]
    fn debye_high_t_recovers_three_n_k() {
        let mut s = EinsteinSolid::debye();
        s.set("temperature", KnobValue::Float(4000.0)).unwrap();
        let ratio = s.cv_over_3nk_at(s.temperature_k);
        assert!(
            (ratio - 1.0).abs() < 0.02,
            "Debye high-T C_V/(3Nk) = {ratio}"
        );
        assert_eq!(verdict(&s, DULONG_PETIT), VerdictKind::Holds);
        assert_eq!(verdict(&s, HIGH_T_CLASSICAL), VerdictKind::Holds);
        assert_eq!(verdict(&s, THIRD_LAW), VerdictKind::Holds);
        assert_eq!(verdict(&s, DEBYE_T3), VerdictKind::Holds);
    }

    #[test]
    fn dulong_petit_is_three_n_k_at_every_t() {
        let dp = EinsteinSolid::dulong_petit();
        assert!((dp.cv_over_3nk_at(dp.temperature_k) - 1.0).abs() < 1e-12);
        assert_eq!(verdict(&dp, DULONG_PETIT), VerdictKind::Holds);
        assert_eq!(verdict(&dp, HIGH_T_CLASSICAL), VerdictKind::Holds);
        assert_eq!(verdict(&dp, THIRD_LAW), VerdictKind::Fails);
        assert_eq!(verdict(&dp, DEBYE_T3), VerdictKind::Fails);
    }

    #[test]
    fn einstein_fails_dulong_petit_and_t3_and_holds_the_third_law() {
        let e = EinsteinSolid::einstein();
        assert_eq!(verdict(&e, DULONG_PETIT), VerdictKind::Fails);
        assert_eq!(verdict(&e, HIGH_T_CLASSICAL), VerdictKind::Fails);
        assert_eq!(verdict(&e, THIRD_LAW), VerdictKind::Holds);
        assert_eq!(verdict(&e, DEBYE_T3), VerdictKind::Fails);
        assert!(e.cv_over_3nk_at(e.temperature_k) < 0.5);
    }

    #[test]
    fn debye_holds_t3_and_the_third_law_and_fails_dulong_petit() {
        let d = EinsteinSolid::debye();
        assert_eq!(verdict(&d, DULONG_PETIT), VerdictKind::Fails);
        assert_eq!(verdict(&d, HIGH_T_CLASSICAL), VerdictKind::Fails);
        assert_eq!(verdict(&d, THIRD_LAW), VerdictKind::Holds);
        assert_eq!(verdict(&d, DEBYE_T3), VerdictKind::Holds);
        // Einstein over-freezes relative to Debye at the same Θ and T.
        let e = EinsteinSolid::einstein();
        assert!(
            e.cv_over_3nk_at(e.temperature_k) < d.cv_over_3nk_at(d.temperature_k),
            "Einstein C_V/(3Nk) = {} should be < Debye {}",
            e.cv_over_3nk_at(e.temperature_k),
            d.cv_over_3nk_at(d.temperature_k)
        );
    }

    #[test]
    fn raising_temperature_recovers_dulong_petit() {
        let mut e = EinsteinSolid::einstein();
        assert_eq!(verdict(&e, DULONG_PETIT), VerdictKind::Fails);
        e.set("temperature", KnobValue::Float(4000.0)).unwrap();
        assert_eq!(verdict(&e, DULONG_PETIT), VerdictKind::Holds);
        assert_eq!(verdict(&e, HIGH_T_CLASSICAL), VerdictKind::Holds);
        // The third law and T³ are T → 0 statements, not about the current T.
        assert_eq!(verdict(&e, THIRD_LAW), VerdictKind::Holds);
        assert_eq!(verdict(&e, DEBYE_T3), VerdictKind::Fails);
    }

    #[test]
    fn quantum_knob_restores_dulong_petit() {
        let mut e = EinsteinSolid::einstein();
        e.set("quantum", KnobValue::Bool(false)).unwrap();
        assert_eq!(verdict(&e, DULONG_PETIT), VerdictKind::Holds);
        assert_eq!(verdict(&e, THIRD_LAW), VerdictKind::Fails);
        assert_eq!(verdict(&e, DEBYE_T3), VerdictKind::Fails);
        assert_eq!(e.id(), "einstein-solid");
    }

    #[test]
    fn spectrum_knob_flips_einstein_to_debye_t3() {
        let mut e = EinsteinSolid::einstein();
        assert_eq!(verdict(&e, DEBYE_T3), VerdictKind::Fails);
        e.set("spectrum", KnobValue::Choice("debye".into()))
            .unwrap();
        assert_eq!(verdict(&e, DEBYE_T3), VerdictKind::Holds);
        assert_eq!(verdict(&e, THIRD_LAW), VerdictKind::Holds);
        assert_eq!(verdict(&e, DULONG_PETIT), VerdictKind::Fails);
        assert_eq!(e.id(), "einstein-solid");
    }

    #[test]
    fn energy_and_heat_capacity_are_typed() {
        let e = EinsteinSolid::einstein();
        let u: Qty<Energy> = e.internal_energy_at(e.temperature_k);
        let cv: Qty<HeatCapacity> = e.heat_capacity_at(e.temperature_k);
        assert!(u.value() > 0.0 && u.value().is_finite());
        assert!(cv.value() > 0.0 && cv.value().is_finite());
        let d = EinsteinSolid::debye();
        let u_d: Qty<Energy> = d.internal_energy_at(d.temperature_k);
        let cv_d: Qty<HeatCapacity> = d.heat_capacity_at(d.temperature_k);
        assert!(u_d.value() > 0.0 && cv_d.value() > 0.0 && cv_d.value().is_finite());
    }

    #[test]
    fn solid_experiment_puts_dulong_petit_and_einstein_on_trial_against_debye() {
        let r = solid();
        assert_eq!(r.id, "solid");
        let cell =
            |claim: &str, theory: &str| r.matrix.get(claim).and_then(|m| m.get(theory)).copied();
        assert_eq!(cell(DULONG_PETIT, "dulong-petit"), Some(VerdictKind::Holds));
        assert_eq!(
            cell(DULONG_PETIT, "einstein-solid"),
            Some(VerdictKind::Fails)
        );
        assert_eq!(cell(DULONG_PETIT, "debye-solid"), Some(VerdictKind::Fails));
        assert_eq!(cell(THIRD_LAW, "dulong-petit"), Some(VerdictKind::Fails));
        assert_eq!(cell(THIRD_LAW, "einstein-solid"), Some(VerdictKind::Holds));
        assert_eq!(cell(THIRD_LAW, "debye-solid"), Some(VerdictKind::Holds));
        assert_eq!(cell(DEBYE_T3, "dulong-petit"), Some(VerdictKind::Fails));
        assert_eq!(cell(DEBYE_T3, "einstein-solid"), Some(VerdictKind::Fails));
        assert_eq!(cell(DEBYE_T3, "debye-solid"), Some(VerdictKind::Holds));
    }
}
