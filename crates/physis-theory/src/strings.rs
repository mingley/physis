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
use physis_core::Scale;
use physis_model::constants::planck_length;
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
    KnobSpec {
        name: "dilaton",
        layer: LayerId::Field,
        doc: "Dilaton VEV φ; string coupling is g_s = e^φ. Large g_s inflates the effective size of the compact space.",
        domain: KnobDomain::Float {
            min: -30.0,
            max: 30.0,
        },
    },
    KnobSpec {
        name: "h11",
        layer: LayerId::Spacetime,
        doc: "Kähler (size) moduli count, a heuristic stand-in for h^{1,1}. Drives the flux landscape.",
        domain: KnobDomain::UInt { min: 0, max: 500 },
    },
    KnobSpec {
        name: "h21",
        layer: LayerId::Spacetime,
        doc: "Complex-structure (shape) moduli count, a heuristic stand-in for h^{2,1}. Drives the flux landscape.",
        domain: KnobDomain::UInt { min: 0, max: 500 },
    },
    KnobSpec {
        name: "euler_number",
        layer: LayerId::Spacetime,
        doc: "Euler characteristic χ of the compactification (0 = unset). Chiral generations = |χ|/2. The value is chosen, not derived — the crux of the predictivity critique.",
        domain: KnobDomain::Int {
            min: -1000,
            max: 1000,
        },
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
    dilaton: f64,
    h11: u32,
    h21: u32,
    euler_number: i64,
}

impl StringTheory {
    /// Build a construction at its critical dimension with default moduli.
    ///
    /// Supersymmetry defaults to whatever the kind requires; the compact space
    /// starts at 4 observed dimensions, radius 1 ℓ_P, dilaton 0 (g_s = 1), and
    /// a small `h11 = h21 = 3` moduli stand-in.
    fn new(kind: StringKind, flux_bits: u32) -> Self {
        Self {
            kind,
            total_dim: kind.critical_dim(),
            observed_dim: 4,
            compact_radius_planck: 1.0,
            supersymmetry: kind.requires_susy(),
            flux_bits,
            dilaton: 0.0,
            h11: 3,
            h21: 3,
            euler_number: 0,
        }
    }

    /// Chiral generations from the compactification topology.
    ///
    /// A Calabi–Yau threefold has an **even** Euler characteristic and gives
    /// `|χ|/2` net generations. Returns `None` when the topology is unset
    /// (`χ = 0`) or `χ` is odd (not a valid Calabi–Yau Euler number).
    fn generations_from_topology(&self) -> Option<i64> {
        if self.euler_number == 0 || self.euler_number % 2 != 0 {
            None
        } else {
            Some(self.euler_number.abs() / 2)
        }
    }

    /// Type IIB at its critical dimension, 6 extra dims at 1 Planck length.
    pub fn type_iib() -> Self {
        Self::new(StringKind::TypeIIB, 200)
    }

    /// Heterotic E₈×E₈, the usual SM-embedding story.
    pub fn heterotic_e8() -> Self {
        Self::new(StringKind::HeteroticE8xE8, 80)
    }

    /// Type I (open + closed) with SO(32) gauge at D=10.
    pub fn type_i() -> Self {
        Self::new(StringKind::TypeI, 100)
    }

    /// Type IIA (closed, non-chiral) at D=10.
    pub fn type_iia() -> Self {
        Self::new(StringKind::TypeIIA, 200)
    }

    /// Heterotic SO(32) at D=10.
    pub fn heterotic_so32() -> Self {
        Self::new(StringKind::HeteroticSO32, 80)
    }

    /// Bosonic string at D=26.
    pub fn bosonic() -> Self {
        Self::new(StringKind::Bosonic, 20)
    }

    /// M-theory at D=11.
    pub fn m_theory() -> Self {
        Self::new(StringKind::MTheory, 150)
    }

    /// Compact extra dimensions.
    fn extra(&self) -> i32 {
        self.total_dim as i32 - self.observed_dim as i32
    }

    /// String coupling g_s = e^φ from the dilaton.
    fn string_coupling(&self) -> f64 {
        self.dilaton.exp()
    }

    /// Total moduli count (Kähler + complex structure stand-ins).
    fn moduli(&self) -> u32 {
        self.h11 + self.h21
    }

    /// Effective compact size seen at low energy: the Kähler volume radius
    /// modulated by the string coupling (a heuristic frame factor). Larger
    /// g_s inflates the effective size, making extra dimensions easier to see.
    fn effective_radius_planck(&self) -> f64 {
        self.compact_radius_planck * self.string_coupling().sqrt()
    }

    /// The effective compact radius as a typed physical length.
    fn effective_radius(&self) -> physis_core::Qty<physis_core::Length> {
        planck_length() * self.effective_radius_planck()
    }

    fn build_world(&self) -> World {
        World {
            spacetime: self.manifold(),
            gauge: self.kind.fundamental_gauge(),
            spectrum: self.spectrum(),
            has_gravity: true,
            supersymmetric: self.supersymmetry,
            // The moduli are the continuous free parameters (plus the dilaton).
            free_parameter_count: 4 + self.moduli() + 1,
            landscape_log10: self.landscape_log10(),
            note: format!(
                "{} at D={}, observed {}, extra {}, g_s={:.2}, moduli h11={}+h21={}, landscape ~10^{:.1}",
                self.kind.as_str(),
                self.total_dim,
                self.observed_dim,
                self.extra(),
                self.string_coupling(),
                self.h11,
                self.h21,
                self.landscape_log10()
            ),
        }
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
    ///
    /// Folklore: flux vacua proliferate as (flux choices)^(number of moduli).
    /// So log₁₀N ≈ (moduli) · (flux_bits · log₁₀2). With no compact space, no
    /// flux, or no moduli there is nothing to scan and the count collapses to
    /// one. This is knob-sensitive folklore, not a computation of the landscape.
    fn landscape_log10(&self) -> f64 {
        if self.extra() <= 0 {
            return 0.0;
        }
        (self.moduli() as f64) * (self.flux_bits as f64) * std::f64::consts::LOG10_2
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
            "dilaton" => Ok(KnobValue::Float(self.dilaton)),
            "h11" => Ok(KnobValue::UInt(self.h11 as u64)),
            "h21" => Ok(KnobValue::UInt(self.h21 as u64)),
            "euler_number" => Ok(KnobValue::Int(self.euler_number)),
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
            ("dilaton", KnobValue::Float(v)) => self.dilaton = v,
            ("h11", KnobValue::UInt(v)) => self.h11 = v as u32,
            ("h21", KnobValue::UInt(v)) => self.h21 = v as u32,
            ("euler_number", KnobValue::Int(v)) => self.euler_number = v,
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

    fn world(&self) -> Option<World> {
        Some(self.build_world())
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
                claims::ANOMALY_CANCELLATION,
                "Chiral gauge/gravitational anomalies cancel (Green–Schwarz in 10D).",
                LayerId::Interaction,
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
        let w = self.build_world();
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
            claims::ANOMALY_CANCELLATION => match self.kind {
                StringKind::Bosonic => Verdict::inapplicable(
                    "non-chiral 26D string: gauge/gravitational anomalies are not the \
                     obstruction here (the tachyon is)",
                ),
                StringKind::TypeIIA | StringKind::TypeIIB => Verdict::holds(
                    Epistemic::EncodedFact,
                    "Type II 10D spectrum is anomaly-free (no chiral gauge anomaly to cancel)",
                ),
                StringKind::MTheory => Verdict::holds(
                    Epistemic::EncodedFact,
                    "11D supergravity is anomaly-free; boundary E₈ factors are the \
                     Hořava–Witten mechanism",
                ),
                StringKind::TypeI | StringKind::HeteroticSO32 | StringKind::HeteroticE8xE8 => {
                    if self.total_dim != 10 {
                        Verdict::undecidable(
                            Epistemic::EncodedFact,
                            "Green–Schwarz cancellation is a 10D statement; off the critical \
                             dimension this encoding does not assert it",
                        )
                    } else if w.gauge.gs_anomaly_free_10d() {
                        Verdict::holds(
                            Epistemic::EncodedFact,
                            format!("{} anomalies cancel via Green–Schwarz", w.gauge.name()),
                        )
                        .with_evidence([
                            "dimension-496 GS solution (SO(32) or E₈×E₈); encoded, not \
                             re-derived from the anomaly polynomial"
                                .to_string(),
                        ])
                    } else {
                        Verdict::fails(
                            Epistemic::EncodedFact,
                            format!("{} is not a 10D Green–Schwarz solution", w.gauge.name()),
                        )
                    }
                }
            },
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
                // Typed lengths, not a magic float: an extra dimension is hidden
                // while its effective radius stays below the shortest length we
                // currently probe (the electroweak scale). The effective radius
                // folds in the Kähler size and the dilaton (g_s) frame factor.
                let r_eff = self.effective_radius();
                let probe = Scale::Electroweak.typical_length();
                if extra <= 0 {
                    Verdict::holds(Epistemic::Heuristic, "no extra dimensions to hide")
                } else if r_eff.value() <= probe.value() {
                    Verdict::holds(
                        Epistemic::Heuristic,
                        format!("effective R = {r_eff} is below the {probe} electroweak probe"),
                    )
                } else {
                    Verdict::fails(
                        Epistemic::Heuristic,
                        "effective compact size exceeds the electroweak probe length",
                    )
                    .with_evidence([
                        format!(
                            "effective R = {r_eff} (radius {} ℓ_P × √g_s, g_s = {:.2})",
                            self.compact_radius_planck,
                            self.string_coupling()
                        ),
                        format!("electroweak probe length = {probe}"),
                    ])
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
                    let chain = w
                        .gauge
                        .verified_contains_sm()
                        .unwrap_or_default()
                        .join(" ⊃ ");
                    Verdict::holds(
                        Epistemic::EncodedFact,
                        format!("{} contains SM", w.gauge.name()),
                    )
                    .with_evidence([
                        format!("verified embedding chain: {chain}"),
                        "checked by maximal-subgroup steps with rank/dimension necessary conditions (Georgi–Glashow SU(5) ⊃ SM); encoded chain, not a root-system proof".to_string(),
                    ])
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
                    if self.euler_number != 0 && self.euler_number % 2 != 0 {
                        return Verdict::fails(
                            Epistemic::EncodedFact,
                            format!(
                                "χ = {} is odd; a Calabi–Yau threefold has an even Euler number",
                                self.euler_number
                            ),
                        );
                    }
                    match self.generations_from_topology() {
                        None => Verdict::undecidable(
                            Epistemic::Open,
                            "generation count depends on the compactification topology (set euler_number)",
                        ),
                        Some(3) => Verdict::holds(
                            Epistemic::EncodedFact,
                            format!(
                                "|χ|/2 = 3 generations from χ = {} — accommodated, not derived",
                                self.euler_number
                            ),
                        )
                        .with_evidence([
                            "the topological count |χ|/2 is a real theorem; but *why* χ = ±6 is not derived — this is the predictivity critique made mechanical".to_string(),
                        ]),
                        Some(g) => Verdict::fails(
                            Epistemic::EncodedFact,
                            format!("|χ|/2 = {g} generations from χ = {}, not 3", self.euler_number),
                        ),
                    }
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
    fn switching_kind_to_bosonic_flips_fermions_and_tachyon() {
        // The `kind` knob is load-bearing: flipping a superstring to the
        // bosonic string must remove the fermions and reintroduce the tachyon.
        let mut t = StringTheory::type_iib();
        assert_eq!(verdict(&t, claims::FERMIONS), VerdictKind::Holds);
        assert_eq!(verdict(&t, claims::NO_TACHYON), VerdictKind::Holds);
        t.set("kind", KnobValue::Choice("bosonic".into())).unwrap();
        assert_eq!(verdict(&t, claims::FERMIONS), VerdictKind::Fails);
        assert_eq!(verdict(&t, claims::NO_TACHYON), VerdictKind::Fails);
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

    #[test]
    fn so32_constructions_embed_sm() {
        // Type I and heterotic SO(32) both carry an SO(32) gauge sector that
        // contains the Standard Model as an encoded textbook fact.
        for t in [StringTheory::type_i(), StringTheory::heterotic_so32()] {
            assert_eq!(verdict(&t, claims::SM_GAUGE), VerdictKind::Holds);
            assert_eq!(verdict(&t, claims::CRITICAL_DIMENSION), VerdictKind::Holds);
            assert_eq!(verdict(&t, claims::NO_TACHYON), VerdictKind::Holds);
        }
    }

    #[test]
    fn type_iia_gauge_is_undecidable() {
        // Like IIB, Type IIA has no perturbative 10D GUT group.
        let t = StringTheory::type_iia();
        assert_eq!(verdict(&t, claims::SM_GAUGE), VerdictKind::Undecidable);
    }

    #[test]
    fn m_theory_is_eleven_dimensional() {
        let mut t = StringTheory::m_theory();
        assert_eq!(t.kind.critical_dim(), 11);
        assert_eq!(verdict(&t, claims::CRITICAL_DIMENSION), VerdictKind::Holds);
        assert_eq!(verdict(&t, claims::GRAVITY), VerdictKind::Holds);
        // M-theory has no worldsheet GUT group either.
        assert_eq!(verdict(&t, claims::SM_GAUGE), VerdictKind::Undecidable);
        t.set("total_dim", KnobValue::UInt(10)).unwrap();
        assert_eq!(verdict(&t, claims::CRITICAL_DIMENSION), VerdictKind::Fails);
    }

    #[test]
    fn green_schwarz_constructions_cancel_anomalies() {
        // SO(32) and E₈×E₈ constructions cancel via Green–Schwarz.
        for t in [
            StringTheory::type_i(),
            StringTheory::heterotic_so32(),
            StringTheory::heterotic_e8(),
        ] {
            assert_eq!(
                verdict(&t, claims::ANOMALY_CANCELLATION),
                VerdictKind::Holds,
                "{} should cancel anomalies via GS",
                t.id()
            );
        }
        // Type II and M are anomaly-free for their own (non-GS) reasons.
        for t in [
            StringTheory::type_iia(),
            StringTheory::type_iib(),
            StringTheory::m_theory(),
        ] {
            assert_eq!(
                verdict(&t, claims::ANOMALY_CANCELLATION),
                VerdictKind::Holds
            );
        }
        // The bosonic string is non-chiral: anomalies are not the obstruction.
        assert_eq!(
            verdict(&StringTheory::bosonic(), claims::ANOMALY_CANCELLATION),
            VerdictKind::Inapplicable
        );
    }

    #[test]
    fn anomaly_claim_is_a_ten_dimensional_statement() {
        // Off the critical dimension, GS cancellation is not asserted.
        let mut t = StringTheory::heterotic_e8();
        assert_eq!(
            verdict(&t, claims::ANOMALY_CANCELLATION),
            VerdictKind::Holds
        );
        t.set("total_dim", KnobValue::UInt(9)).unwrap();
        assert_eq!(
            verdict(&t, claims::ANOMALY_CANCELLATION),
            VerdictKind::Undecidable
        );
    }

    #[test]
    fn euler_number_accommodates_three_generations_without_deriving_them() {
        let mut t = StringTheory::heterotic_e8();
        // With no chosen topology, the generation count is genuinely open.
        assert_eq!(
            verdict(&t, claims::THREE_GENERATIONS),
            VerdictKind::Undecidable
        );
        // χ = ±6 gives |χ|/2 = 3 generations — accommodated by a chosen topology.
        t.set("euler_number", KnobValue::Int(6)).unwrap();
        assert_eq!(verdict(&t, claims::THREE_GENERATIONS), VerdictKind::Holds);
        t.set("euler_number", KnobValue::Int(-6)).unwrap();
        assert_eq!(verdict(&t, claims::THREE_GENERATIONS), VerdictKind::Holds);
        // A different topology gives a different, wrong count.
        t.set("euler_number", KnobValue::Int(8)).unwrap();
        assert_eq!(verdict(&t, claims::THREE_GENERATIONS), VerdictKind::Fails);
        // An odd χ is not a valid Calabi–Yau Euler number (must not truncate to 3).
        t.set("euler_number", KnobValue::Int(7)).unwrap();
        assert_eq!(verdict(&t, claims::THREE_GENERATIONS), VerdictKind::Fails);
    }

    #[test]
    fn moduli_drive_the_landscape() {
        let mut t = StringTheory::type_iib();
        assert_eq!(verdict(&t, claims::UNIQUE_VACUUM), VerdictKind::Fails);
        // With no moduli to stabilize against flux, there is nothing to scan:
        // the landscape collapses and uniqueness is restored.
        t.set("h11", KnobValue::UInt(0)).unwrap();
        t.set("h21", KnobValue::UInt(0)).unwrap();
        assert_eq!(verdict(&t, claims::UNIQUE_VACUUM), VerdictKind::Holds);
    }

    #[test]
    fn zero_flux_restores_uniqueness() {
        let mut t = StringTheory::type_iib();
        t.set("flux_bits", KnobValue::UInt(0)).unwrap();
        assert_eq!(verdict(&t, claims::UNIQUE_VACUUM), VerdictKind::Holds);
    }

    #[test]
    fn kahler_volume_and_dilaton_expose_extra_dimensions() {
        let mut t = StringTheory::type_iib();
        assert_eq!(verdict(&t, claims::HIDDEN_EXTRA_DIMS), VerdictKind::Holds);
        // A large Kähler volume (radius) that is still under the cut stays hidden.
        t.set("compact_radius_planck", KnobValue::Float(1e15))
            .unwrap();
        assert_eq!(verdict(&t, claims::HIDDEN_EXTRA_DIMS), VerdictKind::Holds);
        // The dilaton (g_s) then tips the effective size over the threshold.
        t.set("dilaton", KnobValue::Float(10.0)).unwrap();
        assert_eq!(verdict(&t, claims::HIDDEN_EXTRA_DIMS), VerdictKind::Fails);
    }

    #[test]
    fn every_default_string_construction_fails_uniqueness() {
        // The predictivity objection is mechanical for all default string knobs.
        for t in [
            StringTheory::type_iib(),
            StringTheory::type_iia(),
            StringTheory::type_i(),
            StringTheory::heterotic_e8(),
            StringTheory::heterotic_so32(),
            StringTheory::bosonic(),
            StringTheory::m_theory(),
        ] {
            assert_eq!(
                verdict(&t, claims::UNIQUE_VACUUM),
                VerdictKind::Fails,
                "{} should fail uniqueness under default knobs",
                t.id()
            );
        }
    }
}
