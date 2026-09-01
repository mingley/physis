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

/// Proton-muon mass ratio m_p/m_μ, CODATA 2018.
///
/// This is the recommended centre from the proton section, not the
/// muon-proton mass ratio and not a certificate that the stored
/// centres invert. The proton-tau ratio is a PDG reprint and is not
/// stored. The versioned ledger stores the one-sigma hull; this Qty
/// is that centre.
pub fn proton_muon_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(8.880_243_37)
}

/// Proton-neutron mass ratio m_p/m_n, CODATA 2018.
///
/// This is the recommended centre from the proton section, not the
/// muon-neutron or electron-neutron mass ratios and not a certificate
/// that the stored centres divide. The proton-tau ratio is a PDG
/// reprint and is not stored. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn proton_neutron_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(0.998_623_478_12)
}

/// Proton charge to mass quotient e/m_p (C kg⁻¹), CODATA 2018.
///
/// This is the recommended centre from the proton section, not the
/// electron quotient and not a certificate that this equals e/m_p
/// from the SI-exact charge and the proton-mass hull. The proton-tau
/// ratio is a PDG reprint and is not stored. The versioned ledger
/// stores the one-sigma hull; this Qty is that centre.
pub fn proton_charge_to_mass(
) -> Qty<physis_core::SI<typenum::N1, typenum::Z0, typenum::P1, typenum::P1>> {
    Qty::new(9.578_833_156_0e7)
}

/// Proton molar mass M_p (kg mol⁻¹), CODATA 2018.
///
/// This is the recommended centre in kg mol⁻¹ from the proton section,
/// not electron or muon molar mass, not the mass-in-u row, and not a
/// certificate that this equals N_A times the proton-mass hull. The
/// proton-tau ratio is a PDG reprint and is not stored. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre.
pub fn proton_molar_mass() -> Qty<
    physis_core::SI<typenum::P1, typenum::Z0, typenum::Z0, typenum::Z0, typenum::Z0, typenum::N1>,
> {
    Qty::new(1.007_276_466_27e-3)
}

/// Proton Compton wavelength λ_{C,p} (m), CODATA 2018.
///
/// This is the recommended centre in metres from the proton section, not
/// electron Compton, not muon Compton, and not a certificate of `2π`
/// times a reduced proton Compton wavelength. The reduced proton Compton
/// row is ħ/m_p c and is not stored. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn proton_compton_wavelength() -> Qty<Length> {
    meters(1.321_409_855_39e-15)
}

/// Proton rms charge radius r_p (m), CODATA 2018.
///
/// This is the recommended centre in metres from the proton section, not
/// classical electron radius and not a deuteron radius. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre.
pub fn proton_rms_charge_radius() -> Qty<Length> {
    meters(8.414e-16)
}

/// Proton magnetic moment μ_p (J T⁻¹ = A m²), CODATA 2018.
///
/// This is the recommended signed centre from the proton section, not
/// electron or muon magnetic moment and not vacuum permeability. The
/// shielded proton moment is a later table row stored as mu0p. The
/// versioned ledger stores the one-sigma hull; this Qty is that centre.
pub fn proton_magnetic_moment(
) -> Qty<physis_core::SI<typenum::Z0, typenum::P2, typenum::Z0, typenum::P1>> {
    Qty::new(1.410_606_797_36e-26)
}

/// Proton magnetic moment to Bohr magneton ratio μ_p/μ_B, CODATA 2018.
///
/// This is the recommended signed centre from the proton section, not
/// electron or muon Bohr-magneton ratio and not the proton magnetic
/// moment. The versioned ledger stores the one-sigma hull; this Qty is
/// that centre.
pub fn proton_magnetic_moment_to_bohr_magneton() -> Qty<Dimensionless> {
    Qty::new(1.521_032_202_30e-3)
}

/// Proton magnetic moment to nuclear magneton ratio μ_p/μ_N, CODATA 2018.
///
/// This is the recommended signed centre from the proton section, not
/// electron or muon nuclear-magneton ratio and not the proton
/// Bohr-magneton ratio. The versioned ledger stores the one-sigma hull;
/// this Qty is that centre.
pub fn proton_magnetic_moment_to_nuclear_magneton() -> Qty<Dimensionless> {
    Qty::new(2.792_847_344_63)
}

/// Proton g-factor g_p, CODATA 2018.
///
/// This is the recommended signed centre from the proton section, not
/// electron or muon g-factor and not the proton nuclear-magneton
/// ratio. This Qty is not a certificate that it equals 2 μp/μN. The
/// shielded proton g-factor is a later table row and is not stored.
/// The versioned ledger stores the one-sigma hull; this Qty is that
/// centre.
pub fn proton_g_factor() -> Qty<Dimensionless> {
    Qty::new(5.585_694_689_3)
}

/// Proton-neutron magnetic-moment ratio μ_p/μ_n, CODATA 2018.
///
/// This is the recommended signed centre from the proton section, not
/// the electron-neutron magnetic-moment ratio and not the
/// proton-neutron mass ratio. This Qty is not a certificate that it
/// equals a reconstructed μp/μn from sibling moments. The
/// neutron-proton magnetic-moment ratio is `mu_n_mup`. The shielded
/// proton moment is a later table row
/// stored as mu0p. The versioned ledger stores the one-sigma hull;
/// this Qty is that centre.
pub fn proton_neutron_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(-1.459_898_05)
}

/// Shielded proton magnetic moment μ′_p, CODATA 2018.
///
/// This is the recommended signed centre for the proton in spherical
/// H2O at 25 °C, not the free proton moment and not vacuum
/// permeability. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre.
pub fn shielded_proton_magnetic_moment(
) -> Qty<physis_core::SI<typenum::Z0, typenum::P2, typenum::Z0, typenum::P1>> {
    Qty::new(1.410_570_560e-26)
}

/// Shielded proton magnetic moment to Bohr magneton ratio μ′_p/μ_B,
/// CODATA 2018.
///
/// This is the recommended signed centre for the proton in spherical
/// H2O at 25 °C, not the free proton Bohr-magneton ratio and not the
/// shielded proton moment. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn shielded_proton_magnetic_moment_to_bohr_magneton() -> Qty<Dimensionless> {
    Qty::new(1.520_993_128e-3)
}

/// Shielded proton magnetic moment to nuclear magneton ratio μ′_p/μ_N,
/// CODATA 2018.
///
/// This is the recommended signed centre for the proton in spherical
/// H2O at 25 °C, not the free proton nuclear-magneton ratio and not the
/// shielded Bohr-magneton ratio. This Qty is not a certificate that it
/// equals gp/2. Gyromagnetic ratios cite ħ and are not stored. Shielded
/// g-factor g0p is a glossary identity, not a table XXXI recommended
/// hull, and is not stored. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn shielded_proton_magnetic_moment_to_nuclear_magneton() -> Qty<Dimensionless> {
    Qty::new(2.792_775_599)
}

/// Proton magnetic shielding correction σ₀p, CODATA 2018.
///
/// This is the recommended signed centre for the proton in spherical
/// H2O at 25 °C, not the shielded proton moment and not vacuum
/// permeability. This Qty is not a certificate of the reconstruction
/// 1 − μ′_p/μ_p from sibling moments. Gyromagnetic ratios cite ħ and
/// are not stored. Shielded g-factor g0p is a glossary identity, not a
/// table XXXI recommended hull, and is not stored. The versioned ledger
/// stores the one-sigma hull; this Qty is that centre.
pub fn proton_magnetic_shielding_correction() -> Qty<Dimensionless> {
    Qty::new(2.568_9e-5)
}

/// Neutron mass, CODATA 2018.
///
/// This is the recommended kg centre from the neutron section, not
/// proton mass and not muon mass. This Qty is not a certificate of a
/// reconstruction from sibling masses or mass ratios. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre.
pub fn neutron_mass() -> Qty<Mass> {
    kg(1.674_927_498_04e-27)
}

/// Neutron mass in unified atomic mass units, CODATA 2018.
///
/// This is the recommended centre in u from the neutron section, not
/// the kg hull and not proton or muon mass in u. This Qty is not a
/// certificate of a reconstruction from sibling masses. Ledger unit is
/// u; this Qty is dimensionless, not kg. The versioned ledger stores
/// the one-sigma hull; this Qty is that centre.
pub fn neutron_mass_in_u() -> Qty<Dimensionless> {
    Qty::new(1.008_664_915_95)
}

/// Neutron mass energy equivalent m_n c² (J), CODATA 2018.
///
/// This is the recommended centre in joules from the neutron section,
/// not the kg hull, not the u-row, not proton or muon joule hulls, and
/// not the MeV conversion. This Qty is not a certificate of a
/// reconstruction from sibling masses. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn neutron_mass_energy_equivalent() -> Qty<Energy> {
    joule(1.505_349_762_87e-10)
}

/// Neutron mass energy equivalent in MeV, CODATA 2018.
///
/// This is the recommended centre in MeV from the neutron section, not
/// the joule hull, not proton or muon MeV, and not the exact
/// electronvolt Ratio. This Qty is not a certificate of a
/// reconstruction from sibling masses. Ledger unit is MeV; this Qty is
/// dimensionless, not SI joule. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn neutron_mass_energy_equivalent_in_mev() -> Qty<Dimensionless> {
    Qty::new(939.565_420_52)
}

/// Neutron-electron mass ratio m_n/m_e, CODATA 2018.
///
/// This is the recommended centre from the neutron section, not the
/// electron-neutron mass ratio and not a certificate that the stored
/// centres invert. The neutron-tau ratio is a PDG reprint and is not
/// stored. The versioned ledger stores the one-sigma hull; this Qty
/// is that centre.
pub fn neutron_electron_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(1_838.683_661_73)
}

/// Neutron-muon mass ratio m_n/m_μ, CODATA 2018.
///
/// This is the recommended centre from the neutron section, not the
/// muon-neutron mass ratio and not a certificate that the stored
/// centres invert. The neutron-tau ratio is a PDG reprint and is not
/// stored. The versioned ledger stores the one-sigma hull; this Qty
/// is that centre.
pub fn neutron_muon_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(8.892_484_06)
}

/// Neutron-proton mass ratio m_n/m_p, CODATA 2018.
///
/// This is the recommended centre from the neutron section, not the
/// proton-neutron mass ratio and not a certificate that the stored
/// centres invert. The neutron-tau ratio is a PDG reprint and is not
/// stored. The versioned ledger stores the one-sigma hull; this Qty
/// is that centre.
pub fn neutron_proton_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(1.001_378_419_31)
}

/// Neutron-proton mass difference m_n − m_p, CODATA 2018.
///
/// This is the recommended kg centre from the neutron section, not
/// neutron mass, not proton mass, and not a certificate that stored
/// centres subtract. The u-row and energy equivalents are later table
/// rows and are not stored. The versioned ledger stores the one-sigma
/// hull; this Qty is that centre as the CODATA decimal. Ratio::to_f64
/// on the 10^38 centre is one ulp above this decimal and is not this
/// Qty.
pub fn neutron_proton_mass_difference() -> Qty<Mass> {
    kg(2.305_574_35e-30)
}

/// Neutron-proton mass difference in unified atomic mass units, CODATA 2018.
///
/// This is the recommended centre in u from the neutron section, not
/// the kg hull and not neutron or proton mass in u. This Qty is not a
/// certificate of a reconstruction from sibling masses. Ledger unit is
/// u; this Qty is dimensionless, not kg. The versioned ledger stores
/// the one-sigma hull; this Qty is that centre.
pub fn neutron_proton_mass_difference_in_u() -> Qty<Dimensionless> {
    Qty::new(1.388_449_33e-3)
}

/// Neutron-proton mass difference energy equivalent (m_n − m_p)c², CODATA 2018.
///
/// This is the recommended centre in joules from the neutron section,
/// not the kg hull, not the u-row, not neutron or proton joule hulls,
/// and not the MeV conversion. This Qty is not a certificate of a
/// reconstruction from sibling masses. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn neutron_proton_mass_difference_energy_equivalent() -> Qty<Energy> {
    joule(2.072_146_89e-13)
}

/// Neutron-proton mass difference energy equivalent in MeV, CODATA 2018.
///
/// This is the recommended centre in MeV from the neutron section, not
/// the joule hull, not neutron or proton MeV, and not the exact
/// electronvolt Ratio. This Qty is not a certificate of a
/// reconstruction from sibling masses. Ledger unit is MeV; this Qty is
/// dimensionless, not SI joule. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn neutron_proton_mass_difference_energy_equivalent_in_mev() -> Qty<Dimensionless> {
    Qty::new(1.293_332_36)
}

/// Neutron molar mass M_n (kg mol⁻¹), CODATA 2018.
///
/// This is the recommended centre in kg mol⁻¹ from the neutron section,
/// not proton, electron, or muon molar mass, not the kg hull, not the
/// u-row, and not a certificate that this equals N_A times the
/// neutron-mass hull. The neutron-tau ratio is a PDG reprint and is not
/// stored. The versioned ledger stores the one-sigma hull; this Qty is
/// that centre.
pub fn neutron_molar_mass() -> Qty<
    physis_core::SI<typenum::P1, typenum::Z0, typenum::Z0, typenum::Z0, typenum::Z0, typenum::N1>,
> {
    Qty::new(1.008_664_915_60e-3)
}

/// Neutron Compton wavelength λ_{C,n} (m), CODATA 2018.
///
/// This is the recommended centre in metres from the neutron section,
/// not electron, proton, or muon Compton, and not a certificate of
/// `2π` times a reduced neutron Compton wavelength. The reduced neutron
/// Compton row is ħ/m_n c and is not stored. The versioned ledger
/// stores the one-sigma hull; this Qty is that centre as the CODATA
/// decimal. `Ratio::to_f64` on the `10^26` centre is one ulp below this
/// decimal and is not this Qty.
pub fn neutron_compton_wavelength() -> Qty<Length> {
    meters(1.319_590_905_81e-15)
}

/// Neutron magnetic moment μ_n (J T⁻¹ = A m²), CODATA 2018.
///
/// This is the recommended signed centre from the neutron section, not
/// proton, electron, or muon magnetic moment and not vacuum
/// permeability. This Qty is not a certificate that it equals
/// g_n μ_N / 2. Bohr, nuclear, g-factor, and moment-ratio rows are
/// later table rows and are not stored. The versioned ledger stores
/// the one-sigma hull; this Qty is that centre.
pub fn neutron_magnetic_moment(
) -> Qty<physis_core::SI<typenum::Z0, typenum::P2, typenum::Z0, typenum::P1>> {
    Qty::new(-9.662_365_1e-27)
}

/// Neutron magnetic moment to Bohr magneton ratio μ_n/μ_B, CODATA 2018.
///
/// This is the recommended signed centre from the neutron section, not
/// proton, electron, or muon Bohr-magneton ratio and not the neutron
/// magnetic moment. This Qty is not a certificate that it equals a
/// reconstructed μ_n/μ_B from sibling moments. The nuclear-magneton
/// ratio is `mu_n_muN`. G-factor and moment-ratio rows are later table
/// rows and are not stored. The versioned ledger stores the one-sigma
/// hull; this Qty is that centre.
pub fn neutron_magnetic_moment_to_bohr_magneton() -> Qty<Dimensionless> {
    Qty::new(-1.041_875_63e-3)
}

/// Neutron magnetic moment to nuclear magneton ratio μ_n/μ_N, CODATA 2018.
///
/// This is the recommended signed centre from the neutron section, not
/// proton, electron, or muon nuclear-magneton ratio and not the neutron
/// Bohr-magneton ratio or magnetic moment. This Qty is not a certificate
/// that it equals 2 μ_n/μ_N (the g-factor). The g-factor is `gn`.
/// Moment-ratio rows are later table rows and are not stored. The
/// versioned ledger stores the one-sigma hull; this Qty is that centre.
pub fn neutron_magnetic_moment_to_nuclear_magneton() -> Qty<Dimensionless> {
    Qty::new(-1.913_042_73)
}

/// Neutron g-factor g_n, CODATA 2018.
///
/// This is the recommended signed centre from the neutron section, not
/// electron, muon, or proton g-factor and not the neutron
/// nuclear-magneton ratio. This Qty is not a certificate that it equals
/// 2 μ_n/μ_N. The neutron-electron magnetic-moment ratio is `mu_n_mue`.
/// Neutron-proton and shielded-proton moment-ratio rows are later table
/// rows and are not stored. The versioned ledger stores the one-sigma
/// hull; this Qty is that centre. This is not the CODATA 2022 last-digit
/// 52.
pub fn neutron_g_factor() -> Qty<Dimensionless> {
    Qty::new(-3.826_085_45)
}

/// Neutron-electron magnetic-moment ratio μ_n/μ_e, CODATA 2018.
///
/// This is the recommended centre from the neutron section, not the
/// electron-neutron magnetic-moment ratio and not the neutron-electron
/// mass ratio. This Qty is not a certificate that it equals the inverse
/// of μ_e/μ_n. The neutron-proton magnetic-moment ratio is `mu_n_mup`.
/// Shielded-proton moment-ratio is a later table row and is not stored.
/// The versioned ledger stores the one-sigma hull; this Qty is that
/// centre. This is not the CODATA 2022 last-digit 84.
pub fn neutron_electron_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(1.040_668_82e-3)
}

/// Neutron-proton magnetic-moment ratio μ_n/μ_p, CODATA 2018.
///
/// This is the recommended signed centre from the neutron section, not
/// the proton-neutron magnetic-moment ratio and not the neutron-proton
/// mass ratio. This Qty is not a certificate that it equals the inverse
/// of μ_p/μ_n. The neutron to shielded-proton moment-ratio is
/// `mu_n_mu0p`. The versioned ledger stores the one-sigma hull; this Qty
/// is that centre. This is not the CODATA 2022 last-digit 35.
pub fn neutron_proton_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(-0.684_979_34)
}

/// Neutron to shielded-proton magnetic-moment ratio μ_n/μ′_p, CODATA 2018.
///
/// This is the recommended signed centre from the neutron section for
/// the proton in spherical H2O at 25 °C, not the free neutron-proton
/// magnetic-moment ratio and not the electron to shielded-proton ratio.
/// This Qty is not a certificate that it equals a reconstructed
/// μ_n/μ′_p from sibling moments. Gyromagnetic ratios cite ħ and are
/// not stored. Deuteron mass is `m_d`. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
pub fn neutron_to_shielded_proton_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(-0.684_996_94)
}

/// Deuteron mass, CODATA 2018.
///
/// This is the recommended kg centre from the deuteron section, not
/// neutron, proton, or muon mass. This Qty is not a certificate of a
/// reconstruction from sibling masses or mass ratios. The u-row is
/// `m_d_u`. Energy equivalent is `m_d_c2`. MeV, molar mass, and rms
/// charge radius are later table rows and are not stored. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre. This is
/// not the CODATA 2022 last-digit 7768.
pub fn deuteron_mass() -> Qty<Mass> {
    kg(3.343_583_772_4e-27)
}

/// Deuteron mass in unified atomic mass units, CODATA 2018.
///
/// This is the recommended centre in u from the deuteron section, not
/// the kg hull and not neutron, proton, or muon mass in u. This Qty is
/// not a certificate of a reconstruction from sibling masses. Ledger
/// unit is u; this Qty is dimensionless, not kg. Relative atomic mass
/// is not stored under a different name. The versioned ledger stores
/// the one-sigma hull; this Qty is that centre. This is not the CODATA
/// 2022 last-digit 544.
pub fn deuteron_mass_in_u() -> Qty<Dimensionless> {
    Qty::new(2.013_553_212_745)
}

/// Deuteron mass energy equivalent m_d c² (J), CODATA 2018.
///
/// This is the recommended centre in joules from the deuteron section,
/// not the kg hull, not the u-row, not neutron, proton, or muon joule
/// hulls. The MeV conversion is `m_d_c2_MeV`. This Qty is not a
/// certificate of a reconstruction from sibling masses. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre. This is
/// not the CODATA 2022 last-digit 23491.
pub fn deuteron_mass_energy_equivalent() -> Qty<Energy> {
    joule(3.005_063_231_02e-10)
}

/// Deuteron mass energy equivalent in MeV, CODATA 2018.
///
/// This is the recommended centre in MeV from the deuteron section, not
/// the joule hull, not neutron, proton, or muon MeV, and not the exact
/// electronvolt Ratio. This Qty is not a certificate of a reconstruction
/// from sibling masses. Ledger unit is MeV; this Qty is dimensionless,
/// not SI joule. The versioned ledger stores the one-sigma hull; this
/// Qty is that centre. This is not the CODATA 2022 last-digit 94500.
pub fn deuteron_mass_energy_equivalent_in_mev() -> Qty<Dimensionless> {
    Qty::new(1_875.612_942_57)
}

/// Deuteron-electron mass ratio m_d/m_e, CODATA 2018.
///
/// This is the recommended centre from the deuteron section, not the
/// electron-deuteron mass ratio and not a certificate that the stored
/// centres invert. The deuteron-proton mass ratio is `md_mp`. Molar
/// mass and rms charge radius are later table rows and are not stored.
/// The versioned ledger stores the one-sigma hull; this Qty is that
/// centre. This is not the CODATA 2022 last-digit 655.
pub fn deuteron_electron_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(3_670.482_967_88)
}

/// Deuteron-proton mass ratio m_d/m_p, CODATA 2018.
///
/// This is the recommended centre from the deuteron section, not the
/// neutron-proton mass ratio, not the proton-neutron mass ratio, and
/// not a certificate that the stored centres reconstruct m_d/m_p.
/// The molar mass is `M_d`. The rms charge radius is a later table
/// row and is not stored. The versioned ledger stores the one-sigma
/// hull; this Qty is that centre. This is not the CODATA 2022
/// last-digit 2699.
pub fn deuteron_proton_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(1.999_007_501_39)
}

/// Deuteron molar mass M_d (kg mol⁻¹), CODATA 2018.
///
/// This is the recommended centre in kg mol⁻¹ from the deuteron
/// section, not neutron, proton, electron, or muon molar mass, not the
/// kg hull, not the u-row, and not a certificate that this equals N_A
/// times the deuteron-mass hull. The rms charge radius is `rd`.
/// Magnetic-moment rows are later table rows and are not stored. The
/// versioned ledger stores the one-sigma hull; this Qty is that
/// centre. This is not the CODATA 2022 last-digit 466.
pub fn deuteron_molar_mass() -> Qty<
    physis_core::SI<typenum::P1, typenum::Z0, typenum::Z0, typenum::Z0, typenum::Z0, typenum::N1>,
> {
    Qty::new(2.013_553_212_05e-3)
}

/// Deuteron rms charge radius r_d (m), CODATA 2018.
///
/// This is the recommended centre in metres from the deuteron section,
/// not proton rms charge radius and not classical electron radius.
/// This Qty is not a certificate of a deuteron-proton radius
/// difference. The magnetic moment is `mu_d`. Bohr-magneton and later
/// moment rows are later table rows and are not stored. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre. This is
/// not the CODATA 2022 last-digit 78.
pub fn deuteron_rms_charge_radius() -> Qty<Length> {
    meters(2.127_99e-15)
}

/// Deuteron magnetic moment μ_d (J T⁻¹ = A m²), CODATA 2018.
///
/// This is the recommended signed centre from the deuteron section, not
/// proton, neutron, electron, or muon magnetic moment and not vacuum
/// permeability. This Qty is not a certificate that it equals
/// g_d μ_N / 2 and is not the electron-deuteron moment ratio. The
/// Bohr-magneton ratio is `mu_d_muB`. Nuclear-magneton and later moment
/// rows are later table rows and are not stored. The versioned ledger
/// stores the one-sigma hull; this Qty is that centre. This is not the
/// CODATA 2022 last-digit 7.
pub fn deuteron_magnetic_moment(
) -> Qty<physis_core::SI<typenum::Z0, typenum::P2, typenum::Z0, typenum::P1>> {
    Qty::new(4.330_735_094e-27)
}

/// Deuteron magnetic moment to Bohr magneton ratio μ_d/μ_B, CODATA 2018.
///
/// This is the recommended signed centre from the deuteron section, not
/// proton, neutron, electron, or muon Bohr-magneton ratio and not the
/// deuteron magnetic moment. This Qty is not a certificate that it
/// equals a reconstructed μ_d/μ_B from sibling moments. The
/// nuclear-magneton ratio is `mu_d_muN`. The g-factor is `gd`. The
/// versioned ledger stores the one-sigma hull; this Qty is that
/// centre. This is not the CODATA 2022 last-digit 8.
pub fn deuteron_magnetic_moment_to_bohr_magneton() -> Qty<Dimensionless> {
    Qty::new(4.669_754_570e-4)
}

/// Deuteron magnetic moment to nuclear magneton ratio μ_d/μ_N, CODATA 2018.
///
/// This is the recommended signed centre from the deuteron section, not
/// proton, neutron, electron, or muon nuclear-magneton ratio and not
/// the deuteron Bohr-magneton ratio or magnetic moment. This Qty is not
/// a certificate that it equals the g-factor gd. The g-factor is gd.
/// Moment-ratio rows are later table rows and are not stored. The
/// versioned ledger stores the one-sigma hull; this Qty is that
/// centre. This is not the CODATA 2022 last-digit 5.
pub fn deuteron_magnetic_moment_to_nuclear_magneton() -> Qty<Dimensionless> {
    Qty::new(0.857_438_233_8)
}

/// Deuteron g-factor g_d, CODATA 2018.
///
/// This is the recommended signed centre from the deuteron section, not
/// electron, muon, proton, or neutron g-factor and not the deuteron
/// nuclear-magneton ratio. This Qty is not a certificate that it equals
/// μ_d/μ_N. JPCRD prints the same recommended digits as mu_d_muN
/// because I = 1; each row has its own Claim identity. The
/// deuteron-electron magnetic-moment ratio is `mu_d_mue`. Proton and
/// neutron moment-ratio rows are later table rows and are not stored.
/// The versioned ledger stores the one-sigma hull; this Qty is that
/// centre. This is not the CODATA 2022 last-digit 5.
pub fn deuteron_g_factor() -> Qty<Dimensionless> {
    Qty::new(0.857_438_233_8)
}

/// Deuteron-electron magnetic-moment ratio μ_d/μ_e, CODATA 2018.
///
/// This is the recommended signed centre from the deuteron section, not
/// the electron-deuteron magnetic-moment ratio and not the
/// neutron-electron magnetic-moment ratio. This Qty is not a
/// certificate that it equals the inverse of μ_e/μ_d. The
/// deuteron-proton magnetic-moment ratio is `mu_d_mup`. The
/// deuteron-neutron magnetic-moment ratio is `mu_d_mun`. Triton rows
/// are later table rows and are not stored. The versioned ledger
/// stores the one-sigma hull; this Qty is that centre. This is not the
/// CODATA 2022 last-digit 0.
pub fn deuteron_electron_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(-4.664_345_551e-4)
}

/// Deuteron-proton magnetic-moment ratio μ_d/μ_p, CODATA 2018.
///
/// This is the recommended centre from the deuteron section, not the
/// neutron-proton or electron-proton magnetic-moment ratio and not the
/// deuteron-proton mass ratio. This Qty is not a certificate that it
/// equals a reconstructed μ_d/μ_p from sibling moments. The
/// deuteron-neutron magnetic-moment ratio is `mu_d_mun`. Triton rows
/// are later table rows and are not stored. The versioned ledger
/// stores the one-sigma hull; this Qty is that centre. This is not the
/// CODATA 2022 last-digit 0.
pub fn deuteron_proton_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(0.307_012_209_39)
}

/// Deuteron-neutron magnetic-moment ratio μ_d/μ_n, CODATA 2018.
///
/// This is the recommended signed centre from the deuteron section, not
/// the electron-neutron or proton-neutron magnetic-moment ratio and not
/// the deuteron nuclear-magneton ratio. This Qty is not a certificate
/// that it equals a reconstructed μ_d/μ_n from sibling moments. The
/// live name `mu_d_mun` is not a case-variant of `mu_d_muN`. Triton
/// mass is `m_t`. Later Triton rows are not stored. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre. This is
/// not the CODATA 2022 last-digit 2.
pub fn deuteron_neutron_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(-0.448_206_53)
}

/// Triton mass m_t (kg), CODATA 2018.
///
/// This is the recommended kg centre from the triton section, not
/// deuteron, neutron, proton, or muon mass and not the electron-triton
/// mass ratio. This Qty is not a certificate of a reconstruction from
/// sibling masses or mass ratios. The u-row is `m_t_u`. Energy
/// equivalent is `m_t_c2`. The MeV conversion is `m_t_c2_MeV`. The
/// triton-electron mass ratio is `mt_me`. The triton-proton mass
/// ratio is `mt_mp`. The molar mass is `M_t`. The magnetic moment is
/// `mu_t`. The Bohr-magneton ratio is `mu_t_muB`. The nuclear-magneton
/// ratio is `mu_t_muN`. The g-factor is `gt`. The
/// versioned ledger stores the one-sigma hull; this Qty is that
/// centre. This is not the CODATA 2022 last-digit 7512.
pub fn triton_mass() -> Qty<Mass> {
    kg(5.007_356_744_6e-27)
}

/// Triton mass in unified atomic mass units, CODATA 2018.
///
/// This is the recommended centre in u from the triton section, not the
/// kg hull and not deuteron, neutron, proton, or muon mass in u. This
/// Qty is not a certificate of a reconstruction from sibling masses.
/// Ledger unit is u; this Qty is dimensionless, not kg. Relative atomic
/// mass is not stored under a different name. Energy equivalent is
/// `m_t_c2`. The MeV conversion is `m_t_c2_MeV`. The triton-electron
/// mass ratio is `mt_me`. The triton-proton mass ratio is `mt_mp`.
/// The molar mass is `M_t`. The magnetic moment is `mu_t`.
/// The Bohr-magneton ratio is `mu_t_muB`. The nuclear-magneton ratio
/// is `mu_t_muN`. The g-factor is `gt`.
/// The
/// versioned ledger stores the one-sigma hull; this Qty is that
/// centre. This is not the CODATA 2022 last-digit 597.
pub fn triton_mass_in_u() -> Qty<Dimensionless> {
    Qty::new(3.015_500_716_21)
}

/// Triton mass energy equivalent m_t c² (J), CODATA 2018.
///
/// This is the recommended centre in joules from the triton section,
/// not the kg hull, not the u-row, not deuteron, neutron, proton, or
/// muon joule hulls. This Qty is not a certificate of a reconstruction
/// from sibling masses. Ledger unit is J. The MeV conversion is
/// `m_t_c2_MeV`. The triton-electron mass ratio is `mt_me`. The
/// triton-proton mass ratio is `mt_mp`. The molar mass is `M_t`.
/// The magnetic moment is `mu_t`. The Bohr-magneton ratio is
/// `mu_t_muB`. The nuclear-magneton ratio is `mu_t_muN`. The g-factor
/// is `gt`. The versioned ledger stores the one-sigma hull;
/// this Qty is that centre. This is not the CODATA 2022 last-digit 8119.
pub fn triton_mass_energy_equivalent() -> Qty<Energy> {
    joule(4.500_387_806_0e-10)
}

/// Triton mass energy equivalent in MeV, CODATA 2018.
///
/// This is the recommended centre in MeV from the triton section, not
/// the joule hull, not deuteron, neutron, proton, or muon MeV, and not
/// the exact electronvolt Ratio. This Qty is not a certificate of a
/// reconstruction from sibling masses. Ledger unit is MeV; this Qty is
/// dimensionless, not SI joule. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre. This is not the CODATA
/// 2022 last-digit 13668. The triton-electron mass ratio is `mt_me`.
pub fn triton_mass_energy_equivalent_in_mev() -> Qty<Dimensionless> {
    Qty::new(2_808.921_132_98)
}

/// Triton-electron mass ratio m_t/m_e, CODATA 2018.
///
/// This is the recommended centre from the triton section, not the
/// electron-triton mass ratio and not a certificate that the stored
/// centres invert. The triton-proton mass ratio is `mt_mp`. Molar mass
/// is a later table row and is not stored. The versioned ledger stores
/// the one-sigma hull; this Qty is that centre. This is not the CODATA
/// 2022 last-digit 53551.
pub fn triton_electron_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(5_496.921_535_73)
}

/// Triton-proton mass ratio m_t/m_p, CODATA 2018.
///
/// This is the recommended centre from the triton section, not the
/// deuteron-proton, neutron-proton, or proton-neutron mass ratio, and
/// not a certificate that the stored centres reconstruct m_t/m_p.
/// The molar mass is `M_t`. The magnetic moment is `mu_t`.
/// The Bohr-magneton ratio is `mu_t_muB`. The nuclear-magneton ratio
/// is `mu_t_muN`. The g-factor is `gt`.
/// The versioned
/// ledger stores the one-sigma hull; this Qty is that centre. This is
/// not the CODATA 2022 last-digit 03403.
pub fn triton_proton_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(2.993_717_034_14)
}

/// Triton molar mass M_t (kg mol⁻¹), CODATA 2018.
///
/// This is the recommended centre in kg mol⁻¹ from the triton
/// section, not neutron, proton, electron, or muon molar mass, not the
/// kg hull, not the u-row, and not a certificate that this equals N_A
/// times the triton-mass hull. The magnetic moment is `mu_t`.
/// The Bohr-magneton ratio is `mu_t_muB`. The nuclear-magneton ratio
/// is `mu_t_muN`. The g-factor is `gt`.
/// The
/// versioned ledger stores the one-sigma hull; this Qty is that centre.
/// This is not the CODATA 2022 last-digit 71913.
pub fn triton_molar_mass() -> Qty<
    physis_core::SI<typenum::P1, typenum::Z0, typenum::Z0, typenum::Z0, typenum::Z0, typenum::N1>,
> {
    Qty::new(3.015_500_715_17e-3)
}

/// Triton magnetic moment μ_t (J T⁻¹ = A m²), CODATA 2018.
///
/// This is the recommended signed centre from the triton section, not
/// deuteron, proton, neutron, electron, or muon magnetic moment and not
/// vacuum permeability. This Qty is not a certificate that it equals
/// g_t μ_N / 2. The Bohr-magneton ratio is `mu_t_muB`. The
/// nuclear-magneton ratio is `mu_t_muN`. The g-factor is `gt`. The
/// versioned ledger stores the one-sigma hull; this Qty is that centre.
/// This is not the CODATA 2022 last-digit 5178.
pub fn triton_magnetic_moment(
) -> Qty<physis_core::SI<typenum::Z0, typenum::P2, typenum::Z0, typenum::P1>> {
    Qty::new(1.504_609_520_2e-26)
}

/// Triton magnetic moment to Bohr magneton ratio μ_t/μ_B, CODATA 2018.
///
/// This is the recommended signed centre from the triton section, not
/// deuteron, proton, neutron, electron, or muon Bohr-magneton ratio and
/// not the triton magnetic moment. This Qty is not a certificate that
/// it equals a reconstructed μ_t/μ_B from sibling moments. The
/// nuclear-magneton ratio is `mu_t_muN`. The g-factor is `gt`. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre. This is
/// not the CODATA 2022 last-digit 6648.
pub fn triton_magnetic_moment_to_bohr_magneton() -> Qty<Dimensionless> {
    Qty::new(1.622_393_665_1e-3)
}

/// Triton magnetic moment to nuclear magneton ratio μ_t/μ_N, CODATA 2018.
///
/// This is the recommended signed centre from the triton section, not
/// deuteron, proton, neutron, electron, or muon nuclear-magneton ratio
/// and not the triton magnetic moment or Bohr-magneton ratio. This Qty
/// is not a certificate that it equals a reconstructed μ_t/μ_N from
/// sibling moments and not a certificate that it equals the g-factor
/// gt. The g-factor is `gt`. JPCRD prints different digits from
/// `mu_t_muN` because I = 1/2; each row has its own Claim identity.
/// Helion mass is `m_h`. Later Helion rows are not stored. The
/// versioned ledger stores the one-sigma hull; this Qty is that
/// centre. This is not the CODATA 2022 last-digit 4650.
pub fn triton_magnetic_moment_to_nuclear_magneton() -> Qty<Dimensionless> {
    Qty::new(2.978_962_465_6)
}

/// Triton g-factor g_t, CODATA 2018.
///
/// This is the recommended signed centre from the triton section, not
/// electron, muon, proton, neutron, or deuteron g-factor and not the
/// triton nuclear-magneton ratio. This Qty is not a certificate that
/// it equals 2 μ_t/μ_N from sibling moments. JPCRD prints different
/// digits from mu_t_muN because I = 1/2; each row has its own Claim
/// identity. Helion mass is `m_h`. Later Helion rows are not stored. The
/// versioned ledger stores the one-sigma hull; this Qty is that
/// centre. This is not the CODATA 2022 last-digit 930.
pub fn triton_g_factor() -> Qty<Dimensionless> {
    Qty::new(5.957_924_931)
}

/// Helion mass m_h (kg), CODATA 2018.
///
/// This is the recommended kg centre from the helion section, not
/// triton, deuteron, neutron, proton, or muon mass and not the
/// electron-helion mass ratio. This Qty is not a certificate of a
/// reconstruction from sibling masses or mass ratios. The u-row is
/// `m_h_u`. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre. This is not the CODATA
/// 2022 last-digit 7862.
pub fn helion_mass() -> Qty<Mass> {
    kg(5.006_412_779_6e-27)
}

/// Helion mass in unified atomic mass units, CODATA 2018.
///
/// This is the recommended centre in u from the helion section, not the
/// kg hull and not triton, deuteron, neutron, proton, or muon mass in u.
/// This Qty is not a certificate of a reconstruction from sibling
/// masses. Ledger unit is u; this Qty is dimensionless, not kg. Relative
/// atomic mass is not stored under a different name. Energy equivalent
/// is `m_h_c2`. The versioned ledger stores
/// the one-sigma hull; this Qty is that centre. This is not the CODATA
/// 2022 last-digit 932.
pub fn helion_mass_in_u() -> Qty<Dimensionless> {
    Qty::new(3.014_932_247_175)
}

/// Helion mass energy equivalent m_h c² (J), CODATA 2018.
///
/// This is the recommended centre in joules from the helion section,
/// not the kg hull, not the u-row, not triton, deuteron, neutron,
/// proton, or muon joule hulls. This Qty is not a certificate of a
/// reconstruction from sibling masses. Ledger unit is J. The MeV
/// conversion is `m_h_c2_MeV`. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre. This is
/// not the CODATA 2022 last-digit 4185.
pub fn helion_mass_energy_equivalent() -> Qty<Energy> {
    joule(4.499_539_412_5e-10)
}

/// Helion mass energy equivalent in MeV, CODATA 2018.
///
/// This is the recommended centre in MeV from the helion section, not
/// the joule hull, not triton, deuteron, neutron, proton, or muon MeV,
/// and not the exact electronvolt Ratio. This Qty is not a certificate
/// of a reconstruction from sibling masses. Ledger unit is MeV; this
/// Qty is dimensionless, not SI joule. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre. This is not the CODATA
/// 2022 last-digit 61112. The helion-electron mass ratio is `mh_me`.
pub fn helion_mass_energy_equivalent_in_mev() -> Qty<Dimensionless> {
    Qty::new(2_808.391_607_43)
}

/// Helion-electron mass ratio m_h/m_e, CODATA 2018.
///
/// This is the recommended centre from the helion section, not the
/// electron-helion mass ratio and not a certificate that the stored
/// centres invert. The helion-proton mass ratio is `mh_mp`. The
/// versioned ledger stores the one-sigma hull; this Qty is that centre.
/// This is not the CODATA 2022 last-digit 27984.
pub fn helion_electron_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(5_495.885_280_07)
}

/// Helion-proton mass ratio m_h/m_p, CODATA 2018.
///
/// This is the recommended centre from the helion section, not the
/// triton-proton, deuteron-proton, neutron-proton, or proton-neutron
/// mass ratio, and not a certificate that the stored centres reconstruct
/// m_h/m_p. The molar mass is `M_h`. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre. This is not the CODATA 2022
/// last-digit 671552.
pub fn helion_proton_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(2.993_152_671_67)
}

/// Helion molar mass M_h (kg mol⁻¹), CODATA 2018.
///
/// This is the recommended centre from the helion section, not triton,
/// deuteron, neutron, proton, electron, or muon molar mass, not the kg
/// hull, not the u-row, and not a certificate that this equals N_A times
/// the helion-mass hull. The magnetic moment is `mu_h`. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre. This is
/// not the CODATA 2022 last-digit 25010.
pub fn helion_molar_mass() -> Qty<
    physis_core::SI<typenum::P1, typenum::Z0, typenum::Z0, typenum::Z0, typenum::Z0, typenum::N1>,
> {
    Qty::new(3.014_932_246_13e-3)
}

/// Helion magnetic moment μ_h (J T⁻¹ = A m²), CODATA 2018.
///
/// This is the recommended signed centre from the helion section, not
/// triton, deuteron, neutron, proton, electron, or muon magnetic moment
/// and not vacuum permeability. This Qty is not a certificate that it
/// equals g_h μ_N / 2. The Bohr-magneton ratio is `mu_h_muB`. The
/// nuclear-magneton ratio is `mu_h_muN`. The g-factor is `gh`. The
/// shielded magnetic moment is `mu0h`. The
/// versioned ledger stores the one-sigma hull; this
/// Qty is that centre as the CODATA decimal. `Ratio::to_f64` on the
/// `10^{35}` centre is one ulp below this decimal and is not this Qty.
/// This is not the CODATA 2022 last-digit 55198.
pub fn helion_magnetic_moment(
) -> Qty<physis_core::SI<typenum::Z0, typenum::P2, typenum::Z0, typenum::P1>> {
    Qty::new(-1.074_617_532e-26)
}

/// Helion magnetic moment to Bohr magneton ratio μ_h/μ_B, CODATA 2018.
///
/// This is the recommended signed centre from the helion section, not
/// triton, deuteron, neutron, proton, electron, or muon Bohr-magneton
/// ratio and not the helion magnetic moment. This Qty is not a
/// certificate that it equals a reconstructed μ_h/μ_B from sibling
/// moments. The nuclear-magneton ratio is `mu_h_muN`. The g-factor is
/// `gh`. The versioned ledger stores
/// the one-sigma hull; this Qty is
/// that centre. This is not the CODATA 2022 last-digit 98083.
pub fn helion_magnetic_moment_to_bohr_magneton() -> Qty<Dimensionless> {
    Qty::new(-1.158_740_958e-3)
}

/// Helion magnetic moment to nuclear magneton ratio μ_h/μ_N, CODATA 2018.
///
/// This is the recommended signed centre from the helion section, not
/// triton, deuteron, proton, neutron, electron, or muon nuclear-magneton
/// ratio and not the helion magnetic moment or Bohr-magneton ratio.
/// This Qty is not a certificate that it equals a reconstructed μ_h/μ_N
/// from sibling moments and not a certificate that it equals the
/// g-factor gh. JPCRD prints different digits from mu_h_muN because
/// I = 1/2; each row has its own Claim identity. The g-factor is `gh`.
/// The versioned ledger stores the
/// one-sigma hull; this Qty is that centre. This is not the CODATA 2022
/// last-digit 3498.
pub fn helion_magnetic_moment_to_nuclear_magneton() -> Qty<Dimensionless> {
    Qty::new(-2.127_625_307)
}

/// Helion g-factor g_h, CODATA 2018.
///
/// This is the recommended signed centre from the helion section, not
/// electron, muon, proton, neutron, deuteron, or triton g-factor and not
/// the helion nuclear-magneton ratio. This Qty is not a certificate that
/// it equals 2 μ_h/μ_N from sibling moments. JPCRD prints different
/// digits from mu_h_muN because I = 1/2; each row has its own Claim
/// identity. The shielded magnetic moment is `mu0h`. The versioned
/// ledger stores the one-sigma hull; this Qty is
/// that centre. This is not the CODATA 2022 last-digit 6995.
pub fn helion_g_factor() -> Qty<Dimensionless> {
    Qty::new(-4.255_250_615)
}

/// Shielded helion magnetic moment μ′_h (J T⁻¹ = A m²), CODATA 2018.
///
/// This is the recommended signed centre for the helion in a spherical
/// gas sample at 25 °C, not the free helion moment, not shielded proton
/// moment, not the electron to shielded-helion moment ratio, and not
/// vacuum permeability. The Bohr-magneton ratio is `mu0h_muB`. The
/// nuclear-magneton ratio is a later table row and is not stored.
/// Gyromagnetic ratios cite ħ and are not stored.
/// The versioned ledger stores the one-sigma hull; this Qty is that
/// centre as the CODATA decimal. `Ratio::to_f64` on the `10^{35}`
/// centre is one ulp from this decimal and is not this Qty. This is
/// not the CODATA 2022 last-digit 11035.
pub fn shielded_helion_magnetic_moment(
) -> Qty<physis_core::SI<typenum::Z0, typenum::P2, typenum::Z0, typenum::P1>> {
    Qty::new(-1.074_553_090e-26)
}

/// Shielded helion magnetic moment to Bohr magneton ratio μ′_h/μ_B,
/// CODATA 2018.
///
/// This is the recommended signed centre for the helion in a spherical
/// gas sample at 25 °C, not the free helion Bohr-magneton ratio, not
/// shielded helion magnetic moment, not shielded proton Bohr-magneton
/// ratio, and not vacuum permeability. This Qty is not a certificate
/// that it equals a reconstructed μ′_h/μ_B from sibling moments. The
/// nuclear-magneton ratio is `mu0h_muN`. Shielded helion to proton
/// ratio rows are later table rows and are not stored.
/// Gyromagnetic ratios cite ħ and are not stored. The versioned ledger
/// stores the one-sigma hull; this Qty is that centre. This is not the
/// CODATA 2022 last-digit 49457.
pub fn shielded_helion_magnetic_moment_to_bohr_magneton() -> Qty<Dimensionless> {
    Qty::new(-1.158_671_471e-3)
}

/// Shielded helion magnetic moment to nuclear magneton ratio μ′_h/μ_N,
/// CODATA 2018.
///
/// This is the recommended signed centre for the helion in a spherical
/// gas sample at 25 °C, not the free helion nuclear-magneton ratio, not
/// the shielded Bohr-magneton ratio, and not helion g-factor gh. This
/// Qty is not a certificate that it equals gh/2 or a reconstructed
/// μ′_h/μ_N from sibling moments. JPCRD prints different digits from
/// mu_h_muN because this is the shielded row. The shielded helion to
/// proton ratio is `mu0h_mup`. The shielded helion to shielded proton
/// ratio is a later table row and is not stored. Gyromagnetic
/// ratios cite ħ and are not stored. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre. This is not the CODATA 2022
/// last-digit 7624.
pub fn shielded_helion_magnetic_moment_to_nuclear_magneton() -> Qty<Dimensionless> {
    Qty::new(-2.127_497_719)
}

/// Shielded helion to proton magnetic-moment ratio μ′_h/μ_p, CODATA 2018.
///
/// This is the recommended signed centre for the helion in a spherical
/// gas sample at 25 °C, not the free helion nuclear-magneton ratio, not
/// neutron-proton, electron-proton, or deuteron-proton moment ratios,
/// and not helion-proton mass ratio. This Qty is not a certificate that
/// it equals a reconstructed μ′_h/μ_p from sibling moments. The
/// shielded helion to shielded proton ratio is `mu0h_mu0p`.
/// Gyromagnetic ratios cite ħ and are not stored. The
/// versioned ledger stores the one-sigma hull; this Qty is that centre.
/// This is not the CODATA 2022 last-digit 57721.
pub fn shielded_helion_to_proton_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(-0.761_766_561_8)
}

/// Shielded helion to shielded proton magnetic-moment ratio μ′_h/μ′_p,
/// CODATA 2018.
///
/// This is the recommended signed centre for the helion in a spherical
/// gas sample versus spherical H2O at 25 °C, not the free-proton
/// ratio, not shielded proton magnetic moment, and not neutron or
/// electron to shielded-proton moment ratios. This Qty is not a
/// certificate that it equals a reconstructed μ′_h/μ′_p from sibling
/// moments. Gyromagnetic ratios cite ħ and are not stored. The alpha
/// particle mass is `m_alpha`. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre.
/// This is not the CODATA 2022 last-digit 1334.
pub fn shielded_helion_to_shielded_proton_magnetic_moment_ratio() -> Qty<Dimensionless> {
    Qty::new(-0.761_786_131_3)
}

/// Alpha particle mass m_α (kg), CODATA 2018.
///
/// This is the recommended kg centre from the alpha-particle section,
/// not helion, triton, deuteron, neutron, proton, or muon mass and not
/// the electron-alpha mass ratio. This Qty is not a certificate of a
/// reconstruction from sibling masses or mass ratios. The u-row is
/// `m_alpha_u`. Energy equivalent is `m_alpha_c2`. Mass ratios and molar
/// mass are later table rows and are not stored. The versioned ledger stores the
/// one-sigma hull; this Qty is that centre. This is not the CODATA 2022
/// last-digit 3450.
pub fn alpha_particle_mass() -> Qty<Mass> {
    kg(6.644_657_335_7e-27)
}

/// Alpha particle mass in unified atomic mass units, CODATA 2018.
///
/// This is the recommended centre in u from the alpha-particle section,
/// not the kg hull and not helion, triton, deuteron, neutron, proton, or
/// muon mass in u. This Qty is not a certificate of a reconstruction
/// from sibling masses. Ledger unit is u; this Qty is dimensionless, not
/// kg. Relative atomic mass is not stored under a different name. Energy
/// equivalent is `m_alpha_c2`. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre. This is
/// not the CODATA 2022 last-digit 129.
pub fn alpha_particle_mass_in_u() -> Qty<Dimensionless> {
    Qty::new(4.001_506_179_127)
}

/// Alpha particle mass energy equivalent m_α c² (J), CODATA 2018.
///
/// This is the recommended centre in joules from the alpha-particle
/// section, not the kg hull, not the u-row, not helion, triton,
/// deuteron, neutron, proton, or muon joule hulls. This Qty is not a
/// certificate of a reconstruction from sibling masses. Ledger unit is
/// J. The MeV conversion is `m_alpha_c2_MeV`. The
/// versioned ledger stores the one-sigma hull; this Qty is that centre.
/// This is not the CODATA 2022 last-digit 1997.
pub fn alpha_particle_mass_energy_equivalent() -> Qty<Energy> {
    joule(5.971_920_191_4e-10)
}

/// Alpha particle mass energy equivalent in MeV, CODATA 2018.
///
/// This is the recommended centre in MeV from the alpha-particle
/// section, not the joule hull, not helion, triton, deuteron, neutron,
/// proton, or muon MeV, and not the exact electronvolt Ratio. This Qty
/// is not a certificate of a reconstruction from sibling masses. Ledger
/// unit is MeV; this Qty is dimensionless, not SI joule. The versioned
/// ledger stores the one-sigma hull; this Qty is that centre. This is
/// not the CODATA 2022 last-digit 4118. The alpha-electron mass ratio is
/// `malpha_me`.
pub fn alpha_particle_mass_energy_equivalent_in_mev() -> Qty<Dimensionless> {
    Qty::new(3_727.379_406_6)
}

/// Alpha particle-electron mass ratio m_α/m_e, CODATA 2018.
///
/// This is the recommended centre from the alpha-particle section, not
/// the electron-alpha mass ratio and not a certificate that the stored
/// centres invert. The alpha-proton mass ratio is `malpha_mp`.
/// The versioned ledger stores the one-sigma hull; this Qty is that
/// centre. This is not the CODATA 2022 last-digit 71.
pub fn alpha_particle_electron_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(7_294.299_541_42)
}

/// Alpha particle-proton mass ratio m_α/m_p, CODATA 2018.
///
/// This is the recommended centre from the alpha-particle section, not
/// the helion-proton, triton-proton, deuteron-proton, neutron-proton,
/// or proton-neutron mass ratio, and not a certificate that the stored
/// centres reconstruct m_alpha/m_p. The molar mass is `M_alpha`.
/// The versioned ledger stores the one-sigma hull; this Qty is
/// that centre. This is not the CODATA 2022 last-digit 252.
pub fn alpha_particle_proton_mass_ratio() -> Qty<Dimensionless> {
    Qty::new(3.972_599_690_09)
}

/// Alpha particle molar mass M_α (kg mol⁻¹), CODATA 2018.
///
/// This is the recommended centre from the alpha-particle section, not
/// helion, triton, deuteron, neutron, proton, electron, or muon molar
/// mass, not the kg hull, not the u-row, and not a certificate that this
/// equals N_A times the alpha-particle-mass hull. Relative atomic mass is
/// the same digits as the u-row and is not stored under a second name.
/// The versioned ledger stores the one-sigma hull; this Qty is that
/// centre. This is not the CODATA 2022 last-digit 1833.
pub fn alpha_particle_molar_mass() -> Qty<
    physis_core::SI<typenum::P1, typenum::Z0, typenum::Z0, typenum::Z0, typenum::Z0, typenum::N1>,
> {
    Qty::new(4.001_506_177_7e-3)
}

/// Atomic mass constant m_u (kg), CODATA 2018.
///
/// This is the recommended centre from the PHYSICOCHEMICAL section, not
/// proton, neutron, deuteron, triton, helion, alpha-particle, or muon
/// mass, not alpha-particle molar mass, not Avogadro N_A, not vacuum
/// permeability, and not the unified atomic mass unit stored under a
/// second name. The versioned ledger stores the one-sigma hull; this Qty
/// is that centre. This is not the CODATA 2022 last-digit 892.
pub fn atomic_mass_constant() -> Qty<Mass> {
    kg(1.660_539_066_60e-27)
}

/// Atomic mass constant energy equivalent m_u c² (J), CODATA 2018.
///
/// This is the recommended centre from the PHYSICOCHEMICAL section, not
/// the kg hull, not proton, neutron, deuteron, triton, helion,
/// alpha-particle, or muon energy equivalent, not Hartree, and not the
/// exact electronvolt Ratio. The MeV conversion is `m_u_c2_MeV`.
/// The versioned ledger stores the one-sigma hull; this Qty is that
/// centre. This is not the CODATA 2022 last-digit 768.
pub fn atomic_mass_constant_energy_equivalent() -> Qty<Energy> {
    joule(1.492_418_085_60e-10)
}

/// Atomic mass constant energy equivalent in MeV, CODATA 2018.
///
/// This is the recommended centre in MeV from the PHYSICOCHEMICAL
/// section, not the joule hull, not proton, neutron, deuteron, triton,
/// helion, alpha-particle, or muon MeV, not Hartree, and not the exact
/// electronvolt Ratio. Ledger unit is MeV; this Qty is dimensionless,
/// not SI joule. The versioned ledger stores the one-sigma hull; this
/// Qty is that centre. This is not the CODATA 2022 last-digit 372.
/// The molar mass constant is `M_u`.
pub fn atomic_mass_constant_energy_equivalent_in_mev() -> Qty<Dimensionless> {
    Qty::new(931.494_102_42)
}

/// Molar mass constant M_u (kg mol⁻¹), CODATA 2018.
///
/// This is the recommended centre from the PHYSICOCHEMICAL section, not
/// alpha-particle, helion, triton, deuteron, neutron, proton, electron,
/// or muon molar mass, not the kg hull, not Avogadro N_A, and not a
/// certificate that this equals N_A times the atomic-mass-constant hull.
/// The versioned ledger stores the one-sigma hull; this Qty is that
/// centre. This is not the CODATA 2022 last-digit 105.
/// The molar mass of carbon-12 is `M_12C`.
pub fn molar_mass_constant() -> Qty<
    physis_core::SI<typenum::P1, typenum::Z0, typenum::Z0, typenum::Z0, typenum::Z0, typenum::N1>,
> {
    Qty::new(0.999_999_999_65e-3)
}

/// Molar mass of carbon-12 M(¹²C) (kg mol⁻¹), CODATA 2018.
///
/// This is the recommended centre from the PHYSICOCHEMICAL section, not
/// the molar mass constant, not alpha-particle, helion, triton,
/// deuteron, neutron, proton, electron, or muon molar mass, not the kg
/// hull, not Avogadro N_A, and not a certificate that this equals 12
/// times M_u. The versioned ledger stores the one-sigma hull; this Qty
/// is that centre. This is not the CODATA 2022 last-digit 126.
/// The molar Planck constant is `NAh`.
pub fn carbon_12_molar_mass() -> Qty<
    physis_core::SI<typenum::P1, typenum::Z0, typenum::Z0, typenum::Z0, typenum::Z0, typenum::N1>,
> {
    Qty::new(11.999_999_995_8e-3)
}

/// Molar Planck constant N_A h (J Hz⁻¹ mol⁻¹), SI 2019 exact.
///
/// This is the exact PHYSICOCHEMICAL product listed as NAh, not Planck
/// h, not Avogadro N_A, not ħ, not Hartree, and not a FormalClaim that
/// reconstructs N_A times h. The table prints an ellipsis; the ledger
/// stores the full terminating decimal. The versioned ledger stores the
/// exact Ratio; this Qty is the IEEE rounding of that SI decimal. The
/// molar gas constant is `NAk`.
pub fn molar_planck_constant() -> Qty<
    physis_core::SI<typenum::P1, typenum::P2, typenum::N1, typenum::Z0, typenum::Z0, typenum::N1>,
> {
    Qty::new(3.990_312_712_893_431_4e-10)
}

/// Molar gas constant N_A k (J mol⁻¹ K⁻¹), SI 2019 exact.
///
/// This is the exact PHYSICOCHEMICAL product listed as NAk, not
/// Boltzmann k, not Avogadro N_A, not molar Planck NAh, not Hartree,
/// and not a FormalClaim that reconstructs N_A times k. The table
/// prints an ellipsis; the ledger stores the full terminating decimal.
/// The versioned ledger stores the exact Ratio; this Qty is the IEEE
/// rounding of that SI decimal. Faraday constant is a later table row
/// and is not stored. JPCRD also writes R; that is not a ledger name.
pub fn molar_gas_constant() -> Qty<
    physis_core::SI<typenum::P1, typenum::P2, typenum::N2, typenum::Z0, typenum::N1, typenum::N1>,
> {
    Qty::new(8.314_462_618_153_24)
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
            physis_constants::lookup("m_p_mmu").is_none(),
            "m_p_mmu is not a ledger name; the live name is mp_mmu"
        );
        let mp_mmu = physis_constants::proton_muon_mass_ratio();
        let mp_mmu_centre = Ratio::new(888_024_337, 10i128.pow(8));
        assert_eq!(
            proton_muon_mass_ratio().value(),
            mp_mmu_centre.to_f64(),
            "mp_mmu Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mp_mmu.value.contains(Interval::point(mp_mmu_centre)),
            "mp_mmu Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mp_mmu.value.lo, mp_mmu.value.hi,
            "ledger mp_mmu stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_muon_mass_ratio().hash,
            physis_constants::muon_proton_mass_ratio().hash,
            "mp_mmu is not mmu_mp"
        );
        assert_ne!(
            physis_constants::proton_muon_mass_ratio().hash,
            physis_constants::proton_electron_mass_ratio().hash,
            "mp_mmu is not mp_me"
        );
        assert_ne!(
            physis_constants::proton_muon_mass_ratio().hash,
            physis_constants::proton_mass().hash,
            "mp_mmu is not m_p"
        );
        assert!(
            physis_constants::lookup("mp_mtau").is_none(),
            "proton-tau is a PDG reprint and is not stored"
        );
        assert!(
            physis_constants::lookup("m_p_mn").is_none(),
            "m_p_mn is not a ledger name; the live name is mp_mn"
        );
        let mp_mn = physis_constants::proton_neutron_mass_ratio();
        let mp_mn_centre = Ratio::new(99_862_347_812, 10i128.pow(11));
        assert_eq!(
            proton_neutron_mass_ratio().value(),
            mp_mn_centre.to_f64(),
            "mp_mn Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mp_mn.value.contains(Interval::point(mp_mn_centre)),
            "mp_mn Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mp_mn.value.lo, mp_mn.value.hi,
            "ledger mp_mn stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_neutron_mass_ratio().hash,
            physis_constants::muon_neutron_mass_ratio().hash,
            "mp_mn is not mmu_mn"
        );
        assert_ne!(
            physis_constants::proton_neutron_mass_ratio().hash,
            physis_constants::electron_neutron_mass_ratio().hash,
            "mp_mn is not me_mn"
        );
        assert_ne!(
            physis_constants::proton_neutron_mass_ratio().hash,
            physis_constants::proton_muon_mass_ratio().hash,
            "mp_mn is not mp_mmu"
        );
        assert!(
            physis_constants::lookup("e/mp").is_none(),
            "e/mp is not a ledger name; the live name is e_mp"
        );
        let e_mp = physis_constants::proton_charge_to_mass();
        let e_mp_centre = Ratio::new(95_788_331_560, 10i128.pow(3));
        assert_eq!(
            proton_charge_to_mass().value(),
            e_mp_centre.to_f64(),
            "e_mp Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            e_mp.value.contains(Interval::point(e_mp_centre)),
            "e_mp Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            e_mp.value.lo, e_mp.value.hi,
            "ledger e_mp stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_charge_to_mass().hash,
            physis_constants::electron_charge_to_mass().hash,
            "e_mp is not e_me"
        );
        assert_ne!(
            physis_constants::proton_charge_to_mass().hash,
            physis_constants::proton_neutron_mass_ratio().hash,
            "e_mp is not mp_mn"
        );
        assert_ne!(
            physis_constants::proton_charge_to_mass().hash,
            physis_constants::elementary_charge().hash,
            "e_mp is not the SI-exact elementary charge"
        );
        assert!(
            physis_constants::lookup("Mp").is_none(),
            "Mp is not a ledger name; the live name is M_p"
        );
        let m_p_molar = physis_constants::proton_molar_mass();
        let m_p_molar_centre = Ratio::new(100_727_646_627, 10i128.pow(14));
        assert_eq!(
            proton_molar_mass().value(),
            m_p_molar_centre.to_f64(),
            "M_p Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_p_molar.value.contains(Interval::point(m_p_molar_centre)),
            "M_p Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_p_molar.value.lo, m_p_molar.value.hi,
            "ledger M_p stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_molar_mass().hash,
            physis_constants::electron_molar_mass().hash,
            "M_p is not M_e"
        );
        assert_ne!(
            physis_constants::proton_molar_mass().hash,
            physis_constants::muon_molar_mass().hash,
            "M_p is not M_mu"
        );
        assert_ne!(
            physis_constants::proton_molar_mass().hash,
            physis_constants::proton_mass_in_u().hash,
            "M_p is not m_p_u"
        );
        assert_ne!(
            physis_constants::proton_molar_mass().hash,
            physis_constants::proton_mass().hash,
            "M_p is not m_p"
        );
        assert!(
            physis_constants::lookup("lambdabar_C_p").is_none(),
            "reduced proton Compton is hbar/m_p c and is not stored"
        );
        let lambda_c_p = physis_constants::proton_compton_wavelength();
        let lambda_c_p_centre = Ratio::new(132_140_985_539, 10i128.pow(26));
        assert_eq!(
            proton_compton_wavelength().value(),
            lambda_c_p_centre.to_f64(),
            "lambda_C_p Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            lambda_c_p
                .value
                .contains(Interval::point(lambda_c_p_centre)),
            "lambda_C_p Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            lambda_c_p.value.lo, lambda_c_p.value.hi,
            "ledger lambda_C_p stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_compton_wavelength().hash,
            physis_constants::compton_wavelength().hash,
            "lambda_C_p is not lambda_C"
        );
        assert_ne!(
            physis_constants::proton_compton_wavelength().hash,
            physis_constants::muon_compton_wavelength().hash,
            "lambda_C_p is not lambda_C_mu"
        );
        assert_ne!(
            physis_constants::proton_compton_wavelength().hash,
            physis_constants::reduced_compton_wavelength().hash,
            "lambda_C_p is not lambdabar_C"
        );
        assert_ne!(
            physis_constants::proton_compton_wavelength().hash,
            physis_constants::proton_molar_mass().hash,
            "lambda_C_p is not M_p"
        );
        assert_ne!(
            physis_constants::proton_compton_wavelength().hash,
            physis_constants::proton_mass().hash,
            "lambda_C_p is not m_p"
        );
        assert!(
            physis_constants::lookup("r_p").is_none(),
            "r_p is not a ledger name; the live name is rp"
        );
        let rp = physis_constants::proton_rms_charge_radius();
        let rp_centre = Ratio::new(8414, 10i128.pow(19));
        assert_eq!(
            proton_rms_charge_radius().value(),
            rp_centre.to_f64(),
            "rp Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            rp.value.contains(Interval::point(rp_centre)),
            "rp Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            rp.value.lo, rp.value.hi,
            "ledger rp stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_rms_charge_radius().hash,
            physis_constants::classical_electron_radius().hash,
            "rp is not re"
        );
        assert_ne!(
            physis_constants::proton_rms_charge_radius().hash,
            physis_constants::proton_compton_wavelength().hash,
            "rp is not lambda_C_p"
        );
        assert_ne!(
            physis_constants::proton_rms_charge_radius().hash,
            physis_constants::compton_wavelength().hash,
            "rp is not lambda_C"
        );
        assert_ne!(
            physis_constants::proton_rms_charge_radius().hash,
            physis_constants::proton_mass().hash,
            "rp is not m_p"
        );
        assert!(
            physis_constants::lookup("rd").is_some(),
            "deuteron rms charge radius is the live ledger name rd"
        );
        assert!(
            physis_constants::lookup("mup").is_none(),
            "mup is not a ledger name; the live name is mu_p"
        );
        let mu_p = physis_constants::proton_magnetic_moment();
        let mu_p_centre = Ratio::new(141_060_679_736, 10i128.pow(37));
        assert_eq!(
            proton_magnetic_moment().value(),
            mu_p_centre.to_f64(),
            "mu_p Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_p.value.contains(Interval::point(mu_p_centre)),
            "mu_p Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_p.value.lo, mu_p.value.hi,
            "ledger mu_p stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_magnetic_moment().hash,
            physis_constants::electron_magnetic_moment().hash,
            "mu_p is not mu_e"
        );
        assert_ne!(
            physis_constants::proton_magnetic_moment().hash,
            physis_constants::muon_magnetic_moment().hash,
            "mu_p is not mu_mu"
        );
        assert_ne!(
            physis_constants::proton_magnetic_moment().hash,
            physis_constants::electron_proton_magnetic_moment_ratio().hash,
            "mu_p is not mu_e_mup"
        );
        assert_ne!(
            physis_constants::proton_magnetic_moment().hash,
            physis_constants::vacuum_permeability().hash,
            "mu_p is not mu0"
        );
        assert_ne!(
            physis_constants::proton_magnetic_moment().hash,
            physis_constants::proton_rms_charge_radius().hash,
            "mu_p is not rp"
        );
        assert!(
            physis_constants::lookup("g0p").is_none(),
            "shielded proton g-factor is not stored in this increment"
        );
        assert!(
            physis_constants::lookup("mup_muB").is_none(),
            "mup_muB is not a ledger name; the live name is mu_p_muB"
        );
        let mu_p_mu_b = physis_constants::proton_magnetic_moment_to_bohr_magneton();
        let mu_p_mu_b_centre = Ratio::new(152_103_220_230, 10i128.pow(14));
        assert_eq!(
            proton_magnetic_moment_to_bohr_magneton().value(),
            mu_p_mu_b_centre.to_f64(),
            "mu_p_muB Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_p_mu_b.value.contains(Interval::point(mu_p_mu_b_centre)),
            "mu_p_muB Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_p_mu_b.value.lo, mu_p_mu_b.value.hi,
            "ledger mu_p_muB stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::proton_magnetic_moment().hash,
            "mu_p_muB is not mu_p"
        );
        assert_ne!(
            physis_constants::proton_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::electron_magnetic_moment_to_bohr_magneton().hash,
            "mu_p_muB is not mu_e_muB"
        );
        assert_ne!(
            physis_constants::proton_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::muon_magnetic_moment_to_bohr_magneton().hash,
            "mu_p_muB is not mu_mu_muB"
        );
        assert_ne!(
            physis_constants::proton_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::electron_magnetic_moment().hash,
            "mu_p_muB is not mu_e"
        );
        assert!(
            physis_constants::lookup("mup_muN").is_none(),
            "mup_muN is not a ledger name; the live name is mu_p_muN"
        );
        let mu_p_mu_n = physis_constants::proton_magnetic_moment_to_nuclear_magneton();
        let mu_p_mu_n_centre = Ratio::new(279_284_734_463, 10i128.pow(11));
        assert_eq!(
            proton_magnetic_moment_to_nuclear_magneton().value(),
            mu_p_mu_n_centre.to_f64(),
            "mu_p_muN Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_p_mu_n.value.contains(Interval::point(mu_p_mu_n_centre)),
            "mu_p_muN Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_p_mu_n.value.lo, mu_p_mu_n.value.hi,
            "ledger mu_p_muN stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::proton_magnetic_moment_to_bohr_magneton().hash,
            "mu_p_muN is not mu_p_muB"
        );
        assert_ne!(
            physis_constants::proton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::proton_magnetic_moment().hash,
            "mu_p_muN is not mu_p"
        );
        assert_ne!(
            physis_constants::proton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::electron_magnetic_moment_to_nuclear_magneton().hash,
            "mu_p_muN is not mu_e_muN"
        );
        assert_ne!(
            physis_constants::proton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::muon_magnetic_moment_to_nuclear_magneton().hash,
            "mu_p_muN is not mu_mu_muN"
        );
        assert!(
            physis_constants::lookup("g_p").is_none(),
            "g_p is not a ledger name; the live name is gp"
        );
        let gp = physis_constants::proton_g_factor();
        let gp_centre = Ratio::new(55_856_946_893, 10i128.pow(10));
        assert_eq!(
            proton_g_factor().value(),
            gp_centre.to_f64(),
            "gp Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            gp.value.contains(Interval::point(gp_centre)),
            "gp Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            gp.value.lo, gp.value.hi,
            "ledger gp stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_g_factor().hash,
            physis_constants::proton_magnetic_moment_to_nuclear_magneton().hash,
            "gp is not mu_p_muN"
        );
        assert_ne!(
            physis_constants::proton_g_factor().hash,
            physis_constants::electron_g_factor().hash,
            "gp is not ge"
        );
        assert_ne!(
            physis_constants::proton_g_factor().hash,
            physis_constants::muon_g_factor().hash,
            "gp is not gmu"
        );
        assert!(
            physis_constants::lookup("g0p").is_none(),
            "shielded proton g-factor is not stored in this increment"
        );
        assert!(
            physis_constants::lookup("mup_mun").is_none(),
            "mup_mun is not a ledger name; the live name is mu_p_mun"
        );
        let mu_p_mun = physis_constants::proton_neutron_magnetic_moment_ratio();
        let mu_p_mun_centre = Ratio::new(-145_989_805, 10i128.pow(8));
        assert_eq!(
            proton_neutron_magnetic_moment_ratio().value(),
            mu_p_mun_centre.to_f64(),
            "mu_p_mun Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_p_mun.value.contains(Interval::point(mu_p_mun_centre)),
            "mu_p_mun Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_p_mun.value.lo, mu_p_mun.value.hi,
            "ledger mu_p_mun stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_neutron_magnetic_moment_ratio().hash,
            physis_constants::electron_neutron_magnetic_moment_ratio().hash,
            "mu_p_mun is not mu_e_mun"
        );
        assert_ne!(
            physis_constants::proton_neutron_magnetic_moment_ratio().hash,
            physis_constants::proton_neutron_mass_ratio().hash,
            "mu_p_mun is not mp_mn"
        );
        assert_ne!(
            physis_constants::proton_neutron_magnetic_moment_ratio().hash,
            physis_constants::proton_g_factor().hash,
            "mu_p_mun is not gp"
        );
        assert!(
            physis_constants::lookup("mu_0p").is_none(),
            "mu_0p is not a ledger name; the live name is mu0p"
        );
        let mu0p = physis_constants::shielded_proton_magnetic_moment();
        let mu0p_centre = Ratio::new(1_410_570_560, 10i128.pow(35));
        assert_eq!(
            shielded_proton_magnetic_moment().value(),
            mu0p_centre.to_f64(),
            "mu0p Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu0p.value.contains(Interval::point(mu0p_centre)),
            "mu0p Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu0p.value.lo, mu0p.value.hi,
            "ledger mu0p stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::shielded_proton_magnetic_moment().hash,
            physis_constants::proton_magnetic_moment().hash,
            "mu0p is not mu_p"
        );
        assert_ne!(
            physis_constants::shielded_proton_magnetic_moment().hash,
            physis_constants::vacuum_permeability().hash,
            "mu0p is not mu0"
        );
        assert_ne!(
            physis_constants::shielded_proton_magnetic_moment().hash,
            physis_constants::electron_to_shielded_proton_magnetic_moment_ratio().hash,
            "mu0p is not mu_e_mu0p"
        );
        assert!(
            physis_constants::lookup("mu0p_mub").is_none(),
            "mu0p_mub is not a ledger name; the live name is mu0p_muB"
        );
        let mu0p_mu_b = physis_constants::shielded_proton_magnetic_moment_to_bohr_magneton();
        let mu0p_mu_b_centre = Ratio::new(1_520_993_128, 10i128.pow(12));
        assert_eq!(
            shielded_proton_magnetic_moment_to_bohr_magneton().value(),
            mu0p_mu_b_centre.to_f64(),
            "mu0p_muB Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu0p_mu_b.value.contains(Interval::point(mu0p_mu_b_centre)),
            "mu0p_muB Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu0p_mu_b.value.lo, mu0p_mu_b.value.hi,
            "ledger mu0p_muB stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::shielded_proton_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::proton_magnetic_moment_to_bohr_magneton().hash,
            "mu0p_muB is not mu_p_muB"
        );
        assert_ne!(
            physis_constants::shielded_proton_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::shielded_proton_magnetic_moment().hash,
            "mu0p_muB is not mu0p"
        );
        assert_ne!(
            physis_constants::shielded_proton_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::electron_magnetic_moment_to_bohr_magneton().hash,
            "mu0p_muB is not mu_e_muB"
        );
        assert_ne!(
            physis_constants::shielded_proton_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::electron_to_shielded_proton_magnetic_moment_ratio().hash,
            "mu0p_muB is not mu_e_mu0p"
        );
        assert!(
            physis_constants::lookup("mu0p_mun").is_none(),
            "mu0p_mun is not a ledger name; the live name is mu0p_muN"
        );
        let mu0p_mu_n = physis_constants::shielded_proton_magnetic_moment_to_nuclear_magneton();
        let mu0p_mu_n_centre = Ratio::new(2_792_775_599, 10i128.pow(9));
        assert_eq!(
            shielded_proton_magnetic_moment_to_nuclear_magneton().value(),
            mu0p_mu_n_centre.to_f64(),
            "mu0p_muN Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu0p_mu_n.value.contains(Interval::point(mu0p_mu_n_centre)),
            "mu0p_muN Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu0p_mu_n.value.lo, mu0p_mu_n.value.hi,
            "ledger mu0p_muN stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::shielded_proton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::proton_magnetic_moment_to_nuclear_magneton().hash,
            "mu0p_muN is not mu_p_muN"
        );
        assert_ne!(
            physis_constants::shielded_proton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::shielded_proton_magnetic_moment_to_bohr_magneton().hash,
            "mu0p_muN is not mu0p_muB"
        );
        assert_ne!(
            physis_constants::shielded_proton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::electron_magnetic_moment_to_nuclear_magneton().hash,
            "mu0p_muN is not mu_e_muN"
        );
        assert_ne!(
            physis_constants::shielded_proton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::proton_g_factor().hash,
            "mu0p_muN is not gp"
        );
        assert!(
            physis_constants::lookup("sigma_p").is_none(),
            "sigma_p is not a ledger name; the live name is sigma0p"
        );
        let sigma0p = physis_constants::proton_magnetic_shielding_correction();
        let sigma0p_centre = Ratio::new(25_689, 10i128.pow(9));
        assert_eq!(
            proton_magnetic_shielding_correction().value(),
            sigma0p_centre.to_f64(),
            "sigma0p Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            sigma0p.value.contains(Interval::point(sigma0p_centre)),
            "sigma0p Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            sigma0p.value.lo, sigma0p.value.hi,
            "ledger sigma0p stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::proton_magnetic_shielding_correction().hash,
            physis_constants::shielded_proton_magnetic_moment().hash,
            "sigma0p is not mu0p"
        );
        assert_ne!(
            physis_constants::proton_magnetic_shielding_correction().hash,
            physis_constants::proton_magnetic_moment().hash,
            "sigma0p is not mu_p"
        );
        assert_ne!(
            physis_constants::proton_magnetic_shielding_correction().hash,
            physis_constants::vacuum_permeability().hash,
            "sigma0p is not mu0"
        );
        assert_ne!(
            physis_constants::proton_magnetic_shielding_correction().hash,
            physis_constants::shielded_proton_magnetic_moment_to_nuclear_magneton().hash,
            "sigma0p is not mu0p_muN"
        );
        assert!(
            physis_constants::lookup("mn").is_none(),
            "mn is not a ledger name; the live name is m_n"
        );
        let m_n = physis_constants::neutron_mass();
        let m_n_centre = Ratio::new(167_492_749_804, 10i128.pow(38));
        assert_eq!(
            neutron_mass().value(),
            m_n_centre.to_f64(),
            "m_n Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_n.value.contains(Interval::point(m_n_centre)),
            "m_n Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_n.value.lo, m_n.value.hi,
            "ledger m_n stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_mass().hash,
            physis_constants::proton_mass().hash,
            "m_n is not m_p"
        );
        assert_ne!(
            physis_constants::neutron_mass().hash,
            physis_constants::muon_mass().hash,
            "m_n is not m_mu"
        );
        assert_ne!(
            physis_constants::neutron_mass().hash,
            physis_constants::electron_neutron_mass_ratio().hash,
            "m_n is not me_mn"
        );
        assert_ne!(
            physis_constants::neutron_mass().hash,
            physis_constants::proton_neutron_mass_ratio().hash,
            "m_n is not mp_mn"
        );
        assert_ne!(
            physis_constants::neutron_mass().hash,
            physis_constants::proton_magnetic_shielding_correction().hash,
            "m_n is not sigma0p"
        );
        assert!(
            physis_constants::lookup("mn_u").is_none(),
            "mn_u is not a ledger name; the live name is m_n_u"
        );
        let m_n_u = physis_constants::neutron_mass_in_u();
        let m_n_u_centre = Ratio::new(100_866_491_595, 10i128.pow(11));
        assert_eq!(
            neutron_mass_in_u().value(),
            m_n_u_centre.to_f64(),
            "m_n_u Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_n_u.value.contains(Interval::point(m_n_u_centre)),
            "m_n_u Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_n_u.value.lo, m_n_u.value.hi,
            "ledger m_n_u stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_mass_in_u().hash,
            physis_constants::neutron_mass().hash,
            "m_n_u is not m_n"
        );
        assert_ne!(
            physis_constants::neutron_mass_in_u().hash,
            physis_constants::proton_mass_in_u().hash,
            "m_n_u is not m_p_u"
        );
        assert_ne!(
            physis_constants::neutron_mass_in_u().hash,
            physis_constants::muon_mass_in_u().hash,
            "m_n_u is not m_mu_u"
        );
        assert_ne!(
            physis_constants::neutron_mass_in_u().hash,
            physis_constants::electron_molar_mass().hash,
            "m_n_u is not M_e"
        );
        assert!(
            physis_constants::lookup("mnc2").is_none(),
            "mnc2 is not a ledger name; the live name is m_n_c2"
        );
        let m_n_c2 = physis_constants::neutron_mass_energy_equivalent();
        let m_n_c2_centre = Ratio::new(150_534_976_287, 10i128.pow(21));
        assert_eq!(
            neutron_mass_energy_equivalent().value(),
            m_n_c2_centre.to_f64(),
            "m_n_c2 Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_n_c2.value.contains(Interval::point(m_n_c2_centre)),
            "m_n_c2 Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_n_c2.value.lo, m_n_c2.value.hi,
            "ledger m_n_c2 stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_mass_energy_equivalent().hash,
            physis_constants::neutron_mass().hash,
            "m_n_c2 is not m_n"
        );
        assert_ne!(
            physis_constants::neutron_mass_energy_equivalent().hash,
            physis_constants::neutron_mass_in_u().hash,
            "m_n_c2 is not m_n_u"
        );
        assert_ne!(
            physis_constants::neutron_mass_energy_equivalent().hash,
            physis_constants::proton_mass_energy_equivalent().hash,
            "m_n_c2 is not m_p_c2"
        );
        assert_ne!(
            physis_constants::neutron_mass_energy_equivalent().hash,
            physis_constants::muon_mass_energy_equivalent().hash,
            "m_n_c2 is not m_mu_c2"
        );
        assert!(
            physis_constants::lookup("mnc2_MeV").is_none(),
            "mnc2_MeV is not a ledger name; the live name is m_n_c2_MeV"
        );
        let m_n_c2_mev = physis_constants::neutron_mass_energy_equivalent_in_mev();
        let m_n_c2_mev_centre = Ratio::new(93_956_542_052, 10i128.pow(8));
        assert_eq!(
            neutron_mass_energy_equivalent_in_mev().value(),
            m_n_c2_mev_centre.to_f64(),
            "m_n_c2_MeV Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_n_c2_mev
                .value
                .contains(Interval::point(m_n_c2_mev_centre)),
            "m_n_c2_MeV Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_n_c2_mev.value.lo, m_n_c2_mev.value.hi,
            "ledger m_n_c2_MeV stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_mass_energy_equivalent_in_mev().hash,
            physis_constants::neutron_mass_energy_equivalent().hash,
            "m_n_c2_MeV is not m_n_c2"
        );
        assert_ne!(
            physis_constants::neutron_mass_energy_equivalent_in_mev().hash,
            physis_constants::proton_mass_energy_equivalent_in_mev().hash,
            "m_n_c2_MeV is not m_p_c2_MeV"
        );
        assert_ne!(
            physis_constants::neutron_mass_energy_equivalent_in_mev().hash,
            physis_constants::muon_mass_energy_equivalent_in_mev().hash,
            "m_n_c2_MeV is not m_mu_c2_MeV"
        );
        assert_ne!(
            physis_constants::neutron_mass_energy_equivalent_in_mev().hash,
            physis_constants::electron_volt().hash,
            "m_n_c2_MeV is not eV"
        );
        assert!(
            physis_constants::lookup("mn/me").is_none(),
            "mn/me is not a ledger name; the live name is mn_me"
        );
        let mn_me = physis_constants::neutron_electron_mass_ratio();
        let mn_me_centre = Ratio::new(183_868_366_173, 10i128.pow(8));
        assert_eq!(
            neutron_electron_mass_ratio().value(),
            mn_me_centre.to_f64(),
            "mn_me Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mn_me.value.contains(Interval::point(mn_me_centre)),
            "mn_me Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mn_me.value.lo, mn_me.value.hi,
            "ledger mn_me stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_electron_mass_ratio().hash,
            physis_constants::electron_neutron_mass_ratio().hash,
            "mn_me is not me_mn"
        );
        assert_ne!(
            physis_constants::neutron_electron_mass_ratio().hash,
            physis_constants::proton_electron_mass_ratio().hash,
            "mn_me is not mp_me"
        );
        assert_ne!(
            physis_constants::neutron_electron_mass_ratio().hash,
            physis_constants::muon_electron_mass_ratio().hash,
            "mn_me is not mmu_me"
        );
        assert_ne!(
            physis_constants::neutron_electron_mass_ratio().hash,
            physis_constants::neutron_mass_energy_equivalent_in_mev().hash,
            "mn_me is not m_n_c2_MeV"
        );
        assert!(
            physis_constants::lookup("mn/mmu").is_none(),
            "mn/mmu is not a ledger name; the live name is mn_mmu"
        );
        let mn_mmu = physis_constants::neutron_muon_mass_ratio();
        let mn_mmu_centre = Ratio::new(889_248_406, 10i128.pow(8));
        assert_eq!(
            neutron_muon_mass_ratio().value(),
            mn_mmu_centre.to_f64(),
            "mn_mmu Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mn_mmu.value.contains(Interval::point(mn_mmu_centre)),
            "mn_mmu Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mn_mmu.value.lo, mn_mmu.value.hi,
            "ledger mn_mmu stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_muon_mass_ratio().hash,
            physis_constants::muon_neutron_mass_ratio().hash,
            "mn_mmu is not mmu_mn"
        );
        assert_ne!(
            physis_constants::neutron_muon_mass_ratio().hash,
            physis_constants::proton_muon_mass_ratio().hash,
            "mn_mmu is not mp_mmu"
        );
        assert_ne!(
            physis_constants::neutron_muon_mass_ratio().hash,
            physis_constants::neutron_electron_mass_ratio().hash,
            "mn_mmu is not mn_me"
        );
        assert_ne!(
            physis_constants::neutron_muon_mass_ratio().hash,
            physis_constants::muon_proton_mass_ratio().hash,
            "mn_mmu is not mmu_mp"
        );
        assert!(
            physis_constants::lookup("mn/mp").is_none(),
            "mn/mp is not a ledger name; the live name is mn_mp"
        );
        let mn_mp = physis_constants::neutron_proton_mass_ratio();
        let mn_mp_centre = Ratio::new(100_137_841_931, 10i128.pow(11));
        assert_eq!(
            neutron_proton_mass_ratio().value(),
            mn_mp_centre.to_f64(),
            "mn_mp Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mn_mp.value.contains(Interval::point(mn_mp_centre)),
            "mn_mp Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mn_mp.value.lo, mn_mp.value.hi,
            "ledger mn_mp stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_ratio().hash,
            physis_constants::proton_neutron_mass_ratio().hash,
            "mn_mp is not mp_mn"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_ratio().hash,
            physis_constants::electron_neutron_mass_ratio().hash,
            "mn_mp is not me_mn"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_ratio().hash,
            physis_constants::muon_neutron_mass_ratio().hash,
            "mn_mp is not mmu_mn"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_ratio().hash,
            physis_constants::neutron_muon_mass_ratio().hash,
            "mn_mp is not mn_mmu"
        );
        assert!(
            physis_constants::lookup("mn-mp").is_none(),
            "mn-mp is not a ledger name; the live name is mn_minus_mp"
        );
        let mn_minus_mp = physis_constants::neutron_proton_mass_difference();
        let mn_minus_mp_centre = Ratio::new(230_557_435, 10i128.pow(38));
        assert_eq!(
            neutron_proton_mass_difference().value(),
            2.305_574_35e-30,
            "mn_minus_mp Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mn_minus_mp
                .value
                .contains(Interval::point(mn_minus_mp_centre)),
            "mn_minus_mp Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mn_minus_mp.value.lo, mn_minus_mp.value.hi,
            "ledger mn_minus_mp stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_difference().hash,
            physis_constants::neutron_mass().hash,
            "mn_minus_mp is not m_n"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_difference().hash,
            physis_constants::proton_mass().hash,
            "mn_minus_mp is not m_p"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_difference().hash,
            physis_constants::neutron_proton_mass_ratio().hash,
            "mn_minus_mp is not mn_mp"
        );
        assert!(
            physis_constants::lookup("mn-mp_u").is_none(),
            "mn-mp_u is not a ledger name; the live name is mn_minus_mp_u"
        );
        let mn_minus_mp_u = physis_constants::neutron_proton_mass_difference_in_u();
        let mn_minus_mp_u_centre = Ratio::new(138_844_933, 10i128.pow(11));
        assert_eq!(
            neutron_proton_mass_difference_in_u().value(),
            mn_minus_mp_u_centre.to_f64(),
            "mn_minus_mp_u Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mn_minus_mp_u
                .value
                .contains(Interval::point(mn_minus_mp_u_centre)),
            "mn_minus_mp_u Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mn_minus_mp_u.value.lo, mn_minus_mp_u.value.hi,
            "ledger mn_minus_mp_u stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_difference_in_u().hash,
            physis_constants::neutron_proton_mass_difference().hash,
            "mn_minus_mp_u is not mn_minus_mp"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_difference_in_u().hash,
            physis_constants::neutron_mass_in_u().hash,
            "mn_minus_mp_u is not m_n_u"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_difference_in_u().hash,
            physis_constants::proton_mass_in_u().hash,
            "mn_minus_mp_u is not m_p_u"
        );
        assert!(
            physis_constants::lookup("mn-mp_c2").is_none(),
            "mn-mp_c2 is not a ledger name; the live name is mn_minus_mp_c2"
        );
        let mn_minus_mp_c2 = physis_constants::neutron_proton_mass_difference_energy_equivalent();
        let mn_minus_mp_c2_centre = Ratio::new(207_214_689, 10i128.pow(21));
        assert_eq!(
            neutron_proton_mass_difference_energy_equivalent().value(),
            mn_minus_mp_c2_centre.to_f64(),
            "mn_minus_mp_c2 Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mn_minus_mp_c2
                .value
                .contains(Interval::point(mn_minus_mp_c2_centre)),
            "mn_minus_mp_c2 Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mn_minus_mp_c2.value.lo, mn_minus_mp_c2.value.hi,
            "ledger mn_minus_mp_c2 stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_difference_energy_equivalent().hash,
            physis_constants::neutron_proton_mass_difference().hash,
            "mn_minus_mp_c2 is not mn_minus_mp"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_difference_energy_equivalent().hash,
            physis_constants::neutron_mass_energy_equivalent().hash,
            "mn_minus_mp_c2 is not m_n_c2"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_difference_energy_equivalent().hash,
            physis_constants::proton_mass_energy_equivalent().hash,
            "mn_minus_mp_c2 is not m_p_c2"
        );
        assert!(
            physis_constants::lookup("mn-mp_c2_MeV").is_none(),
            "mn-mp_c2_MeV is not a ledger name; the live name is mn_minus_mp_c2_MeV"
        );
        let mn_minus_mp_c2_mev =
            physis_constants::neutron_proton_mass_difference_energy_equivalent_in_mev();
        let mn_minus_mp_c2_mev_centre = Ratio::new(129_333_236, 10i128.pow(8));
        assert_eq!(
            neutron_proton_mass_difference_energy_equivalent_in_mev().value(),
            mn_minus_mp_c2_mev_centre.to_f64(),
            "mn_minus_mp_c2_MeV Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mn_minus_mp_c2_mev
                .value
                .contains(Interval::point(mn_minus_mp_c2_mev_centre)),
            "mn_minus_mp_c2_MeV Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mn_minus_mp_c2_mev.value.lo, mn_minus_mp_c2_mev.value.hi,
            "ledger mn_minus_mp_c2_MeV stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_difference_energy_equivalent_in_mev().hash,
            physis_constants::neutron_proton_mass_difference_energy_equivalent().hash,
            "mn_minus_mp_c2_MeV is not mn_minus_mp_c2"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_difference_energy_equivalent_in_mev().hash,
            physis_constants::neutron_mass_energy_equivalent_in_mev().hash,
            "mn_minus_mp_c2_MeV is not m_n_c2_MeV"
        );
        assert_ne!(
            physis_constants::neutron_proton_mass_difference_energy_equivalent_in_mev().hash,
            physis_constants::proton_mass_energy_equivalent_in_mev().hash,
            "mn_minus_mp_c2_MeV is not m_p_c2_MeV"
        );
        assert!(
            physis_constants::lookup("Mn").is_none(),
            "Mn is not a ledger name; the live name is M_n"
        );
        let m_n_molar = physis_constants::neutron_molar_mass();
        let m_n_molar_centre = Ratio::new(100_866_491_560, 10i128.pow(14));
        assert_eq!(
            neutron_molar_mass().value(),
            m_n_molar_centre.to_f64(),
            "M_n Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_n_molar.value.contains(Interval::point(m_n_molar_centre)),
            "M_n Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_n_molar.value.lo, m_n_molar.value.hi,
            "ledger M_n stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_molar_mass().hash,
            physis_constants::proton_molar_mass().hash,
            "M_n is not M_p"
        );
        assert_ne!(
            physis_constants::neutron_molar_mass().hash,
            physis_constants::neutron_mass().hash,
            "M_n is not m_n"
        );
        assert_ne!(
            physis_constants::neutron_molar_mass().hash,
            physis_constants::neutron_mass_in_u().hash,
            "M_n is not m_n_u"
        );
        assert!(
            physis_constants::lookup("lambdabar_C_n").is_none(),
            "lambdabar_C_n is not a ledger name; reduced neutron Compton cites hbar"
        );
        let lambda_c_n = physis_constants::neutron_compton_wavelength();
        let lambda_c_n_centre = Ratio::new(131_959_090_581, 10i128.pow(26));
        assert_eq!(
            neutron_compton_wavelength().value(),
            1.319_590_905_81e-15,
            "lambda_C_n Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            lambda_c_n
                .value
                .contains(Interval::point(lambda_c_n_centre)),
            "lambda_C_n Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            lambda_c_n.value.lo, lambda_c_n.value.hi,
            "ledger lambda_C_n stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_compton_wavelength().hash,
            physis_constants::proton_compton_wavelength().hash,
            "lambda_C_n is not lambda_C_p"
        );
        assert_ne!(
            physis_constants::neutron_compton_wavelength().hash,
            physis_constants::compton_wavelength().hash,
            "lambda_C_n is not lambda_C"
        );
        assert_ne!(
            physis_constants::neutron_compton_wavelength().hash,
            physis_constants::muon_compton_wavelength().hash,
            "lambda_C_n is not lambda_C_mu"
        );
        assert!(
            physis_constants::lookup("mun").is_none(),
            "mun is not a ledger name; the live name is mu_n"
        );
        let mu_n = physis_constants::neutron_magnetic_moment();
        let mu_n_centre = Ratio::new(-96_623_651, 10i128.pow(34));
        assert_eq!(
            neutron_magnetic_moment().value(),
            mu_n_centre.to_f64(),
            "mu_n Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_n.value.contains(Interval::point(mu_n_centre)),
            "mu_n Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_n.value.lo, mu_n.value.hi,
            "ledger mu_n stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_magnetic_moment().hash,
            physis_constants::proton_magnetic_moment().hash,
            "mu_n is not mu_p"
        );
        assert_ne!(
            physis_constants::neutron_magnetic_moment().hash,
            physis_constants::electron_magnetic_moment().hash,
            "mu_n is not mu_e"
        );
        assert_ne!(
            physis_constants::neutron_magnetic_moment().hash,
            physis_constants::muon_magnetic_moment().hash,
            "mu_n is not mu_mu"
        );
        assert_ne!(
            physis_constants::neutron_magnetic_moment().hash,
            physis_constants::vacuum_permeability().hash,
            "mu_n is not mu0"
        );
        assert!(
            physis_constants::lookup("mun_muB").is_none(),
            "mun_muB is not a ledger name; the live name is mu_n_muB"
        );
        let mu_n_mu_b = physis_constants::neutron_magnetic_moment_to_bohr_magneton();
        let mu_n_mu_b_centre = Ratio::new(-104_187_563, 10i128.pow(11));
        assert_eq!(
            neutron_magnetic_moment_to_bohr_magneton().value(),
            mu_n_mu_b_centre.to_f64(),
            "mu_n_muB Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_n_mu_b.value.contains(Interval::point(mu_n_mu_b_centre)),
            "mu_n_muB Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_n_mu_b.value.lo, mu_n_mu_b.value.hi,
            "ledger mu_n_muB stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::neutron_magnetic_moment().hash,
            "mu_n_muB is not mu_n"
        );
        assert_ne!(
            physis_constants::neutron_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::proton_magnetic_moment_to_bohr_magneton().hash,
            "mu_n_muB is not mu_p_muB"
        );
        assert_ne!(
            physis_constants::neutron_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::electron_magnetic_moment_to_bohr_magneton().hash,
            "mu_n_muB is not mu_e_muB"
        );
        assert_ne!(
            physis_constants::neutron_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::muon_magnetic_moment_to_bohr_magneton().hash,
            "mu_n_muB is not mu_mu_muB"
        );
        assert!(
            physis_constants::lookup("mun_muN").is_none(),
            "mun_muN is not a ledger name; the live name is mu_n_muN"
        );
        let mu_n_mu_n = physis_constants::neutron_magnetic_moment_to_nuclear_magneton();
        let mu_n_mu_n_centre = Ratio::new(-191_304_273, 10i128.pow(8));
        assert_eq!(
            neutron_magnetic_moment_to_nuclear_magneton().value(),
            mu_n_mu_n_centre.to_f64(),
            "mu_n_muN Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_n_mu_n.value.contains(Interval::point(mu_n_mu_n_centre)),
            "mu_n_muN Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_n_mu_n.value.lo, mu_n_mu_n.value.hi,
            "ledger mu_n_muN stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::neutron_magnetic_moment().hash,
            "mu_n_muN is not mu_n"
        );
        assert_ne!(
            physis_constants::neutron_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::neutron_magnetic_moment_to_bohr_magneton().hash,
            "mu_n_muN is not mu_n_muB"
        );
        assert_ne!(
            physis_constants::neutron_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::proton_magnetic_moment_to_nuclear_magneton().hash,
            "mu_n_muN is not mu_p_muN"
        );
        assert_ne!(
            physis_constants::neutron_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::electron_magnetic_moment_to_nuclear_magneton().hash,
            "mu_n_muN is not mu_e_muN"
        );
        assert_ne!(
            physis_constants::neutron_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::muon_magnetic_moment_to_nuclear_magneton().hash,
            "mu_n_muN is not mu_mu_muN"
        );
        assert!(
            physis_constants::lookup("g_n").is_none(),
            "g_n is not a ledger name; the live name is gn"
        );
        let gn = physis_constants::neutron_g_factor();
        let gn_centre = Ratio::new(-382_608_545, 10i128.pow(8));
        assert_eq!(
            neutron_g_factor().value(),
            gn_centre.to_f64(),
            "gn Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            gn.value.contains(Interval::point(gn_centre)),
            "gn Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            gn.value.lo, gn.value.hi,
            "ledger gn stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_g_factor().hash,
            physis_constants::neutron_magnetic_moment_to_nuclear_magneton().hash,
            "gn is not mu_n_muN"
        );
        assert_ne!(
            physis_constants::neutron_g_factor().hash,
            physis_constants::electron_g_factor().hash,
            "gn is not ge"
        );
        assert_ne!(
            physis_constants::neutron_g_factor().hash,
            physis_constants::muon_g_factor().hash,
            "gn is not gmu"
        );
        assert_ne!(
            physis_constants::neutron_g_factor().hash,
            physis_constants::proton_g_factor().hash,
            "gn is not gp"
        );
        assert!(
            physis_constants::lookup("mun_mue").is_none(),
            "mun_mue is not a ledger name; the live name is mu_n_mue"
        );
        let mu_n_mue = physis_constants::neutron_electron_magnetic_moment_ratio();
        let mu_n_mue_centre = Ratio::new(104_066_882, 10i128.pow(11));
        assert_eq!(
            neutron_electron_magnetic_moment_ratio().value(),
            mu_n_mue_centre.to_f64(),
            "mu_n_mue Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_n_mue.value.contains(Interval::point(mu_n_mue_centre)),
            "mu_n_mue Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_n_mue.value.lo, mu_n_mue.value.hi,
            "ledger mu_n_mue stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_electron_magnetic_moment_ratio().hash,
            physis_constants::electron_neutron_magnetic_moment_ratio().hash,
            "mu_n_mue is not mu_e_mun"
        );
        assert_ne!(
            physis_constants::neutron_electron_magnetic_moment_ratio().hash,
            physis_constants::neutron_g_factor().hash,
            "mu_n_mue is not gn"
        );
        assert_ne!(
            physis_constants::neutron_electron_magnetic_moment_ratio().hash,
            physis_constants::electron_proton_magnetic_moment_ratio().hash,
            "mu_n_mue is not mu_e_mup"
        );
        assert_ne!(
            physis_constants::neutron_electron_magnetic_moment_ratio().hash,
            physis_constants::neutron_electron_mass_ratio().hash,
            "mu_n_mue is not mn_me"
        );
        assert!(
            physis_constants::lookup("mun_mup").is_none(),
            "mun_mup is not a ledger name; the live name is mu_n_mup"
        );
        let mu_n_mup = physis_constants::neutron_proton_magnetic_moment_ratio();
        let mu_n_mup_centre = Ratio::new(-68_497_934, 10i128.pow(8));
        assert_eq!(
            neutron_proton_magnetic_moment_ratio().value(),
            mu_n_mup_centre.to_f64(),
            "mu_n_mup Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_n_mup.value.contains(Interval::point(mu_n_mup_centre)),
            "mu_n_mup Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_n_mup.value.lo, mu_n_mup.value.hi,
            "ledger mu_n_mup stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_proton_magnetic_moment_ratio().hash,
            physis_constants::proton_neutron_magnetic_moment_ratio().hash,
            "mu_n_mup is not mu_p_mun"
        );
        assert_ne!(
            physis_constants::neutron_proton_magnetic_moment_ratio().hash,
            physis_constants::neutron_electron_magnetic_moment_ratio().hash,
            "mu_n_mup is not mu_n_mue"
        );
        assert_ne!(
            physis_constants::neutron_proton_magnetic_moment_ratio().hash,
            physis_constants::electron_proton_magnetic_moment_ratio().hash,
            "mu_n_mup is not mu_e_mup"
        );
        assert_ne!(
            physis_constants::neutron_proton_magnetic_moment_ratio().hash,
            physis_constants::neutron_proton_mass_ratio().hash,
            "mu_n_mup is not mn_mp"
        );
        assert!(
            physis_constants::lookup("mun_mu0p").is_none(),
            "mun_mu0p is not a ledger name; the live name is mu_n_mu0p"
        );
        let mu_n_mu0p = physis_constants::neutron_to_shielded_proton_magnetic_moment_ratio();
        let mu_n_mu0p_centre = Ratio::new(-68_499_694, 10i128.pow(8));
        assert_eq!(
            neutron_to_shielded_proton_magnetic_moment_ratio().value(),
            mu_n_mu0p_centre.to_f64(),
            "mu_n_mu0p Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_n_mu0p.value.contains(Interval::point(mu_n_mu0p_centre)),
            "mu_n_mu0p Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_n_mu0p.value.lo, mu_n_mu0p.value.hi,
            "ledger mu_n_mu0p stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::neutron_to_shielded_proton_magnetic_moment_ratio().hash,
            physis_constants::neutron_proton_magnetic_moment_ratio().hash,
            "mu_n_mu0p is not mu_n_mup"
        );
        assert_ne!(
            physis_constants::neutron_to_shielded_proton_magnetic_moment_ratio().hash,
            physis_constants::electron_to_shielded_proton_magnetic_moment_ratio().hash,
            "mu_n_mu0p is not mu_e_mu0p"
        );
        assert_ne!(
            physis_constants::neutron_to_shielded_proton_magnetic_moment_ratio().hash,
            physis_constants::shielded_proton_magnetic_moment().hash,
            "mu_n_mu0p is not mu0p"
        );
        assert_ne!(
            physis_constants::neutron_to_shielded_proton_magnetic_moment_ratio().hash,
            physis_constants::neutron_electron_magnetic_moment_ratio().hash,
            "mu_n_mu0p is not mu_n_mue"
        );
        assert!(
            physis_constants::lookup("md").is_none(),
            "md is not a ledger name; the live name is m_d"
        );
        let m_d = physis_constants::deuteron_mass();
        let m_d_centre = Ratio::new(33_435_837_724, 10i128.pow(37));
        assert_eq!(
            deuteron_mass().value(),
            m_d_centre.to_f64(),
            "m_d Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_d.value.contains(Interval::point(m_d_centre)),
            "m_d Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_d.value.lo, m_d.value.hi,
            "ledger m_d stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_mass().hash,
            physis_constants::neutron_mass().hash,
            "m_d is not m_n"
        );
        assert_ne!(
            physis_constants::deuteron_mass().hash,
            physis_constants::proton_mass().hash,
            "m_d is not m_p"
        );
        assert_ne!(
            physis_constants::deuteron_mass().hash,
            physis_constants::muon_mass().hash,
            "m_d is not m_mu"
        );
        assert_ne!(
            physis_constants::deuteron_mass().hash,
            physis_constants::electron_deuteron_mass_ratio().hash,
            "m_d is not me_md"
        );
        assert!(
            physis_constants::lookup("md_u").is_none(),
            "md_u is not a ledger name; the live name is m_d_u"
        );
        let m_d_u = physis_constants::deuteron_mass_in_u();
        let m_d_u_centre = Ratio::new(2_013_553_212_745, 10i128.pow(12));
        assert_eq!(
            deuteron_mass_in_u().value(),
            m_d_u_centre.to_f64(),
            "m_d_u Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_d_u.value.contains(Interval::point(m_d_u_centre)),
            "m_d_u Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_d_u.value.lo, m_d_u.value.hi,
            "ledger m_d_u stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_mass_in_u().hash,
            physis_constants::deuteron_mass().hash,
            "m_d_u is not m_d"
        );
        assert_ne!(
            physis_constants::deuteron_mass_in_u().hash,
            physis_constants::neutron_mass_in_u().hash,
            "m_d_u is not m_n_u"
        );
        assert_ne!(
            physis_constants::deuteron_mass_in_u().hash,
            physis_constants::proton_mass_in_u().hash,
            "m_d_u is not m_p_u"
        );
        assert_ne!(
            physis_constants::deuteron_mass_in_u().hash,
            physis_constants::muon_mass_in_u().hash,
            "m_d_u is not m_mu_u"
        );
        assert!(
            physis_constants::lookup("mdc2").is_none(),
            "mdc2 is not a ledger name; the live name is m_d_c2"
        );
        let m_d_c2 = physis_constants::deuteron_mass_energy_equivalent();
        let m_d_c2_centre = Ratio::new(300_506_323_102, 10i128.pow(21));
        assert_eq!(
            deuteron_mass_energy_equivalent().value(),
            m_d_c2_centre.to_f64(),
            "m_d_c2 Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_d_c2.value.contains(Interval::point(m_d_c2_centre)),
            "m_d_c2 Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_d_c2.value.lo, m_d_c2.value.hi,
            "ledger m_d_c2 stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_mass_energy_equivalent().hash,
            physis_constants::deuteron_mass().hash,
            "m_d_c2 is not m_d"
        );
        assert_ne!(
            physis_constants::deuteron_mass_energy_equivalent().hash,
            physis_constants::deuteron_mass_in_u().hash,
            "m_d_c2 is not m_d_u"
        );
        assert_ne!(
            physis_constants::deuteron_mass_energy_equivalent().hash,
            physis_constants::neutron_mass_energy_equivalent().hash,
            "m_d_c2 is not m_n_c2"
        );
        assert_ne!(
            physis_constants::deuteron_mass_energy_equivalent().hash,
            physis_constants::proton_mass_energy_equivalent().hash,
            "m_d_c2 is not m_p_c2"
        );
        assert_ne!(
            physis_constants::deuteron_mass_energy_equivalent().hash,
            physis_constants::muon_mass_energy_equivalent().hash,
            "m_d_c2 is not m_mu_c2"
        );
        assert!(
            physis_constants::lookup("mdc2_MeV").is_none(),
            "mdc2_MeV is not a ledger name; the live name is m_d_c2_MeV"
        );
        let m_d_c2_mev = physis_constants::deuteron_mass_energy_equivalent_in_mev();
        let m_d_c2_mev_centre = Ratio::new(187_561_294_257, 10i128.pow(8));
        assert_eq!(
            deuteron_mass_energy_equivalent_in_mev().value(),
            m_d_c2_mev_centre.to_f64(),
            "m_d_c2_MeV Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_d_c2_mev
                .value
                .contains(Interval::point(m_d_c2_mev_centre)),
            "m_d_c2_MeV Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_d_c2_mev.value.lo, m_d_c2_mev.value.hi,
            "ledger m_d_c2_MeV stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_mass_energy_equivalent_in_mev().hash,
            physis_constants::deuteron_mass_energy_equivalent().hash,
            "m_d_c2_MeV is not m_d_c2"
        );
        assert_ne!(
            physis_constants::deuteron_mass_energy_equivalent_in_mev().hash,
            physis_constants::neutron_mass_energy_equivalent_in_mev().hash,
            "m_d_c2_MeV is not m_n_c2_MeV"
        );
        assert_ne!(
            physis_constants::deuteron_mass_energy_equivalent_in_mev().hash,
            physis_constants::proton_mass_energy_equivalent_in_mev().hash,
            "m_d_c2_MeV is not m_p_c2_MeV"
        );
        assert_ne!(
            physis_constants::deuteron_mass_energy_equivalent_in_mev().hash,
            physis_constants::muon_mass_energy_equivalent_in_mev().hash,
            "m_d_c2_MeV is not m_mu_c2_MeV"
        );
        assert!(
            physis_constants::lookup("md/me").is_none(),
            "md/me is not a ledger name; the live name is md_me"
        );
        let md_me = physis_constants::deuteron_electron_mass_ratio();
        let md_me_centre = Ratio::new(367_048_296_788, 10i128.pow(8));
        assert_eq!(
            deuteron_electron_mass_ratio().value(),
            md_me_centre.to_f64(),
            "md_me Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            md_me.value.contains(Interval::point(md_me_centre)),
            "md_me Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            md_me.value.lo, md_me.value.hi,
            "ledger md_me stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_electron_mass_ratio().hash,
            physis_constants::electron_deuteron_mass_ratio().hash,
            "md_me is not me_md"
        );
        assert_ne!(
            physis_constants::deuteron_electron_mass_ratio().hash,
            physis_constants::neutron_electron_mass_ratio().hash,
            "md_me is not mn_me"
        );
        assert_ne!(
            physis_constants::deuteron_electron_mass_ratio().hash,
            physis_constants::proton_electron_mass_ratio().hash,
            "md_me is not mp_me"
        );
        assert_ne!(
            physis_constants::deuteron_electron_mass_ratio().hash,
            physis_constants::muon_electron_mass_ratio().hash,
            "md_me is not mmu_me"
        );
        assert!(
            physis_constants::lookup("md/mp").is_none(),
            "md/mp is not a ledger name; the live name is md_mp"
        );
        let md_mp = physis_constants::deuteron_proton_mass_ratio();
        let md_mp_centre = Ratio::new(199_900_750_139, 10i128.pow(11));
        assert_eq!(
            deuteron_proton_mass_ratio().value(),
            md_mp_centre.to_f64(),
            "md_mp Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            md_mp.value.contains(Interval::point(md_mp_centre)),
            "md_mp Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            md_mp.value.lo, md_mp.value.hi,
            "ledger md_mp stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_proton_mass_ratio().hash,
            physis_constants::deuteron_electron_mass_ratio().hash,
            "md_mp is not md_me"
        );
        assert_ne!(
            physis_constants::deuteron_proton_mass_ratio().hash,
            physis_constants::neutron_proton_mass_ratio().hash,
            "md_mp is not mn_mp"
        );
        assert_ne!(
            physis_constants::deuteron_proton_mass_ratio().hash,
            physis_constants::proton_neutron_mass_ratio().hash,
            "md_mp is not mp_mn"
        );
        assert!(
            physis_constants::lookup("Md").is_none(),
            "Md is not a ledger name; the live name is M_d"
        );
        let m_d_molar = physis_constants::deuteron_molar_mass();
        let m_d_molar_centre = Ratio::new(201_355_321_205, 10i128.pow(14));
        assert_eq!(
            deuteron_molar_mass().value(),
            m_d_molar_centre.to_f64(),
            "M_d Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_d_molar.value.contains(Interval::point(m_d_molar_centre)),
            "M_d Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_d_molar.value.lo, m_d_molar.value.hi,
            "ledger M_d stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_molar_mass().hash,
            physis_constants::neutron_molar_mass().hash,
            "M_d is not M_n"
        );
        assert_ne!(
            physis_constants::deuteron_molar_mass().hash,
            physis_constants::proton_molar_mass().hash,
            "M_d is not M_p"
        );
        assert_ne!(
            physis_constants::deuteron_molar_mass().hash,
            physis_constants::electron_molar_mass().hash,
            "M_d is not M_e"
        );
        assert_ne!(
            physis_constants::deuteron_molar_mass().hash,
            physis_constants::muon_molar_mass().hash,
            "M_d is not M_mu"
        );
        assert!(
            physis_constants::lookup("r_d").is_none(),
            "r_d is not a ledger name; the live name is rd"
        );
        let rd = physis_constants::deuteron_rms_charge_radius();
        let rd_centre = Ratio::new(212_799, 10i128.pow(20));
        assert_eq!(
            deuteron_rms_charge_radius().value(),
            rd_centre.to_f64(),
            "rd Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            rd.value.contains(Interval::point(rd_centre)),
            "rd Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            rd.value.lo, rd.value.hi,
            "ledger rd stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_rms_charge_radius().hash,
            physis_constants::proton_rms_charge_radius().hash,
            "rd is not rp"
        );
        assert_ne!(
            physis_constants::deuteron_rms_charge_radius().hash,
            physis_constants::classical_electron_radius().hash,
            "rd is not re"
        );
        assert_ne!(
            physis_constants::deuteron_rms_charge_radius().hash,
            physis_constants::deuteron_molar_mass().hash,
            "rd is not M_d"
        );
        assert!(
            physis_constants::lookup("mu-d").is_none(),
            "mu-d is not a ledger name; the live name is mu_d"
        );
        let mu_d = physis_constants::deuteron_magnetic_moment();
        let mu_d_centre = Ratio::new(4_330_735_094, 10i128.pow(36));
        assert_eq!(
            deuteron_magnetic_moment().value(),
            mu_d_centre.to_f64(),
            "mu_d Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_d.value.contains(Interval::point(mu_d_centre)),
            "mu_d Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_d.value.lo, mu_d.value.hi,
            "ledger mu_d stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_magnetic_moment().hash,
            physis_constants::proton_magnetic_moment().hash,
            "mu_d is not mu_p"
        );
        assert_ne!(
            physis_constants::deuteron_magnetic_moment().hash,
            physis_constants::neutron_magnetic_moment().hash,
            "mu_d is not mu_n"
        );
        assert_ne!(
            physis_constants::deuteron_magnetic_moment().hash,
            physis_constants::electron_magnetic_moment().hash,
            "mu_d is not mu_e"
        );
        assert_ne!(
            physis_constants::deuteron_magnetic_moment().hash,
            physis_constants::electron_deuteron_magnetic_moment_ratio().hash,
            "mu_d is not mu_e_mud"
        );
        assert!(
            physis_constants::lookup("mu_d/muB").is_none(),
            "mu_d/muB is not a ledger name; the live name is mu_d_muB"
        );
        let mu_d_mub = physis_constants::deuteron_magnetic_moment_to_bohr_magneton();
        let mu_d_mub_centre = Ratio::new(4_669_754_570, 10i128.pow(13));
        assert_eq!(
            deuteron_magnetic_moment_to_bohr_magneton().value(),
            mu_d_mub_centre.to_f64(),
            "mu_d_muB Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_d_mub.value.contains(Interval::point(mu_d_mub_centre)),
            "mu_d_muB Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_d_mub.value.lo, mu_d_mub.value.hi,
            "ledger mu_d_muB stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::deuteron_magnetic_moment().hash,
            "mu_d_muB is not mu_d"
        );
        assert_ne!(
            physis_constants::deuteron_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::proton_magnetic_moment_to_bohr_magneton().hash,
            "mu_d_muB is not mu_p_muB"
        );
        assert_ne!(
            physis_constants::deuteron_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::neutron_magnetic_moment_to_bohr_magneton().hash,
            "mu_d_muB is not mu_n_muB"
        );
        assert!(
            physis_constants::lookup("mu_d/muN").is_none(),
            "mu_d/muN is not a ledger name; the live name is mu_d_muN"
        );
        let mu_d_to_mu_n = physis_constants::deuteron_magnetic_moment_to_nuclear_magneton();
        let mu_d_to_mu_n_centre = Ratio::new(8_574_382_338, 10i128.pow(10));
        assert_eq!(
            deuteron_magnetic_moment_to_nuclear_magneton().value(),
            mu_d_to_mu_n_centre.to_f64(),
            "mu_d_muN Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_d_to_mu_n
                .value
                .contains(Interval::point(mu_d_to_mu_n_centre)),
            "mu_d_muN Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_d_to_mu_n.value.lo, mu_d_to_mu_n.value.hi,
            "ledger mu_d_muN stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::deuteron_magnetic_moment_to_bohr_magneton().hash,
            "mu_d_muN is not mu_d_muB"
        );
        assert_ne!(
            physis_constants::deuteron_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::proton_magnetic_moment_to_nuclear_magneton().hash,
            "mu_d_muN is not mu_p_muN"
        );
        assert_ne!(
            physis_constants::deuteron_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::neutron_magnetic_moment_to_nuclear_magneton().hash,
            "mu_d_muN is not mu_n_muN"
        );
        assert!(
            physis_constants::lookup("g_d").is_none(),
            "g_d is not a ledger name; the live name is gd"
        );
        let gd = physis_constants::deuteron_g_factor();
        let gd_centre = Ratio::new(8_574_382_338, 10i128.pow(10));
        assert_eq!(
            deuteron_g_factor().value(),
            gd_centre.to_f64(),
            "gd Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            gd.value.contains(Interval::point(gd_centre)),
            "gd Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            gd.value.lo, gd.value.hi,
            "ledger gd stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_g_factor().hash,
            physis_constants::deuteron_magnetic_moment_to_nuclear_magneton().hash,
            "gd is not mu_d_muN"
        );
        assert_ne!(
            physis_constants::deuteron_g_factor().hash,
            physis_constants::electron_g_factor().hash,
            "gd is not ge"
        );
        assert_ne!(
            physis_constants::deuteron_g_factor().hash,
            physis_constants::neutron_g_factor().hash,
            "gd is not gn"
        );
        assert_ne!(
            physis_constants::deuteron_g_factor().hash,
            physis_constants::proton_g_factor().hash,
            "gd is not gp"
        );
        assert!(
            physis_constants::lookup("mu_d/mue").is_none(),
            "mu_d/mue is not a ledger name; the live name is mu_d_mue"
        );
        let mu_d_mue = physis_constants::deuteron_electron_magnetic_moment_ratio();
        let mu_d_mue_centre = Ratio::new(-4_664_345_551, 10i128.pow(13));
        assert_eq!(
            deuteron_electron_magnetic_moment_ratio().value(),
            mu_d_mue_centre.to_f64(),
            "mu_d_mue Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_d_mue.value.contains(Interval::point(mu_d_mue_centre)),
            "mu_d_mue Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_d_mue.value.lo, mu_d_mue.value.hi,
            "ledger mu_d_mue stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_electron_magnetic_moment_ratio().hash,
            physis_constants::electron_deuteron_magnetic_moment_ratio().hash,
            "mu_d_mue is not mu_e_mud"
        );
        assert_ne!(
            physis_constants::deuteron_electron_magnetic_moment_ratio().hash,
            physis_constants::neutron_electron_magnetic_moment_ratio().hash,
            "mu_d_mue is not mu_n_mue"
        );
        assert_ne!(
            physis_constants::deuteron_electron_magnetic_moment_ratio().hash,
            physis_constants::deuteron_g_factor().hash,
            "mu_d_mue is not gd"
        );
        assert_ne!(
            physis_constants::deuteron_electron_magnetic_moment_ratio().hash,
            physis_constants::deuteron_magnetic_moment_to_bohr_magneton().hash,
            "mu_d_mue is not mu_d_muB"
        );
        assert!(
            physis_constants::lookup("mu_d/mup").is_none(),
            "mu_d/mup is not a ledger name; the live name is mu_d_mup"
        );
        let mu_d_mup = physis_constants::deuteron_proton_magnetic_moment_ratio();
        let mu_d_mup_centre = Ratio::new(30_701_220_939, 10i128.pow(11));
        assert_eq!(
            deuteron_proton_magnetic_moment_ratio().value(),
            mu_d_mup_centre.to_f64(),
            "mu_d_mup Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_d_mup.value.contains(Interval::point(mu_d_mup_centre)),
            "mu_d_mup Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_d_mup.value.lo, mu_d_mup.value.hi,
            "ledger mu_d_mup stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_proton_magnetic_moment_ratio().hash,
            physis_constants::neutron_proton_magnetic_moment_ratio().hash,
            "mu_d_mup is not mu_n_mup"
        );
        assert_ne!(
            physis_constants::deuteron_proton_magnetic_moment_ratio().hash,
            physis_constants::electron_proton_magnetic_moment_ratio().hash,
            "mu_d_mup is not mu_e_mup"
        );
        assert_ne!(
            physis_constants::deuteron_proton_magnetic_moment_ratio().hash,
            physis_constants::deuteron_proton_mass_ratio().hash,
            "mu_d_mup is not md_mp"
        );
        assert_ne!(
            physis_constants::deuteron_proton_magnetic_moment_ratio().hash,
            physis_constants::deuteron_electron_magnetic_moment_ratio().hash,
            "mu_d_mup is not mu_d_mue"
        );
        assert!(
            physis_constants::lookup("mu_d/mun").is_none(),
            "mu_d/mun is not a ledger name; the live name is mu_d_mun"
        );
        let mu_d_mun = physis_constants::deuteron_neutron_magnetic_moment_ratio();
        let mu_d_mun_centre = Ratio::new(-44_820_653, 10i128.pow(8));
        assert_eq!(
            deuteron_neutron_magnetic_moment_ratio().value(),
            mu_d_mun_centre.to_f64(),
            "mu_d_mun Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_d_mun.value.contains(Interval::point(mu_d_mun_centre)),
            "mu_d_mun Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_d_mun.value.lo, mu_d_mun.value.hi,
            "ledger mu_d_mun stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::deuteron_neutron_magnetic_moment_ratio().hash,
            physis_constants::electron_neutron_magnetic_moment_ratio().hash,
            "mu_d_mun is not mu_e_mun"
        );
        assert_ne!(
            physis_constants::deuteron_neutron_magnetic_moment_ratio().hash,
            physis_constants::proton_neutron_magnetic_moment_ratio().hash,
            "mu_d_mun is not mu_p_mun"
        );
        assert_ne!(
            physis_constants::deuteron_neutron_magnetic_moment_ratio().hash,
            physis_constants::deuteron_magnetic_moment_to_nuclear_magneton().hash,
            "mu_d_mun is not mu_d_muN"
        );
        assert_ne!(
            physis_constants::deuteron_neutron_magnetic_moment_ratio().hash,
            physis_constants::deuteron_proton_magnetic_moment_ratio().hash,
            "mu_d_mun is not mu_d_mup"
        );
        assert!(
            physis_constants::lookup("mt").is_none(),
            "mt is not a ledger name; the live name is m_t"
        );
        let m_t = physis_constants::triton_mass();
        let m_t_centre = Ratio::new(50_073_567_446, 10i128.pow(37));
        assert_eq!(
            triton_mass().value(),
            m_t_centre.to_f64(),
            "m_t Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_t.value.contains(Interval::point(m_t_centre)),
            "m_t Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_t.value.lo, m_t.value.hi,
            "ledger m_t stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::triton_mass().hash,
            physis_constants::deuteron_mass().hash,
            "m_t is not m_d"
        );
        assert_ne!(
            physis_constants::triton_mass().hash,
            physis_constants::neutron_mass().hash,
            "m_t is not m_n"
        );
        assert_ne!(
            physis_constants::triton_mass().hash,
            physis_constants::proton_mass().hash,
            "m_t is not m_p"
        );
        assert_ne!(
            physis_constants::triton_mass().hash,
            physis_constants::muon_mass().hash,
            "m_t is not m_mu"
        );
        assert_ne!(
            physis_constants::triton_mass().hash,
            physis_constants::electron_triton_mass_ratio().hash,
            "m_t is not me_mt"
        );
        assert!(
            physis_constants::lookup("mt_u").is_none(),
            "mt_u is not a ledger name; the live name is m_t_u"
        );
        let m_t_u = physis_constants::triton_mass_in_u();
        let m_t_u_centre = Ratio::new(301_550_071_621, 10i128.pow(11));
        assert_eq!(
            triton_mass_in_u().value(),
            m_t_u_centre.to_f64(),
            "m_t_u Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_t_u.value.contains(Interval::point(m_t_u_centre)),
            "m_t_u Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_t_u.value.lo, m_t_u.value.hi,
            "ledger m_t_u stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::triton_mass_in_u().hash,
            physis_constants::triton_mass().hash,
            "m_t_u is not m_t"
        );
        assert_ne!(
            physis_constants::triton_mass_in_u().hash,
            physis_constants::deuteron_mass_in_u().hash,
            "m_t_u is not m_d_u"
        );
        assert_ne!(
            physis_constants::triton_mass_in_u().hash,
            physis_constants::neutron_mass_in_u().hash,
            "m_t_u is not m_n_u"
        );
        assert_ne!(
            physis_constants::triton_mass_in_u().hash,
            physis_constants::proton_mass_in_u().hash,
            "m_t_u is not m_p_u"
        );
        assert_ne!(
            physis_constants::triton_mass_in_u().hash,
            physis_constants::muon_mass_in_u().hash,
            "m_t_u is not m_mu_u"
        );
        assert_ne!(
            physis_constants::triton_mass_in_u().hash,
            physis_constants::electron_triton_mass_ratio().hash,
            "m_t_u is not me_mt"
        );
        assert!(
            physis_constants::lookup("mtc2").is_none(),
            "mtc2 is not a ledger name; the live name is m_t_c2"
        );
        let m_t_c2 = physis_constants::triton_mass_energy_equivalent();
        let m_t_c2_centre = Ratio::new(45_003_878_060, 10i128.pow(20));
        assert_eq!(
            triton_mass_energy_equivalent().value(),
            m_t_c2_centre.to_f64(),
            "m_t_c2 Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_t_c2.value.contains(Interval::point(m_t_c2_centre)),
            "m_t_c2 Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_t_c2.value.lo, m_t_c2.value.hi,
            "ledger m_t_c2 stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::triton_mass_energy_equivalent().hash,
            physis_constants::triton_mass().hash,
            "m_t_c2 is not m_t"
        );
        assert_ne!(
            physis_constants::triton_mass_energy_equivalent().hash,
            physis_constants::triton_mass_in_u().hash,
            "m_t_c2 is not m_t_u"
        );
        assert_ne!(
            physis_constants::triton_mass_energy_equivalent().hash,
            physis_constants::deuteron_mass_energy_equivalent().hash,
            "m_t_c2 is not m_d_c2"
        );
        assert_ne!(
            physis_constants::triton_mass_energy_equivalent().hash,
            physis_constants::neutron_mass_energy_equivalent().hash,
            "m_t_c2 is not m_n_c2"
        );
        assert_ne!(
            physis_constants::triton_mass_energy_equivalent().hash,
            physis_constants::proton_mass_energy_equivalent().hash,
            "m_t_c2 is not m_p_c2"
        );
        assert_ne!(
            physis_constants::triton_mass_energy_equivalent().hash,
            physis_constants::muon_mass_energy_equivalent().hash,
            "m_t_c2 is not m_mu_c2"
        );
        assert_ne!(
            physis_constants::triton_mass_energy_equivalent().hash,
            physis_constants::electron_volt().hash,
            "m_t_c2 is not eV"
        );
        assert!(
            physis_constants::lookup("mtc2_MeV").is_none(),
            "mtc2_MeV is not a ledger name; the live name is m_t_c2_MeV"
        );
        let m_t_c2_mev = physis_constants::triton_mass_energy_equivalent_in_mev();
        let m_t_c2_mev_centre = Ratio::new(280_892_113_298, 10i128.pow(8));
        assert_eq!(
            triton_mass_energy_equivalent_in_mev().value(),
            m_t_c2_mev_centre.to_f64(),
            "m_t_c2_MeV Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_t_c2_mev
                .value
                .contains(Interval::point(m_t_c2_mev_centre)),
            "m_t_c2_MeV Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_t_c2_mev.value.lo, m_t_c2_mev.value.hi,
            "ledger m_t_c2_MeV stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::triton_mass_energy_equivalent_in_mev().hash,
            physis_constants::triton_mass_energy_equivalent().hash,
            "m_t_c2_MeV is not m_t_c2"
        );
        assert_ne!(
            physis_constants::triton_mass_energy_equivalent_in_mev().hash,
            physis_constants::deuteron_mass_energy_equivalent_in_mev().hash,
            "m_t_c2_MeV is not m_d_c2_MeV"
        );
        assert_ne!(
            physis_constants::triton_mass_energy_equivalent_in_mev().hash,
            physis_constants::neutron_mass_energy_equivalent_in_mev().hash,
            "m_t_c2_MeV is not m_n_c2_MeV"
        );
        assert_ne!(
            physis_constants::triton_mass_energy_equivalent_in_mev().hash,
            physis_constants::proton_mass_energy_equivalent_in_mev().hash,
            "m_t_c2_MeV is not m_p_c2_MeV"
        );
        assert_ne!(
            physis_constants::triton_mass_energy_equivalent_in_mev().hash,
            physis_constants::muon_mass_energy_equivalent_in_mev().hash,
            "m_t_c2_MeV is not m_mu_c2_MeV"
        );
        assert_ne!(
            physis_constants::triton_mass_energy_equivalent_in_mev().hash,
            physis_constants::electron_volt().hash,
            "m_t_c2_MeV is not eV"
        );
        assert!(
            physis_constants::lookup("mt/me").is_none(),
            "mt/me is not a ledger name; the live name is mt_me"
        );
        let mt_me = physis_constants::triton_electron_mass_ratio();
        let mt_me_centre = Ratio::new(549_692_153_573, 10i128.pow(8));
        assert_eq!(
            triton_electron_mass_ratio().value(),
            mt_me_centre.to_f64(),
            "mt_me Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mt_me.value.contains(Interval::point(mt_me_centre)),
            "mt_me Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mt_me.value.lo, mt_me.value.hi,
            "ledger mt_me stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::triton_electron_mass_ratio().hash,
            physis_constants::electron_triton_mass_ratio().hash,
            "mt_me is not me_mt"
        );
        assert_ne!(
            physis_constants::triton_electron_mass_ratio().hash,
            physis_constants::deuteron_electron_mass_ratio().hash,
            "mt_me is not md_me"
        );
        assert_ne!(
            physis_constants::triton_electron_mass_ratio().hash,
            physis_constants::neutron_electron_mass_ratio().hash,
            "mt_me is not mn_me"
        );
        assert_ne!(
            physis_constants::triton_electron_mass_ratio().hash,
            physis_constants::proton_electron_mass_ratio().hash,
            "mt_me is not mp_me"
        );
        assert_ne!(
            physis_constants::triton_electron_mass_ratio().hash,
            physis_constants::muon_electron_mass_ratio().hash,
            "mt_me is not mmu_me"
        );
        assert!(
            physis_constants::lookup("mt/mp").is_none(),
            "mt/mp is not a ledger name; the live name is mt_mp"
        );
        let mt_mp = physis_constants::triton_proton_mass_ratio();
        let mt_mp_centre = Ratio::new(299_371_703_414, 10i128.pow(11));
        assert_eq!(
            triton_proton_mass_ratio().value(),
            mt_mp_centre.to_f64(),
            "mt_mp Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mt_mp.value.contains(Interval::point(mt_mp_centre)),
            "mt_mp Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mt_mp.value.lo, mt_mp.value.hi,
            "ledger mt_mp stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::triton_proton_mass_ratio().hash,
            physis_constants::deuteron_proton_mass_ratio().hash,
            "mt_mp is not md_mp"
        );
        assert_ne!(
            physis_constants::triton_proton_mass_ratio().hash,
            physis_constants::neutron_proton_mass_ratio().hash,
            "mt_mp is not mn_mp"
        );
        assert_ne!(
            physis_constants::triton_proton_mass_ratio().hash,
            physis_constants::proton_neutron_mass_ratio().hash,
            "mt_mp is not mp_mn"
        );
        assert_ne!(
            physis_constants::triton_proton_mass_ratio().hash,
            physis_constants::triton_electron_mass_ratio().hash,
            "mt_mp is not mt_me"
        );
        assert!(
            physis_constants::lookup("Mt").is_none(),
            "Mt is not a ledger name; the live name is M_t"
        );
        let m_t_molar = physis_constants::triton_molar_mass();
        let m_t_molar_centre = Ratio::new(301_550_071_517, 10i128.pow(14));
        assert_eq!(
            triton_molar_mass().value(),
            m_t_molar_centre.to_f64(),
            "M_t Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_t_molar.value.contains(Interval::point(m_t_molar_centre)),
            "M_t Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_t_molar.value.lo, m_t_molar.value.hi,
            "ledger M_t stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::triton_molar_mass().hash,
            physis_constants::neutron_molar_mass().hash,
            "M_t is not M_n"
        );
        assert_ne!(
            physis_constants::triton_molar_mass().hash,
            physis_constants::proton_molar_mass().hash,
            "M_t is not M_p"
        );
        assert_ne!(
            physis_constants::triton_molar_mass().hash,
            physis_constants::electron_molar_mass().hash,
            "M_t is not M_e"
        );
        assert_ne!(
            physis_constants::triton_molar_mass().hash,
            physis_constants::muon_molar_mass().hash,
            "M_t is not M_mu"
        );
        assert_ne!(
            physis_constants::triton_molar_mass().hash,
            physis_constants::triton_mass().hash,
            "M_t is not m_t"
        );
        assert!(
            physis_constants::lookup("mut").is_none(),
            "mut is not a ledger name; the live name is mu_t"
        );
        let mu_t = physis_constants::triton_magnetic_moment();
        let mu_t_centre = Ratio::new(15_046_095_202, 10i128.pow(36));
        assert_eq!(
            triton_magnetic_moment().value(),
            mu_t_centre.to_f64(),
            "mu_t Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_t.value.contains(Interval::point(mu_t_centre)),
            "mu_t Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_t.value.lo, mu_t.value.hi,
            "ledger mu_t stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment().hash,
            physis_constants::deuteron_magnetic_moment().hash,
            "mu_t is not mu_d"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment().hash,
            physis_constants::proton_magnetic_moment().hash,
            "mu_t is not mu_p"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment().hash,
            physis_constants::neutron_magnetic_moment().hash,
            "mu_t is not mu_n"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment().hash,
            physis_constants::electron_magnetic_moment().hash,
            "mu_t is not mu_e"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment().hash,
            physis_constants::muon_magnetic_moment().hash,
            "mu_t is not mu_mu"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment().hash,
            physis_constants::vacuum_permeability().hash,
            "mu_t is not mu0"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment().hash,
            physis_constants::triton_molar_mass().hash,
            "mu_t is not M_t"
        );
        assert!(
            physis_constants::lookup("mut_muB").is_none(),
            "mut_muB is not a ledger name; the live name is mu_t_muB"
        );
        let mu_t_mub = physis_constants::triton_magnetic_moment_to_bohr_magneton();
        let mu_t_mub_centre = Ratio::new(16_223_936_651, 10i128.pow(13));
        assert_eq!(
            triton_magnetic_moment_to_bohr_magneton().value(),
            mu_t_mub_centre.to_f64(),
            "mu_t_muB Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_t_mub.value.contains(Interval::point(mu_t_mub_centre)),
            "mu_t_muB Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_t_mub.value.lo, mu_t_mub.value.hi,
            "ledger mu_t_muB stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::triton_magnetic_moment().hash,
            "mu_t_muB is not mu_t"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::deuteron_magnetic_moment_to_bohr_magneton().hash,
            "mu_t_muB is not mu_d_muB"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::proton_magnetic_moment_to_bohr_magneton().hash,
            "mu_t_muB is not mu_p_muB"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::neutron_magnetic_moment_to_bohr_magneton().hash,
            "mu_t_muB is not mu_n_muB"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::electron_magnetic_moment_to_bohr_magneton().hash,
            "mu_t_muB is not mu_e_muB"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::muon_magnetic_moment_to_bohr_magneton().hash,
            "mu_t_muB is not mu_mu_muB"
        );
        assert!(
            physis_constants::lookup("mu_t/muN").is_none(),
            "mu_t/muN is not a ledger name; the live name is mu_t_muN"
        );
        let mu_t_to_mu_n = physis_constants::triton_magnetic_moment_to_nuclear_magneton();
        let mu_t_to_mu_n_centre = Ratio::new(29_789_624_656, 10i128.pow(10));
        assert_eq!(
            triton_magnetic_moment_to_nuclear_magneton().value(),
            mu_t_to_mu_n_centre.to_f64(),
            "mu_t_muN Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_t_to_mu_n
                .value
                .contains(Interval::point(mu_t_to_mu_n_centre)),
            "mu_t_muN Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_t_to_mu_n.value.lo, mu_t_to_mu_n.value.hi,
            "ledger mu_t_muN stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::triton_magnetic_moment().hash,
            "mu_t_muN is not mu_t"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::triton_magnetic_moment_to_bohr_magneton().hash,
            "mu_t_muN is not mu_t_muB"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::deuteron_magnetic_moment_to_nuclear_magneton().hash,
            "mu_t_muN is not mu_d_muN"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::proton_magnetic_moment_to_nuclear_magneton().hash,
            "mu_t_muN is not mu_p_muN"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::neutron_magnetic_moment_to_nuclear_magneton().hash,
            "mu_t_muN is not mu_n_muN"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::electron_magnetic_moment_to_nuclear_magneton().hash,
            "mu_t_muN is not mu_e_muN"
        );
        assert_ne!(
            physis_constants::triton_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::muon_magnetic_moment_to_nuclear_magneton().hash,
            "mu_t_muN is not mu_mu_muN"
        );
        assert!(
            physis_constants::lookup("g_t").is_none(),
            "g_t is not a ledger name; the live name is gt"
        );
        let gt = physis_constants::triton_g_factor();
        let gt_centre = Ratio::new(5_957_924_931, 10i128.pow(9));
        assert_eq!(
            triton_g_factor().value(),
            gt_centre.to_f64(),
            "gt Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            gt.value.contains(Interval::point(gt_centre)),
            "gt Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            gt.value.lo, gt.value.hi,
            "ledger gt stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::triton_g_factor().hash,
            physis_constants::triton_magnetic_moment_to_nuclear_magneton().hash,
            "gt is not mu_t_muN"
        );
        assert_ne!(
            physis_constants::triton_g_factor().hash,
            physis_constants::deuteron_g_factor().hash,
            "gt is not gd"
        );
        assert_ne!(
            physis_constants::triton_g_factor().hash,
            physis_constants::electron_g_factor().hash,
            "gt is not ge"
        );
        assert_ne!(
            physis_constants::triton_g_factor().hash,
            physis_constants::muon_g_factor().hash,
            "gt is not gmu"
        );
        assert_ne!(
            physis_constants::triton_g_factor().hash,
            physis_constants::proton_g_factor().hash,
            "gt is not gp"
        );
        assert_ne!(
            physis_constants::triton_g_factor().hash,
            physis_constants::neutron_g_factor().hash,
            "gt is not gn"
        );
        assert!(
            physis_constants::lookup("mh").is_none(),
            "mh is not a ledger name; the live name is m_h"
        );
        let m_h = physis_constants::helion_mass();
        let m_h_centre = Ratio::new(50_064_127_796, 10i128.pow(37));
        assert_eq!(
            helion_mass().value(),
            m_h_centre.to_f64(),
            "m_h Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_h.value.contains(Interval::point(m_h_centre)),
            "m_h Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_h.value.lo, m_h.value.hi,
            "ledger m_h stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::helion_mass().hash,
            physis_constants::triton_mass().hash,
            "m_h is not m_t"
        );
        assert_ne!(
            physis_constants::helion_mass().hash,
            physis_constants::deuteron_mass().hash,
            "m_h is not m_d"
        );
        assert_ne!(
            physis_constants::helion_mass().hash,
            physis_constants::neutron_mass().hash,
            "m_h is not m_n"
        );
        assert_ne!(
            physis_constants::helion_mass().hash,
            physis_constants::proton_mass().hash,
            "m_h is not m_p"
        );
        assert_ne!(
            physis_constants::helion_mass().hash,
            physis_constants::muon_mass().hash,
            "m_h is not m_mu"
        );
        assert_ne!(
            physis_constants::helion_mass().hash,
            physis_constants::electron_helion_mass_ratio().hash,
            "m_h is not me_mh"
        );
        assert!(
            physis_constants::lookup("mh_u").is_none(),
            "mh_u is not a ledger name; the live name is m_h_u"
        );
        let m_h_u = physis_constants::helion_mass_in_u();
        let m_h_u_centre = Ratio::new(3_014_932_247_175, 10i128.pow(12));
        assert_eq!(
            helion_mass_in_u().value(),
            m_h_u_centre.to_f64(),
            "m_h_u Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_h_u.value.contains(Interval::point(m_h_u_centre)),
            "m_h_u Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_h_u.value.lo, m_h_u.value.hi,
            "ledger m_h_u stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::helion_mass_in_u().hash,
            physis_constants::helion_mass().hash,
            "m_h_u is not m_h"
        );
        assert_ne!(
            physis_constants::helion_mass_in_u().hash,
            physis_constants::triton_mass_in_u().hash,
            "m_h_u is not m_t_u"
        );
        assert_ne!(
            physis_constants::helion_mass_in_u().hash,
            physis_constants::deuteron_mass_in_u().hash,
            "m_h_u is not m_d_u"
        );
        assert_ne!(
            physis_constants::helion_mass_in_u().hash,
            physis_constants::neutron_mass_in_u().hash,
            "m_h_u is not m_n_u"
        );
        assert_ne!(
            physis_constants::helion_mass_in_u().hash,
            physis_constants::proton_mass_in_u().hash,
            "m_h_u is not m_p_u"
        );
        assert_ne!(
            physis_constants::helion_mass_in_u().hash,
            physis_constants::muon_mass_in_u().hash,
            "m_h_u is not m_mu_u"
        );
        assert_ne!(
            physis_constants::helion_mass_in_u().hash,
            physis_constants::electron_helion_mass_ratio().hash,
            "m_h_u is not me_mh"
        );
        assert_ne!(
            physis_constants::helion_mass_in_u().hash,
            physis_constants::electron_molar_mass().hash,
            "m_h_u is not M_e"
        );
        assert!(
            physis_constants::lookup("mhc2").is_none(),
            "mhc2 is not a ledger name; the live name is m_h_c2"
        );
        let m_h_c2 = physis_constants::helion_mass_energy_equivalent();
        let m_h_c2_centre = Ratio::new(44_995_394_125, 10i128.pow(20));
        assert_eq!(
            helion_mass_energy_equivalent().value(),
            m_h_c2_centre.to_f64(),
            "m_h_c2 Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_h_c2.value.contains(Interval::point(m_h_c2_centre)),
            "m_h_c2 Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_h_c2.value.lo, m_h_c2.value.hi,
            "ledger m_h_c2 stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent().hash,
            physis_constants::helion_mass().hash,
            "m_h_c2 is not m_h"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent().hash,
            physis_constants::helion_mass_in_u().hash,
            "m_h_c2 is not m_h_u"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent().hash,
            physis_constants::triton_mass_energy_equivalent().hash,
            "m_h_c2 is not m_t_c2"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent().hash,
            physis_constants::deuteron_mass_energy_equivalent().hash,
            "m_h_c2 is not m_d_c2"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent().hash,
            physis_constants::neutron_mass_energy_equivalent().hash,
            "m_h_c2 is not m_n_c2"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent().hash,
            physis_constants::proton_mass_energy_equivalent().hash,
            "m_h_c2 is not m_p_c2"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent().hash,
            physis_constants::muon_mass_energy_equivalent().hash,
            "m_h_c2 is not m_mu_c2"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent().hash,
            physis_constants::electron_volt().hash,
            "m_h_c2 is not eV"
        );
        assert!(
            physis_constants::lookup("mhc2_MeV").is_none(),
            "mhc2_MeV is not a ledger name; the live name is m_h_c2_MeV"
        );
        let m_h_c2_mev = physis_constants::helion_mass_energy_equivalent_in_mev();
        let m_h_c2_mev_centre = Ratio::new(280_839_160_743, 10i128.pow(8));
        assert_eq!(
            helion_mass_energy_equivalent_in_mev().value(),
            m_h_c2_mev_centre.to_f64(),
            "m_h_c2_MeV Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_h_c2_mev
                .value
                .contains(Interval::point(m_h_c2_mev_centre)),
            "m_h_c2_MeV Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_h_c2_mev.value.lo, m_h_c2_mev.value.hi,
            "ledger m_h_c2_MeV stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent_in_mev().hash,
            physis_constants::helion_mass_energy_equivalent().hash,
            "m_h_c2_MeV is not m_h_c2"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent_in_mev().hash,
            physis_constants::triton_mass_energy_equivalent_in_mev().hash,
            "m_h_c2_MeV is not m_t_c2_MeV"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent_in_mev().hash,
            physis_constants::deuteron_mass_energy_equivalent_in_mev().hash,
            "m_h_c2_MeV is not m_d_c2_MeV"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent_in_mev().hash,
            physis_constants::neutron_mass_energy_equivalent_in_mev().hash,
            "m_h_c2_MeV is not m_n_c2_MeV"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent_in_mev().hash,
            physis_constants::proton_mass_energy_equivalent_in_mev().hash,
            "m_h_c2_MeV is not m_p_c2_MeV"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent_in_mev().hash,
            physis_constants::muon_mass_energy_equivalent_in_mev().hash,
            "m_h_c2_MeV is not m_mu_c2_MeV"
        );
        assert_ne!(
            physis_constants::helion_mass_energy_equivalent_in_mev().hash,
            physis_constants::electron_volt().hash,
            "m_h_c2_MeV is not eV"
        );
        assert!(
            physis_constants::lookup("mh/me").is_none(),
            "mh/me is not a ledger name; the live name is mh_me"
        );
        let mh_me = physis_constants::helion_electron_mass_ratio();
        let mh_me_centre = Ratio::new(549_588_528_007, 10i128.pow(8));
        assert_eq!(
            helion_electron_mass_ratio().value(),
            mh_me_centre.to_f64(),
            "mh_me Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mh_me.value.contains(Interval::point(mh_me_centre)),
            "mh_me Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mh_me.value.lo, mh_me.value.hi,
            "ledger mh_me stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::helion_electron_mass_ratio().hash,
            physis_constants::electron_helion_mass_ratio().hash,
            "mh_me is not me_mh"
        );
        assert_ne!(
            physis_constants::helion_electron_mass_ratio().hash,
            physis_constants::triton_electron_mass_ratio().hash,
            "mh_me is not mt_me"
        );
        assert_ne!(
            physis_constants::helion_electron_mass_ratio().hash,
            physis_constants::deuteron_electron_mass_ratio().hash,
            "mh_me is not md_me"
        );
        assert_ne!(
            physis_constants::helion_electron_mass_ratio().hash,
            physis_constants::neutron_electron_mass_ratio().hash,
            "mh_me is not mn_me"
        );
        assert_ne!(
            physis_constants::helion_electron_mass_ratio().hash,
            physis_constants::proton_electron_mass_ratio().hash,
            "mh_me is not mp_me"
        );
        assert_ne!(
            physis_constants::helion_electron_mass_ratio().hash,
            physis_constants::muon_electron_mass_ratio().hash,
            "mh_me is not mmu_me"
        );
        assert_ne!(
            physis_constants::helion_electron_mass_ratio().hash,
            physis_constants::helion_mass_energy_equivalent_in_mev().hash,
            "mh_me is not m_h_c2_MeV"
        );
        assert!(
            physis_constants::lookup("mh/mp").is_none(),
            "mh/mp is not a ledger name; the live name is mh_mp"
        );
        let mh_mp = physis_constants::helion_proton_mass_ratio();
        let mh_mp_centre = Ratio::new(299_315_267_167, 10i128.pow(11));
        assert_eq!(
            helion_proton_mass_ratio().value(),
            mh_mp_centre.to_f64(),
            "mh_mp Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mh_mp.value.contains(Interval::point(mh_mp_centre)),
            "mh_mp Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mh_mp.value.lo, mh_mp.value.hi,
            "ledger mh_mp stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::helion_proton_mass_ratio().hash,
            physis_constants::triton_proton_mass_ratio().hash,
            "mh_mp is not mt_mp"
        );
        assert_ne!(
            physis_constants::helion_proton_mass_ratio().hash,
            physis_constants::deuteron_proton_mass_ratio().hash,
            "mh_mp is not md_mp"
        );
        assert_ne!(
            physis_constants::helion_proton_mass_ratio().hash,
            physis_constants::neutron_proton_mass_ratio().hash,
            "mh_mp is not mn_mp"
        );
        assert_ne!(
            physis_constants::helion_proton_mass_ratio().hash,
            physis_constants::proton_neutron_mass_ratio().hash,
            "mh_mp is not mp_mn"
        );
        assert_ne!(
            physis_constants::helion_proton_mass_ratio().hash,
            physis_constants::helion_electron_mass_ratio().hash,
            "mh_mp is not mh_me"
        );
        assert_ne!(
            physis_constants::helion_proton_mass_ratio().hash,
            physis_constants::helion_mass().hash,
            "mh_mp is not m_h"
        );
        assert_ne!(
            physis_constants::helion_proton_mass_ratio().hash,
            physis_constants::proton_mass().hash,
            "mh_mp is not m_p"
        );
        assert!(
            physis_constants::lookup("Mh").is_none(),
            "Mh is not a ledger name; the live name is M_h"
        );
        let m_h_molar = physis_constants::helion_molar_mass();
        let m_h_molar_centre = Ratio::new(301_493_224_613, 10i128.pow(14));
        assert_eq!(
            helion_molar_mass().value(),
            m_h_molar_centre.to_f64(),
            "M_h Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            m_h_molar.value.contains(Interval::point(m_h_molar_centre)),
            "M_h Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_h_molar.value.lo, m_h_molar.value.hi,
            "ledger M_h stays an Interval; the Qty is not that Interval"
        );
        assert_ne!(
            physis_constants::helion_molar_mass().hash,
            physis_constants::triton_molar_mass().hash,
            "M_h is not M_t"
        );
        assert_ne!(
            physis_constants::helion_molar_mass().hash,
            physis_constants::deuteron_molar_mass().hash,
            "M_h is not M_d"
        );
        assert_ne!(
            physis_constants::helion_molar_mass().hash,
            physis_constants::neutron_molar_mass().hash,
            "M_h is not M_n"
        );
        assert_ne!(
            physis_constants::helion_molar_mass().hash,
            physis_constants::proton_molar_mass().hash,
            "M_h is not M_p"
        );
        assert_ne!(
            physis_constants::helion_molar_mass().hash,
            physis_constants::electron_molar_mass().hash,
            "M_h is not M_e"
        );
        assert_ne!(
            physis_constants::helion_molar_mass().hash,
            physis_constants::muon_molar_mass().hash,
            "M_h is not M_mu"
        );
        assert_ne!(
            physis_constants::helion_molar_mass().hash,
            physis_constants::helion_mass().hash,
            "M_h is not m_h"
        );
        assert_ne!(
            physis_constants::helion_molar_mass().hash,
            physis_constants::helion_mass_in_u().hash,
            "M_h is not m_h_u"
        );
        assert_ne!(
            physis_constants::helion_molar_mass().hash,
            physis_constants::helion_proton_mass_ratio().hash,
            "M_h is not mh_mp"
        );
        assert!(
            physis_constants::lookup("muh").is_none(),
            "muh is not a ledger name; the live name is mu_h"
        );
        let mu_h = physis_constants::helion_magnetic_moment();
        let mu_h_centre = Ratio::new(-1_074_617_532, 10i128.pow(35));
        assert_eq!(
            helion_magnetic_moment().value(),
            -1.074_617_532e-26,
            "mu_h Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_h.value.contains(Interval::point(mu_h_centre)),
            "mu_h Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_h.value.lo, mu_h.value.hi,
            "ledger mu_h stays an Interval; the Qty is not that Interval"
        );
        assert!(
            mu_h.value.hi < Ratio::int(0),
            "ledger mu_h stays the signed helion moment"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment().hash,
            physis_constants::triton_magnetic_moment().hash,
            "mu_h is not mu_t"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment().hash,
            physis_constants::deuteron_magnetic_moment().hash,
            "mu_h is not mu_d"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment().hash,
            physis_constants::neutron_magnetic_moment().hash,
            "mu_h is not mu_n"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment().hash,
            physis_constants::proton_magnetic_moment().hash,
            "mu_h is not mu_p"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment().hash,
            physis_constants::electron_magnetic_moment().hash,
            "mu_h is not mu_e"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment().hash,
            physis_constants::muon_magnetic_moment().hash,
            "mu_h is not mu_mu"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment().hash,
            physis_constants::vacuum_permeability().hash,
            "mu_h is not mu0"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment().hash,
            physis_constants::helion_molar_mass().hash,
            "mu_h is not M_h"
        );
        assert!(
            physis_constants::lookup("muh_muB").is_none(),
            "muh_muB is not a ledger name; the live name is mu_h_muB"
        );
        let mu_h_mub = physis_constants::helion_magnetic_moment_to_bohr_magneton();
        let mu_h_mub_centre = Ratio::new(-1_158_740_958, 10i128.pow(12));
        assert_eq!(
            helion_magnetic_moment_to_bohr_magneton().value(),
            mu_h_mub_centre.to_f64(),
            "mu_h_muB Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_h_mub.value.contains(Interval::point(mu_h_mub_centre)),
            "mu_h_muB Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_h_mub.value.lo, mu_h_mub.value.hi,
            "ledger mu_h_muB stays an Interval; the Qty is not that Interval"
        );
        assert!(
            mu_h_mub.value.hi < Ratio::int(0),
            "ledger mu_h_muB stays the signed helion Bohr-magneton ratio"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::helion_magnetic_moment().hash,
            "mu_h_muB is not mu_h"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::triton_magnetic_moment_to_bohr_magneton().hash,
            "mu_h_muB is not mu_t_muB"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::deuteron_magnetic_moment_to_bohr_magneton().hash,
            "mu_h_muB is not mu_d_muB"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::neutron_magnetic_moment_to_bohr_magneton().hash,
            "mu_h_muB is not mu_n_muB"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::proton_magnetic_moment_to_bohr_magneton().hash,
            "mu_h_muB is not mu_p_muB"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::electron_magnetic_moment_to_bohr_magneton().hash,
            "mu_h_muB is not mu_e_muB"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::muon_magnetic_moment_to_bohr_magneton().hash,
            "mu_h_muB is not mu_mu_muB"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::vacuum_permeability().hash,
            "mu_h_muB is not mu0"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::helion_molar_mass().hash,
            "mu_h_muB is not M_h"
        );
        assert!(
            physis_constants::lookup("mu_h/muN").is_none(),
            "mu_h/muN is not a ledger name; the live name is mu_h_muN"
        );
        let mu_h_to_mu_n = physis_constants::helion_magnetic_moment_to_nuclear_magneton();
        let mu_h_to_mu_n_centre = Ratio::new(-2_127_625_307, 10i128.pow(9));
        assert_eq!(
            helion_magnetic_moment_to_nuclear_magneton().value(),
            mu_h_to_mu_n_centre.to_f64(),
            "mu_h_muN Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            mu_h_to_mu_n
                .value
                .contains(Interval::point(mu_h_to_mu_n_centre)),
            "mu_h_muN Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu_h_to_mu_n.value.lo, mu_h_to_mu_n.value.hi,
            "ledger mu_h_muN stays an Interval; the Qty is not that Interval"
        );
        assert!(
            mu_h_to_mu_n.value.hi < Ratio::int(0),
            "ledger mu_h_muN stays the signed helion nuclear-magneton ratio"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::helion_magnetic_moment().hash,
            "mu_h_muN is not mu_h"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::helion_magnetic_moment_to_bohr_magneton().hash,
            "mu_h_muN is not mu_h_muB"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::triton_magnetic_moment_to_nuclear_magneton().hash,
            "mu_h_muN is not mu_t_muN"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::deuteron_magnetic_moment_to_nuclear_magneton().hash,
            "mu_h_muN is not mu_d_muN"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::proton_magnetic_moment_to_nuclear_magneton().hash,
            "mu_h_muN is not mu_p_muN"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::neutron_magnetic_moment_to_nuclear_magneton().hash,
            "mu_h_muN is not mu_n_muN"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::electron_magnetic_moment_to_nuclear_magneton().hash,
            "mu_h_muN is not mu_e_muN"
        );
        assert_ne!(
            physis_constants::helion_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::muon_magnetic_moment_to_nuclear_magneton().hash,
            "mu_h_muN is not mu_mu_muN"
        );
        assert!(
            physis_constants::lookup("g_h").is_none(),
            "g_h is not a ledger name; the live name is gh"
        );
        let gh = physis_constants::helion_g_factor();
        let gh_centre = Ratio::new(-4_255_250_615, 10i128.pow(9));
        assert_eq!(
            helion_g_factor().value(),
            gh_centre.to_f64(),
            "gh Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert!(
            gh.value.contains(Interval::point(gh_centre)),
            "gh Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            gh.value.lo, gh.value.hi,
            "ledger gh stays an Interval; the Qty is not that Interval"
        );
        assert!(
            gh.value.hi < Ratio::int(0),
            "ledger gh stays the signed helion g-factor"
        );
        assert_ne!(
            physis_constants::helion_g_factor().hash,
            physis_constants::helion_magnetic_moment_to_nuclear_magneton().hash,
            "gh is not mu_h_muN"
        );
        assert_ne!(
            physis_constants::helion_g_factor().hash,
            physis_constants::triton_g_factor().hash,
            "gh is not gt"
        );
        assert_ne!(
            physis_constants::helion_g_factor().hash,
            physis_constants::deuteron_g_factor().hash,
            "gh is not gd"
        );
        assert_ne!(
            physis_constants::helion_g_factor().hash,
            physis_constants::electron_g_factor().hash,
            "gh is not ge"
        );
        assert_ne!(
            physis_constants::helion_g_factor().hash,
            physis_constants::muon_g_factor().hash,
            "gh is not gmu"
        );
        assert_ne!(
            physis_constants::helion_g_factor().hash,
            physis_constants::proton_g_factor().hash,
            "gh is not gp"
        );
        assert_ne!(
            physis_constants::helion_g_factor().hash,
            physis_constants::neutron_g_factor().hash,
            "gh is not gn"
        );
        assert!(
            physis_constants::lookup("mu_0h").is_none(),
            "mu_0h is not a ledger name; the live name is mu0h"
        );
        let mu0h = physis_constants::shielded_helion_magnetic_moment();
        let mu0h_centre = Ratio::new(-1_074_553_090, 10i128.pow(35));
        assert_eq!(
            shielded_helion_magnetic_moment().value(),
            -1.074_553_090e-26,
            "mu0h Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_ne!(
            shielded_helion_magnetic_moment().value(),
            mu0h_centre.to_f64(),
            "Ratio::to_f64 on the 10^35 centre is one ulp from the CODATA decimal"
        );
        assert!(
            mu0h.value.contains(Interval::point(mu0h_centre)),
            "mu0h Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu0h.value.lo, mu0h.value.hi,
            "ledger mu0h stays an Interval; the Qty is not that Interval"
        );
        assert!(
            mu0h.value.hi < Ratio::int(0),
            "ledger mu0h stays the signed shielded helion moment"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment().hash,
            physis_constants::helion_magnetic_moment().hash,
            "mu0h is not mu_h"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment().hash,
            physis_constants::shielded_proton_magnetic_moment().hash,
            "mu0h is not mu0p"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment().hash,
            physis_constants::electron_to_shielded_helion_magnetic_moment_ratio().hash,
            "mu0h is not mu_e_mu0h"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment().hash,
            physis_constants::vacuum_permeability().hash,
            "mu0h is not mu0"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment().hash,
            physis_constants::helion_g_factor().hash,
            "mu0h is not gh"
        );
        assert!(
            physis_constants::lookup("mu0h_mub").is_none(),
            "mu0h_mub is not a ledger name; the live name is mu0h_muB"
        );
        let mu0h_mub = physis_constants::shielded_helion_magnetic_moment_to_bohr_magneton();
        let mu0h_mub_centre = Ratio::new(-1_158_671_471, 10i128.pow(12));
        assert_eq!(
            shielded_helion_magnetic_moment_to_bohr_magneton().value(),
            -1.158_671_471e-3,
            "mu0h_muB Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            shielded_helion_magnetic_moment_to_bohr_magneton().value(),
            mu0h_mub_centre.to_f64(),
            "mu0h_muB Qty locksteps to Ratio::to_f64 on the 10^12 centre"
        );
        assert!(
            mu0h_mub.value.contains(Interval::point(mu0h_mub_centre)),
            "mu0h_muB Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu0h_mub.value.lo, mu0h_mub.value.hi,
            "ledger mu0h_muB stays an Interval; the Qty is not that Interval"
        );
        assert!(
            mu0h_mub.value.hi < Ratio::int(0),
            "ledger mu0h_muB stays the signed shielded helion Bohr-magneton ratio"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::shielded_helion_magnetic_moment().hash,
            "mu0h_muB is not mu0h"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::helion_magnetic_moment_to_bohr_magneton().hash,
            "mu0h_muB is not mu_h_muB"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::shielded_proton_magnetic_moment_to_bohr_magneton().hash,
            "mu0h_muB is not mu0p_muB"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::triton_magnetic_moment_to_bohr_magneton().hash,
            "mu0h_muB is not mu_t_muB"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::electron_magnetic_moment_to_bohr_magneton().hash,
            "mu0h_muB is not mu_e_muB"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::vacuum_permeability().hash,
            "mu0h_muB is not mu0"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment_to_bohr_magneton().hash,
            physis_constants::helion_g_factor().hash,
            "mu0h_muB is not gh"
        );
        assert!(
            physis_constants::lookup("mu0h_mun").is_none(),
            "mu0h_mun is not a ledger name; the live name is mu0h_muN"
        );
        let mu0h_mun = physis_constants::shielded_helion_magnetic_moment_to_nuclear_magneton();
        let mu0h_mun_centre = Ratio::new(-2_127_497_719, 10i128.pow(9));
        assert_eq!(
            shielded_helion_magnetic_moment_to_nuclear_magneton().value(),
            -2.127_497_719,
            "mu0h_muN Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            shielded_helion_magnetic_moment_to_nuclear_magneton().value(),
            mu0h_mun_centre.to_f64(),
            "mu0h_muN Qty locksteps to Ratio::to_f64 on the 10^9 centre"
        );
        assert!(
            mu0h_mun.value.contains(Interval::point(mu0h_mun_centre)),
            "mu0h_muN Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu0h_mun.value.lo, mu0h_mun.value.hi,
            "ledger mu0h_muN stays an Interval; the Qty is not that Interval"
        );
        assert!(
            mu0h_mun.value.hi < Ratio::int(0),
            "ledger mu0h_muN stays the signed shielded helion nuclear-magneton ratio"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::shielded_helion_magnetic_moment().hash,
            "mu0h_muN is not mu0h"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::shielded_helion_magnetic_moment_to_bohr_magneton().hash,
            "mu0h_muN is not mu0h_muB"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::helion_magnetic_moment_to_nuclear_magneton().hash,
            "mu0h_muN is not mu_h_muN"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::shielded_proton_magnetic_moment_to_nuclear_magneton().hash,
            "mu0h_muN is not mu0p_muN"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::triton_magnetic_moment_to_nuclear_magneton().hash,
            "mu0h_muN is not mu_t_muN"
        );
        assert_ne!(
            physis_constants::shielded_helion_magnetic_moment_to_nuclear_magneton().hash,
            physis_constants::helion_g_factor().hash,
            "mu0h_muN is not gh"
        );
        assert!(
            physis_constants::lookup("mu0h/mup").is_none(),
            "mu0h/mup is not a ledger name; the live name is mu0h_mup"
        );
        let mu0h_mup = physis_constants::shielded_helion_to_proton_magnetic_moment_ratio();
        let mu0h_mup_centre = Ratio::new(-7_617_665_618, 10i128.pow(10));
        assert_eq!(
            shielded_helion_to_proton_magnetic_moment_ratio().value(),
            -0.761_766_561_8,
            "mu0h_mup Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            shielded_helion_to_proton_magnetic_moment_ratio().value(),
            mu0h_mup_centre.to_f64(),
            "mu0h_mup Qty locksteps to Ratio::to_f64 on the 10^10 centre"
        );
        assert!(
            mu0h_mup.value.contains(Interval::point(mu0h_mup_centre)),
            "mu0h_mup Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu0h_mup.value.lo, mu0h_mup.value.hi,
            "ledger mu0h_mup stays an Interval; the Qty is not that Interval"
        );
        assert!(
            mu0h_mup.value.hi < Ratio::int(0),
            "ledger mu0h_mup stays the signed shielded helion to proton ratio"
        );
        assert_ne!(
            physis_constants::shielded_helion_to_proton_magnetic_moment_ratio().hash,
            physis_constants::shielded_helion_magnetic_moment().hash,
            "mu0h_mup is not mu0h"
        );
        assert_ne!(
            physis_constants::shielded_helion_to_proton_magnetic_moment_ratio().hash,
            physis_constants::shielded_helion_magnetic_moment_to_nuclear_magneton().hash,
            "mu0h_mup is not mu0h_muN"
        );
        assert_ne!(
            physis_constants::shielded_helion_to_proton_magnetic_moment_ratio().hash,
            physis_constants::neutron_proton_magnetic_moment_ratio().hash,
            "mu0h_mup is not mu_n_mup"
        );
        assert_ne!(
            physis_constants::shielded_helion_to_proton_magnetic_moment_ratio().hash,
            physis_constants::electron_proton_magnetic_moment_ratio().hash,
            "mu0h_mup is not mu_e_mup"
        );
        assert_ne!(
            physis_constants::shielded_helion_to_proton_magnetic_moment_ratio().hash,
            physis_constants::deuteron_proton_magnetic_moment_ratio().hash,
            "mu0h_mup is not mu_d_mup"
        );
        assert_ne!(
            physis_constants::shielded_helion_to_proton_magnetic_moment_ratio().hash,
            physis_constants::helion_g_factor().hash,
            "mu0h_mup is not gh"
        );
        assert!(
            physis_constants::lookup("gamma0h").is_none(),
            "gamma0h cites hbar and is not a ledger name; the live ratio is mu0h_mu0p"
        );
        let mu0h_mu0p =
            physis_constants::shielded_helion_to_shielded_proton_magnetic_moment_ratio();
        let mu0h_mu0p_centre = Ratio::new(-7_617_861_313, 10i128.pow(10));
        assert_eq!(
            shielded_helion_to_shielded_proton_magnetic_moment_ratio().value(),
            -0.761_786_131_3,
            "mu0h_mu0p Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            shielded_helion_to_shielded_proton_magnetic_moment_ratio().value(),
            mu0h_mu0p_centre.to_f64(),
            "mu0h_mu0p Qty locksteps to Ratio::to_f64 on the 10^10 centre"
        );
        assert!(
            mu0h_mu0p.value.contains(Interval::point(mu0h_mu0p_centre)),
            "mu0h_mu0p Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            mu0h_mu0p.value.lo, mu0h_mu0p.value.hi,
            "ledger mu0h_mu0p stays an Interval; the Qty is not that Interval"
        );
        assert!(
            mu0h_mu0p.value.hi < Ratio::int(0),
            "ledger mu0h_mu0p stays the signed shielded helion to shielded proton ratio"
        );
        assert_ne!(
            physis_constants::shielded_helion_to_shielded_proton_magnetic_moment_ratio().hash,
            physis_constants::shielded_helion_to_proton_magnetic_moment_ratio().hash,
            "mu0h_mu0p is not mu0h_mup"
        );
        assert_ne!(
            physis_constants::shielded_helion_to_shielded_proton_magnetic_moment_ratio().hash,
            physis_constants::shielded_helion_magnetic_moment().hash,
            "mu0h_mu0p is not mu0h"
        );
        assert_ne!(
            physis_constants::shielded_helion_to_shielded_proton_magnetic_moment_ratio().hash,
            physis_constants::shielded_proton_magnetic_moment().hash,
            "mu0h_mu0p is not mu0p"
        );
        assert_ne!(
            physis_constants::shielded_helion_to_shielded_proton_magnetic_moment_ratio().hash,
            physis_constants::neutron_to_shielded_proton_magnetic_moment_ratio().hash,
            "mu0h_mu0p is not mu_n_mu0p"
        );
        assert_ne!(
            physis_constants::shielded_helion_to_shielded_proton_magnetic_moment_ratio().hash,
            physis_constants::electron_to_shielded_proton_magnetic_moment_ratio().hash,
            "mu0h_mu0p is not mu_e_mu0p"
        );
        assert_ne!(
            physis_constants::shielded_helion_to_shielded_proton_magnetic_moment_ratio().hash,
            physis_constants::helion_g_factor().hash,
            "mu0h_mu0p is not gh"
        );
        assert!(
            physis_constants::lookup("malpha").is_none(),
            "malpha is not a ledger name; the live name is m_alpha"
        );
        let m_alpha = physis_constants::alpha_particle_mass();
        let m_alpha_centre = Ratio::new(66_446_573_357, 10i128.pow(37));
        assert_eq!(
            alpha_particle_mass().value(),
            6.644_657_335_7e-27,
            "m_alpha Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            alpha_particle_mass().value(),
            m_alpha_centre.to_f64(),
            "m_alpha Qty locksteps to Ratio::to_f64 on the 10^37 centre"
        );
        assert!(
            m_alpha.value.contains(Interval::point(m_alpha_centre)),
            "m_alpha Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_alpha.value.lo, m_alpha.value.hi,
            "ledger m_alpha stays an Interval; the Qty is not that Interval"
        );
        assert!(
            m_alpha.value.lo > Ratio::int(0),
            "ledger m_alpha stays the signed alpha particle mass"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass().hash,
            physis_constants::helion_mass().hash,
            "m_alpha is not m_h"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass().hash,
            physis_constants::triton_mass().hash,
            "m_alpha is not m_t"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass().hash,
            physis_constants::deuteron_mass().hash,
            "m_alpha is not m_d"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass().hash,
            physis_constants::proton_mass().hash,
            "m_alpha is not m_p"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass().hash,
            physis_constants::electron_alpha_mass_ratio().hash,
            "m_alpha is not me_malpha"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass().hash,
            physis_constants::newtonian_g().hash,
            "m_alpha is not G"
        );
        assert!(
            physis_constants::lookup("malpha_u").is_none(),
            "malpha_u is not a ledger name; the live name is m_alpha_u"
        );
        let m_alpha_u = physis_constants::alpha_particle_mass_in_u();
        let m_alpha_u_centre = Ratio::new(4_001_506_179_127, 10i128.pow(12));
        assert_eq!(
            alpha_particle_mass_in_u().value(),
            4.001_506_179_127,
            "m_alpha_u Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            alpha_particle_mass_in_u().value(),
            m_alpha_u_centre.to_f64(),
            "m_alpha_u Qty locksteps to Ratio::to_f64 on the 10^12 centre"
        );
        assert!(
            m_alpha_u.value.contains(Interval::point(m_alpha_u_centre)),
            "m_alpha_u Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_alpha_u.value.lo, m_alpha_u.value.hi,
            "ledger m_alpha_u stays an Interval; the Qty is not that Interval"
        );
        assert!(
            m_alpha_u.value.lo > Ratio::int(0),
            "ledger m_alpha_u stays a positive mass-in-u hull"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_in_u().hash,
            physis_constants::alpha_particle_mass().hash,
            "m_alpha_u is not m_alpha"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_in_u().hash,
            physis_constants::helion_mass_in_u().hash,
            "m_alpha_u is not m_h_u"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_in_u().hash,
            physis_constants::triton_mass_in_u().hash,
            "m_alpha_u is not m_t_u"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_in_u().hash,
            physis_constants::deuteron_mass_in_u().hash,
            "m_alpha_u is not m_d_u"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_in_u().hash,
            physis_constants::neutron_mass_in_u().hash,
            "m_alpha_u is not m_n_u"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_in_u().hash,
            physis_constants::proton_mass_in_u().hash,
            "m_alpha_u is not m_p_u"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_in_u().hash,
            physis_constants::muon_mass_in_u().hash,
            "m_alpha_u is not m_mu_u"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_in_u().hash,
            physis_constants::electron_alpha_mass_ratio().hash,
            "m_alpha_u is not me_malpha"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_in_u().hash,
            physis_constants::electron_molar_mass().hash,
            "m_alpha_u is not M_e"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_in_u().hash,
            physis_constants::newtonian_g().hash,
            "m_alpha_u is not G"
        );
        assert!(
            physis_constants::lookup("malpha_c2").is_none(),
            "malpha_c2 is not a ledger name; the live name is m_alpha_c2"
        );
        let m_alpha_c2 = physis_constants::alpha_particle_mass_energy_equivalent();
        let m_alpha_c2_centre = Ratio::new(59_719_201_914, 10i128.pow(20));
        assert_eq!(
            alpha_particle_mass_energy_equivalent().value(),
            5.971_920_191_4e-10,
            "m_alpha_c2 Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            alpha_particle_mass_energy_equivalent().value(),
            m_alpha_c2_centre.to_f64(),
            "m_alpha_c2 Qty locksteps to Ratio::to_f64 on the 10^20 centre"
        );
        assert!(
            m_alpha_c2
                .value
                .contains(Interval::point(m_alpha_c2_centre)),
            "m_alpha_c2 Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_alpha_c2.value.lo, m_alpha_c2.value.hi,
            "ledger m_alpha_c2 stays an Interval; the Qty is not that Interval"
        );
        assert!(
            m_alpha_c2.value.lo > Ratio::int(0),
            "ledger m_alpha_c2 stays a positive energy hull"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent().hash,
            physis_constants::alpha_particle_mass().hash,
            "m_alpha_c2 is not m_alpha"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent().hash,
            physis_constants::alpha_particle_mass_in_u().hash,
            "m_alpha_c2 is not m_alpha_u"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent().hash,
            physis_constants::helion_mass_energy_equivalent().hash,
            "m_alpha_c2 is not m_h_c2"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent().hash,
            physis_constants::triton_mass_energy_equivalent().hash,
            "m_alpha_c2 is not m_t_c2"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent().hash,
            physis_constants::deuteron_mass_energy_equivalent().hash,
            "m_alpha_c2 is not m_d_c2"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent().hash,
            physis_constants::neutron_mass_energy_equivalent().hash,
            "m_alpha_c2 is not m_n_c2"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent().hash,
            physis_constants::proton_mass_energy_equivalent().hash,
            "m_alpha_c2 is not m_p_c2"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent().hash,
            physis_constants::muon_mass_energy_equivalent().hash,
            "m_alpha_c2 is not m_mu_c2"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent().hash,
            physis_constants::electron_volt().hash,
            "m_alpha_c2 is not eV"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent().hash,
            physis_constants::newtonian_g().hash,
            "m_alpha_c2 is not G"
        );
        assert!(
            physis_constants::lookup("malpha_c2_MeV").is_none(),
            "malpha_c2_MeV is not a ledger name; the live name is m_alpha_c2_MeV"
        );
        let m_alpha_c2_mev = physis_constants::alpha_particle_mass_energy_equivalent_in_mev();
        let m_alpha_c2_mev_centre = Ratio::new(37_273_794_066, 10i128.pow(7));
        assert_eq!(
            alpha_particle_mass_energy_equivalent_in_mev().value(),
            3_727.379_406_6,
            "m_alpha_c2_MeV Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            alpha_particle_mass_energy_equivalent_in_mev().value(),
            m_alpha_c2_mev_centre.to_f64(),
            "m_alpha_c2_MeV Qty locksteps to Ratio::to_f64 on the 10^7 centre"
        );
        assert!(
            m_alpha_c2_mev
                .value
                .contains(Interval::point(m_alpha_c2_mev_centre)),
            "m_alpha_c2_MeV Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_alpha_c2_mev.value.lo, m_alpha_c2_mev.value.hi,
            "ledger m_alpha_c2_MeV stays an Interval; the Qty is not that Interval"
        );
        assert!(
            m_alpha_c2_mev.value.lo > Ratio::int(0),
            "ledger m_alpha_c2_MeV stays a positive energy hull"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent_in_mev().hash,
            physis_constants::alpha_particle_mass_energy_equivalent().hash,
            "m_alpha_c2_MeV is not m_alpha_c2"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent_in_mev().hash,
            physis_constants::helion_mass_energy_equivalent_in_mev().hash,
            "m_alpha_c2_MeV is not m_h_c2_MeV"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent_in_mev().hash,
            physis_constants::triton_mass_energy_equivalent_in_mev().hash,
            "m_alpha_c2_MeV is not m_t_c2_MeV"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent_in_mev().hash,
            physis_constants::deuteron_mass_energy_equivalent_in_mev().hash,
            "m_alpha_c2_MeV is not m_d_c2_MeV"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent_in_mev().hash,
            physis_constants::neutron_mass_energy_equivalent_in_mev().hash,
            "m_alpha_c2_MeV is not m_n_c2_MeV"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent_in_mev().hash,
            physis_constants::proton_mass_energy_equivalent_in_mev().hash,
            "m_alpha_c2_MeV is not m_p_c2_MeV"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent_in_mev().hash,
            physis_constants::muon_mass_energy_equivalent_in_mev().hash,
            "m_alpha_c2_MeV is not m_mu_c2_MeV"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent_in_mev().hash,
            physis_constants::electron_volt().hash,
            "m_alpha_c2_MeV is not eV"
        );
        assert_ne!(
            physis_constants::alpha_particle_mass_energy_equivalent_in_mev().hash,
            physis_constants::newtonian_g().hash,
            "m_alpha_c2_MeV is not G"
        );
        assert!(
            physis_constants::lookup("malpha/me").is_none(),
            "malpha/me is not a ledger name; the live name is malpha_me"
        );
        let malpha_me = physis_constants::alpha_particle_electron_mass_ratio();
        let malpha_me_centre = Ratio::new(729_429_954_142, 10i128.pow(8));
        assert_eq!(
            alpha_particle_electron_mass_ratio().value(),
            7_294.299_541_42,
            "malpha_me Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            alpha_particle_electron_mass_ratio().value(),
            malpha_me_centre.to_f64(),
            "malpha_me Qty locksteps to Ratio::to_f64 on the 10^8 centre"
        );
        assert!(
            malpha_me.value.contains(Interval::point(malpha_me_centre)),
            "malpha_me Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            malpha_me.value.lo, malpha_me.value.hi,
            "ledger malpha_me stays an Interval; the Qty is not that Interval"
        );
        assert!(
            malpha_me.value.lo > Ratio::int(0),
            "ledger malpha_me stays a positive mass-ratio hull"
        );
        assert_ne!(
            physis_constants::alpha_particle_electron_mass_ratio().hash,
            physis_constants::electron_alpha_mass_ratio().hash,
            "malpha_me is not me_malpha"
        );
        assert_ne!(
            physis_constants::alpha_particle_electron_mass_ratio().hash,
            physis_constants::helion_electron_mass_ratio().hash,
            "malpha_me is not mh_me"
        );
        assert_ne!(
            physis_constants::alpha_particle_electron_mass_ratio().hash,
            physis_constants::triton_electron_mass_ratio().hash,
            "malpha_me is not mt_me"
        );
        assert_ne!(
            physis_constants::alpha_particle_electron_mass_ratio().hash,
            physis_constants::deuteron_electron_mass_ratio().hash,
            "malpha_me is not md_me"
        );
        assert_ne!(
            physis_constants::alpha_particle_electron_mass_ratio().hash,
            physis_constants::neutron_electron_mass_ratio().hash,
            "malpha_me is not mn_me"
        );
        assert_ne!(
            physis_constants::alpha_particle_electron_mass_ratio().hash,
            physis_constants::proton_electron_mass_ratio().hash,
            "malpha_me is not mp_me"
        );
        assert_ne!(
            physis_constants::alpha_particle_electron_mass_ratio().hash,
            physis_constants::muon_electron_mass_ratio().hash,
            "malpha_me is not mmu_me"
        );
        assert_ne!(
            physis_constants::alpha_particle_electron_mass_ratio().hash,
            physis_constants::alpha_particle_mass_energy_equivalent_in_mev().hash,
            "malpha_me is not m_alpha_c2_MeV"
        );
        assert_ne!(
            physis_constants::alpha_particle_electron_mass_ratio().hash,
            physis_constants::newtonian_g().hash,
            "malpha_me is not G"
        );
        assert!(
            physis_constants::lookup("malpha/mp").is_none(),
            "malpha/mp is not a ledger name; the live name is malpha_mp"
        );
        let malpha_mp = physis_constants::alpha_particle_proton_mass_ratio();
        let malpha_mp_centre = Ratio::new(397_259_969_009, 10i128.pow(11));
        assert_eq!(
            alpha_particle_proton_mass_ratio().value(),
            3.972_599_690_09,
            "malpha_mp Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            alpha_particle_proton_mass_ratio().value(),
            malpha_mp_centre.to_f64(),
            "malpha_mp Qty locksteps to Ratio::to_f64 on the 10^11 centre"
        );
        assert!(
            malpha_mp.value.contains(Interval::point(malpha_mp_centre)),
            "malpha_mp Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            malpha_mp.value.lo, malpha_mp.value.hi,
            "ledger malpha_mp stays an Interval; the Qty is not that Interval"
        );
        assert!(
            malpha_mp.value.lo > Ratio::int(0),
            "ledger malpha_mp stays a positive mass-ratio hull"
        );
        assert_ne!(
            physis_constants::alpha_particle_proton_mass_ratio().hash,
            physis_constants::helion_proton_mass_ratio().hash,
            "malpha_mp is not mh_mp"
        );
        assert_ne!(
            physis_constants::alpha_particle_proton_mass_ratio().hash,
            physis_constants::triton_proton_mass_ratio().hash,
            "malpha_mp is not mt_mp"
        );
        assert_ne!(
            physis_constants::alpha_particle_proton_mass_ratio().hash,
            physis_constants::deuteron_proton_mass_ratio().hash,
            "malpha_mp is not md_mp"
        );
        assert_ne!(
            physis_constants::alpha_particle_proton_mass_ratio().hash,
            physis_constants::neutron_proton_mass_ratio().hash,
            "malpha_mp is not mn_mp"
        );
        assert_ne!(
            physis_constants::alpha_particle_proton_mass_ratio().hash,
            physis_constants::proton_neutron_mass_ratio().hash,
            "malpha_mp is not mp_mn"
        );
        assert_ne!(
            physis_constants::alpha_particle_proton_mass_ratio().hash,
            physis_constants::muon_proton_mass_ratio().hash,
            "malpha_mp is not mmu_mp"
        );
        assert_ne!(
            physis_constants::alpha_particle_proton_mass_ratio().hash,
            physis_constants::alpha_particle_electron_mass_ratio().hash,
            "malpha_mp is not malpha_me"
        );
        assert_ne!(
            physis_constants::alpha_particle_proton_mass_ratio().hash,
            physis_constants::proton_mass().hash,
            "malpha_mp is not m_p"
        );
        assert_ne!(
            physis_constants::alpha_particle_proton_mass_ratio().hash,
            physis_constants::newtonian_g().hash,
            "malpha_mp is not G"
        );

        assert!(
            physis_constants::lookup("Malpha").is_none(),
            "Malpha is not a ledger name; the live name is M_alpha"
        );
        let m_alpha_molar = physis_constants::alpha_particle_molar_mass();
        let m_alpha_molar_centre = Ratio::new(40_015_061_777, 10i128.pow(13));
        assert_eq!(
            alpha_particle_molar_mass().value(),
            4.001_506_177_7e-3,
            "M_alpha Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            alpha_particle_molar_mass().value(),
            m_alpha_molar_centre.to_f64(),
            "M_alpha Qty locksteps to Ratio::to_f64 on the 10^13 centre"
        );
        assert!(
            m_alpha_molar
                .value
                .contains(Interval::point(m_alpha_molar_centre)),
            "M_alpha Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_alpha_molar.value.lo, m_alpha_molar.value.hi,
            "ledger M_alpha stays an Interval; the Qty is not that Interval"
        );
        assert!(
            m_alpha_molar.value.lo > Ratio::int(0),
            "ledger M_alpha stays a positive molar-mass hull"
        );
        assert_ne!(
            physis_constants::alpha_particle_molar_mass().hash,
            physis_constants::helion_molar_mass().hash,
            "M_alpha is not M_h"
        );
        assert_ne!(
            physis_constants::alpha_particle_molar_mass().hash,
            physis_constants::triton_molar_mass().hash,
            "M_alpha is not M_t"
        );
        assert_ne!(
            physis_constants::alpha_particle_molar_mass().hash,
            physis_constants::deuteron_molar_mass().hash,
            "M_alpha is not M_d"
        );
        assert_ne!(
            physis_constants::alpha_particle_molar_mass().hash,
            physis_constants::neutron_molar_mass().hash,
            "M_alpha is not M_n"
        );
        assert_ne!(
            physis_constants::alpha_particle_molar_mass().hash,
            physis_constants::proton_molar_mass().hash,
            "M_alpha is not M_p"
        );
        assert_ne!(
            physis_constants::alpha_particle_molar_mass().hash,
            physis_constants::electron_molar_mass().hash,
            "M_alpha is not M_e"
        );
        assert_ne!(
            physis_constants::alpha_particle_molar_mass().hash,
            physis_constants::muon_molar_mass().hash,
            "M_alpha is not M_mu"
        );
        assert_ne!(
            physis_constants::alpha_particle_molar_mass().hash,
            physis_constants::alpha_particle_mass().hash,
            "M_alpha is not m_alpha"
        );
        assert_ne!(
            physis_constants::alpha_particle_molar_mass().hash,
            physis_constants::alpha_particle_mass_in_u().hash,
            "M_alpha is not m_alpha_u"
        );
        assert_ne!(
            physis_constants::alpha_particle_molar_mass().hash,
            physis_constants::alpha_particle_proton_mass_ratio().hash,
            "M_alpha is not malpha_mp"
        );
        assert_ne!(
            physis_constants::alpha_particle_molar_mass().hash,
            physis_constants::newtonian_g().hash,
            "M_alpha is not G"
        );

        assert!(
            physis_constants::lookup("mu").is_none(),
            "mu is not a ledger name; the live name is m_u"
        );
        assert!(
            physis_constants::lookup("u").is_none(),
            "unified atomic mass unit is the same digits and is not stored under a second name"
        );
        let m_u = physis_constants::atomic_mass_constant();
        let m_u_centre = Ratio::new(166_053_906_660, 10i128.pow(38));
        assert_eq!(
            atomic_mass_constant().value(),
            1.660_539_066_60e-27,
            "m_u Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            atomic_mass_constant().value(),
            m_u_centre.to_f64(),
            "m_u Qty locksteps to Ratio::to_f64 on the 10^38 centre"
        );
        assert!(
            m_u.value.contains(Interval::point(m_u_centre)),
            "m_u Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_u.value.lo, m_u.value.hi,
            "ledger m_u stays an Interval; the Qty is not that Interval"
        );
        assert!(
            m_u.value.lo > Ratio::int(0),
            "ledger m_u stays a positive mass hull"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant().hash,
            physis_constants::proton_mass().hash,
            "m_u is not m_p"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant().hash,
            physis_constants::neutron_mass().hash,
            "m_u is not m_n"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant().hash,
            physis_constants::deuteron_mass().hash,
            "m_u is not m_d"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant().hash,
            physis_constants::triton_mass().hash,
            "m_u is not m_t"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant().hash,
            physis_constants::helion_mass().hash,
            "m_u is not m_h"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant().hash,
            physis_constants::alpha_particle_mass().hash,
            "m_u is not m_alpha"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant().hash,
            physis_constants::muon_mass().hash,
            "m_u is not m_mu"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant().hash,
            physis_constants::alpha_particle_molar_mass().hash,
            "m_u is not M_alpha"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant().hash,
            physis_constants::avogadro().hash,
            "m_u is not N_A"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant().hash,
            physis_constants::vacuum_permeability().hash,
            "m_u is not mu0"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant().hash,
            physis_constants::newtonian_g().hash,
            "m_u is not G"
        );

        assert!(
            physis_constants::lookup("muc2").is_none(),
            "muc2 is not a ledger name; the live name is m_u_c2"
        );
        let m_u_c2 = physis_constants::atomic_mass_constant_energy_equivalent();
        let m_u_c2_centre = Ratio::new(149_241_808_560, 10i128.pow(21));
        assert_eq!(
            atomic_mass_constant_energy_equivalent().value(),
            1.492_418_085_60e-10,
            "m_u_c2 Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            atomic_mass_constant_energy_equivalent().value(),
            m_u_c2_centre.to_f64(),
            "m_u_c2 Qty locksteps to Ratio::to_f64 on the 10^21 centre"
        );
        assert!(
            m_u_c2.value.contains(Interval::point(m_u_c2_centre)),
            "m_u_c2 Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_u_c2.value.lo, m_u_c2.value.hi,
            "ledger m_u_c2 stays an Interval; the Qty is not that Interval"
        );
        assert!(
            m_u_c2.value.lo > Ratio::int(0),
            "ledger m_u_c2 stays a positive energy hull"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent().hash,
            physis_constants::atomic_mass_constant().hash,
            "m_u_c2 is not m_u"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent().hash,
            physis_constants::proton_mass_energy_equivalent().hash,
            "m_u_c2 is not m_p_c2"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent().hash,
            physis_constants::neutron_mass_energy_equivalent().hash,
            "m_u_c2 is not m_n_c2"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent().hash,
            physis_constants::deuteron_mass_energy_equivalent().hash,
            "m_u_c2 is not m_d_c2"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent().hash,
            physis_constants::triton_mass_energy_equivalent().hash,
            "m_u_c2 is not m_t_c2"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent().hash,
            physis_constants::helion_mass_energy_equivalent().hash,
            "m_u_c2 is not m_h_c2"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent().hash,
            physis_constants::alpha_particle_mass_energy_equivalent().hash,
            "m_u_c2 is not m_alpha_c2"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent().hash,
            physis_constants::muon_mass_energy_equivalent().hash,
            "m_u_c2 is not m_mu_c2"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent().hash,
            physis_constants::hartree_energy().hash,
            "m_u_c2 is not Eh"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent().hash,
            physis_constants::electron_volt().hash,
            "m_u_c2 is not eV"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent().hash,
            physis_constants::newtonian_g().hash,
            "m_u_c2 is not G"
        );

        assert!(
            physis_constants::lookup("muc2_MeV").is_none(),
            "muc2_MeV is not a ledger name; the live name is m_u_c2_MeV"
        );
        let m_u_c2_mev = physis_constants::atomic_mass_constant_energy_equivalent_in_mev();
        let m_u_c2_mev_centre = Ratio::new(93_149_410_242, 10i128.pow(8));
        assert_eq!(
            atomic_mass_constant_energy_equivalent_in_mev().value(),
            931.494_102_42,
            "m_u_c2_MeV Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            atomic_mass_constant_energy_equivalent_in_mev().value(),
            m_u_c2_mev_centre.to_f64(),
            "m_u_c2_MeV Qty locksteps to Ratio::to_f64 on the 10^8 centre"
        );
        assert!(
            m_u_c2_mev
                .value
                .contains(Interval::point(m_u_c2_mev_centre)),
            "m_u_c2_MeV Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_u_c2_mev.value.lo, m_u_c2_mev.value.hi,
            "ledger m_u_c2_MeV stays an Interval; the Qty is not that Interval"
        );
        assert!(
            m_u_c2_mev.value.lo > Ratio::int(0),
            "ledger m_u_c2_MeV stays a positive energy-equivalent hull"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent_in_mev().hash,
            physis_constants::atomic_mass_constant_energy_equivalent().hash,
            "m_u_c2_MeV is not m_u_c2"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent_in_mev().hash,
            physis_constants::proton_mass_energy_equivalent_in_mev().hash,
            "m_u_c2_MeV is not m_p_c2_MeV"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent_in_mev().hash,
            physis_constants::neutron_mass_energy_equivalent_in_mev().hash,
            "m_u_c2_MeV is not m_n_c2_MeV"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent_in_mev().hash,
            physis_constants::deuteron_mass_energy_equivalent_in_mev().hash,
            "m_u_c2_MeV is not m_d_c2_MeV"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent_in_mev().hash,
            physis_constants::triton_mass_energy_equivalent_in_mev().hash,
            "m_u_c2_MeV is not m_t_c2_MeV"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent_in_mev().hash,
            physis_constants::helion_mass_energy_equivalent_in_mev().hash,
            "m_u_c2_MeV is not m_h_c2_MeV"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent_in_mev().hash,
            physis_constants::alpha_particle_mass_energy_equivalent_in_mev().hash,
            "m_u_c2_MeV is not m_alpha_c2_MeV"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent_in_mev().hash,
            physis_constants::muon_mass_energy_equivalent_in_mev().hash,
            "m_u_c2_MeV is not m_mu_c2_MeV"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent_in_mev().hash,
            physis_constants::hartree_energy().hash,
            "m_u_c2_MeV is not Eh"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent_in_mev().hash,
            physis_constants::electron_volt().hash,
            "m_u_c2_MeV is not eV"
        );
        assert_ne!(
            physis_constants::atomic_mass_constant_energy_equivalent_in_mev().hash,
            physis_constants::newtonian_g().hash,
            "m_u_c2_MeV is not G"
        );

        assert!(
            physis_constants::lookup("Mu").is_none(),
            "Mu is not a ledger name; the live name is M_u"
        );
        let m_u_molar = physis_constants::molar_mass_constant();
        let m_u_molar_centre = Ratio::new(99_999_999_965, 10i128.pow(14));
        assert_eq!(
            molar_mass_constant().value(),
            0.999_999_999_65e-3,
            "M_u Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            molar_mass_constant().value(),
            m_u_molar_centre.to_f64(),
            "M_u Qty locksteps to Ratio::to_f64 on the 10^14 centre"
        );
        assert!(
            m_u_molar.value.contains(Interval::point(m_u_molar_centre)),
            "M_u Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_u_molar.value.lo, m_u_molar.value.hi,
            "ledger M_u stays an Interval; the Qty is not that Interval"
        );
        assert!(
            m_u_molar.value.lo > Ratio::int(0),
            "ledger M_u stays a positive molar-mass hull"
        );
        assert_ne!(
            physis_constants::molar_mass_constant().hash,
            physis_constants::alpha_particle_molar_mass().hash,
            "M_u is not M_alpha"
        );
        assert_ne!(
            physis_constants::molar_mass_constant().hash,
            physis_constants::helion_molar_mass().hash,
            "M_u is not M_h"
        );
        assert_ne!(
            physis_constants::molar_mass_constant().hash,
            physis_constants::triton_molar_mass().hash,
            "M_u is not M_t"
        );
        assert_ne!(
            physis_constants::molar_mass_constant().hash,
            physis_constants::deuteron_molar_mass().hash,
            "M_u is not M_d"
        );
        assert_ne!(
            physis_constants::molar_mass_constant().hash,
            physis_constants::neutron_molar_mass().hash,
            "M_u is not M_n"
        );
        assert_ne!(
            physis_constants::molar_mass_constant().hash,
            physis_constants::proton_molar_mass().hash,
            "M_u is not M_p"
        );
        assert_ne!(
            physis_constants::molar_mass_constant().hash,
            physis_constants::electron_molar_mass().hash,
            "M_u is not M_e"
        );
        assert_ne!(
            physis_constants::molar_mass_constant().hash,
            physis_constants::muon_molar_mass().hash,
            "M_u is not M_mu"
        );
        assert_ne!(
            physis_constants::molar_mass_constant().hash,
            physis_constants::atomic_mass_constant().hash,
            "M_u is not m_u"
        );
        assert_ne!(
            physis_constants::molar_mass_constant().hash,
            physis_constants::avogadro().hash,
            "M_u is not N_A"
        );
        assert_ne!(
            physis_constants::molar_mass_constant().hash,
            physis_constants::newtonian_g().hash,
            "M_u is not G"
        );

        assert!(
            physis_constants::lookup("M12C").is_none(),
            "M12C is not a ledger name; the live name is M_12C"
        );
        let m_12c = physis_constants::carbon_12_molar_mass();
        let m_12c_centre = Ratio::new(119_999_999_958, 10i128.pow(13));
        assert_eq!(
            carbon_12_molar_mass().value(),
            11.999_999_995_8e-3,
            "M_12C Qty is the CODATA 2018 centre, not an SI-exact Ratio"
        );
        assert_eq!(
            carbon_12_molar_mass().value(),
            m_12c_centre.to_f64(),
            "M_12C Qty locksteps to Ratio::to_f64 on the 10^13 centre"
        );
        assert!(
            m_12c.value.contains(Interval::point(m_12c_centre)),
            "M_12C Qty centre must lie in the versioned one-sigma hull"
        );
        assert_ne!(
            m_12c.value.lo, m_12c.value.hi,
            "ledger M_12C stays an Interval; the Qty is not that Interval"
        );
        assert!(
            m_12c.value.lo > Ratio::int(0),
            "ledger M_12C stays a positive molar-mass hull"
        );
        assert_ne!(
            physis_constants::carbon_12_molar_mass().hash,
            physis_constants::molar_mass_constant().hash,
            "M_12C is not M_u"
        );
        assert_ne!(
            physis_constants::carbon_12_molar_mass().hash,
            physis_constants::alpha_particle_molar_mass().hash,
            "M_12C is not M_alpha"
        );
        assert_ne!(
            physis_constants::carbon_12_molar_mass().hash,
            physis_constants::helion_molar_mass().hash,
            "M_12C is not M_h"
        );
        assert_ne!(
            physis_constants::carbon_12_molar_mass().hash,
            physis_constants::triton_molar_mass().hash,
            "M_12C is not M_t"
        );
        assert_ne!(
            physis_constants::carbon_12_molar_mass().hash,
            physis_constants::deuteron_molar_mass().hash,
            "M_12C is not M_d"
        );
        assert_ne!(
            physis_constants::carbon_12_molar_mass().hash,
            physis_constants::neutron_molar_mass().hash,
            "M_12C is not M_n"
        );
        assert_ne!(
            physis_constants::carbon_12_molar_mass().hash,
            physis_constants::proton_molar_mass().hash,
            "M_12C is not M_p"
        );
        assert_ne!(
            physis_constants::carbon_12_molar_mass().hash,
            physis_constants::electron_molar_mass().hash,
            "M_12C is not M_e"
        );
        assert_ne!(
            physis_constants::carbon_12_molar_mass().hash,
            physis_constants::muon_molar_mass().hash,
            "M_12C is not M_mu"
        );
        assert_ne!(
            physis_constants::carbon_12_molar_mass().hash,
            physis_constants::atomic_mass_constant().hash,
            "M_12C is not m_u"
        );
        assert_ne!(
            physis_constants::carbon_12_molar_mass().hash,
            physis_constants::avogadro().hash,
            "M_12C is not N_A"
        );
        assert_ne!(
            physis_constants::carbon_12_molar_mass().hash,
            physis_constants::newtonian_g().hash,
            "M_12C is not G"
        );

        assert!(
            physis_constants::lookup("NA_h").is_none(),
            "NA_h is not a ledger name; the live name is NAh"
        );
        let n_a_h = physis_constants::molar_planck_constant();
        let n_a_h_value = Ratio::new(602_214_076i128 * 662_607_015i128, 10i128.pow(27));
        assert_eq!(
            n_a_h.value, n_a_h_value,
            "ledger NAh is the exact SI product"
        );
        assert_eq!(
            SciExact::new(39_903_127_128_934_314, -26).to_ratio(),
            Some(n_a_h.value),
            "NAh fits Ratio; SciExact and Ratio are the same decimal"
        );
        assert_eq!(
            molar_planck_constant().value(),
            SciExact::new(39_903_127_128_934_314, -26).to_f64(),
            "NAh Qty is the IEEE rounding of the SI decimal, not Ratio::to_f64 of the reduced fraction"
        );
        assert_eq!(
            molar_planck_constant().value(),
            3.990_312_712_893_431_4e-10,
            "NAh Qty locksteps to the SI 2019 terminating decimal literal"
        );
        assert!(
            n_a_h.value > Ratio::int(0),
            "ledger NAh stays a positive exact Ratio"
        );
        assert_ne!(
            physis_constants::molar_planck_constant().hash,
            physis_constants::avogadro().hash,
            "NAh is not N_A"
        );
        assert_ne!(
            physis_constants::molar_planck_constant().hash,
            physis_constants::planck_h().hash,
            "NAh is not h"
        );
        assert_ne!(
            physis_constants::molar_planck_constant().hash,
            physis_constants::carbon_12_molar_mass().hash,
            "NAh is not M_12C"
        );
        assert_ne!(
            physis_constants::molar_planck_constant().hash,
            physis_constants::molar_mass_constant().hash,
            "NAh is not M_u"
        );
        assert_ne!(
            physis_constants::molar_planck_constant().hash,
            physis_constants::electron_volt().hash,
            "NAh is not eV"
        );
        assert_ne!(
            physis_constants::molar_planck_constant().hash,
            physis_constants::newtonian_g().hash,
            "NAh is not G"
        );

        assert!(
            physis_constants::lookup("R").is_none(),
            "R is not a ledger name; the live name is NAk"
        );
        let n_a_k = physis_constants::molar_gas_constant();
        let n_a_k_value = Ratio::new(602_214_076i128 * 1_380_649i128, 10i128.pow(14));
        assert_eq!(
            n_a_k.value, n_a_k_value,
            "ledger NAk is the exact SI product"
        );
        assert_eq!(
            SciExact::new(831_446_261_815_324, -14).to_ratio(),
            Some(n_a_k.value),
            "NAk fits Ratio; SciExact and Ratio are the same decimal"
        );
        assert_eq!(
            molar_gas_constant().value(),
            SciExact::new(831_446_261_815_324, -14).to_f64(),
            "NAk Qty is the IEEE rounding of the SI decimal"
        );
        assert_eq!(
            molar_gas_constant().value(),
            8.314_462_618_153_24,
            "NAk Qty locksteps to the SI 2019 terminating decimal literal"
        );
        assert_eq!(
            molar_gas_constant().value(),
            n_a_k_value.to_f64(),
            "NAk reduced Ratio::to_f64 matches SciExact::to_f64 at this scale"
        );
        assert!(
            n_a_k.value > Ratio::int(0),
            "ledger NAk stays a positive exact Ratio"
        );
        assert_ne!(
            physis_constants::molar_gas_constant().hash,
            physis_constants::avogadro().hash,
            "NAk is not N_A"
        );
        assert_ne!(
            physis_constants::molar_gas_constant().hash,
            physis_constants::boltzmann().hash,
            "NAk is not k"
        );
        assert_ne!(
            physis_constants::molar_gas_constant().hash,
            physis_constants::molar_planck_constant().hash,
            "NAk is not NAh"
        );
        assert_ne!(
            physis_constants::molar_gas_constant().hash,
            physis_constants::planck_h().hash,
            "NAk is not h"
        );
        assert_ne!(
            physis_constants::molar_gas_constant().hash,
            physis_constants::electron_volt().hash,
            "NAk is not eV"
        );
        assert_ne!(
            physis_constants::molar_gas_constant().hash,
            physis_constants::newtonian_g().hash,
            "NAk is not G"
        );
        assert!(
            physis_constants::lookup("NAe").is_none(),
            "NAe Faraday constant is a later PHYSICOCHEMICAL row"
        );
        assert!(
            physis_constants::lookup("g0p").is_none(),
            "g0p is a glossary identity, not a table XXXI recommended hull"
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
