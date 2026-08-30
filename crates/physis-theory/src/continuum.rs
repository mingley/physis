//! Continuum (M4): a field as an actual local object, not a flag.
//!
//! [`KleinGordonField`] is a real scalar field on a finite 1D periodic lattice.
//! Its degrees of freedom are the N lattice sites, coupled by a nearest-
//! neighbour discrete Laplacian, so its normal modes are *computed*:
//!
//! ```text
//! ω_j² = m² + (4/a²) · sin²(π j / N),   j = 0 … N-1
//! ```
//!
//! The claims are theorems of that computation rather than tabulated facts: the
//! long-wavelength dispersion matches the continuum `ω² = m² + k²`, and a
//! negative `mass_squared` produces a genuine tachyonic mode (`min ω² < 0`) —
//! the same instability notion as the string bosonic tachyon, but computed.
//!
//! Nearest-neighbour coupling lives on the IR package. Next-nearest stencil
//! is a package mutation (`add-next-nearest`), not a knob.
//!
//! [`DiracFermion`] is a 1D naive lattice Dirac operator on the same kind of
//! periodic chain. Its poles are *computed*: `sin(ka) = 0` at `k = 0` and
//! `k = π/a` when `N` is even, so `fermion.no-doublers` fails. A Wilson `r`
//! term is a package mutation (`add-wilson`), not a mass knob: the edge
//! copy gets mass `m + 2r/a` and the cell holds. That fork is still this
//! object, not a silent Klein–Gordon install. `sites` / `mass` / `spacing`
//! stay knobs.

use std::f64::consts::PI;

use physis_core::assumption::DomainOfValidity;
use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::EmpiricalStatus;
use physis_core::ParameterOrigin;
use physis_ir::{apply_mutation, parse_package, render_package, PackageMutation, TheoryPackage};
use physis_model::{GaugeGroup, Manifold, Spectrum, World};

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;

/// The field has a finite number of normal modes.
pub const FINITE_MODES: &str = "field.finite-modes";
/// The lattice dispersion matches the continuum ω² = m² + k² at long wavelength.
pub const DISPERSION: &str = "field.dispersion-continuum-limit";
/// No tachyonic mode: min ω² ≥ 0.
pub const STABLE: &str = "field.stable";
/// The group velocity is bounded by c.
pub const CAUSAL: &str = "field.causal";
/// The coupling is local (nearest-neighbour).
pub const LOCAL: &str = "field.local";
/// The discretization is second-order accurate (error ∝ a²).
pub const SECOND_ORDER: &str = "field.second-order-accurate";
/// The Brillouin zone has one light fermion, not a doubler at k = π/a.
pub const NO_DOUBLERS: &str = "fermion.no-doublers";

/// Long-wavelength domain for the O(a²) identity: `|k a| < 1` at the
/// convergence probe. Outside it, Richardson's `p` is not a verdict on the
/// stencil — the lattice does not resolve the probe.
const ASYMPTOTIC_KA: f64 = 1.0;

/// Matrix rows for the field lab.
pub fn field_rows() -> [&'static str; 7] {
    [
        FINITE_MODES,
        DISPERSION,
        STABLE,
        CAUSAL,
        LOCAL,
        SECOND_ORDER,
        NO_DOUBLERS,
    ]
}

/// Fixed physical wavenumber used to probe the discretization's accuracy order.
const CONVERGENCE_PROBE_K: f64 = 0.1;

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "sites",
        layer: LayerId::Field,
        doc: "Number of lattice sites N (the field's local degrees of freedom).",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 2, max: 4096 },
    },
    KnobSpec {
        name: "mass_squared",
        layer: LayerId::Field,
        doc: "Mass-squared m² in natural units. Negative values make the zero mode tachyonic.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: -100.0,
            max: 100.0,
        },
    },
    KnobSpec {
        name: "spacing",
        layer: LayerId::Field,
        doc: "Lattice spacing a in natural units.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 1.0e-3,
            max: 1.0e3,
        },
    },
];

/// IR equation for the nearest-neighbour discrete Laplacian.
const NN_EQUATION: &str = "laplacian nn";
/// IR equation for an extra next-nearest stencil term.
const NNN_EQUATION: &str = "laplacian nnn";

/// A real scalar (Klein–Gordon) field on a finite 1D periodic lattice.
///
/// The stencil is package structure, not a knob. Next-nearest coupling is
/// set only by an IR fork.
#[derive(Clone, Debug, PartialEq)]
pub struct KleinGordonField {
    sites: u32,
    mass_squared: f64,
    spacing: f64,
    /// Extra next-nearest Laplacian term. Not a knob.
    next_nearest: bool,
}

impl Default for KleinGordonField {
    fn default() -> Self {
        Self {
            sites: 16,
            mass_squared: 1.0,
            spacing: 1.0,
            next_nearest: false,
        }
    }
}

fn local_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["nearest-neighbour 1D periodic lattice".into()],
        vec!["discrete Laplacian on N sites".into()],
        "Locality here is nearest-neighbour. A next-nearest stencil is a new \
         encoding, not a silent local field.",
    )
}

impl KleinGordonField {
    /// ω² for normal mode `j` (0-based) from the discrete Laplacian.
    fn omega_sq(&self, j: u32) -> f64 {
        let n = self.sites as f64;
        let a = self.spacing;
        let s = (PI * j as f64 / n).sin();
        let nn = (4.0 / (a * a)) * s * s;
        let nnn = if self.next_nearest {
            let ka = self.k(j) * a;
            (1.0 / (a * a)) * ka.sin() * ka.sin()
        } else {
            0.0
        };
        self.mass_squared + nn + nnn
    }

    /// Wavenumber of mode `j`.
    fn k(&self, j: u32) -> f64 {
        2.0 * PI * j as f64 / (self.sites as f64 * self.spacing)
    }

    /// Minimum ω² over all modes (the stability-controlling value).
    fn min_omega_sq(&self) -> f64 {
        (0..self.sites)
            .map(|j| self.omega_sq(j))
            .fold(f64::INFINITY, f64::min)
    }

    /// Largest group velocity dω/dk over the modes.
    ///
    /// A tachyonic mode (ω² < 0) has an imaginary frequency and no sensible,
    /// causal group velocity, so the field is treated as non-causal. The
    /// massless zero mode (ω² = 0) is fine: its group velocity is 0.
    fn max_group_velocity(&self) -> f64 {
        let a = self.spacing;
        let mut max = 0.0_f64;
        for j in 0..self.sites {
            let w2 = self.omega_sq(j);
            if w2 < 0.0 {
                return f64::INFINITY;
            }
            // d(ω²)/dk = (2/a) sin(ka); v_g = that / (2ω). The zero mode has
            // sin(0) = 0, so v_g = 0 even as ω → 0.
            let ka = self.k(j) * a;
            let vg = if w2 <= 0.0 {
                0.0
            } else {
                // NN: d(ω²)/dk = (2/a) sin(ka). NNN adds (2/a) sin(ka) cos(ka).
                let mut d_w2_dk = (2.0 / a) * ka.sin();
                if self.next_nearest {
                    d_w2_dk += (2.0 / a) * ka.sin() * ka.cos();
                }
                d_w2_dk.abs() / (2.0 * w2.sqrt())
            };
            max = max.max(vg);
        }
        max
    }

    /// Absolute dispersion error of the discrete Laplacian at a fixed physical
    /// wavenumber `k`, for lattice spacing `a`. Nearest-neighbour is
    /// `(4/a²) sin²(ka/2)`; a next-nearest term adds `(1/a²) sin²(ka)`.
    fn dispersion_abs_error(&self, k: f64, a: f64) -> f64 {
        let s = (k * a / 2.0).sin();
        let nn = (4.0 / (a * a)) * s * s;
        let nnn = if self.next_nearest {
            let ka = k * a;
            (1.0 / (a * a)) * ka.sin() * ka.sin()
        } else {
            0.0
        };
        (nn + nnn - k * k).abs()
    }

    /// Empirical convergence order p, from the error at spacing `a` vs `a/2`:
    /// `p = log2(err(a) / err(a/2))`. A second-order scheme gives p ≈ 2.
    fn convergence_order(&self) -> f64 {
        let a = self.spacing;
        let e1 = self.dispersion_abs_error(CONVERGENCE_PROBE_K, a);
        let e2 = self.dispersion_abs_error(CONVERGENCE_PROBE_K, a / 2.0);
        if e2 <= 0.0 || e1 <= 0.0 {
            // Exact at this probe: treat as (at least) second order.
            return 2.0;
        }
        (e1 / e2).log2()
    }

    /// Probe wavenumber times spacing. The O(a²) expansion of
    /// `(4/a²) sin²(ka/2)` is a long-wavelength statement.
    fn probe_ka(&self) -> f64 {
        CONVERGENCE_PROBE_K * self.spacing
    }

    /// True when the convergence probe is outside the long-wavelength domain.
    fn too_coarse_for_order(&self) -> bool {
        self.probe_ka().abs() >= ASYMPTOTIC_KA
    }

    /// Relative error between the longest-wavelength mode and the continuum
    /// `ω² = m² + k²`.
    fn long_wavelength_rel_error(&self) -> f64 {
        let j = 1; // longest non-zero wavelength
        let lattice = self.omega_sq(j);
        let k = self.k(j);
        let continuum = self.mass_squared + k * k;
        if continuum.abs() < 1e-12 {
            (lattice - continuum).abs()
        } else {
            (lattice - continuum).abs() / continuum.abs()
        }
    }

    /// IR package for this stencil. Equations are `laplacian nn` and,
    /// when forked, `laplacian nnn`. Knobs stay on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![NN_EQUATION.to_string()];
        if self.next_nearest {
            equations.push(NNN_EQUATION.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["1d-periodic-lattice".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: LOCAL.into(),
                statement: "The coupling is local (nearest-neighbour).".into(),
                layer: "field".into(),
                class: "model-internal".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load a stencil from a package. Knobs default; overlay them from a
    /// live field when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        let mut nn = false;
        let mut nnn = false;
        for eq in &pkg.equations {
            match eq.trim() {
                NN_EQUATION => nn = true,
                NNN_EQUATION => nnn = true,
                _ => {}
            }
        }
        if !nn {
            return Err("klein-gordon package has no nearest-neighbour laplacian".into());
        }
        Ok(Self {
            next_nearest: nnn,
            ..Self::default()
        })
    }

    fn nnn_equation() -> String {
        NNN_EQUATION.to_string()
    }
}

impl Knobbed for KleinGordonField {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "sites" => Ok(KnobValue::UInt(self.sites as u64)),
            "mass_squared" => Ok(KnobValue::Float(self.mass_squared)),
            "spacing" => Ok(KnobValue::Float(self.spacing)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("sites", KnobValue::UInt(v)) => self.sites = v as u32,
            ("mass_squared", KnobValue::Float(v)) => self.mass_squared = v,
            ("spacing", KnobValue::Float(v)) => self.spacing = v,
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

impl Theory for KleinGordonField {
    fn id(&self) -> &'static str {
        "klein-gordon"
    }
    fn name(&self) -> &'static str {
        "Klein–Gordon field (1D lattice)"
    }
    fn summary(&self) -> &'static str {
        "A real scalar field as an actual local object: N lattice sites coupled \
         by a nearest-neighbour Laplacian, with computed normal modes. \
         Next-nearest coupling is an IR package mutation, not a knob. Stability \
         and dispersion are theorems of the computation, not flags."
    }
    fn world(&self) -> Option<World> {
        // A 1+1 D field: one time direction, one spatial lattice direction.
        Some(World {
            spacetime: Manifold {
                dim: 2,
                signature: physis_model::Signature { time: 1, space: 1 },
                compact_extra: 0,
                compact_radius_planck: 0.0,
                topology: physis_model::Topology::Minkowski,
                convention: physis_model::SignConvention::MostlyPlus,
            },
            gauge: GaugeGroup::trivial(),
            spectrum: Spectrum::empty(),
            has_gravity: false,
            supersymmetric: false,
            free_parameter_count: 3,
            landscape_log10: 0.0,
            note: format!(
                "Klein–Gordon scalar on {} sites, m²={}, a={}, min ω²={:.4}",
                self.sites,
                self.mass_squared,
                self.spacing,
                self.min_omega_sq()
            ),
        })
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                FINITE_MODES,
                "The field has a finite number of normal modes.",
                LayerId::Field,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                DISPERSION,
                "The long-wavelength dispersion matches the continuum ω² = m² + k².",
                LayerId::Field,
                ClaimClass::ModelInternal,
            )
            .with_domain(DomainOfValidity::new(
                vec!["longest non-zero lattice mode".into()],
                vec!["discrete Laplacian vs continuum ω² = m² + k²".into()],
                "This is the long-wavelength mode, not the Nyquist mode and not \
                 the Richardson |k a| < 1 probe. Using the continuum dispersion \
                 at short wavelength is a new claim.",
            )),
            Claim::new(
                STABLE,
                "There is no tachyonic mode (min ω² ≥ 0).",
                LayerId::Field,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                CAUSAL,
                "The group velocity is bounded by c.",
                LayerId::Field,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                LOCAL,
                "The coupling is local (nearest-neighbour).",
                LayerId::Field,
                ClaimClass::ModelInternal,
            )
            .with_domain(local_domain()),
            Claim::new(
                SECOND_ORDER,
                "The discretization is second-order accurate (error ∝ a²).",
                LayerId::Field,
                ClaimClass::ModelInternal,
            )
            .with_domain(DomainOfValidity::new(
                vec!["|k a| < 1 at the Richardson probe".into()],
                vec!["O(a^2) stencil at long wavelength".into()],
                "Outside |k a| < 1 the Richardson order is not a stencil verdict. \
                 That spacing is undecidable / insufficient-precision, not a failed theorem.",
            )),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id_str() {
            FINITE_MODES => {
                Verdict::holds(claim, format!("{} normal modes on the lattice", self.sites))
            }
            DISPERSION => {
                let err = self.long_wavelength_rel_error();
                if err < 0.05 {
                    Verdict::holds(claim, "long-wavelength mode matches continuum ω² = m² + k²")
                        .with_evidence([format!(
                            "relative error {:.2}% at the longest wavelength",
                            err * 100.0
                        )])
                } else {
                    Verdict::fails(
                        claim,
                        format!(
                            "lattice too coarse: {:.1}% error vs the continuum dispersion",
                            err * 100.0
                        ),
                    )
                }
            }
            STABLE => {
                let m = self.min_omega_sq();
                if m >= -1e-12 {
                    Verdict::holds(claim, format!("min ω² = {m:.4} ≥ 0"))
                } else {
                    Verdict::fails(
                        claim,
                        format!("tachyonic mode: min ω² = {m:.4} < 0 (unstable)"),
                    )
                    .with_evidence([
                        "same instability notion as the bosonic-string tachyon, here computed"
                            .to_string(),
                    ])
                }
            }
            CAUSAL => {
                let v = self.max_group_velocity();
                if v <= 1.0 + 1e-9 {
                    Verdict::holds(claim, format!("max group velocity {v:.4} ≤ c"))
                } else {
                    Verdict::fails(claim, format!("superluminal group velocity {v:.4} > c"))
                }
            }
            LOCAL => {
                if self.next_nearest {
                    Verdict::fails(
                        claim,
                        "next-nearest stencil: the coupling is not nearest-neighbour",
                    )
                } else {
                    Verdict::holds(claim, "nearest-neighbour Laplacian: the coupling is local")
                }
            }
            SECOND_ORDER => {
                let p = self.convergence_order();
                let ka = self.probe_ka();
                if self.too_coarse_for_order() {
                    Verdict::undecidable(
                        claim,
                        format!(
                            "probe |k a| = {ka:.3} ≥ {ASYMPTOTIC_KA}: too coarse to certify O(a²)"
                        ),
                    )
                    .with_empirical(EmpiricalStatus::Inconclusive)
                    .with_evidence([
                        format!("Richardson p = {p:.3} is not a stencil verdict outside |k a| < 1"),
                        "the discrete Laplacian is still O(a²) at long wavelength; this spacing does not resolve the probe"
                            .to_string(),
                    ])
                } else if (1.8..=2.2).contains(&p) {
                    Verdict::holds(
                        claim,
                        format!("measured convergence order p = {p:.3} ≈ 2 (error ∝ a²)"),
                    )
                    .with_evidence([
                        "computed by halving the lattice spacing at a fixed physical wavenumber"
                            .to_string(),
                        format!("probe |k a| = {ka:.3} is inside the long-wavelength domain"),
                    ])
                } else {
                    Verdict::fails(
                        claim,
                        format!("measured order p = {p:.3} ≠ 2 inside the long-wavelength domain"),
                    )
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by a field object"),
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.next_nearest = parsed.next_nearest;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        if self.next_nearest {
            return Vec::new();
        }
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mutated = apply_mutation(&pkg, &PackageMutation::AppendEquation(Self::nnn_equation()));
        match Self::from_package(&mutated) {
            Ok(parsed) if parsed.next_nearest => {
                let mut fork = self.clone();
                fork.next_nearest = true;
                vec![("add-next-nearest".into(), Box::new(fork))]
            }
            _ => Vec::new(),
        }
    }
}

/// Naive 1D lattice Dirac: `E² = (sin(ka)/a)² + m²`.
const DIRAC_NAIVE_EQ: &str = "dirac naive";
/// Wilson r term: `M(k) = m + (2r/a) sin²(ka/2)`.
const DIRAC_WILSON_EQ: &str = "dirac wilson";
/// Wilson r (natural units). Not a knob; the term is the IR fork.
const WILSON_R: f64 = 1.0;

const DIRAC_SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "sites",
        layer: LayerId::Field,
        doc: "Number of lattice sites N (the fermion's local degrees of freedom).",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 2, max: 4096 },
    },
    KnobSpec {
        name: "mass",
        layer: LayerId::Field,
        doc: "Dirac mass m in natural units. Doubling is not this knob: add-wilson is an IR mutation.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: -100.0,
            max: 100.0,
        },
    },
    KnobSpec {
        name: "spacing",
        layer: LayerId::Field,
        doc: "Lattice spacing a in natural units.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 1.0e-3,
            max: 1.0e3,
        },
    },
];

fn parse_dirac_operator(pkg: &TheoryPackage) -> Result<bool, String> {
    let mut naive = false;
    let mut wilson = false;
    for eq in &pkg.equations {
        match eq.trim() {
            DIRAC_NAIVE_EQ => naive = true,
            DIRAC_WILSON_EQ => wilson = true,
            _ => {}
        }
    }
    if !naive {
        return Err(format!("{} package has no naive Dirac operator", pkg.id));
    }
    Ok(wilson)
}

fn no_doublers_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["naive 1D lattice Dirac".into()],
        vec!["Brillouin-zone poles of sin(ka) at k = 0 and k = π/a".into()],
        "The no-doublers cell is the naive Dirac encoding. A Wilson r term \
         is a new encoding, not a silent mass knob.",
    )
}

/// A Dirac fermion on a finite 1D periodic lattice.
///
/// The operator lives on the IR package. A Wilson r term is a package
/// mutation (`add-wilson`), not a knob: naive `sin(ka) = 0` has a light
/// copy at the Brillouin edge and `fermion.no-doublers` fails; Wilson
/// lifts that copy to mass `m + 2r/a` and the cell holds. That fork is
/// still this object, not a silent Klein–Gordon install. `sites` /
/// `mass` / `spacing` stay knobs.
#[derive(Clone, Debug, PartialEq)]
pub struct DiracFermion {
    sites: u32,
    mass: f64,
    spacing: f64,
    /// Wilson r term. Not a knob.
    wilson: bool,
}

impl Default for DiracFermion {
    fn default() -> Self {
        Self {
            sites: 16,
            mass: 1.0,
            spacing: 1.0,
            wilson: false,
        }
    }
}

impl DiracFermion {
    fn k(&self, j: u32) -> f64 {
        2.0 * PI * j as f64 / (self.sites as f64 * self.spacing)
    }

    /// Momentum-dependent mass: `m` on the naive encoding, `m + (2r/a) sin²(ka/2)`
    /// with the Wilson term.
    fn dirac_mass(&self, j: u32) -> f64 {
        let m = self.mass;
        if !self.wilson {
            return m;
        }
        let ka = self.k(j) * self.spacing;
        m + (2.0 * WILSON_R / self.spacing) * (ka / 2.0).sin().powi(2)
    }

    /// Energy of lattice momentum `j`: `E = √( (sin(ka)/a)² + M(k)² )`.
    fn energy(&self, j: u32) -> f64 {
        let a = self.spacing;
        let ka = self.k(j) * a;
        let kin = ka.sin() / a;
        let m = self.dirac_mass(j);
        (kin * kin + m * m).sqrt()
    }

    /// Lattice momenta where the naive kinetic piece vanishes (`sin(ka) = 0`).
    fn kinetic_zeros(&self) -> Vec<u32> {
        (0..self.sites)
            .filter(|&j| {
                let ka = self.k(j) * self.spacing;
                ka.sin().abs() < 1e-9
            })
            .collect()
    }

    /// Kinetic zeros whose mass is the physical `m`, not the Wilson edge copy.
    fn light_copies(&self) -> usize {
        let m = self.mass;
        let tol = 1e-9 * (1.0 + m.abs());
        self.kinetic_zeros()
            .into_iter()
            .filter(|&j| (self.dirac_mass(j) - m).abs() <= tol)
            .count()
    }

    /// Relative error of `E²` at the longest non-zero mode vs continuum `m² + k²`.
    fn long_wavelength_rel_error(&self) -> f64 {
        let j = 1;
        let lattice = self.energy(j).powi(2);
        let k = self.k(j);
        let continuum = self.mass * self.mass + k * k;
        if continuum.abs() < 1e-12 {
            (lattice - continuum).abs()
        } else {
            (lattice - continuum).abs() / continuum.abs()
        }
    }

    /// IR package for this operator. Equations are `dirac naive` and, when
    /// forked, `dirac wilson`. Knobs stay on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![DIRAC_NAIVE_EQ.to_string()];
        if self.wilson {
            equations.push(DIRAC_WILSON_EQ.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["1d-periodic-lattice-dirac".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: NO_DOUBLERS.into(),
                statement: "The Brillouin zone has one light fermion, not a doubler at k = π/a."
                    .into(),
                layer: "field".into(),
                class: "model-internal".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load an operator encoding from a package. Knobs default; overlay
    /// them from a live fermion when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        if pkg.id != "dirac-fermion" {
            return Err(format!(
                "dirac-fermion package id '{}' is not dirac-fermion",
                pkg.id
            ));
        }
        Ok(Self {
            wilson: parse_dirac_operator(pkg)?,
            ..Self::default()
        })
    }

    fn wilson_equation() -> String {
        DIRAC_WILSON_EQ.to_string()
    }
}

impl Knobbed for DiracFermion {
    fn specs(&self) -> &'static [KnobSpec] {
        DIRAC_SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "sites" => Ok(KnobValue::UInt(self.sites as u64)),
            "mass" => Ok(KnobValue::Float(self.mass)),
            "spacing" => Ok(KnobValue::Float(self.spacing)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("sites", KnobValue::UInt(v)) => self.sites = v as u32,
            ("mass", KnobValue::Float(v)) => self.mass = v,
            ("spacing", KnobValue::Float(v)) => self.spacing = v,
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

impl Theory for DiracFermion {
    fn id(&self) -> &'static str {
        "dirac-fermion"
    }
    fn name(&self) -> &'static str {
        "Dirac fermion (1D lattice)"
    }
    fn summary(&self) -> &'static str {
        "A 1D lattice Dirac fermion as an actual local object: N sites with a \
         computed naive operator. Doubling at k = π/a is a package fact; a \
         Wilson r term is an IR mutation, not a mass knob, and not a silent \
         Klein–Gordon install."
    }
    fn world(&self) -> Option<World> {
        Some(World {
            spacetime: Manifold {
                dim: 2,
                signature: physis_model::Signature { time: 1, space: 1 },
                compact_extra: 0,
                compact_radius_planck: 0.0,
                topology: physis_model::Topology::Minkowski,
                convention: physis_model::SignConvention::MostlyPlus,
            },
            gauge: GaugeGroup::trivial(),
            spectrum: Spectrum::empty(),
            has_gravity: false,
            supersymmetric: false,
            free_parameter_count: 3,
            landscape_log10: 0.0,
            note: format!(
                "Dirac fermion on {} sites, m={}, a={}, light copies={}",
                self.sites,
                self.mass,
                self.spacing,
                self.light_copies()
            ),
        })
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                FINITE_MODES,
                "The field has a finite number of normal modes.",
                LayerId::Field,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                DISPERSION,
                "The long-wavelength dispersion matches the continuum E² = m² + k².",
                LayerId::Field,
                ClaimClass::ModelInternal,
            )
            .with_domain(DomainOfValidity::new(
                vec!["longest non-zero lattice mode".into()],
                vec!["naive or Wilson Dirac vs continuum E² = m² + k²".into()],
                "This is the long-wavelength mode, not the Nyquist mode. Using \
                 the continuum Dirac dispersion at the Brillouin edge is a new claim.",
            )),
            Claim::new(
                LOCAL,
                "The coupling is local (nearest-neighbour).",
                LayerId::Field,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                NO_DOUBLERS,
                "The Brillouin zone has one light fermion, not a doubler at k = π/a.",
                LayerId::Field,
                ClaimClass::ModelInternal,
            )
            .with_domain(no_doublers_domain()),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id_str() {
            FINITE_MODES => Verdict::holds(
                claim,
                format!("{} Dirac momenta on the lattice", self.sites),
            ),
            DISPERSION => {
                let err = self.long_wavelength_rel_error();
                if err < 0.05 {
                    Verdict::holds(claim, "long-wavelength mode matches continuum E² = m² + k²")
                        .with_evidence([format!(
                            "relative error {:.2}% at the longest wavelength",
                            err * 100.0
                        )])
                } else {
                    Verdict::fails(
                        claim,
                        format!(
                            "lattice too coarse or Wilson-shifted: {:.1}% error vs continuum E²",
                            err * 100.0
                        ),
                    )
                }
            }
            LOCAL => Verdict::holds(
                claim,
                "naive hopping and the Wilson r term are both nearest-neighbour",
            ),
            NO_DOUBLERS => {
                let zeros = self.kinetic_zeros().len();
                if zeros < 2 {
                    Verdict::inapplicable(
                        claim,
                        "doubling is a statement about the Brillouin edge k = π/a, which is not a lattice mode on this N",
                    )
                } else {
                    let n = self.light_copies();
                    if n == 1 {
                        Verdict::holds(
                            claim,
                            "Wilson r lifts the k = π/a copy; one light fermion remains",
                        )
                        .with_evidence([format!(
                            "light copies = {n}; edge mass M(π/a) = {:.4}",
                            self.dirac_mass(self.sites / 2)
                        )])
                    } else {
                        Verdict::fails(
                            claim,
                            format!("naive Dirac has {n} light copies (k = 0 and k = π/a)"),
                        )
                        .with_evidence([format!(
                            "E(0) = {:.4}, E(π/a) = {:.4}",
                            self.energy(0),
                            self.energy(self.sites / 2)
                        )])
                    }
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by a Dirac fermion"),
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.wilson = parsed.wilson;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        if self.wilson {
            return Vec::new();
        }
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(Self::wilson_equation()),
        );
        match Self::from_package(&mutated) {
            Ok(parsed) if parsed.wilson => {
                let mut fork = self.clone();
                fork.wilson = true;
                vec![("add-wilson".into(), Box::new(fork))]
            }
            _ => Vec::new(),
        }
    }
}

/// The field-modes experiment: a scalar field's computed spectrum and stability.
pub fn field_modes() -> ExperimentReport {
    let theories: Vec<Box<dyn Theory>> = vec![
        Box::new(KleinGordonField::default()),
        Box::new(DiracFermion::default()),
    ];
    report_from_rows(
        "field-modes",
        "Field modes lab",
        "Can a field be an actual local object — N lattice sites with a computed \
         spectrum — so that stability and the continuum dispersion are theorems \
         of the computation, a negative mass² produces a real tachyon, and naive \
         Dirac doubling is a computed Brillouin-zone fact?",
        "The normal modes are computed from the discrete Laplacian or Dirac \
         operator, not tabulated. `field.stable` reads the sign of the minimum \
         ω²; a negative mass_squared knob makes it fail, the same instability as \
         the bosonic string's tachyon. `fermion.no-doublers` reads the light \
         copies of sin(ka)=0; add-wilson is IR, not a mass knob.",
        vec![
            "`holds` / `fails` are internal to the encoding.".into(),
            "Modes and dispersion are computed: ω_j² = m² + (4/a²) sin²(π j / N).".into(),
            "`set klein-gordon mass_squared -1` makes the zero mode tachyonic and `field.stable` fails.".into(),
            "`hypothesize dirac-fermion`: add-wilson is IR, not set.".into(),
        ],
        &field_rows(),
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
    fn default_field_is_stable_causal_and_dispersive() {
        let f = KleinGordonField::default();
        assert_eq!(verdict(&f, STABLE), VerdictKind::Holds);
        assert_eq!(verdict(&f, CAUSAL), VerdictKind::Holds);
        assert_eq!(verdict(&f, DISPERSION), VerdictKind::Holds);
        assert_eq!(verdict(&f, LOCAL), VerdictKind::Holds);
        let disp = f
            .claims()
            .into_iter()
            .find(|c| c.id_str() == DISPERSION)
            .unwrap();
        assert!(
            !disp.domain().is_encoding_wide(),
            "long-wavelength dispersion must name a regime: {:?}",
            disp.domain()
        );
        assert!(
            disp.domain()
                .regimes
                .iter()
                .any(|r| r.contains("longest non-zero")),
            "dispersion regime: {:?}",
            disp.domain()
        );
        let stable = f
            .claims()
            .into_iter()
            .find(|c| c.id_str() == STABLE)
            .unwrap();
        assert!(
            stable.domain().is_encoding_wide(),
            "stability is the current lattice encoding, not a hidden continuum regime"
        );
        let local = f
            .claims()
            .into_iter()
            .find(|c| c.id_str() == LOCAL)
            .unwrap();
        assert!(
            !local.domain().is_encoding_wide(),
            "locality must name nearest-neighbour, not encoding-wide: {:?}",
            local.domain()
        );
        assert!(
            local
                .domain()
                .regimes
                .iter()
                .any(|r| r.contains("nearest-neighbour")),
            "local regime: {:?}",
            local.domain()
        );
    }

    #[test]
    fn negative_mass_squared_is_a_computed_tachyon() {
        // The field knob → verdict diff: a negative mass² destabilizes the field
        // and, being an imaginary frequency, also makes it non-causal.
        let mut f = KleinGordonField::default();
        assert_eq!(verdict(&f, STABLE), VerdictKind::Holds);
        assert_eq!(verdict(&f, CAUSAL), VerdictKind::Holds);
        f.set("mass_squared", KnobValue::Float(-1.0)).unwrap();
        assert_eq!(verdict(&f, STABLE), VerdictKind::Fails);
        assert_eq!(verdict(&f, CAUSAL), VerdictKind::Fails);
    }

    #[test]
    fn even_a_slightly_negative_mass_squared_flips_both() {
        // Regression: when only the zero mode is tachyonic, both stable and
        // causal must still fail (the j=0 mode must not be skipped).
        for &m2 in &[-0.05_f64, -1.0] {
            let mut f = KleinGordonField::default();
            f.set("mass_squared", KnobValue::Float(m2)).unwrap();
            assert_eq!(verdict(&f, STABLE), VerdictKind::Fails, "m²={m2}");
            assert_eq!(verdict(&f, CAUSAL), VerdictKind::Fails, "m²={m2}");
        }
        // And on a tiny lattice where only the zero mode can be tachyonic.
        let mut small = KleinGordonField::default();
        small.set("sites", KnobValue::UInt(2)).unwrap();
        small.set("mass_squared", KnobValue::Float(-1.0)).unwrap();
        assert_eq!(verdict(&small, STABLE), VerdictKind::Fails);
        assert_eq!(verdict(&small, CAUSAL), VerdictKind::Fails);
    }

    #[test]
    fn massless_field_is_stable_and_causal() {
        // The zero mode has ω² = 0; that is fine, not a tachyon.
        let mut f = KleinGordonField::default();
        f.set("mass_squared", KnobValue::Float(0.0)).unwrap();
        assert_eq!(verdict(&f, STABLE), VerdictKind::Holds);
        assert_eq!(verdict(&f, CAUSAL), VerdictKind::Holds);
    }

    #[test]
    fn discretization_is_second_order_accurate() {
        // The discrete Laplacian converges at O(a²): computed order ≈ 2.
        let f = KleinGordonField::default();
        let v = {
            let c = f
                .claims()
                .into_iter()
                .find(|c| c.id_str() == SECOND_ORDER)
                .unwrap();
            f.evaluate(&c)
        };
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_eq!(v.empirical(), EmpiricalStatus::NotApplicable);
        let order_claim = f
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SECOND_ORDER)
            .unwrap();
        assert!(
            order_claim
                .domain()
                .regimes
                .iter()
                .any(|r| r.contains("|k a| < 1")),
            "second-order is a long-wavelength claim, not encoding-wide: {:?}",
            order_claim.domain()
        );
        assert!((f.convergence_order() - 2.0).abs() < 0.1);
        // An absurdly coarse lattice is a resolution gap, not a failed stencil.
        let mut coarse = KleinGordonField::default();
        coarse.set("spacing", KnobValue::Float(100.0)).unwrap();
        let c = coarse
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SECOND_ORDER)
            .unwrap();
        let u = coarse.evaluate(&c);
        assert_eq!(u.kind, VerdictKind::Undecidable);
        assert_eq!(u.empirical(), EmpiricalStatus::Inconclusive);
        assert_ne!(u.kind, VerdictKind::Fails);
        assert_ne!(
            u.derivation(),
            physis_core::DerivationAssurance::CertifiedNumeric
        );
    }

    #[test]
    fn mode_count_follows_the_sites_knob() {
        let mut f = KleinGordonField::default();
        f.set("sites", KnobValue::UInt(32)).unwrap();
        assert_eq!(f.claims().len(), 6);
        // 32 sites → 32 modes; the minimum ω² is the zero mode = mass².
        assert!((f.min_omega_sq() - f.mass_squared).abs() < 1e-9);
    }

    #[test]
    fn field_experiment_builds_a_matrix() {
        let r = field_modes();
        assert_eq!(r.id, "field-modes");
        assert_eq!(r.theories.len(), 2);
        assert_eq!(
            r.matrix
                .get(STABLE)
                .and_then(|m| m.get("klein-gordon"))
                .copied(),
            Some(VerdictKind::Holds)
        );
        assert_eq!(r.theories.len(), 2);
        assert_eq!(
            r.matrix
                .get(NO_DOUBLERS)
                .and_then(|m| m.get("dirac-fermion"))
                .copied(),
            Some(VerdictKind::Fails)
        );
        assert_eq!(
            r.matrix
                .get(NO_DOUBLERS)
                .and_then(|m| m.get("klein-gordon"))
                .copied(),
            Some(VerdictKind::Inapplicable)
        );
    }

    #[test]
    fn next_nearest_is_ir_not_a_knob() {
        let mut f = KleinGordonField::default();
        assert!(
            f.set("next_nearest", KnobValue::Bool(true)).is_err(),
            "next-nearest is an IR mutation, not a knob"
        );
        let src = render_package(&f.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            KleinGordonField::from_package(&pkg).unwrap(),
            f,
            "IR round-trip must preserve the nearest-neighbour stencil"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(KleinGordonField::nnn_equation()),
        );
        let parsed = KleinGordonField::from_package(&mutated).unwrap();
        assert!(parsed.next_nearest);
        let mut fork = f.clone();
        fork.next_nearest = true;
        assert_eq!(verdict(&fork, LOCAL), VerdictKind::Fails);
        assert_eq!(verdict(&f, LOCAL), VerdictKind::Holds);
        assert_eq!(verdict(&fork, DISPERSION), VerdictKind::Fails);
        let probes = f.structural_mutations();
        assert_eq!(probes.len(), 1);
        assert_eq!(probes[0].0, "add-next-nearest");
        assert_eq!(verdict(probes[0].1.as_ref(), LOCAL), VerdictKind::Fails);
        assert_eq!(verdict(&f, LOCAL), VerdictKind::Holds);
        assert!(fork.structural_mutations().is_empty());
        let canonical = physis_ir::certify_round_trip(&f.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = f.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), f.package());
        assert_eq!(verdict(rebuilt.as_ref(), LOCAL), VerdictKind::Holds);
        assert!(
            KleinGordonField::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-wilson"),
            "klein-gordon must not grow add-wilson"
        );
    }

    #[test]
    fn wilson_term_is_ir_not_a_knob() {
        let mut d = DiracFermion::default();
        assert!(
            DiracFermion::default()
                .set("wilson", KnobValue::Bool(true))
                .is_err(),
            "Wilson r is an IR mutation, not a knob"
        );
        assert!(
            DiracFermion::default()
                .set("mass_squared", KnobValue::Float(-1.0))
                .is_err(),
            "dirac-fermion must not grow mass_squared; that stays on klein-gordon"
        );
        assert!(
            DiracFermion::default()
                .set("next_nearest", KnobValue::Bool(true))
                .is_err(),
            "dirac-fermion must not grow next_nearest; that stays on klein-gordon"
        );
        let src = render_package(&d.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            DiracFermion::from_package(&pkg).unwrap(),
            d,
            "IR round-trip must preserve the naive Dirac operator"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(DiracFermion::wilson_equation()),
        );
        let parsed = DiracFermion::from_package(&mutated).unwrap();
        assert!(parsed.wilson);
        let mut fork = d.clone();
        fork.wilson = true;
        assert_eq!(verdict(&fork, NO_DOUBLERS), VerdictKind::Holds);
        assert_eq!(verdict(&d, NO_DOUBLERS), VerdictKind::Fails);
        assert_eq!(verdict(&fork, LOCAL), VerdictKind::Holds);
        assert_eq!(verdict(&d, LOCAL), VerdictKind::Holds);
        assert_eq!(verdict(&d, DISPERSION), VerdictKind::Holds);
        assert_eq!(verdict(&fork, DISPERSION), VerdictKind::Fails);
        assert_eq!(d.light_copies(), 2);
        assert_eq!(fork.light_copies(), 1);
        let edge = d.energy(d.sites / 2);
        assert!(
            (edge - d.mass.abs()).abs() < 1e-9,
            "naive edge energy must equal |m|, got {edge}"
        );
        let lifted = fork.dirac_mass(fork.sites / 2);
        assert!(
            (lifted - (fork.mass + 2.0 * WILSON_R / fork.spacing)).abs() < 1e-9,
            "Wilson edge mass must be m+2r/a, got {lifted}"
        );
        d.set("mass", KnobValue::Float(-1.0)).unwrap();
        assert_eq!(verdict(&d, NO_DOUBLERS), VerdictKind::Fails);
        let mut odd = DiracFermion::default();
        odd.set("sites", KnobValue::UInt(15)).unwrap();
        assert_eq!(verdict(&odd, NO_DOUBLERS), VerdictKind::Inapplicable);
        let probes = DiracFermion::default().structural_mutations();
        assert_eq!(probes.len(), 1);
        assert_eq!(probes[0].0, "add-wilson");
        assert_eq!(
            verdict(probes[0].1.as_ref(), NO_DOUBLERS),
            VerdictKind::Holds
        );
        assert!(fork.structural_mutations().is_empty());
        let live = DiracFermion::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), live.package());
        assert_eq!(
            rebuilt.get("mass").unwrap(),
            KnobValue::Float(1.0),
            "reparse must overlay operator IR onto live knobs"
        );
        assert_eq!(
            rebuilt.get("sites").unwrap(),
            KnobValue::UInt(16),
            "reparse must overlay operator IR onto live knobs"
        );
        assert_eq!(verdict(rebuilt.as_ref(), NO_DOUBLERS), VerdictKind::Fails);
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == NO_DOUBLERS)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "dirac no-doublers must name naive Dirac: {:?}",
            cell.domain()
        );
        let kg = KleinGordonField::default();
        assert!(
            kg.claims().iter().all(|c| c.id_str() != NO_DOUBLERS),
            "klein-gordon must not grow fermion.no-doublers"
        );
        assert!(
            DiracFermion::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-next-nearest"),
            "dirac-fermion must not grow add-next-nearest"
        );
        assert!(
            KleinGordonField::default()
                .set("mass_squared", KnobValue::Float(-1.0))
                .is_ok(),
            "klein-gordon keeps the mass_squared knob"
        );
    }
}
