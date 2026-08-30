//! Type-level SI dimensions.
//!
//! A dimension is a 7-tuple of type-level integers
//! `(M, L, T, I, Θ, N, J)` in the SI base:
//! mass, length, time, current, temperature, amount, luminous intensity.
//!
//! Multiplication of quantities *adds* exponents (via `typenum`).
//! Addition of quantities is only implemented for identical dimension types.

use std::fmt;
use std::marker::PhantomData;

use typenum::{Integer, N1, N2, N3, N4, P1, P2, Z0};

/// SI dimension vector as a zero-sized type.
///
/// Type parameters are `typenum` integers. Defaults are zero, so
/// `SI<P1, Z0, Z0>` is mass and `SI<Z0, P1, Z0>` is length.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SI<M, L, T, I = Z0, Th = Z0, N = Z0, J = Z0> {
    _m: PhantomData<fn() -> M>,
    _l: PhantomData<fn() -> L>,
    _t: PhantomData<fn() -> T>,
    _i: PhantomData<fn() -> I>,
    _th: PhantomData<fn() -> Th>,
    _n: PhantomData<fn() -> N>,
    _j: PhantomData<fn() -> J>,
}

/// Runtime view of a dimension (for display, journals, agents).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DimExponents {
    /// Mass (kg).
    pub mass: i8,
    /// Length (m).
    pub length: i8,
    /// Time (s).
    pub time: i8,
    /// Current (A).
    pub current: i8,
    /// Temperature (K).
    pub temperature: i8,
    /// Amount (mol).
    pub amount: i8,
    /// Luminous intensity (cd).
    pub luminous: i8,
}

impl DimExponents {
    /// All seven exponents.
    pub const fn as_array(self) -> [i8; 7] {
        [
            self.mass,
            self.length,
            self.time,
            self.current,
            self.temperature,
            self.amount,
            self.luminous,
        ]
    }

    /// True if every exponent is zero.
    pub const fn is_dimensionless(self) -> bool {
        self.mass == 0
            && self.length == 0
            && self.time == 0
            && self.current == 0
            && self.temperature == 0
            && self.amount == 0
            && self.luminous == 0
    }
}

impl fmt::Display for DimExponents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_dimensionless() {
            return write!(f, "1");
        }
        const SYMS: [&str; 7] = ["kg", "m", "s", "A", "K", "mol", "cd"];
        let exp = self.as_array();
        let mut first = true;
        for (sym, e) in SYMS.iter().zip(exp) {
            if e == 0 {
                continue;
            }
            if !first {
                write!(f, "·")?;
            }
            first = false;
            if e == 1 {
                write!(f, "{sym}")?;
            } else {
                write!(f, "{sym}^{e}")?;
            }
        }
        Ok(())
    }
}

/// Types that have a runtime SI exponent vector.
pub trait HasDim {
    /// Exponents of this dimension type.
    fn exponents() -> DimExponents;
}

impl<M, L, T, I, Th, N, J> HasDim for SI<M, L, T, I, Th, N, J>
where
    M: Integer,
    L: Integer,
    T: Integer,
    I: Integer,
    Th: Integer,
    N: Integer,
    J: Integer,
{
    fn exponents() -> DimExponents {
        DimExponents {
            mass: M::to_i8(),
            length: L::to_i8(),
            time: T::to_i8(),
            current: I::to_i8(),
            temperature: Th::to_i8(),
            amount: N::to_i8(),
            luminous: J::to_i8(),
        }
    }
}

/// Dimensionless (all exponents zero).
pub type Dimensionless = SI<Z0, Z0, Z0>;
/// Mass, `M`.
pub type Mass = SI<P1, Z0, Z0>;
/// Length, `L`.
pub type Length = SI<Z0, P1, Z0>;
/// Time, `T`.
pub type Time = SI<Z0, Z0, P1>;
/// Electric current, `I`.
pub type Current = SI<Z0, Z0, Z0, P1>;
/// Thermodynamic temperature, `Θ`.
pub type Temperature = SI<Z0, Z0, Z0, Z0, P1>;
/// Amount of substance, `N`.
pub type Amount = SI<Z0, Z0, Z0, Z0, Z0, P1>;
/// Luminous intensity, `J`.
pub type Luminous = SI<Z0, Z0, Z0, Z0, Z0, Z0, P1>;
/// Velocity, `L T⁻¹`.
pub type Velocity = SI<Z0, P1, N1>;
/// Momentum, `M L T⁻¹`.
pub type Momentum = SI<P1, P1, N1>;
/// Acceleration, `L T⁻²`.
pub type Acceleration = SI<Z0, P1, N2>;
/// Force, `M L T⁻²`.
pub type Force = SI<P1, P1, N2>;
/// Energy, `M L² T⁻²`.
pub type Energy = SI<P1, P2, N2>;
/// Power, `M L² T⁻³`.
pub type Power = SI<P1, P2, N3>;
/// Pressure, `M L⁻¹ T⁻²`.
pub type Pressure = SI<P1, N1, N2>;
/// Frequency, `T⁻¹`.
pub type Frequency = SI<Z0, Z0, N1>;
/// Electric charge, `I T`.
pub type Charge = SI<Z0, Z0, P1, P1>;
/// Action / angular momentum, `M L² T⁻¹` (Planck's constant).
pub type Action = SI<P1, P2, N1>;
/// Energy density, `M L⁻¹ T⁻²` (J/m³). Same dimension as [`Pressure`].
pub type EnergyDensity = Pressure;
/// Spectral energy density per unit frequency, `M L⁻¹ T⁻¹` (J m⁻³ Hz⁻¹).
pub type SpectralEnergyDensity = SI<P1, N1, N1>;
/// Stefan–Boltzmann constant, `M T⁻³ Θ⁻⁴` (W m⁻² K⁻⁴).
pub type StefanBoltzmann = SI<P1, Z0, N3, Z0, N4>;
/// Radiation density constant `a` in `u = a T⁴`, `M L⁻¹ T⁻² Θ⁻⁴` (J m⁻³ K⁻⁴).
pub type RadiationConstant = SI<P1, N1, N2, Z0, N4>;
/// Wien displacement product `λ T`, `L Θ` (m·K).
pub type LengthTemperature = SI<Z0, P1, Z0, Z0, P1>;
/// Heat capacity (and entropy), `M L² T⁻² Θ⁻¹` (J/K).
pub type HeatCapacity = SI<P1, P2, N2, Z0, N1>;
/// Irradiance / heat flux, `M T⁻³` (W/m²). Stefan–Boltzmann `σ T⁴` has this dimension.
pub type Irradiance = SI<P1, Z0, N3>;
/// Luminosity density, `M L⁻¹ T⁻³` (W/m³). Times a length, an irradiance.
pub type LuminosityDensity = SI<P1, N1, N3>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn energy_exponents() {
        let e = <Energy as HasDim>::exponents();
        assert_eq!(e.mass, 1);
        assert_eq!(e.length, 2);
        assert_eq!(e.time, -2);
        assert_eq!(format!("{e}"), "kg·m^2·s^-2");
    }

    #[test]
    fn heat_capacity_exponents() {
        let c = <HeatCapacity as HasDim>::exponents();
        assert_eq!(c.mass, 1);
        assert_eq!(c.length, 2);
        assert_eq!(c.time, -2);
        assert_eq!(c.temperature, -1);
        assert_eq!(format!("{c}"), "kg·m^2·s^-2·K^-1");
    }

    #[test]
    fn irradiance_exponents() {
        let i = <Irradiance as HasDim>::exponents();
        assert_eq!(i.mass, 1);
        assert_eq!(i.length, 0);
        assert_eq!(i.time, -3);
        assert_eq!(format!("{i}"), "kg·s^-3");
    }

    #[test]
    fn luminosity_density_exponents() {
        let l = <LuminosityDensity as HasDim>::exponents();
        assert_eq!(l.mass, 1);
        assert_eq!(l.length, -1);
        assert_eq!(l.time, -3);
        assert_eq!(format!("{l}"), "kg·m^-1·s^-3");
    }

    #[test]
    fn dimensionless_display() {
        let d = <Dimensionless as HasDim>::exponents();
        assert!(d.is_dimensionless());
        assert_eq!(format!("{d}"), "1");
    }

    #[test]
    fn energy_density_matches_pressure() {
        let u = <EnergyDensity as HasDim>::exponents();
        let p = <Pressure as HasDim>::exponents();
        assert_eq!(u, p);
        assert_eq!(u.mass, 1);
        assert_eq!(u.length, -1);
        assert_eq!(u.time, -2);
    }
}
