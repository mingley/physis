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
use physis_core::qty::{joule, kg, meters, seconds, Qty};

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
/// one-sigma hull; this Qty is that centre. This is not the mass in u.
pub fn proton_mass() -> Qty<Mass> {
    kg(1.672_621_923_69e-27)
}

/// Proton mass in unified atomic mass units, CODATA 2018.
///
/// This is the recommended centre in u, not the kg hull and not muon
/// mass in u. The versioned ledger stores the one-sigma hull; this Qty
/// is that centre. Ledger unit is u; this Qty is dimensionless, not kg.
pub fn proton_mass_in_u() -> Qty<Dimensionless> {
    Qty::new(1.007_276_466_621)
}

/// Proton mass energy equivalent m_p c² (J), CODATA 2018.
///
/// This is the recommended centre in joules, not the kg hull, not the
/// u-row, and not the MeV conversion. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn proton_mass_energy_equivalent() -> Qty<Energy> {
    joule(1.503_277_615_98e-10)
}

/// Proton mass energy equivalent in MeV, CODATA 2018.
///
/// This is the recommended centre in MeV, not the joule hull and not
/// the exact electronvolt Ratio. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre. Ledger unit is MeV; this
/// Qty is dimensionless, not SI joule.
pub fn proton_mass_energy_equivalent_in_mev() -> Qty<Dimensionless> {
    Qty::new(938.272_088_16)
}

/// Proton-electron mass ratio m_p/m_e, CODATA 2018.
///
/// This is the recommended centre from the proton section, not the
/// electron-proton mass ratio and not a certificate that the stored
/// centres invert. The versioned ledger stores the one-sigma hull;
/// this Qty is that centre.
pub fn proton_electron_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(1_836.152_673_43)
}

/// Muon mass.
///
/// CODATA 2018 recommended centre. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre. This is not the
/// electron-muon mass ratio.
pub fn muon_mass() -> Qty<Mass> {
    kg(1.883_531_627e-28)
}

/// Muon mass in unified atomic mass units, CODATA 2018.
///
/// This is the recommended centre in u, not the kg hull and not
/// electron molar mass. The versioned ledger stores the one-sigma
/// hull; this Qty is that centre. Ledger unit is u; this Qty is
/// dimensionless, not kg.
pub fn muon_mass_in_u() -> Qty<Dimensionless> {
    Qty::new(0.113_428_925_9)
}

/// Muon mass energy equivalent m_μ c² (J), CODATA 2018.
///
/// This is the recommended centre in joules, not the kg hull, not the
/// u-row, and not the MeV conversion. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn muon_mass_energy_equivalent() -> Qty<Energy> {
    joule(1.692_833_804e-11)
}

/// Muon mass energy equivalent in MeV, CODATA 2018.
///
/// This is the recommended centre in MeV, not the joule hull and not
/// the exact electronvolt Ratio. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre. Ledger unit is MeV; this
/// Qty is dimensionless, not SI joule.
pub fn muon_mass_energy_equivalent_in_mev() -> Qty<Dimensionless> {
    Qty::new(105.658_375_5)
}

/// Muon-electron mass ratio m_μ/m_e, CODATA 2018.
///
/// This is the recommended centre from the muon section, not the
/// electron-muon mass ratio and not a certificate that the stored
/// centres invert. The versioned ledger stores the one-sigma hull;
/// this Qty is that centre.
pub fn muon_electron_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(206.768_283_0)
}

/// Muon-proton mass ratio m_μ/m_p, CODATA 2018.
///
/// This is the recommended centre from the muon section, not the
/// electron-proton mass ratio and not a certificate that the stored
/// centres divide. The muon-tau ratio is a PDG reprint and is not
/// stored. The versioned ledger stores the one-sigma hull; this Qty
/// is that centre.
pub fn muon_proton_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(0.112_609_526_4)
}

/// Muon-neutron mass ratio m_μ/m_n, CODATA 2018.
///
/// This is the recommended centre from the muon section, not the
/// electron-neutron mass ratio and not a certificate that the stored
/// centres divide. The muon-tau ratio is a PDG reprint and is not
/// stored. The versioned ledger stores the one-sigma hull; this Qty
/// is that centre.
pub fn muon_neutron_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(0.112_454_517_0)
}

/// Muon molar mass M_μ (kg mol⁻¹), CODATA 2018.
///
/// This is the recommended centre in kg mol⁻¹, not the muon mass in u,
/// not `N_A × m_μ` as a derived product, and not electron molar mass.
/// The versioned ledger stores the one-sigma hull; this Qty is that
/// centre.
pub fn muon_molar_mass() -> Qty<
    physis_core::SI<typenum::P1, typenum::Z0, typenum::Z0, typenum::Z0, typenum::Z0, typenum::N1>,
> {
    Qty::new(1.134_289_259e-4)
}

/// Muon Compton wavelength λ_{C,μ} (m), CODATA 2018.
///
/// This is the recommended centre in metres, not electron Compton and
/// not a certificate of `2π` times a reduced muon Compton wavelength.
/// The reduced muon Compton row is ħ/m_μc and is not stored. The
/// versioned ledger stores the one-sigma hull; this Qty is that
/// centre.
pub fn muon_compton_wavelength() -> Qty<Length> {
    meters(1.173_444_110e-14)
}

/// Muon magnetic moment μ_μ (J T⁻¹ = A m²), CODATA 2018.
///
/// This is the recommended signed centre, not electron magnetic moment
/// and not vacuum permeability. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn muon_magnetic_moment(
) -> Qty<physis_core::SI<typenum::Z0, typenum::P2, typenum::Z0, typenum::P1>> {
    Qty::new(-449_044_830.0 / 1e34)
}

/// Muon magnetic moment to Bohr magneton ratio μ_μ/μ_B, CODATA 2018.
///
/// This is the recommended signed centre, not electron Bohr-magneton
/// ratio and not the muon magnetic moment. The versioned ledger stores
/// the one-sigma hull; this Qty is that centre.
pub fn muon_magnetic_moment_to_bohr_magneton() -> Qty<Dimensionless> {
    Qty::new(-4.841_970_47e-3)
}

/// Muon magnetic moment to nuclear magneton ratio μ_μ/μ_N, CODATA 2018.
///
/// This is the recommended signed centre, not electron nuclear-magneton
/// ratio and not the muon Bohr-magneton ratio. The versioned ledger
/// stores the one-sigma hull; this Qty is that centre.
pub fn muon_magnetic_moment_to_nuclear_magneton() -> Qty<Dimensionless> {
    Qty::new(-8.890_597_03)
}

/// Muon magnetic-moment anomaly a_μ, CODATA 2018.
///
/// This is the recommended centre, not electron anomaly and not the
/// signed Bohr-magneton ratio. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn muon_magnetic_moment_anomaly() -> Qty<Dimensionless> {
    Qty::new(1.165_920_89e-3)
}

/// Muon g-factor g_μ, CODATA 2018.
///
/// This is the recommended signed centre, not electron g-factor and
/// not the muon anomaly. The versioned ledger stores the one-sigma
/// hull; this Qty is that centre.
pub fn muon_g_factor() -> Qty<Dimensionless> {
    Qty::new(-2.002_331_841_8)
}

/// Muon-proton magnetic-moment ratio μ_μ/μ_p, CODATA 2018.
///
/// This is the recommended signed centre, not electron-proton
/// magnetic-moment ratio and not the muon-proton mass ratio. The
/// versioned ledger stores the one-sigma hull; this Qty is that
/// centre.
pub fn muon_proton_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(-3.183_345_142)
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

/// Inverse fine-structure constant α⁻¹ (dimensionless), CODATA 2018.
///
/// This is the zero-momentum recommended centre, not `1/α` as a derived
/// exact value and not the PDG `α_em⁻¹(M_Z)` running value. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre.
pub fn inv_alpha() -> Qty<Dimensionless> {
    Qty::new(137.035_999_084)
}

/// Rydberg constant R∞ (m⁻¹), CODATA 2018.
///
/// This is the recommended centre in inverse metres, not the Rydberg
/// frequency. The versioned ledger stores the one-sigma hull; this Qty
/// is that centre.
pub fn rydberg() -> Qty<physis_core::SI<typenum::Z0, typenum::N1, typenum::Z0>> {
    Qty::new(10_973_731.568_160)
}

/// Rydberg frequency cR∞ (Hz), CODATA 2018.
///
/// This is the recommended centre in hertz. The versioned ledger stores
/// the one-sigma hull; this Qty is that centre.
pub fn rydberg_frequency() -> Qty<Frequency> {
    Qty::new(3.289_841_960_250_8e15)
}

/// Rydberg energy equivalent hcR∞ (J), CODATA 2018.
///
/// This is the recommended centre in joules, not the eV conversion.
/// The versioned ledger stores the one-sigma hull; this Qty is that centre.
pub fn rydberg_energy_equivalent() -> Qty<Energy> {
    joule(2.179_872_361_103_5e-18)
}

/// Bohr radius a₀ (m), CODATA 2018.
///
/// This is the recommended centre, not the Hartree energy. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre.
pub fn bohr_radius() -> Qty<Length> {
    meters(5.291_772_109_03e-11)
}

/// Hartree energy E_h (J), CODATA 2018.
///
/// This is the recommended centre in joules, not the eV conversion.
/// The versioned ledger stores the one-sigma hull; this Qty is that centre.
pub fn hartree_energy() -> Qty<Energy> {
    joule(4.359_744_722_207_1e-18)
}

/// Electron-muon mass ratio m_e/m_μ, CODATA 2018.
///
/// This is the recommended centre, not electron mass. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre.
pub fn electron_muon_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(4.836_331_69e-3)
}

/// Electron-proton mass ratio m_e/m_p, CODATA 2018.
///
/// This is the recommended centre, not electron mass. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre.
pub fn electron_proton_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(5.446_170_214_87e-4)
}

/// Electron-neutron mass ratio m_e/m_n, CODATA 2018.
///
/// This is the recommended centre, not electron mass. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre.
pub fn electron_neutron_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(5.438_673_442_4e-4)
}

/// Electron-deuteron mass ratio m_e/m_d, CODATA 2018.
///
/// This is the recommended centre, not electron mass. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre.
pub fn electron_deuteron_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(2.724_437_107_462e-4)
}

/// Electron-triton mass ratio m_e/m_t, CODATA 2018.
///
/// This is the recommended centre, not electron mass. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre.
pub fn electron_triton_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(1.819_200_062_251e-4)
}

/// Electron-helion mass ratio m_e/m_h, CODATA 2018.
///
/// This is the recommended centre, not electron mass. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre.
pub fn electron_helion_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(1.819_543_074_573e-4)
}

/// Electron to alpha particle mass ratio m_e/m_α, CODATA 2018.
///
/// This is the recommended centre, not electron mass. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre.
pub fn electron_alpha_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(1.370_933_554_787e-4)
}

/// Electron charge to mass quotient −e/m_e (C kg⁻¹), CODATA 2018.
///
/// This is the recommended signed centre, not electron mass and not
/// `e/m_e` from the SI-exact charge. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn electron_charge_to_mass(
) -> Qty<physis_core::SI<typenum::N1, typenum::Z0, typenum::P1, typenum::P1>> {
    Qty::new(-1.758_820_010_76e11)
}

/// Electron molar mass M_e (kg mol⁻¹), CODATA 2018.
///
/// This is the recommended centre in kg mol⁻¹, not electron mass in kg
/// and not the mass-in-u row. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn electron_molar_mass() -> Qty<
    physis_core::SI<typenum::P1, typenum::Z0, typenum::Z0, typenum::Z0, typenum::Z0, typenum::N1>,
> {
    Qty::new(5.485_799_088_8e-7)
}

/// Reduced Compton wavelength ƛ_C (m), CODATA 2018.
///
/// This is the recommended centre, not a certificate of `α a₀` and not
/// the Compton wavelength. The versioned ledger stores the one-sigma
/// hull; this Qty is that centre.
pub fn reduced_compton_wavelength() -> Qty<Length> {
    // IEEE value of the reduced centre Ratio 38615926796/10^23, not extra
    // CODATA digits. The decimal literal 3.861_592_679_6e-13 is one ulp
    // below Ratio::to_f64.
    meters(3.861_592_679_600_000_3e-13)
}

/// Compton wavelength λ_C (m), CODATA 2018.
///
/// This is the recommended centre, not a certificate of `2π ƛ_C` and not
/// the reduced Compton wavelength. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn compton_wavelength() -> Qty<Length> {
    // IEEE value of the reduced centre Ratio 242631023867/10^23, not extra
    // CODATA digits. The decimal literal 2.426_310_238_67e-12 is one ulp
    // below Ratio::to_f64.
    meters(2.426_310_238_670_000_2e-12)
}

/// Classical electron radius r_e (m), CODATA 2018.
///
/// This is the recommended centre, not a certificate of `α² a₀` and not
/// the Thomson cross section. The versioned ledger stores the one-sigma
/// hull; this Qty is that centre.
pub fn classical_electron_radius() -> Qty<Length> {
    // IEEE value of the reduced centre Ratio 28179403262/10^25, not extra
    // CODATA digits. The decimal literal 2.817_940_3262e-15 is one ulp
    // above Ratio::to_f64.
    meters(2.817_940_326_199_999_6e-15)
}

/// Electron magnetic moment μ_e (J T⁻¹ = A m²), CODATA 2018.
///
/// This is the recommended signed centre, not the Thomson cross
/// section. The versioned ledger stores the one-sigma hull; this Qty
/// is that centre.
pub fn electron_magnetic_moment(
) -> Qty<physis_core::SI<typenum::Z0, typenum::P2, typenum::Z0, typenum::P1>> {
    Qty::new(-9.284_764_704_3e-24)
}

/// Electron magnetic moment to Bohr magneton ratio μ_e/μ_B, CODATA 2018.
///
/// This is the recommended signed centre, not the g-factor and not the
/// magnetic-moment anomaly. The versioned ledger stores the one-sigma
/// hull; this Qty is that centre.
pub fn electron_magnetic_moment_to_bohr_magneton() -> Qty<Dimensionless> {
    Qty::new(-1.001_159_652_181_28)
}

/// Electron magnetic moment to nuclear magneton ratio μ_e/μ_N, CODATA 2018.
///
/// This is the recommended signed centre, not the g-factor and not the
/// magnetic-moment anomaly. The versioned ledger stores the one-sigma
/// hull; this Qty is that centre.
pub fn electron_magnetic_moment_to_nuclear_magneton() -> Qty<Dimensionless> {
    Qty::new(-1_838.281_971_88)
}

/// Electron magnetic-moment anomaly a_e, CODATA 2018.
///
/// This is the recommended centre |μ_e|/μ_B − 1, not the signed
/// Bohr-magneton ratio and not the g-factor. The versioned ledger
/// stores the one-sigma hull; this Qty is that centre.
pub fn electron_magnetic_moment_anomaly() -> Qty<Dimensionless> {
    Qty::new(1.159_652_181_28e-3)
}

/// Electron g-factor g_e, CODATA 2018.
///
/// This is the recommended signed centre −2(1 + a_e), not the anomaly
/// and not the signed Bohr-magneton ratio. The versioned ledger stores
/// the one-sigma hull; this Qty is that centre.
pub fn electron_g_factor() -> Qty<Dimensionless> {
    Qty::new(-2.002_319_304_362_56)
}

/// Electron-muon magnetic-moment ratio μ_e/μ_μ, CODATA 2018.
///
/// This is the recommended centre, not the electron-muon mass ratio.
/// The versioned ledger stores the one-sigma hull; this Qty is that
/// centre.
pub fn electron_muon_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(206.766_988_3)
}

/// Electron-proton magnetic-moment ratio μ_e/μ_p, CODATA 2018.
///
/// This is the recommended signed centre, not the electron-proton mass
/// ratio and not the shielded-proton moment ratio. The versioned ledger
/// stores the one-sigma hull; this Qty is that centre.
pub fn electron_proton_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(-658.210_687_89)
}

/// Electron to shielded-proton magnetic-moment ratio μ_e/μ′_p, CODATA 2018.
///
/// This is the recommended signed centre for the proton in spherical
/// H2O at 25 °C, not the free-proton moment ratio. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre.
pub fn electron_to_shielded_proton_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(-658.227_597_1)
}

/// Electron-neutron magnetic-moment ratio μ_e/μ_n, CODATA 2018.
///
/// This is the recommended centre, not the electron-neutron mass ratio.
/// The versioned ledger stores the one-sigma hull; this Qty is that
/// centre.
pub fn electron_neutron_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(960.920_50)
}

/// Electron-deuteron magnetic-moment ratio μ_e/μ_d, CODATA 2018.
///
/// This is the recommended signed centre, not the electron-deuteron mass
/// ratio. The versioned ledger stores the one-sigma hull; this Qty is
/// that centre.
pub fn electron_deuteron_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(-2_143.923_491_5)
}

/// Electron to shielded-helion magnetic-moment ratio μ_e/μ′_h, CODATA 2018.
///
/// This is the recommended centre for the helion in spherical gas at
/// 25 °C, not the electron-helion mass ratio and not the
/// shielded-proton moment ratio. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn electron_to_shielded_helion_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(864.058_257)
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
/// recommended centre. `Y₀` is not a ledger entry.
pub fn epsilon0() -> Qty<physis_core::SI<typenum::N1, typenum::N3, typenum::P4, typenum::P2>> {
    Qty::new(8.854_187_812_8e-12)
}

/// Characteristic impedance of vacuum Z₀ (ohm), CODATA.
/// Units: kg·m²·s⁻³·A⁻².
///
/// After SI 2019 this is the derived value `μ₀ c`, not exact.
/// The versioned ledger stores the one-sigma hull; this Qty is the
/// recommended centre. `Y₀` is not a ledger entry.
pub fn z0() -> Qty<physis_core::SI<typenum::P1, typenum::P2, typenum::N3, typenum::N2>> {
    Qty::new(376.730_313_668)
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
            physis_constants::lookup("epsilon_0").is_none(),
            "epsilon_0 is not a ledger name; the live name is epsilon0"
        );

        let z0_c = physis_constants::vacuum_impedance();
        let z0_centre = Ratio::new(376_730_313_668, 10i128.pow(9));
        assert_eq!(
            z0().value(),
            z0_centre.to_f64(),
            "Z0 Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            z0_c.value.contains(Interval::point(z0_centre)),
            "Z0 Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            z0_c.value.lo, z0_c.value.hi,
            "ledger Z0 stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("Y0").is_none(),
            "Y0 is a different recommended value and is not stored"
        );
        assert!(
            physis_constants::lookup("Z_0").is_none(),
            "Z_0 is not a ledger name; the live name is Z0"
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
            "alpha-inv is not a ledger name; the live name is inv_alpha"
        );

        let inv = physis_constants::inverse_fine_structure_constant();
        let inv_centre = Ratio::new(137_035_999_084, 10i128.pow(9));
        assert_eq!(
            inv_alpha().value(),
            inv_centre.to_f64(),
            "inv_alpha Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            inv.value.contains(Interval::point(inv_centre)),
            "inv_alpha Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            inv.value.lo, inv.value.hi,
            "ledger inv_alpha stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("alpha_inv").is_none(),
            "alpha_inv is not a ledger name; the live name is inv_alpha"
        );
        assert!(
            physis_constants::lookup("inverse-alpha").is_none(),
            "inverse-alpha is not a ledger name; the live name is inv_alpha"
        );

        let rinf = physis_constants::rydberg_constant();
        let rinf_centre = Ratio::new(10_973_731_568_160, 10i128.pow(6));
        assert_eq!(
            rydberg().value(),
            rinf_centre.to_f64(),
            "Rinf Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            rinf.value.contains(Interval::point(rinf_centre)),
            "Rinf Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            rinf.value.lo, rinf.value.hi,
            "ledger Rinf stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("R_inf").is_none(),
            "R_inf is not a ledger name; the live name is Rinf"
        );

        let crinf = physis_constants::rydberg_frequency();
        let crinf_centre = Ratio::int(3_289_841_960_250_800);
        assert_eq!(
            rydberg_frequency().value(),
            crinf_centre.to_f64(),
            "cRinf Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            crinf.value.contains(Interval::point(crinf_centre)),
            "cRinf Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            crinf.value.lo, crinf.value.hi,
            "ledger cRinf stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("c_Rinf").is_none(),
            "c_Rinf is not a ledger name; the live name is cRinf"
        );

        let hcrinf = physis_constants::rydberg_energy_equivalent();
        let hcrinf_centre = Ratio::new(21_798_723_611_035, 10i128.pow(31));
        assert_eq!(
            rydberg_energy_equivalent().value(),
            hcrinf_centre.to_f64(),
            "hcRinf Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            hcrinf.value.contains(Interval::point(hcrinf_centre)),
            "hcRinf Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            hcrinf.value.lo, hcrinf.value.hi,
            "ledger hcRinf stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("hc_Rinf").is_none(),
            "hc_Rinf is not a ledger name; the live name is hcRinf"
        );
        assert!(
            physis_constants::lookup("hcRinf_eV").is_none(),
            "Rydberg energy equivalent in eV is a different recommended value and is not stored"
        );

        let a0_c = physis_constants::bohr_radius();
        let a0_centre = Ratio::new(529_177_210_903, 10i128.pow(22));
        assert_eq!(
            bohr_radius().value(),
            a0_centre.to_f64(),
            "a0 Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            a0_c.value.contains(Interval::point(a0_centre)),
            "a0 Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            a0_c.value.lo, a0_c.value.hi,
            "ledger a0 stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("a_0").is_none(),
            "a_0 is not a ledger name; the live name is a0"
        );
        assert!(
            physis_constants::lookup("E_h").is_none(),
            "E_h is not a ledger name; the live name is Eh"
        );

        let eh_c = physis_constants::hartree_energy();
        let eh_centre = Ratio::new(43_597_447_222_071, 10i128.pow(31));
        assert_eq!(
            hartree_energy().value(),
            eh_centre.to_f64(),
            "Eh Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            eh_c.value.contains(Interval::point(eh_centre)),
            "Eh Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            eh_c.value.lo, eh_c.value.hi,
            "ledger Eh stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("hartree").is_none(),
            "hartree is not a ledger name; the live name is Eh"
        );
        assert!(
            physis_constants::lookup("Eh_eV").is_none(),
            "Hartree energy in eV is a different recommended value and is not stored"
        );

        let me_mmu = physis_constants::electron_muon_mass_ratio();
        let me_mmu_centre = Ratio::new(483_633_169, 10i128.pow(11));
        assert_eq!(
            electron_muon_mass_ratio().value(),
            me_mmu_centre.to_f64(),
            "me_mmu Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            me_mmu.value.contains(Interval::point(me_mmu_centre)),
            "me_mmu Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            me_mmu.value.lo, me_mmu.value.hi,
            "ledger me_mmu stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("me/m_mu").is_none(),
            "me/m_mu is not a ledger name; the live name is me_mmu"
        );

        let me_mp = physis_constants::electron_proton_mass_ratio();
        let me_mp_centre = Ratio::new(544_617_021_487, 10i128.pow(15));
        assert_eq!(
            electron_proton_mass_ratio().value(),
            me_mp_centre.to_f64(),
            "me_mp Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            me_mp.value.contains(Interval::point(me_mp_centre)),
            "me_mp Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            me_mp.value.lo, me_mp.value.hi,
            "ledger me_mp stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("me/m_p").is_none(),
            "me/m_p is not a ledger name; the live name is me_mp"
        );

        let me_mn = physis_constants::electron_neutron_mass_ratio();
        let me_mn_centre = Ratio::new(54_386_734_424, 10i128.pow(14));
        assert_eq!(
            electron_neutron_mass_ratio().value(),
            me_mn_centre.to_f64(),
            "me_mn Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            me_mn.value.contains(Interval::point(me_mn_centre)),
            "me_mn Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            me_mn.value.lo, me_mn.value.hi,
            "ledger me_mn stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("me/m_n").is_none(),
            "me/m_n is not a ledger name; the live name is me_mn"
        );
        let me_md = physis_constants::electron_deuteron_mass_ratio();
        let me_md_centre = Ratio::new(2_724_437_107_462, 10i128.pow(16));
        assert_eq!(
            electron_deuteron_mass_ratio().value(),
            me_md_centre.to_f64(),
            "me_md Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            me_md.value.contains(Interval::point(me_md_centre)),
            "me_md Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            me_md.value.lo, me_md.value.hi,
            "ledger me_md stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("me/m_d").is_none(),
            "me/m_d is not a ledger name; the live name is me_md"
        );
        let me_mt = physis_constants::electron_triton_mass_ratio();
        let me_mt_centre = Ratio::new(1_819_200_062_251, 10i128.pow(16));
        assert_eq!(
            electron_triton_mass_ratio().value(),
            me_mt_centre.to_f64(),
            "me_mt Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            me_mt.value.contains(Interval::point(me_mt_centre)),
            "me_mt Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            me_mt.value.lo, me_mt.value.hi,
            "ledger me_mt stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("me/m_t").is_none(),
            "me/m_t is not a ledger name; the live name is me_mt"
        );
        let me_mh = physis_constants::electron_helion_mass_ratio();
        let me_mh_centre = Ratio::new(1_819_543_074_573, 10i128.pow(16));
        assert_eq!(
            electron_helion_mass_ratio().value(),
            me_mh_centre.to_f64(),
            "me_mh Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            me_mh.value.contains(Interval::point(me_mh_centre)),
            "me_mh Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            me_mh.value.lo, me_mh.value.hi,
            "ledger me_mh stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("me/m_h").is_none(),
            "me/m_h is not a ledger name; the live name is me_mh"
        );
        let me_malpha = physis_constants::electron_alpha_mass_ratio();
        let me_malpha_centre = Ratio::new(1_370_933_554_787, 10i128.pow(16));
        assert_eq!(
            electron_alpha_mass_ratio().value(),
            me_malpha_centre.to_f64(),
            "me_malpha Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            me_malpha.value.contains(Interval::point(me_malpha_centre)),
            "me_malpha Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            me_malpha.value.lo, me_malpha.value.hi,
            "ledger me_malpha stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("me/m_a").is_none(),
            "me/m_a is not a ledger name; the live name is me_malpha"
        );
        let e_me = physis_constants::electron_charge_to_mass();
        let e_me_centre = Ratio::int(-175_882_001_076);
        assert_eq!(
            electron_charge_to_mass().value(),
            e_me_centre.to_f64(),
            "e_me Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            e_me.value.contains(Interval::point(e_me_centre)),
            "e_me Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            e_me.value.lo, e_me.value.hi,
            "ledger e_me stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("-e/me").is_none(),
            "-e/me is not a ledger name; the live name is e_me"
        );
        let molar = physis_constants::electron_molar_mass();
        let molar_centre = Ratio::new(54_857_990_888, 10i128.pow(17));
        assert_eq!(
            electron_molar_mass().value(),
            molar_centre.to_f64(),
            "M_e Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            molar.value.contains(Interval::point(molar_centre)),
            "M_e Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            molar.value.lo, molar.value.hi,
            "ledger M_e stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("Me").is_none(),
            "Me is not a ledger name; the live name is M_e"
        );
        let rcbar = physis_constants::reduced_compton_wavelength();
        let rcbar_centre = Ratio::new(38_615_926_796, 10i128.pow(23));
        assert_eq!(
            reduced_compton_wavelength().value(),
            rcbar_centre.to_f64(),
            "lambdabar_C Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            rcbar.value.contains(Interval::point(rcbar_centre)),
            "lambdabar_C Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            rcbar.value.lo, rcbar.value.hi,
            "ledger lambdabar_C stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("lambdaC").is_none(),
            "lambdaC is not a ledger name; the live name is lambda_C"
        );
        let rc = physis_constants::compton_wavelength();
        let rc_centre = Ratio::new(242_631_023_867, 10i128.pow(23));
        assert_eq!(
            compton_wavelength().value(),
            rc_centre.to_f64(),
            "lambda_C Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            rc.value.contains(Interval::point(rc_centre)),
            "lambda_C Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            rc.value.lo, rc.value.hi,
            "ledger lambda_C stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("r_e").is_none(),
            "r_e is not a ledger name; the live name is re"
        );
        let re = physis_constants::classical_electron_radius();
        let re_centre = Ratio::new(28_179_403_262, 10i128.pow(25));
        assert_eq!(
            classical_electron_radius().value(),
            re_centre.to_f64(),
            "re Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            re.value.contains(Interval::point(re_centre)),
            "re Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            re.value.lo, re.value.hi,
            "ledger re stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("sigma_e").is_none(),
            "Thomson cross section is not a Ratio because it contains pi"
        );
        assert!(
            physis_constants::lookup("mue").is_none(),
            "mue is not a ledger name; the live name is mu_e"
        );
        let mu_e = physis_constants::electron_magnetic_moment();
        let mu_e_centre = Ratio::new(-92_847_647_043, 10i128.pow(34));
        assert_eq!(
            electron_magnetic_moment().value(),
            mu_e_centre.to_f64(),
            "mu_e Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_e.value.contains(Interval::point(mu_e_centre)),
            "mu_e Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_e.value.lo, mu_e.value.hi,
            "ledger mu_e stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("g_e").is_none(),
            "g_e is not a ledger name; the live name is ge"
        );
        assert!(
            physis_constants::lookup("mue_muB").is_none(),
            "mue_muB is not a ledger name; the live name is mu_e_muB"
        );
        let mu_e_mu_b = physis_constants::electron_magnetic_moment_to_bohr_magneton();
        let mu_e_mu_b_centre = Ratio::new(-100_115_965_218_128, 10i128.pow(14));
        assert_eq!(
            electron_magnetic_moment_to_bohr_magneton().value(),
            mu_e_mu_b_centre.to_f64(),
            "mu_e_muB Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_e_mu_b.value.contains(Interval::point(mu_e_mu_b_centre)),
            "mu_e_muB Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_e_mu_b.value.lo, mu_e_mu_b.value.hi,
            "ledger mu_e_muB stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("mue_muN").is_none(),
            "mue_muN is not a ledger name; the live name is mu_e_muN"
        );
        let mu_e_mu_n = physis_constants::electron_magnetic_moment_to_nuclear_magneton();
        let mu_e_mu_n_centre = Ratio::new(-183_828_197_188, 10i128.pow(8));
        assert_eq!(
            electron_magnetic_moment_to_nuclear_magneton().value(),
            mu_e_mu_n_centre.to_f64(),
            "mu_e_muN Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_e_mu_n.value.contains(Interval::point(mu_e_mu_n_centre)),
            "mu_e_muN Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_e_mu_n.value.lo, mu_e_mu_n.value.hi,
            "ledger mu_e_muN stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("a_e").is_none(),
            "a_e is not a ledger name; the live name is ae"
        );
        let ae = physis_constants::electron_magnetic_moment_anomaly();
        let ae_centre = Ratio::new(115_965_218_128, 10i128.pow(14));
        assert_eq!(
            electron_magnetic_moment_anomaly().value(),
            ae_centre.to_f64(),
            "ae Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            ae.value.contains(Interval::point(ae_centre)),
            "ae Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            ae.value.lo, ae.value.hi,
            "ledger ae stays an Interval; the Qty is not that Interval"
        );
        let ge = physis_constants::electron_g_factor();
        let ge_centre = Ratio::new(-200_231_930_436_256, 10i128.pow(14));
        assert_eq!(
            electron_g_factor().value(),
            ge_centre.to_f64(),
            "ge Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            ge.value.contains(Interval::point(ge_centre)),
            "ge Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            ge.value.lo, ge.value.hi,
            "ledger ge stays an Interval; the Qty is not that Interval"
        );
        assert!(
            physis_constants::lookup("mue_mmu").is_none(),
            "mue_mmu is not a ledger name; the live name is mu_e_mmu"
        );
        let mu_e_mmu = physis_constants::electron_muon_magnetic_moment_ratio();
        let mu_e_mmu_centre = Ratio::new(2_067_669_883, 10i128.pow(7));
        assert_eq!(
            electron_muon_magnetic_moment_ratio().value(),
            mu_e_mmu_centre.to_f64(),
            "mu_e_mmu Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_e_mmu.value.contains(Interval::point(mu_e_mmu_centre)),
            "mu_e_mmu Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_e_mmu.value.lo, mu_e_mmu.value.hi,
            "ledger mu_e_mmu stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::electron_muon_magnetic_moment_ratio().hash,
            physis_constants::electron_muon_mass_ratio().hash,
            "mu_e_mmu is not me_mmu"
        );
        assert!(
            physis_constants::lookup("mue_mup").is_none(),
            "mue_mup is not a ledger name; the live name is mu_e_mup"
        );
        let mu_e_mup = physis_constants::electron_proton_magnetic_moment_ratio();
        let mu_e_mup_centre = Ratio::new(-65_821_068_789, 10i128.pow(8));
        assert_eq!(
            electron_proton_magnetic_moment_ratio().value(),
            mu_e_mup_centre.to_f64(),
            "mu_e_mup Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_e_mup.value.contains(Interval::point(mu_e_mup_centre)),
            "mu_e_mup Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_e_mup.value.lo, mu_e_mup.value.hi,
            "ledger mu_e_mup stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::electron_proton_magnetic_moment_ratio().hash,
            physis_constants::electron_proton_mass_ratio().hash,
            "mu_e_mup is not me_mp"
        );
        assert!(
            physis_constants::lookup("mue_mu0p").is_none(),
            "mue_mu0p is not a ledger name; the live name is mu_e_mu0p"
        );
        let mu_e_mu0p = physis_constants::electron_to_shielded_proton_magnetic_moment_ratio();
        let mu_e_mu0p_centre = Ratio::new(-6_582_275_971, 10i128.pow(7));
        assert_eq!(
            electron_to_shielded_proton_magnetic_moment_ratio().value(),
            mu_e_mu0p_centre.to_f64(),
            "mu_e_mu0p Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_e_mu0p.value.contains(Interval::point(mu_e_mu0p_centre)),
            "mu_e_mu0p Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_e_mu0p.value.lo, mu_e_mu0p.value.hi,
            "ledger mu_e_mu0p stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::electron_to_shielded_proton_magnetic_moment_ratio().hash,
            physis_constants::electron_proton_magnetic_moment_ratio().hash,
            "mu_e_mu0p is not mu_e_mup"
        );
        assert_ne!(
            physis_constants::electron_to_shielded_proton_magnetic_moment_ratio().hash,
            physis_constants::vacuum_permeability().hash,
            "mu_e_mu0p is not mu0"
        );
        assert!(
            physis_constants::lookup("mue_mun").is_none(),
            "mue_mun is not a ledger name; the live name is mu_e_mun"
        );
        let mu_e_mun = physis_constants::electron_neutron_magnetic_moment_ratio();
        let mu_e_mun_centre = Ratio::new(96_092_050, 10i128.pow(5));
        assert_eq!(
            electron_neutron_magnetic_moment_ratio().value(),
            mu_e_mun_centre.to_f64(),
            "mu_e_mun Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_e_mun.value.contains(Interval::point(mu_e_mun_centre)),
            "mu_e_mun Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_e_mun.value.lo, mu_e_mun.value.hi,
            "ledger mu_e_mun stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::electron_neutron_magnetic_moment_ratio().hash,
            physis_constants::electron_neutron_mass_ratio().hash,
            "mu_e_mun is not me_mn"
        );
        assert!(
            physis_constants::lookup("mue_mud").is_none(),
            "mue_mud is not a ledger name; the live name is mu_e_mud"
        );
        let mu_e_mud = physis_constants::electron_deuteron_magnetic_moment_ratio();
        let mu_e_mud_centre = Ratio::new(-21_439_234_915, 10i128.pow(7));
        assert_eq!(
            electron_deuteron_magnetic_moment_ratio().value(),
            mu_e_mud_centre.to_f64(),
            "mu_e_mud Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_e_mud.value.contains(Interval::point(mu_e_mud_centre)),
            "mu_e_mud Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_e_mud.value.lo, mu_e_mud.value.hi,
            "ledger mu_e_mud stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::electron_deuteron_magnetic_moment_ratio().hash,
            physis_constants::electron_deuteron_mass_ratio().hash,
            "mu_e_mud is not me_md"
        );
        assert!(
            physis_constants::lookup("mue_mu0h").is_none(),
            "mue_mu0h is not a ledger name; the live name is mu_e_mu0h"
        );
        let mu_e_mu0h = physis_constants::electron_to_shielded_helion_magnetic_moment_ratio();
        let mu_e_mu0h_centre = Ratio::new(864_058_257, 10i128.pow(6));
        assert_eq!(
            electron_to_shielded_helion_magnetic_moment_ratio().value(),
            mu_e_mu0h_centre.to_f64(),
            "mu_e_mu0h Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_e_mu0h.value.contains(Interval::point(mu_e_mu0h_centre)),
            "mu_e_mu0h Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_e_mu0h.value.lo, mu_e_mu0h.value.hi,
            "ledger mu_e_mu0h stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::electron_to_shielded_helion_magnetic_moment_ratio().hash,
            physis_constants::electron_helion_mass_ratio().hash,
            "mu_e_mu0h is not me_mh"
        );
        assert_ne!(
            physis_constants::electron_to_shielded_helion_magnetic_moment_ratio().hash,
            physis_constants::electron_to_shielded_proton_magnetic_moment_ratio().hash,
            "mu_e_mu0h is not mu_e_mu0p"
        );
        assert_ne!(
            physis_constants::electron_to_shielded_helion_magnetic_moment_ratio().hash,
            physis_constants::vacuum_permeability().hash,
            "mu_e_mu0h is not mu0"
        );
        assert!(
            physis_constants::lookup("mmu").is_none(),
            "mmu is not a ledger name; the live name is m_mu"
        );
        let m_mu = physis_constants::muon_mass();
        let m_mu_centre = Ratio::new(1_883_531_627, 10i128.pow(37));
        assert_eq!(
            muon_mass().value(),
            m_mu_centre.to_f64(),
            "m_mu Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_mu.value.contains(Interval::point(m_mu_centre)),
            "m_mu Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_mu.value.lo, m_mu.value.hi,
            "ledger m_mu stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_mass().hash,
            physis_constants::electron_muon_mass_ratio().hash,
            "m_mu is not me_mmu"
        );
        assert_ne!(
            physis_constants::muon_mass().hash,
            physis_constants::proton_mass().hash,
            "m_mu is not m_p"
        );
        assert!(
            physis_constants::lookup("mmu_u").is_none(),
            "mmu_u is not a ledger name; the live name is m_mu_u"
        );
        let m_mu_u = physis_constants::muon_mass_in_u();
        let m_mu_u_centre = Ratio::new(1_134_289_259, 10i128.pow(10));
        assert_eq!(
            muon_mass_in_u().value(),
            m_mu_u_centre.to_f64(),
            "m_mu_u Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_mu_u.value.contains(Interval::point(m_mu_u_centre)),
            "m_mu_u Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_mu_u.value.lo, m_mu_u.value.hi,
            "ledger m_mu_u stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_mass_in_u().hash,
            physis_constants::muon_mass().hash,
            "m_mu_u is not m_mu"
        );
        assert_ne!(
            physis_constants::muon_mass_in_u().hash,
            physis_constants::electron_molar_mass().hash,
            "m_mu_u is not M_e"
        );
        assert_ne!(
            physis_constants::muon_mass_in_u().hash,
            physis_constants::electron_muon_mass_ratio().hash,
            "m_mu_u is not me_mmu"
        );
        assert_ne!(
            physis_constants::muon_mass_in_u().hash,
            physis_constants::proton_mass().hash,
            "m_mu_u is not m_p"
        );
        assert!(
            physis_constants::lookup("mmu_c2").is_none(),
            "mmu_c2 is not a ledger name; the live name is m_mu_c2"
        );
        let m_mu_c2 = physis_constants::muon_mass_energy_equivalent();
        let m_mu_c2_centre = Ratio::new(1_692_833_804, 10i128.pow(20));
        assert_eq!(
            muon_mass_energy_equivalent().value(),
            m_mu_c2_centre.to_f64(),
            "m_mu_c2 Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_mu_c2.value.contains(Interval::point(m_mu_c2_centre)),
            "m_mu_c2 Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_mu_c2.value.lo, m_mu_c2.value.hi,
            "ledger m_mu_c2 stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_mass_energy_equivalent().hash,
            physis_constants::muon_mass().hash,
            "m_mu_c2 is not m_mu"
        );
        assert_ne!(
            physis_constants::muon_mass_energy_equivalent().hash,
            physis_constants::muon_mass_in_u().hash,
            "m_mu_c2 is not m_mu_u"
        );
        assert_ne!(
            physis_constants::muon_mass_energy_equivalent().hash,
            physis_constants::rydberg_energy_equivalent().hash,
            "m_mu_c2 is not hcRinf"
        );
        assert_ne!(
            physis_constants::muon_mass_energy_equivalent().hash,
            physis_constants::hartree_energy().hash,
            "m_mu_c2 is not Eh"
        );
        assert!(
            physis_constants::lookup("mmuc2_MeV").is_none(),
            "mmuc2_MeV is not a ledger name; the live name is m_mu_c2_MeV"
        );
        let m_mu_c2_mev = physis_constants::muon_mass_energy_equivalent_in_mev();
        let m_mu_c2_mev_centre = Ratio::new(1_056_583_755, 10i128.pow(7));
        assert_eq!(
            muon_mass_energy_equivalent_in_mev().value(),
            m_mu_c2_mev_centre.to_f64(),
            "m_mu_c2_MeV Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_mu_c2_mev
                .value
                .contains(Interval::point(m_mu_c2_mev_centre)),
            "m_mu_c2_MeV Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_mu_c2_mev.value.lo, m_mu_c2_mev.value.hi,
            "ledger m_mu_c2_MeV stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_mass_energy_equivalent_in_mev().hash,
            physis_constants::muon_mass_energy_equivalent().hash,
            "m_mu_c2_MeV is not m_mu_c2"
        );
        assert_ne!(
            physis_constants::muon_mass_energy_equivalent_in_mev().hash,
            physis_constants::electron_volt().hash,
            "m_mu_c2_MeV is not eV"
        );
        assert_ne!(
            physis_constants::muon_mass_energy_equivalent_in_mev().hash,
            physis_constants::hartree_energy().hash,
            "m_mu_c2_MeV is not Eh"
        );
        assert!(
            physis_constants::lookup("m_mu_me").is_none(),
            "m_mu_me is not a ledger name; the live name is mmu_me"
        );
        let mmu_me = physis_constants::muon_electron_mass_ratio();
        let mmu_me_centre = Ratio::new(2_067_682_830, 10i128.pow(7));
        assert_eq!(
            muon_electron_mass_ratio().value(),
            mmu_me_centre.to_f64(),
            "mmu_me Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mmu_me.value.contains(Interval::point(mmu_me_centre)),
            "mmu_me Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mmu_me.value.lo, mmu_me.value.hi,
            "ledger mmu_me stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_electron_mass_ratio().hash,
            physis_constants::electron_muon_mass_ratio().hash,
            "mmu_me is not me_mmu"
        );
        assert_ne!(
            physis_constants::muon_electron_mass_ratio().hash,
            physis_constants::electron_muon_magnetic_moment_ratio().hash,
            "mmu_me is not mu_e_mmu"
        );
        assert_ne!(
            physis_constants::muon_electron_mass_ratio().hash,
            physis_constants::muon_mass_energy_equivalent_in_mev().hash,
            "mmu_me is not m_mu_c2_MeV"
        );
        assert!(
            physis_constants::lookup("m_mu_mp").is_none(),
            "m_mu_mp is not a ledger name; the live name is mmu_mp"
        );
        assert!(
            physis_constants::lookup("mmu_mtau").is_none(),
            "muon-tau is a PDG reprint of m_tau c^2 and is not stored"
        );
        let mmu_mp = physis_constants::muon_proton_mass_ratio();
        let mmu_mp_centre = Ratio::new(1_126_095_264, 10i128.pow(10));
        assert_eq!(
            muon_proton_mass_ratio().value(),
            mmu_mp_centre.to_f64(),
            "mmu_mp Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mmu_mp.value.contains(Interval::point(mmu_mp_centre)),
            "mmu_mp Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mmu_mp.value.lo, mmu_mp.value.hi,
            "ledger mmu_mp stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_proton_mass_ratio().hash,
            physis_constants::electron_proton_mass_ratio().hash,
            "mmu_mp is not me_mp"
        );
        assert_ne!(
            physis_constants::muon_proton_mass_ratio().hash,
            physis_constants::muon_electron_mass_ratio().hash,
            "mmu_mp is not mmu_me"
        );
        assert_ne!(
            physis_constants::muon_proton_mass_ratio().hash,
            physis_constants::proton_mass().hash,
            "mmu_mp is not m_p"
        );
        assert!(
            physis_constants::lookup("m_mu_mn").is_none(),
            "m_mu_mn is not a ledger name; the live name is mmu_mn"
        );
        let mmu_mn = physis_constants::muon_neutron_mass_ratio();
        let mmu_mn_centre = Ratio::new(1_124_545_170, 10i128.pow(10));
        assert_eq!(
            muon_neutron_mass_ratio().value(),
            mmu_mn_centre.to_f64(),
            "mmu_mn Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mmu_mn.value.contains(Interval::point(mmu_mn_centre)),
            "mmu_mn Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mmu_mn.value.lo, mmu_mn.value.hi,
            "ledger mmu_mn stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_neutron_mass_ratio().hash,
            physis_constants::electron_neutron_mass_ratio().hash,
            "mmu_mn is not me_mn"
        );
        assert_ne!(
            physis_constants::muon_neutron_mass_ratio().hash,
            physis_constants::muon_proton_mass_ratio().hash,
            "mmu_mn is not mmu_mp"
        );
        assert_ne!(
            physis_constants::muon_neutron_mass_ratio().hash,
            physis_constants::proton_mass().hash,
            "mmu_mn is not m_p"
        );
        assert!(
            physis_constants::lookup("Mmu").is_none(),
            "Mmu is not a ledger name; the live name is M_mu"
        );
        let m_mu_molar = physis_constants::muon_molar_mass();
        let m_mu_molar_centre = Ratio::new(1_134_289_259, 10i128.pow(13));
        assert_eq!(
            muon_molar_mass().value(),
            m_mu_molar_centre.to_f64(),
            "M_mu Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_mu_molar
                .value
                .contains(Interval::point(m_mu_molar_centre)),
            "M_mu Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_mu_molar.value.lo, m_mu_molar.value.hi,
            "ledger M_mu stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_molar_mass().hash,
            physis_constants::muon_mass_in_u().hash,
            "M_mu is not m_mu_u"
        );
        assert_ne!(
            physis_constants::muon_molar_mass().hash,
            physis_constants::electron_molar_mass().hash,
            "M_mu is not M_e"
        );
        assert_ne!(
            physis_constants::muon_molar_mass().hash,
            physis_constants::muon_mass().hash,
            "M_mu is not m_mu"
        );
        assert_ne!(
            physis_constants::muon_molar_mass().hash,
            physis_constants::proton_mass().hash,
            "M_mu is not m_p"
        );
        assert_ne!(
            physis_constants::muon_molar_mass().hash,
            physis_constants::muon_neutron_mass_ratio().hash,
            "M_mu is not mmu_mn"
        );
        assert!(
            physis_constants::lookup("lambda_Cmu").is_none(),
            "lambda_Cmu is not a ledger name; the live name is lambda_C_mu"
        );
        let lambda_c_mu = physis_constants::muon_compton_wavelength();
        let lambda_c_mu_centre = Ratio::new(1_173_444_110, 10i128.pow(23));
        assert_eq!(
            muon_compton_wavelength().value(),
            lambda_c_mu_centre.to_f64(),
            "lambda_C_mu Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            lambda_c_mu
                .value
                .contains(Interval::point(lambda_c_mu_centre)),
            "lambda_C_mu Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            lambda_c_mu.value.lo, lambda_c_mu.value.hi,
            "ledger lambda_C_mu stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_compton_wavelength().hash,
            physis_constants::compton_wavelength().hash,
            "lambda_C_mu is not lambda_C"
        );
        assert_ne!(
            physis_constants::muon_compton_wavelength().hash,
            physis_constants::reduced_compton_wavelength().hash,
            "lambda_C_mu is not lambdabar_C"
        );
        assert_ne!(
            physis_constants::muon_compton_wavelength().hash,
            physis_constants::muon_molar_mass().hash,
            "lambda_C_mu is not M_mu"
        );
        assert_ne!(
            physis_constants::muon_compton_wavelength().hash,
            physis_constants::muon_mass().hash,
            "lambda_C_mu is not m_mu"
        );
        assert_ne!(
            physis_constants::muon_compton_wavelength().hash,
            physis_constants::proton_mass().hash,
            "lambda_C_mu is not m_p"
        );
        assert!(
            physis_constants::lookup("lambdabar_C_mu").is_none(),
            "reduced muon Compton is hbar/m_mu c and is not stored"
        );
        assert!(
            physis_constants::lookup("mumu").is_none(),
            "mumu is not a ledger name; the live name is mu_mu"
        );
        let mu_mu = physis_constants::muon_magnetic_moment();
        let mu_mu_centre = Ratio::new(-449_044_830, 10i128.pow(34));
        assert_eq!(
            muon_magnetic_moment().value(),
            mu_mu_centre.to_f64(),
            "mu_mu Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_mu.value.contains(Interval::point(mu_mu_centre)),
            "mu_mu Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_mu.value.lo, mu_mu.value.hi,
            "ledger mu_mu stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment().hash,
            physis_constants::electron_magnetic_moment().hash,
            "mu_mu is not mu_e"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment().hash,
            physis_constants::electron_muon_magnetic_moment_ratio().hash,
            "mu_mu is not mu_e_mmu"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment().hash,
            physis_constants::vacuum_permeability().hash,
            "mu_mu is not mu0"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment().hash,
            physis_constants::muon_compton_wavelength().hash,
            "mu_mu is not lambda_C_mu"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment().hash,
            physis_constants::proton_mass().hash,
            "mu_mu is not m_p"
        );
        assert!(
            physis_constants::lookup("mumu_muB").is_none(),
            "mumu_muB is not a ledger name; the live name is mu_mu_muB"
        );
        let mu_mu_mu_b = physis_constants::muon_magnetic_moment_to_bohr_magneton();
        let mu_mu_mu_b_centre = Ratio::new(-484_197_047, 10i128.pow(11));
        assert_eq!(
            muon_magnetic_moment_to_bohr_magneton().value(),
            mu_mu_mu_b_centre.to_f64(),
            "mu_mu_muB Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_mu_mu_b
                .value
                .contains(Interval::point(mu_mu_mu_b_centre)),
            "mu_mu_muB Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_mu_mu_b.value.lo, mu_mu_mu_b.value.hi,
            "ledger mu_mu_muB stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::muon_magnetic_moment().hash,
            "mu_mu_muB is not mu_mu"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::electron_magnetic_moment_to_bohr_magneton().hash,
            "mu_mu_muB is not mu_e_muB"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::electron_magnetic_moment().hash,
            "mu_mu_muB is not mu_e"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::vacuum_permeability().hash,
            "mu_mu_muB is not mu0"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::proton_mass().hash,
            "mu_mu_muB is not m_p"
        );
        assert!(
            physis_constants::lookup("mumu_muN").is_none(),
            "mumu_muN is not a ledger name; the live name is mu_mu_muN"
        );
        let mu_mu_mu_n = physis_constants::muon_magnetic_moment_to_nuclear_magneton();
        let mu_mu_mu_n_centre = Ratio::new(-889_059_703, 10i128.pow(8));
        assert_eq!(
            muon_magnetic_moment_to_nuclear_magneton().value(),
            mu_mu_mu_n_centre.to_f64(),
            "mu_mu_muN Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_mu_mu_n
                .value
                .contains(Interval::point(mu_mu_mu_n_centre)),
            "mu_mu_muN Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_mu_mu_n.value.lo, mu_mu_mu_n.value.hi,
            "ledger mu_mu_muN stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::electron_magnetic_moment_to_nuclear_magneton().hash,
            "mu_mu_muN is not mu_e_muN"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::muon_magnetic_moment_to_bohr_magneton().hash,
            "mu_mu_muN is not mu_mu_muB"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::muon_magnetic_moment().hash,
            "mu_mu_muN is not mu_mu"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::electron_magnetic_moment().hash,
            "mu_mu_muN is not mu_e"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::vacuum_permeability().hash,
            "mu_mu_muN is not mu0"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::proton_mass().hash,
            "mu_mu_muN is not m_p"
        );
        assert!(
            physis_constants::lookup("a_mu").is_none(),
            "a_mu is not a ledger name; the live name is amu"
        );
        let amu = physis_constants::muon_magnetic_moment_anomaly();
        let amu_centre = Ratio::new(116_592_089, 10i128.pow(11));
        assert_eq!(
            muon_magnetic_moment_anomaly().value(),
            amu_centre.to_f64(),
            "amu Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            amu.value.contains(Interval::point(amu_centre)),
            "amu Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            amu.value.lo, amu.value.hi,
            "ledger amu stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_anomaly().hash,
            physis_constants::electron_magnetic_moment_anomaly().hash,
            "amu is not ae"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_anomaly().hash,
            physis_constants::electron_g_factor().hash,
            "amu is not ge"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_anomaly().hash,
            physis_constants::muon_magnetic_moment_to_bohr_magneton().hash,
            "amu is not mu_mu_muB"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_anomaly().hash,
            physis_constants::muon_magnetic_moment_to_nuclear_magneton().hash,
            "amu is not mu_mu_muN"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_anomaly().hash,
            physis_constants::muon_magnetic_moment().hash,
            "amu is not mu_mu"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_anomaly().hash,
            physis_constants::vacuum_permeability().hash,
            "amu is not mu0"
        );
        assert_ne!(
            physis_constants::muon_magnetic_moment_anomaly().hash,
            physis_constants::proton_mass().hash,
            "amu is not m_p"
        );
        assert!(
            physis_constants::lookup("g_mu").is_none(),
            "g_mu is not a ledger name; the live name is gmu"
        );
        let gmu = physis_constants::muon_g_factor();
        let gmu_centre = Ratio::new(-20_023_318_418, 10i128.pow(10));
        assert_eq!(
            muon_g_factor().value(),
            gmu_centre.to_f64(),
            "gmu Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            gmu.value.contains(Interval::point(gmu_centre)),
            "gmu Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            gmu.value.lo, gmu.value.hi,
            "ledger gmu stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_g_factor().hash,
            physis_constants::electron_g_factor().hash,
            "gmu is not ge"
        );
        assert_ne!(
            physis_constants::muon_g_factor().hash,
            physis_constants::muon_magnetic_moment_anomaly().hash,
            "gmu is not amu"
        );
        assert_ne!(
            physis_constants::muon_g_factor().hash,
            physis_constants::electron_magnetic_moment_anomaly().hash,
            "gmu is not ae"
        );
        assert_ne!(
            physis_constants::muon_g_factor().hash,
            physis_constants::muon_magnetic_moment_to_bohr_magneton().hash,
            "gmu is not mu_mu_muB"
        );
        assert_ne!(
            physis_constants::muon_g_factor().hash,
            physis_constants::muon_magnetic_moment_to_nuclear_magneton().hash,
            "gmu is not mu_mu_muN"
        );
        assert_ne!(
            physis_constants::muon_g_factor().hash,
            physis_constants::muon_magnetic_moment().hash,
            "gmu is not mu_mu"
        );
        assert_ne!(
            physis_constants::muon_g_factor().hash,
            physis_constants::vacuum_permeability().hash,
            "gmu is not mu0"
        );
        assert_ne!(
            physis_constants::muon_g_factor().hash,
            physis_constants::proton_mass().hash,
            "gmu is not m_p"
        );
        assert!(
            physis_constants::lookup("mumu_mup").is_none(),
            "mumu_mup is not a ledger name; the live name is mu_mu_mup"
        );
        let mu_mu_mup = physis_constants::muon_proton_magnetic_moment_ratio();
        let mu_mu_mup_centre = Ratio::new(-3_183_345_142, 10i128.pow(9));
        assert_eq!(
            muon_proton_magnetic_moment_ratio().value(),
            mu_mu_mup_centre.to_f64(),
            "mu_mu_mup Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_mu_mup.value.contains(Interval::point(mu_mu_mup_centre)),
            "mu_mu_mup Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_mu_mup.value.lo, mu_mu_mup.value.hi,
            "ledger mu_mu_mup stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::muon_proton_magnetic_moment_ratio().hash,
            physis_constants::electron_proton_magnetic_moment_ratio().hash,
            "mu_mu_mup is not mu_e_mup"
        );
        assert_ne!(
            physis_constants::muon_proton_magnetic_moment_ratio().hash,
            physis_constants::muon_proton_mass_ratio().hash,
            "mu_mu_mup is not mmu_mp"
        );
        assert_ne!(
            physis_constants::muon_proton_magnetic_moment_ratio().hash,
            physis_constants::muon_g_factor().hash,
            "mu_mu_mup is not gmu"
        );
        assert_ne!(
            physis_constants::muon_proton_magnetic_moment_ratio().hash,
            physis_constants::muon_magnetic_moment_anomaly().hash,
            "mu_mu_mup is not amu"
        );
        assert_ne!(
            physis_constants::muon_proton_magnetic_moment_ratio().hash,
            physis_constants::muon_magnetic_moment().hash,
            "mu_mu_mup is not mu_mu"
        );
        assert_ne!(
            physis_constants::muon_proton_magnetic_moment_ratio().hash,
            physis_constants::vacuum_permeability().hash,
            "mu_mu_mup is not mu0"
        );
        assert_ne!(
            physis_constants::muon_proton_magnetic_moment_ratio().hash,
            physis_constants::proton_mass().hash,
            "mu_mu_mup is not m_p"
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
            physis_constants::lookup("mp_u").is_none(),
            "mp_u is not a ledger name; the live name is m_p_u"
        );
        let m_p_u = physis_constants::proton_mass_in_u();
        let m_p_u_centre = Ratio::new(1_007_276_466_621, 10i128.pow(12));
        assert_eq!(
            proton_mass_in_u().value(),
            m_p_u_centre.to_f64(),
            "m_p_u Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_p_u.value.contains(Interval::point(m_p_u_centre)),
            "m_p_u Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_p_u.value.lo, m_p_u.value.hi,
            "ledger m_p_u stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_mass_in_u().hash,
            physis_constants::proton_mass().hash,
            "m_p_u is not m_p"
        );
        assert_ne!(
            physis_constants::proton_mass_in_u().hash,
            physis_constants::muon_mass_in_u().hash,
            "m_p_u is not m_mu_u"
        );
        assert_ne!(
            physis_constants::proton_mass_in_u().hash,
            physis_constants::electron_molar_mass().hash,
            "m_p_u is not M_e"
        );
        assert!(
            physis_constants::lookup("mpc2").is_none(),
            "mpc2 is not a ledger name; the live name is m_p_c2"
        );
        let m_p_c2 = physis_constants::proton_mass_energy_equivalent();
        let m_p_c2_centre = Ratio::new(150_327_761_598, 10i128.pow(21));
        assert_eq!(
            proton_mass_energy_equivalent().value(),
            m_p_c2_centre.to_f64(),
            "m_p_c2 Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_p_c2.value.contains(Interval::point(m_p_c2_centre)),
            "m_p_c2 Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_p_c2.value.lo, m_p_c2.value.hi,
            "ledger m_p_c2 stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_mass_energy_equivalent().hash,
            physis_constants::proton_mass().hash,
            "m_p_c2 is not m_p"
        );
        assert_ne!(
            physis_constants::proton_mass_energy_equivalent().hash,
            physis_constants::proton_mass_in_u().hash,
            "m_p_c2 is not m_p_u"
        );
        assert_ne!(
            physis_constants::proton_mass_energy_equivalent().hash,
            physis_constants::muon_mass_energy_equivalent().hash,
            "m_p_c2 is not m_mu_c2"
        );
        assert_ne!(
            physis_constants::proton_mass_energy_equivalent().hash,
            physis_constants::rydberg_energy_equivalent().hash,
            "m_p_c2 is not hcRinf"
        );
        assert!(
            physis_constants::lookup("mpc2_MeV").is_none(),
            "mpc2_MeV is not a ledger name; the live name is m_p_c2_MeV"
        );
        let m_p_c2_mev = physis_constants::proton_mass_energy_equivalent_in_mev();
        let m_p_c2_mev_centre = Ratio::new(93_827_208_816, 10i128.pow(8));
        assert_eq!(
            proton_mass_energy_equivalent_in_mev().value(),
            m_p_c2_mev_centre.to_f64(),
            "m_p_c2_MeV Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_p_c2_mev
                .value
                .contains(Interval::point(m_p_c2_mev_centre)),
            "m_p_c2_MeV Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_p_c2_mev.value.lo, m_p_c2_mev.value.hi,
            "ledger m_p_c2_MeV stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_mass_energy_equivalent_in_mev().hash,
            physis_constants::proton_mass_energy_equivalent().hash,
            "m_p_c2_MeV is not m_p_c2"
        );
        assert_ne!(
            physis_constants::proton_mass_energy_equivalent_in_mev().hash,
            physis_constants::muon_mass_energy_equivalent_in_mev().hash,
            "m_p_c2_MeV is not m_mu_c2_MeV"
        );
        assert_ne!(
            physis_constants::proton_mass_energy_equivalent_in_mev().hash,
            physis_constants::electron_volt().hash,
            "m_p_c2_MeV is not eV"
        );
        assert!(
            physis_constants::lookup("m_p_me").is_none(),
            "m_p_me is not a ledger name; the live name is mp_me"
        );
        let mp_me = physis_constants::proton_electron_mass_ratio();
        let mp_me_centre = Ratio::new(183_615_267_343, 10i128.pow(8));
        assert_eq!(
            proton_electron_mass_ratio().value(),
            mp_me_centre.to_f64(),
            "mp_me Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mp_me.value.contains(Interval::point(mp_me_centre)),
            "mp_me Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mp_me.value.lo, mp_me.value.hi,
            "ledger mp_me stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_electron_mass_ratio().hash,
            physis_constants::electron_proton_mass_ratio().hash,
            "mp_me is not me_mp"
        );
        assert_ne!(
            physis_constants::proton_electron_mass_ratio().hash,
            physis_constants::muon_electron_mass_ratio().hash,
            "mp_me is not mmu_me"
        );
        assert_ne!(
            physis_constants::proton_electron_mass_ratio().hash,
            physis_constants::proton_mass().hash,
            "mp_me is not m_p"
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
