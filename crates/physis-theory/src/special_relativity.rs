//! Special relativity as mechanized kinematics — and a knob that turns it off.
//!
//! Einstein's 1905 kinematics is not asserted here; it is *computed*. Six
//! identities are checked:
//!
//! - the spacetime interval `s² = (cΔt)² − Δx²` is unchanged by a boost,
//! - composing two subluminal velocities stays subluminal,
//! - the energy–momentum invariant `E² − (pc)² = (mc²)²` is frame-independent,
//!   built from *typed* quantities so `pc` and `mc²` are forced to be energies,
//! - the integer cross-product Jacobi identity (so(3) Lie bracket) holds
//!   as a catalog polynomial, independent of the boost encoding,
//! - the integer Lagrange identity |a × b|² + (a · b)² = |a|² |b|² holds
//!   as a distinct degree-4 catalog polynomial, also independent of boosts,
//! - and det(AB) = det(A) det(B) holds as a distinct degree-4 catalog
//!   polynomial on 2×2 integer matrices, independent of boosts.
//!
//! The Lorentz boost and the catalog interval, composition, mass-shell,
//! cross-product Jacobi, Lagrange identity, and 2x2 determinant-product trees live on the IR package. A truncated
//! binomial γ (`add-binomial-gamma`) is a package mutation, not the
//! `absolute_time` knob: interval and mass-shell fail on that fork.
//! Velocity composition stays Einstein on that fork. A minus-uv
//! collinear composition (`add-minus-uv`) is a second package mutation:
//! `w = (u+v)/(1−uv)` exceeds `c` while Lorentz boosts still hold the
//! interval and mass shell. That is not Galilean `u+v` and not the
//! `absolute_time` knob. `absolute_time` still switches exact Lorentz
//! to Galilean.

use physis_core::claim::{Claim, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::EmpiricalStatus;
use physis_core::ParameterOrigin;
use physis_core::{Energy, Mass, Momentum, Qty, Velocity};
use physis_ir::{apply_mutation, parse_package, render_package, PackageMutation, TheoryPackage};
use physis_model::constants::C;
use physis_model::{GaugeGroup, Manifold, Spectrum, World};
use physis_numeric::{residual_relation, Interval, Ratio, ResidualRelation, SciExact};
use physis_proof::catalog::{
    cross_product_jacobi, einstein_composition, energy_momentum, lagrange_identity,
    lorentz_interval, matrix_det_product,
};
use physis_proof::{identity_is_zero, lookup, parse_expr};

use crate::framework::Theory;

/// The spacetime interval is invariant under the boost.
pub const SR_INVARIANT_INTERVAL: &str = "sr.invariant-interval";
/// Composing two subluminal velocities stays subluminal.
pub const SR_SUBLUMINAL_COMPOSITION: &str = "sr.subluminal-composition";
/// The energy–momentum invariant `E² − (pc)² = (mc²)²` is frame-independent.
pub const SR_ENERGY_MOMENTUM: &str = "sr.energy-momentum-invariant";
/// Integer Jacobi identity for the R^3 cross product (x-component).
pub const SR_CROSS_PRODUCT_JACOBI: &str = "sr.cross-product-jacobi";
/// Integer Lagrange identity for Euclidean cross and dot products on Z^3.
pub const SR_LAGRANGE_IDENTITY: &str = "sr.lagrange-identity";
/// Integer 2×2 determinant multiplicativity over M_2(Z).
pub const SR_MATRIX_DET_PRODUCT: &str = "sr.matrix-det-product";

/// The demonstration boost speed, as a fraction of `c`.
const BETA: f64 = 0.6;
/// Tiny boost where a truncated γ can look Lorentzian in a sample.
#[cfg(test)]
const BETA_VANISHING: f64 = 1.0e-6;
/// Exact Lorentz boost on the live SR package.
const BOOST_LORENTZ: &str = "boost lorentz";
/// Catalog interval identity tree (c = 1). Not a kernel proof.
const INTERVAL_EQ: &str = "(t - beta * x)^2 - (x - beta * t)^2 - (1 - beta^2) * (t^2 - x^2)";
/// Catalog Einstein composition identity tree (c = 1). Not a kernel proof.
const COMPOSITION_EQ: &str = "(1 + u * v)^2 - (u + v)^2 - (1 - u^2) * (1 - v^2)";
/// Catalog mass-shell identity tree (c = 1). Not a kernel proof.
const MASS_SHELL_EQ: &str = "(E - beta * p)^2 - (p - beta * E)^2 - (1 - beta^2) * (E^2 - p^2)";
/// Catalog cross-product Jacobi identity tree. Not a kernel proof.
const JACOBI_EQ: &str = "((((a2 * ((b1 * c2) - (b2 * c1))) - (a3 * ((b3 * c1) - (b1 * c3)))) + ((b2 * ((c1 * a2) - (c2 * a1))) - (b3 * ((c3 * a1) - (c1 * a3))))) + ((c2 * ((a1 * b2) - (a2 * b1))) - (c3 * ((a3 * b1) - (a1 * b3)))))";
/// Catalog Lagrange identity tree. Not a kernel proof.
const LAGRANGE_EQ: &str = "(((((((a2 * b3) - (a3 * b2)))^2 + (((a3 * b1) - (a1 * b3)))^2) + (((a1 * b2) - (a2 * b1)))^2) + ((((a1 * b1) + (a2 * b2)) + (a3 * b3)))^2) - ((((a1)^2 + (a2)^2) + (a3)^2) * (((b1)^2 + (b2)^2) + (b3)^2)))";
/// Catalog 2×2 determinant-product identity tree. Not a kernel proof.
const DET_PRODUCT_EQ: &str = "(((((a11 * b11) + (a12 * b21)) * ((a21 * b12) + (a22 * b22))) - (((a11 * b12) + (a12 * b22)) * ((a21 * b11) + (a22 * b21)))) - (((a11 * a22) - (a12 * a21)) * ((b11 * b22) - (b12 * b21))))";
/// Truncated binomial γ = 1 + β²/2.
const BOOST_BINOMIAL: &str = "boost binomial-gamma";
/// Collinear Einstein addition with the denominator sign flipped.
const COMPOSE_MINUS_UV: &str = "compose minus-uv";
/// Composition probe used by the evaluator (`0.8c ⊕ 0.7c`).
const COMPOSE_U: f64 = 0.8;
const COMPOSE_V: f64 = 0.7;

fn parse_sr_encoding(pkg: &TheoryPackage) -> Result<(bool, bool), String> {
    let interval = lorentz_interval().canonical();
    let composition = einstein_composition().canonical();
    let mass_shell = energy_momentum().canonical();
    let jacobi = cross_product_jacobi().canonical();
    let lagrange = lagrange_identity().canonical();
    let det_product = matrix_det_product().canonical();
    let mut lorentz = false;
    let mut binomial = false;
    let mut minus_uv = false;
    let mut interval_tree = false;
    let mut composition_tree = false;
    let mut mass_shell_tree = false;
    let mut jacobi_tree = false;
    let mut lagrange_tree = false;
    let mut det_product_tree = false;
    for eq in &pkg.equations {
        match eq.trim() {
            BOOST_LORENTZ => lorentz = true,
            BOOST_BINOMIAL => binomial = true,
            COMPOSE_MINUS_UV => minus_uv = true,
            t => {
                let tree = parse_expr(t)
                    .map_err(|e| format!("special-relativity catalog identity equation: {e}"))?
                    .canonical();
                if tree == interval {
                    if interval_tree {
                        return Err(format!(
                            "{} package has two interval identity trees",
                            pkg.id
                        ));
                    }
                    interval_tree = true;
                } else if tree == composition {
                    if composition_tree {
                        return Err(format!(
                            "{} package has two composition identity trees",
                            pkg.id
                        ));
                    }
                    composition_tree = true;
                } else if tree == mass_shell {
                    if mass_shell_tree {
                        return Err(format!(
                            "{} package has two mass-shell identity trees",
                            pkg.id
                        ));
                    }
                    mass_shell_tree = true;
                } else if tree == jacobi {
                    if jacobi_tree {
                        return Err(format!(
                            "{} package has two cross-product Jacobi identity trees",
                            pkg.id
                        ));
                    }
                    jacobi_tree = true;
                } else if tree == lagrange {
                    if lagrange_tree {
                        return Err(format!(
                            "{} package has two Lagrange identity trees",
                            pkg.id
                        ));
                    }
                    lagrange_tree = true;
                } else if tree == det_product {
                    if det_product_tree {
                        return Err(format!(
                            "{} package has two 2x2 determinant-product identity trees",
                            pkg.id
                        ));
                    }
                    det_product_tree = true;
                } else {
                    return Err(format!(
                        "special-relativity equation is not the Lorentz boost, a catalog identity tree, binomial-gamma, or minus-uv: {t}"
                    ));
                }
            }
        }
    }
    if !lorentz {
        return Err(format!("{} package has no Lorentz boost", pkg.id));
    }
    if !interval_tree {
        return Err(format!("{} package has no interval identity tree", pkg.id));
    }
    if !composition_tree {
        return Err(format!(
            "{} package has no composition identity tree",
            pkg.id
        ));
    }
    if !mass_shell_tree {
        return Err(format!(
            "{} package has no mass-shell identity tree",
            pkg.id
        ));
    }
    if !jacobi_tree {
        return Err(format!(
            "{} package has no cross-product Jacobi identity tree",
            pkg.id
        ));
    }
    if !lagrange_tree {
        return Err(format!("{} package has no Lagrange identity tree", pkg.id));
    }
    if !det_product_tree {
        return Err(format!(
            "{} package has no 2x2 determinant-product identity tree",
            pkg.id
        ));
    }
    Ok((binomial, minus_uv))
}

fn gamma_lorentz(beta: f64) -> f64 {
    1.0 / (1.0 - beta * beta).sqrt()
}

/// Midpoint of a same-exp10 [`physis_numeric::SciInterval`]. Not a certificate.
fn sci_interval_centre(hull: physis_numeric::SciInterval) -> SciExact {
    assert_eq!(
        hull.lo.exp10, hull.hi.exp10,
        "LEDGER m_e SciInterval endpoints must share exp10 so the midpoint is a SciExact"
    );
    SciExact::new(
        (hull.lo.significand + hull.hi.significand) / 2,
        hull.lo.exp10,
    )
}

/// Versioned `c` and `m_e` for the mass-shell sample. Missing LEDGER names fail closed.
fn versioned_mass_shell_inputs() -> (Qty<Mass>, Qty<Velocity>, [String; 2]) {
    let c_listing =
        physis_constants::lookup("c").expect("LEDGER name c must exist for SR mass-shell");
    let me_listing =
        physis_constants::lookup("m_e").expect("LEDGER name m_e must exist for SR mass-shell");
    let c: Qty<Velocity> = Qty::new(physis_constants::speed_of_light().value.to_f64());
    let m: Qty<Mass> =
        Qty::new(sci_interval_centre(physis_constants::electron_mass().value).to_f64());
    let lines = [
        format!(
            "versioned c  kind {}  hash {}  value {}  unit {}  exact SI 2019 Ratio (not a measured hull, not P3N)",
            c_listing.kind,
            c_listing.hash.to_hex(),
            c_listing.value,
            c_listing.unit
        ),
        format!(
            "versioned m_e  kind {}  hash {}  hull {}  unit {}  CODATA 2018 one-sigma SciInterval (not an SI defining Ratio, not P3N)",
            me_listing.kind,
            me_listing.hash.to_hex(),
            me_listing.value,
            me_listing.unit
        ),
    ];
    (m, c, lines)
}

fn gamma_binomial(beta: f64) -> f64 {
    1.0 + 0.5 * beta * beta
}

/// Residual γ_L − γ_bin. Evidence, not the encoding. β → 0 recovers
/// Lorentz and the interval cell still fails.
fn binomial_gamma_residual(beta: f64) -> f64 {
    gamma_lorentz(beta) - gamma_binomial(beta)
}

/// Residual w_minus − w_Einstein. Evidence, not the encoding. Tiny
/// speeds stay subluminal and the composition cell still fails.
fn minus_uv_residual(u: f64, v: f64) -> f64 {
    let einstein = (u + v) / (1.0 + u * v);
    let minus = (u + v) / (1.0 - u * v);
    minus - einstein
}

/// Half-width of the relative IEEE rounding band, in ulps of 1.0.
///
/// 64 ulps = `2^{-46}`. This 1+1 sample does ~20 flops (two Minkowski
/// squares, a Lorentz or Galilean boost including `√(1-β²)` and a
/// reciprocal, then a relative difference). Round-to-nearest is ≤ 0.5 ulp
/// per op; sqrt is ~1 ulp. A few-dozen-ulp relative band is several times
/// that count. It is **not** folklore `1e-9` and **not** the CODATA `m_e`
/// hull. Measured residuals of this sample: interval `|s0−s1|/|s0| ≈
/// 1.78e-16` (~1 ulp of `s² ~ 5`); mass-shell residual `0` (γ(β=0.6) is
/// exactly 1.25 in f64). Galilean relative error is O(1).
const SAMPLE_ROUNDING_ULPS: i128 = 64;
const F64_ULP_DEN: i128 = 1i128 << 52;

/// Relative IEEE rounding band around 0 for this sample's boost arithmetic.
fn sample_rounding_band() -> Interval {
    Interval::new(
        Ratio::new(-SAMPLE_ROUNDING_ULPS, F64_ULP_DEN),
        Ratio::new(SAMPLE_ROUNDING_ULPS, F64_ULP_DEN),
    )
}

/// Enclose the relative residual `(before − after) / |before|`.
///
/// The computed ratio is snapped to a dyadic of `2^{-52}` (one ulp of 1)
/// and widened by one ulp. That is the scale of [`sample_rounding_band`].
/// `Interval::from_f64_approx` of a ~1 ulp relative residual saturates
/// `i128` products (denominators ~`2^{105}`) and is not this enclosure.
fn relative_residual_hull(before: f64, after: f64) -> Interval {
    let scale = before.abs();
    let rel = if scale == 0.0 {
        after
    } else {
        (before - after) / scale
    };
    if !rel.is_finite() {
        return Interval::from_f64_approx(0.0);
    }
    let snapped = Ratio::nearest(rel, F64_ULP_DEN);
    let ulp = Ratio::new(1, F64_ULP_DEN);
    Interval::new(snapped - ulp, snapped + ulp)
}

/// Classify a residual hull against this sample's IEEE rounding band.
fn classify_boost_residual(residual: Interval) -> ResidualRelation {
    residual_relation(residual, sample_rounding_band())
}

fn sample_numeric_evidence(
    domain: &str,
    residual: Interval,
    extra: impl IntoIterator<Item = String>,
) -> Vec<String> {
    let band = sample_rounding_band();
    let relation = classify_boost_residual(residual);
    let mut lines = vec![
        format!("sample domain: {domain}"),
        "error sources: IEEE rounding of this boost arithmetic (64-ulp relative band 2^{-46} for ~20 flops; not folklore 1e-9); overlap of intervals is not equality".into(),
        "the catalog integer identity is a distinct kernel obligation, not this floating-point sample".into(),
        format!("relative residual hull {residual} vs rounding band {band} ({relation})"),
    ];
    lines.extend(extra);
    lines
}

/// Decide a Lorentz/Galilean interval or mass-shell **sample**.
///
/// Contained → Holds. Disjoint → Fails. Overlap without containment →
/// `Undecidable` + `Inconclusive` (`numeric unresolved`), not a theorem.
/// Derivation stays Executed. Does not mint CertifiedNumeric / P3N.
fn decide_boost_sample(
    claim: &Claim,
    residual: Interval,
    hold_summary: impl Into<String>,
    fail_summary: impl Into<String>,
    evidence: impl IntoIterator<Item = String>,
) -> Verdict {
    let relation = classify_boost_residual(residual);
    let v = match relation {
        ResidualRelation::Contained => Verdict::holds(claim, hold_summary),
        ResidualRelation::Disjoint => Verdict::fails(claim, fail_summary),
        ResidualRelation::OverlapsWithoutContainment => Verdict::undecidable(
            claim,
            "relative residual overlaps the IEEE rounding band without containment; overlap is not equality",
        )
        .with_empirical(EmpiricalStatus::Inconclusive),
    };
    v.with_evidence(evidence)
        .with_interval_enclosure(residual.lo.to_string(), residual.hi.to_string())
}

const SPECS: &[KnobSpec] = &[KnobSpec {
    name: "absolute_time",
    layer: LayerId::Spacetime,
    doc: "If true, boosts are Galilean (time is absolute) instead of Lorentzian. Turning this on breaks every relativistic invariant — the pre-1905 worldview. A truncated γ is not this knob: add-binomial-gamma is an IR mutation. Minus-uv composition is not this knob: add-minus-uv is an IR mutation.",
    origin: ParameterOrigin::Chosen,
    domain: KnobDomain::Bool,
}];

/// Special relativity: flat Minkowski kinematics with a Galilean-toggle knob.
///
/// The Lorentz boost and the catalog interval, composition, mass-shell,
/// cross-product Jacobi, Lagrange identity, and 2x2 determinant-product trees live on the IR package. Truncated binomial γ
/// (`add-binomial-gamma`) is a package mutation, not a knob: interval
/// and mass-shell fail. Minus-uv collinear composition (`add-minus-uv`)
/// is a second package mutation, not a knob: subluminal composition
/// fails while Lorentz boosts still hold. The Jacobi, Lagrange, and det-product identities stay Holds
/// on those forks. `absolute_time` still selects Galilean boosts.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SpecialRelativity {
    /// If true, use Galilean boosts (absolute time) instead of Lorentzian.
    absolute_time: bool,
    /// Whether the encoding uses γ = 1 + β²/2 instead of exact Lorentz.
    binomial_gamma: bool,
    /// Whether collinear composition uses (u+v)/(1−uv) instead of Einstein.
    minus_uv: bool,
}

impl SpecialRelativity {
    /// IR package for this boost encoding. Equations are `boost lorentz`,
    /// the catalog interval, composition, mass-shell, cross-product
    /// Jacobi, Lagrange identity, and 2x2 determinant-product trees, and, when forked, `boost binomial-gamma`
    /// and/or `compose minus-uv`. `absolute_time` stays on the struct.
    /// `lean_ref` is the catalog interval type, not a Physlib pointer
    /// without the tree.
    pub fn package(&self) -> TheoryPackage {
        let interval = lookup(SR_INVARIANT_INTERVAL).expect("interval is a catalog identity");
        let composition =
            lookup(SR_SUBLUMINAL_COMPOSITION).expect("composition is a catalog identity");
        let mass_shell = lookup(SR_ENERGY_MOMENTUM).expect("mass shell is a catalog identity");
        let jacobi = lookup(SR_CROSS_PRODUCT_JACOBI).expect("Jacobi is a catalog identity");
        let lagrange = lookup(SR_LAGRANGE_IDENTITY).expect("Lagrange is a catalog identity");
        let det_product = lookup(SR_MATRIX_DET_PRODUCT).expect("det-product is a catalog identity");
        let mut equations = vec![
            BOOST_LORENTZ.to_string(),
            INTERVAL_EQ.to_string(),
            COMPOSITION_EQ.to_string(),
            MASS_SHELL_EQ.to_string(),
            JACOBI_EQ.to_string(),
            LAGRANGE_EQ.to_string(),
            DET_PRODUCT_EQ.to_string(),
        ];
        if self.binomial_gamma {
            equations.push(BOOST_BINOMIAL.to_string());
        }
        if self.minus_uv {
            equations.push(COMPOSE_MINUS_UV.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["lorentz-boost".into()],
            equations,
            claims: vec![
                physis_ir::ClaimDecl {
                    id: SR_INVARIANT_INTERVAL.into(),
                    statement: interval.statement.into(),
                    layer: "spacetime".into(),
                    class: "model-internal".into(),
                },
                physis_ir::ClaimDecl {
                    id: SR_SUBLUMINAL_COMPOSITION.into(),
                    statement: composition.statement.into(),
                    layer: "spacetime".into(),
                    class: "model-internal".into(),
                },
                physis_ir::ClaimDecl {
                    id: SR_ENERGY_MOMENTUM.into(),
                    statement: mass_shell.statement.into(),
                    layer: "particle".into(),
                    class: "model-internal".into(),
                },
                physis_ir::ClaimDecl {
                    id: SR_CROSS_PRODUCT_JACOBI.into(),
                    statement: jacobi.statement.into(),
                    layer: "mathematical".into(),
                    class: "mathematical".into(),
                },
                physis_ir::ClaimDecl {
                    id: SR_LAGRANGE_IDENTITY.into(),
                    statement: lagrange.statement.into(),
                    layer: "mathematical".into(),
                    class: "mathematical".into(),
                },
                physis_ir::ClaimDecl {
                    id: SR_MATRIX_DET_PRODUCT.into(),
                    statement: det_product.statement.into(),
                    layer: "mathematical".into(),
                    class: "mathematical".into(),
                },
            ],
            lean_ref: Some(interval.lean_type.into()),
        }
    }

    /// Load a boost encoding from a package. Knobs default; overlay them
    /// from a live SR object when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        if pkg.id != "special-relativity" {
            return Err(format!(
                "special-relativity package id '{}' is not special-relativity",
                pkg.id
            ));
        }
        let (binomial_gamma, minus_uv) = parse_sr_encoding(pkg)?;
        Ok(Self {
            binomial_gamma,
            minus_uv,
            ..Self::default()
        })
    }

    fn binomial_equation() -> String {
        BOOST_BINOMIAL.to_string()
    }

    fn minus_uv_equation() -> String {
        COMPOSE_MINUS_UV.to_string()
    }

    /// Boost a two-vector `(a0, a1)` by `beta` (in units of `c`).
    ///
    /// Binomial γ is an IR encoding. Otherwise Lorentz, or Galilean when
    /// `absolute_time` is set. Both `(ct, x)` and `(E, pc)` are 4-vectors.
    fn boost(&self, a0: f64, a1: f64, beta: f64) -> (f64, f64) {
        if self.binomial_gamma {
            let gamma = gamma_binomial(beta);
            (gamma * (a0 - beta * a1), gamma * (a1 - beta * a0))
        } else if self.absolute_time {
            (a0, a1 - beta * a0)
        } else {
            let gamma = gamma_lorentz(beta);
            (gamma * (a0 - beta * a1), gamma * (a1 - beta * a0))
        }
    }

    /// Relativistic (or, under the knob, Galilean) composition of two speeds
    /// given as fractions of `c`. Minus-uv is an IR encoding.
    fn compose_speeds(&self, u: f64, v: f64) -> f64 {
        if self.minus_uv {
            (u + v) / (1.0 - u * v)
        } else if self.absolute_time {
            u + v
        } else {
            (u + v) / (1.0 + u * v)
        }
    }
}

impl Knobbed for SpecialRelativity {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "absolute_time" => Ok(KnobValue::Bool(self.absolute_time)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("absolute_time", KnobValue::Bool(v)) => self.absolute_time = v,
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

impl Theory for SpecialRelativity {
    fn id(&self) -> &'static str {
        "special-relativity"
    }
    fn name(&self) -> &'static str {
        "Special relativity"
    }
    fn summary(&self) -> &'static str {
        "Flat Minkowski kinematics: the invariant interval, subluminal velocity \
         composition, the mass shell E² = (pc)² + (mc²)², the integer \
         cross-product Jacobi identity, the integer Lagrange identity, and the integer 2x2 determinant product, all computed. The Lorentz boost is an IR \
         encoding. Truncated binomial γ is an IR mutation, not the absolute_time \
         knob. Minus-uv collinear composition is a second IR mutation, not that \
         knob. That knob still replaces exact Lorentz with Galilean boosts. The \
         Jacobi, Lagrange, and det-product identities are independent of those encodings."
    }
    fn world(&self) -> Option<World> {
        Some(World {
            spacetime: Manifold::observed_4d(),
            gauge: GaugeGroup::trivial(),
            spectrum: Spectrum::empty(),
            has_gravity: false,
            supersymmetric: false,
            free_parameter_count: 0, // c is a unit conversion, not a free parameter
            landscape_log10: 0.0,
            note: format!(
                "special relativity, boosts = {}",
                if self.binomial_gamma {
                    "binomial-γ"
                } else if self.absolute_time {
                    "Galilean"
                } else {
                    "Lorentzian"
                }
            ),
        })
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            lookup(SR_INVARIANT_INTERVAL)
                .expect("interval is a catalog identity")
                .lab_claim(),
            lookup(SR_SUBLUMINAL_COMPOSITION)
                .expect("composition is a catalog identity")
                .lab_claim()
                .with_dependencies(&[SR_INVARIANT_INTERVAL]),
            lookup(SR_ENERGY_MOMENTUM)
                .expect("mass shell is a catalog identity")
                .lab_claim()
                .with_dependencies(&[SR_INVARIANT_INTERVAL]),
            lookup(SR_CROSS_PRODUCT_JACOBI)
                .expect("Jacobi is a catalog identity")
                .lab_claim(),
            lookup(SR_LAGRANGE_IDENTITY)
                .expect("Lagrange is a catalog identity")
                .lab_claim(),
            lookup(SR_MATRIX_DET_PRODUCT)
                .expect("det-product is a catalog identity")
                .lab_claim(),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id_str() {
            SR_INVARIANT_INTERVAL => {
                // Floating-point sample. Not identity_is_zero: the catalog
                // lorentz_interval polynomial is a distinct kernel obligation.
                let c = C.value();
                let ct0 = c * 1.0e-8;
                let x0 = 2.0;
                let s0 = ct0 * ct0 - x0 * x0;
                let (ct1, x1) = self.boost(ct0, x0, BETA);
                let s1 = ct1 * ct1 - x1 * x1;
                if self.binomial_gamma {
                    let residual = binomial_gamma_residual(BETA);
                    Verdict::fails(claim, "binomial γ: the interval is not Lorentz-invariant")
                        .with_evidence([format!(
                        "γ_L − γ_bin = {residual:.6} at β = {BETA}; s² = {s0:.4e} m² → {s1:.4e} m²"
                    )])
                } else {
                    let residual = relative_residual_hull(s0, s1);
                    decide_boost_sample(
                        claim,
                        residual,
                        "s² is unchanged by the Lorentz boost",
                        "the interval is not invariant under a Galilean boost",
                        sample_numeric_evidence(
                            "1+1 Minkowski, β=0.6, event (c·10 ns, 2 m)",
                            residual,
                            [
                                format!("s² = {s0:.4e} m² before and {s1:.4e} m² after"),
                                "interval uses model C (SI exact Ratio as f64); not a new constant"
                                    .into(),
                            ],
                        ),
                    )
                }
            }
            SR_SUBLUMINAL_COMPOSITION => {
                let (u, v) = (COMPOSE_U, COMPOSE_V);
                let w = self.compose_speeds(u, v);
                if self.minus_uv {
                    let residual = minus_uv_residual(u, v);
                    Verdict::fails(
                        claim,
                        "minus-uv composition: two subluminal speeds exceed c",
                    )
                    .with_evidence([format!(
                        "(u+v)/(1-uv) − (u+v)/(1+uv) = {residual:.4} at {u}c ⊕ {v}c = {w:.4}c"
                    )])
                } else if w < 1.0 {
                    Verdict::holds(claim, "relativistic composition keeps the result below c")
                        .with_evidence([format!("0.8c ⊕ 0.7c = {w:.4}c < c")])
                } else {
                    Verdict::fails(claim, "Galilean addition exceeds c")
                        .with_evidence([format!("0.8c + 0.7c = {w:.4}c ≥ c")])
                }
            }
            SR_ENERGY_MOMENTUM => {
                // Floating-point sample. Not identity_is_zero: the catalog
                // energy_momentum polynomial is a distinct kernel obligation.
                let (m, c, versioned) = versioned_mass_shell_inputs();
                let mc2: Qty<Energy> = m * c * c;
                let (e1, pc1) = self.boost(mc2.value(), 0.0, BETA);
                let shell = e1 * e1 - pc1 * pc1;
                let rest = mc2.value() * mc2.value();
                let p1: Qty<Momentum> = Qty::<Energy>::new(pc1) / c;
                if self.binomial_gamma {
                    let residual = binomial_gamma_residual(BETA);
                    Verdict::fails(
                        claim,
                        "binomial γ: the mass shell is not Lorentz-invariant",
                    )
                    .with_evidence([format!(
                        "γ_L − γ_bin = {residual:.6} at β = {BETA}; E² − (pc)² = {shell:.4e} J² vs (mc²)² = {rest:.4e} J²"
                    )])
                    .with_evidence(versioned)
                } else {
                    let residual = relative_residual_hull(rest, shell);
                    let mut extra = vec![
                        format!(
                            "mc² = {:.4e} J, boosted |p| = {:.4e} kg·m/s",
                            mc2.value(),
                            p1.value()
                        ),
                        format!("E² − (pc)² = {shell:.4e} J² vs (mc²)² = {rest:.4e} J²"),
                        "c is exact SI 2019 Ratio; m_e SciInterval is input identity, not this rounding bound".into(),
                    ];
                    extra.extend(versioned);
                    decide_boost_sample(
                        claim,
                        residual,
                        "E² − (pc)² equals (mc²)² in the boosted frame",
                        "the mass shell is not preserved by a Galilean boost",
                        sample_numeric_evidence(
                            "1+1 Minkowski, β=0.6, rest electron",
                            residual,
                            extra,
                        ),
                    )
                }
            }
            SR_CROSS_PRODUCT_JACOBI => match identity_is_zero(&cross_product_jacobi()) {
                Ok(()) => Verdict::holds(
                    claim,
                    "a × (b × c) + cyclic vanishes on the x-component over Z",
                )
                .with_evidence([
                    "recursive and postfix expanders agree the cross-product Jacobi x-component is the zero polynomial".to_string(),
                ]),
                Err(e) => Verdict::fails(claim, e),
            },
            SR_LAGRANGE_IDENTITY => match identity_is_zero(&lagrange_identity()) {
                Ok(()) => Verdict::holds(claim, "the Lagrange identity holds over Z^3").with_evidence([
                    "recursive and postfix expanders agree the Lagrange identity is the zero polynomial".to_string(),
                ]),
                Err(e) => Verdict::fails(claim, e),
            },
            SR_MATRIX_DET_PRODUCT => match identity_is_zero(&matrix_det_product()) {
                Ok(()) => Verdict::holds(
                    claim,
                    "det(AB) equals det(A) det(B) over 2x2 integer matrices",
                )
                .with_evidence([
                    "recursive and postfix expanders agree the 2x2 determinant product is the zero polynomial".to_string(),
                ]),
                Err(e) => Verdict::fails(claim, e),
            },
            _ => Verdict::inapplicable(claim, "claim not made by the special-relativity object"),
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.binomial_gamma = parsed.binomial_gamma;
        fork.minus_uv = parsed.minus_uv;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mut out: Vec<(String, Box<dyn Theory>)> = Vec::new();
        if !self.binomial_gamma {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::binomial_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.binomial_gamma {
                    let mut fork = self.clone();
                    fork.binomial_gamma = true;
                    out.push(("add-binomial-gamma".into(), Box::new(fork)));
                }
            }
        }
        if !self.minus_uv {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::minus_uv_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.minus_uv {
                    let mut fork = self.clone();
                    fork.minus_uv = true;
                    out.push(("add-minus-uv".into(), Box::new(fork)));
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;
    use physis_core::DerivationAssurance;
    use physis_numeric::SciExact;

    fn kind(t: &dyn Theory, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id_str() == id).unwrap();
        t.evaluate(&c).kind
    }

    fn verdict(t: &dyn Theory, id: &str) -> Verdict {
        let c = t.claims().into_iter().find(|c| c.id_str() == id).unwrap();
        t.evaluate(&c)
    }

    fn evidence_blob(v: &Verdict) -> String {
        v.evidence.join("\n")
    }

    fn assert_numeric_sample_honesty(blob: &str) {
        assert!(blob.contains("1+1 Minkowski"), "{blob}");
        assert!(blob.contains("β=0.6"), "{blob}");
        assert!(
            blob.contains("IEEE") && blob.contains("rounding"),
            "error sources must name IEEE rounding: {blob}"
        );
        assert!(
            blob.contains("overlap") && blob.contains("not equality"),
            "overlap is not equality: {blob}"
        );
        assert!(
            blob.contains("catalog") && blob.contains("sample"),
            "catalog polynomial is not this sample: {blob}"
        );
        assert!(
            !blob.contains("zero polynomial"),
            "float sample must not claim identity_is_zero: {blob}"
        );
        assert!(
            !blob.contains("expanders agree"),
            "float sample must not call identity_is_zero: {blob}"
        );
    }

    #[test]
    fn lorentz_kinematics_holds_all_invariants() {
        let sr = SpecialRelativity::default();
        assert!(!sr.absolute_time);
        assert_eq!(kind(&sr, SR_INVARIANT_INTERVAL), VerdictKind::Holds);
        assert_eq!(kind(&sr, SR_SUBLUMINAL_COMPOSITION), VerdictKind::Holds);
        assert_eq!(kind(&sr, SR_ENERGY_MOMENTUM), VerdictKind::Holds);
        assert_eq!(kind(&sr, SR_CROSS_PRODUCT_JACOBI), VerdictKind::Holds);
        assert_eq!(kind(&sr, SR_LAGRANGE_IDENTITY), VerdictKind::Holds);
        assert_eq!(kind(&sr, SR_MATRIX_DET_PRODUCT), VerdictKind::Holds);
    }

    #[test]
    fn live_package_binds_the_catalog_identity_trees() {
        let pkg = SpecialRelativity::default().package();
        let bound = physis_proof::catalog_tree_binding(pkg.lean_ref.as_deref(), &pkg.equations)
            .unwrap()
            .expect("live SR must bind the interval tree");
        assert_eq!(bound.claim_id, SR_INVARIANT_INTERVAL);
        assert_eq!(pkg.equations[0], BOOST_LORENTZ);
        assert_eq!(pkg.equations[1], INTERVAL_EQ);
        assert_eq!(pkg.equations[2], COMPOSITION_EQ);
        assert_eq!(pkg.equations[3], MASS_SHELL_EQ);
        assert_eq!(pkg.equations[4], JACOBI_EQ);
        assert_eq!(pkg.equations[5], LAGRANGE_EQ);
        assert_eq!(pkg.equations[6], DET_PRODUCT_EQ);
        let composition = lookup(SR_SUBLUMINAL_COMPOSITION).unwrap();
        let bound_c =
            physis_proof::catalog_tree_binding(Some(composition.lean_type), &pkg.equations)
                .unwrap()
                .expect("live SR must carry the composition tree");
        assert_eq!(bound_c.claim_id, SR_SUBLUMINAL_COMPOSITION);
        let mass_shell = lookup(SR_ENERGY_MOMENTUM).unwrap();
        let bound_m =
            physis_proof::catalog_tree_binding(Some(mass_shell.lean_type), &pkg.equations)
                .unwrap()
                .expect("live SR must carry the mass-shell tree");
        assert_eq!(bound_m.claim_id, SR_ENERGY_MOMENTUM);
        let jacobi = lookup(SR_CROSS_PRODUCT_JACOBI).unwrap();
        let bound_j = physis_proof::catalog_tree_binding(Some(jacobi.lean_type), &pkg.equations)
            .unwrap()
            .expect("live SR must carry the Jacobi tree");
        assert_eq!(bound_j.claim_id, SR_CROSS_PRODUCT_JACOBI);
        assert_eq!(
            parse_expr(JACOBI_EQ).unwrap().canonical(),
            cross_product_jacobi().canonical()
        );
        let lagrange = lookup(SR_LAGRANGE_IDENTITY).unwrap();
        let bound_l = physis_proof::catalog_tree_binding(Some(lagrange.lean_type), &pkg.equations)
            .unwrap()
            .expect("live SR must carry the Lagrange tree");
        assert_eq!(bound_l.claim_id, SR_LAGRANGE_IDENTITY);
        assert_eq!(
            parse_expr(LAGRANGE_EQ).unwrap().canonical(),
            lagrange_identity().canonical()
        );
        let det_product = lookup(SR_MATRIX_DET_PRODUCT).unwrap();
        let bound_d =
            physis_proof::catalog_tree_binding(Some(det_product.lean_type), &pkg.equations)
                .unwrap()
                .expect("live SR must carry the det-product tree");
        assert_eq!(bound_d.claim_id, SR_MATRIX_DET_PRODUCT);
        assert_eq!(
            parse_expr(DET_PRODUCT_EQ).unwrap().canonical(),
            matrix_det_product().canonical()
        );
        assert_eq!(pkg.claims.len(), 6);
    }

    #[test]
    fn token_boost_without_the_interval_tree_is_closed() {
        let mut pkg = SpecialRelativity::default().package();
        pkg.equations = vec![BOOST_LORENTZ.to_string()];
        let err = SpecialRelativity::from_package(&pkg).unwrap_err();
        assert!(err.contains("no interval identity tree"), "{err}");
        assert!(!err.contains("receipt"), "{err}");
    }

    #[test]
    fn interval_tree_without_composition_or_mass_shell_is_closed() {
        let mut pkg = SpecialRelativity::default().package();
        pkg.equations = vec![BOOST_LORENTZ.to_string(), INTERVAL_EQ.to_string()];
        let err = SpecialRelativity::from_package(&pkg).unwrap_err();
        assert!(err.contains("no composition identity tree"), "{err}");
        pkg.equations = vec![
            BOOST_LORENTZ.to_string(),
            INTERVAL_EQ.to_string(),
            COMPOSITION_EQ.to_string(),
        ];
        let err = SpecialRelativity::from_package(&pkg).unwrap_err();
        assert!(err.contains("no mass-shell identity tree"), "{err}");
        pkg.equations = vec![
            BOOST_LORENTZ.to_string(),
            INTERVAL_EQ.to_string(),
            COMPOSITION_EQ.to_string(),
            MASS_SHELL_EQ.to_string(),
        ];
        let err = SpecialRelativity::from_package(&pkg).unwrap_err();
        assert!(
            err.contains("no cross-product Jacobi identity tree"),
            "{err}"
        );
        pkg.equations = vec![
            BOOST_LORENTZ.to_string(),
            INTERVAL_EQ.to_string(),
            COMPOSITION_EQ.to_string(),
            MASS_SHELL_EQ.to_string(),
            JACOBI_EQ.to_string(),
        ];
        let err = SpecialRelativity::from_package(&pkg).unwrap_err();
        assert!(err.contains("no Lagrange identity tree"), "{err}");
        pkg.equations = vec![
            BOOST_LORENTZ.to_string(),
            INTERVAL_EQ.to_string(),
            COMPOSITION_EQ.to_string(),
            MASS_SHELL_EQ.to_string(),
            JACOBI_EQ.to_string(),
            LAGRANGE_EQ.to_string(),
        ];
        let err = SpecialRelativity::from_package(&pkg).unwrap_err();
        assert!(
            err.contains("no 2x2 determinant-product identity tree"),
            "{err}"
        );
        assert!(!err.contains("receipt"), "{err}");
    }

    #[test]
    fn absolute_time_knob_breaks_every_invariant() {
        // The Galilean→Einstein revolution as one knob turn: flipping to
        // absolute time makes all three relativistic theorems fail.
        let mut sr = SpecialRelativity::default();
        sr.set("absolute_time", KnobValue::Bool(true)).unwrap();
        assert_eq!(kind(&sr, SR_INVARIANT_INTERVAL), VerdictKind::Fails);
        assert_eq!(kind(&sr, SR_SUBLUMINAL_COMPOSITION), VerdictKind::Fails);
        assert_eq!(kind(&sr, SR_ENERGY_MOMENTUM), VerdictKind::Fails);
        assert_eq!(
            kind(&sr, SR_CROSS_PRODUCT_JACOBI),
            VerdictKind::Holds,
            "Jacobi is not a boost identity"
        );
        assert_eq!(
            kind(&sr, SR_LAGRANGE_IDENTITY),
            VerdictKind::Holds,
            "Lagrange is not a boost identity"
        );
        assert_eq!(
            kind(&sr, SR_MATRIX_DET_PRODUCT),
            VerdictKind::Holds,
            "det-product is not a boost identity"
        );
    }

    #[test]
    fn mass_shell_depends_on_the_interval() {
        let sr = SpecialRelativity::default();
        let em = sr
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_ENERGY_MOMENTUM)
            .unwrap();
        assert_eq!(em.depends_on[0].0, SR_INVARIANT_INTERVAL);
    }

    #[test]
    fn jacobi_is_not_an_interval_lemma() {
        let sr = SpecialRelativity::default();
        let j = sr
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_CROSS_PRODUCT_JACOBI)
            .unwrap();
        assert!(j.depends_on.is_empty(), "Jacobi is not a boost lemma");
    }

    #[test]
    fn lagrange_is_not_an_interval_lemma() {
        let sr = SpecialRelativity::default();
        let l = sr
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_LAGRANGE_IDENTITY)
            .unwrap();
        assert!(l.depends_on.is_empty(), "Lagrange is not a boost lemma");
    }

    #[test]
    fn det_product_is_not_an_interval_lemma() {
        let sr = SpecialRelativity::default();
        let d = sr
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_MATRIX_DET_PRODUCT)
            .unwrap();
        assert!(d.depends_on.is_empty(), "det-product is not a boost lemma");
    }

    #[test]
    fn velocity_composition_numbers_are_right() {
        let sr = SpecialRelativity::default();
        // 0.8c ⊕ 0.7c = 1.5 / 1.56 ≈ 0.9615c.
        assert!((sr.compose_speeds(0.8, 0.7) - 1.5 / 1.56).abs() < 1e-12);
        // Galilean would give 1.5c, superluminal.
        let mut g = SpecialRelativity::default();
        g.set("absolute_time", KnobValue::Bool(true)).unwrap();
        assert!((g.compose_speeds(0.8, 0.7) - 1.5).abs() < 1e-12);
    }

    #[test]
    fn interval_invariance_is_exact_under_lorentz() {
        let sr = SpecialRelativity::default();
        let c = C.value();
        let (ct0, x0) = (c * 1.0e-8, 2.0);
        let (ct1, x1) = sr.boost(ct0, x0, BETA);
        let s0 = ct0 * ct0 - x0 * x0;
        let s1 = ct1 * ct1 - x1 * x1;
        assert_eq!(
            classify_boost_residual(relative_residual_hull(s0, s1)),
            ResidualRelation::Contained
        );
    }

    #[test]
    fn binomial_gamma_is_ir_not_a_knob() {
        let t = SpecialRelativity::default();
        assert!(
            SpecialRelativity::default()
                .set("binomial_gamma", KnobValue::Bool(true))
                .is_err(),
            "binomial γ is an IR mutation, not a knob"
        );
        assert!(SpecialRelativity::default()
            .set("gamma", KnobValue::Bool(true))
            .is_err());
        assert_eq!(
            t.get("absolute_time").unwrap(),
            KnobValue::Bool(false),
            "absolute_time stays a knob"
        );
        let src = render_package(&t.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            SpecialRelativity::from_package(&pkg).unwrap(),
            t,
            "IR round-trip must preserve the Lorentz boost"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(SpecialRelativity::binomial_equation()),
        );
        let parsed = SpecialRelativity::from_package(&mutated).unwrap();
        assert!(parsed.binomial_gamma);
        assert!(
            !parsed.minus_uv,
            "binomial mutation must not install minus-uv"
        );
        let mut fork = t.clone();
        fork.binomial_gamma = true;
        assert_eq!(fork.id(), "special-relativity");
        assert_eq!(kind(&fork, SR_INVARIANT_INTERVAL), VerdictKind::Fails);
        assert_eq!(kind(&fork, SR_ENERGY_MOMENTUM), VerdictKind::Fails);
        assert_eq!(
            kind(&fork, SR_SUBLUMINAL_COMPOSITION),
            VerdictKind::Holds,
            "binomial γ is not the Galilean composition fork"
        );
        assert_eq!(
            kind(&fork, SR_CROSS_PRODUCT_JACOBI),
            VerdictKind::Holds,
            "binomial γ is not the Jacobi identity"
        );
        assert_eq!(
            kind(&fork, SR_LAGRANGE_IDENTITY),
            VerdictKind::Holds,
            "binomial γ is not the Lagrange identity"
        );
        assert_eq!(
            kind(&fork, SR_MATRIX_DET_PRODUCT),
            VerdictKind::Holds,
            "binomial γ is not the det-product identity"
        );
        assert_eq!(kind(&t, SR_INVARIANT_INTERVAL), VerdictKind::Holds);
        let interval = fork
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_INVARIANT_INTERVAL)
            .unwrap();
        let v = fork.evaluate(&interval);
        assert!(
            !v.summary.contains("Galilean"),
            "binomial γ is not the absolute_time knob: {}",
            v.summary
        );
        let residual = binomial_gamma_residual(BETA);
        assert!(
            residual.abs() > 0.05,
            "residual must be the γ mismatch, not a unit flag, got {residual}"
        );
        assert!(
            v.evidence.iter().any(|e| e.contains("γ_L − γ_bin")),
            "got {:?}",
            v.evidence
        );
        let c = C.value();
        let (ct0, x0) = (c * 1.0e-8, 2.0);
        let (ct1, x1) = fork.boost(ct0, x0, BETA_VANISHING);
        let s0 = ct0 * ct0 - x0 * x0;
        let s1 = ct1 * ct1 - x1 * x1;
        assert_eq!(
            classify_boost_residual(relative_residual_hull(s0, s1)),
            ResidualRelation::Contained,
            "tiny β makes the sample look Lorentzian; residual is not the encoding"
        );
        assert_eq!(
            kind(&fork, SR_INVARIANT_INTERVAL),
            VerdictKind::Fails,
            "binomial encoding must fail the interval even when a tiny-β sample looks invariant"
        );
        let mut galilean = SpecialRelativity::default();
        galilean
            .set("absolute_time", KnobValue::Bool(true))
            .unwrap();
        assert_eq!(kind(&galilean, SR_INVARIANT_INTERVAL), VerdictKind::Fails);
        assert_eq!(
            kind(&galilean, SR_SUBLUMINAL_COMPOSITION),
            VerdictKind::Fails
        );
        let probes = SpecialRelativity::default().structural_mutations();
        assert!(
            probes
                .iter()
                .any(|(label, _)| label == "add-binomial-gamma"),
            "live SR must offer add-binomial-gamma: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let probe = probes
            .iter()
            .find(|(label, _)| label == "add-binomial-gamma")
            .expect("add-binomial-gamma");
        assert_eq!(
            kind(probe.1.as_ref(), SR_INVARIANT_INTERVAL),
            VerdictKind::Fails
        );
        let fork_probes = fork.structural_mutations();
        assert!(
            fork_probes
                .iter()
                .all(|(label, _)| label != "add-binomial-gamma"),
            "binomial fork must not re-offer add-binomial-gamma"
        );
        assert!(
            fork_probes.iter().any(|(label, _)| label == "add-minus-uv"),
            "binomial fork must still offer add-minus-uv"
        );
        let live = SpecialRelativity::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let mut abs = SpecialRelativity::default();
        abs.set("absolute_time", KnobValue::Bool(true)).unwrap();
        let rebuilt = abs.reparse_package(&parsed).unwrap();
        assert_eq!(
            rebuilt.get("absolute_time").unwrap(),
            KnobValue::Bool(true),
            "reparse must overlay binomial IR onto live knobs"
        );
        assert_eq!(
            kind(rebuilt.as_ref(), SR_INVARIANT_INTERVAL),
            VerdictKind::Fails,
            "absolute_time still Fails interval on the live Lorentz encoding"
        );
        let live_rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(
            kind(live_rebuilt.as_ref(), SR_INVARIANT_INTERVAL),
            VerdictKind::Holds
        );
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_INVARIANT_INTERVAL)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "interval must keep the catalog Minkowski domain: {:?}",
            cell.domain()
        );
        assert!(
            crate::relativity::GeneralRelativity::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-binomial-gamma"),
            "general-relativity must not grow add-binomial-gamma"
        );
        assert!(
            crate::computation::TuringMachine::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-binomial-gamma"),
            "turing-machine must not grow add-binomial-gamma"
        );
    }

    #[test]
    fn minus_uv_composition_is_ir_not_a_knob() {
        let t = SpecialRelativity::default();
        assert!(
            SpecialRelativity::default()
                .set("minus_uv", KnobValue::Bool(true))
                .is_err(),
            "minus-uv composition is an IR mutation, not a knob"
        );
        assert!(SpecialRelativity::default()
            .set("compose", KnobValue::Bool(true))
            .is_err());
        assert!(
            SpecialRelativity::default()
                .set("add-minus-uv", KnobValue::Bool(true))
                .is_err(),
            "add-minus-uv is not a knob"
        );
        assert_eq!(
            t.get("absolute_time").unwrap(),
            KnobValue::Bool(false),
            "absolute_time stays a knob"
        );
        let src = render_package(&t.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            pkg.equations.len(),
            7,
            "live package must stay boost lorentz plus the catalog identity trees"
        );
        assert_eq!(pkg.equations[0], BOOST_LORENTZ);
        assert_eq!(pkg.equations[1], INTERVAL_EQ);
        assert_eq!(pkg.equations[2], COMPOSITION_EQ);
        assert_eq!(pkg.equations[3], MASS_SHELL_EQ);
        assert_eq!(pkg.equations[4], JACOBI_EQ);
        assert_eq!(pkg.equations[5], LAGRANGE_EQ);
        assert_eq!(pkg.equations[6], DET_PRODUCT_EQ);
        assert_eq!(
            pkg.lean_ref.as_deref(),
            Some(lookup(SR_INVARIANT_INTERVAL).unwrap().lean_type)
        );
        assert_eq!(
            SpecialRelativity::from_package(&pkg).unwrap(),
            t,
            "IR round-trip must preserve the Lorentz boost"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(SpecialRelativity::minus_uv_equation()),
        );
        let parsed = SpecialRelativity::from_package(&mutated).unwrap();
        assert!(parsed.minus_uv);
        assert!(
            !parsed.binomial_gamma,
            "minus-uv mutation must not install binomial γ"
        );
        let mut fork = t.clone();
        fork.minus_uv = true;
        assert_eq!(fork.id(), "special-relativity");
        assert_eq!(kind(&fork, SR_SUBLUMINAL_COMPOSITION), VerdictKind::Fails);
        assert_eq!(
            kind(&fork, SR_INVARIANT_INTERVAL),
            VerdictKind::Holds,
            "minus-uv is not the binomial γ interval fork"
        );
        assert_eq!(
            kind(&fork, SR_ENERGY_MOMENTUM),
            VerdictKind::Holds,
            "minus-uv is not the binomial γ mass-shell fork"
        );
        assert_eq!(
            kind(&fork, SR_CROSS_PRODUCT_JACOBI),
            VerdictKind::Holds,
            "minus-uv is not the Jacobi identity"
        );
        assert_eq!(
            kind(&fork, SR_LAGRANGE_IDENTITY),
            VerdictKind::Holds,
            "minus-uv is not the Lagrange identity"
        );
        assert_eq!(
            kind(&fork, SR_MATRIX_DET_PRODUCT),
            VerdictKind::Holds,
            "minus-uv is not the det-product identity"
        );
        assert_eq!(kind(&t, SR_SUBLUMINAL_COMPOSITION), VerdictKind::Holds);
        let composition = fork
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_SUBLUMINAL_COMPOSITION)
            .unwrap();
        let v = fork.evaluate(&composition);
        assert!(
            !v.summary.contains("Galilean"),
            "minus-uv is not the absolute_time knob: {}",
            v.summary
        );
        assert!(
            !v.summary.contains("kind")
                && !v.summary.contains("total_dim")
                && !v.summary.contains("supersymmetry"),
            "fail summary must not name string knobs: {}",
            v.summary
        );
        let residual = minus_uv_residual(COMPOSE_U, COMPOSE_V);
        assert!(
            residual.abs() > 0.5,
            "residual must be the composition mismatch, not a unit flag, got {residual}"
        );
        assert!(
            v.evidence.iter().any(|e| e.contains("(u+v)/(1-uv)")),
            "got {:?}",
            v.evidence
        );
        let w_tiny = fork.compose_speeds(1.0e-6, 1.0e-6);
        assert!(
            w_tiny < 1.0,
            "tiny speeds stay subluminal under minus-uv, got {w_tiny}"
        );
        assert_eq!(
            kind(&fork, SR_SUBLUMINAL_COMPOSITION),
            VerdictKind::Fails,
            "minus-uv encoding must fail composition even when a tiny-speed sample looks subluminal"
        );
        assert!(
            (fork.compose_speeds(COMPOSE_U, COMPOSE_V)
                - (COMPOSE_U + COMPOSE_V) / (1.0 - COMPOSE_U * COMPOSE_V))
                .abs()
                < 1e-12,
            "minus-uv must be (u+v)/(1-uv), not Galilean u+v"
        );
        let mut galilean = SpecialRelativity::default();
        galilean
            .set("absolute_time", KnobValue::Bool(true))
            .unwrap();
        assert_eq!(
            kind(&galilean, SR_SUBLUMINAL_COMPOSITION),
            VerdictKind::Fails
        );
        assert_eq!(kind(&galilean, SR_INVARIANT_INTERVAL), VerdictKind::Fails);
        let probes = SpecialRelativity::default().structural_mutations();
        assert!(
            probes.iter().any(|(label, _)| label == "add-minus-uv"),
            "live SR must offer add-minus-uv: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        assert!(
            probes
                .iter()
                .any(|(label, _)| label == "add-binomial-gamma"),
            "live SR must still offer add-binomial-gamma"
        );
        let probe = probes
            .iter()
            .find(|(label, _)| label == "add-minus-uv")
            .expect("add-minus-uv");
        assert_eq!(
            kind(probe.1.as_ref(), SR_SUBLUMINAL_COMPOSITION),
            VerdictKind::Fails
        );
        assert_eq!(
            kind(probe.1.as_ref(), SR_INVARIANT_INTERVAL),
            VerdictKind::Holds
        );
        let fork_probes = fork.structural_mutations();
        assert!(
            fork_probes.iter().all(|(label, _)| label != "add-minus-uv"),
            "minus-uv fork must not re-offer add-minus-uv"
        );
        assert!(
            fork_probes
                .iter()
                .any(|(label, _)| label == "add-binomial-gamma"),
            "minus-uv fork must still offer add-binomial-gamma"
        );
        let live = SpecialRelativity::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed_live = parse_package(&canonical).unwrap();
        let mut abs = SpecialRelativity::default();
        abs.set("absolute_time", KnobValue::Bool(true)).unwrap();
        let rebuilt = abs.reparse_package(&parsed_live).unwrap();
        assert_eq!(
            rebuilt.get("absolute_time").unwrap(),
            KnobValue::Bool(true),
            "reparse must overlay minus-uv IR onto live knobs"
        );
        assert_eq!(
            kind(rebuilt.as_ref(), SR_SUBLUMINAL_COMPOSITION),
            VerdictKind::Fails,
            "absolute_time still Fails composition on the live Lorentz encoding"
        );
        let live_rebuilt = live.reparse_package(&parsed_live).unwrap();
        assert_eq!(
            kind(live_rebuilt.as_ref(), SR_SUBLUMINAL_COMPOSITION),
            VerdictKind::Holds
        );
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_SUBLUMINAL_COMPOSITION)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "composition must keep the catalog collinear domain: {:?}",
            cell.domain()
        );
        assert!(
            crate::relativity::GeneralRelativity::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-minus-uv"),
            "general-relativity must not grow add-minus-uv"
        );
        assert!(
            crate::computation::TuringMachine::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-minus-uv"),
            "turing-machine must not grow add-minus-uv"
        );
        assert!(
            crate::blackbody::Blackbody::planck()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-minus-uv"),
            "planck must not grow add-minus-uv"
        );
        assert!(
            crate::strings::StringTheory::type_i()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-minus-uv"),
            "type-i must not grow add-minus-uv"
        );
    }

    fn evidence_cites_listing(evidence: &[String], listing: &physis_constants::ConstantListing) {
        assert!(
            evidence.iter().any(|line| {
                line.contains(&listing.name)
                    && line.contains(listing.kind)
                    && line.contains(&listing.hash.to_hex())
                    && line.contains(&listing.value)
            }),
            "evidence must cite {} kind {} hash {} display {}: {:?}",
            listing.name,
            listing.kind,
            listing.hash.to_hex(),
            listing.value,
            evidence
        );
    }

    fn live_me_centre(hull: physis_numeric::SciInterval) -> SciExact {
        assert_eq!(
            hull.lo.exp10, hull.hi.exp10,
            "live m_e hull endpoints must share exp10"
        );
        SciExact::new(
            (hull.lo.significand + hull.hi.significand) / 2,
            hull.lo.exp10,
        )
    }

    #[test]
    fn mass_shell_cites_versioned_c_and_m_e() {
        let sr = SpecialRelativity::default();
        let claim = sr
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_ENERGY_MOMENTUM)
            .unwrap();
        let v = sr.evaluate(&claim);
        let c_listing = physis_constants::lookup("c").expect("c is on LEDGER");
        let me_listing = physis_constants::lookup("m_e").expect("m_e is on LEDGER");
        assert_eq!(c_listing.kind, "ratio");
        assert_eq!(me_listing.kind, "sci-interval");
        evidence_cites_listing(&v.evidence, &c_listing);
        evidence_cites_listing(&v.evidence, &me_listing);
        assert_eq!(v.derivation(), DerivationAssurance::Executed);
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_ne!(v.derivation(), DerivationAssurance::CertifiedNumeric);
        assert_eq!(
            physis_constants::speed_of_light().value.to_f64(),
            physis_model::constants::C.value(),
            "ledger c Ratio to_f64 locksteps model C"
        );
        let me_centre = live_me_centre(physis_constants::electron_mass().value);
        assert_eq!(
            me_centre.to_f64(),
            physis_model::constants::electron_mass().value(),
            "SciInterval midpoint locksteps model electron_mass Qty"
        );

        let mut galilean = SpecialRelativity::default();
        galilean
            .set("absolute_time", KnobValue::Bool(true))
            .unwrap();
        assert_eq!(kind(&galilean, SR_INVARIANT_INTERVAL), VerdictKind::Fails);
        assert_eq!(
            kind(&galilean, SR_SUBLUMINAL_COMPOSITION),
            VerdictKind::Fails
        );
        let gal_claim = galilean
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_ENERGY_MOMENTUM)
            .unwrap();
        let gal = galilean.evaluate(&gal_claim);
        assert_eq!(gal.kind, VerdictKind::Fails);
        assert_eq!(gal.derivation(), DerivationAssurance::Executed);
        evidence_cites_listing(&gal.evidence, &c_listing);
        evidence_cites_listing(&gal.evidence, &me_listing);
        assert_eq!(kind(&galilean, SR_CROSS_PRODUCT_JACOBI), VerdictKind::Holds);
        assert_eq!(kind(&galilean, SR_LAGRANGE_IDENTITY), VerdictKind::Holds);
        assert_eq!(kind(&galilean, SR_MATRIX_DET_PRODUCT), VerdictKind::Holds);

        let binomial = SpecialRelativity {
            binomial_gamma: true,
            ..SpecialRelativity::default()
        };
        let bin_claim = binomial
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_ENERGY_MOMENTUM)
            .unwrap();
        let bin = binomial.evaluate(&bin_claim);
        assert_eq!(bin.kind, VerdictKind::Fails);
        assert_eq!(bin.derivation(), DerivationAssurance::Executed);
        evidence_cites_listing(&bin.evidence, &c_listing);
        evidence_cites_listing(&bin.evidence, &me_listing);
    }

    #[test]
    fn lorentz_interval_and_mass_shell_samples_name_domain_and_error_sources() {
        let sr = SpecialRelativity::default();
        let interval = verdict(&sr, SR_INVARIANT_INTERVAL);
        assert_eq!(interval.kind, VerdictKind::Holds);
        assert_eq!(interval.derivation(), DerivationAssurance::Executed);
        assert_ne!(
            interval.derivation(),
            DerivationAssurance::CertifiedNumeric,
            "float sample is not P3N"
        );
        let ib = evidence_blob(&interval);
        assert_numeric_sample_honesty(&ib);
        assert!(
            ib.contains("10 ns") && ib.contains("2 m"),
            "interval sample event: {ib}"
        );

        let mass = verdict(&sr, SR_ENERGY_MOMENTUM);
        assert_eq!(mass.kind, VerdictKind::Holds);
        assert_eq!(mass.derivation(), DerivationAssurance::Executed);
        assert_ne!(mass.derivation(), DerivationAssurance::CertifiedNumeric);
        let mb = evidence_blob(&mass);
        assert_numeric_sample_honesty(&mb);
        assert!(mb.contains("rest electron"), "{mb}");
        assert!(
            mb.contains("input identity") && mb.contains("m_e"),
            "m_e SciInterval is input identity, not this rounding bound: {mb}"
        );
        let c_listing = physis_constants::lookup("c").expect("c is on LEDGER");
        let me_listing = physis_constants::lookup("m_e").expect("m_e is on LEDGER");
        evidence_cites_listing(&mass.evidence, &c_listing);
        evidence_cites_listing(&mass.evidence, &me_listing);

        let pkg = sr.package();
        let bound = physis_proof::catalog_tree_binding(pkg.lean_ref.as_deref(), &pkg.equations)
            .unwrap()
            .expect("live SR must bind the interval tree");
        assert_eq!(bound.claim_id, SR_INVARIANT_INTERVAL);
        assert_eq!(pkg.equations[1], INTERVAL_EQ);
        assert_eq!(pkg.equations[3], MASS_SHELL_EQ);
        // Float Holds is not a kernel proof: Jacobi still uses identity_is_zero.
        let jacobi = verdict(&sr, SR_CROSS_PRODUCT_JACOBI);
        assert_eq!(jacobi.kind, VerdictKind::Holds);
        assert!(
            evidence_blob(&jacobi).contains("expanders agree"),
            "Jacobi still dual-expands: {:?}",
            jacobi.evidence
        );
        assert!(evidence_blob(&verdict(&sr, SR_LAGRANGE_IDENTITY)).contains("expanders agree"));
        assert!(evidence_blob(&verdict(&sr, SR_MATRIX_DET_PRODUCT)).contains("expanders agree"));
    }

    #[test]
    fn galilean_boost_sample_fails_with_numeric_honesty() {
        let mut galilean = SpecialRelativity::default();
        galilean
            .set("absolute_time", KnobValue::Bool(true))
            .unwrap();
        let interval = verdict(&galilean, SR_INVARIANT_INTERVAL);
        assert_eq!(interval.kind, VerdictKind::Fails);
        assert_eq!(interval.derivation(), DerivationAssurance::Executed);
        assert_numeric_sample_honesty(&evidence_blob(&interval));
        assert_eq!(
            kind(&galilean, SR_SUBLUMINAL_COMPOSITION),
            VerdictKind::Fails
        );
        let mass = verdict(&galilean, SR_ENERGY_MOMENTUM);
        assert_eq!(mass.kind, VerdictKind::Fails);
        assert_numeric_sample_honesty(&evidence_blob(&mass));
        assert_eq!(kind(&galilean, SR_CROSS_PRODUCT_JACOBI), VerdictKind::Holds);
        assert_eq!(kind(&galilean, SR_LAGRANGE_IDENTITY), VerdictKind::Holds);
        assert_eq!(kind(&galilean, SR_MATRIX_DET_PRODUCT), VerdictKind::Holds);
    }

    #[test]
    fn boost_residual_straddle_is_undecidable_not_a_theorem() {
        // Residual twice as wide as the 64-ulp band 2^{-46}: overlap without
        // containment. Not equality.
        let residual = Interval::new(Ratio::new(-1, 1i128 << 45), Ratio::new(1, 1i128 << 45));
        let band = sample_rounding_band();
        assert_eq!(
            residual_relation(residual, band),
            ResidualRelation::OverlapsWithoutContainment
        );
        assert_eq!(
            classify_boost_residual(residual),
            ResidualRelation::OverlapsWithoutContainment
        );
        let claim = SpecialRelativity::default()
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_INVARIANT_INTERVAL)
            .unwrap();
        let v = decide_boost_sample(
            &claim,
            residual,
            "s² is unchanged by the Lorentz boost",
            "the interval is not invariant under a Galilean boost",
            vec!["synthetic straddle".into()],
        );
        assert_eq!(v.kind, VerdictKind::Undecidable);
        assert_eq!(v.empirical(), EmpiricalStatus::Inconclusive);
        assert_eq!(v.derivation(), DerivationAssurance::Executed);
        assert_ne!(v.derivation(), DerivationAssurance::CertifiedNumeric);
        let j = physis_core::judgment::Judgment::from_lab(
            v.class,
            v.kind,
            v.empirical(),
            v.derivation(),
            false,
            v.numeric_lo(),
            v.numeric_hi(),
            v.statistical_nll(),
        );
        assert_eq!(j.label(), "numeric unresolved");
    }
}
