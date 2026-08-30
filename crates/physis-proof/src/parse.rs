//! Parse untrusted IR equations into trusted [`Expr`] trees.
//!
//! The catalog owns the identity that must match. A vacuous `0` parses, but
//! its canonical form is not the coboundary (or interval) tree, so a later
//! syntactic check rejects it as an encoding of `d² = 0`.

use crate::expr::{add, mul, pow, sub, Expr};

/// Parse an infix expression over integer constants and named indeterminates.
///
/// Grammar (Pratt): `+`/`-` left, `*` left, `^` right, unary minus.
/// Identifiers are ASCII `[A-Za-z_][A-Za-z0-9_]*`.
pub fn parse_expr(input: &str) -> Result<Expr, String> {
    let mut p = Parser {
        s: input.as_bytes(),
        i: 0,
    };
    p.skip();
    if p.i == p.s.len() {
        return Err("empty expression".into());
    }
    let e = p.parse_bp(0)?;
    p.skip();
    if p.i != p.s.len() {
        return Err(format!(
            "trailing input at byte {}: {}",
            p.i,
            String::from_utf8_lossy(&p.s[p.i..])
        ));
    }
    Ok(e)
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl Parser<'_> {
    fn skip(&mut self) {
        while self.i < self.s.len() && self.s[self.i].is_ascii_whitespace() {
            self.i += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.s.get(self.i).copied()
    }

    fn parse_bp(&mut self, min_bp: u8) -> Result<Expr, String> {
        self.skip();
        let mut lhs = self.nud()?;
        loop {
            self.skip();
            let Some((lbp, rbp, op)) = infix_bp(self.peek()) else {
                break;
            };
            if lbp < min_bp {
                break;
            }
            self.i += 1;
            let rhs = self.parse_bp(rbp)?;
            lhs = match op {
                b'+' => add(lhs, rhs),
                b'-' => sub(lhs, rhs),
                b'*' => mul(lhs, rhs),
                b'^' => {
                    let Expr::Const(k) = rhs else {
                        return Err("exponent must be a non-negative integer literal".into());
                    };
                    if k < 0 || k > u32::MAX as i128 {
                        return Err("exponent out of range".into());
                    }
                    pow(lhs, k as u32)
                }
                _ => unreachable!(),
            };
        }
        Ok(lhs)
    }

    fn nud(&mut self) -> Result<Expr, String> {
        self.skip();
        match self.peek() {
            None => Err("unexpected end of expression".into()),
            Some(b'(') => {
                self.i += 1;
                let inner = self.parse_bp(0)?;
                self.skip();
                if self.peek() != Some(b')') {
                    return Err("missing closing parenthesis".into());
                }
                self.i += 1;
                Ok(inner)
            }
            Some(b'-') => {
                self.i += 1;
                // Unary minus binds tighter than infix +/-, looser than ^.
                let rhs = self.parse_bp(35)?;
                match rhs {
                    Expr::Const(n) => Ok(Expr::c(-n)),
                    other => Ok(sub(Expr::c(0), other)),
                }
            }
            Some(b) if b.is_ascii_digit() => self.number(),
            Some(b) if b.is_ascii_alphabetic() || b == b'_' => self.ident(),
            Some(b) => Err(format!("unexpected character {:?}", b as char)),
        }
    }

    fn number(&mut self) -> Result<Expr, String> {
        let start = self.i;
        while self.i < self.s.len() && self.s[self.i].is_ascii_digit() {
            self.i += 1;
        }
        let s = std::str::from_utf8(&self.s[start..self.i]).unwrap();
        let n: i128 = s.parse().map_err(|_| format!("bad integer '{s}'"))?;
        Ok(Expr::c(n))
    }

    fn ident(&mut self) -> Result<Expr, String> {
        let start = self.i;
        self.i += 1;
        while self.i < self.s.len() {
            let b = self.s[self.i];
            if b.is_ascii_alphanumeric() || b == b'_' {
                self.i += 1;
            } else {
                break;
            }
        }
        let s = std::str::from_utf8(&self.s[start..self.i]).unwrap();
        Ok(Expr::var(s))
    }
}

fn infix_bp(op: Option<u8>) -> Option<(u8, u8, u8)> {
    match op {
        Some(b'+') | Some(b'-') => Some((10, 11, op.unwrap())),
        Some(b'*') => Some((20, 21, b'*')),
        Some(b'^') => Some((30, 30, b'^')),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::{discrete_d2, einstein_composition, energy_momentum, lorentz_interval};

    #[test]
    fn human_d2_matches_catalog_tree() {
        let e = parse_expr("(b - a) - (c - a) + (c - b)").unwrap();
        assert_eq!(e, discrete_d2());
    }

    #[test]
    fn human_lorentz_matches_catalog_tree() {
        let e =
            parse_expr("(t - beta * x)^2 - (x - beta * t)^2 - (1 - beta^2) * (t^2 - x^2)").unwrap();
        assert_eq!(e, lorentz_interval());
    }

    #[test]
    fn human_composition_matches_catalog_tree() {
        let e = parse_expr("(1 + u * v)^2 - (u + v)^2 - (1 - u^2) * (1 - v^2)").unwrap();
        assert_eq!(e, einstein_composition());
    }

    #[test]
    fn human_mass_shell_matches_catalog_tree() {
        let e =
            parse_expr("(E - beta * p)^2 - (p - beta * E)^2 - (1 - beta^2) * (E^2 - p^2)").unwrap();
        assert_eq!(e, energy_momentum());
    }

    #[test]
    fn display_round_trips_catalog() {
        for e in [
            discrete_d2(),
            lorentz_interval(),
            einstein_composition(),
            energy_momentum(),
        ] {
            assert_eq!(parse_expr(&e.to_string()).unwrap(), e);
        }
    }

    #[test]
    fn vacuous_zero_is_not_the_d2_tree() {
        let z = parse_expr("0").unwrap();
        assert_ne!(z.canonical(), discrete_d2().canonical());
    }

    #[test]
    fn trailing_junk_is_rejected() {
        assert!(parse_expr("a + b extra").is_err());
    }
}
