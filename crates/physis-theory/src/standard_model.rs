//! Standard Model as an effective quantum field theory: empirically sharp,
//! UV-incomplete, many parameters.

use physis_core::claim::{Claim, Epistemic, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_model::{GaugeGroup, Manifold, Spectrum, World};

use crate::claims;
use crate::framework::Theory;

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
];

/// The Standard Model of particle physics (effective QFT).
#[derive(Clone, Debug)]
pub struct StandardModel {
    generations: u8,
    include_higgs: bool,
    include_gravity: bool,
}

impl Default for StandardModel {
    fn default() -> Self {
        Self {
            generations: 3,
            include_higgs: true,
            include_gravity: false,
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

    fn world(&self) -> World {
        World {
            spacetime: Manifold::observed_4d(),
            gauge: GaugeGroup::standard_model(),
            spectrum: self.spectrum(),
            has_gravity: self.include_gravity,
            supersymmetric: false,
            free_parameter_count: 19,
            landscape_log10: 0.0,
            note: format!(
                "SM generations={} higgs={} gravity={}",
                self.generations, self.include_higgs, self.include_gravity
            ),
        }
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
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::THREE_GENERATIONS,
                "Three generations of fermions.",
                LayerId::Particle,
                Epistemic::EncodedFact,
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
            claims::ANOMALY_CANCELLATION => Verdict::holds(
                Epistemic::EncodedFact,
                "each SM generation is anomaly-free (hypercharge trace and Witten SU(2) conditions)",
            ),
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
    fn sm_cancels_anomalies() {
        let t = StandardModel::default();
        let c = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == claims::ANOMALY_CANCELLATION)
            .unwrap();
        assert_eq!(t.evaluate(&c).kind, VerdictKind::Holds);
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
