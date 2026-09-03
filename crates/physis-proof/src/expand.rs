//! Two independent expanders for multivariate polynomials over ℤ.
//!
//! Agreement that an expression is the zero polynomial is the exact-certificate
//! analogue of dual kernel replay. Neither expander is a Lean kernel; the
//! receipt names the backend honestly.

use std::collections::BTreeMap;

use crate::expr::Expr;

/// Canonical polynomial: exponent vectors over a fixed variable order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Poly {
    /// Variable names, in the order they were first seen.
    pub vars: Vec<String>,
    /// Coefficient of each monomial. Zero coefficients are omitted.
    pub terms: BTreeMap<Vec<u32>, i128>,
}

impl Poly {
    fn const_val(n: i128) -> Self {
        let mut p = Poly::default();
        if n != 0 {
            p.terms.insert(vec![], n);
        }
        p
    }

    fn var(name: &str) -> Self {
        let mut p = Poly::default();
        p.vars.push(name.to_string());
        p.terms.insert(vec![1], 1);
        p
    }

    fn is_zero(&self) -> bool {
        self.terms.values().all(|c| *c == 0)
    }

    fn align(&self, other_vars: &[String]) -> Self {
        if self.vars == other_vars {
            return self.clone();
        }
        let mut out = Poly {
            vars: other_vars.to_vec(),
            terms: BTreeMap::new(),
        };
        for (exp, coeff) in &self.terms {
            let mut new_exp = vec![0u32; other_vars.len()];
            for (i, v) in self.vars.iter().enumerate() {
                let e = *exp.get(i).unwrap_or(&0);
                if e == 0 {
                    continue;
                }
                let j = other_vars.iter().position(|w| w == v).expect("var");
                new_exp[j] = e;
            }
            *out.terms.entry(new_exp).or_insert(0) += coeff;
        }
        out.terms.retain(|_, c| *c != 0);
        out
    }

    fn merge_vars(&self, other: &Poly) -> Vec<String> {
        let mut vars = self.vars.clone();
        for v in &other.vars {
            if !vars.iter().any(|w| w == v) {
                vars.push(v.clone());
            }
        }
        vars
    }

    fn add_poly(&self, other: &Poly) -> Self {
        let vars = self.merge_vars(other);
        let a = self.align(&vars);
        let b = other.align(&vars);
        let mut terms = a.terms;
        for (k, c) in b.terms {
            *terms.entry(k).or_insert(0) += c;
        }
        terms.retain(|_, c| *c != 0);
        Poly { vars, terms }
    }

    fn sub_poly(&self, other: &Poly) -> Self {
        let mut neg = other.clone();
        for c in neg.terms.values_mut() {
            *c = -*c;
        }
        self.add_poly(&neg)
    }

    fn mul_poly(&self, other: &Poly) -> Self {
        let vars = self.merge_vars(other);
        let a = self.align(&vars);
        let b = other.align(&vars);
        let n = vars.len();
        let mut terms: BTreeMap<Vec<u32>, i128> = BTreeMap::new();
        for (ea, ca) in &a.terms {
            for (eb, cb) in &b.terms {
                let mut e = vec![0u32; n];
                for i in 0..n {
                    e[i] = ea[i] + eb[i];
                }
                *terms.entry(e).or_insert(0) += ca.saturating_mul(*cb);
            }
        }
        terms.retain(|_, c| *c != 0);
        Poly { vars, terms }
    }

    fn pow_poly(&self, k: u32) -> Self {
        if k == 0 {
            return Poly::const_val(1);
        }
        let mut acc = Poly::const_val(1);
        let mut base = self.clone();
        let mut exp = k;
        while exp > 0 {
            if exp & 1 == 1 {
                acc = acc.mul_poly(&base);
            }
            exp >>= 1;
            if exp > 0 {
                base = base.mul_poly(&base);
            }
        }
        acc
    }
}

/// Expander A: recursive walk of the AST, distributing at each node.
pub fn expand_recursive(expr: &Expr) -> Poly {
    match expr {
        Expr::Var(v) => Poly::var(v),
        Expr::Const(n) => Poly::const_val(*n),
        Expr::Add(a, b) => expand_recursive(a).add_poly(&expand_recursive(b)),
        Expr::Sub(a, b) => expand_recursive(a).sub_poly(&expand_recursive(b)),
        Expr::Mul(a, b) => expand_recursive(a).mul_poly(&expand_recursive(b)),
        Expr::Pow(a, k) => expand_recursive(a).pow_poly(*k),
    }
}

/// Expander B: flatten to postfix, then a stack machine. Same ring, different
/// control flow, so an off-by-one in recursion is unlikely to be shared.
pub fn expand_postfix(expr: &Expr) -> Poly {
    let mut ops: Vec<Op> = Vec::new();
    flatten(expr, &mut ops);
    let mut stack: Vec<Poly> = Vec::new();
    for op in ops {
        match op {
            Op::Var(v) => stack.push(Poly::var(&v)),
            Op::Const(n) => stack.push(Poly::const_val(n)),
            Op::Add => {
                let b = stack.pop().expect("add rhs");
                let a = stack.pop().expect("add lhs");
                stack.push(a.add_poly(&b));
            }
            Op::Sub => {
                let b = stack.pop().expect("sub rhs");
                let a = stack.pop().expect("sub lhs");
                stack.push(a.sub_poly(&b));
            }
            Op::Mul => {
                let b = stack.pop().expect("mul rhs");
                let a = stack.pop().expect("mul lhs");
                stack.push(a.mul_poly(&b));
            }
            Op::Pow(k) => {
                let a = stack.pop().expect("pow base");
                stack.push(a.pow_poly(k));
            }
        }
    }
    stack.pop().expect("postfix result")
}

enum Op {
    Var(String),
    Const(i128),
    Add,
    Sub,
    Mul,
    Pow(u32),
}

fn flatten(expr: &Expr, out: &mut Vec<Op>) {
    match expr {
        Expr::Var(v) => out.push(Op::Var(v.clone())),
        Expr::Const(n) => out.push(Op::Const(*n)),
        Expr::Add(a, b) => {
            flatten(a, out);
            flatten(b, out);
            out.push(Op::Add);
        }
        Expr::Sub(a, b) => {
            flatten(a, out);
            flatten(b, out);
            out.push(Op::Sub);
        }
        Expr::Mul(a, b) => {
            flatten(a, out);
            flatten(b, out);
            out.push(Op::Mul);
        }
        Expr::Pow(a, k) => {
            flatten(a, out);
            out.push(Op::Pow(*k));
        }
    }
}

/// Dual-expand: both checkers must agree the polynomial is identically zero.
pub fn identity_is_zero(expr: &Expr) -> Result<(), String> {
    let a = expand_recursive(expr);
    let b = expand_postfix(expr);
    if !a.is_zero() {
        return Err(format!("recursive expander: leftover terms {:?}", a.terms));
    }
    if !b.is_zero() {
        return Err(format!("postfix expander: leftover terms {:?}", b.terms));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{
        cross_product_jacobi, discrete_d2, einstein_composition, energy_momentum,
        lagrange_identity, lorentz_interval, tetrahedron_d2,
    };
    use crate::expr::{add, mul, sub, Expr};

    #[test]
    fn triangle_coboundary_is_zero() {
        identity_is_zero(&discrete_d2()).unwrap();
    }

    #[test]
    fn tetrahedron_coboundary_is_zero() {
        identity_is_zero(&tetrahedron_d2()).unwrap();
    }

    #[test]
    fn lorentz_interval_is_zero() {
        identity_is_zero(&lorentz_interval()).unwrap();
    }

    #[test]
    fn einstein_composition_is_zero() {
        identity_is_zero(&einstein_composition()).unwrap();
    }

    #[test]
    fn energy_momentum_is_zero() {
        identity_is_zero(&energy_momentum()).unwrap();
    }

    #[test]
    fn cross_product_jacobi_is_zero() {
        identity_is_zero(&cross_product_jacobi()).unwrap();
    }

    #[test]
    fn lagrange_identity_is_zero() {
        identity_is_zero(&lagrange_identity()).unwrap();
    }

    #[test]
    fn a_sign_flip_is_not_zero() {
        // (b - a) - (c - a) - (c - b)  (the last plus became minus)
        let a = Expr::var("a");
        let b = Expr::var("b");
        let c = Expr::var("c");
        let flipped = sub(sub(sub(b.clone(), a.clone()), sub(c.clone(), a)), sub(c, b));
        assert!(identity_is_zero(&flipped).is_err());
    }

    #[test]
    fn expanders_agree_on_a_nonzero_poly() {
        let e = add(Expr::var("x"), Expr::c(1));
        let a = expand_recursive(&e);
        let b = expand_postfix(&e);
        assert_eq!(a.align(&b.vars).terms, b.terms);
        assert!(!a.is_zero());
    }

    #[test]
    fn galilean_interval_is_not_identity() {
        // t² − (x − β t)² − (t² − x²)  is not identically zero.
        let t = Expr::var("t");
        let x = Expr::var("x");
        let b = Expr::var("beta");
        let boosted = sub(
            pow_t(&t),
            crate::expr::pow(sub(x.clone(), mul(b, t.clone())), 2),
        );
        let orig = sub(pow_t(&t), crate::expr::pow(x, 2));
        fn pow_t(t: &Expr) -> Expr {
            crate::expr::pow(t.clone(), 2)
        }
        assert!(identity_is_zero(&sub(boosted, orig)).is_err());
    }

    #[test]
    fn galilean_composition_is_not_identity() {
        // 1 − (u+v)² − (1−u²)(1−v²) is not identically zero.
        let u = Expr::var("u");
        let v = Expr::var("v");
        let galilean = sub(
            sub(Expr::c(1), crate::expr::pow(add(u.clone(), v.clone()), 2)),
            mul(
                sub(Expr::c(1), crate::expr::pow(u, 2)),
                sub(Expr::c(1), crate::expr::pow(v, 2)),
            ),
        );
        assert!(identity_is_zero(&galilean).is_err());
    }

    #[test]
    fn galilean_mass_shell_is_not_identity() {
        // E² − (p − β E)² − (E² − p²) is not identically zero.
        let e = Expr::var("E");
        let p = Expr::var("p");
        let b = Expr::var("beta");
        let boosted = sub(
            crate::expr::pow(e.clone(), 2),
            crate::expr::pow(sub(p.clone(), mul(b, e.clone())), 2),
        );
        let orig = sub(crate::expr::pow(e, 2), crate::expr::pow(p, 2));
        assert!(identity_is_zero(&sub(boosted, orig)).is_err());
    }
}
