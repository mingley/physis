//! Named constants in SI, typed.
//!
//! Values are 2018/2019 SI exact or CODATA 2018 point estimates.
//! They are *knobs of nature* in a deeper theory; here they are constants
//! so that theories can be compared against the same measuring sticks.

use physis_core::dim::{
    Action, Dimensionless, Energy, EnergyDensity, Length, Mass, RadiationConstant, StefanBoltzmann,
    Time, Velocity,
};
use physis_core::qty::{kg, meters, seconds, Qty};

/// Speed of light in vacuum (exact, SI).
pub const C: Qty<Velocity> = Qty::new(299_792_458.0);

/// Planck constant over 2π, J·s = kg m² s⁻¹.
pub fn hbar() -> Qty<Action> {
    Qty::new(1.054_571_817e-34)
}

/// Planck constant h (exact, SI 2019). Units: J·s = kg m² s⁻¹.
pub fn planck_h() -> Qty<Action> {
    Qty::new(6.626_070_15e-34)
}

/// Newtonian gravitational constant, m³ kg⁻¹ s⁻².
pub fn g_newton() -> Qty<physis_core::SI<typenum::N1, typenum::P3, typenum::N2>> {
    Qty::new(6.674_30e-11)
}

/// Elementary charge, coulomb (exact, SI).
pub fn e_charge() -> physis_core::qty::Qty<physis_core::Charge> {
    physis_core::qty::coulomb(1.602_176_634e-19)
}

/// Electron mass.
pub fn electron_mass() -> Qty<Mass> {
    kg(9.109_383_701_5e-31)
}

/// Proton mass.
pub fn proton_mass() -> Qty<Mass> {
    kg(1.672_621_923_69e-27)
}

/// Solar standard gravitational parameter GM_☉ (IAU 2015), m³ s⁻².
///
/// Using `GM` rather than `G · M_☉` keeps the solar-system theorems free of
/// the relatively large uncertainty on `G`.
pub fn solar_gm() -> Qty<physis_core::SI<typenum::Z0, typenum::P3, typenum::N2>> {
    Qty::new(1.327_124_4e20)
}

/// Nominal solar radius (IAU 2015), metres.
pub fn solar_radius() -> Qty<Length> {
    meters(6.957e8)
}

/// Mercury's semi-major axis, metres (JPL DE).
pub fn mercury_semi_major() -> Qty<Length> {
    meters(5.790_917_5e10)
}

/// Mercury's orbital eccentricity (JPL DE).
pub fn mercury_eccentricity() -> Qty<Dimensionless> {
    Qty::new(0.205_630)
}

/// Mercury sidereal orbits per Julian century (36525 days / 87.969 d).
pub fn mercury_orbits_per_century() -> f64 {
    36525.0 / 87.969
}

/// Planck length (derived constant, CODATA-style value).
pub fn planck_length() -> Qty<Length> {
    meters(1.616_255e-35)
}

/// Planck time.
pub fn planck_time() -> Qty<Time> {
    seconds(5.391_247e-44)
}

/// Electron-volt in joules, as energy.
pub fn electron_volt() -> Qty<Energy> {
    Qty::new(1.602_176_634e-19)
}

/// Fine-structure constant α ≈ 1/137.035999 (dimensionless), CODATA 2018.
///
/// A coupling is a first-class dimensioned quantity here, not a bare float.
/// Its value is M2 scope; running it with energy is M4.
pub fn fine_structure_constant() -> Qty<Dimensionless> {
    Qty::new(7.297_352_569_3e-3)
}

/// Strong coupling α_s at the Z mass (dimensionless), PDG 2022.
pub fn strong_coupling_mz() -> Qty<Dimensionless> {
    Qty::new(0.1179)
}

/// Inverse electromagnetic coupling α_em⁻¹ at the Z mass (dimensionless), PDG.
///
/// The fine-structure constant *runs*: it is ≈1/137 at zero momentum but
/// ≈1/128 at the electroweak scale. Gauge-coupling unification is stated at
/// `M_Z`, so this is the value the running starts from.
pub fn inverse_alpha_em_mz() -> Qty<Dimensionless> {
    Qty::new(127.951)
}

/// Weak mixing angle sin²θ_W at the Z mass (dimensionless, MS-bar), PDG.
pub fn weak_mixing_angle_sin2_mz() -> Qty<Dimensionless> {
    Qty::new(0.231_21)
}

/// Z boson mass in GeV (PDG), the reference scale for electroweak running.
pub fn z_mass_gev() -> Qty<Dimensionless> {
    Qty::new(91.1876)
}

/// Boltzmann constant k_B (exact, SI). Units: J/K = kg·m²·s⁻²·K⁻¹.
pub fn k_boltzmann(
) -> Qty<physis_core::SI<typenum::P1, typenum::P2, typenum::N2, typenum::Z0, typenum::N1>> {
    Qty::new(1.380_649e-23)
}

/// Stefan–Boltzmann constant σ = 2π⁵ k⁴ / (15 h³ c²), derived from the exact
/// SI values of h, k_B, and c. Units: W m⁻² K⁻⁴.
pub fn stefan_boltzmann_constant() -> Qty<StefanBoltzmann> {
    let k = k_boltzmann();
    let h = planck_h();
    let k4 = k * k * k * k;
    let h3 = h * h * h;
    let c2 = C * C;
    k4 / (h3 * c2) * (2.0 * std::f64::consts::PI.powi(5) / 15.0)
}

/// Radiation density constant `a = 4σ/c` so that a photon gas has `u = a T⁴`.
/// Units: J m⁻³ K⁻⁴.
pub fn radiation_density_constant() -> Qty<RadiationConstant> {
    stefan_boltzmann_constant() / C * 4.0
}

/// Photon-gas energy density `u = a T⁴` (Planck, infinite bandwidth).
pub fn planck_energy_density(temperature: Qty<physis_core::Temperature>) -> Qty<EnergyDensity> {
    let t2 = temperature * temperature;
    radiation_density_constant() * t2 * t2
}

/// Vacuum permittivity ε₀ (F/m), CODATA. Units: A²·s⁴·kg⁻¹·m⁻³.
pub fn epsilon0() -> Qty<physis_core::SI<typenum::N1, typenum::N3, typenum::P4, typenum::P2>> {
    Qty::new(8.854_187_812_8e-12)
}

/// Vacuum permeability μ₀ (H/m), CODATA. Units: kg·m·s⁻²·A⁻².
pub fn mu0() -> Qty<physis_core::SI<typenum::P1, typenum::P1, typenum::N2, typenum::N2>> {
    Qty::new(1.256_637_062_12e-6)
}

/// Fermi coupling constant G_F, as a typed energy⁻² quantity (SI J⁻²).
///
/// The measured value is `G_F/(ħc)³ = 1.166_378_7e-5 GeV⁻²`; converted to SI
/// joules⁻² here. The type `energy⁻²` is the point: multiplying `G_F` by two
/// energies is a dimensionless number *by construction* (see the test), and
/// multiplying it by anything else is a compile error.
pub fn fermi_coupling() -> Qty<physis_core::SI<typenum::N2, typenum::N4, typenum::P4>> {
    // 1.1663787e-5 GeV^-2 × (1 GeV / 1.602176634e-10 J)^2  ≈ 4.5438e14 J^-2.
    Qty::new(4.5438e14)
}

/// Speed of light (function form for tests that want `.value()`).
pub fn c_value() -> f64 {
    C.value()
}

/// Lorentz factor `γ = 1/sqrt(1 - β²)` for `|v| < c`.
pub fn lorentz_gamma(v: Qty<Velocity>) -> Option<f64> {
    let beta = v.value() / C.value();
    let s = 1.0 - beta * beta;
    if s <= 0.0 {
        None
    } else {
        Some(1.0 / s.sqrt())
    }
}

/// Relativistic energy `γ m c²`.
pub fn rest_energy(m: Qty<Mass>) -> Qty<Energy> {
    m * C * C
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::qty::meters_per_second;

    #[test]
    fn rest_energy_electron_order() {
        let e = rest_energy(electron_mass());
        // 511 keV ~ 8.2e-14 J
        assert!(e.value() > 8e-14 && e.value() < 9e-14);
    }

    #[test]
    fn gamma_at_rest_is_one() {
        assert!((lorentz_gamma(meters_per_second(0.0)).unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn gamma_rejects_superluminal() {
        assert!(lorentz_gamma(meters_per_second(C.value() * 1.1)).is_none());
    }

    #[test]
    fn light_speed_from_permittivity_and_permeability() {
        // 1/√(ε₀μ₀) = c, encoded mechanically: ε₀·μ₀·c² is dimensionless and
        // equals 1. The type annotation compiles only if the units cancel.
        let one: Qty<Dimensionless> = epsilon0() * mu0() * C * C;
        assert!(
            (one.value() - 1.0).abs() < 1e-6,
            "ε₀·μ₀·c² = {} (should be 1)",
            one.value()
        );
    }

    #[test]
    fn couplings_are_typed_quantities() {
        // α is dimensionless and ≈ 1/137.
        let alpha = fine_structure_constant();
        assert!((1.0 / alpha.value() - 137.036).abs() < 0.1);
        // α_s(M_Z) ≈ 0.1179.
        assert!((strong_coupling_mz().value() - 0.1179).abs() < 1e-6);

        // G_F is energy⁻²: G_F · E · E is dimensionless *by construction*. The
        // type annotation below only compiles if the dimensions cancel exactly.
        let e = rest_energy(electron_mass());
        let dimensionless: Qty<Dimensionless> = fermi_coupling() * e * e;
        assert!(dimensionless.value() > 0.0 && dimensionless.value().is_finite());
    }

    #[test]
    fn planck_h_is_two_pi_hbar() {
        let ratio = planck_h().value() / (std::f64::consts::TAU * hbar().value());
        assert!(
            (ratio - 1.0).abs() < 1e-9,
            "h / (2π ħ) = {ratio} (should be 1)"
        );
    }

    #[test]
    fn stefan_boltzmann_matches_codata() {
        // Derived from exact h, k, c; CODATA 2018/2019 value 5.670374419e-8.
        let sigma = stefan_boltzmann_constant().value();
        assert!(
            (sigma - 5.670_374_419e-8).abs() / 5.670_374_419e-8 < 1e-9,
            "σ = {sigma}"
        );
    }

    #[test]
    fn planck_energy_density_is_typed_and_finite() {
        use physis_core::qty::kelvin;
        let u = planck_energy_density(kelvin(5000.0));
        // a T⁴ ≈ 7.5657e-16 * 6.25e14 ≈ 0.473 J/m³.
        assert!(u.value() > 0.4 && u.value() < 0.55, "u = {}", u.value());
        assert!(u.value().is_finite());
    }

    #[test]
    fn solar_schwarzschild_radius_is_a_length() {
        // GM/c² is a length by construction (half the Schwarzschild radius).
        let m: Qty<Length> = solar_gm() / (C * C);
        // 1.477 km.
        assert!(
            m.value() > 1.4e3 && m.value() < 1.5e3,
            "GM/c² = {}",
            m.value()
        );
    }
}
