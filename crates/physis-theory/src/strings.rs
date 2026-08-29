//! Superstring / bosonic string / M-theory as knobbed theories.
//!
//! Critical dimensions (26, 10, 11) are theorems of the worldsheet
//! (or membrane) conformal anomaly, encoded here as `Epistemic::Theorem`.
//! Landscape counts are `Heuristic`. SM embeddings are `EncodedFact`.
//!
//! This is a laboratory object, not a compactification engine.

use physis_core::claim::{Claim, Epistemic, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_model::{GaugeGroup, Manifold, Signature, Spectrum, Topology, World};

use crate::claims;
use crate::framework::Theory;

/// Which string / M construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StringKind {
    /// 26D bosonic string.
    Bosonic,
    /// Type I (open + closed, SO(32), N=1).
    TypeI,
    /// Type IIA (closed, non-chiral N=2).
    TypeIIA,
    /// Type IIB (closed, chiral N=2).
    TypeIIB,
    /// Heterotic SO(32).
    HeteroticSO32,
    /// Heterotic E₈×E₈.
    HeteroticE8xE8,
    /// 11D M-theory.
    MTheory,
}

impl StringKind {
    /// CLI / knob token.
    pub const fn as_str(self) -> &'static str {
        match self {
            StringKind::Bosonic => "bosonic",
            StringKind::TypeI => "type-i",
            StringKind::TypeIIA => "type-iia",
            StringKind::TypeIIB => "type-iib",
            StringKind::HeteroticSO32 => "heterotic-so32",
            StringKind::HeteroticE8xE8 => "heterotic-e8e8",
            StringKind::MTheory => "m-theory",
        }
    }

    /// Parse token.
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "bosonic" => StringKind::Bosonic,
            "type-i" => StringKind::TypeI,
            "type-iia" => StringKind::TypeIIA,
            "type-iib" => StringKind::TypeIIB,
            "heterotic-so32" => StringKind::HeteroticSO32,
            "heterotic-e8e8" => StringKind::HeteroticE8xE8,
            "m-theory" => StringKind::MTheory,
            _ => return None,
        })
    }

    /// All tokens.
    pub const ALL: [&'static str; 7] = [
        "bosonic",
        "type-i",
        "type-iia",
        "type-iib",
        "heterotic-so32",
        "heterotic-e8e8",
        "m-theory",
    ];

    /// Critical spacetime dimension (theorem of the construction).
    pub const fn critical_dim(self) -> u8 {
        match self {
            StringKind::Bosonic => 26,
            StringKind::MTheory => 11,
            _ => 10,
        }
    }

    /// Construction uses supersymmetry as a structural ingredient.
    pub const fn requires_susy(self) -> bool {
        !matches!(self, StringKind::Bosonic)
    }

    /// Default gauge group of the 10D/11D theory (before breaking).
    pub fn fundamental_gauge(self) -> GaugeGroup {
        match self {
            StringKind::Bosonic => GaugeGroup::trivial(),
            StringKind::TypeI | StringKind::HeteroticSO32 => GaugeGroup::so32(),
            StringKind::TypeIIA | StringKind::TypeIIB | StringKind::MTheory => {
                GaugeGroup::trivial()
            }
            StringKind::HeteroticE8xE8 => GaugeGroup::e8e8(),
        }
    }

    /// Closed strings contain a graviton: yes for all of these.
    pub const fn has_closed_strings(self) -> bool {
        true
    }
}

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "kind",
        layer: LayerId::Field,
        doc: "Which string/M construction (sets critical dimension and default gauge).",
        domain: KnobDomain::Choice(&StringKind::ALL),
    },
    KnobSpec {
        name: "total_dim",
        layer: LayerId::Spacetime,
        doc: "Total spacetime dimension D. Superstring theorem: D=10; bosonic D=26; M D=11.",
        domain: KnobDomain::UInt { min: 2, max: 32 },
    },
    KnobSpec {
        name: "observed_dim",
        layer: LayerId::Spacetime,
        doc: "Non-compact macroscopic dimension. Empirical target: 4.",
        domain: KnobDomain::UInt { min: 1, max: 32 },
    },
    KnobSpec {
        name: "compact_radius_planck",
        layer: LayerId::Spacetime,
        doc: "Compactification radius in Planck lengths. O(1) hides extra dims; huge radii would be seen.",
        domain: KnobDomain::Float {
            min: 1e-6,
            max: 1e40,
        },
    },
    KnobSpec {
        name: "supersymmetry",
        layer: LayerId::Field,
        doc: "Whether the construction includes spacetime supersymmetry.",
        domain: KnobDomain::Bool,
    },
    KnobSpec {
        name: "flux_bits",
        layer: LayerId::Interaction,
        doc: "Heuristic bits of flux/moduli data contributing to a landscape count.",
        domain: KnobDomain::UInt { min: 0, max: 10_000 },
    },
];

/// A knobbed string / M-theory object.
#[derive(Clone, Debug)]
pub struct StringTheory {
    kind: StringKind,
    total_dim: u8,
    observed_dim: u8,
    compact_radius_planck: f64,
    supersymmetry: bool,
    flux_bits: u32,
}

impl StringTheory {
    /// Type IIB at its critical dimension, 6 extra dims at 1 Planck length.
    pub fn type_iib() -> Self {
        Self {
            kind: StringKind::TypeIIB,
            total_dim: 10,
            observed_dim: 4,
            compact_radius_planck: 1.0,
            supersymmetry: true,
            flux_bits: 200,
        }
    }

    /// Heterotic E₈×E₈, the usual SM-embedding story.
    pub fn heterotic_e8() -> Self {
        Self {
            kind: StringKind::HeteroticE8xE8,
            total_dim: 10,
            observed_dim: 4,
            compact_radius_planck: 1.0,
            supersymmetry: true,
            flux_bits: 80,
        }
    }

    /// Bosonic string at D=26.
    pub fn bosonic() -> Self {
        Self {
            kind: StringKind::Bosonic,
            total_dim: 26,
            observed_dim: 4,
            compact_radius_planck: 1.0,
            supersymmetry: false,
            flux_bits: 20,
        }
    }

    /// M-theory at D=11.
    pub fn m_theory() -> Self {
        Self {
            kind: StringKind::MTheory,
            total_dim: 11,
            observed_dim: 4,
            compact_radius_planck: 1.0,
            supersymmetry: true,
            flux_bits: 150,
        }
    }

    /// Compact extra dimensions.
    fn extra(&self) -> i32 {
        self.total_dim as i32 - self.observed_dim as i32
    }

    fn manifold(&self) -> Manifold {
        let extra = self.extra().max(0) as u8;
        let space = self.total_dim.saturating_sub(1);
        Manifold {
            dim: self.total_dim,
            signature: Signature { time: 1, space },
            compact_extra: extra,
            compact_radius_planck: if extra == 0 {
                0.0
            } else {
                self.compact_radius_planck
            },
            topology: if extra == 0 {
                Topology::Minkowski
            } else if matches!(
                self.kind,
                StringKind::TypeIIA | StringKind::TypeIIB | StringKind::HeteroticE8xE8
            ) {
                Topology::CalabiYau
            } else {
                Topology::Torus
            },
            convention: physis_model::SignConvention::MostlyPlus,
        }
    }

    /// Heuristic log₁₀(number of vacua). 0 means "unique" for our threshold.
    fn landscape_log10(&self) -> f64 {
        let extra = self.extra().max(0) as f64;
        if extra == 0.0 || self.flux_bits == 0 {
            return 0.0;
        }
        // Folklore: flux compactifications proliferate with extra dims and fluxes.
        // This is a knob-sensitive heuristic, not a computation of the landscape.
        (self.flux_bits as f64) * extra / 4.0
    }

    fn spectrum(&self) -> Spectrum {
        if self.kind == StringKind::Bosonic {
            // Closed bosonic string: graviton, dilaton, Kalb-Ramond; tachyon; no fermions.
            let mut s = Spectrum::empty();
            s.species.push(physis_model::Species::graviton());
            s
        } else {
            Spectrum::standard_model_plus_graviton()
        }
    }
}

impl Knobbed for StringTheory {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }

    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "kind" => Ok(KnobValue::Choice(self.kind.as_str().into())),
            "total_dim" => Ok(KnobValue::UInt(self.total_dim as u64)),
            "observed_dim" => Ok(KnobValue::UInt(self.observed_dim as u64)),
            "compact_radius_planck" => Ok(KnobValue::Float(self.compact_radius_planck)),
            "supersymmetry" => Ok(KnobValue::Bool(self.supersymmetry)),
            "flux_bits" => Ok(KnobValue::UInt(self.flux_bits as u64)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }

    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("kind", KnobValue::Choice(s)) => {
                self.kind = StringKind::parse(&s).ok_or_else(|| CoreError::Domain {
                    name: name.into(),
                    reason: format!("unknown kind {s}"),
                })?;
            }
            ("total_dim", KnobValue::UInt(v)) => self.total_dim = v as u8,
            ("observed_dim", KnobValue::UInt(v)) => self.observed_dim = v as u8,
            ("compact_radius_planck", KnobValue::Float(v)) => self.compact_radius_planck = v,
            ("supersymmetry", KnobValue::Bool(v)) => self.supersymmetry = v,
            ("flux_bits", KnobValue::UInt(v)) => self.flux_bits = v as u32,
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

impl Theory for StringTheory {
    fn id(&self) -> &'static str {
        self.kind.as_str()
    }

    fn name(&self) -> &'static str {
        match self.kind {
            StringKind::Bosonic => "Bosonic string",
            StringKind::TypeI => "Type I string",
            StringKind::TypeIIA => "Type IIA string",
            StringKind::TypeIIB => "Type IIB string",
            StringKind::HeteroticSO32 => "Heterotic SO(32)",
            StringKind::HeteroticE8xE8 => "Heterotic E₈×E₈",
            StringKind::MTheory => "M-theory",
        }
    }

    fn summary(&self) -> &'static str {
        "Extended 1D (or 2D membrane) UV completion of gravity. Critical dimension is a theorem. \
         Vacuum uniqueness is not. This object encodes those facts as knobs and claims; \
         it does not compactify Calabi–Yau manifolds."
    }

    fn world(&self) -> World {
        let extra = self.extra().max(0) as u32;
        World {
            spacetime: self.manifold(),
            gauge: self.kind.fundamental_gauge(),
            spectrum: self.spectrum(),
            has_gravity: true,
            supersymmetric: self.supersymmetry,
            free_parameter_count: 20 + extra * self.flux_bits.max(1),
            landscape_log10: self.landscape_log10(),
            note: format!(
                "{} at D={}, observed {}, extra {}, landscape ~10^{:.1}",
                self.kind.as_str(),
                self.total_dim,
                self.observed_dim,
                self.extra(),
                self.landscape_log10()
            ),
        }
    }

    fn claims(&self) -> Vec<Claim> {
        vec![
            claims::c(
                claims::SPACETIME_STRUCTURE,
                "Signature, dimension, and compact extra directions are internally consistent.",
                LayerId::Spacetime,
                Epistemic::Theorem,
            ),
            claims::c(
                claims::CRITICAL_DIMENSION,
                "Total dimension equals the construction's critical dimension.",
                LayerId::Spacetime,
                Epistemic::Theorem,
            ),
            claims::c(
                claims::SUSY_CONSTRUCTION,
                "Supersymmetry is present if and only if the construction requires it.",
                LayerId::Field,
                Epistemic::Theorem,
            ),
            claims::c(
                claims::NO_TACHYON,
                "The construction does not have a tachyon in its perturbative spectrum.",
                LayerId::Particle,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::OBSERVED_4D,
                "Non-compact spacetime is 3+1.",
                LayerId::Spacetime,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::HIDDEN_EXTRA_DIMS,
                "Compact extra dimensions are not visible at electroweak scales.",
                LayerId::Effective,
                Epistemic::Heuristic,
            ),
            claims::c(
                claims::FERMIONS,
                "The low-energy spectrum contains fermions.",
                LayerId::Particle,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::SM_GAUGE,
                "The fundamental gauge group can contain the Standard Model.",
                LayerId::Interaction,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::THREE_GENERATIONS,
                "Low-energy charged leptons come in three generations.",
                LayerId::Particle,
                Epistemic::Heuristic,
            ),
            claims::c(
                claims::GRAVITY,
                "A massless spin-2 is in the spectrum (closed string / graviton).",
                LayerId::Particle,
                Epistemic::EncodedFact,
            ),
            claims::c(
                claims::UNIQUE_VACUUM,
                "The theory selects a unique vacuum (no landscape).",
                LayerId::Effective,
                Epistemic::Heuristic,
            ),
            claims::c(
                claims::UV_COMPLETION,
                "The theory is a candidate UV completion of gravity plus matter.",
                LayerId::Field,
                Epistemic::Conjecture,
            ),
        ]
    }

    fn evaluate(&self, claim: &Claim) -> Verdict {
        let w = self.world();
        match claim.id.0.as_str() {
            claims::SPACETIME_STRUCTURE => {
                if w.spacetime.structurally_ok() && self.extra() >= 0 {
                    Verdict::holds(
                        Epistemic::Theorem,
                        "dim, signature, and compact extras are consistent",
                    )
                } else {
                    Verdict::fails(Epistemic::Theorem, "spacetime numbers do not fit together")
                        .with_evidence([format!(
                            "dim={} extra={} observed={}",
                            self.total_dim,
                            self.extra(),
                            self.observed_dim
                        )])
                }
            }
            claims::CRITICAL_DIMENSION => {
                let crit = self.kind.critical_dim();
                if self.total_dim == crit {
                    Verdict::holds(
                        Epistemic::Theorem,
                        format!("D={} equals critical dimension", crit),
                    )
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        format!(
                            "D={} but {} requires D={}",
                            self.total_dim,
                            self.kind.as_str(),
                            crit
                        ),
                    )
                    .with_evidence([
                        "worldsheet/membrane conformal anomaly cancellation fixes D".to_string(),
                    ])
                }
            }
            claims::SUSY_CONSTRUCTION => {
                let req = self.kind.requires_susy();
                if self.supersymmetry == req {
                    Verdict::holds(
                        Epistemic::Theorem,
                        if req {
                            "supersymmetry on, as the construction requires"
                        } else {
                            "supersymmetry off, as the bosonic string allows"
                        },
                    )
                } else if req && !self.supersymmetry {
                    Verdict::fails(
                        Epistemic::Theorem,
                        "superstring/M construction with supersymmetry switched off",
                    )
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        "bosonic string with supersymmetry switched on is a different theory",
                    )
                }
            }
            claims::NO_TACHYON => {
                if self.kind == StringKind::Bosonic {
                    Verdict::fails(
                        Epistemic::EncodedFact,
                        "the 26D bosonic string has a tachyon",
                    )
                } else if self.supersymmetry {
                    Verdict::holds(
                        Epistemic::EncodedFact,
                        "spacetime SUSY removes the tachyon in the superstring",
                    )
                } else {
                    Verdict::fails(
                        Epistemic::EncodedFact,
                        "without SUSY the superstring construction is not the usual tachyon-free one",
                    )
                }
            }
            claims::OBSERVED_4D => {
                if self.observed_dim == 4 {
                    Verdict::holds(Epistemic::EncodedFact, "observed_dim = 4")
                } else {
                    Verdict::fails(
                        Epistemic::EncodedFact,
                        format!("observed_dim = {}, not 4", self.observed_dim),
                    )
                }
            }
            claims::HIDDEN_EXTRA_DIMS => {
                let extra = self.extra();
                if extra <= 0 {
                    Verdict::holds(Epistemic::Heuristic, "no extra dimensions to hide")
                } else if self.compact_radius_planck <= 1e16 {
                    // Extremely loose: Planck-to-electroweak is ~10^16 in length ratio-ish
                    // (this is a placeholder scale cut, labelled Heuristic).
                    Verdict::holds(
                        Epistemic::Heuristic,
                        format!(
                            "R = {} ℓ_P is treated as hidden at current colliders",
                            self.compact_radius_planck
                        ),
                    )
                } else {
                    Verdict::fails(
                        Epistemic::Heuristic,
                        "compactification radius is large enough to be visible",
                    )
                    .with_evidence([format!("R = {} ℓ_P", self.compact_radius_planck)])
                }
            }
            claims::FERMIONS => {
                if w.spectrum.has_fermions() {
                    Verdict::holds(
                        Epistemic::EncodedFact,
                        "fermions present in projected spectrum",
                    )
                } else {
                    Verdict::fails(
                        Epistemic::EncodedFact,
                        "bosonic string projection has no fermions",
                    )
                }
            }
            claims::SM_GAUGE => {
                let e = w.gauge.sm_embed();
                if e.contains_sm() {
                    Verdict::holds(
                        Epistemic::EncodedFact,
                        format!("{} contains SM ({e:?})", w.gauge.name()),
                    )
                } else if matches!(
                    self.kind,
                    StringKind::TypeIIA | StringKind::TypeIIB | StringKind::MTheory
                ) {
                    Verdict::undecidable(
                        Epistemic::Heuristic,
                        "Type II / M have no 10D/11D GUT group; SM would have to arise from compactification / branes",
                    )
                } else {
                    Verdict::fails(
                        Epistemic::EncodedFact,
                        format!("{} does not contain SM in this encoding", w.gauge.name()),
                    )
                }
            }
            claims::THREE_GENERATIONS => {
                if self.kind == StringKind::Bosonic {
                    Verdict::fails(Epistemic::EncodedFact, "no fermions, so no generations")
                } else {
                    Verdict::undecidable(
                        Epistemic::Open,
                        "generation count depends on compactification topology, not encoded here",
                    )
                    .with_evidence([
                        "replace this with a typed compactification once that layer exists",
                    ])
                }
            }
            claims::GRAVITY => {
                if w.has_gravity {
                    Verdict::holds(
                        Epistemic::EncodedFact,
                        "closed-string / 11D graviton in the spectrum",
                    )
                } else {
                    Verdict::fails(Epistemic::EncodedFact, "no graviton in projection")
                }
            }
            claims::UNIQUE_VACUUM => {
                let logn = self.landscape_log10();
                if logn < 0.5 {
                    Verdict::holds(
                        Epistemic::Heuristic,
                        "landscape estimate is ~1 vacuum with current knobs",
                    )
                } else {
                    Verdict::fails(
                        Epistemic::Heuristic,
                        format!("landscape estimate ~10^{logn:.1} vacua"),
                    )
                    .with_evidence([
                        format!("flux_bits={}", self.flux_bits),
                        format!("extra={}", self.extra()),
                        "this is the Weinstein-adjacent predictivity objection, encoded as a heuristic claim — not a proof that string theory is false".to_string(),
                    ])
                }
            }
            claims::UV_COMPLETION => {
                if self.total_dim == self.kind.critical_dim()
                    && (self.supersymmetry == self.kind.requires_susy())
                {
                    Verdict::holds(
                        Epistemic::Conjecture,
                        "internally consistent construction; UV-completeness is still a conjecture about nature",
                    )
                } else {
                    Verdict::fails(
                        Epistemic::Conjecture,
                        "construction knobs are off-critical or off-SUSY; not the usual UV candidate",
                    )
                }
            }
            _ => Verdict::inapplicable("claim not made by this string object"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;

    fn verdict(t: &StringTheory, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id.0 == id).unwrap();
        t.evaluate(&c).kind
    }

    #[test]
    fn iib_defaults_hold_critical_dim() {
        let t = StringTheory::type_iib();
        assert_eq!(verdict(&t, claims::CRITICAL_DIMENSION), VerdictKind::Holds);
        assert_eq!(verdict(&t, claims::OBSERVED_4D), VerdictKind::Holds);
        assert_eq!(verdict(&t, claims::FERMIONS), VerdictKind::Holds);
        assert_eq!(verdict(&t, claims::UNIQUE_VACUUM), VerdictKind::Fails);
    }

    #[test]
    fn turning_d_to_nine_fails_critical_dim() {
        let mut t = StringTheory::type_iib();
        t.set("total_dim", KnobValue::UInt(9)).unwrap();
        assert_eq!(verdict(&t, claims::CRITICAL_DIMENSION), VerdictKind::Fails);
    }

    #[test]
    fn bosonic_has_tachyon_and_no_fermions() {
        let t = StringTheory::bosonic();
        assert_eq!(verdict(&t, claims::NO_TACHYON), VerdictKind::Fails);
        assert_eq!(verdict(&t, claims::FERMIONS), VerdictKind::Fails);
        assert_eq!(verdict(&t, claims::CRITICAL_DIMENSION), VerdictKind::Holds);
    }

    #[test]
    fn heterotic_encodes_sm_embedding() {
        let t = StringTheory::heterotic_e8();
        assert_eq!(verdict(&t, claims::SM_GAUGE), VerdictKind::Holds);
    }
}
