//! Dulong–Petit on trial: classical oscillators vs the Einstein solid.
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
//! not the law.
//!
//! Two lab objects share this encoding:
//! - [`EinsteinSolid::dulong_petit`] — the standing classical theory
//! - [`EinsteinSolid::einstein`] — the 1907 resolution
//!
//! `set einstein-solid quantum false` restores Dulong–Petit. Raising
//! `temperature` on the Einstein object far above `Θ_E` makes Dulong–Petit
//! hold again (correspondence), without resurrecting the third-law failure.

use physis_core::claim::{Claim, Epistemic, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::qty::kelvin;
use physis_core::{Energy, Qty};
use physis_model::constants::k_boltzmann;
use physis_model::World;

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;
use crate::thermo::THIRD_LAW;

/// Heat capacity of a solid is 3 N k, independent of temperature (Dulong–Petit).
pub const DULONG_PETIT: &str = "thermo.dulong-petit";
/// At T ≫ Θ_E the heat capacity recovers the classical 3 N k (correspondence).
pub const HIGH_T_CLASSICAL: &str = "thermo.high-t-classical";

/// Matrix rows for the solid lab.
pub fn solid_rows() -> [&'static str; 3] {
    [DULONG_PETIT, HIGH_T_CLASSICAL, THIRD_LAW]
}

const DEFAULT_T_K: f64 = 60.0;
const DEFAULT_THETA_K: f64 = 300.0;
const DEFAULT_N: f64 = 1.0e23;
/// Probe temperature for the third law, as a fraction of Θ_E.
const THIRD_LAW_T_OVER_THETA: f64 = 1.0 / 40.0;
/// High-T correspondence: T / Θ_E above this counts as classical.
const HIGH_T_RATIO: f64 = 8.0;

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "quantum",
        layer: LayerId::Quantum,
        doc: "If true, oscillators are Einstein (Bose). If false, every oscillator has energy kT (Dulong–Petit). Turning this off is the 1819 standing theory.",
        domain: KnobDomain::Bool,
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
        doc: "Einstein temperature Θ_E = ħω/k in kelvin. Classical Dulong–Petit ignores this; Einstein physics depends on T/Θ_E.",
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

/// A lattice of 3N oscillators: Dulong–Petit or Einstein.
#[derive(Clone, Debug)]
pub struct EinsteinSolid {
    id: &'static str,
    quantum: bool,
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
            temperature_k: DEFAULT_T_K,
            einstein_temp_k: DEFAULT_THETA_K,
            oscillators: DEFAULT_N,
        }
    }

    fn x_at(&self, t_k: f64) -> f64 {
        self.einstein_temp_k / t_k
    }

    /// Internal energy of 3N oscillators, typed.
    fn internal_energy_at(&self, t_k: f64) -> Qty<Energy> {
        let n3 = 3.0 * self.oscillators;
        let kt: Qty<Energy> = k_boltzmann() * kelvin(t_k);
        if !self.quantum {
            return kt * n3;
        }
        let x = self.x_at(t_k);
        if x < 1.0e-10 {
            kt * n3
        } else {
            // U = 3N · k Θ_E / (e^x − 1) = 3N · kT · x / (e^x − 1)
            kt * n3 * (x / (x.exp() - 1.0))
        }
    }

    fn heat_capacity_at(&self, t_k: f64) -> f64 {
        if self.quantum {
            // Analytic: C_V = 3 N k x² e^x / (e^x − 1)²
            let x = self.x_at(t_k);
            let n3k = 3.0 * self.oscillators * k_boltzmann().value();
            if x < 1.0e-8 {
                return n3k;
            }
            let ex = x.exp();
            n3k * x * x * ex / ((ex - 1.0) * (ex - 1.0))
        } else {
            3.0 * self.oscillators * k_boltzmann().value()
        }
    }

    fn cv_over_3nk_at(&self, t_k: f64) -> f64 {
        self.heat_capacity_at(t_k) / (3.0 * self.oscillators * k_boltzmann().value())
    }
}

impl Knobbed for EinsteinSolid {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "quantum" => Ok(KnobValue::Bool(self.quantum)),
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
        if self.quantum {
            "Einstein solid"
        } else {
            "Dulong–Petit (classical solid)"
        }
    }
    fn summary(&self) -> &'static str {
        "A lattice of 3N oscillators. Classical equipartition gives C_V = 3 N k \
         at every T (Dulong–Petit) and fails the third law. Einstein's Bose \
         occupation makes C_V vanish as T → 0 and recover 3 N k only for T ≫ Θ_E."
    }
    fn world(&self) -> Option<World> {
        None
    }
    fn note(&self) -> String {
        format!(
            "{} solid: T = {} K, Θ_E = {} K, N = {:.2e}, C_V/(3Nk) = {:.4}",
            if self.quantum {
                "Einstein"
            } else {
                "Dulong–Petit"
            },
            self.temperature_k,
            self.einstein_temp_k,
            self.oscillators,
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
                "At T ≫ Θ_E the heat capacity recovers the classical 3 N k.",
                LayerId::Statistical,
                Epistemic::Theorem,
            ),
            Claim::new(
                THIRD_LAW,
                "Heat capacity (and therefore entropy) tends to zero as T → 0.",
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
                        "U = {:.4e} J; C_V/(3Nk) = {ratio:.4} at T/Θ_E = {:.3}",
                        u.value(),
                        self.temperature_k / self.einstein_temp_k
                    )])
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        "C_V is not 3 N k: Einstein oscillators are frozen out",
                    )
                    .with_evidence([format!(
                        "U = {:.4e} J; C_V/(3Nk) = {ratio:.4} at T/Θ_E = {:.3} (Dulong–Petit requires 1)",
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
                        "T ≫ Θ_E: C_V has recovered the classical 3 N k",
                    )
                    .with_evidence([format!("T/Θ_E = {ratio_t:.2}, C_V/(3Nk) = {cv:.4}")])
                } else if !self.quantum && (cv - 1.0).abs() < 0.05 {
                    Verdict::holds(
                        Epistemic::Theorem,
                        "classical C_V = 3 N k at every T, including T ≫ Θ_E",
                    )
                    .with_evidence([format!(
                        "C_V/(3Nk) = {cv:.4} (independent of T/Θ_E = {ratio_t:.3})"
                    )])
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        "not in the high-T regime: C_V has not recovered 3 N k",
                    )
                    .with_evidence([format!(
                        "T/Θ_E = {ratio_t:.3} (need ≥ {HIGH_T_RATIO}), C_V/(3Nk) = {cv:.4}"
                    )])
                }
            }
            THIRD_LAW => {
                let t_probe = self.einstein_temp_k * THIRD_LAW_T_OVER_THETA;
                let cv = self.cv_over_3nk_at(t_probe);
                if cv < 0.05 {
                    Verdict::holds(Epistemic::Theorem, "C_V → 0 as T → 0 (Einstein freeze-out)")
                        .with_evidence([format!(
                            "C_V/(3Nk) = {cv:.3e} at T = Θ_E/40 = {t_probe:.3} K"
                        )])
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        "classical C_V = 3 N k down to T → 0; the third law fails",
                    )
                    .with_evidence([format!(
                        "C_V/(3Nk) = {cv:.4} at T = Θ_E/40 = {t_probe:.3} K (does not vanish)"
                    )])
                }
            }
            _ => Verdict::inapplicable("claim not made by a solid-oscillator object"),
        }
    }
}

/// Dulong–Petit vs Einstein: classical solid heat capacity on trial.
pub fn solid() -> ExperimentReport {
    report_from_rows(
        "solid",
        "Einstein solid lab",
        "Does classical equipartition of lattice oscillators survive contact \
         with the third law and the observed drop of C_V at low T — or does \
         Dulong–Petit fail those theorems, while Einstein's Bose occupation \
         holds the third law and recovers 3 N k only as T ≫ Θ_E?",
        "Verdicts are internal to the encoding. Dulong–Petit is a computed \
         C_V = 3 N k at every T, not a slogan. Einstein's C_V is the analytic \
         x² e^x / (e^x − 1)² with x = Θ_E/T, as a typed energy derivative.",
        vec![
            "`thermo.dulong-petit` is the standing 1819 claim: it holds for the classical solid and fails for Einstein at T ≲ Θ_E.".into(),
            "`thermo.third-law` fails classically (C_V stays 3 N k) and holds for Einstein (exponential freeze-out).".into(),
            "`thermo.high-t-classical` is the correspondence: raising T far above Θ_E on einstein-solid flips dulong-petit fails → holds.".into(),
            "`set einstein-solid quantum false` restores Dulong–Petit.".into(),
        ],
        &solid_rows(),
        vec![
            Box::new(EinsteinSolid::dulong_petit()),
            Box::new(EinsteinSolid::einstein()),
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

    #[test]
    fn einstein_cv_matches_analytic_and_finite_difference() {
        let s = EinsteinSolid::einstein();
        let analytic = s.heat_capacity_at(s.temperature_k);
        let h = s.temperature_k * 1e-5;
        let du = (s.internal_energy_at(s.temperature_k + h).value()
            - s.internal_energy_at(s.temperature_k - h).value())
            / (2.0 * h);
        assert!(
            (analytic - du).abs() / analytic < 1e-6,
            "analytic {analytic} vs dU/dT {du}"
        );
        assert!(s.cv_over_3nk_at(s.temperature_k) < 0.5);
    }

    #[test]
    fn dulong_petit_is_three_n_k_at_every_t() {
        let dp = EinsteinSolid::dulong_petit();
        assert!((dp.cv_over_3nk_at(dp.temperature_k) - 1.0).abs() < 1e-12);
        assert_eq!(verdict(&dp, DULONG_PETIT), VerdictKind::Holds);
        assert_eq!(verdict(&dp, HIGH_T_CLASSICAL), VerdictKind::Holds);
        assert_eq!(verdict(&dp, THIRD_LAW), VerdictKind::Fails);
    }

    #[test]
    fn einstein_fails_dulong_petit_at_low_t_and_holds_the_third_law() {
        let e = EinsteinSolid::einstein();
        assert_eq!(verdict(&e, DULONG_PETIT), VerdictKind::Fails);
        assert_eq!(verdict(&e, HIGH_T_CLASSICAL), VerdictKind::Fails);
        assert_eq!(verdict(&e, THIRD_LAW), VerdictKind::Holds);
    }

    #[test]
    fn raising_temperature_recovers_dulong_petit() {
        let mut e = EinsteinSolid::einstein();
        assert_eq!(verdict(&e, DULONG_PETIT), VerdictKind::Fails);
        e.set("temperature", KnobValue::Float(4000.0)).unwrap();
        assert_eq!(verdict(&e, DULONG_PETIT), VerdictKind::Holds);
        assert_eq!(verdict(&e, HIGH_T_CLASSICAL), VerdictKind::Holds);
        // The third law is a T → 0 statement, not about the current T.
        assert_eq!(verdict(&e, THIRD_LAW), VerdictKind::Holds);
    }

    #[test]
    fn quantum_knob_restores_dulong_petit() {
        let mut e = EinsteinSolid::einstein();
        e.set("quantum", KnobValue::Bool(false)).unwrap();
        assert_eq!(verdict(&e, DULONG_PETIT), VerdictKind::Holds);
        assert_eq!(verdict(&e, THIRD_LAW), VerdictKind::Fails);
        assert_eq!(e.id(), "einstein-solid");
    }

    #[test]
    fn energy_is_typed() {
        let e = EinsteinSolid::einstein();
        let u: Qty<Energy> = e.internal_energy_at(e.temperature_k);
        assert!(u.value() > 0.0 && u.value().is_finite());
    }

    #[test]
    fn solid_experiment_puts_dulong_petit_on_trial() {
        let r = solid();
        assert_eq!(r.id, "solid");
        let cell =
            |claim: &str, theory: &str| r.matrix.get(claim).and_then(|m| m.get(theory)).copied();
        assert_eq!(cell(DULONG_PETIT, "dulong-petit"), Some(VerdictKind::Holds));
        assert_eq!(
            cell(DULONG_PETIT, "einstein-solid"),
            Some(VerdictKind::Fails)
        );
        assert_eq!(cell(THIRD_LAW, "dulong-petit"), Some(VerdictKind::Fails));
        assert_eq!(cell(THIRD_LAW, "einstein-solid"), Some(VerdictKind::Holds));
    }
}
