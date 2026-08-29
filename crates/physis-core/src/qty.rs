//! Dimensioned quantities.
//!
//! Arithmetic is typed: you can add two lengths, you cannot add a length
//! to a mass, and multiplying mass by velocity squared yields energy.

use std::fmt;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Neg, Sub};

use typenum::Z0;

use crate::dim::{
    Acceleration, Charge, Current, Dimensionless, Energy, EnergyDensity, Force, Frequency, HasDim,
    Length, Mass, Power, Pressure, Temperature, Time, Velocity, SI,
};

/// A scalar with a type-level SI dimension.
///
/// The numeric payload is `f64` in SI base units. The type parameter is the
/// proof of dimension. This is a *mechanical* model, not interval arithmetic:
/// values are point estimates. Uncertainty belongs at the claim layer.
#[derive(Debug, PartialEq)]
pub struct Qty<D> {
    value: f64,
    _dim: PhantomData<fn() -> D>,
}

impl<D> Copy for Qty<D> {}

impl<D> Clone for Qty<D> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<D> Qty<D> {
    /// Construct a quantity in SI base units.
    pub const fn new(value: f64) -> Self {
        Self {
            value,
            _dim: PhantomData,
        }
    }

    /// Numeric value in SI base units.
    pub const fn value(self) -> f64 {
        self.value
    }

    /// Absolute value, same dimension.
    pub fn abs(self) -> Self {
        Self::new(self.value.abs())
    }

    /// Square of a quantity (exponents double via `Mul`).
    pub fn pow2(self) -> <Self as Mul<Self>>::Output
    where
        Self: Mul<Self>,
    {
        self * self
    }
}

impl<D: HasDim> Qty<D> {
    /// Runtime exponents, for journals and display.
    pub fn exponents() -> crate::dim::DimExponents {
        D::exponents()
    }
}

impl<D> Default for Qty<D> {
    fn default() -> Self {
        Self::new(0.0)
    }
}

impl<D: HasDim> fmt::Display for Qty<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let u = D::exponents();
        if u.is_dimensionless() {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{} {}", self.value, u)
        }
    }
}

impl<D> Add for Qty<D> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.value + rhs.value)
    }
}

impl<D> Sub for Qty<D> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.value - rhs.value)
    }
}

impl<D> Neg for Qty<D> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.value)
    }
}

impl<D> Mul<f64> for Qty<D> {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.value * rhs)
    }
}

impl<D> Mul<Qty<D>> for f64 {
    type Output = Qty<D>;
    fn mul(self, rhs: Qty<D>) -> Self::Output {
        Qty::new(self * rhs.value)
    }
}

impl<D> Div<f64> for Qty<D> {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.value / rhs)
    }
}

impl<M1, L1, T1, I1, Th1, N1, J1, M2, L2, T2, I2, Th2, N2, J2>
    Mul<Qty<SI<M2, L2, T2, I2, Th2, N2, J2>>> for Qty<SI<M1, L1, T1, I1, Th1, N1, J1>>
where
    M1: Add<M2>,
    L1: Add<L2>,
    T1: Add<T2>,
    I1: Add<I2>,
    Th1: Add<Th2>,
    N1: Add<N2>,
    J1: Add<J2>,
{
    type Output = Qty<
        SI<
            <M1 as Add<M2>>::Output,
            <L1 as Add<L2>>::Output,
            <T1 as Add<T2>>::Output,
            <I1 as Add<I2>>::Output,
            <Th1 as Add<Th2>>::Output,
            <N1 as Add<N2>>::Output,
            <J1 as Add<J2>>::Output,
        >,
    >;

    fn mul(self, rhs: Qty<SI<M2, L2, T2, I2, Th2, N2, J2>>) -> Self::Output {
        Qty::new(self.value * rhs.value)
    }
}

impl<M1, L1, T1, I1, Th1, N1, J1, M2, L2, T2, I2, Th2, N2, J2>
    Div<Qty<SI<M2, L2, T2, I2, Th2, N2, J2>>> for Qty<SI<M1, L1, T1, I1, Th1, N1, J1>>
where
    M1: Sub<M2>,
    L1: Sub<L2>,
    T1: Sub<T2>,
    I1: Sub<I2>,
    Th1: Sub<Th2>,
    N1: Sub<N2>,
    J1: Sub<J2>,
{
    type Output = Qty<
        SI<
            <M1 as Sub<M2>>::Output,
            <L1 as Sub<L2>>::Output,
            <T1 as Sub<T2>>::Output,
            <I1 as Sub<I2>>::Output,
            <Th1 as Sub<Th2>>::Output,
            <N1 as Sub<N2>>::Output,
            <J1 as Sub<J2>>::Output,
        >,
    >;

    fn div(self, rhs: Qty<SI<M2, L2, T2, I2, Th2, N2, J2>>) -> Self::Output {
        Qty::new(self.value / rhs.value)
    }
}

/// Dimensionless scalar.
pub fn scalar(v: f64) -> Qty<Dimensionless> {
    Qty::new(v)
}
/// Kilograms.
pub fn kg(v: f64) -> Qty<Mass> {
    Qty::new(v)
}
/// Meters.
pub fn meters(v: f64) -> Qty<Length> {
    Qty::new(v)
}
/// Seconds.
pub fn seconds(v: f64) -> Qty<Time> {
    Qty::new(v)
}
/// Meters per second.
pub fn meters_per_second(v: f64) -> Qty<Velocity> {
    Qty::new(v)
}
/// Meters per second squared.
pub fn meters_per_second_squared(v: f64) -> Qty<Acceleration> {
    Qty::new(v)
}
/// Newtons.
pub fn newton(v: f64) -> Qty<Force> {
    Qty::new(v)
}
/// Joules.
pub fn joule(v: f64) -> Qty<Energy> {
    Qty::new(v)
}
/// Watts.
pub fn watt(v: f64) -> Qty<Power> {
    Qty::new(v)
}
/// Pascals (also joules per cubic metre: energy density shares this dimension).
pub fn pascal(v: f64) -> Qty<Pressure> {
    Qty::new(v)
}
/// Joules per cubic metre. Same SI dimension as [`pascal`].
pub fn joules_per_cubic_meter(v: f64) -> Qty<EnergyDensity> {
    Qty::new(v)
}
/// Hertz.
pub fn hertz(v: f64) -> Qty<Frequency> {
    Qty::new(v)
}
/// Amperes.
pub fn ampere(v: f64) -> Qty<Current> {
    Qty::new(v)
}
/// Coulombs.
pub fn coulomb(v: f64) -> Qty<Charge> {
    Qty::new(v)
}
/// Kelvin.
pub fn kelvin(v: f64) -> Qty<Temperature> {
    Qty::new(v)
}

/// A quantity whose dimension is exactly dimensionless.
pub type Unitless = Qty<SI<Z0, Z0, Z0, Z0, Z0, Z0, Z0>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dim::Energy;

    #[test]
    fn kinetic_energy_type_and_value() {
        let m = kg(2.0);
        let v = meters_per_second(3.0);
        let k: Qty<Energy> = m * v * v * 0.5;
        assert!((k.value() - 9.0).abs() < 1e-12);
    }

    #[test]
    fn force_from_mass_and_accel() {
        let f = kg(10.0) * meters_per_second_squared(2.0);
        assert!((f.value() - 20.0).abs() < 1e-12);
    }

    #[test]
    fn same_dimension_adds() {
        let a = meters(1.0) + meters(2.5);
        assert!((a.value() - 3.5).abs() < 1e-12);
    }

    #[test]
    fn display_includes_unit() {
        let e = joule(4.2);
        let s = format!("{e}");
        assert!(s.contains("kg"));
        assert!(s.contains("m^2"));
    }

    #[test]
    fn energy_density_times_volume_is_energy() {
        let u = joules_per_cubic_meter(3.0);
        let side = meters(2.0);
        let energy: Qty<Energy> = u * side * side * side;
        assert!((energy.value() - 24.0).abs() < 1e-12);
    }
}
