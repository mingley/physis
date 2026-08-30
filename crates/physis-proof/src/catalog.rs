//! Trusted identities the exact-certificate backend is allowed to check.
//!
//! These expressions are part of the challenge, not inputs from a solver.
//! Mutating a sign produces a different challenge hash and, for these
//! particular identities, a non-zero polynomial that both checkers reject.

use crate::expr::{add, mul, pow, sub, Expr};

/// Catalog row: claim id, Lean-facing type (documentation of the obligation),
/// the polynomial that must be identically zero, and axiom ids in the closure.
#[derive(Clone, Copy, Debug)]
pub struct IdentitySpec {
    /// Lab claim id.
    pub claim_id: &'static str,
    /// Lean theorem name in [`PHYSLIB_SOURCE`].
    pub lean_theorem: &'static str,
    /// Lean-shaped theorem type the obligation corresponds to.
    pub lean_type: &'static str,
    /// Axiom ids listed on the receipt.
    pub axioms: &'static [&'static str],
    /// Builder for the trusted identity.
    pub identity: fn() -> Expr,
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

/// Known exact identities. A claim not in this list cannot be promoted by
/// the exact-certificate backend.
pub const CATALOG: &[IdentitySpec] = &[
    IdentitySpec {
        claim_id: "dec.d-squared-zero",
        lean_theorem: "d_squared_zero",
        lean_type: "∀ (a b c : Int), (b - a) - (c - a) + (c - b) = 0",
        axioms: &["integer-arithmetic", "discrete-coboundary"],
        identity: discrete_d2,
    },
    IdentitySpec {
        claim_id: "sr.invariant-interval",
        lean_theorem: "invariant_interval",
        lean_type: "∀ (t x β : Int), (t - β*x)^2 - (x - β*t)^2 = (1 - β^2)*(t^2 - x^2)",
        axioms: &["integer-arithmetic", "minkowski-interval-signature"],
        identity: lorentz_interval,
    },
];

/// Lookup by claim id.
pub fn lookup(claim_id: &str) -> Option<&'static IdentitySpec> {
    CATALOG.iter().find(|s| s.claim_id == claim_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_covers_the_vertical_slice() {
        assert!(lookup("dec.d-squared-zero").is_some());
        assert!(lookup("sr.invariant-interval").is_some());
        assert!(lookup("predictivity.unique-vacuum").is_none());
    }
}
