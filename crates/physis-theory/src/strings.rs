//! Superstring / bosonic string / M-theory as knobbed theories.
//!
//! Critical dimensions (26, 10, 11) are theorems of the worldsheet
//! (or membrane) conformal anomaly, encoded here as `ClaimClass::ModelInternal`.
//! Landscape counts are `Heuristic`. SM embeddings are `EncodedFact`.
//!
//! This is a laboratory object, not a compactification engine.
//!
//! Heterotic `E8 x E8` lives on the IR package of `heterotic-e8e8`. A
//! missing E8 (`add-missing-e8`) is a package mutation, not the `kind`
//! or `total_dim` knob: Green–Schwarz fails because dimension 248 is
//! not a 10D solution, while SM still embeds in the remaining E8.
//! Heterotic `SO(32)` lives on the IR package of `heterotic-so32`.
//! Appending `SO(16)` (`add-so16`) is a package mutation, not those
//! knobs: Green–Schwarz fails because dimension 120 is not a 10D
//! solution, while SM still embeds via SO(10). Type I `Chan-Paton
//! SO(32)` lives on the IR package of `type-i`. Appending
//! `Chan-Paton SO(16)` (`add-chan-paton-16`) is a package mutation,
//! not those knobs: Green–Schwarz fails because dimension 120 is not
//! a 10D solution, while SM still embeds via SO(10). Type II, bosonic,
//! and M-theory have no package.

use physis_core::assumption::DomainOfValidity;
use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::ParameterOrigin;
use physis_core::Scale;
use physis_ir::{apply_mutation, parse_package, render_package, PackageMutation, TheoryPackage};
use physis_model::constants::planck_length;
use physis_model::{GaugeGroup, Manifold, Signature, Spectrum, Topology, World};

use crate::claims;
use crate::framework::Theory;

/// Live heterotic gauge on the `heterotic-e8e8` package.
const E8E8_EQ: &str = "E8 x E8";
/// Incomplete encoding: one E8 is missing.
const MISSING_E8_EQ: &str = "missing E8";
/// Live heterotic gauge on the `heterotic-so32` package.
const SO32_EQ: &str = "SO(32)";
/// Incomplete encoding: SO(16) in place of the live SO(32).
const SO16_EQ: &str = "SO(16)";
/// Live Type I gauge on the `type-i` package.
const CHAN_PATON_SO32_EQ: &str = "Chan-Paton SO(32)";
/// Incomplete Type I encoding: Chan-Paton rank 16.
const CHAN_PATON_SO16_EQ: &str = "Chan-Paton SO(16)";

fn e8e8_gs_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["E8 x E8".into()],
        vec!["dimension 496 Green-Schwarz solution".into()],
        "Complete E8 x E8 (dimension 496). A missing E8 factor is not a Green-Schwarz solution.",
    )
}

fn so32_gs_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["SO(32)".into()],
        vec!["dimension 496 Green-Schwarz solution".into()],
        "Complete SO(32) (dimension 496). Appending SO(16) is not a Green-Schwarz solution.",
    )
}

fn type_i_gs_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["Chan-Paton SO(32)".into()],
        vec!["dimension 496 Green-Schwarz solution".into()],
        "Complete Chan-Paton SO(32) (dimension 496). Appending Chan-Paton SO(16) is not a Green-Schwarz solution.",
    )
}

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

    /// Worldsheet central charges `(c_matter per spacetime dimension, c_ghost)`.
    ///
    /// The critical dimension is where the total conformal anomaly cancels,
    /// `c_matter·D + c_ghost = 0`: bosonic `1·D − 26 = 0 ⇒ 26`; superstring
    /// `(3/2)·D − 15 = 0 ⇒ 10`. Returns `None` for M-theory, which is a
    /// membrane / 11D supergravity with no worldsheet conformal anomaly.
    pub fn worldsheet_central_charge(self) -> Option<(f64, f64)> {
        match self {
            StringKind::Bosonic => Some((1.0, -26.0)),
            StringKind::MTheory => None,
            _ => Some((1.5, -15.0)),
        }
    }

    /// Critical dimension derived from the conformal-anomaly cancellation,
    /// when the object is a worldsheet theory.
    pub fn critical_dim_from_anomaly(self) -> Option<u8> {
        self.worldsheet_central_charge()
            .map(|(c_matter, c_ghost)| (-c_ghost / c_matter).round() as u8)
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
        doc: "Which string/M construction (sets critical dimension and default gauge). A missing E8 is not this knob: add-missing-e8 is an IR mutation on heterotic-e8e8. SO(16) is not this knob: add-so16 is an IR mutation on heterotic-so32. Chan-Paton SO(16) is not this knob: add-chan-paton-16 is an IR mutation on type-i.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Choice(&StringKind::ALL),
    },
    KnobSpec {
        name: "total_dim",
        layer: LayerId::Spacetime,
        doc: "Total spacetime dimension D. Superstring theorem: D=10; bosonic D=26; M D=11. A missing E8 is not this knob: add-missing-e8 is an IR mutation on heterotic-e8e8. SO(16) is not this knob: add-so16 is an IR mutation on heterotic-so32. Chan-Paton SO(16) is not this knob: add-chan-paton-16 is an IR mutation on type-i.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 2, max: 32 },
    },
    KnobSpec {
        name: "observed_dim",
        layer: LayerId::Spacetime,
        doc: "Non-compact macroscopic dimension. Empirical target: 4.",
        origin: ParameterOrigin::Measured,
        domain: KnobDomain::UInt { min: 1, max: 32 },
    },
    KnobSpec {
        name: "compact_radius_planck",
        layer: LayerId::Spacetime,
        doc: "Compactification radius in Planck lengths. O(1) hides extra dims; huge radii would be seen.",
        origin: ParameterOrigin::Fitted,
        domain: KnobDomain::Float {
            min: 1e-6,
            max: 1e40,
        },
    },
    KnobSpec {
        name: "supersymmetry",
        layer: LayerId::Field,
        doc: "Whether the construction includes spacetime supersymmetry.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Bool,
    },
    KnobSpec {
        name: "flux_bits",
        layer: LayerId::Interaction,
        doc: "Heuristic bits of flux/moduli data contributing to a landscape count.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 0, max: 10_000 },
    },
    KnobSpec {
        name: "dilaton",
        layer: LayerId::Field,
        doc: "Dilaton VEV φ; string coupling is g_s = e^φ. Large g_s inflates the effective size of the compact space.",
        origin: ParameterOrigin::Fitted,
        domain: KnobDomain::Float {
            min: -30.0,
            max: 30.0,
        },
    },
    KnobSpec {
        name: "h11",
        layer: LayerId::Spacetime,
        doc: "Kähler (size) moduli count, a heuristic stand-in for h^{1,1}. Drives the flux landscape.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 0, max: 500 },
    },
    KnobSpec {
        name: "h21",
        layer: LayerId::Spacetime,
        doc: "Complex-structure (shape) moduli count, a heuristic stand-in for h^{2,1}. Drives the flux landscape.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 0, max: 500 },
    },
    KnobSpec {
        name: "euler_number",
        layer: LayerId::Spacetime,
        doc: "Euler characteristic χ of the compactification (0 = unset). Chiral generations = |χ|/2. The value is chosen, not derived — the crux of the predictivity critique.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Int {
            min: -1000,
            max: 1000,
        },
    },
];

/// A knobbed string / M-theory object.
///
/// Heterotic `E8 x E8` lives on the IR package of `heterotic-e8e8`.
/// A missing E8 (`add-missing-e8`) is a package mutation, not a knob.
/// Heterotic `SO(32)` lives on the IR package of `heterotic-so32`.
/// Appending `SO(16)` (`add-so16`) is a package mutation, not a knob.
/// Type I `Chan-Paton SO(32)` lives on the IR package of `type-i`.
/// Appending `Chan-Paton SO(16)` (`add-chan-paton-16`) is a package
/// mutation, not a knob. `kind` and `total_dim` stay knobs. Type II
/// has no package.
#[derive(Clone, Debug, PartialEq)]
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
    /// Live heterotic-e8e8 encodings carry complete `E8 x E8`. Hypothesis
    /// search may append `missing E8`, which this flag records. Default
    /// false. Not a scientific knob.
    missing_e8: bool,
    /// Live heterotic-so32 encodings carry complete `SO(32)`. Hypothesis
    /// search may append `SO(16)`, which this flag records. Default
    /// false. Not a scientific knob.
    so16: bool,
    /// Live type-i encodings carry complete `Chan-Paton SO(32)`.
    /// Hypothesis search may append `Chan-Paton SO(16)`, which this
    /// flag records. Default false. Not a scientific knob.
    chan_paton_16: bool,
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
            missing_e8: false,
            so16: false,
            chan_paton_16: false,
        }
    }

    /// Gauge algebra of this construction. Live heterotic-E8×E8 encodings
    /// keep both E8 factors. Hypothesis search may drop one (`add-missing-e8`).
    /// Live heterotic-SO(32) encodings keep SO(32). Hypothesis search may
    /// append SO(16) (`add-so16`). Live Type I encodings keep Chan-Paton
    /// SO(32). Hypothesis search may append Chan-Paton SO(16)
    /// (`add-chan-paton-16`).
    fn gauge(&self) -> GaugeGroup {
        if self.missing_e8 && self.kind == StringKind::HeteroticE8xE8 {
            GaugeGroup::e8()
        } else if (self.so16 && self.kind == StringKind::HeteroticSO32)
            || (self.chan_paton_16 && self.kind == StringKind::TypeI)
        {
            GaugeGroup::so16()
        } else {
            self.kind.fundamental_gauge()
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

    /// Leading ground-state mass² in α' = 1 units. `< 0` is a tachyon — the
    /// same stability notion as the scalar field's minimum ω² in `field-modes`.
    ///
    /// Bosonic string: `α'm² = −1` (the ground state is tachyonic). Superstring
    /// with the GSO projection (SUSY on): `0` (massless, tachyon removed).
    /// Superstring with GSO off: `−1/2` (the NS tachyon returns).
    fn ground_state_mass_squared(&self) -> f64 {
        if !self.kind.requires_susy() {
            -1.0
        } else if self.supersymmetry {
            0.0
        } else {
            -0.5
        }
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
            gauge: self.gauge(),
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

    /// IR package for this construction. Heterotic-e8e8 equations are
    /// `E8 x E8` and, when forked, `missing E8`. Heterotic-so32 equations
    /// are `SO(32)` and, when forked, `SO(16)`. Type I equations are
    /// `Chan-Paton SO(32)` and, when forked, `Chan-Paton SO(16)`.
    /// `kind` and `total_dim` stay on the struct.
    pub fn package(&self) -> TheoryPackage {
        match self.id() {
            "heterotic-so32" => self.so32_package(),
            "type-i" => self.type_i_package(),
            _ => self.e8e8_package(),
        }
    }

    fn e8e8_package(&self) -> TheoryPackage {
        let mut equations = vec![E8E8_EQ.to_string()];
        if self.missing_e8 {
            equations.push(MISSING_E8_EQ.to_string());
        }
        TheoryPackage {
            id: "heterotic-e8e8".into(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["complete-e8-x-e8".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: claims::ANOMALY_CANCELLATION.into(),
                statement: "Chiral gauge/gravitational anomalies cancel (Green–Schwarz in 10D)."
                    .into(),
                layer: "interaction".into(),
                class: "phenomenological".into(),
            }],
            lean_ref: None,
        }
    }

    fn so32_package(&self) -> TheoryPackage {
        let mut equations = vec![SO32_EQ.to_string()];
        if self.so16 {
            equations.push(SO16_EQ.to_string());
        }
        TheoryPackage {
            id: "heterotic-so32".into(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["complete-so32".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: claims::ANOMALY_CANCELLATION.into(),
                statement: "Chiral gauge/gravitational anomalies cancel (Green–Schwarz in 10D)."
                    .into(),
                layer: "interaction".into(),
                class: "phenomenological".into(),
            }],
            lean_ref: None,
        }
    }

    fn type_i_package(&self) -> TheoryPackage {
        let mut equations = vec![CHAN_PATON_SO32_EQ.to_string()];
        if self.chan_paton_16 {
            equations.push(CHAN_PATON_SO16_EQ.to_string());
        }
        TheoryPackage {
            id: "type-i".into(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["complete-chan-paton-so32".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: claims::ANOMALY_CANCELLATION.into(),
                statement: "Chiral gauge/gravitational anomalies cancel (Green–Schwarz in 10D)."
                    .into(),
                layer: "interaction".into(),
                class: "phenomenological".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load a heterotic or Type I encoding from a package. Knobs default;
    /// overlay them from a live object when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        match pkg.id.as_str() {
            "heterotic-e8e8" => {
                let missing_e8 = parse_e8e8_gauge(pkg)?;
                Ok(Self {
                    missing_e8,
                    ..Self::heterotic_e8()
                })
            }
            "heterotic-so32" => {
                let so16 = parse_so32_gauge(pkg)?;
                Ok(Self {
                    so16,
                    ..Self::heterotic_so32()
                })
            }
            "type-i" => {
                let chan_paton_16 = parse_type_i_gauge(pkg)?;
                Ok(Self {
                    chan_paton_16,
                    ..Self::type_i()
                })
            }
            other => Err(format!(
                "string package id '{other}' is not heterotic-e8e8, heterotic-so32, or type-i"
            )),
        }
    }

    fn missing_equation() -> String {
        MISSING_E8_EQ.to_string()
    }

    fn so16_equation() -> String {
        SO16_EQ.to_string()
    }

    fn chan_paton_16_equation() -> String {
        CHAN_PATON_SO16_EQ.to_string()
    }
}

fn parse_e8e8_gauge(pkg: &TheoryPackage) -> Result<bool, String> {
    let mut complete = false;
    let mut missing = false;
    for eq in &pkg.equations {
        match eq.trim() {
            E8E8_EQ => complete = true,
            MISSING_E8_EQ => missing = true,
            _ => {}
        }
    }
    if !complete {
        return Err(format!(
            "{} package has no E8 x E8 gauge assignment",
            pkg.id
        ));
    }
    Ok(missing)
}

fn parse_so32_gauge(pkg: &TheoryPackage) -> Result<bool, String> {
    let mut complete = false;
    let mut so16 = false;
    for eq in &pkg.equations {
        match eq.trim() {
            SO32_EQ => complete = true,
            SO16_EQ => so16 = true,
            _ => {}
        }
    }
    if !complete {
        return Err(format!("{} package has no SO(32) gauge assignment", pkg.id));
    }
    Ok(so16)
}

fn parse_type_i_gauge(pkg: &TheoryPackage) -> Result<bool, String> {
    let mut complete = false;
    let mut chan_paton_16 = false;
    for eq in &pkg.equations {
        match eq.trim() {
            CHAN_PATON_SO32_EQ => complete = true,
            CHAN_PATON_SO16_EQ => chan_paton_16 = true,
            _ => {}
        }
    }
    if !complete {
        return Err(format!(
            "{} package has no Chan-Paton SO(32) gauge assignment",
            pkg.id
        ));
    }
    Ok(chan_paton_16)
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
        let mut anomaly = claims::c(
            claims::ANOMALY_CANCELLATION,
            "Chiral gauge/gravitational anomalies cancel (Green–Schwarz in 10D).",
            LayerId::Interaction,
            ClaimClass::Phenomenological,
        );
        if self.kind == StringKind::HeteroticE8xE8 {
            anomaly = anomaly.with_domain(e8e8_gs_domain());
        } else if self.kind == StringKind::HeteroticSO32 {
            anomaly = anomaly.with_domain(so32_gs_domain());
        } else if self.kind == StringKind::TypeI {
            anomaly = anomaly.with_domain(type_i_gs_domain());
        }
        vec![
            claims::c(
                claims::SPACETIME_STRUCTURE,
                "Signature, dimension, and compact extra directions are internally consistent.",
                LayerId::Spacetime,
                ClaimClass::ModelInternal,
            ),
            claims::c(
                claims::CRITICAL_DIMENSION,
                "Total dimension equals the construction's critical dimension.",
                LayerId::Spacetime,
                ClaimClass::ModelInternal,
            ),
            claims::c(
                claims::SUSY_CONSTRUCTION,
                "Supersymmetry is present if and only if the construction requires it.",
                LayerId::Field,
                ClaimClass::ModelInternal,
            ),
            claims::c(
                claims::NO_TACHYON,
                "The construction does not have a tachyon in its perturbative spectrum.",
                LayerId::Particle,
                ClaimClass::Phenomenological,
            ),
            anomaly,
            claims::c(
                claims::OBSERVED_4D,
                "Non-compact spacetime is 3+1.",
                LayerId::Spacetime,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::HIDDEN_EXTRA_DIMS,
                "Compact extra dimensions are not visible at electroweak scales.",
                LayerId::Effective,
                ClaimClass::Heuristic,
            ),
            claims::c(
                claims::FERMIONS,
                "The low-energy spectrum contains fermions.",
                LayerId::Particle,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::SM_GAUGE,
                "The fundamental gauge group can contain the Standard Model.",
                LayerId::Interaction,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::THREE_GENERATIONS,
                "Low-energy charged leptons come in three generations.",
                LayerId::Particle,
                ClaimClass::Heuristic,
            ),
            claims::c(
                claims::GRAVITY,
                "A massless spin-2 is in the spectrum (closed string / graviton).",
                LayerId::Particle,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::UNIQUE_VACUUM,
                "The theory selects a unique vacuum (no landscape).",
                LayerId::Effective,
                ClaimClass::Heuristic,
            )
            .with_domain(DomainOfValidity::new(
                vec!["flux/moduli landscape".into()],
                vec!["landscape_log10 is a heuristic count, not a vacuum enumeration".into()],
                "This is the string compactification landscape (flux_bits, h11, h21), \
                 not a theorem that string theory is false. Observer-geometry, GR, and \
                 the SM are different FormalClaims of this slug. Using it outside those \
                 knobs is a new claim.",
            )),
            claims::c(
                claims::UV_COMPLETION,
                "The theory is a candidate UV completion of gravity plus matter.",
                LayerId::Field,
                ClaimClass::Conjecture,
            ),
        ]
    }

    fn evaluate(&self, claim: &Claim) -> Verdict {
        let w = self.build_world();
        match claim.id_str() {
            claims::SPACETIME_STRUCTURE => {
                if w.spacetime.structurally_ok() && self.extra() >= 0 {
                    Verdict::holds(claim, "dim, signature, and compact extras are consistent")
                } else {
                    Verdict::fails(claim, "spacetime numbers do not fit together").with_evidence([
                        format!(
                            "dim={} extra={} observed={}",
                            self.total_dim,
                            self.extra(),
                            self.observed_dim
                        ),
                    ])
                }
            }
            claims::CRITICAL_DIMENSION => {
                let crit = self.kind.critical_dim();
                let anomaly_note = match self.kind.worldsheet_central_charge() {
                    Some((cm, cg)) => {
                        format!("conformal anomaly cancels: {cm}·D {cg:+} = 0 ⇒ D = {crit}",)
                    }
                    None => "11D from supergravity; no worldsheet conformal anomaly".to_string(),
                };
                if self.total_dim == crit {
                    Verdict::holds(claim, format!("D={crit} equals the critical dimension"))
                        .with_evidence([anomaly_note])
                } else {
                    Verdict::fails(
                        claim,
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
                        claim,
                        if req {
                            "supersymmetry on, as the construction requires"
                        } else {
                            "supersymmetry off, as the bosonic string allows"
                        },
                    )
                } else if req && !self.supersymmetry {
                    Verdict::fails(
                        claim,
                        "superstring/M construction with supersymmetry switched off",
                    )
                } else {
                    Verdict::fails(
                        claim,
                        "bosonic string with supersymmetry switched on is a different theory",
                    )
                }
            }
            claims::NO_TACHYON => {
                let m2 = self.ground_state_mass_squared();
                if m2 >= 0.0 {
                    Verdict::holds(
                        claim,
                        format!("ground-state α'm² = {m2:.1} ≥ 0: no tachyon"),
                    )
                    .with_evidence([
                        "spacetime SUSY / GSO projection lifts the ground state to m² ≥ 0"
                            .to_string(),
                    ])
                } else {
                    Verdict::fails(claim,
                        format!("ground-state α'm² = {m2:.1} < 0: a tachyon"),
                    )
                    .with_evidence([
                        "same instability notion as the scalar field's min ω² < 0 (see field-modes)".to_string(),
                    ])
                }
            }
            claims::ANOMALY_CANCELLATION => match self.kind {
                StringKind::Bosonic => Verdict::inapplicable(
                    claim,
                    "non-chiral 26D string: gauge/gravitational anomalies are not the \
                     obstruction here (the tachyon is)",
                ),
                StringKind::TypeIIA | StringKind::TypeIIB => Verdict::holds(
                    claim,
                    "Type II 10D spectrum is anomaly-free (no chiral gauge anomaly to cancel)",
                ),
                StringKind::MTheory => Verdict::holds(
                    claim,
                    "11D supergravity is anomaly-free; boundary E₈ factors are the \
                     Hořava–Witten mechanism",
                ),
                StringKind::TypeI | StringKind::HeteroticSO32 | StringKind::HeteroticE8xE8 => {
                    if self.total_dim != 10 {
                        Verdict::undecidable(
                            claim,
                            "Green–Schwarz cancellation is a 10D statement; off the critical \
                             dimension this encoding does not assert it",
                        )
                    } else if w.gauge.gs_anomaly_free_10d() {
                        Verdict::holds(
                            claim,
                            format!("{} anomalies cancel via Green–Schwarz", w.gauge.name()),
                        )
                        .with_evidence([
                            "dimension-496 GS solution (SO(32) or E₈×E₈); encoded, not \
                             re-derived from the anomaly polynomial"
                                .to_string(),
                        ])
                    } else if self.missing_e8 && self.kind == StringKind::HeteroticE8xE8 {
                        Verdict::fails(
                            claim,
                            "missing E8: dimension 248 is not a Green-Schwarz solution",
                        )
                        .with_evidence([
                            "live encoding is E8 x E8 (dimension 496); a single E8 is not a 10D GS identity"
                                .to_string(),
                        ])
                    } else if self.so16 && self.kind == StringKind::HeteroticSO32 {
                        Verdict::fails(
                            claim,
                            "SO(16): dimension 120 is not a Green-Schwarz solution",
                        )
                        .with_evidence([
                            "live encoding is SO(32) (dimension 496); SO(16) is not a 10D GS identity"
                                .to_string(),
                        ])
                    } else if self.chan_paton_16 && self.kind == StringKind::TypeI {
                        Verdict::fails(
                            claim,
                            "Chan-Paton SO(16): dimension 120 is not a Green-Schwarz solution",
                        )
                        .with_evidence([
                            "live encoding is Chan-Paton SO(32) (dimension 496); Chan-Paton SO(16) is not a 10D GS identity"
                                .to_string(),
                        ])
                    } else {
                        Verdict::fails(
                            claim,
                            format!("{} is not a 10D Green–Schwarz solution", w.gauge.name()),
                        )
                    }
                }
            },
            claims::OBSERVED_4D => {
                if self.observed_dim == 4 {
                    Verdict::holds(claim, "observed_dim = 4")
                } else {
                    Verdict::fails(
                        claim,
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
                    Verdict::holds(claim, "no extra dimensions to hide")
                } else if r_eff.value() <= probe.value() {
                    Verdict::holds(
                        claim,
                        format!("effective R = {r_eff} is below the {probe} electroweak probe"),
                    )
                } else {
                    Verdict::fails(
                        claim,
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
                    Verdict::holds(claim, "fermions present in projected spectrum")
                } else {
                    Verdict::fails(claim, "bosonic string projection has no fermions")
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
                    Verdict::holds(claim,
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
                    Verdict::undecidable(claim,
                        "Type II / M have no 10D/11D GUT group; SM would have to arise from compactification / branes",
                    )
                } else {
                    Verdict::fails(
                        claim,
                        format!("{} does not contain SM in this encoding", w.gauge.name()),
                    )
                }
            }
            claims::THREE_GENERATIONS => {
                if self.kind == StringKind::Bosonic {
                    Verdict::fails(claim, "no fermions, so no generations")
                } else {
                    if self.euler_number != 0 && self.euler_number % 2 != 0 {
                        return Verdict::fails(
                            claim,
                            format!(
                                "χ = {} is odd; a Calabi–Yau threefold has an even Euler number",
                                self.euler_number
                            ),
                        );
                    }
                    match self.generations_from_topology() {
                        None => Verdict::undecidable(claim,
                            "generation count depends on the compactification topology (set euler_number)",
                        ),
                        Some(3) => Verdict::holds(claim,
                            format!(
                                "|χ|/2 = 3 generations from χ = {} — accommodated, not derived",
                                self.euler_number
                            ),
                        )
                        .with_evidence([
                            "the topological count |χ|/2 is a real theorem; but *why* χ = ±6 is not derived — this is the predictivity critique made mechanical".to_string(),
                        ]),
                        Some(g) => Verdict::fails(claim,
                            format!("|χ|/2 = {g} generations from χ = {}, not 3", self.euler_number),
                        ),
                    }
                }
            }
            claims::GRAVITY => {
                if w.has_gravity {
                    Verdict::holds(claim, "closed-string / 11D graviton in the spectrum")
                } else {
                    Verdict::fails(claim, "no graviton in projection")
                }
            }
            claims::UNIQUE_VACUUM => {
                let logn = self.landscape_log10();
                if logn < 0.5 {
                    Verdict::holds(claim, "landscape estimate is ~1 vacuum with current knobs")
                } else {
                    Verdict::fails(claim,
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
                    Verdict::holds(claim,
                        "internally consistent construction; UV-completeness is still a conjecture about nature",
                    )
                } else {
                    Verdict::fails(claim,
                        "construction knobs are off-critical or off-SUSY; not the usual UV candidate",
                    )
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by this string object"),
        }
    }

    fn ir_package(&self) -> Option<TheoryPackage> {
        matches!(self.id(), "heterotic-e8e8" | "heterotic-so32" | "type-i").then(|| self.package())
    }

    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        if pkg.id != self.id() {
            return Err(format!(
                "{} cannot reparse package id '{}'",
                self.id(),
                pkg.id
            ));
        }
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.missing_e8 = parsed.missing_e8;
        fork.so16 = parsed.so16;
        fork.chan_paton_16 = parsed.chan_paton_16;
        Ok(Box::new(fork))
    }

    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        if self.id() == "heterotic-e8e8" && !self.missing_e8 {
            let src = render_package(&self.package());
            let Ok(pkg) = parse_package(&src) else {
                return Vec::new();
            };
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::missing_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.missing_e8 {
                    let mut fork = self.clone();
                    fork.missing_e8 = true;
                    return vec![("add-missing-e8".into(), Box::new(fork))];
                }
            }
            return Vec::new();
        }
        if self.id() == "heterotic-so32" && !self.so16 {
            let src = render_package(&self.package());
            let Ok(pkg) = parse_package(&src) else {
                return Vec::new();
            };
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::so16_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.so16 {
                    let mut fork = self.clone();
                    fork.so16 = true;
                    return vec![("add-so16".into(), Box::new(fork))];
                }
            }
            return Vec::new();
        }
        if self.id() == "type-i" && !self.chan_paton_16 {
            let src = render_package(&self.package());
            let Ok(pkg) = parse_package(&src) else {
                return Vec::new();
            };
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::chan_paton_16_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.chan_paton_16 {
                    let mut fork = self.clone();
                    fork.chan_paton_16 = true;
                    return vec![("add-chan-paton-16".into(), Box::new(fork))];
                }
            }
        }
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::Theory;
    use physis_core::claim::VerdictKind;

    fn verdict(t: &StringTheory, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id_str() == id).unwrap();
        t.evaluate(&c).kind
    }

    #[test]
    fn observed_dim_is_measured_euler_number_is_chosen() {
        let t = StringTheory::type_iib();
        assert_eq!(
            t.spec("observed_dim").unwrap().origin,
            ParameterOrigin::Measured
        );
        assert_eq!(
            t.spec("compact_radius_planck").unwrap().origin,
            ParameterOrigin::Fitted
        );
        assert_eq!(t.spec("dilaton").unwrap().origin, ParameterOrigin::Fitted);
        assert_eq!(
            t.spec("euler_number").unwrap().origin,
            ParameterOrigin::Chosen
        );
        assert_eq!(t.spec("total_dim").unwrap().origin, ParameterOrigin::Chosen);
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
    fn unique_vacuum_names_the_landscape_regime() {
        let iib = StringTheory::type_iib();
        let het = StringTheory::heterotic_e8();
        let claim = |t: &StringTheory| {
            t.claims()
                .into_iter()
                .find(|c| c.id_str() == claims::UNIQUE_VACUUM)
                .unwrap()
        };
        let a = claim(&iib);
        let b = claim(&het);
        assert!(
            !a.domain().is_encoding_wide(),
            "string unique-vacuum must name the landscape, not encoding-wide: {:?}",
            a.domain()
        );
        assert!(
            a.domain()
                .regimes
                .iter()
                .any(|r| r.contains("flux/moduli landscape")),
            "string regime: {:?}",
            a.domain()
        );
        assert_eq!(
            a.statement_hash(),
            b.statement_hash(),
            "string constructions share one FormalClaim of unique-vacuum"
        );
    }

    #[test]
    fn critical_dimension_is_derived_from_the_conformal_anomaly() {
        // c_matter·D + c_ghost = 0 reproduces 26 (bosonic) and 10 (superstring).
        for kind in [
            StringKind::Bosonic,
            StringKind::TypeIIA,
            StringKind::TypeIIB,
            StringKind::TypeI,
            StringKind::HeteroticE8xE8,
            StringKind::HeteroticSO32,
        ] {
            assert_eq!(
                kind.critical_dim_from_anomaly(),
                Some(kind.critical_dim()),
                "{}",
                kind.as_str()
            );
        }
        // M-theory is not a worldsheet theory: no conformal anomaly.
        assert_eq!(StringKind::MTheory.critical_dim_from_anomaly(), None);
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
    fn tachyon_is_the_sign_of_the_ground_state_mass_squared() {
        // The string tachyon shares the field's stability notion: m² < 0.
        assert!(StringTheory::bosonic().ground_state_mass_squared() < 0.0);
        assert!(StringTheory::type_iib().ground_state_mass_squared() >= 0.0);
        let mut no_susy = StringTheory::type_iib();
        no_susy
            .set("supersymmetry", KnobValue::Bool(false))
            .unwrap();
        assert!(no_susy.ground_state_mass_squared() < 0.0);
        assert_eq!(verdict(&no_susy, claims::NO_TACHYON), VerdictKind::Fails);
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

    fn kind(t: &dyn Theory, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id_str() == id).unwrap();
        t.evaluate(&c).kind
    }

    #[test]
    fn missing_e8_is_ir_not_a_knob() {
        assert!(
            StringTheory::heterotic_e8()
                .set("missing_e8", KnobValue::Bool(true))
                .is_err(),
            "missing E8 is an IR mutation, not a knob"
        );
        assert!(
            StringTheory::heterotic_e8()
                .set("missing-e8", KnobValue::Bool(true))
                .is_err(),
            "missing-e8 is not a knob"
        );
        assert!(
            StringTheory::heterotic_e8()
                .set("add-missing-e8", KnobValue::Bool(true))
                .is_err(),
            "add-missing-e8 is not a knob"
        );
        let het = StringTheory::heterotic_e8();
        assert!(!het.missing_e8);
        let src = render_package(&het.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(pkg.equations.len(), 1, "live package must stay complete");
        assert_eq!(pkg.equations[0], E8E8_EQ);
        assert_eq!(
            StringTheory::from_package(&pkg).unwrap(),
            het,
            "IR round-trip must preserve E8 x E8"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(StringTheory::missing_equation()),
        );
        let parsed = StringTheory::from_package(&mutated).unwrap();
        assert!(parsed.missing_e8);
        let mut fork = het.clone();
        fork.missing_e8 = true;
        assert_eq!(fork.id(), "heterotic-e8e8");
        let gs = fork.evaluate(
            &fork
                .claims()
                .into_iter()
                .find(|c| c.id_str() == claims::ANOMALY_CANCELLATION)
                .unwrap(),
        );
        assert_eq!(gs.kind, VerdictKind::Fails);
        assert!(
            gs.summary.contains("missing E8") && gs.summary.contains("248"),
            "missing E8 must name dimension 248: {}",
            gs.summary
        );
        assert!(
            !gs.summary.contains("kind")
                && !gs.summary.contains("total_dim")
                && !gs.summary.contains("supersymmetry")
                && !gs.summary.contains("euler_number")
                && !gs.summary.contains("flux")
                && !gs.summary.contains("Higgs"),
            "missing E8 is not a knob: {}",
            gs.summary
        );
        assert_eq!(kind(&fork, claims::SM_GAUGE), VerdictKind::Holds);
        assert_eq!(kind(&fork, claims::CRITICAL_DIMENSION), VerdictKind::Holds);
        assert_eq!(kind(&fork, claims::NO_TACHYON), VerdictKind::Holds);
        assert_eq!(kind(&fork, claims::GRAVITY), VerdictKind::Holds);
        assert_eq!(kind(&fork, claims::UNIQUE_VACUUM), VerdictKind::Fails);
        assert_eq!(kind(&het, claims::ANOMALY_CANCELLATION), VerdictKind::Holds);
        assert_eq!(kind(&het, claims::SM_GAUGE), VerdictKind::Holds);

        let probes = StringTheory::heterotic_e8().structural_mutations();
        assert!(
            probes.iter().any(|(label, _)| label == "add-missing-e8"),
            "live heterotic-e8e8 must offer add-missing-e8: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let probe = probes
            .iter()
            .find(|(label, _)| label == "add-missing-e8")
            .expect("add-missing-e8");
        assert_eq!(
            kind(probe.1.as_ref(), claims::ANOMALY_CANCELLATION),
            VerdictKind::Fails
        );
        assert_eq!(kind(probe.1.as_ref(), claims::SM_GAUGE), VerdictKind::Holds);
        assert_eq!(
            kind(probe.1.as_ref(), claims::UNIQUE_VACUUM),
            VerdictKind::Fails
        );
        let fork_probes = fork.structural_mutations();
        assert!(
            fork_probes
                .iter()
                .all(|(label, _)| label != "add-missing-e8"),
            "missing-e8 fork must not re-offer add-missing-e8"
        );
        let live = StringTheory::heterotic_e8();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let mut nine = StringTheory::heterotic_e8();
        nine.set("total_dim", KnobValue::UInt(9)).unwrap();
        let rebuilt = nine.reparse_package(&parsed).unwrap();
        assert_eq!(
            rebuilt.get("total_dim").unwrap(),
            KnobValue::UInt(9),
            "reparse must overlay missing-e8 IR onto live knobs"
        );
        assert_eq!(
            kind(rebuilt.as_ref(), claims::ANOMALY_CANCELLATION),
            VerdictKind::Undecidable,
            "off-critical live E8 x E8 stays Undecidable for Green-Schwarz"
        );
        let live_rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(
            kind(live_rebuilt.as_ref(), claims::ANOMALY_CANCELLATION),
            VerdictKind::Holds
        );
        assert!(
            StringTheory::type_iib()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-missing-e8"),
            "type-iib must not grow add-missing-e8"
        );
        assert!(
            crate::gut::Su5Gut::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-missing-e8"),
            "su5-gut must not grow add-missing-e8"
        );
        assert!(
            crate::standard_model::StandardModel::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-missing-e8"),
            "standard-model must not grow add-missing-e8"
        );
        assert!(
            crate::solid::EinsteinSolid::debye()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-missing-e8"),
            "debye-solid must not grow add-missing-e8"
        );
        assert!(
            crate::solid::EinsteinSolid::dulong_petit()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-missing-e8"),
            "dulong-petit must not grow add-missing-e8"
        );
        assert!(
            crate::geometry::ObserverGeometry::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-missing-e8"),
            "observer-geometry must not grow add-missing-e8"
        );
        assert!(
            StringTheory::heterotic_e8()
                .set("kind", KnobValue::Choice("bosonic".into()))
                .is_ok(),
            "heterotic-e8e8 keeps the kind knob"
        );
        assert!(
            StringTheory::heterotic_e8()
                .set("total_dim", KnobValue::UInt(9))
                .is_ok(),
            "heterotic-e8e8 keeps the total_dim knob"
        );
        assert!(StringTheory::type_iib().ir_package().is_none());
        assert!(StringTheory::type_iia().ir_package().is_none());
        assert!(StringTheory::bosonic().ir_package().is_none());
        assert!(StringTheory::m_theory().ir_package().is_none());
        assert!(StringTheory::heterotic_e8().ir_package().is_some());
        assert!(StringTheory::type_i().ir_package().is_some());
        assert!(
            probes.iter().all(|(label, _)| label != "add-so16"),
            "heterotic-e8e8 must not grow add-so16"
        );
        assert!(
            probes.iter().all(|(label, _)| label != "add-chan-paton-16"),
            "heterotic-e8e8 must not grow add-chan-paton-16"
        );

        let gs_claim = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::ANOMALY_CANCELLATION)
            .unwrap();
        assert!(
            !gs_claim.domain().is_encoding_wide(),
            "heterotic GS must name E8 x E8: {:?}",
            gs_claim.domain()
        );
        assert!(
            gs_claim
                .domain()
                .regimes
                .iter()
                .any(|r| r.contains("E8 x E8")),
            "heterotic GS regime: {:?}",
            gs_claim.domain()
        );
        assert!(
            !gs_claim.domain().notes.contains("theory "),
            "heterotic GS notes must not split why_theory_block: {:?}",
            gs_claim.domain()
        );
        let iib_gs = StringTheory::type_iib()
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::ANOMALY_CANCELLATION)
            .unwrap();
        assert!(
            iib_gs.domain().is_encoding_wide(),
            "Type II Green-Schwarz stays encoding-wide: {:?}",
            iib_gs.domain()
        );
        assert_ne!(
            gs_claim.statement_hash(),
            iib_gs.statement_hash(),
            "heterotic GS is a distinct FormalClaim from Type II"
        );
    }

    #[test]
    fn so16_is_ir_not_a_knob() {
        assert!(
            StringTheory::heterotic_so32()
                .set("so16", KnobValue::Bool(true))
                .is_err(),
            "SO(16) is an IR mutation, not a knob"
        );
        assert!(
            StringTheory::heterotic_so32()
                .set("so-16", KnobValue::Bool(true))
                .is_err(),
            "so-16 is not a knob"
        );
        assert!(
            StringTheory::heterotic_so32()
                .set("add-so16", KnobValue::Bool(true))
                .is_err(),
            "add-so16 is not a knob"
        );
        let het = StringTheory::heterotic_so32();
        assert!(!het.so16);
        let src = render_package(&het.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(pkg.equations.len(), 1, "live package must stay complete");
        assert_eq!(pkg.equations[0], SO32_EQ);
        assert_eq!(
            StringTheory::from_package(&pkg).unwrap(),
            het,
            "IR round-trip must preserve SO(32)"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(StringTheory::so16_equation()),
        );
        let parsed = StringTheory::from_package(&mutated).unwrap();
        assert!(parsed.so16);
        let mut fork = het.clone();
        fork.so16 = true;
        assert_eq!(fork.id(), "heterotic-so32");
        let gs = fork.evaluate(
            &fork
                .claims()
                .into_iter()
                .find(|c| c.id_str() == claims::ANOMALY_CANCELLATION)
                .unwrap(),
        );
        assert_eq!(gs.kind, VerdictKind::Fails);
        assert!(
            gs.summary.contains("SO(16)") && gs.summary.contains("120"),
            "SO(16) must name dimension 120: {}",
            gs.summary
        );
        assert!(
            !gs.summary.contains("kind")
                && !gs.summary.contains("total_dim")
                && !gs.summary.contains("supersymmetry")
                && !gs.summary.contains("euler_number")
                && !gs.summary.contains("flux")
                && !gs.summary.contains("Higgs"),
            "SO(16) is not a knob: {}",
            gs.summary
        );
        assert_eq!(kind(&fork, claims::SM_GAUGE), VerdictKind::Holds);
        assert_eq!(kind(&fork, claims::CRITICAL_DIMENSION), VerdictKind::Holds);
        assert_eq!(kind(&fork, claims::NO_TACHYON), VerdictKind::Holds);
        assert_eq!(kind(&fork, claims::GRAVITY), VerdictKind::Holds);
        assert_eq!(kind(&fork, claims::UNIQUE_VACUUM), VerdictKind::Fails);
        assert_eq!(kind(&het, claims::ANOMALY_CANCELLATION), VerdictKind::Holds);
        assert_eq!(kind(&het, claims::SM_GAUGE), VerdictKind::Holds);

        let probes = StringTheory::heterotic_so32().structural_mutations();
        assert!(
            probes.iter().any(|(label, _)| label == "add-so16"),
            "live heterotic-so32 must offer add-so16: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        assert!(
            probes.iter().all(|(label, _)| label != "add-missing-e8"),
            "heterotic-so32 must not grow add-missing-e8"
        );
        let probe = probes
            .iter()
            .find(|(label, _)| label == "add-so16")
            .expect("add-so16");
        assert_eq!(
            kind(probe.1.as_ref(), claims::ANOMALY_CANCELLATION),
            VerdictKind::Fails
        );
        assert_eq!(kind(probe.1.as_ref(), claims::SM_GAUGE), VerdictKind::Holds);
        assert_eq!(
            kind(probe.1.as_ref(), claims::UNIQUE_VACUUM),
            VerdictKind::Fails
        );
        let fork_probes = fork.structural_mutations();
        assert!(
            fork_probes.iter().all(|(label, _)| label != "add-so16"),
            "so16 fork must not re-offer add-so16"
        );
        let live = StringTheory::heterotic_so32();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let mut nine = StringTheory::heterotic_so32();
        nine.set("total_dim", KnobValue::UInt(9)).unwrap();
        let rebuilt = nine.reparse_package(&parsed).unwrap();
        assert_eq!(
            rebuilt.get("total_dim").unwrap(),
            KnobValue::UInt(9),
            "reparse must overlay so16 IR onto live knobs"
        );
        assert_eq!(
            kind(rebuilt.as_ref(), claims::ANOMALY_CANCELLATION),
            VerdictKind::Undecidable,
            "off-critical live SO(32) stays Undecidable for Green-Schwarz"
        );
        let live_rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(
            kind(live_rebuilt.as_ref(), claims::ANOMALY_CANCELLATION),
            VerdictKind::Holds
        );
        assert!(
            StringTheory::type_iib()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-so16"),
            "type-iib must not grow add-so16"
        );
        assert!(
            StringTheory::type_i()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-so16"),
            "type-i must not grow add-so16"
        );
        assert!(
            StringTheory::heterotic_e8()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-so16"),
            "heterotic-e8e8 must not grow add-so16"
        );
        assert!(
            crate::gut::Su5Gut::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-so16"),
            "su5-gut must not grow add-so16"
        );
        assert!(
            crate::standard_model::StandardModel::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-so16"),
            "standard-model must not grow add-so16"
        );
        assert!(
            crate::solid::EinsteinSolid::debye()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-so16"),
            "debye-solid must not grow add-so16"
        );
        assert!(
            crate::solid::EinsteinSolid::dulong_petit()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-so16"),
            "dulong-petit must not grow add-so16"
        );
        assert!(
            crate::geometry::ObserverGeometry::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-so16"),
            "observer-geometry must not grow add-so16"
        );
        assert!(
            StringTheory::heterotic_so32()
                .set("kind", KnobValue::Choice("bosonic".into()))
                .is_ok(),
            "heterotic-so32 keeps the kind knob"
        );
        assert!(
            StringTheory::heterotic_so32()
                .set("total_dim", KnobValue::UInt(9))
                .is_ok(),
            "heterotic-so32 keeps the total_dim knob"
        );
        assert!(StringTheory::type_iib().ir_package().is_none());
        assert!(StringTheory::type_iia().ir_package().is_none());
        assert!(StringTheory::bosonic().ir_package().is_none());
        assert!(StringTheory::m_theory().ir_package().is_none());
        assert!(StringTheory::heterotic_so32().ir_package().is_some());
        assert!(StringTheory::heterotic_e8().ir_package().is_some());
        assert!(StringTheory::type_i().ir_package().is_some());
        assert!(
            probes.iter().all(|(label, _)| label != "add-chan-paton-16"),
            "heterotic-so32 must not grow add-chan-paton-16"
        );

        let gs_claim = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::ANOMALY_CANCELLATION)
            .unwrap();
        assert!(
            !gs_claim.domain().is_encoding_wide(),
            "heterotic-so32 GS must name SO(32): {:?}",
            gs_claim.domain()
        );
        assert!(
            gs_claim
                .domain()
                .regimes
                .iter()
                .any(|r| r.contains("SO(32)")),
            "heterotic-so32 GS regime: {:?}",
            gs_claim.domain()
        );
        assert!(
            !gs_claim.domain().notes.contains("theory "),
            "heterotic-so32 GS notes must not split why_theory_block: {:?}",
            gs_claim.domain()
        );
        let type_i_gs = StringTheory::type_i()
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::ANOMALY_CANCELLATION)
            .unwrap();
        assert!(
            !type_i_gs.domain().is_encoding_wide(),
            "Type I Green-Schwarz names Chan-Paton SO(32): {:?}",
            type_i_gs.domain()
        );
        assert!(
            type_i_gs
                .domain()
                .regimes
                .iter()
                .any(|r| r.contains("Chan-Paton SO(32)")),
            "Type I GS regime: {:?}",
            type_i_gs.domain()
        );
        assert_ne!(
            gs_claim.statement_hash(),
            type_i_gs.statement_hash(),
            "heterotic-so32 GS is a distinct FormalClaim from Type I"
        );
        let iib_gs = StringTheory::type_iib()
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::ANOMALY_CANCELLATION)
            .unwrap();
        assert!(
            iib_gs.domain().is_encoding_wide(),
            "Type II Green-Schwarz stays encoding-wide: {:?}",
            iib_gs.domain()
        );
        let e8e8_gs = StringTheory::heterotic_e8()
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::ANOMALY_CANCELLATION)
            .unwrap();
        assert_ne!(
            gs_claim.statement_hash(),
            e8e8_gs.statement_hash(),
            "heterotic-so32 GS is a distinct FormalClaim from heterotic-e8e8"
        );
    }

    #[test]
    fn chan_paton_16_is_ir_not_a_knob() {
        assert!(
            StringTheory::type_i()
                .set("chan_paton_16", KnobValue::Bool(true))
                .is_err(),
            "Chan-Paton SO(16) is an IR mutation, not a knob"
        );
        assert!(
            StringTheory::type_i()
                .set("chan-paton-16", KnobValue::Bool(true))
                .is_err(),
            "chan-paton-16 is not a knob"
        );
        assert!(
            StringTheory::type_i()
                .set("add-chan-paton-16", KnobValue::Bool(true))
                .is_err(),
            "add-chan-paton-16 is not a knob"
        );
        let het = StringTheory::type_i();
        assert!(!het.chan_paton_16);
        let src = render_package(&het.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(pkg.equations.len(), 1, "live package must stay complete");
        assert_eq!(pkg.equations[0], CHAN_PATON_SO32_EQ);
        assert_eq!(
            StringTheory::from_package(&pkg).unwrap(),
            het,
            "IR round-trip must preserve Chan-Paton SO(32)"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(StringTheory::chan_paton_16_equation()),
        );
        let parsed = StringTheory::from_package(&mutated).unwrap();
        assert!(parsed.chan_paton_16);
        let mut fork = het.clone();
        fork.chan_paton_16 = true;
        assert_eq!(fork.id(), "type-i");
        let gs = fork.evaluate(
            &fork
                .claims()
                .into_iter()
                .find(|c| c.id_str() == claims::ANOMALY_CANCELLATION)
                .unwrap(),
        );
        assert_eq!(gs.kind, VerdictKind::Fails);
        assert!(
            gs.summary.contains("Chan-Paton SO(16)") && gs.summary.contains("120"),
            "Chan-Paton SO(16) must name dimension 120: {}",
            gs.summary
        );
        assert!(
            !gs.summary.contains("kind")
                && !gs.summary.contains("total_dim")
                && !gs.summary.contains("supersymmetry")
                && !gs.summary.contains("euler_number")
                && !gs.summary.contains("flux")
                && !gs.summary.contains("Higgs"),
            "Chan-Paton SO(16) is not a knob: {}",
            gs.summary
        );
        assert_eq!(kind(&fork, claims::SM_GAUGE), VerdictKind::Holds);
        assert_eq!(kind(&fork, claims::CRITICAL_DIMENSION), VerdictKind::Holds);
        assert_eq!(kind(&fork, claims::NO_TACHYON), VerdictKind::Holds);
        assert_eq!(kind(&fork, claims::GRAVITY), VerdictKind::Holds);
        assert_eq!(kind(&fork, claims::UNIQUE_VACUUM), VerdictKind::Fails);
        assert_eq!(kind(&het, claims::ANOMALY_CANCELLATION), VerdictKind::Holds);
        assert_eq!(kind(&het, claims::SM_GAUGE), VerdictKind::Holds);

        let probes = StringTheory::type_i().structural_mutations();
        assert!(
            probes.iter().any(|(label, _)| label == "add-chan-paton-16"),
            "live type-i must offer add-chan-paton-16: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        assert!(
            probes.iter().all(|(label, _)| label != "add-so16"),
            "type-i must not grow add-so16"
        );
        assert!(
            probes.iter().all(|(label, _)| label != "add-missing-e8"),
            "type-i must not grow add-missing-e8"
        );
        let probe = probes
            .iter()
            .find(|(label, _)| label == "add-chan-paton-16")
            .expect("add-chan-paton-16");
        assert_eq!(
            kind(probe.1.as_ref(), claims::ANOMALY_CANCELLATION),
            VerdictKind::Fails
        );
        assert_eq!(kind(probe.1.as_ref(), claims::SM_GAUGE), VerdictKind::Holds);
        assert_eq!(
            kind(probe.1.as_ref(), claims::UNIQUE_VACUUM),
            VerdictKind::Fails
        );
        let fork_probes = fork.structural_mutations();
        assert!(
            fork_probes
                .iter()
                .all(|(label, _)| label != "add-chan-paton-16"),
            "chan-paton-16 fork must not re-offer add-chan-paton-16"
        );
        let live = StringTheory::type_i();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let mut nine = StringTheory::type_i();
        nine.set("total_dim", KnobValue::UInt(9)).unwrap();
        let rebuilt = nine.reparse_package(&parsed).unwrap();
        assert_eq!(
            rebuilt.get("total_dim").unwrap(),
            KnobValue::UInt(9),
            "reparse must overlay Chan-Paton IR onto live knobs"
        );
        assert_eq!(
            kind(rebuilt.as_ref(), claims::ANOMALY_CANCELLATION),
            VerdictKind::Undecidable,
            "off-critical live Chan-Paton SO(32) stays Undecidable for Green-Schwarz"
        );
        let live_rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(
            kind(live_rebuilt.as_ref(), claims::ANOMALY_CANCELLATION),
            VerdictKind::Holds
        );
        assert!(
            StringTheory::type_iib()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-chan-paton-16"),
            "type-iib must not grow add-chan-paton-16"
        );
        assert!(
            StringTheory::heterotic_so32()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-chan-paton-16"),
            "heterotic-so32 must not grow add-chan-paton-16"
        );
        assert!(
            StringTheory::heterotic_e8()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-chan-paton-16"),
            "heterotic-e8e8 must not grow add-chan-paton-16"
        );
        assert!(
            crate::gut::Su5Gut::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-chan-paton-16"),
            "su5-gut must not grow add-chan-paton-16"
        );
        assert!(
            crate::standard_model::StandardModel::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-chan-paton-16"),
            "standard-model must not grow add-chan-paton-16"
        );
        assert!(
            crate::solid::EinsteinSolid::debye()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-chan-paton-16"),
            "debye-solid must not grow add-chan-paton-16"
        );
        assert!(
            crate::solid::EinsteinSolid::dulong_petit()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-chan-paton-16"),
            "dulong-petit must not grow add-chan-paton-16"
        );
        assert!(
            crate::geometry::ObserverGeometry::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-chan-paton-16"),
            "observer-geometry must not grow add-chan-paton-16"
        );
        assert!(
            StringTheory::type_i()
                .set("kind", KnobValue::Choice("bosonic".into()))
                .is_ok(),
            "type-i keeps the kind knob"
        );
        assert!(
            StringTheory::type_i()
                .set("total_dim", KnobValue::UInt(9))
                .is_ok(),
            "type-i keeps the total_dim knob"
        );
        assert!(StringTheory::type_iib().ir_package().is_none());
        assert!(StringTheory::type_iia().ir_package().is_none());
        assert!(StringTheory::bosonic().ir_package().is_none());
        assert!(StringTheory::m_theory().ir_package().is_none());
        assert!(StringTheory::type_i().ir_package().is_some());
        assert!(StringTheory::heterotic_so32().ir_package().is_some());
        assert!(StringTheory::heterotic_e8().ir_package().is_some());

        let gs_claim = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::ANOMALY_CANCELLATION)
            .unwrap();
        assert!(
            !gs_claim.domain().is_encoding_wide(),
            "type-i GS must name Chan-Paton SO(32): {:?}",
            gs_claim.domain()
        );
        assert!(
            gs_claim
                .domain()
                .regimes
                .iter()
                .any(|r| r.contains("Chan-Paton SO(32)")),
            "type-i GS regime: {:?}",
            gs_claim.domain()
        );
        assert!(
            !gs_claim.domain().notes.contains("theory "),
            "type-i GS notes must not split why_theory_block: {:?}",
            gs_claim.domain()
        );
        let iib_gs = StringTheory::type_iib()
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::ANOMALY_CANCELLATION)
            .unwrap();
        assert!(
            iib_gs.domain().is_encoding_wide(),
            "Type II Green-Schwarz stays encoding-wide: {:?}",
            iib_gs.domain()
        );
        assert_ne!(
            gs_claim.statement_hash(),
            iib_gs.statement_hash(),
            "type-i GS is a distinct FormalClaim from Type II"
        );
        let so32_gs = StringTheory::heterotic_so32()
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::ANOMALY_CANCELLATION)
            .unwrap();
        assert_ne!(
            gs_claim.statement_hash(),
            so32_gs.statement_hash(),
            "type-i GS is a distinct FormalClaim from heterotic-so32"
        );
    }
}
