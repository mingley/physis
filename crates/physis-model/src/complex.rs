//! Complex amplitudes. No external crate: this layer must stay tiny and owned.

use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

/// Complex number `re + i im` with `f64` components.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Complex {
    /// Real part.
    pub re: f64,
    /// Imaginary part.
    pub im: f64,
}

impl Complex {
    /// Zero.
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };
    /// One.
    pub const ONE: Self = Self { re: 1.0, im: 0.0 };
    /// `i`.
    pub const I: Self = Self { re: 0.0, im: 1.0 };

    /// Construct.
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Real number as complex.
    pub const fn from_re(re: f64) -> Self {
        Self { re, im: 0.0 }
    }

    /// Complex conjugate.
    pub const fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// Squared modulus `|z|²`.
    pub fn norm_sqr(self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Modulus `|z|`.
    pub fn norm(self) -> f64 {
        self.norm_sqr().sqrt()
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.re, -self.im)
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.re * rhs, self.im * rhs)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im.abs() < 1e-15 {
            write!(f, "{}", self.re)
        } else if self.re.abs() < 1e-15 {
            write!(f, "{}i", self.im)
        } else {
            write!(f, "{}{:+}i", self.re, self.im)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i_squared_is_minus_one() {
        let m = Complex::I * Complex::I;
        assert!((m.re + 1.0).abs() < 1e-12);
        assert!(m.im.abs() < 1e-12);
    }
}
