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
//!
//! Maxwell–Boltzmann statistics live on the IR package. Bose quantum
//! statistics are a package mutation (`add-bose`), not a temperature knob:
//! the Sackur–Tetrode `ln T` entropy is unbounded below, and
//! `thermo.third-law` fails; the mutant's low-T Bose entropy vanishes and
//! the cell holds. Degenerate Fermi statistics are a second package
//! mutation (`add-fermi`): Sommerfeld `C_V = (π²/2) N k (T/T_F)` is not
//! `(3/2) N k`, so `thermo.equipartition` fails. `temperature` /
//! `volume_ratio` / `particles` still scale the classical gas. Those forks
//! are still this object, not a silent Einstein-solid install.

use std::f64::consts::PI;

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

/// Maxwell–Boltzmann statistics on the live ideal-gas package.
const MAXWELL_BOLTZMANN_EQ: &str = "gas maxwell-boltzmann";
/// Bose quantum statistics (ideal Bose gas below a schematic T_c).
const BOSE_EQ: &str = "gas bose";
/// Degenerate Fermi statistics (Sommerfeld expansion at T ≪ T_F).
const FERMI_EQ: &str = "gas fermi";
/// Schematic Bose condensation temperature (K). Default T = 300 K is
/// deeply non-degenerate, so equipartition stays (3/2) N k.
const BOSE_TC_K: f64 = 1.0;
/// Schematic Fermi temperature (K). Default T = 300 K is T/T_F ≪ 1,
/// so Sommerfeld C_V is not (3/2) N k.
const FERMI_TF_K: f64 = 5.0e4;
/// Low-T third-law probe, as a fraction of T_c (same Θ/40 idea as Einstein).
const THIRD_LAW_T_OVER_TC: f64 = 1.0 / 40.0;
/// Fermi third-law probe: S/Nk = (π²/2)(T/T_F); T_F/400 keeps it vanishing.
const FERMI_THIRD_LAW_T_OVER_TF: f64 = 1.0 / 400.0;
/// (5/2) ζ(5/2)/ζ(3/2): S/Nk = this × (T/T_c)^{3/2} for T < T_c.
const BOSE_S_PREF: f64 = 1.283_788;
/// S/Nk below this at the probe counts as vanishing.
const THIRD_LAW_S_NK: f64 = 0.05;

fn parse_gas_statistics(pkg: &TheoryPackage) -> Result<(bool, bool), String> {
    let mut maxwell = false;
    let mut bose = false;
    let mut fermi = false;
    for eq in &pkg.equations {
        match eq.trim() {
            MAXWELL_BOLTZMANN_EQ => maxwell = true,
            BOSE_EQ => bose = true,
            FERMI_EQ => fermi = true,
            _ => {}
        }
    }
    if !maxwell {
        return Err(format!(
            "{} package has no Maxwell-Boltzmann statistics",
            pkg.id
        ));
    }
    Ok((bose, fermi))
}

fn third_law_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["classical Maxwell-Boltzmann Sackur-Tetrode".into()],
        vec!["S ∝ ln T with no ground-state cutoff".into()],
        "The third-law cell is the classical gas encoding. Bose or Fermi \
         statistics are a new encoding, not a silent temperature knob.",
    )
}

fn equipartition_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["classical C_V = 3/2 Nk".into()],
        vec!["U = 3/2 N k T on Maxwell-Boltzmann and non-degenerate Bose".into()],
        "The equipartition cell is the classical gas encoding. A degenerate \
         Fermi sea is a new encoding, not a silent temperature knob.",
    )
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

/// A monatomic ideal gas.
///
/// Statistics live on the IR package. Bose quantum statistics are a
/// package mutation (`add-bose`), not a knob: the third law fails on the
/// classical encoding and holds on the mutant. Degenerate Fermi statistics
/// are a second mutation (`add-fermi`): Sommerfeld heat capacity is not
/// (3/2) N k, so equipartition fails. Those forks are still this object,
/// not a silent Einstein-solid install. `temperature` /
/// `volume_ratio` / `particles` stay knobs.
#[derive(Clone, Debug, PartialEq)]
pub struct IdealGas {
    temperature_k: f64,
    volume_ratio: f64,
    particles: f64,
    bose: bool,
    /// Degenerate Fermi sea. Not a knob.
    fermi: bool,
}

impl Default for IdealGas {
    fn default() -> Self {
        Self {
            temperature_k: 300.0,
            volume_ratio: 2.0,
            particles: 1.0e23,
            bose: false,
            fermi: false,
        }
    }
}

impl IdealGas {
    /// Internal energy. Live / Bose: U = (3/2) N k T. Fermi: Sommerfeld
    /// U = (3/5) N k T_F [1 + (5π²/12)(T/T_F)²].
    fn internal_energy(&self) -> Qty<Energy> {
        if self.fermi {
            let t = self.temperature_k;
            let tf = FERMI_TF_K;
            let ratio = t / tf;
            let u = 0.6
                * self.particles
                * k_boltzmann().value()
                * tf
                * (1.0 + (5.0 * PI * PI / 12.0) * ratio * ratio);
            Qty::new(u)
        } else {
            k_boltzmann() * kelvin(self.temperature_k) * (1.5 * self.particles)
        }
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

    /// IR package for this statistics encoding. Equations are
    /// `gas maxwell-boltzmann` and, when forked, `gas bose` and/or
    /// `gas fermi`. Temperature, volume ratio, and N stay on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![MAXWELL_BOLTZMANN_EQ.to_string()];
        if self.bose {
            equations.push(BOSE_EQ.to_string());
        }
        if self.fermi {
            equations.push(FERMI_EQ.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["maxwell-boltzmann-statistics".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: THIRD_LAW.into(),
                statement: "Entropy tends to zero as temperature tends to zero.".into(),
                layer: "statistical".into(),
                class: "model-internal".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load a statistics encoding from a package. Knobs default; overlay
    /// them from a live gas when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        if pkg.id != "ideal-gas" {
            return Err(format!(
                "ideal-gas package id '{}' is not ideal-gas",
                pkg.id
            ));
        }
        let (bose, fermi) = parse_gas_statistics(pkg)?;
        Ok(Self {
            bose,
            fermi,
            ..Self::default()
        })
    }

    fn bose_equation() -> String {
        BOSE_EQ.to_string()
    }

    fn fermi_equation() -> String {
        FERMI_EQ.to_string()
    }

    /// Low-T entropy per particle in units of k. Classical Sackur–Tetrode
    /// is unbounded below (`∝ ln T`). Below T_c an ideal Bose gas has
    /// `S/Nk = (5/2) ζ(5/2)/ζ(3/2) (T/T_c)^{3/2} → 0`. A degenerate Fermi
    /// gas has `S/Nk = (π²/2) (T/T_F) → 0`.
    fn entropy_over_nk_at(&self, temperature_k: f64) -> f64 {
        if self.fermi {
            (PI * PI / 2.0) * (temperature_k / FERMI_TF_K)
        } else if self.bose && temperature_k < BOSE_TC_K {
            BOSE_S_PREF * (temperature_k / BOSE_TC_K).powf(1.5)
        } else {
            1.5 * temperature_k.ln()
        }
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
        "A monatomic ideal gas on the statistical layer. Equipartition \
         and the second law hold on the classical encoding; the third law \
         fails because Sackur–Tetrode entropy is unbounded below (S ∝ ln T). \
         Bose statistics are an IR mutation, not a temperature knob. \
         Degenerate Fermi statistics are a second IR mutation: Sommerfeld \
         C_V is not (3/2) N k. Neither fork is a silent Einstein-solid install."
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
            )
            .with_domain(equipartition_domain()),
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
            )
            .with_domain(third_law_domain()),
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
                    Verdict::fails(
                        claim,
                        format!(
                            "heat capacity is not (3/2) N k (C_v/(Nk) = {:.4})",
                            cv / (self.particles * k_boltzmann().value())
                        ),
                    )
                    .with_evidence([format!(
                        "Sommerfeld C_V = (π²/2) N k (T/T_F) at T = {} K, T_F = {FERMI_TF_K} K",
                        self.temperature_k
                    )])
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
                if self.fermi {
                    let t_probe = FERMI_TF_K * FERMI_THIRD_LAW_T_OVER_TF;
                    let s = self.entropy_over_nk_at(t_probe);
                    if s.abs() < THIRD_LAW_S_NK {
                        Verdict::holds(claim, "Fermi entropy S/Nk = (π²/2)(T/T_F) → 0 as T → 0")
                            .with_evidence([format!(
                                "S/Nk = {s:.3e} at T = T_F/400 = {t_probe:.3} K"
                            )])
                    } else {
                        Verdict::fails(claim, "Fermi entropy at the low-T probe is not vanishing")
                            .with_evidence([format!(
                                "S/Nk = {s:.3e} at T = T_F/400 = {t_probe:.3} K"
                            )])
                    }
                } else if self.bose {
                    let t_probe = BOSE_TC_K * THIRD_LAW_T_OVER_TC;
                    let s = self.entropy_over_nk_at(t_probe);
                    if s.abs() < THIRD_LAW_S_NK {
                        Verdict::holds(claim, "Bose entropy S/Nk ∝ (T/T_c)^{3/2} → 0 as T → 0")
                            .with_evidence([format!(
                                "S/Nk = {s:.3e} at T = T_c/40 = {t_probe:.3} K"
                            )])
                    } else {
                        Verdict::fails(claim, "Bose entropy at the low-T probe is not vanishing")
                            .with_evidence([format!(
                                "S/Nk = {s:.3e} at T = T_c/40 = {t_probe:.3} K"
                            )])
                    }
                } else {
                    // Classical S has a (3/2)Nk·ln T term with no lower bound.
                    let s_term = self.entropy_over_nk_at(self.temperature_k);
                    Verdict::fails(claim,
                        "classical ideal-gas entropy S ∝ (3/2) ln T → −∞ as T → 0; the third law needs quantum statistics",
                    )
                    .with_evidence([format!(
                        "the ln-T term is {s_term:.2} and diverges as T → 0"
                    )])
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by a thermodynamic object"),
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.bose = parsed.bose;
        fork.fermi = parsed.fermi;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mut out: Vec<(String, Box<dyn Theory>)> = Vec::new();
        if !self.bose {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::bose_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.bose {
                    let mut fork = self.clone();
                    fork.bose = true;
                    out.push(("add-bose".into(), Box::new(fork)));
                }
            }
        }
        if !self.fermi {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::fermi_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.fermi {
                    let mut fork = self.clone();
                    fork.fermi = true;
                    out.push(("add-fermi".into(), Box::new(fork)));
                }
            }
        }
        out
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
         row; only Debye holds thermo.debye-t3. Bose and Fermi statistics on the \
         ideal gas are IR (`add-bose` / `add-fermi` are IR forks, not temperature knobs).",
        vec![
            "`holds` / `fails` are internal to the encoding.".into(),
            "U, C_v and ΔS are computed; T and energy are distinct types.".into(),
            "The classical ideal gas and Dulong–Petit both fail the third law; Einstein and Debye hold it.".into(),
            "`thermo.debye-t3` fails for Einstein (exponential) and holds for Debye (phonon continuum).".into(),
            "`hypothesize ideal-gas`: add-bose and add-fermi are IR, not set.".into(),
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

    #[test]
    fn bose_statistics_is_ir_not_a_knob() {
        let mut g = IdealGas::default();
        assert!(
            IdealGas::default()
                .set("bose", KnobValue::Bool(true))
                .is_err(),
            "Bose statistics is an IR mutation, not a knob"
        );
        assert!(
            IdealGas::default()
                .set("quantum", KnobValue::Bool(true))
                .is_err(),
            "ideal-gas must not grow a quantum knob; that stays on Einstein-solid"
        );
        let src = render_package(&g.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            IdealGas::from_package(&pkg).unwrap(),
            g,
            "IR round-trip must preserve Maxwell-Boltzmann statistics"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(IdealGas::bose_equation()),
        );
        let parsed = IdealGas::from_package(&mutated).unwrap();
        assert!(parsed.bose);
        let mut fork = g.clone();
        fork.bose = true;
        assert_eq!(verdict(&fork, THIRD_LAW), VerdictKind::Holds);
        assert_eq!(verdict(&g, THIRD_LAW), VerdictKind::Fails);
        assert_eq!(verdict(&fork, EQUIPARTITION), VerdictKind::Holds);
        assert_eq!(verdict(&fork, SECOND_LAW), VerdictKind::Holds);
        g.set("volume_ratio", KnobValue::Float(0.5)).unwrap();
        assert_eq!(verdict(&g, SECOND_LAW), VerdictKind::Fails);
        assert_eq!(verdict(&g, THIRD_LAW), VerdictKind::Fails);
        let t_probe = BOSE_TC_K * THIRD_LAW_T_OVER_TC;
        let s = fork.entropy_over_nk_at(t_probe);
        assert!(
            s.abs() < THIRD_LAW_S_NK,
            "Bose S/Nk at T_c/40 must vanish, got {s}"
        );
        let probes = IdealGas::default().structural_mutations();
        assert_eq!(probes.len(), 2);
        assert!(
            probes.iter().any(|(label, _)| label == "add-bose"),
            "live gas must offer add-bose: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        assert!(
            probes.iter().any(|(label, _)| label == "add-fermi"),
            "live gas must offer add-fermi: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let bose_probe = probes
            .iter()
            .find(|(label, _)| label == "add-bose")
            .unwrap();
        assert_eq!(
            verdict(bose_probe.1.as_ref(), THIRD_LAW),
            VerdictKind::Holds
        );
        let bose_probes = fork.structural_mutations();
        assert!(
            bose_probes.iter().all(|(label, _)| label != "add-bose"),
            "bose fork must not re-offer add-bose"
        );
        assert!(
            bose_probes.iter().any(|(label, _)| label == "add-fermi"),
            "bose fork must still offer add-fermi"
        );
        let live = IdealGas::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), live.package());
        assert_eq!(
            rebuilt.get("temperature").unwrap(),
            KnobValue::Float(300.0),
            "reparse must overlay statistics IR onto live knobs"
        );
        assert_eq!(verdict(rebuilt.as_ref(), THIRD_LAW), VerdictKind::Fails);
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == THIRD_LAW)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "ideal-gas third law must name Sackur-Tetrode: {:?}",
            cell.domain()
        );
        let einstein = EinsteinSolid::einstein();
        let ecell = einstein
            .claims()
            .into_iter()
            .find(|c| c.id_str() == THIRD_LAW)
            .unwrap();
        assert!(
            ecell.domain().is_encoding_wide(),
            "Einstein-solid third law stays encoding-wide: {:?}",
            ecell.domain()
        );
        assert_eq!(verdict(&einstein, THIRD_LAW), VerdictKind::Holds);
        assert!(
            EinsteinSolid::einstein()
                .set("quantum", KnobValue::Bool(false))
                .is_ok(),
            "Einstein-solid keeps the quantum knob"
        );
        assert!(
            EinsteinSolid::einstein()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-fermi"),
            "einstein-solid must not grow add-fermi"
        );
    }

    #[test]
    fn fermi_statistics_is_ir_not_a_knob() {
        let mut g = IdealGas::default();
        assert!(
            IdealGas::default()
                .set("fermi", KnobValue::Bool(true))
                .is_err(),
            "Fermi statistics is an IR mutation, not a knob"
        );
        assert!(
            IdealGas::default()
                .set("fermi_temp", KnobValue::Float(1.0e4))
                .is_err(),
            "T_F is not a knob"
        );
        assert!(
            IdealGas::default()
                .set("quantum", KnobValue::Bool(true))
                .is_err(),
            "ideal-gas must not grow a quantum knob; that stays on Einstein-solid"
        );
        let src = render_package(&g.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            pkg.equations.len(),
            1,
            "live package must stay maxwell-boltzmann"
        );
        assert_eq!(pkg.equations[0], MAXWELL_BOLTZMANN_EQ);
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(IdealGas::fermi_equation()),
        );
        let parsed = IdealGas::from_package(&mutated).unwrap();
        assert!(parsed.fermi);
        let mut fork = g.clone();
        fork.fermi = true;
        assert_eq!(verdict(&fork, EQUIPARTITION), VerdictKind::Fails);
        assert_eq!(verdict(&g, EQUIPARTITION), VerdictKind::Holds);
        assert_eq!(verdict(&fork, THIRD_LAW), VerdictKind::Holds);
        assert_eq!(verdict(&g, THIRD_LAW), VerdictKind::Fails);
        assert_eq!(verdict(&fork, SECOND_LAW), VerdictKind::Holds);
        let cv_over_nk = fork.heat_capacity() / (fork.particles * k_boltzmann().value());
        let expected = (PI * PI / 2.0) * (fork.temperature_k / FERMI_TF_K);
        assert!(
            (cv_over_nk - expected).abs() < 1e-4,
            "Sommerfeld C_V/(Nk) must match (π²/2)(T/T_F) = {expected}, got {cv_over_nk}"
        );
        g.set("volume_ratio", KnobValue::Float(0.5)).unwrap();
        assert_eq!(verdict(&g, SECOND_LAW), VerdictKind::Fails);
        assert_eq!(verdict(&g, EQUIPARTITION), VerdictKind::Holds);
        let t_probe = FERMI_TF_K * FERMI_THIRD_LAW_T_OVER_TF;
        let s = fork.entropy_over_nk_at(t_probe);
        assert!(
            s.abs() < THIRD_LAW_S_NK,
            "Fermi S/Nk at T_F/400 must vanish, got {s}"
        );
        let probes = IdealGas::default().structural_mutations();
        let f = probes
            .iter()
            .find(|(label, _)| label == "add-fermi")
            .expect("add-fermi");
        assert_eq!(verdict(f.1.as_ref(), EQUIPARTITION), VerdictKind::Fails);
        let fermi_probes = fork.structural_mutations();
        assert!(
            fermi_probes.iter().all(|(l, _)| l != "add-fermi"),
            "fermi fork must not re-offer add-fermi"
        );
        assert!(
            fermi_probes.iter().any(|(l, _)| l == "add-bose"),
            "fermi fork must still offer add-bose"
        );
        let live = IdealGas::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), live.package());
        assert_eq!(
            rebuilt.get("temperature").unwrap(),
            KnobValue::Float(300.0),
            "reparse must overlay statistics IR onto live knobs"
        );
        assert_eq!(verdict(rebuilt.as_ref(), EQUIPARTITION), VerdictKind::Holds);
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == EQUIPARTITION)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "ideal-gas equipartition must name classical C_V: {:?}",
            cell.domain()
        );
        assert!(
            EinsteinSolid::einstein()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-fermi"),
            "einstein-solid must not grow add-fermi"
        );
        assert!(
            EinsteinSolid::einstein()
                .set("quantum", KnobValue::Bool(false))
                .is_ok(),
            "Einstein-solid keeps the quantum knob"
        );
        assert!(
            IdealGas::default()
                .set("temperature", KnobValue::Float(1.0))
                .is_ok(),
            "ideal-gas keeps the temperature knob"
        );
    }
}
