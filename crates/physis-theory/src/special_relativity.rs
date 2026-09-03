//! Special relativity as mechanized kinematics — and a knob that turns it off.
//!
//! Einstein's 1905 kinematics is not asserted here; it is *computed*. Four
//! identities are checked:
//!
//! - the spacetime interval `s² = (cΔt)² − Δx²` is unchanged by a boost,
//! - composing two subluminal velocities stays subluminal,
//! - the energy–momentum invariant `E² − (pc)² = (mc²)²` is frame-independent,
//!   built from *typed* quantities so `pc` and `mc²` are forced to be energies,
//! - and the integer cross-product Jacobi identity (so(3) Lie bracket) holds
//!   as a catalog polynomial, independent of the boost encoding.
//!
//! The Lorentz boost and the catalog interval, composition, mass-shell,
//! and cross-product Jacobi identity trees live on the IR package. A truncated
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
use physis_core::ParameterOrigin;
use physis_core::{Energy, Momentum, Qty};
use physis_ir::{apply_mutation, parse_package, render_package, PackageMutation, TheoryPackage};
use physis_model::constants::{electron_mass, C};
use physis_model::{GaugeGroup, Manifold, Spectrum, World};
use physis_proof::catalog::{
    cross_product_jacobi, einstein_composition, energy_momentum, lorentz_interval,
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
    let mut lorentz = false;
    let mut binomial = false;
    let mut minus_uv = false;
    let mut interval_tree = false;
    let mut composition_tree = false;
    let mut mass_shell_tree = false;
    let mut jacobi_tree = false;
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
    Ok((binomial, minus_uv))
}

fn gamma_lorentz(beta: f64) -> f64 {
    1.0 / (1.0 - beta * beta).sqrt()
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
/// and cross-product Jacobi identity trees live on the IR package. Truncated binomial γ
/// (`add-binomial-gamma`) is a package mutation, not a knob: interval
/// and mass-shell fail. Minus-uv collinear composition (`add-minus-uv`)
/// is a second package mutation, not a knob: subluminal composition
/// fails while Lorentz boosts still hold. The Jacobi identity stays Holds
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
    /// the catalog interval, composition, mass-shell, and cross-product
    /// Jacobi identity trees, and, when forked, `boost binomial-gamma`
    /// and/or `compose minus-uv`. `absolute_time` stays on the struct.
    /// `lean_ref` is the catalog interval type, not a Physlib pointer
    /// without the tree.
    pub fn package(&self) -> TheoryPackage {
        let interval = lookup(SR_INVARIANT_INTERVAL).expect("interval is a catalog identity");
        let composition =
            lookup(SR_SUBLUMINAL_COMPOSITION).expect("composition is a catalog identity");
        let mass_shell = lookup(SR_ENERGY_MOMENTUM).expect("mass shell is a catalog identity");
        let jacobi = lookup(SR_CROSS_PRODUCT_JACOBI).expect("Jacobi is a catalog identity");
        let mut equations = vec![
            BOOST_LORENTZ.to_string(),
            INTERVAL_EQ.to_string(),
            COMPOSITION_EQ.to_string(),
            MASS_SHELL_EQ.to_string(),
            JACOBI_EQ.to_string(),
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
         composition, the mass shell E² = (pc)² + (mc²)², and the integer \
         cross-product Jacobi identity, all computed. The Lorentz boost is an IR \
         encoding. Truncated binomial γ is an IR mutation, not the absolute_time \
         knob. Minus-uv collinear composition is a second IR mutation, not that \
         knob. That knob still replaces exact Lorentz with Galilean boosts. The \
         Jacobi identity is independent of those encodings."
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
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        let c = C.value();
        match claim.id_str() {
            SR_INVARIANT_INTERVAL => {
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
                    let invariant = (s0 - s1).abs() <= 1e-9 * s0.abs();
                    if invariant {
                        Verdict::holds(claim, "s² is unchanged by the Lorentz boost").with_evidence(
                            [format!("s² = {s0:.4e} m² before and {s1:.4e} m² after")],
                        )
                    } else {
                        Verdict::fails(
                            claim,
                            "the interval is not invariant under a Galilean boost",
                        )
                        .with_evidence([format!("s² = {s0:.4e} m² → {s1:.4e} m² (changed)")])
                    }
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
                let m = electron_mass();
                let mc2: Qty<Energy> = m * C * C;
                let (e1, pc1) = self.boost(mc2.value(), 0.0, BETA);
                let shell = e1 * e1 - pc1 * pc1;
                let rest = mc2.value() * mc2.value();
                let p1: Qty<Momentum> = Qty::new(pc1 / c);
                if self.binomial_gamma {
                    let residual = binomial_gamma_residual(BETA);
                    Verdict::fails(
                        claim,
                        "binomial γ: the mass shell is not Lorentz-invariant",
                    )
                    .with_evidence([format!(
                        "γ_L − γ_bin = {residual:.6} at β = {BETA}; E² − (pc)² = {shell:.4e} J² vs (mc²)² = {rest:.4e} J²"
                    )])
                } else {
                    let invariant = (shell - rest).abs() <= 1e-9 * rest.abs();
                    if invariant {
                        Verdict::holds(claim, "E² − (pc)² equals (mc²)² in the boosted frame")
                            .with_evidence([
                                format!(
                                    "mc² = {:.4e} J, boosted |p| = {:.4e} kg·m/s",
                                    mc2.value(),
                                    p1.value()
                                ),
                                format!("E² − (pc)² = {shell:.4e} J² vs (mc²)² = {rest:.4e} J²"),
                            ])
                    } else {
                        Verdict::fails(claim, "the mass shell is not preserved by a Galilean boost")
                            .with_evidence([format!(
                                "E² − (pc)² = {shell:.4e} J² ≠ (mc²)² = {rest:.4e} J²"
                            )])
                    }
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

    fn kind(t: &dyn Theory, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id_str() == id).unwrap();
        t.evaluate(&c).kind
    }

    #[test]
    fn lorentz_kinematics_holds_all_invariants() {
        let sr = SpecialRelativity::default();
        assert!(!sr.absolute_time);
        assert_eq!(kind(&sr, SR_INVARIANT_INTERVAL), VerdictKind::Holds);
        assert_eq!(kind(&sr, SR_SUBLUMINAL_COMPOSITION), VerdictKind::Holds);
        assert_eq!(kind(&sr, SR_ENERGY_MOMENTUM), VerdictKind::Holds);
        assert_eq!(kind(&sr, SR_CROSS_PRODUCT_JACOBI), VerdictKind::Holds);
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
        assert_eq!(pkg.claims.len(), 4);
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
        assert!((s0 - s1).abs() <= 1e-9 * s0.abs());
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
        assert!(
            (s0 - s1).abs() <= 1e-9 * s0.abs(),
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
            5,
            "live package must stay boost lorentz plus the catalog identity trees"
        );
        assert_eq!(pkg.equations[0], BOOST_LORENTZ);
        assert_eq!(pkg.equations[1], INTERVAL_EQ);
        assert_eq!(pkg.equations[2], COMPOSITION_EQ);
        assert_eq!(pkg.equations[3], MASS_SHELL_EQ);
        assert_eq!(pkg.equations[4], JACOBI_EQ);
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
}
