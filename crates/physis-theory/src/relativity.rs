//! General relativity as a classical spacetime theory.

use physis_core::claim::{Claim, Epistemic, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_model::{GaugeGroup, Manifold, Signature, Spectrum, Topology, World};

use crate::claims;
use crate::framework::Theory;

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "dim",
        layer: LayerId::Spacetime,
        doc: "Spacetime dimension. Empirical GR is 4. Higher-D GR is a well-defined classical theory.",
        domain: KnobDomain::UInt { min: 2, max: 26 },
    },
    KnobSpec {
        name: "cosmological_constant",
        layer: LayerId::Spacetime,
        doc: "Λ in Planck units (order-of-magnitude knob, not a precision cosmology fit).",
        domain: KnobDomain::Float {
            min: -1.0,
            max: 1.0,
        },
    },
];

/// Einstein gravity.
#[derive(Clone, Debug)]
pub struct GeneralRelativity {
    dim: u8,
    cosmological_constant: f64,
}

impl Default for GeneralRelativity {
    fn default() -> Self {
        Self {
            dim: 4,
            cosmological_constant: 0.0,
        }
    }
}

impl Knobbed for GeneralRelativity {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }

    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "dim" => Ok(KnobValue::UInt(self.dim as u64)),
            "cosmological_constant" => Ok(KnobValue::Float(self.cosmological_constant)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }

    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("dim", KnobValue::UInt(v)) => self.dim = v as u8,
            ("cosmological_constant", KnobValue::Float(v)) => self.cosmological_constant = v,
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

impl GeneralRelativity {
    fn build_world(&self) -> World {
        let space = self.dim.saturating_sub(1);
        World {
            spacetime: Manifold {
                dim: self.dim,
                signature: Signature { time: 1, space },
                compact_extra: 0,
                compact_radius_planck: 0.0,
                topology: Topology::Minkowski,
                convention: physis_model::SignConvention::MostlyMinus,
            },
            gauge: GaugeGroup::trivial(),
            spectrum: {
                let mut s = Spectrum::empty();
                s.species.push(physis_model::Species::graviton());
                s
            },
            has_gravity: true,
            supersymmetric: false,
            free_parameter_count: 2, // G and Λ, roughly
            landscape_log10: 0.0,
            note: format!("GR in D={} Λ={}", self.dim, self.cosmological_constant),
        }
    }
}

impl Theory for GeneralRelativity {
    fn id(&self) -> &'static str {
        "general-relativity"
    }
    fn name(&self) -> &'static str {
        "General relativity"
    }
    fn summary(&self) -> &'static str {
        "Classical dynamical spacetime. Matches gravity from tabletop to cosmology. \
         Not a quantum theory. Not a theory of the Standard Model spectrum."
    }

    fn world(&self) -> Option<World> {
        Some(self.build_world())
    }

    fn claims(&self) -> Vec<Claim> {
        vec![
            claims::c(
                claims::SPACETIME_STRUCTURE,
                "Lorentzian manifold of the chosen dimension.",
                LayerId::Spacetime,
                Epistemic::Theorem,
            ),
            claims::c(
                claims::OBSERVED_4D,
                "Spacetime dimension is 4.",
                LayerId::Spacetime,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::GRAVITY,
                "Gravity is dynamical spacetime.",
                LayerId::Spacetime,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::FERMIONS,
                "GR contains the Standard Model fermions.",
                LayerId::Particle,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::SM_GAUGE,
                "GR contains the Standard Model gauge group.",
                LayerId::Interaction,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::UV_COMPLETION,
                "GR is UV-complete as a quantum theory.",
                LayerId::Quantum,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::UNIQUE_VACUUM,
                "Einstein gravity plus Λ is a unique classical theory (not a landscape).",
                LayerId::Spacetime,
                Epistemic::Heuristic,
            ),
        ]
    }

    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id.0.as_str() {
            claims::SPACETIME_STRUCTURE => {
                if self.build_world().spacetime.structurally_ok() {
                    Verdict::holds(Epistemic::Theorem, "Lorentzian, consistent dim")
                } else {
                    Verdict::fails(Epistemic::Theorem, "inconsistent manifold numbers")
                }
            }
            claims::OBSERVED_4D => {
                if self.dim == 4 {
                    Verdict::holds(Epistemic::EncodedFact, "D=4")
                } else {
                    Verdict::fails(Epistemic::EncodedFact, format!("D={}, not 4", self.dim))
                }
            }
            claims::GRAVITY => Verdict::holds(Epistemic::EncodedFact, "Einstein-Hilbert gravity"),
            claims::FERMIONS | claims::SM_GAUGE => Verdict::fails(
                Epistemic::EncodedFact,
                "GR has no Standard Model matter content",
            ),
            claims::UV_COMPLETION => Verdict::fails(
                Epistemic::EncodedFact,
                "perturbative quantum GR is not renormalizable; not a UV completion",
            ),
            claims::UNIQUE_VACUUM => Verdict::holds(
                Epistemic::Heuristic,
                "classical GR is a unique theory given D and Λ, not a landscape of 10^500 vacua",
            ),
            _ => Verdict::inapplicable("claim not made by the GR object"),
        }
    }
}
