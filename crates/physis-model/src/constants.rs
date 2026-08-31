//! Named constants in SI, typed.
//!
//! Values are 2018/2019 SI exact or CODATA 2018 point estimates.
//! They are *knobs of nature* in a deeper theory; here they are constants
//! so that theories can be compared against the same measuring sticks.
//! Overlapping SI/CODATA floats lockstep the versioned constants ledger
//! in tests; evaluators still use these `f64` Qty values, not that ledger.

use physis_core::dim::{
    Action, Dimensionless, Energy, EnergyDensity, Frequency, HeatCapacity, Length,
    LuminosityDensity, Mass, Power, RadiationConstant, StefanBoltzmann, Time, Velocity,
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
///
/// CODATA 2018 recommended centre. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn proton_mass() -> Qty<Mass> {
    kg(1.672_621_923_69e-27)
}

/// Solar standard gravitational parameter GM_☉ (IAU 2015 nominal), m³ s⁻².
///
/// This is the IAU 2015 conversion ruler `(GM)_☉^N`, not a measured solar
/// mass and not `G · M_☉`. Using `GM` rather than `G · M_☉` keeps the
/// solar-system theorems free of the relatively large uncertainty on `G`.
pub fn solar_gm() -> Qty<physis_core::SI<typenum::Z0, typenum::P3, typenum::N2>> {
    Qty::new(1.327_124_4e20)
}

/// Nominal solar radius (IAU 2015 conversion ruler), metres.
///
/// This is `R_☉^N`, not a measured photospheric radius.
pub fn solar_radius() -> Qty<Length> {
    meters(6.957e8)
}

/// Nominal solar luminosity (IAU 2015 conversion ruler), watts.
///
/// This is `L_☉^N`, not a measured solar luminosity.
pub fn solar_luminosity() -> Qty<Power> {
    Qty::new(3.828e26)
}

/// Astronomical unit (IAU 2012 / BIPM table 8), metres. Exact.
pub fn astronomical_unit() -> Qty<Length> {
    meters(149_597_870_700.0)
}

/// Parsec, metres. IAU 2015: `(648 000 / π)` astronomical units, with the AU exact.
///
/// π means this is not a Ratio. The versioned ledger stores `au`, not `pc`.
pub fn parsec() -> Qty<Length> {
    use std::f64::consts::PI;
    meters((648_000.0 / PI) * astronomical_unit().value())
}

/// Hubble constant H₀ ≈ 70 km s⁻¹ Mpc⁻¹, as a frequency (s⁻¹).
///
/// Order-of-magnitude cosmology, not a precision H₀ fit.
pub fn hubble_constant() -> Qty<Frequency> {
    let v = Qty::<Velocity>::new(70_000.0); // 70 km/s
    v / (parsec() * 1.0e6)
}

/// Mean cosmic starlight luminosity density, ~10⁸ L_☉ / Mpc³.
///
/// An order-of-magnitude extragalactic average, not a galaxy-survey fit.
pub fn cosmic_luminosity_density() -> Qty<LuminosityDensity> {
    let mpc = parsec() * 1.0e6;
    solar_luminosity() / (mpc * mpc * mpc) * 1.0e8
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

/// Electron-volt in joules, as energy (SI 2019 exact, BIPM table 8).
pub fn electron_volt() -> Qty<Energy> {
    Qty::new(1.602_176_634e-19)
}

/// Fine-structure constant α ≈ 1/137.035999 (dimensionless), CODATA 2018.
///
/// A coupling is a first-class dimensioned quantity here, not a bare float.
/// Its value is M2 scope; running it with energy is M4. The versioned
/// ledger stores the one-sigma hull; this Qty is the recommended centre.
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
pub fn k_boltzmann() -> Qty<HeatCapacity> {
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
///
/// After SI 2019 this is the derived value `1/(μ₀ c²)`, not exact.
/// The versioned ledger stores the one-sigma hull; this Qty is the
/// recommended centre. `Z₀` is not a ledger entry.
pub fn epsilon0() -> Qty<physis_core::SI<typenum::N1, typenum::N3, typenum::P4, typenum::P2>> {
    Qty::new(8.854_187_812_8e-12)
}

/// Vacuum permeability μ₀ (H/m), CODATA. Units: kg·m·s⁻²·A⁻².
///
/// After SI 2019 this is a measured value, not exact `4π×10^{-7}`.
/// The versioned ledger stores the one-sigma hull; this Qty is the
/// recommended centre.
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

    #[test]
    fn cosmic_luminosity_density_times_length_is_irradiance() {
        use physis_core::dim::Irradiance;
        use physis_core::qty::meters;
        let f: Qty<Irradiance> = cosmic_luminosity_density() * meters(1.0);
        assert!(f.value() > 0.0 && f.value().is_finite());
        let h = hubble_constant().value();
        assert!((h - 2.27e-18).abs() / 2.27e-18 < 0.05, "H₀ = {h} /s");
    }

    #[test]
    fn overlapping_qty_floats_lockstep_the_versioned_ledger() {
        use physis_numeric::{Interval, Ratio, SciExact};

        assert_eq!(
            C.value(),
            physis_constants::speed_of_light().value.to_f64(),
            "c is an integer Ratio; Qty matches to_f64"
        );

        let e = physis_constants::elementary_charge();
        assert_eq!(
            e.value,
            Ratio::new(1_602_176_634, 10i128.pow(28)),
            "ledger e is the SI 2019 fraction"
        );
        assert_eq!(
            SciExact::new(1_602_176_634, -28).to_ratio(),
            Some(e.value),
            "e fits in i128; SciExact and Ratio are the same decimal"
        );
        assert_eq!(
            e_charge().value(),
            SciExact::new(1_602_176_634, -28).to_f64(),
            "e Qty is the IEEE rounding of the SI decimal, not Ratio::to_f64 of the reduced fraction"
        );

        let k = physis_constants::boltzmann();
        assert_eq!(k.value, Ratio::new(1_380_649, 10i128.pow(29)));
        assert_eq!(SciExact::new(1_380_649, -29).to_ratio(), Some(k.value));
        assert_eq!(
            k_boltzmann().value(),
            SciExact::new(1_380_649, -29).to_f64(),
            "k Qty is the IEEE rounding of the SI decimal"
        );

        let h = physis_constants::planck_h();
        assert_eq!(h.value.to_ratio(), None, "h still does not fit in i128");
        assert_eq!(
            planck_h().value(),
            h.value.to_f64(),
            "h Qty must match the SI 2019 SciExact decimal float"
        );

        let g = physis_constants::newtonian_g();
        let centre = Ratio::new(667_430, 10i128.pow(16));
        assert_eq!(
            g_newton().value(),
            centre.to_f64(),
            "G Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            g.value.contains(Interval::point(centre)),
            "G Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            g.value.lo, g.value.hi,
            "ledger G stays an Interval; the Qty is not that Interval"
        );

        let mu0_c = physis_constants::vacuum_permeability();
        let mu0_centre = Ratio::new(125_663_706_212, 10i128.pow(17));
        assert_eq!(
            mu0().value(),
            mu0_centre.to_f64(),
            "mu0 Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu0_c.value.contains(Interval::point(mu0_centre)),
            "mu0 Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu0_c.value.lo, mu0_c.value.hi,
            "ledger mu0 stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("mu_0").is_none(),
            "mu_0 is not a ledger name; the live name is mu0"
        );

        let eps = physis_constants::vacuum_permittivity();
        let eps_centre = Ratio::new(88_541_878_128, 10i128.pow(22));
        assert_eq!(
            epsilon0().value(),
            eps_centre.to_f64(),
            "epsilon0 Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            eps.value.contains(Interval::point(eps_centre)),
            "epsilon0 Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            eps.value.lo, eps.value.hi,
            "ledger epsilon0 stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("Z0").is_none(),
            "Z0 is a different recommended value and is not stored"
        );
        assert!(
            physis_constants::lookup("epsilon_0").is_none(),
            "epsilon_0 is not a ledger name; the live name is epsilon0"
        );

        let alpha = physis_constants::fine_structure_constant();
        let alpha_centre = Ratio::new(72_973_525_693, 10i128.pow(13));
        assert_eq!(
            fine_structure_constant().value(),
            alpha_centre.to_f64(),
            "alpha Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            alpha.value.contains(Interval::point(alpha_centre)),
            "alpha Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            alpha.value.lo, alpha.value.hi,
            "ledger alpha stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("hbar").is_none(),
            "ħ is not a terminating decimal and is not a ledger entry"
        );
        assert!(
            physis_constants::lookup("alpha-inv").is_none(),
            "inverse-alpha is a different recommended value and is not stored"
        );

        let mp = physis_constants::proton_mass();
        let mp_centre = Ratio::new(167_262_192_369, 10i128.pow(38));
        assert_eq!(
            proton_mass().value(),
            mp_centre.to_f64(),
            "m_p Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mp.value.contains(Interval::point(mp_centre)),
            "m_p Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mp.value.lo, mp.value.hi,
            "ledger m_p stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("m_e").is_none(),
            "electron mass overflows i128 and is not a ledger entry"
        );

        let au = physis_constants::astronomical_unit();
        assert_eq!(au.value, Ratio::int(149_597_870_700), "ledger au is exact");
        assert_eq!(
            astronomical_unit().value(),
            au.value.to_f64(),
            "au is an integer Ratio; Qty matches to_f64"
        );
        assert_eq!(
            astronomical_unit().value(),
            149_597_870_700.0,
            "IAU 2012 au is the exact metre count"
        );

        let gm = physis_constants::solar_gm();
        assert_eq!(
            gm.value,
            Ratio::int(13_271_244i128 * 10i128.pow(13)),
            "ledger GM_sun is the IAU 2015 integer Ratio"
        );
        assert_eq!(
            solar_gm().value(),
            gm.value.to_f64(),
            "GM_sun is an integer Ratio; Qty matches to_f64"
        );
        assert_eq!(
            solar_gm().value(),
            1.327_124_4e20,
            "IAU 2015 (GM)_sun^N is the exact conversion ruler"
        );

        let r = physis_constants::solar_radius();
        assert_eq!(
            r.value,
            Ratio::int(695_700_000),
            "ledger R_sun is the IAU 2015 integer Ratio"
        );
        assert_eq!(
            solar_radius().value(),
            r.value.to_f64(),
            "R_sun is an integer Ratio; Qty matches to_f64"
        );
        assert_eq!(
            solar_radius().value(),
            6.957e8,
            "IAU 2015 R_sun^N is the exact conversion ruler"
        );

        let l = physis_constants::solar_luminosity();
        assert_eq!(
            l.value,
            Ratio::int(3_828i128 * 10i128.pow(23)),
            "ledger L_sun is the IAU 2015 integer Ratio"
        );
        assert_eq!(
            solar_luminosity().value(),
            l.value.to_f64(),
            "L_sun is an integer Ratio; Qty matches to_f64"
        );
        assert_eq!(
            solar_luminosity().value(),
            3.828e26,
            "IAU 2015 L_sun^N is the exact conversion ruler"
        );

        let ev = physis_constants::electron_volt();
        assert_eq!(
            ev.value,
            Ratio::new(1_602_176_634, 10i128.pow(28)),
            "ledger eV is the SI 2019 fraction"
        );
        assert_eq!(ev.value, physis_constants::elementary_charge().value);
        assert_eq!(SciExact::new(1_602_176_634, -28).to_ratio(), Some(ev.value));
        assert_eq!(
            electron_volt().value(),
            SciExact::new(1_602_176_634, -28).to_f64(),
            "eV Qty is the IEEE rounding of the SI decimal, not Ratio::to_f64 of the reduced fraction"
        );
        assert_eq!(
            electron_volt().value(),
            e_charge().value(),
            "1 eV is e * 1 V numerically"
        );
    }
}
