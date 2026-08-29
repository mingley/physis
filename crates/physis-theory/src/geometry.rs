//! Observer-geometry: a scaffold for *unique-geometry* unification programs.
//!
//! This is **not** Geometric Unity, not a quantization of it, and not a
//! claim that Eric Weinstein's program is correct. It exists so that
//! agents can compare a *shape of theory* — "start from geometry, demand
//! uniqueness, try to derive the gauge group" — against string constructions
//! whose distinctive failure mode (in this lab) is a landscape of vacua.
//!
//! The total dimension is not a magic number: it is `observed_dim + fibre_dim`.
//! The default `fibre_dim = 10` is the *smallest* fibre that can host the
//! conjectured Spin(10) gauge group (which acts on a 10-dimensional space), so
//! the default total `14 = 4 + 10` is a toy constraint — the minimal geometric
//! room for the assignment — rather than an unexplained choice. It is still a
//! scaffold, not a derivation of Geometric Unity.

use physis_core::claim::{Claim, Epistemic, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_model::{GaugeGroup, Manifold, Signature, Spectrum, Topology, World};

use crate::claims;
use crate::framework::Theory;

/// Minimal fibre dimension that can host the conjectured Spin(10) gauge group.
const SPIN10_MIN_FIBRE: u8 = 10;

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "fibre_dim",
        layer: LayerId::Spacetime,
        doc: "Dimension of the internal fibre over observed spacetime. Total dimension is observed_dim + fibre_dim. Default 10 is the minimal fibre that can carry Spin(10).",
        domain: KnobDomain::UInt { min: 0, max: 22 },
    },
    KnobSpec {
        name: "observed_dim",
        layer: LayerId::Spacetime,
        doc: "Observed spacetime dimension. Empirical target: 4.",
        domain: KnobDomain::UInt { min: 1, max: 26 },
    },
    KnobSpec {
        name: "derive_gauge",
        layer: LayerId::Interaction,
        doc: "If true, pretends the gauge group is an output (assigned Spin(10) as a conjecture, not a proof).",
        domain: KnobDomain::Bool,
    },
    KnobSpec {
        name: "unique_vacuum",
        layer: LayerId::Mathematical,
        doc: "Program-level demand: the geometry selects one vacuum. This is an axiom of the program, not a theorem.",
        domain: KnobDomain::Bool,
    },
];

/// Unique-geometry unification scaffold.
#[derive(Clone, Debug)]
pub struct ObserverGeometry {
    fibre_dim: u8,
    observed_dim: u8,
    derive_gauge: bool,
    unique_vacuum: bool,
}

impl Default for ObserverGeometry {
    fn default() -> Self {
        Self {
            fibre_dim: SPIN10_MIN_FIBRE,
            observed_dim: 4,
            derive_gauge: true,
            unique_vacuum: true,
        }
    }
}

impl ObserverGeometry {
    /// Total geometric dimension: observed spacetime plus the internal fibre.
    fn total_dim(&self) -> u8 {
        self.observed_dim.saturating_add(self.fibre_dim)
    }

    /// Whether the (conjectural) Spin(10) assignment has geometric room: the
    /// fibre must be at least 10-dimensional for Spin(10) to act on it.
    fn fibre_can_host_spin10(&self) -> bool {
        self.fibre_dim >= SPIN10_MIN_FIBRE
    }

    fn build_world(&self) -> World {
        let total = self.total_dim();
        let extra = self.fibre_dim;
        let space = total.saturating_sub(1);
        World {
            spacetime: Manifold {
                dim: total,
                signature: Signature { time: 1, space },
                compact_extra: extra,
                compact_radius_planck: if extra == 0 { 0.0 } else { 1.0 },
                topology: Topology::Unspecified,
                convention: physis_model::SignConvention::MostlyPlus,
            },
            gauge: self.gauge(),
            spectrum: Spectrum::standard_model_plus_graviton(),
            has_gravity: true,
            supersymmetric: false,
            free_parameter_count: if self.unique_vacuum { 1 } else { 40 },
            landscape_log10: if self.unique_vacuum { 0.0 } else { 12.0 },
            note: format!(
                "observer-geometry D={}=({}+{}) derive_gauge={} unique={}",
                total, self.observed_dim, self.fibre_dim, self.derive_gauge, self.unique_vacuum
            ),
        }
    }

    fn gauge(&self) -> GaugeGroup {
        if self.derive_gauge {
            GaugeGroup::spin10()
        } else {
            GaugeGroup::standard_model()
        }
    }
}

impl Knobbed for ObserverGeometry {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }

    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "fibre_dim" => Ok(KnobValue::UInt(self.fibre_dim as u64)),
            "observed_dim" => Ok(KnobValue::UInt(self.observed_dim as u64)),
            "derive_gauge" => Ok(KnobValue::Bool(self.derive_gauge)),
            "unique_vacuum" => Ok(KnobValue::Bool(self.unique_vacuum)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }

    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("fibre_dim", KnobValue::UInt(v)) => self.fibre_dim = v as u8,
            ("observed_dim", KnobValue::UInt(v)) => self.observed_dim = v as u8,
            ("derive_gauge", KnobValue::Bool(v)) => self.derive_gauge = v,
            ("unique_vacuum", KnobValue::Bool(v)) => self.unique_vacuum = v,
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

impl Theory for ObserverGeometry {
    fn id(&self) -> &'static str {
        "observer-geometry"
    }
    fn name(&self) -> &'static str {
        "Observer geometry (scaffold)"
    }
    fn summary(&self) -> &'static str {
        "A typed scaffold for programs that try to derive particle physics from a unique \
         geometric starting point. Encodes the *shape* of landscape critiques. \
         Does not implement Geometric Unity. Gauge 'derivation' is a conjectural assignment \
         (Spin(10)), labelled as such."
    }

    fn world(&self) -> Option<World> {
        Some(self.build_world())
    }

    fn claims(&self) -> Vec<Claim> {
        vec![
            claims::c(
                claims::SPACETIME_STRUCTURE,
                "Geometric dimension and observed dimension are consistent.",
                LayerId::Spacetime,
                Epistemic::Theorem,
            ),
            claims::c(
                claims::CRITICAL_DIMENSION,
                "A worldsheet critical dimension applies.",
                LayerId::Spacetime,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::OBSERVED_4D,
                "Observed spacetime is 3+1.",
                LayerId::Spacetime,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::FERMIONS,
                "Fermions arise from the geometry.",
                LayerId::Particle,
                Epistemic::Conjecture,
            ),
            claims::c(
                claims::SM_GAUGE,
                "The Standard Model gauge group is derived, not postulated.",
                LayerId::Interaction,
                Epistemic::Conjecture,
            ),
            claims::c(
                claims::THREE_GENERATIONS,
                "Three generations are selected by the geometry.",
                LayerId::Particle,
                Epistemic::Open,
            ),
            claims::c(
                claims::GRAVITY,
                "Gravity is geometric (metric / Einstein sector).",
                LayerId::Spacetime,
                Epistemic::Conjecture,
            ),
            claims::c(
                claims::UNIQUE_VACUUM,
                "The construction selects a unique vacuum.",
                LayerId::Mathematical,
                Epistemic::Conjecture,
            ),
            claims::c(
                claims::UV_COMPLETION,
                "The construction is a UV completion of gravity plus matter.",
                LayerId::Field,
                Epistemic::Open,
            ),
        ]
    }

    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id.0.as_str() {
            claims::SPACETIME_STRUCTURE => {
                if self.build_world().spacetime.structurally_ok() {
                    Verdict::holds(Epistemic::Theorem, "dimension numbers fit")
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        "spacetime numbers are not internally consistent",
                    )
                    .with_evidence([format!(
                        "observed={} fibre={} total={}",
                        self.observed_dim,
                        self.fibre_dim,
                        self.total_dim()
                    )])
                }
            }
            claims::CRITICAL_DIMENSION => Verdict::inapplicable(
                "observer-geometry is not a worldsheet theory; it has no Polyakov conformal anomaly",
            ),
            claims::OBSERVED_4D => {
                if self.observed_dim == 4 {
                    Verdict::holds(Epistemic::EncodedFact, "observed_dim = 4")
                } else {
                    Verdict::fails(
                        Epistemic::EncodedFact,
                        format!("observed_dim = {}", self.observed_dim),
                    )
                }
            }
            claims::FERMIONS => Verdict::undecidable(
                Epistemic::Conjecture,
                "fermions are *assumed* in the projected spectrum; they are not derived in this encoding",
            ),
            claims::SM_GAUGE => {
                if !self.derive_gauge {
                    Verdict::fails(
                        Epistemic::Conjecture,
                        "derive_gauge is off: SM is postulated, which is the thing this program wanted to avoid",
                    )
                } else if !self.fibre_can_host_spin10() {
                    // Toy geometric constraint: Spin(10) acts on R^10, so a
                    // fibre smaller than 10 has no room for the assignment.
                    Verdict::fails(
                        Epistemic::Conjecture,
                        format!(
                            "fibre_dim = {} < {} has no geometric room for Spin(10)",
                            self.fibre_dim, SPIN10_MIN_FIBRE
                        ),
                    )
                    .with_evidence([
                        "this is why the default fibre is 10 (hence total 14 = 4 + 10): the minimal carrier of Spin(10)".to_string(),
                    ])
                } else if self.gauge().sm_embed().contains_sm() {
                    let chain = self
                        .gauge()
                        .verified_contains_sm()
                        .unwrap_or_default()
                        .join(" ⊃ ");
                    Verdict::holds(
                        Epistemic::Conjecture,
                        "Spin(10) is assigned as a derived group and does contain SM — assignment, not a proof",
                    )
                    .with_evidence([
                        format!("fibre_dim = {} ≥ {} can host Spin(10)", self.fibre_dim, SPIN10_MIN_FIBRE),
                        format!("verified embedding chain: {chain}"),
                        "replace this assignment with an actual geometric derivation before treating it as a theorem".to_string(),
                    ])
                } else {
                    Verdict::fails(
                        Epistemic::Conjecture,
                        "derived group does not contain SM in this encoding",
                    )
                }
            }
            claims::THREE_GENERATIONS => Verdict::undecidable(
                Epistemic::Open,
                "generation count is not selected by any encoded geometric rule yet",
            ),
            claims::GRAVITY => Verdict::holds(
                Epistemic::Conjecture,
                "a metric sector is part of the scaffold; not derived from a theorem here",
            ),
            claims::UNIQUE_VACUUM => {
                if self.unique_vacuum {
                    Verdict::holds(
                        Epistemic::Conjecture,
                        "uniqueness is an axiom of this program (knob unique_vacuum=true), not a computed theorem",
                    )
                    .with_evidence([
                        "this is the contrast class for the string landscape, not evidence that geometry has succeeded",
                    ])
                } else {
                    Verdict::fails(
                        Epistemic::Conjecture,
                        "unique_vacuum knob is off; the program has dropped its distinctive demand",
                    )
                }
            }
            claims::UV_COMPLETION => Verdict::undecidable(
                Epistemic::Open,
                "no quantum construction is encoded; UV completeness is open",
            ),
            claims::SUSY_CONSTRUCTION | claims::NO_TACHYON | claims::HIDDEN_EXTRA_DIMS => {
                Verdict::inapplicable("not a string construction")
            }
            _ => Verdict::inapplicable("claim not made by observer-geometry"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;

    #[test]
    fn uniqueness_is_conjectural_hold() {
        let t = ObserverGeometry::default();
        let c = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == claims::UNIQUE_VACUUM)
            .unwrap();
        let v = t.evaluate(&c);
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_eq!(v.epistemic, Epistemic::Conjecture);
    }

    #[test]
    fn no_critical_dimension() {
        let t = ObserverGeometry::default();
        let c = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == claims::CRITICAL_DIMENSION)
            .unwrap();
        assert_eq!(t.evaluate(&c).kind, VerdictKind::Inapplicable);
    }

    fn verdict(t: &ObserverGeometry, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id.0 == id).unwrap();
        t.evaluate(&c).kind
    }

    #[test]
    fn total_dimension_is_composed_not_magic() {
        // 14 is not a literal: it is observed (4) + fibre (10).
        let t = ObserverGeometry::default();
        assert_eq!(t.total_dim(), 14);
        assert_eq!(t.observed_dim, 4);
        assert_eq!(t.fibre_dim, 10);
    }

    #[test]
    fn shrinking_the_fibre_below_ten_starves_the_gauge_assignment() {
        // The toy constraint: Spin(10) needs a fibre of at least 10 dimensions.
        let mut t = ObserverGeometry::default();
        assert_eq!(verdict(&t, claims::SM_GAUGE), VerdictKind::Holds);
        t.set("fibre_dim", KnobValue::UInt(9)).unwrap();
        assert_eq!(verdict(&t, claims::SM_GAUGE), VerdictKind::Fails);
        // Restoring the minimal fibre restores the (conjectural) assignment.
        t.set("fibre_dim", KnobValue::UInt(10)).unwrap();
        assert_eq!(verdict(&t, claims::SM_GAUGE), VerdictKind::Holds);
    }
}
