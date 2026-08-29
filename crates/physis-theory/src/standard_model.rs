//! Standard Model as an effective quantum field theory: empirically sharp,
//! UV-incomplete, many parameters.

use physis_core::claim::{Claim, Epistemic, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_model::{GaugeGroup, Manifold, Spectrum, World};

use crate::claims;
use crate::framework::Theory;

/// One generation of left-handed Weyl fermions as `(multiplicity, hypercharge Y)`
/// with the convention `Q = T₃ + Y`. Anomaly cancellation is a *computation*
/// over this content, not a stored fact.
const SM_GENERATION_WEYL: &[(f64, f64)] = &[
    (6.0, 1.0 / 6.0),  // quark doublet Q_L: 3 colours × 2 weak
    (3.0, -2.0 / 3.0), // anti-up  u_R^c: 3 colours
    (3.0, 1.0 / 3.0),  // anti-down d_R^c: 3 colours
    (2.0, -1.0 / 2.0), // lepton doublet L_L: 2 weak
    (1.0, 1.0),        // anti-electron e_R^c
];

/// Number of SU(2) doublets in one generation (3 quark colours + 1 lepton):
/// the Witten SU(2) global anomaly needs this to be even.
const SM_WEAK_DOUBLETS: u32 = 4;

/// Electric charge (in units of e/3) of a species by flavor, from the catalog.
fn charge_thirds(flavor: physis_model::Flavor) -> i32 {
    physis_model::Spectrum::standard_model()
        .species
        .iter()
        .find(|s| s.flavor == flavor)
        .map(|s| s.charge_thirds as i32)
        .unwrap_or(0)
}

/// Net charge of a hydrogen atom (proton `uud` + electron), in units of e/3.
/// Zero is charge quantization / atom neutrality, computed from the catalog.
fn hydrogen_charge_thirds() -> i32 {
    use physis_model::Flavor;
    let proton = 2 * charge_thirds(Flavor::Up) + charge_thirds(Flavor::Down);
    proton + charge_thirds(Flavor::Electron)
}

/// Sum of hypercharge over one generation ([grav]²U(1) and mixed anomalies).
fn hypercharge_sum() -> f64 {
    SM_GENERATION_WEYL.iter().map(|(m, y)| m * y).sum()
}

/// Sum of hypercharge cubed over one generation (the [U(1)]³ anomaly).
fn hypercharge_cube_sum() -> f64 {
    SM_GENERATION_WEYL.iter().map(|(m, y)| m * y * y * y).sum()
}

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "generations",
        layer: LayerId::Particle,
        doc: "Number of fermion generations. Nature: 3. This knob exists so agents can watch empirical claims flip.",
        domain: KnobDomain::UInt { min: 1, max: 4 },
    },
    KnobSpec {
        name: "include_higgs",
        layer: LayerId::Particle,
        doc: "Whether the Higgs scalar is in the spectrum.",
        domain: KnobDomain::Bool,
    },
    KnobSpec {
        name: "include_gravity",
        layer: LayerId::Field,
        doc: "SM as usually taught does not include gravity. Flip this to ask 'SM + graviton'.",
        domain: KnobDomain::Bool,
    },
    KnobSpec {
        name: "neutrino_masses",
        layer: LayerId::Particle,
        doc: "Whether neutrino masses are included. The minimal SM stores them as 0; oscillation experiments show they are nonzero.",
        domain: KnobDomain::Bool,
    },
];

/// The Standard Model of particle physics (effective QFT).
#[derive(Clone, Debug)]
pub struct StandardModel {
    generations: u8,
    include_higgs: bool,
    include_gravity: bool,
    neutrino_masses: bool,
}

impl Default for StandardModel {
    fn default() -> Self {
        Self {
            generations: 3,
            include_higgs: true,
            include_gravity: false,
            // The textbook minimal SM leaves neutrinos massless — a known lie.
            neutrino_masses: false,
        }
    }
}

impl StandardModel {
    fn spectrum(&self) -> Spectrum {
        let mut s = if self.include_gravity {
            Spectrum::standard_model_plus_graviton()
        } else {
            Spectrum::standard_model()
        };
        if !self.include_higgs {
            s.species
                .retain(|p| p.flavor != physis_model::Flavor::Higgs);
        }
        if self.generations < 3 {
            let drop_tau = self.generations < 3;
            let drop_muon = self.generations < 2;
            s.species.retain(|p| {
                if drop_muon
                    && matches!(
                        p.flavor,
                        physis_model::Flavor::Muon
                            | physis_model::Flavor::NuMu
                            | physis_model::Flavor::Charm
                            | physis_model::Flavor::Strange
                    )
                {
                    return false;
                }
                if drop_tau
                    && matches!(
                        p.flavor,
                        physis_model::Flavor::Tau
                            | physis_model::Flavor::NuTau
                            | physis_model::Flavor::Top
                            | physis_model::Flavor::Bottom
                    )
                {
                    return false;
                }
                true
            });
        }
        s
    }
}

impl Knobbed for StandardModel {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }

    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "generations" => Ok(KnobValue::UInt(self.generations as u64)),
            "include_higgs" => Ok(KnobValue::Bool(self.include_higgs)),
            "include_gravity" => Ok(KnobValue::Bool(self.include_gravity)),
            "neutrino_masses" => Ok(KnobValue::Bool(self.neutrino_masses)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }

    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("generations", KnobValue::UInt(v)) => self.generations = v as u8,
            ("include_higgs", KnobValue::Bool(v)) => self.include_higgs = v,
            ("include_gravity", KnobValue::Bool(v)) => self.include_gravity = v,
            ("neutrino_masses", KnobValue::Bool(v)) => self.neutrino_masses = v,
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

impl Theory for StandardModel {
    fn id(&self) -> &'static str {
        "standard-model"
    }
    fn name(&self) -> &'static str {
        "Standard Model"
    }
    fn summary(&self) -> &'static str {
        "SU(3)×SU(2)×U(1) quantum field theory of observed particles. \
         Empirically unmatched below the electroweak scale. Not a theory of gravity \
         and not a UV completion. ~19 free parameters."
    }

    fn world(&self) -> Option<World> {
        Some(World {
            spacetime: Manifold::observed_4d(),
            gauge: GaugeGroup::standard_model(),
            spectrum: self.spectrum(),
            has_gravity: self.include_gravity,
            supersymmetric: false,
            free_parameter_count: 19,
            landscape_log10: 0.0,
            note: format!(
                "SM generations={} higgs={} gravity={} neutrino_masses={}",
                self.generations, self.include_higgs, self.include_gravity, self.neutrino_masses
            ),
        })
    }

    fn claims(&self) -> Vec<Claim> {
        vec![
            claims::c(
                claims::SPACETIME_STRUCTURE,
                "3+1 Minkowski spacetime, no extra dimensions.",
                LayerId::Spacetime,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::OBSERVED_4D,
                "Macroscopic spacetime is 3+1.",
                LayerId::Spacetime,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::HIDDEN_EXTRA_DIMS,
                "No extra dimensions in the SM as an effective theory.",
                LayerId::Spacetime,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::FERMIONS,
                "Quarks and leptons exist.",
                LayerId::Particle,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::SM_GAUGE,
                "Gauge group is exactly the Standard Model.",
                LayerId::Interaction,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::ANOMALY_CANCELLATION,
                "Chiral gauge anomalies cancel within each generation.",
                LayerId::Interaction,
                Epistemic::Theorem,
            ),
            claims::c(
                claims::THREE_GENERATIONS,
                "Three generations of fermions.",
                LayerId::Particle,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::NEUTRINO_MASSES,
                "Neutrinos have nonzero mass.",
                LayerId::Particle,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::CHARGE_QUANTIZATION,
                "Electric charge is quantized so that atoms are exactly neutral.",
                LayerId::Particle,
                Epistemic::Theorem,
            ),
            claims::c(
                claims::GRAVITY,
                "Gravity is part of the Standard Model.",
                LayerId::Field,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::UNIQUE_VACUUM,
                "The SM vacuum (given its parameters) is the one we use; no string landscape.",
                LayerId::Effective,
                Epistemic::Heuristic,
            ),
            claims::c(
                claims::FEW_PARAMETERS,
                "The theory has few free parameters.",
                LayerId::Interaction,
                Epistemic::Heuristic,
            ),
            claims::c(
                claims::UV_COMPLETION,
                "The Standard Model is a UV-complete theory of nature.",
                LayerId::Field,
                Epistemic::EncodedFact,
            ),
        ]
    }

    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id.0.as_str() {
            claims::SPACETIME_STRUCTURE | claims::OBSERVED_4D | claims::HIDDEN_EXTRA_DIMS => {
                Verdict::holds(Epistemic::EncodedFact, "SM is formulated in 3+1 Minkowski")
            }
            claims::FERMIONS => Verdict::holds(Epistemic::EncodedFact, "quarks and leptons"),
            claims::SM_GAUGE => Verdict::holds(Epistemic::EncodedFact, "SU(3)×SU(2)×U(1)"),
            claims::ANOMALY_CANCELLATION => {
                let sy = hypercharge_sum();
                let sy3 = hypercharge_cube_sum();
                if sy.abs() < 1e-12 && sy3.abs() < 1e-12 && SM_WEAK_DOUBLETS % 2 == 0 {
                    Verdict::holds(
                        Epistemic::Theorem,
                        "SM chiral anomalies cancel within each generation",
                    )
                    .with_evidence([
                        format!("computed over one generation: ΣY = {sy:.3}, ΣY³ = {sy3:.3} (both 0)"),
                        format!("Witten SU(2): {SM_WEAK_DOUBLETS} doublets (even)"),
                    ])
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        format!("anomaly not cancelled: ΣY = {sy:.3}, ΣY³ = {sy3:.3}"),
                    )
                }
            }
            claims::THREE_GENERATIONS => {
                if self.generations == 3 {
                    Verdict::holds(Epistemic::EncodedFact, "three generations")
                } else {
                    Verdict::fails(
                        Epistemic::EncodedFact,
                        format!("generations = {}, not 3", self.generations),
                    )
                }
            }
            claims::NEUTRINO_MASSES => {
                if self.neutrino_masses {
                    Verdict::holds(
                        Epistemic::EncodedFact,
                        "neutrino masses included (beyond the minimal SM, e.g. via a seesaw)",
                    )
                } else {
                    Verdict::fails(
                        Epistemic::EncodedFact,
                        "minimal SM stores neutrino masses as 0, but oscillations prove they are nonzero",
                    )
                }
            }
            claims::CHARGE_QUANTIZATION => {
                let h = hydrogen_charge_thirds();
                if h == 0 {
                    Verdict::holds(
                        Epistemic::Theorem,
                        "a hydrogen atom (uud + e⁻) is exactly neutral",
                    )
                    .with_evidence([
                        "computed from the catalog: 2·Q(u) + Q(d) + Q(e⁻) = 0 (units of e/3)".to_string(),
                    ])
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        format!("hydrogen net charge = {h}/3 ≠ 0"),
                    )
                }
            }
            claims::GRAVITY => {
                if self.include_gravity {
                    Verdict::holds(
                        Epistemic::Heuristic,
                        "graviton added by hand; not a UV completion of gravity",
                    )
                } else {
                    Verdict::fails(
                        Epistemic::EncodedFact,
                        "the Standard Model does not contain gravity",
                    )
                }
            }
            claims::UNIQUE_VACUUM => Verdict::holds(
                Epistemic::Heuristic,
                "no landscape; parameters are inputs, not scanned vacua",
            ),
            claims::FEW_PARAMETERS => Verdict::fails(
                Epistemic::Heuristic,
                "≈19 free parameters; not few by the standard this lab uses",
            ),
            claims::UV_COMPLETION => Verdict::fails(
                Epistemic::EncodedFact,
                "SM is an effective theory: Landau poles, triviality, no gravity, no dark matter, no neutrino masses in the minimal form",
            ),
            claims::CRITICAL_DIMENSION | claims::SUSY_CONSTRUCTION | claims::NO_TACHYON => {
                Verdict::inapplicable("not a worldsheet theory")
            }
            _ => Verdict::inapplicable("claim not made by the Standard Model object"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;

    #[test]
    fn default_sm_has_three_generations_no_gravity() {
        let t = StandardModel::default();
        let gens = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == claims::THREE_GENERATIONS)
            .unwrap();
        assert_eq!(t.evaluate(&gens).kind, VerdictKind::Holds);
        let g = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == claims::GRAVITY)
            .unwrap();
        assert_eq!(t.evaluate(&g).kind, VerdictKind::Fails);
    }

    #[test]
    fn neutrino_masses_are_a_known_sm_gap() {
        let verdict = |t: &StandardModel| {
            let c = t
                .claims()
                .into_iter()
                .find(|c| c.id.0 == claims::NEUTRINO_MASSES)
                .unwrap();
            t.evaluate(&c).kind
        };
        let mut t = StandardModel::default();
        // The minimal SM leaves neutrinos massless: a known empirical failure.
        assert_eq!(verdict(&t), VerdictKind::Fails);
        t.set("neutrino_masses", KnobValue::Bool(true)).unwrap();
        assert_eq!(verdict(&t), VerdictKind::Holds);
    }

    #[test]
    fn sm_cancels_anomalies() {
        let t = StandardModel::default();
        let c = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == claims::ANOMALY_CANCELLATION)
            .unwrap();
        let v = t.evaluate(&c);
        assert_eq!(v.kind, VerdictKind::Holds);
        // Now a computed theorem, not a stored fact.
        assert_eq!(v.epistemic, Epistemic::Theorem);
    }

    #[test]
    fn hydrogen_is_neutral_by_computation() {
        assert_eq!(hydrogen_charge_thirds(), 0);
        let t = StandardModel::default();
        let c = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == claims::CHARGE_QUANTIZATION)
            .unwrap();
        let v = t.evaluate(&c);
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_eq!(v.epistemic, Epistemic::Theorem);
    }

    #[test]
    fn hypercharge_sums_vanish_over_a_generation() {
        // The actual arithmetic behind anomaly cancellation.
        assert!(hypercharge_sum().abs() < 1e-12);
        assert!(hypercharge_cube_sum().abs() < 1e-12);
        assert_eq!(SM_WEAK_DOUBLETS % 2, 0);
    }

    #[test]
    fn dropping_a_generation_fails() {
        let mut t = StandardModel::default();
        t.set("generations", KnobValue::UInt(2)).unwrap();
        let gens = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == claims::THREE_GENERATIONS)
            .unwrap();
        assert_eq!(t.evaluate(&gens).kind, VerdictKind::Fails);
    }
}
