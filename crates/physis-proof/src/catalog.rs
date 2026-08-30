//! Trusted identities the exact-certificate backend is allowed to check.
//!
//! These expressions are part of the challenge, not inputs from a solver.
//! Mutating a sign produces a different challenge hash and, for these
//! particular identities, a non-zero polynomial that both checkers reject.

use physis_core::assurance::ClaimClass;
use physis_core::claim::Claim;
use physis_core::formal::{ClaimCommitments, FormalClaim};
use physis_core::id::LayerId;

use crate::expr::{add, mul, pow, sub, Expr};

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
    /// should use this so the live `statement_hash` is the catalog hash.
    pub fn lab_claim(&self) -> Claim {
        Claim::new(self.claim_id, self.statement, self.layer, self.class)
            .with_commitments((self.commitments)())
    }

    /// Formal identity the exact / Lean backends may prove.
    pub fn formal_claim(&self) -> FormalClaim {
        FormalClaim::from_claim(&self.lab_claim())
    }

    /// True when `claim` is this catalog identity, not merely the same slug.
    pub fn matches(&self, claim: &FormalClaim) -> bool {
        claim.statement_hash == self.formal_claim().statement_hash
    }
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

/// Known exact identities. A claim not in this list cannot be promoted by
/// the exact-certificate backend.
pub const CATALOG: &[IdentitySpec] = &[
    IdentitySpec {
        claim_id: "dec.d-squared-zero",
        statement: "The exterior derivative is nilpotent: d ∘ d = 0.",
        class: ClaimClass::Mathematical,
        layer: LayerId::Mathematical,
        commitments: physlib_d2_commitments,
        lean_theorem: "d_squared_zero",
        lean_type: "∀ (a b c : Int), (b - a) - (c - a) + (c - b) = 0",
        axioms: &["integer-arithmetic", "discrete-coboundary"],
        identity: discrete_d2,
    },
    IdentitySpec {
        claim_id: "sr.invariant-interval",
        statement: "The spacetime interval s² = (cΔt)² − Δx² is invariant under a boost.",
        class: ClaimClass::ModelInternal,
        layer: LayerId::Spacetime,
        commitments: minkowski_interval_commitments,
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
        lean_theorem: "energy_momentum_invariant",
        lean_type: "∀ (E p β : Int), (E - β*p)^2 - (p - β*E)^2 = (1 - β^2)*(E^2 - p^2)",
        axioms: &["integer-arithmetic", "minkowski-interval-signature"],
        identity: energy_momentum,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_covers_the_vertical_slice() {
        assert!(lookup("dec.d-squared-zero").is_some());
        assert!(lookup("sr.invariant-interval").is_some());
        assert!(lookup("sr.subluminal-composition").is_some());
        assert!(lookup("sr.energy-momentum-invariant").is_some());
        assert!(lookup("predictivity.unique-vacuum").is_none());
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
}
