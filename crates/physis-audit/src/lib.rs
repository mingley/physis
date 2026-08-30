//! Red-team corpus: intentionally wrong encodings that must not promote.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use physis_core::formal::FormalClaim;
use physis_proof::expr::{add, mul, pow, sub, Expr};
use physis_proof::{identity_is_zero, lookup, Challenge, UntrustedProof};
use physis_verifier::{verify, VerifyError};

/// A named corruption and whether verify/identity must reject it.
pub struct Mutation {
    /// Short name.
    pub name: &'static str,
    /// How to attack.
    pub kind: MutationKind,
}

/// Kind of attack.
pub enum MutationKind {
    /// A polynomial that must *not* be identically zero.
    NonZeroIdentity(fn() -> Expr),
    /// Lean source that must be rejected as unauthorized.
    HostileLean(&'static str),
}

/// The standing corpus (spec §46, first slice).
pub fn corpus() -> Vec<Mutation> {
    vec![
        Mutation {
            name: "wrong Maxwell / coboundary sign",
            kind: MutationKind::NonZeroIdentity(|| {
                let a = Expr::var("a");
                let b = Expr::var("b");
                let c = Expr::var("c");
                // last plus flipped
                sub(sub(sub(b.clone(), a.clone()), sub(c.clone(), a)), sub(c, b))
            }),
        },
        Mutation {
            name: "metric signature / Galilean interval",
            kind: MutationKind::NonZeroIdentity(|| {
                let t = Expr::var("t");
                let x = Expr::var("x");
                let b = Expr::var("beta");
                sub(
                    sub(pow(t.clone(), 2), pow(sub(x.clone(), mul(b, t.clone())), 2)),
                    sub(pow(t, 2), pow(x, 2)),
                )
            }),
        },
        Mutation {
            name: "c instead of c^2 (drop a square)",
            kind: MutationKind::NonZeroIdentity(|| {
                let t = Expr::var("t");
                let x = Expr::var("x");
                let b = Expr::var("beta");
                // drop the square on the boosted time term
                sub(
                    sub(
                        sub(t.clone(), mul(b.clone(), x.clone())),
                        pow(sub(x.clone(), mul(b.clone(), t.clone())), 2),
                    ),
                    mul(sub(Expr::c(1), pow(b, 2)), sub(pow(t, 2), pow(x, 2))),
                )
            }),
        },
        Mutation {
            name: "forall silently rewritten by sorry",
            kind: MutationKind::HostileLean("theorem T : Desired := sorry\n"),
        },
        Mutation {
            name: "hidden axiom answer_is_true",
            kind: MutationKind::HostileLean("axiom answer_is_true : DesiredTheorem\n"),
        },
        Mutation {
            name: "vacuous True as a stand-in for d^2 = 0",
            kind: MutationKind::NonZeroIdentity(|| add(Expr::var("a"), Expr::c(1))),
        },
        Mutation {
            name: "factor 2 deletion on (b-a)-(c-a)+2(c-b)",
            kind: MutationKind::NonZeroIdentity(|| {
                let a = Expr::var("a");
                let b = Expr::var("b");
                let c = Expr::var("c");
                add(
                    sub(sub(b.clone(), a.clone()), sub(c.clone(), a)),
                    mul(Expr::c(2), sub(c, b)),
                )
            }),
        },
    ]
}

/// Run the corpus. Every mutation must fail to promote.
pub fn attack() -> Result<(), String> {
    let claim = lookup("dec.d-squared-zero").unwrap().lab_claim();
    let challenge = Challenge::generate(&FormalClaim::from_claim(&claim));
    for m in corpus() {
        match m.kind {
            MutationKind::NonZeroIdentity(f) => {
                if identity_is_zero(&f()).is_ok() {
                    return Err(format!("{}: identity was zero; coverage hole", m.name));
                }
            }
            MutationKind::HostileLean(src) => {
                match verify(
                    &challenge,
                    &UntrustedProof::LeanSource { source: src.into() },
                ) {
                    Err(VerifyError::UnauthorizedAxiom(_)) => {}
                    other => {
                        return Err(format!(
                            "{}: expected unauthorized axiom, got {other:?}",
                            m.name
                        ))
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_team_corpus_is_caught() {
        attack().unwrap();
    }
}
