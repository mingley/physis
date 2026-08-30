//! Continuum (M4), gauge sector: a gauge field as **link objects**, not a flag.
//!
//! [`WilsonU1`] is a compact U(1) lattice gauge theory. The degrees of freedom
//! live on the links between sites; the unimproved Wilson action sums
//! `1 - cos(θ)` over 1×1 plaquettes. That stencil lives on the IR package.
//! A 2×1 rectangle term is a package mutation (`add-rectangle`), not a knob:
//! `gauge.local` fails on the mutant. Gauge invariance remains a structural
//! theorem of any Wilson loop. The confinement/deconfinement behaviour is a
//! knob-sensitive, honestly-labelled result of lattice gauge theory:
//!
//! - compact U(1) confines at all couplings in 2D and 3D;
//! - in 4D it has a phase transition near `β ≈ 1.01`: confining below, a
//!   Coulomb (free-photon) phase above.
//!
//! In two dimensions the theory is **exactly solvable**: the gauge integral
//! factorizes plaquette by plaquette, so the fundamental Wilson loop of area `A`
//! is exactly `⟨W⟩ = (I₁(β)/I₀(β))ᴬ`, an area law with string tension
//! `σ = −ln(I₁(β)/I₀(β)) > 0` at *every* finite coupling. That exact confinement
//! result (`gauge.exact-area-law-2d`) is a theorem, in honest contrast to the
//! 4D Yang–Mills mass gap, which stays a `conjecture`.

use physis_core::assumption::DomainOfValidity;
use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::ParameterOrigin;
use physis_ir::{apply_mutation, parse_package, render_package, PackageMutation, TheoryPackage};
use physis_model::{GaugeGroup, Manifold, SimpleGroup, Spectrum, World};

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;

/// Unimproved Wilson stencil: 1×1 plaquettes.
const PLAQUETTE_EQ: &str = "wilson-plaquette 1x1";
/// Rectangle (Symanzik / next-nearest) term: 2×1 Wilson loops.
const RECTANGLE_EQ: &str = "wilson-rectangle 2x1";

fn gauge_local_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["nearest-neighbour Wilson plaquettes".into()],
        vec!["unimproved 1x1 plaquette action".into()],
        "Locality here is the 1x1 Wilson stencil. A 2x1 rectangle term is a new \
         encoding, not a silent local action.",
    )
}

fn parse_wilson_stencil(pkg: &TheoryPackage) -> Result<bool, String> {
    let mut plaquette = false;
    let mut rectangle = false;
    for eq in &pkg.equations {
        match eq.trim() {
            PLAQUETTE_EQ => plaquette = true,
            RECTANGLE_EQ => rectangle = true,
            _ => {}
        }
    }
    if !plaquette {
        return Err(format!("{} package has no 1x1 Wilson plaquette", pkg.id));
    }
    Ok(rectangle)
}

fn wilson_package(id: &str, name: &str, rectangle: bool) -> TheoryPackage {
    let mut equations = vec![PLAQUETTE_EQ.to_string()];
    if rectangle {
        equations.push(RECTANGLE_EQ.to_string());
    }
    TheoryPackage {
        id: id.to_string(),
        name: name.to_string(),
        parameters: vec![],
        assumptions: vec!["unimproved-wilson-plaquettes".into()],
        equations,
        claims: vec![physis_ir::ClaimDecl {
            id: GAUGE_LOCAL.into(),
            statement: "The action couples only neighbouring links (plaquettes).".into(),
            layer: "interaction".into(),
            class: "model-internal".into(),
        }],
        lean_ref: None,
    }
}

/// The action is invariant under local gauge transformations of the links.
pub const GAUGE_INVARIANT: &str = "gauge.invariant";
/// The action couples only neighbouring links (plaquettes).
pub const GAUGE_LOCAL: &str = "gauge.local";
/// Static charges are confined (area law / linear potential).
pub const CONFINING: &str = "gauge.confining";
/// The coupling runs to zero at high energy (asymptotic freedom).
pub const ASYMPTOTIC_FREEDOM: &str = "gauge.asymptotic-freedom";
/// The leading strong-coupling expansion yields an area law (σ > 0).
pub const STRONG_COUPLING_AREA_LAW: &str = "gauge.strong-coupling-area-law";
/// In 2D the Wilson loop obeys an *exact* area law at all couplings.
pub const EXACT_AREA_LAW_2D: &str = "gauge.exact-area-law-2d";

fn area_law_2d_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["2D Wilson lattice".into()],
        vec!["exact plaquette factorization".into()],
        "The exact area law is two-dimensional. In D > 2 this cell is inapplicable; \
         4D confinement is gauge.confining, not this identity.",
    )
}

/// Ratio `I₁(x)/I₀(x)` of modified Bessel functions, from their convergent
/// power series. Stable for the `β` range this lab uses (no factorial overflow:
/// each term is built from the previous by a bounded ratio).
///
/// `I₀(x) = Σ (x²/4)ᵏ/(k!)²` and `I₁(x) = (x/2)·Σ (x²/4)ᵏ/(k!(k+1)!)`.
fn bessel_i1_over_i0(x: f64) -> f64 {
    let z = x * x / 4.0;
    let (mut i0, mut term0) = (1.0_f64, 1.0_f64);
    let (mut i1_series, mut term1) = (1.0_f64, 1.0_f64);
    for k in 1..10_000 {
        let kf = k as f64;
        term0 *= z / (kf * kf);
        i0 += term0;
        term1 *= z / (kf * (kf + 1.0));
        i1_series += term1;
        if term0 <= 1e-18 * i0 && term1 <= 1e-18 * i1_series {
            break;
        }
    }
    (x / 2.0) * i1_series / i0
}

/// The quadratic Casimir of the fundamental representation of SU(N),
/// `C₂(fund) = (N²−1)/(2N)`.
fn su_casimir_fundamental(n: f64) -> f64 {
    (n * n - 1.0) / (2.0 * n)
}

/// The **exact** string tension of 2D SU(N) Yang–Mills for a fundamental Wilson
/// loop, `σ = (g²/2)·C₂(fund) = (N²−1)/(2β)` with `β = 2N/g²`.
///
/// Two-dimensional Yang–Mills is exactly solvable (Migdal, Witten): the
/// fundamental Wilson loop obeys an exact area law `⟨W⟩ = exp(−σ·Area)`. Since
/// `σ > 0` for every finite `β`, 2D SU(N) confines at **all** couplings — a
/// theorem, the non-abelian analogue of the 2D compact-U(1) result.
fn exact_2d_string_tension_sun(n: f64, beta: f64) -> f64 {
    // σ = (g²/2)·C₂ with g² = 2N/β ⇒ σ = (N/β)·C₂(fund) = (N²−1)/(2β).
    (n / beta) * su_casimir_fundamental(n)
}

/// The **exact** string tension of 2D compact U(1) lattice gauge theory,
/// `σ = −ln(I₁(β)/I₀(β))`.
///
/// In two dimensions the gauge integral factorizes plaquette by plaquette, so
/// the fundamental Wilson loop of area `A` (in plaquettes) is exactly
/// `⟨W⟩ = (I₁(β)/I₀(β))ᴬ = e^{−σA}`. Since `0 < I₁/I₀ < 1` for every finite `β`,
/// `σ > 0` always: 2D compact U(1) confines at **all** couplings. This is a
/// theorem, not a strong-coupling approximation.
fn exact_2d_string_tension(beta: f64) -> f64 {
    -bessel_i1_over_i0(beta).ln()
}

/// Matrix rows for the lattice-gauge lab.
pub fn gauge_rows() -> [&'static str; 5] {
    [
        GAUGE_INVARIANT,
        GAUGE_LOCAL,
        CONFINING,
        ASYMPTOTIC_FREEDOM,
        STRONG_COUPLING_AREA_LAW,
    ]
}

/// Leading-order strong-coupling string tension `σ = −ln(β / 2N²)`.
///
/// This is the first term of the convergent strong-coupling (high-temperature)
/// expansion of the fundamental Wilson loop. `σ > 0` is a genuine area law
/// (confinement) in that regime; the expansion breaks down at large β. `n_group`
/// is the SU(N) rank for non-abelian groups, and 1 for compact U(1).
fn strong_coupling_string_tension(beta: f64, n_group: f64) -> f64 {
    -(beta / (2.0 * n_group * n_group)).ln()
}

/// Verdict for the computed strong-coupling area law.
fn strong_coupling_verdict(beta: f64, n_group: f64, claim: &Claim) -> Verdict {
    let sigma = strong_coupling_string_tension(beta, n_group);
    if sigma > 0.0 {
        Verdict::holds(
            claim,
            format!("leading strong-coupling string tension σ = {sigma:.3} > 0: area law"),
        )
        .with_evidence([
            "first term of the convergent strong-coupling expansion of the Wilson loop".to_string(),
        ])
    } else {
        Verdict::fails(claim,
            format!(
                "σ = {sigma:.3} ≤ 0 at β = {beta}: the strong-coupling expansion gives no area law here"
            ),
        )
    }
}

/// Approximate 4D compact-U(1) deconfinement coupling (β = 1/g²).
const BETA_C_4D: f64 = 1.01;

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "dimension",
        layer: LayerId::Spacetime,
        doc: "Lattice spacetime dimension (2–4). Compact U(1) confines at all β in 2D/3D.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 2, max: 4 },
    },
    KnobSpec {
        name: "beta",
        layer: LayerId::Interaction,
        doc: "Inverse coupling β = 1/g². In 4D, β below ~1.01 confines; above it is the Coulomb phase.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 0.0,
            max: 100.0,
        },
    },
    KnobSpec {
        name: "sites_per_side",
        layer: LayerId::Spacetime,
        doc: "Linear lattice size L (the lattice has L^dimension sites).",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 2, max: 256 },
    },
];

/// Compact U(1) lattice gauge theory (Wilson action).
///
/// The unimproved 1×1 plaquette stencil lives on the IR package.
/// A 2×1 rectangle term is a package mutation (`add-rectangle`), not a knob.
#[derive(Clone, Debug, PartialEq)]
pub struct WilsonU1 {
    dimension: u8,
    beta: f64,
    sites_per_side: u32,
    rectangle: bool,
}

impl Default for WilsonU1 {
    fn default() -> Self {
        // 4D at β = 1.0: just inside the confining phase.
        Self {
            dimension: 4,
            beta: 1.0,
            sites_per_side: 8,
            rectangle: false,
        }
    }
}

impl WilsonU1 {
    fn is_confining(&self) -> bool {
        match self.dimension {
            2 | 3 => true,              // compact U(1) always confines here
            _ => self.beta < BETA_C_4D, // 4D: confining below the transition
        }
    }

    /// IR package for this Wilson stencil. Equations are `wilson-plaquette 1x1`
    /// and, when forked, `wilson-rectangle 2x1`. Knobs stay on the struct.
    pub fn package(&self) -> TheoryPackage {
        wilson_package(self.id(), self.name(), self.rectangle)
    }

    /// Load a Wilson stencil from a package. Knobs default; overlay them from a
    /// live field when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        Ok(Self {
            rectangle: parse_wilson_stencil(pkg)?,
            ..Self::default()
        })
    }

    fn rectangle_equation() -> String {
        RECTANGLE_EQ.to_string()
    }
}

impl Knobbed for WilsonU1 {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "dimension" => Ok(KnobValue::UInt(self.dimension as u64)),
            "beta" => Ok(KnobValue::Float(self.beta)),
            "sites_per_side" => Ok(KnobValue::UInt(self.sites_per_side as u64)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("dimension", KnobValue::UInt(v)) => self.dimension = v as u8,
            ("beta", KnobValue::Float(v)) => self.beta = v,
            ("sites_per_side", KnobValue::UInt(v)) => self.sites_per_side = v as u32,
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

impl Theory for WilsonU1 {
    fn id(&self) -> &'static str {
        "wilson-u1"
    }
    fn name(&self) -> &'static str {
        "Wilson U(1) lattice gauge"
    }
    fn summary(&self) -> &'static str {
        "Compact U(1) lattice gauge theory: the gauge field lives on links and \
         the action sums 1 − cos(θ) over plaquettes. Gauge invariance is a \
         structural theorem; locality is the unimproved 1×1 Wilson stencil \
         (a 2×1 rectangle is an IR mutation, not a knob). Confinement is a \
         knob-sensitive lattice result (all β in 2D/3D; a transition near \
         β ≈ 1.01 in 4D)."
    }
    fn world(&self) -> Option<World> {
        let space = self.dimension.saturating_sub(1);
        Some(World {
            spacetime: Manifold {
                dim: self.dimension,
                signature: physis_model::Signature { time: 1, space },
                compact_extra: 0,
                compact_radius_planck: 0.0,
                topology: physis_model::Topology::Minkowski,
                convention: physis_model::SignConvention::MostlyPlus,
            },
            gauge: GaugeGroup {
                factors: vec![SimpleGroup::U1],
            },
            spectrum: Spectrum::empty(),
            has_gravity: false,
            supersymmetric: false,
            free_parameter_count: 1,
            landscape_log10: 0.0,
            note: format!(
                "compact U(1) on a {}^{} lattice, β={}, {}",
                self.sites_per_side,
                self.dimension,
                self.beta,
                if self.is_confining() {
                    "confining"
                } else {
                    "Coulomb phase"
                }
            ),
        })
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                GAUGE_INVARIANT,
                "The action is invariant under local gauge transformations of the links.",
                LayerId::Interaction,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                GAUGE_LOCAL,
                "The action couples only neighbouring links (plaquettes).",
                LayerId::Interaction,
                ClaimClass::ModelInternal,
            )
            .with_domain(gauge_local_domain()),
            Claim::new(
                CONFINING,
                "Static charges are confined.",
                LayerId::Interaction,
                ClaimClass::Heuristic,
            ),
            Claim::new(
                ASYMPTOTIC_FREEDOM,
                "The coupling runs to zero at high energy.",
                LayerId::Interaction,
                ClaimClass::Phenomenological,
            ),
            Claim::new(
                STRONG_COUPLING_AREA_LAW,
                "The leading strong-coupling expansion yields an area law.",
                LayerId::Interaction,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                EXACT_AREA_LAW_2D,
                "In 2D the Wilson loop obeys an exact area law at all couplings.",
                LayerId::Interaction,
                ClaimClass::ModelInternal,
            )
            .with_domain(area_law_2d_domain()),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id_str() {
            STRONG_COUPLING_AREA_LAW => strong_coupling_verdict(self.beta, 1.0, claim),
            EXACT_AREA_LAW_2D => {
                if self.rectangle {
                    Verdict::inapplicable(
                        claim,
                        "exact plaquette factorization is the unimproved 1x1 Wilson action; a 2x1 rectangle term is a new encoding",
                    )
                } else if self.dimension == 2 {
                    let sigma = exact_2d_string_tension(self.beta);
                    let ratio = bessel_i1_over_i0(self.beta);
                    if sigma > 0.0 {
                        Verdict::holds(claim,
                            format!(
                                "exact 2D string tension σ = −ln(I₁/I₀) = {sigma:.4} > 0 at β = {}: confines at all couplings",
                                self.beta
                            ),
                        )
                        .with_evidence([
                            format!("⟨W⟩ = (I₁(β)/I₀(β))^Area = {ratio:.4}^Area (exact plaquette factorization)"),
                            "0 < I₁/I₀ < 1 for all finite β, so σ > 0 for every coupling".to_string(),
                        ])
                    } else {
                        // Only reachable in the β → ∞ continuum limit (σ → 0⁺).
                        Verdict::fails(claim,
                            format!("σ = {sigma:.4} ≤ 0 at β = {}", self.beta),
                        )
                    }
                } else {
                    Verdict::inapplicable(
                        claim,
                        "the exact plaquette factorization is special to 2D; in higher D see gauge.confining",
                    )
                }
            }
            ASYMPTOTIC_FREEDOM => Verdict::fails(claim,
                "abelian U(1) is not asymptotically free: the coupling grows with energy (Landau pole)",
            ),
            GAUGE_INVARIANT => Verdict::holds(claim,
                "plaquette action is invariant under U_μ(x) → g(x) U_μ(x) g(x+μ̂)†",
            ),
            GAUGE_LOCAL => {
                if self.rectangle {
                    Verdict::fails(
                        claim,
                        "2x1 rectangle term: the action couples next-nearest links",
                    )
                } else {
                    Verdict::holds(
                        claim,
                        "the action sums over 1x1 plaquettes: only neighbouring links couple",
                    )
                }
            }
            CONFINING => match self.dimension {
                2 | 3 => Verdict::holds(claim,
                    format!(
                        "compact U(1) confines at all β in {}D (Polyakov)",
                        self.dimension
                    ),
                ),
                _ => {
                    if self.beta < BETA_C_4D {
                        Verdict::holds(claim,
                            format!(
                                "4D strong coupling β={} < β_c≈{BETA_C_4D}: confining",
                                self.beta
                            ),
                        )
                    } else {
                        Verdict::fails(claim,
                            format!(
                                "4D weak coupling β={} ≥ β_c≈{BETA_C_4D}: Coulomb (deconfined) phase",
                                self.beta
                            ),
                        )
                        .with_evidence([
                            "compact U(1) in 4D has a phase transition; the continuum limit here is free Maxwell".to_string(),
                        ])
                    }
                }
            },
            _ => Verdict::inapplicable(claim, "claim not made by a lattice gauge object"),
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.rectangle = parsed.rectangle;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        if self.rectangle {
            return Vec::new();
        }
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(Self::rectangle_equation()),
        );
        match Self::from_package(&mutated) {
            Ok(parsed) if parsed.rectangle => {
                let mut fork = self.clone();
                fork.rectangle = true;
                vec![("add-rectangle".into(), Box::new(fork))]
            }
            _ => Vec::new(),
        }
    }
}

const SUN_SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "dimension",
        layer: LayerId::Spacetime,
        doc: "Lattice spacetime dimension (2–4).",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 2, max: 4 },
    },
    KnobSpec {
        name: "beta",
        layer: LayerId::Interaction,
        doc: "Inverse coupling β = 2N/g². Non-abelian gauge theories confine at all β in this encoding.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 0.0,
            max: 100.0,
        },
    },
    KnobSpec {
        name: "sites_per_side",
        layer: LayerId::Spacetime,
        doc: "Linear lattice size L.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 2, max: 256 },
    },
];

/// Non-abelian SU(N) Wilson lattice gauge theory (N = 2 or 3).
///
/// The unimproved 1×1 plaquette stencil lives on the IR package, same
/// dialect as [`WilsonU1`]. A 2×1 rectangle term is a package mutation
/// (`add-rectangle`), not a knob.
#[derive(Clone, Debug, PartialEq)]
pub struct WilsonSun {
    n: u8,
    dimension: u8,
    beta: f64,
    sites_per_side: u32,
    rectangle: bool,
}

impl WilsonSun {
    /// SU(2) Yang–Mills.
    pub fn su2() -> Self {
        Self {
            n: 2,
            dimension: 4,
            beta: 2.3,
            sites_per_side: 8,
            rectangle: false,
        }
    }

    /// SU(3) Yang–Mills (the gauge group of QCD).
    pub fn su3() -> Self {
        Self {
            n: 3,
            dimension: 4,
            beta: 6.0,
            sites_per_side: 8,
            rectangle: false,
        }
    }

    /// IR package for this Wilson stencil. Knobs stay on the struct.
    pub fn package(&self) -> TheoryPackage {
        wilson_package(self.id(), self.name(), self.rectangle)
    }

    /// Load a Wilson stencil from a package. Group rank comes from the
    /// package id; knobs default; overlay them from a live field when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        let mut base = match pkg.id.as_str() {
            "wilson-su2" => Self::su2(),
            "wilson-su3" => Self::su3(),
            other => {
                return Err(format!(
                    "wilson su(n) package id '{other}' is not wilson-su2 or wilson-su3"
                ))
            }
        };
        base.rectangle = parse_wilson_stencil(pkg)?;
        Ok(base)
    }

    fn rectangle_equation() -> String {
        RECTANGLE_EQ.to_string()
    }
}

impl Knobbed for WilsonSun {
    fn specs(&self) -> &'static [KnobSpec] {
        SUN_SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "dimension" => Ok(KnobValue::UInt(self.dimension as u64)),
            "beta" => Ok(KnobValue::Float(self.beta)),
            "sites_per_side" => Ok(KnobValue::UInt(self.sites_per_side as u64)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("dimension", KnobValue::UInt(v)) => self.dimension = v as u8,
            ("beta", KnobValue::Float(v)) => self.beta = v,
            ("sites_per_side", KnobValue::UInt(v)) => self.sites_per_side = v as u32,
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

impl Theory for WilsonSun {
    fn id(&self) -> &'static str {
        match self.n {
            2 => "wilson-su2",
            _ => "wilson-su3",
        }
    }
    fn name(&self) -> &'static str {
        match self.n {
            2 => "Wilson SU(2) lattice gauge",
            _ => "Wilson SU(3) lattice gauge",
        }
    }
    fn summary(&self) -> &'static str {
        "Non-abelian Wilson lattice gauge theory. Unlike compact U(1), SU(N) is \
         asymptotically free and is expected to confine at all couplings in 4D — \
         but 4D confinement / the Yang–Mills mass gap is unproven (a Millennium \
         Problem), so that verdict is honestly a conjecture. Locality is the \
         unimproved 1×1 Wilson stencil (a 2×1 rectangle is an IR mutation, not a knob)."
    }
    fn world(&self) -> Option<World> {
        let space = self.dimension.saturating_sub(1);
        Some(World {
            spacetime: Manifold {
                dim: self.dimension,
                signature: physis_model::Signature { time: 1, space },
                compact_extra: 0,
                compact_radius_planck: 0.0,
                topology: physis_model::Topology::Minkowski,
                convention: physis_model::SignConvention::MostlyPlus,
            },
            gauge: GaugeGroup {
                factors: vec![SimpleGroup::Su(self.n)],
            },
            spectrum: Spectrum::empty(),
            has_gravity: false,
            supersymmetric: false,
            free_parameter_count: 1,
            landscape_log10: 0.0,
            note: format!(
                "SU({}) on a {}^{} lattice, β={}",
                self.n, self.sites_per_side, self.dimension, self.beta
            ),
        })
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                GAUGE_INVARIANT,
                "The action is invariant under local gauge transformations of the links.",
                LayerId::Interaction,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                GAUGE_LOCAL,
                "The action couples only neighbouring links (plaquettes).",
                LayerId::Interaction,
                ClaimClass::ModelInternal,
            )
            .with_domain(gauge_local_domain()),
            Claim::new(
                CONFINING,
                "Static charges are confined.",
                LayerId::Interaction,
                ClaimClass::Conjecture,
            ),
            Claim::new(
                ASYMPTOTIC_FREEDOM,
                "The coupling runs to zero at high energy.",
                LayerId::Interaction,
                ClaimClass::Phenomenological,
            ),
            Claim::new(
                STRONG_COUPLING_AREA_LAW,
                "The leading strong-coupling expansion yields an area law.",
                LayerId::Interaction,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                EXACT_AREA_LAW_2D,
                "In 2D the Wilson loop obeys an exact area law at all couplings.",
                LayerId::Interaction,
                ClaimClass::ModelInternal,
            )
            .with_domain(area_law_2d_domain()),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id_str() {
            STRONG_COUPLING_AREA_LAW => strong_coupling_verdict(self.beta, self.n as f64, claim),
            EXACT_AREA_LAW_2D => {
                if self.rectangle {
                    Verdict::inapplicable(
                        claim,
                        "exact 2D factorization is the unimproved 1x1 Wilson action; a 2x1 rectangle term is a new encoding",
                    )
                } else if self.dimension == 2 {
                    let n = self.n as f64;
                    let sigma = exact_2d_string_tension_sun(n, self.beta);
                    if sigma > 0.0 {
                        Verdict::holds(claim,
                            format!(
                                "exact 2D string tension σ = (N²−1)/(2β) = {sigma:.4} > 0 at β = {}: SU({}) confines at all couplings",
                                self.beta, self.n
                            ),
                        )
                        .with_evidence([
                            format!(
                                "2D Yang–Mills is exactly solvable; σ = (g²/2)·C₂(fund), C₂(fund) = {:.4}",
                                su_casimir_fundamental(n)
                            ),
                            "unlike 4D, this needs no mass-gap conjecture — it is a theorem".to_string(),
                        ])
                    } else {
                        Verdict::fails(claim,
                            format!("σ = {sigma:.4} ≤ 0 at β = {}", self.beta),
                        )
                    }
                } else {
                    Verdict::inapplicable(
                        claim,
                        "the exact 2D solution is special to two dimensions; in 4D see gauge.confining (mass-gap conjecture)",
                    )
                }
            }
            GAUGE_INVARIANT => Verdict::holds(claim,
                "non-abelian plaquette action is gauge invariant by construction",
            ),
            GAUGE_LOCAL => {
                if self.rectangle {
                    Verdict::fails(
                        claim,
                        "2x1 rectangle term: the action couples next-nearest links",
                    )
                } else {
                    Verdict::holds(
                        claim,
                        "the action sums over 1x1 plaquettes: only neighbouring links couple",
                    )
                }
            }
            ASYMPTOTIC_FREEDOM => Verdict::holds(claim,
                "non-abelian SU(N) is asymptotically free (Gross–Wilczek–Politzer 1973)",
            ),
            CONFINING => match self.dimension {
                2 | 3 => Verdict::holds(claim,
                    format!("SU({}) confines in {}D", self.n, self.dimension),
                ),
                _ => Verdict::holds(claim,
                    format!(
                        "SU({}) is expected to confine in 4D at all β, but the mass gap is unproven",
                        self.n
                    ),
                )
                .with_evidence([
                    "4D Yang–Mills existence and mass gap is a Clay Millennium Problem".to_string(),
                ]),
            },
            _ => Verdict::inapplicable(claim, "claim not made by a lattice gauge object"),
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.rectangle = parsed.rectangle;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        if self.rectangle {
            return Vec::new();
        }
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(Self::rectangle_equation()),
        );
        match Self::from_package(&mutated) {
            Ok(parsed) if parsed.rectangle => {
                let mut fork = self.clone();
                fork.rectangle = true;
                vec![("add-rectangle".into(), Box::new(fork))]
            }
            _ => Vec::new(),
        }
    }
}

/// The lattice-gauge experiment: compact U(1) vs non-abelian SU(2)/SU(3).
pub fn gauge_lattice() -> ExperimentReport {
    let theories: Vec<Box<dyn Theory>> = vec![
        Box::new(WilsonU1::default()),
        Box::new(WilsonSun::su2()),
        Box::new(WilsonSun::su3()),
    ];
    report_from_rows(
        "gauge-lattice",
        "Lattice gauge lab",
        "How do abelian and non-abelian gauge fields on a lattice differ? Compact \
         U(1) (QED-like) vs SU(2)/SU(3) (Yang–Mills): which confine, which are \
         asymptotically free, and which claims are theorems vs conjectures?",
        "Gauge invariance is a theorem of any Wilson loop. Locality is the \
         unimproved 1×1 stencil (`add-rectangle` is an IR fork, not a knob). \
         U(1) is not asymptotically free and deconfines in 4D above β≈1.01; SU(N) \
         is asymptotically free and is *expected* to confine in 4D — but that is \
         the unproven Yang–Mills mass gap, so it is honestly a conjecture.",
        vec![
            "`holds` / `fails` are internal to the encoding; read `class` and `derivation`.".into(),
            "The gauge field lives on links; the action sums over plaquettes.".into(),
            "U(1): `set wilson-u1 beta 2` deconfines the 4D theory (Coulomb phase).".into(),
            "SU(N): 4D confinement holds as a *conjecture* — the Millennium mass-gap problem."
                .into(),
        ],
        &gauge_rows(),
        theories,
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
    fn gauge_invariance_and_locality_hold_as_executed_claims() {
        let w = WilsonU1::default();
        assert_eq!(verdict(&w, GAUGE_INVARIANT), VerdictKind::Holds);
        assert_eq!(verdict(&w, GAUGE_LOCAL), VerdictKind::Holds);
        let local = w
            .claims()
            .into_iter()
            .find(|c| c.id_str() == GAUGE_LOCAL)
            .unwrap();
        assert!(
            !local.domain().is_encoding_wide(),
            "gauge.local must name the 1x1 Wilson stencil: {:?}",
            local.domain()
        );
    }

    #[test]
    fn four_d_coupling_flips_confinement() {
        // The gauge knob → verdict diff: 4D compact U(1) deconfines at weak coupling.
        let mut w = WilsonU1::default(); // 4D, β=1.0
        assert_eq!(verdict(&w, CONFINING), VerdictKind::Holds);
        w.set("beta", KnobValue::Float(2.0)).unwrap();
        assert_eq!(verdict(&w, CONFINING), VerdictKind::Fails);
    }

    #[test]
    fn low_dimensions_always_confine() {
        let mut w = WilsonU1::default();
        w.set("dimension", KnobValue::UInt(3)).unwrap();
        w.set("beta", KnobValue::Float(50.0)).unwrap();
        assert_eq!(verdict(&w, CONFINING), VerdictKind::Holds);
    }

    #[test]
    fn qed_and_qcd_differ_on_asymptotic_freedom() {
        // U(1) is not asymptotically free; SU(N) is.
        assert_eq!(
            verdict(&WilsonU1::default(), ASYMPTOTIC_FREEDOM),
            VerdictKind::Fails
        );
        assert_eq!(
            verdict(&WilsonSun::su3(), ASYMPTOTIC_FREEDOM),
            VerdictKind::Holds
        );
    }

    #[test]
    fn four_d_su3_confinement_is_a_conjecture() {
        // SU(3) confines in 4D (holds) but only as a conjecture (mass gap unproven).
        let qcd = WilsonSun::su3();
        let c = qcd
            .claims()
            .into_iter()
            .find(|c| c.id_str() == CONFINING)
            .unwrap();
        let v = qcd.evaluate(&c);
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_eq!(v.class, ClaimClass::Conjecture);
        // Unlike U(1), it stays confining at weak coupling.
        let mut qcd_weak = WilsonSun::su3();
        qcd_weak.set("beta", KnobValue::Float(50.0)).unwrap();
        assert_eq!(verdict(&qcd_weak, CONFINING), VerdictKind::Holds);
    }

    #[test]
    fn strong_coupling_area_law_is_computed_and_knob_sensitive() {
        // U(1): σ = −ln(β/2) > 0 at β=1 (area law), fails once β > 2.
        let mut u1 = WilsonU1::default();
        assert_eq!(verdict(&u1, STRONG_COUPLING_AREA_LAW), VerdictKind::Holds);
        u1.set("beta", KnobValue::Float(3.0)).unwrap();
        assert_eq!(verdict(&u1, STRONG_COUPLING_AREA_LAW), VerdictKind::Fails);

        // SU(3): σ = −ln(β/18); holds at β=6, fails at very weak coupling.
        let mut qcd = WilsonSun::su3();
        assert_eq!(verdict(&qcd, STRONG_COUPLING_AREA_LAW), VerdictKind::Holds);
        qcd.set("beta", KnobValue::Float(100.0)).unwrap();
        assert_eq!(verdict(&qcd, STRONG_COUPLING_AREA_LAW), VerdictKind::Fails);
    }

    #[test]
    fn string_tension_matches_the_closed_form() {
        // −ln(β/2N²) for SU(3) at β=18 is exactly 0 (the strong-coupling radius).
        assert!(super::strong_coupling_string_tension(18.0, 3.0).abs() < 1e-12);
        assert!(super::strong_coupling_string_tension(2.0, 1.0).abs() < 1e-12);
    }

    #[test]
    fn bessel_ratio_matches_known_values() {
        // I₁(2)/I₀(2) = 1.5906.../2.2796... = 0.6977...
        assert!((super::bessel_i1_over_i0(2.0) - 0.697_774_65).abs() < 1e-6);
        // Small x: I₁/I₀ ≈ x/2.
        assert!((super::bessel_i1_over_i0(0.02) - 0.01).abs() < 1e-4);
        // Large x: I₁/I₀ → 1⁻.
        assert!(super::bessel_i1_over_i0(50.0) < 1.0);
        assert!(super::bessel_i1_over_i0(50.0) > 0.98);
    }

    #[test]
    fn two_d_u1_confines_at_all_couplings_exactly() {
        // The exact (not strong-coupling) 2D area law: σ = −ln(I₁/I₀) > 0 for
        // every β — 2D compact U(1) confines at all couplings.
        let mut w = WilsonU1::default();
        w.set("dimension", KnobValue::UInt(2)).unwrap();
        for beta in [0.1, 1.0, 2.0, 10.0, 50.0] {
            w.set("beta", KnobValue::Float(beta)).unwrap();
            assert_eq!(
                verdict(&w, EXACT_AREA_LAW_2D),
                VerdictKind::Holds,
                "β = {beta}"
            );
            assert!(super::exact_2d_string_tension(beta) > 0.0, "β = {beta}");
        }
        // The tension decreases with β (toward the continuum limit).
        assert!(super::exact_2d_string_tension(1.0) > super::exact_2d_string_tension(10.0));
    }

    #[test]
    fn two_d_sun_confines_at_all_couplings_exactly() {
        // 2D SU(N) Yang–Mills: σ = (N²−1)/(2β) > 0 for every β (exact theorem).
        for mk in [WilsonSun::su2, WilsonSun::su3] {
            let mut w = mk();
            w.set("dimension", KnobValue::UInt(2)).unwrap();
            for beta in [0.5, 2.0, 6.0, 50.0] {
                w.set("beta", KnobValue::Float(beta)).unwrap();
                assert_eq!(
                    verdict(&w, EXACT_AREA_LAW_2D),
                    VerdictKind::Holds,
                    "β={beta}"
                );
            }
        }
        // Casimir values: C₂(fund) = 3/4 for SU(2), 4/3 for SU(3).
        assert!((super::su_casimir_fundamental(2.0) - 0.75).abs() < 1e-12);
        assert!((super::su_casimir_fundamental(3.0) - 4.0 / 3.0).abs() < 1e-12);
        // σ for SU(3) at β=6 is (9−1)/(2·6) = 2/3.
        assert!((super::exact_2d_string_tension_sun(3.0, 6.0) - 2.0 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn sun_exact_area_law_is_2d_only() {
        // In 4D the exact solution does not apply — that is the mass-gap regime.
        assert_eq!(
            verdict(&WilsonSun::su3(), EXACT_AREA_LAW_2D),
            VerdictKind::Inapplicable
        );
    }

    #[test]
    fn exact_area_law_is_2d_only() {
        // The exact plaquette factorization is special to 2D; 4D is the open
        // problem, so the exact claim is inapplicable there.
        let w = WilsonU1::default(); // 4D
        assert_eq!(verdict(&w, EXACT_AREA_LAW_2D), VerdictKind::Inapplicable);
        let mut w3 = WilsonU1::default();
        w3.set("dimension", KnobValue::UInt(3)).unwrap();
        assert_eq!(verdict(&w3, EXACT_AREA_LAW_2D), VerdictKind::Inapplicable);
        let cell = w
            .claims()
            .into_iter()
            .find(|c| c.id_str() == EXACT_AREA_LAW_2D)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "exact area law must name 2D, not encoding-wide: {:?}",
            cell.domain()
        );
        assert!(
            cell.domain().regimes.iter().any(|r| r.contains("2D")),
            "2D regime: {:?}",
            cell.domain()
        );
    }

    #[test]
    fn gauge_experiment_builds_a_matrix() {
        let r = gauge_lattice();
        assert_eq!(r.id, "gauge-lattice");
        assert_eq!(r.theories.len(), 3);
        let af = r.matrix.get(ASYMPTOTIC_FREEDOM).expect("row");
        assert_eq!(af.get("wilson-u1").copied(), Some(VerdictKind::Fails));
        assert_eq!(af.get("wilson-su3").copied(), Some(VerdictKind::Holds));
    }

    #[test]
    fn rectangle_term_is_ir_not_a_knob() {
        let mut w = WilsonU1::default();
        assert!(
            w.set("rectangle", KnobValue::Bool(true)).is_err(),
            "2x1 rectangle is an IR mutation, not a knob"
        );
        let src = render_package(&w.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            WilsonU1::from_package(&pkg).unwrap(),
            w,
            "IR round-trip must preserve the 1x1 Wilson stencil"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(WilsonU1::rectangle_equation()),
        );
        let parsed = WilsonU1::from_package(&mutated).unwrap();
        assert!(parsed.rectangle);
        let mut fork = w.clone();
        fork.rectangle = true;
        assert_eq!(verdict(&fork, GAUGE_LOCAL), VerdictKind::Fails);
        assert_eq!(verdict(&w, GAUGE_LOCAL), VerdictKind::Holds);
        assert_eq!(verdict(&fork, GAUGE_INVARIANT), VerdictKind::Holds);
        let mut two_d = w.clone();
        two_d.set("dimension", KnobValue::UInt(2)).unwrap();
        assert_eq!(verdict(&two_d, EXACT_AREA_LAW_2D), VerdictKind::Holds);
        two_d.rectangle = true;
        assert_eq!(
            verdict(&two_d, EXACT_AREA_LAW_2D),
            VerdictKind::Inapplicable
        );
        let probes = w.structural_mutations();
        assert_eq!(probes.len(), 1);
        assert_eq!(probes[0].0, "add-rectangle");
        assert_eq!(
            verdict(probes[0].1.as_ref(), GAUGE_LOCAL),
            VerdictKind::Fails
        );
        assert_eq!(verdict(&w, GAUGE_LOCAL), VerdictKind::Holds);
        assert!(fork.structural_mutations().is_empty());
        let canonical = physis_ir::certify_round_trip(&w.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = w.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), w.package());
        assert_eq!(verdict(rebuilt.as_ref(), GAUGE_LOCAL), VerdictKind::Holds);
    }

    #[test]
    fn sun_rectangle_term_is_ir_not_a_knob() {
        assert_ne!(
            WilsonSun::su2().package(),
            WilsonSun::su3().package(),
            "SU(2) and SU(3) packages must differ by id/name"
        );
        for mk in [WilsonSun::su2, WilsonSun::su3] {
            let mut w = mk();
            assert!(
                w.set("rectangle", KnobValue::Bool(true)).is_err(),
                "2x1 rectangle is an IR mutation, not a knob"
            );
            let src = render_package(&w.package());
            let pkg = parse_package(&src).unwrap();
            assert_eq!(
                WilsonSun::from_package(&pkg).unwrap(),
                w,
                "IR round-trip must preserve the 1x1 Wilson stencil"
            );
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(WilsonSun::rectangle_equation()),
            );
            let parsed = WilsonSun::from_package(&mutated).unwrap();
            assert!(parsed.rectangle);
            let mut fork = w.clone();
            fork.rectangle = true;
            assert_eq!(verdict(&fork, GAUGE_LOCAL), VerdictKind::Fails);
            assert_eq!(verdict(&w, GAUGE_LOCAL), VerdictKind::Holds);
            assert_eq!(verdict(&fork, GAUGE_INVARIANT), VerdictKind::Holds);
            let mut two_d = w.clone();
            two_d.set("dimension", KnobValue::UInt(2)).unwrap();
            assert_eq!(verdict(&two_d, EXACT_AREA_LAW_2D), VerdictKind::Holds);
            two_d.rectangle = true;
            assert_eq!(
                verdict(&two_d, EXACT_AREA_LAW_2D),
                VerdictKind::Inapplicable
            );
            let probes = w.structural_mutations();
            assert_eq!(probes.len(), 1);
            assert_eq!(probes[0].0, "add-rectangle");
            assert_eq!(
                verdict(probes[0].1.as_ref(), GAUGE_LOCAL),
                VerdictKind::Fails
            );
            assert_eq!(verdict(&w, GAUGE_LOCAL), VerdictKind::Holds);
            assert!(fork.structural_mutations().is_empty());
            let canonical = physis_ir::certify_round_trip(&w.ir_package().unwrap()).unwrap();
            let parsed = parse_package(&canonical).unwrap();
            let rebuilt = w.reparse_package(&parsed).unwrap();
            assert_eq!(rebuilt.ir_package().unwrap(), w.package());
            assert_eq!(verdict(rebuilt.as_ref(), GAUGE_LOCAL), VerdictKind::Holds);
        }
        assert!(
            WilsonSun::from_package(&WilsonU1::default().package()).is_err(),
            "SU(N) must not load a U(1) package id"
        );
    }
}
