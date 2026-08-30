//! Trusted algebraic expressions. The challenge owns the expression; the
//! untrusted solver does not get to pick the theorem it is judged against.

use serde::{Deserialize, Serialize};

/// Multivariate expression over the integers (and rationals via delayed
/// division-free identities).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Expr {
    /// Named indeterminate.
    Var(String),
    /// Integer constant.
    Const(i128),
    /// Sum.
    Add(Box<Expr>, Box<Expr>),
    /// Difference.
    Sub(Box<Expr>, Box<Expr>),
    /// Product.
    Mul(Box<Expr>, Box<Expr>),
    /// Non-negative integer power.
    Pow(Box<Expr>, u32),
}

impl Expr {
    /// Variable.
    pub fn var(name: impl Into<String>) -> Self {
        Expr::Var(name.into())
    }

    /// Constant.
    pub fn c(n: i128) -> Self {
        Expr::Const(n)
    }

    /// Prefix-notation canonical bytes. Changing a sign or a quantifier-like
    /// rewrite of the identity changes this string.
    pub fn canonical(&self) -> String {
        match self {
            Expr::Var(v) => format!("v:{v}"),
            Expr::Const(n) => format!("n:{n}"),
            Expr::Add(a, b) => format!("+:{},{}", a.canonical(), b.canonical()),
            Expr::Sub(a, b) => format!("-:{},{}", a.canonical(), b.canonical()),
            Expr::Mul(a, b) => format!("*:{},{}", a.canonical(), b.canonical()),
            Expr::Pow(a, k) => format!("^{k}:{}", a.canonical()),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Var(v) => f.write_str(v),
            Expr::Const(n) => write!(f, "{n}"),
            Expr::Add(a, b) => write!(f, "({a} + {b})"),
            Expr::Sub(a, b) => write!(f, "({a} - {b})"),
            Expr::Mul(a, b) => write!(f, "({a} * {b})"),
            Expr::Pow(a, k) => write!(f, "({a})^{k}"),
        }
    }
}

/// Helpers for building identities without drowning in `Box`.
pub fn add(a: Expr, b: Expr) -> Expr {
    Expr::Add(Box::new(a), Box::new(b))
}
/// Difference.
pub fn sub(a: Expr, b: Expr) -> Expr {
    Expr::Sub(Box::new(a), Box::new(b))
}
/// Product.
pub fn mul(a: Expr, b: Expr) -> Expr {
    Expr::Mul(Box::new(a), Box::new(b))
}
/// Power.
pub fn pow(a: Expr, k: u32) -> Expr {
    Expr::Pow(Box::new(a), k)
}
