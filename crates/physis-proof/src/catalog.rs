//! Trusted identities the exact-certificate backend is allowed to check.
//!
//! These expressions are part of the challenge, not inputs from a solver.
//! Mutating a sign produces a different challenge hash and, for these
//! particular identities, a non-zero polynomial that both checkers reject.

use physis_core::assumption::{Assumption, AssumptionSet, DomainOfValidity};
use physis_core::assurance::ClaimClass;
use physis_core::claim::Claim;
use physis_core::formal::{ClaimCommitments, FormalClaim};
use physis_core::id::LayerId;
use physis_core::{AxiomId, AxiomLedger};

use crate::expr::{add, mul, pow, sub, Expr};
use crate::parse::parse_expr;

/// Catalog row: the FormalClaim identity this polynomial / Lean type is
/// about, plus the trusted obligation. A matching slug with different
/// commitments is a different sentence and is not this row.
#[derive(Clone, Copy, Debug)]
pub struct IdentitySpec {
    /// Lab claim id.
    pub claim_id: &'static str,
    /// English statement the catalog identity is about.
    pub statement: &'static str,
    /// Claim class.
    pub class: ClaimClass,
    /// Layer.
    pub layer: LayerId,
    /// First-class identity fields committed in the statement hash.
    pub commitments: fn() -> ClaimCommitments,
    /// Named regime. Encoding-wide is not a catalog identity.
    pub domain: fn() -> DomainOfValidity,
    /// Lean theorem name in [`crate::PHYSLIB_SOURCE`].
    pub lean_theorem: &'static str,
    /// Lean-shaped theorem type the obligation corresponds to.
    pub lean_type: &'static str,
    /// Axiom ids listed on the receipt.
    pub axioms: &'static [&'static str],
    /// Builder for the trusted identity.
    pub identity: fn() -> Expr,
}

impl IdentitySpec {
    /// Lab claim for this catalog row. Theories that host the identity
    /// should use this so the live `statement_hash` is the catalog hash
    /// (commitments, named domain, and catalog axioms — not encoding-wide
    /// Physlib forall with only `encoding-is-the-model`).
    pub fn lab_claim(&self) -> Claim {
        Claim::new(self.claim_id, self.statement, self.layer, self.class)
            .with_assumptions(self.assumption_set())
            .with_commitments((self.commitments)())
            .with_domain((self.domain)())
    }

    /// Encoding-internal default plus this row's catalog axioms.
    /// Lean kernel axioms live on the receipt, not here.
    pub fn assumption_set(&self) -> AssumptionSet {
        assumption_set_for(self.axioms)
    }

    /// Formal identity the exact / Lean backends may prove.
    pub fn formal_claim(&self) -> FormalClaim {
        FormalClaim::from_claim(&self.lab_claim())
    }

    /// True when `claim` is this catalog identity, not merely the same slug.
    pub fn matches(&self, claim: &FormalClaim) -> bool {
        claim.statement_hash() == self.formal_claim().statement_hash()
    }
}

fn assumption_set_for(axioms: &[&str]) -> AssumptionSet {
    let ledger = AxiomLedger::physis_defaults();
    let mut items = AssumptionSet::encoding_internal().items;
    for id in axioms {
        let rec = ledger.get(&AxiomId::new(*id)).unwrap_or_else(|| {
            panic!("catalog axiom {id} must be on AxiomLedger::physis_defaults")
        });
        items.push(Assumption {
            id: rec.id.0.clone(),
            statement: rec.provenance.clone(),
            class: rec.class,
        });
    }
    AssumptionSet::new(items)
}

fn physlib_d2_commitments() -> ClaimCommitments {
    ClaimCommitments::physlib_forall()
}

fn minkowski_interval_commitments() -> ClaimCommitments {
    let mut c = ClaimCommitments::physlib_forall();
    c.constants = vec!["c=1".into()];
    c.conventions = vec!["minkowski-mostly-minus".into()];
    c
}

fn einstein_composition_commitments() -> ClaimCommitments {
    let mut c = ClaimCommitments::physlib_forall();
    c.constants = vec!["c=1".into()];
    c
}

fn d2_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["oriented 2-simplex coboundary over Z".into()],
        vec!["discrete exterior calculus encoding".into()],
        "The catalog identity is (b-a)-(c-a)+(c-b)=0 on vertex values. \
         It is not de Rham cohomology of a smooth manifold. Using it \
         outside that encoding is a new claim.",
    )
}

fn tet_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["oriented 3-simplex coboundary over Z".into()],
        vec!["discrete exterior calculus encoding".into()],
        "The catalog identity is d2 of d1 on the six edge values of one \
         tetrahedron. It is not the triangle 0-form identity and not de \
         Rham cohomology of a smooth 3-manifold. Using it outside that \
         encoding is a new claim. The live 2-complex is not this cell.",
    )
}

fn interval_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["1+1 Minkowski".into(), "c = 1".into(), "|β| < 1".into()],
        vec!["special relativity (no gravity)".into()],
        "The catalog identity is the polynomial boost of s². |β|<1 over ℝ \
         remains the evaluator; the polynomial holds as an integer identity. \
         Using it in curved spacetime is a new claim.",
    )
}

fn composition_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["c = 1".into(), "|u| < 1".into(), "|v| < 1".into()],
        vec!["collinear Einstein velocity addition".into()],
        "The catalog identity is (1+uv)²-(u+v)²-(1-u²)(1-v²)=0. |w|<1 over ℝ \
         remains the evaluator. Using it for non-collinear boosts is a new claim.",
    )
}

fn mass_shell_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["1+1 Minkowski".into(), "c = 1".into(), "|β| < 1".into()],
        vec![
            "special relativity (no gravity)".into(),
            "on-shell 4-momentum".into(),
        ],
        "The catalog identity is the interval polynomial on (E, p), not a new \
         postulate. The typed rest-mass check remains the evaluator.",
    )
}

fn jacobi_commitments() -> ClaimCommitments {
    ClaimCommitments::physlib_forall()
}

fn jacobi_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["integer cross-product Jacobi in R^3".into()],
        vec!["so(3) Lie bracket as the R^3 cross product".into()],
        "The catalog identity is the x-component of a cross (b cross c) plus \
         cyclic, as an integer polynomial. It is not the Minkowski interval \
         and not a boost identity. Using it for so(1,3) structure constants \
         is a new claim.",
    )
}

fn lagrange_commitments() -> ClaimCommitments {
    ClaimCommitments::physlib_forall()
}

fn lagrange_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["integer Lagrange identity in R^3".into()],
        vec!["Euclidean cross and dot product on Z^3".into()],
        "The catalog identity is |a cross b|^2 + (a dot b)^2 - |a|^2 |b|^2 \
         as a degree-4 integer polynomial. It is not the Jacobi identity of \
         nested crosses and not the Minkowski interval. Using it for a \
         Lorentzian inner product is a new claim.",
    )
}

fn det_product_commitments() -> ClaimCommitments {
    ClaimCommitments::physlib_forall()
}

fn det_product_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["integer 2x2 determinant product".into()],
        vec!["unimodular matrix multiplication on M_2(Z)".into()],
        "The catalog identity is det(AB) - det(A) det(B) as a degree-4 \
         integer polynomial on eight matrix entries. It is not the Jacobi \
         identity of nested crosses, not the Lagrange identity of cross \
         and dot, and not the Minkowski interval. Using a 1+1 boost matrix \
         with gamma is a new claim.",
    )
}

/// Discrete exterior calculus: `(b − a) − (c − a) + (c − b) ≡ 0`.
///
/// That is `d₁(d₀ f)` on a single oriented triangle, for a 0-form with
/// values `a,b,c` at the vertices. It is the algebraic content of `d² = 0`.
pub fn discrete_d2() -> Expr {
    let a = Expr::var("a");
    let b = Expr::var("b");
    let c = Expr::var("c");
    // (b - a) - (c - a) + (c - b)
    add(sub(sub(b.clone(), a.clone()), sub(c.clone(), a)), sub(c, b))
}

/// Discrete exterior calculus: `d₂ ∘ d₁ = 0` on one oriented 3-simplex.
///
/// Edge values `ab, ac, ad, bc, bd, cd`. Face coboundaries are
/// `ω_bc − ω_ac + ω_ab` (and the three faces that include `d`). `d₂` of
/// those four 2-cochain values is identically zero. Not a rename of
/// [`discrete_d2`]: different grade, different indeterminates.
pub fn tetrahedron_d2() -> Expr {
    let ab = Expr::var("ab");
    let ac = Expr::var("ac");
    let ad = Expr::var("ad");
    let bc = Expr::var("bc");
    let bd = Expr::var("bd");
    let cd = Expr::var("cd");
    let face_abc = add(sub(bc.clone(), ac.clone()), ab.clone());
    let face_abd = add(sub(bd.clone(), ad.clone()), ab);
    let face_acd = add(sub(cd.clone(), ad), ac);
    let face_bcd = add(sub(cd, bd), bc);
    sub(add(sub(face_bcd, face_acd), face_abd), face_abc)
}

/// Lorentz interval identity (c = 1):
/// `(t − βx)² − (x − βt)² − (1 − β²)(t² − x²) ≡ 0`.
///
/// Multiplying the boosted interval by `1/γ² = 1 − β²` recovers the
/// original interval. The identity is polynomial; `γ` never appears.
pub fn lorentz_interval() -> Expr {
    let t = Expr::var("t");
    let x = Expr::var("x");
    let b = Expr::var("beta");
    let boosted = sub(
        pow(sub(t.clone(), mul(b.clone(), x.clone())), 2),
        pow(sub(x.clone(), mul(b.clone(), t.clone())), 2),
    );
    let orig = mul(sub(Expr::c(1), pow(b, 2)), sub(pow(t, 2), pow(x, 2)));
    sub(boosted, orig)
}

/// Einstein velocity addition: `(1 + uv)² − (u + v)² − (1 − u²)(1 − v²) ≡ 0`.
///
/// If `w = (u + v) / (1 + uv)` then `1 − w²` is proportional to
/// `(1 − u²)(1 − v²)`. The catalog checks the polynomial; `|w| < 1` over
/// the reals remains the evaluator.
pub fn einstein_composition() -> Expr {
    let u = Expr::var("u");
    let v = Expr::var("v");
    let left = sub(
        pow(add(Expr::c(1), mul(u.clone(), v.clone())), 2),
        pow(add(u.clone(), v.clone()), 2),
    );
    let right = mul(sub(Expr::c(1), pow(u, 2)), sub(Expr::c(1), pow(v, 2)));
    sub(left, right)
}

/// Mass-shell identity (c = 1):
/// `(E − βp)² − (p − βE)² − (1 − β²)(E² − p²) ≡ 0`.
///
/// This is the Minkowski bilinear form on 4-momentum: the same algebraic
/// obligation as [`lorentz_interval`] with `(t, x) → (E, p)`. It is not a
/// new physical postulate; the axioms stay `integer-arithmetic` and
/// `minkowski-interval-signature`. The lab still treats the claims as
/// distinct (spacetime interval vs particle mass shell). The typed
/// rest-mass check `E² − (pc)² = (mc²)²` remains the evaluator.
pub fn energy_momentum() -> Expr {
    let e = Expr::var("E");
    let p = Expr::var("p");
    let b = Expr::var("beta");
    let boosted = sub(
        pow(sub(e.clone(), mul(b.clone(), p.clone())), 2),
        pow(sub(p.clone(), mul(b.clone(), e.clone())), 2),
    );
    let orig = mul(sub(Expr::c(1), pow(b, 2)), sub(pow(e, 2), pow(p, 2)));
    sub(boosted, orig)
}

/// Jacobi identity for the R^3 cross product: the x-component of
/// `a × (b × c) + b × (c × a) + c × (a × b)` is identically zero.
///
/// Different algebraic idea from coboundary nilpotence and from the
/// Minkowski bilinear form. y- and z-components are the same identity
/// on cyclic relabeling and are not extra catalog rows.
pub fn cross_product_jacobi() -> Expr {
    let a1 = Expr::var("a1");
    let a2 = Expr::var("a2");
    let a3 = Expr::var("a3");
    let b1 = Expr::var("b1");
    let b2 = Expr::var("b2");
    let b3 = Expr::var("b3");
    let c1 = Expr::var("c1");
    let c2 = Expr::var("c2");
    let c3 = Expr::var("c3");
    let bxc_y = sub(mul(b3.clone(), c1.clone()), mul(b1.clone(), c3.clone()));
    let bxc_z = sub(mul(b1.clone(), c2.clone()), mul(b2.clone(), c1.clone()));
    let a_term = sub(mul(a2.clone(), bxc_z), mul(a3.clone(), bxc_y));
    let cxa_y = sub(mul(c3.clone(), a1.clone()), mul(c1.clone(), a3.clone()));
    let cxa_z = sub(mul(c1.clone(), a2.clone()), mul(c2.clone(), a1.clone()));
    let b_term = sub(mul(b2.clone(), cxa_z), mul(b3.clone(), cxa_y));
    let axb_y = sub(mul(a3, b1.clone()), mul(a1.clone(), b3));
    let axb_z = sub(mul(a1, b2), mul(a2, b1));
    let c_term = sub(mul(c2, axb_z), mul(c3, axb_y));
    add(add(a_term, b_term), c_term)
}

/// Lagrange identity: `|a × b|² + (a · b)² − |a|² |b|² ≡ 0` over Z^3.
///
/// Degree 4 Euclidean relation between the cross product and the dot
/// product. Not the degree-3 Jacobi identity of nested crosses, not
/// coboundary nilpotence, and not the Minkowski bilinear form.
pub fn lagrange_identity() -> Expr {
    let a1 = Expr::var("a1");
    let a2 = Expr::var("a2");
    let a3 = Expr::var("a3");
    let b1 = Expr::var("b1");
    let b2 = Expr::var("b2");
    let b3 = Expr::var("b3");
    let cx = sub(mul(a2.clone(), b3.clone()), mul(a3.clone(), b2.clone()));
    let cy = sub(mul(a3.clone(), b1.clone()), mul(a1.clone(), b3.clone()));
    let cz = sub(mul(a1.clone(), b2.clone()), mul(a2.clone(), b1.clone()));
    let cross2 = add(add(pow(cx, 2), pow(cy, 2)), pow(cz, 2));
    let dot = add(
        add(mul(a1.clone(), b1.clone()), mul(a2.clone(), b2.clone())),
        mul(a3.clone(), b3.clone()),
    );
    let na2 = add(add(pow(a1, 2), pow(a2, 2)), pow(a3, 2));
    let nb2 = add(add(pow(b1, 2), pow(b2, 2)), pow(b3, 2));
    sub(add(cross2, pow(dot, 2)), mul(na2, nb2))
}

/// 2×2 determinant multiplicativity: `det(AB) − det(A) det(B) ≡ 0` over `M_2(Z)`.
///
/// Degree 4 relation on eight integer matrix entries. Not Jacobi of nested
/// crosses, not Lagrange of Euclidean cross and dot, and not Minkowski.
pub fn matrix_det_product() -> Expr {
    let a11 = Expr::var("a11");
    let a12 = Expr::var("a12");
    let a21 = Expr::var("a21");
    let a22 = Expr::var("a22");
    let b11 = Expr::var("b11");
    let b12 = Expr::var("b12");
    let b21 = Expr::var("b21");
    let b22 = Expr::var("b22");
    let det_a = sub(mul(a11.clone(), a22.clone()), mul(a12.clone(), a21.clone()));
    let det_b = sub(mul(b11.clone(), b22.clone()), mul(b12.clone(), b21.clone()));
    let ab11 = add(mul(a11.clone(), b11.clone()), mul(a12.clone(), b21.clone()));
    let ab12 = add(mul(a11.clone(), b12.clone()), mul(a12.clone(), b22.clone()));
    let ab21 = add(mul(a21.clone(), b11), mul(a22.clone(), b21));
    let ab22 = add(mul(a21, b12), mul(a22, b22));
    let det_ab = sub(mul(ab11, ab22), mul(ab12, ab21));
    sub(det_ab, mul(det_a, det_b))
}

/// Known exact identities. A claim not in this list cannot be promoted by
/// the exact-certificate backend.
pub const CATALOG: &[IdentitySpec] = &[
    IdentitySpec {
        claim_id: "dec.d-squared-zero",
        statement: "The exterior derivative is nilpotent: d ∘ d = 0.",
        class: ClaimClass::Mathematical,
        layer: LayerId::Mathematical,
        commitments: physlib_d2_commitments,
        domain: d2_domain,
        lean_theorem: "d_squared_zero",
        lean_type: "∀ (a b c : Int), (b - a) - (c - a) + (c - b) = 0",
        axioms: &["integer-arithmetic", "discrete-coboundary"],
        identity: discrete_d2,
    },
    IdentitySpec {
        claim_id: "dec.d-squared-one",
        statement: "The coboundary of a 1-cochain is closed: d ∘ d = 0 on an oriented 3-simplex.",
        class: ClaimClass::Mathematical,
        layer: LayerId::Mathematical,
        commitments: physlib_d2_commitments,
        domain: tet_domain,
        lean_theorem: "d_squared_one",
        lean_type: "∀ (ab ac ad bc bd cd : Int), (((((cd - bd) + bc) - ((cd - ad) + ac)) + ((bd - ad) + ab)) - ((bc - ac) + ab)) = 0",
        axioms: &["integer-arithmetic", "discrete-coboundary"],
        identity: tetrahedron_d2,
    },
    IdentitySpec {
        claim_id: "sr.invariant-interval",
        statement: "The spacetime interval s² = (cΔt)² − Δx² is invariant under a boost.",
        class: ClaimClass::ModelInternal,
        layer: LayerId::Spacetime,
        commitments: minkowski_interval_commitments,
        domain: interval_domain,
        lean_theorem: "invariant_interval",
        lean_type: "∀ (t x β : Int), (t - β*x)^2 - (x - β*t)^2 = (1 - β^2)*(t^2 - x^2)",
        axioms: &["integer-arithmetic", "minkowski-interval-signature"],
        identity: lorentz_interval,
    },
    IdentitySpec {
        claim_id: "sr.subluminal-composition",
        statement: "Composing two subluminal velocities stays below c.",
        class: ClaimClass::ModelInternal,
        layer: LayerId::Spacetime,
        commitments: einstein_composition_commitments,
        domain: composition_domain,
        lean_theorem: "subluminal_composition",
        lean_type: "∀ (u v : Int), (1 + u*v)^2 - (u + v)^2 = (1 - u^2)*(1 - v^2)",
        axioms: &["integer-arithmetic", "einstein-velocity-addition"],
        identity: einstein_composition,
    },
    IdentitySpec {
        claim_id: "sr.energy-momentum-invariant",
        statement: "The mass shell E² − (pc)² = (mc²)² is frame-independent.",
        class: ClaimClass::ModelInternal,
        layer: LayerId::Particle,
        commitments: minkowski_interval_commitments,
        domain: mass_shell_domain,
        lean_theorem: "energy_momentum_invariant",
        lean_type: "∀ (E p β : Int), (E - β*p)^2 - (p - β*E)^2 = (1 - β^2)*(E^2 - p^2)",
        axioms: &["integer-arithmetic", "minkowski-interval-signature"],
        identity: energy_momentum,
    },
    IdentitySpec {
        claim_id: "sr.cross-product-jacobi",
        statement: "The x-component of a × (b × c) + cyclic vanishes.",
        class: ClaimClass::Mathematical,
        layer: LayerId::Mathematical,
        commitments: jacobi_commitments,
        domain: jacobi_domain,
        lean_theorem: "cross_product_jacobi",
        lean_type: "∀ (a1 a2 a3 b1 b2 b3 c1 c2 c3 : Int), ((((a2 * ((b1 * c2) - (b2 * c1))) - (a3 * ((b3 * c1) - (b1 * c3)))) + ((b2 * ((c1 * a2) - (c2 * a1))) - (b3 * ((c3 * a1) - (c1 * a3))))) + ((c2 * ((a1 * b2) - (a2 * b1))) - (c3 * ((a3 * b1) - (a1 * b3))))) = 0",
        axioms: &["integer-arithmetic"],
        identity: cross_product_jacobi,
    },
    IdentitySpec {
        claim_id: "sr.lagrange-identity",
        statement: "The Lagrange identity |a × b|² + (a · b)² = |a|² |b|² holds over Z^3.",
        class: ClaimClass::Mathematical,
        layer: LayerId::Mathematical,
        commitments: lagrange_commitments,
        domain: lagrange_domain,
        lean_theorem: "lagrange_identity",
        lean_type: "∀ (a1 a2 a3 b1 b2 b3 : Int), (((((((a2 * b3) - (a3 * b2)))^2 + (((a3 * b1) - (a1 * b3)))^2) + (((a1 * b2) - (a2 * b1)))^2) + ((((a1 * b1) + (a2 * b2)) + (a3 * b3)))^2) - ((((a1)^2 + (a2)^2) + (a3)^2) * (((b1)^2 + (b2)^2) + (b3)^2))) = 0",
        axioms: &["integer-arithmetic"],
        identity: lagrange_identity,
    },
    IdentitySpec {
        claim_id: "sr.matrix-det-product",
        statement: "det(AB) equals det(A) det(B) for 2×2 integer matrices.",
        class: ClaimClass::Mathematical,
        layer: LayerId::Mathematical,
        commitments: det_product_commitments,
        domain: det_product_domain,
        lean_theorem: "matrix_det_product",
        lean_type: "∀ (a11 a12 a21 a22 b11 b12 b21 b22 : Int), (((((a11 * b11) + (a12 * b21)) * ((a21 * b12) + (a22 * b22))) - (((a11 * b12) + (a12 * b22)) * ((a21 * b11) + (a22 * b21)))) - (((a11 * a22) - (a12 * a21)) * ((b11 * b22) - (b12 * b21)))) = 0",
        axioms: &["integer-arithmetic"],
        identity: matrix_det_product,
    },
];

/// Lookup by claim slug. Not a catalog proof: a changed FormalClaim
/// identity keeps the slug and must use [`lookup_matching`].
pub fn lookup(claim_id: &str) -> Option<&'static IdentitySpec> {
    CATALOG.iter().find(|s| s.claim_id == claim_id)
}

/// Catalog row whose FormalClaim identity is `claim`. Same slug with
/// different commitments is not a hit.
pub fn lookup_matching(claim: &FormalClaim) -> Option<&'static IdentitySpec> {
    CATALOG.iter().find(|s| s.matches(claim))
}

/// Bind a live IR `lean_ref` to a catalog identity tree.
///
/// Token packages omit `lean_ref` and skip (`Ok(None)`). A catalog Lean
/// type must appear as an equation whose canonical tree is that row's
/// identity. Extra unparsed token equations are ignored when one equation
/// matches. A Physlib pointer that is not a catalog type, or a catalog
/// type without the tree, fails closed. This is not a kernel proof.
pub fn catalog_tree_binding(
    lean_ref: Option<&str>,
    equations: &[impl AsRef<str>],
) -> Result<Option<&'static IdentitySpec>, String> {
    let Some(lean_ref) = lean_ref else {
        return Ok(None);
    };
    let spec = CATALOG
        .iter()
        .find(|s| s.lean_type == lean_ref)
        .ok_or_else(|| "lean_ref is not a catalog identity type".to_string())?;
    let catalog = (spec.identity)().canonical();
    let matched = equations.iter().any(|eq| {
        parse_expr(eq.as_ref())
            .ok()
            .is_some_and(|e| e.canonical() == catalog)
    });
    if matched {
        Ok(Some(spec))
    } else {
        Err("lean_ref names a catalog identity whose tree is not in the equations".into())
    }
}

/// Catalog identities whose trees appear in `equations`.
///
/// Token equations that do not parse are skipped. Order follows
/// [`CATALOG`]. This is not a kernel proof.
pub fn catalog_trees_in(equations: &[impl AsRef<str>]) -> Vec<&'static IdentitySpec> {
    let parsed: Vec<String> = equations
        .iter()
        .filter_map(|eq| parse_expr(eq.as_ref()).ok().map(|e| e.canonical()))
        .collect();
    CATALOG
        .iter()
        .filter(|spec| {
            let catalog = (spec.identity)().canonical();
            parsed.iter().any(|t| t == &catalog)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_covers_the_vertical_slice() {
        assert!(lookup("dec.d-squared-zero").is_some());
        assert!(lookup("dec.d-squared-one").is_some());
        assert!(lookup("sr.invariant-interval").is_some());
        assert!(lookup("sr.subluminal-composition").is_some());
        assert!(lookup("sr.energy-momentum-invariant").is_some());
        assert!(lookup("sr.cross-product-jacobi").is_some());
        assert!(lookup("sr.lagrange-identity").is_some());
        assert!(lookup("sr.matrix-det-product").is_some());
        assert!(lookup("predictivity.unique-vacuum").is_none());
    }

    #[test]
    fn tetrahedron_is_not_the_triangle_challenge() {
        assert_ne!(tetrahedron_d2().canonical(), discrete_d2().canonical());
        let spec = lookup("dec.d-squared-one").unwrap();
        assert_eq!(spec.axioms, lookup("dec.d-squared-zero").unwrap().axioms);
        assert_ne!(
            spec.formal_claim().statement_hash(),
            lookup("dec.d-squared-zero")
                .unwrap()
                .formal_claim()
                .statement_hash()
        );
    }

    #[test]
    fn mass_shell_is_not_the_interval_challenge() {
        // Same bilinear form, different indeterminates: a rename is a
        // different challenge hash, not a silent alias.
        assert_ne!(
            energy_momentum().canonical(),
            lorentz_interval().canonical()
        );
        let spec = lookup("sr.energy-momentum-invariant").unwrap();
        assert_eq!(spec.axioms, lookup("sr.invariant-interval").unwrap().axioms);
    }

    #[test]
    fn unspecified_slug_is_not_the_catalog_identity() {
        let spec = lookup("dec.d-squared-zero").unwrap();
        let unspecified = FormalClaim::from_claim(&Claim::new(
            spec.claim_id,
            spec.statement,
            spec.layer,
            spec.class,
        ));
        assert!(!spec.matches(&unspecified));
        assert!(lookup_matching(&unspecified).is_none());
        assert!(lookup_matching(&spec.formal_claim()).is_some());
    }

    #[test]
    fn catalog_identities_are_not_encoding_wide() {
        for spec in CATALOG {
            let live = spec.lab_claim();
            assert!(
                !live.domain().is_encoding_wide(),
                "{} must name a regime, not encoding-wide",
                spec.claim_id
            );
            assert!(!live.domain().regimes.is_empty(), "{}", spec.claim_id);
            let wide = Claim::new(spec.claim_id, spec.statement, spec.layer, spec.class)
                .with_commitments((spec.commitments)());
            assert!(wide.domain().is_encoding_wide());
            assert!(
                !spec.matches(&FormalClaim::from_claim(&wide)),
                "physlib forall with the encoding-wide placeholder is not {}",
                spec.claim_id
            );
        }
    }

    #[test]
    fn catalog_assumptions_are_identity() {
        for spec in CATALOG {
            let live = spec.lab_claim();
            assert!(
                live.assumptions()
                    .items
                    .iter()
                    .any(|a| a.id == "encoding-is-the-model"),
                "{}",
                spec.claim_id
            );
            for ax in spec.axioms {
                assert!(
                    live.assumptions().items.iter().any(|a| a.id == *ax),
                    "{} missing assumption {ax}",
                    spec.claim_id
                );
            }
            let without = Claim::new(spec.claim_id, spec.statement, spec.layer, spec.class)
                .with_commitments((spec.commitments)())
                .with_domain((spec.domain)());
            assert!(
                !spec.matches(&FormalClaim::from_claim(&without)),
                "named domain without catalog axioms is not {}",
                spec.claim_id
            );
        }
    }

    #[test]
    fn catalog_tree_binds_coboundary() {
        let spec = lookup("dec.d-squared-zero").unwrap();
        let bound = catalog_tree_binding(Some(spec.lean_type), &["(b - a) - (c - a) + (c - b)"])
            .unwrap()
            .expect("coboundary tree must bind");
        assert_eq!(bound.claim_id, spec.claim_id);
        let with_token = catalog_tree_binding(
            Some(spec.lean_type),
            &["(b - a) - (c - a) + (c - b)", "laplacian down"],
        )
        .unwrap()
        .expect("token equations must not block a matching tree");
        assert_eq!(with_token.claim_id, spec.claim_id);
    }

    #[test]
    fn catalog_tree_binds_tetrahedron_beside_triangle() {
        let tet = lookup("dec.d-squared-one").unwrap();
        let tet_eq = tetrahedron_d2().to_string();
        let eqs = [
            "(b - a) - (c - a) + (c - b)",
            tet_eq.as_str(),
            "laplacian down",
        ];
        let bound = catalog_tree_binding(Some(tet.lean_type), &eqs)
            .unwrap()
            .expect("3-simplex tree must bind beside the triangle");
        assert_eq!(bound.claim_id, tet.claim_id);
        let triangle = lookup("dec.d-squared-zero").unwrap();
        let bound_t = catalog_tree_binding(Some(triangle.lean_type), &eqs)
            .unwrap()
            .expect("triangle tree must still bind among the coboundary identities");
        assert_eq!(bound_t.claim_id, triangle.claim_id);
        let listed: Vec<_> = catalog_trees_in(&eqs)
            .into_iter()
            .map(|s| s.claim_id)
            .collect();
        assert_eq!(listed, vec!["dec.d-squared-zero", "dec.d-squared-one"]);
    }

    #[test]
    fn catalog_tree_binds_interval_beside_lorentz_token() {
        let interval = lookup("sr.invariant-interval").unwrap();
        let bound = catalog_tree_binding(
            Some(interval.lean_type),
            &[
                "boost lorentz",
                "(t - beta * x)^2 - (x - beta * t)^2 - (1 - beta^2) * (t^2 - x^2)",
            ],
        )
        .unwrap()
        .expect("interval tree must bind beside the Lorentz token");
        assert_eq!(bound.claim_id, interval.claim_id);
        let token_only =
            catalog_tree_binding(Some(interval.lean_type), &["boost lorentz"]).unwrap_err();
        assert!(
            token_only.contains("catalog identity whose tree is not in the equations"),
            "{token_only}"
        );
    }

    #[test]
    fn catalog_tree_binds_composition_and_mass_shell() {
        let eqs = [
            "boost lorentz",
            "(t - beta * x)^2 - (x - beta * t)^2 - (1 - beta^2) * (t^2 - x^2)",
            "(1 + u * v)^2 - (u + v)^2 - (1 - u^2) * (1 - v^2)",
            "(E - beta * p)^2 - (p - beta * E)^2 - (1 - beta^2) * (E^2 - p^2)",
        ];
        let composition = lookup("sr.subluminal-composition").unwrap();
        let bound_c = catalog_tree_binding(Some(composition.lean_type), &eqs)
            .unwrap()
            .expect("composition tree must bind beside the Lorentz token");
        assert_eq!(bound_c.claim_id, composition.claim_id);
        let mass_shell = lookup("sr.energy-momentum-invariant").unwrap();
        let bound_m = catalog_tree_binding(Some(mass_shell.lean_type), &eqs)
            .unwrap()
            .expect("mass-shell tree must bind beside the Lorentz token");
        assert_eq!(bound_m.claim_id, mass_shell.claim_id);
        let interval = lookup("sr.invariant-interval").unwrap();
        let bound_i = catalog_tree_binding(Some(interval.lean_type), &eqs)
            .unwrap()
            .expect("interval tree must still bind among the three identities");
        assert_eq!(bound_i.claim_id, interval.claim_id);
        let listed: Vec<_> = catalog_trees_in(&eqs)
            .into_iter()
            .map(|s| s.claim_id)
            .collect();
        assert_eq!(
            listed,
            vec![
                "sr.invariant-interval",
                "sr.subluminal-composition",
                "sr.energy-momentum-invariant",
            ]
        );
        assert!(catalog_trees_in(&["boost lorentz"]).is_empty());
    }

    #[test]
    fn jacobi_is_not_the_interval_challenge() {
        assert_ne!(
            cross_product_jacobi().canonical(),
            lorentz_interval().canonical()
        );
        assert_ne!(
            cross_product_jacobi().canonical(),
            energy_momentum().canonical()
        );
        assert_ne!(
            cross_product_jacobi().canonical(),
            tetrahedron_d2().canonical()
        );
        let spec = lookup("sr.cross-product-jacobi").unwrap();
        assert_eq!(spec.axioms, &["integer-arithmetic"]);
        assert_ne!(
            spec.formal_claim().statement_hash(),
            lookup("sr.invariant-interval")
                .unwrap()
                .formal_claim()
                .statement_hash()
        );
        let jac_eq = cross_product_jacobi().to_string();
        let eqs = [
            "boost lorentz",
            "(t - beta * x)^2 - (x - beta * t)^2 - (1 - beta^2) * (t^2 - x^2)",
            "(1 + u * v)^2 - (u + v)^2 - (1 - u^2) * (1 - v^2)",
            "(E - beta * p)^2 - (p - beta * E)^2 - (1 - beta^2) * (E^2 - p^2)",
            jac_eq.as_str(),
        ];
        let bound = catalog_tree_binding(Some(spec.lean_type), &eqs)
            .unwrap()
            .expect("Jacobi tree must bind beside the SR trees");
        assert_eq!(bound.claim_id, spec.claim_id);
        let listed: Vec<_> = catalog_trees_in(&eqs)
            .into_iter()
            .map(|s| s.claim_id)
            .collect();
        assert_eq!(
            listed,
            vec![
                "sr.invariant-interval",
                "sr.subluminal-composition",
                "sr.energy-momentum-invariant",
                "sr.cross-product-jacobi",
            ]
        );
    }

    #[test]
    fn lagrange_is_not_the_jacobi_or_interval_challenge() {
        assert_ne!(
            lagrange_identity().canonical(),
            cross_product_jacobi().canonical()
        );
        assert_ne!(
            lagrange_identity().canonical(),
            lorentz_interval().canonical()
        );
        assert_ne!(
            lagrange_identity().canonical(),
            energy_momentum().canonical()
        );
        assert_ne!(
            lagrange_identity().canonical(),
            tetrahedron_d2().canonical()
        );
        let spec = lookup("sr.lagrange-identity").unwrap();
        assert_eq!(spec.axioms, &["integer-arithmetic"]);
        assert_eq!(
            lagrange_identity().to_string(),
            "(((((((a2 * b3) - (a3 * b2)))^2 + (((a3 * b1) - (a1 * b3)))^2) + (((a1 * b2) - (a2 * b1)))^2) + ((((a1 * b1) + (a2 * b2)) + (a3 * b3)))^2) - ((((a1)^2 + (a2)^2) + (a3)^2) * (((b1)^2 + (b2)^2) + (b3)^2)))"
        );
        assert_ne!(
            spec.formal_claim().statement_hash(),
            lookup("sr.cross-product-jacobi")
                .unwrap()
                .formal_claim()
                .statement_hash()
        );
        let jac_eq = cross_product_jacobi().to_string();
        let lag_eq = lagrange_identity().to_string();
        let eqs = [
            "boost lorentz",
            "(t - beta * x)^2 - (x - beta * t)^2 - (1 - beta^2) * (t^2 - x^2)",
            "(1 + u * v)^2 - (u + v)^2 - (1 - u^2) * (1 - v^2)",
            "(E - beta * p)^2 - (p - beta * E)^2 - (1 - beta^2) * (E^2 - p^2)",
            jac_eq.as_str(),
            lag_eq.as_str(),
        ];
        let bound = catalog_tree_binding(Some(spec.lean_type), &eqs)
            .unwrap()
            .expect("Lagrange tree must bind beside the SR trees");
        assert_eq!(bound.claim_id, spec.claim_id);
        let listed: Vec<_> = catalog_trees_in(&eqs)
            .into_iter()
            .map(|s| s.claim_id)
            .collect();
        assert_eq!(
            listed,
            vec![
                "sr.invariant-interval",
                "sr.subluminal-composition",
                "sr.energy-momentum-invariant",
                "sr.cross-product-jacobi",
                "sr.lagrange-identity",
            ]
        );
    }

    #[test]
    fn det_product_is_not_the_lagrange_or_jacobi_or_interval_challenge() {
        assert_ne!(
            matrix_det_product().canonical(),
            lagrange_identity().canonical()
        );
        assert_ne!(
            matrix_det_product().canonical(),
            cross_product_jacobi().canonical()
        );
        assert_ne!(
            matrix_det_product().canonical(),
            lorentz_interval().canonical()
        );
        assert_ne!(
            matrix_det_product().canonical(),
            energy_momentum().canonical()
        );
        assert_ne!(
            matrix_det_product().canonical(),
            tetrahedron_d2().canonical()
        );
        let spec = lookup("sr.matrix-det-product").unwrap();
        assert_eq!(spec.axioms, &["integer-arithmetic"]);
        assert_eq!(
            matrix_det_product().to_string(),
            "(((((a11 * b11) + (a12 * b21)) * ((a21 * b12) + (a22 * b22))) - (((a11 * b12) + (a12 * b22)) * ((a21 * b11) + (a22 * b21)))) - (((a11 * a22) - (a12 * a21)) * ((b11 * b22) - (b12 * b21))))"
        );
        assert_ne!(
            spec.formal_claim().statement_hash(),
            lookup("sr.lagrange-identity")
                .unwrap()
                .formal_claim()
                .statement_hash()
        );
        let jac_eq = cross_product_jacobi().to_string();
        let lag_eq = lagrange_identity().to_string();
        let det_eq = matrix_det_product().to_string();
        let eqs = [
            "boost lorentz",
            "(t - beta * x)^2 - (x - beta * t)^2 - (1 - beta^2) * (t^2 - x^2)",
            "(1 + u * v)^2 - (u + v)^2 - (1 - u^2) * (1 - v^2)",
            "(E - beta * p)^2 - (p - beta * E)^2 - (1 - beta^2) * (E^2 - p^2)",
            jac_eq.as_str(),
            lag_eq.as_str(),
            det_eq.as_str(),
        ];
        let bound = catalog_tree_binding(Some(spec.lean_type), &eqs)
            .unwrap()
            .expect("det-product tree must bind beside the SR trees");
        assert_eq!(bound.claim_id, spec.claim_id);
        let listed: Vec<_> = catalog_trees_in(&eqs)
            .into_iter()
            .map(|s| s.claim_id)
            .collect();
        assert_eq!(
            listed,
            vec![
                "sr.invariant-interval",
                "sr.subluminal-composition",
                "sr.energy-momentum-invariant",
                "sr.cross-product-jacobi",
                "sr.lagrange-identity",
                "sr.matrix-det-product",
            ]
        );
    }

    #[test]
    fn catalog_tree_skips_missing_lean_ref() {
        assert!(catalog_tree_binding(None, &["(b - a) - (c - a) + (c - b)"])
            .unwrap()
            .is_none());
        assert!(catalog_tree_binding(None, &["boost lorentz"])
            .unwrap()
            .is_none());
    }

    #[test]
    fn catalog_lean_ref_without_tree_is_closed() {
        let spec = lookup("dec.d-squared-zero").unwrap();
        let vacuous = catalog_tree_binding(Some(spec.lean_type), &["0"]).unwrap_err();
        assert!(
            vacuous.contains("catalog identity whose tree is not in the equations"),
            "{vacuous}"
        );
        assert!(!vacuous.contains("receipt"), "{vacuous}");
        let flipped = catalog_tree_binding(Some(spec.lean_type), &["(b + a) - (c - a) + (c - b)"])
            .unwrap_err();
        assert!(
            flipped.contains("catalog identity whose tree is not in the equations"),
            "{flipped}"
        );
        let interval = lookup("sr.invariant-interval").unwrap();
        let wrong_tree =
            catalog_tree_binding(Some(interval.lean_type), &["(b - a) - (c - a) + (c - b)"])
                .unwrap_err();
        assert!(
            wrong_tree.contains("catalog identity whose tree is not in the equations"),
            "{wrong_tree}"
        );
    }

    #[test]
    fn unknown_lean_ref_is_closed() {
        let err = catalog_tree_binding(
            Some("Physlib.Exterior.d_squared"),
            &["(b - a) - (c - a) + (c - b)"],
        )
        .unwrap_err();
        assert!(
            err.contains("lean_ref is not a catalog identity type"),
            "{err}"
        );
        assert!(!err.contains("receipt"), "{err}");
        assert!(!err.contains("theorem"), "{err}");
    }
}
