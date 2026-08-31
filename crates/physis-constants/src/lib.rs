//! Versioned physical constants. Never scatter magic floats in theories.
//!
//! SI 2019 defining constants that fit in [`physis_numeric::Ratio`] are
//! `c`, `Δν_Cs`, `e`, `k`, `N_A`, and `K_cd`. Planck's `h` is SI-exact
//! [`physis_numeric::SciExact`] `662607015e-42` J s: the reduced
//! denominator does not fit in `i128`, so it is not a Ratio. `ħ` is not
//! a terminating decimal. CODATA 2018 Newtonian `G` is a one-sigma
//! [`Interval`], not an exact Ratio. CODATA 2018 vacuum permeability
//! `μ₀` is a one-sigma [`Interval`] `1.25663706212(19)×10^{-6}` N A⁻²
//! (JPCRD table XXXI, UNIVERSAL): measured after SI 2019, not an
//! exact Ratio. CODATA 2018 vacuum permittivity `ε₀` is a one-sigma
//! [`Interval`] `8.8541878128(13)×10^{-12}` F m⁻¹ (JPCRD table XXXI,
//! UNIVERSAL): `1/(μ₀ c²)` after SI 2019, not an exact Ratio. CODATA
//! 2018 characteristic impedance `Z₀` is a one-sigma [`Interval`]
//! `376.730313668(57)` ohm (JPCRD table XXXI, UNIVERSAL): `μ₀ c` after
//! SI 2019, not an exact Ratio. Vacuum admittance `Y₀` is not stored.
//! CODATA 2018 fine-structure `α` is
//! a one-sigma [`Interval`] `7.2973525693(11)×10^{-3}` (JPCRD table
//! XXXI, ATOMIC AND NUCLEAR): a measured hull, not an SI defining
//! Ratio. CODATA 2018 inverse fine-structure `α⁻¹` is a one-sigma
//! [`Interval`] `137.035999084(21)` from the same table: a different
//! recommended hull, not `1/α` as a derived Ratio. CODATA 2018 Rydberg
//! frequency `cR∞` is a one-sigma [`Interval`]
//! `3.2898419602508(64)×10^{15}` Hz from the same table: a measured hull,
//! not an SI defining Ratio. CODATA 2018 Rydberg energy equivalent `hcR∞`
//! is a one-sigma [`Interval`] `2.1798723611035(42)×10^{-18}` J from the
//! same table: a measured hull, not an SI defining Ratio, and not the
//! eV conversion. CODATA 2018 Rydberg constant `R∞` is a one-sigma
//! [`Interval`] `10973731.568160(21)` m⁻¹
//! from the same table: a measured hull, not an SI defining Ratio.
//! CODATA 2018 Bohr radius `a₀` is a one-sigma [`Interval`]
//! `5.29177210903(80)×10^{-11}` m from the same table: a measured hull,
//! not an SI defining Ratio. CODATA 2018 Hartree energy `E_h` is a
//! one-sigma [`Interval`] `4.3597447222071(85)×10^{-18}` J from the
//! same table: a measured hull, not an SI defining Ratio, and not the
//! eV conversion. CODATA 2018 electron-muon mass ratio `m_e/m_μ` is a
//! one-sigma [`Interval`] `4.83633169(11)×10^{-3}` (JPCRD table XXXI,
//! Electron, e-): a measured hull, not an SI defining Ratio. CODATA 2018
//! electron-proton mass ratio `m_e/m_p` is a one-sigma [`Interval`]
//! `5.44617021487(33)×10^{-4}` from the same section: a measured hull,
//! not an SI defining Ratio. CODATA 2018 electron-neutron mass ratio
//! `m_e/m_n` is a one-sigma [`Interval`] `5.4386734424(26)×10^{-4}` from
//! the same section: a measured hull, not an SI defining Ratio. CODATA
//! 2018 electron-deuteron mass ratio `m_e/m_d` is a one-sigma
//! [`Interval`] `2.724437107462(96)×10^{-4}` from the same section: a
//! measured hull, not an SI defining Ratio. CODATA 2018 electron-triton
//! mass ratio `m_e/m_t` is a one-sigma [`Interval`]
//! `1.819200062251(90)×10^{-4}` from the same section: a measured hull,
//! not an SI defining Ratio. CODATA 2018 electron-helion mass ratio
//! `m_e/m_h` is a one-sigma [`Interval`] `1.819543074573(79)×10^{-4}`
//! from the same section: a measured hull, not an SI defining Ratio.
//! CODATA 2018 electron to alpha particle mass ratio `m_e/m_α` is a
//! one-sigma [`Interval`] `1.370933554787(45)×10^{-4}` from the same
//! section: a measured hull, not an SI defining Ratio. CODATA 2018
//! electron charge to mass quotient `−e/m_e` is a one-sigma
//! [`Interval`] `−1.75882001076(53)×10^{11}` C kg⁻¹ from the same
//! section: a measured hull, not an SI defining Ratio, and not
//! electron mass. CODATA 2018 electron molar mass `M_e` is a one-sigma
//! [`Interval`] `5.4857990888(17)×10^{-7}` kg mol⁻¹ from the same
//! section: a measured hull, not an SI defining Ratio, not electron
//! mass in kg, and not the mass-in-u row. CODATA 2018 reduced Compton
//! wavelength `ƛ_C` is a one-sigma [`Interval`]
//! `3.8615926796(12)×10^{-13}` m from the same section: a measured hull,
//! not an SI defining Ratio, and not a certificate of `α a₀`. CODATA 2018
//! Compton wavelength `λ_C` is a one-sigma [`Interval`]
//! `2.42631023867(73)×10^{-12}` m from the same section: a measured hull,
//! not an SI defining Ratio, and not a certificate of `2π ƛ_C`. CODATA 2018
//! classical electron radius `r_e` is a one-sigma [`Interval`]
//! `2.8179403262(13)×10^{-15}` m from the same section: a measured hull,
//! not an SI defining Ratio, and not a certificate of `α² a₀`. The Thomson
//! cross section and the quantum of circulation are not stored: `π` means
//! they are not a Ratio.
//! CODATA 2018 proton mass `m_p` is a
//! one-sigma [`Interval`] `1.67262192369(51)×10^{-27}` kg (JPCRD table
//! XXXI, Proton, p): a measured hull, not an SI defining Ratio.
//! Electron mass is not stored: `10^{42}` overflows `i128`. The IAU 2012 astronomical unit is
//! an exact [`Ratio`] `149597870700` m (BIPM table 8). The parsec is
//! `(648000/π) au` and is not a Ratio. IAU 2015 `(GM)_☉^N` is an exact
//! [`Ratio`] `1.3271244×10^20` m³ s⁻² (AJ 152, 41 table 1): a
//! conversion ruler, not a measured solar mass. IAU 2015 `R_☉^N` is an
//! exact [`Ratio`] `695700000` m from the same table: also a conversion
//! ruler, not a measured photospheric radius. IAU 2015 `L_☉^N` is an
//! exact [`Ratio`] `3.828×10^26` W from the same table: a conversion
//! ruler, not a measured solar luminosity. The electronvolt is an exact
//! [`Ratio`] `1.602176634×10^{-19}` J (BIPM table 8), the same SI 2019
//! decimal as `e` with unit joule, not coulomb. Theories still use
//! `physis_model` `f64` Qty constants. This crate does not mint a kernel
//! proof. Overlapping `physis_model` Qty floats are lockstepped in
//! `physis-model` tests; theories still evaluate with those Qty.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use physis_core::artifact::ArtifactId;
use physis_numeric::{Interval, Ratio, SciExact};
use physis_provenance::{Citation, SourceLocator, SourceRecord};
use serde::{Deserialize, Serialize};

/// A constants release (CODATA 2018/2022, PDG 2024, SI 2019 exact, …).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConstantRelease {
    /// SI 2019 exact values plus CODATA 2018 estimates where still measured.
    Si2019Codata2018,
    /// IAU 2015 nominal solar and planetary conversion constants (B3).
    Iau2015,
}

impl ConstantRelease {
    /// Stable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            ConstantRelease::Si2019Codata2018 => "si-2019-codata-2018",
            ConstantRelease::Iau2015 => "iau-2015",
        }
    }
}

/// A named constant with provenance. `T` is typically [`Ratio`] for exact
/// SI quantities and a ratio enclosure for measured ones.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Constant<T> {
    /// Name (`c`, `hbar`, …).
    pub name: String,
    /// Value.
    pub value: T,
    /// Unit string (`m/s`, …) until the IR owns units.
    pub unit: String,
    /// Provenance.
    pub provenance: SourceRecord,
    /// Release.
    pub release: ConstantRelease,
    /// Content hash.
    pub hash: ArtifactId,
}

impl<T: std::fmt::Debug> Constant<T> {
    /// Build and hash.
    pub fn new(
        name: impl Into<String>,
        value: T,
        unit: impl Into<String>,
        provenance: SourceRecord,
        release: ConstantRelease,
    ) -> Self {
        let name = name.into();
        let unit = unit.into();
        let mut buf = String::new();
        buf.push_str(&name);
        buf.push('\n');
        buf.push_str(&format!("{value:?}\n{unit}\n{}", release.as_str()));
        buf.push_str(&provenance.source_hash.to_hex());
        Self {
            hash: ArtifactId::of(buf.as_bytes()),
            name,
            value,
            unit,
            provenance,
            release,
        }
    }
}

fn si_brochure() -> SourceRecord {
    SourceRecord::new(
        Citation {
            work: "BIPM SI Brochure".into(),
            edition: "9th".into(),
        },
        "2019",
        SourceLocator {
            page: Some(127),
            section: Some("Defining constants".into()),
            equation: None,
            figure: None,
            table: Some("1".into()),
            dataset_range: None,
            experiment: None,
        },
        ArtifactId::of(b"si-brochure-9"),
        None,
    )
    .expect("si brochure locator is precise")
}

fn si2019_exact(name: impl Into<String>, value: Ratio, unit: impl Into<String>) -> Constant<Ratio> {
    Constant::new(
        name,
        value,
        unit,
        si_brochure(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// Speed of light, exact, SI 2019.
pub fn speed_of_light() -> Constant<Ratio> {
    si2019_exact("c", Ratio::int(299_792_458), "m/s")
}

/// Caesium hyperfine frequency Δν_Cs, exact, SI 2019.
pub fn caesium_hyperfine() -> Constant<Ratio> {
    si2019_exact("delta-nu-Cs", Ratio::int(9_192_631_770), "Hz")
}

/// Elementary charge, exact, SI 2019.
pub fn elementary_charge() -> Constant<Ratio> {
    si2019_exact("e", Ratio::new(1_602_176_634, 10i128.pow(28)), "C")
}

/// Boltzmann constant, exact, SI 2019.
pub fn boltzmann() -> Constant<Ratio> {
    si2019_exact("k", Ratio::new(1_380_649, 10i128.pow(29)), "J/K")
}

/// Avogadro constant, exact, SI 2019.
pub fn avogadro() -> Constant<Ratio> {
    si2019_exact(
        "N_A",
        Ratio::int(602_214_076i128 * 1_000_000_000_000_000i128),
        "1/mol",
    )
}

/// Luminous efficacy of 540 THz radiation K_cd, exact, SI 2019.
pub fn luminous_efficacy() -> Constant<Ratio> {
    si2019_exact("K_cd", Ratio::int(683), "lm/W")
}

/// Planck constant h, exact, SI 2019, as a terminating decimal.
///
/// `h = 6.62607015×10⁻³⁴ J s = 662607015 × 10⁻⁴²`. That is not a
/// [`Ratio`]: `10^42` overflows `i128`. `ħ = h/2π` is not stored here.
pub fn planck_h() -> Constant<SciExact> {
    Constant::new(
        "h",
        SciExact::new(662_607_015, -42),
        "J s",
        si_brochure(),
        ConstantRelease::Si2019Codata2018,
    )
}

fn codata_2018_jpcrd(section: &str, range: &str) -> SourceRecord {
    SourceRecord::new(
        Citation {
            work: "CODATA recommended values of the fundamental physical constants: 2018".into(),
            edition: "J. Phys. Chem. Ref. Data 50, 033105".into(),
        },
        "2018",
        SourceLocator {
            page: None,
            section: Some(section.into()),
            equation: None,
            figure: None,
            table: Some("XXXI".into()),
            dataset_range: Some(range.into()),
            experiment: None,
        },
        ArtifactId::of(b"codata-2018-jpcrd-50-033105"),
        None,
    )
    .expect("CODATA 2018 JPCRD locator names a table and range")
}

fn codata_2018_g_source() -> SourceRecord {
    codata_2018_jpcrd("UNIVERSAL", "G = 6.67430(15)e-11")
}

fn codata_2018_mu0_source() -> SourceRecord {
    codata_2018_jpcrd("UNIVERSAL", "mu0 = 1.25663706212(19)e-6")
}

fn codata_2018_epsilon0_source() -> SourceRecord {
    codata_2018_jpcrd("UNIVERSAL", "epsilon0 = 8.8541878128(13)e-12")
}

fn codata_2018_z0_source() -> SourceRecord {
    codata_2018_jpcrd("UNIVERSAL", "Z0 = 376.730313668(57)")
}

fn codata_2018_alpha_source() -> SourceRecord {
    codata_2018_jpcrd("ATOMIC AND NUCLEAR", "alpha = 7.2973525693(11)e-3")
}

fn codata_2018_inv_alpha_source() -> SourceRecord {
    codata_2018_jpcrd("ATOMIC AND NUCLEAR", "inv_alpha = 137.035999084(21)")
}

fn codata_2018_rydberg_frequency_source() -> SourceRecord {
    codata_2018_jpcrd("ATOMIC AND NUCLEAR", "cRinf = 3.2898419602508(64)e15")
}

fn codata_2018_rydberg_energy_source() -> SourceRecord {
    codata_2018_jpcrd("ATOMIC AND NUCLEAR", "hcRinf = 2.1798723611035(42)e-18")
}

fn codata_2018_rydberg_source() -> SourceRecord {
    codata_2018_jpcrd("ATOMIC AND NUCLEAR", "Rinf = 10973731.568160(21)")
}

fn codata_2018_bohr_source() -> SourceRecord {
    codata_2018_jpcrd("ATOMIC AND NUCLEAR", "a0 = 5.29177210903(80)e-11")
}

fn codata_2018_hartree_source() -> SourceRecord {
    codata_2018_jpcrd("ATOMIC AND NUCLEAR", "Eh = 4.3597447222071(85)e-18")
}

fn codata_2018_electron_muon_source() -> SourceRecord {
    codata_2018_jpcrd("Electron, e-", "me/mmu = 4.83633169(11)e-3")
}

fn codata_2018_electron_proton_source() -> SourceRecord {
    codata_2018_jpcrd("Electron, e-", "me/mp = 5.44617021487(33)e-4")
}

fn codata_2018_electron_neutron_source() -> SourceRecord {
    codata_2018_jpcrd("Electron, e-", "me/mn = 5.4386734424(26)e-4")
}

fn codata_2018_electron_deuteron_source() -> SourceRecord {
    codata_2018_jpcrd("Electron, e-", "me/md = 2.724437107462(96)e-4")
}

fn codata_2018_electron_triton_source() -> SourceRecord {
    codata_2018_jpcrd("Electron, e-", "me/mt = 1.819200062251(90)e-4")
}

fn codata_2018_electron_helion_source() -> SourceRecord {
    codata_2018_jpcrd("Electron, e-", "me/mh = 1.819543074573(79)e-4")
}

fn codata_2018_electron_alpha_source() -> SourceRecord {
    codata_2018_jpcrd("Electron, e-", "me/malpha = 1.370933554787(45)e-4")
}

fn codata_2018_electron_charge_to_mass_source() -> SourceRecord {
    codata_2018_jpcrd("Electron, e-", "-e/me = -1.75882001076(53)e11")
}

fn codata_2018_electron_molar_mass_source() -> SourceRecord {
    codata_2018_jpcrd("Electron, e-", "Me = 5.4857990888(17)e-7")
}

fn codata_2018_reduced_compton_source() -> SourceRecord {
    codata_2018_jpcrd("Electron, e-", "lambdabar_C = 3.8615926796(12)e-13")
}

fn codata_2018_compton_source() -> SourceRecord {
    codata_2018_jpcrd("Electron, e-", "lambda_C = 2.42631023867(73)e-12")
}

fn codata_2018_classical_radius_source() -> SourceRecord {
    codata_2018_jpcrd("Electron, e-", "re = 2.8179403262(13)e-15")
}

fn codata_2018_proton_mass_source() -> SourceRecord {
    codata_2018_jpcrd("Proton, p", "mp = 1.67262192369(51)e-27")
}

/// CODATA 2018 one-sigma hull of 6.67430(15)×10⁻¹¹ m³ kg⁻¹ s⁻².
fn codata_2018_g_interval() -> Interval {
    let scale = 10i128.pow(16);
    let mu = 667_430;
    let sigma = 15;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Newtonian gravitational constant, CODATA 2018 one-sigma enclosure.
///
/// This is a measured hull, not an SI defining Ratio and not P3N.
/// Theories still use `physis_model` `f64` Qty.
pub fn newtonian_g() -> Constant<Interval> {
    Constant::new(
        "G",
        codata_2018_g_interval(),
        "m^3 kg^{-1} s^{-2}",
        codata_2018_g_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 1.25663706212(19)×10⁻⁶ N A⁻².
fn codata_2018_mu0_interval() -> Interval {
    let scale = 10i128.pow(17);
    let mu = 125_663_706_212;
    let sigma = 19;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Vacuum magnetic permeability μ₀, CODATA 2018 one-sigma enclosure.
///
/// After SI 2019 this is a measured hull, not an exact `4π×10^{-7}`
/// Ratio and not P3N. Theories still use `physis_model` `f64` Qty.
pub fn vacuum_permeability() -> Constant<Interval> {
    Constant::new(
        "mu0",
        codata_2018_mu0_interval(),
        "N A^{-2}",
        codata_2018_mu0_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 8.8541878128(13)×10⁻¹² F m⁻¹.
fn codata_2018_epsilon0_interval() -> Interval {
    let scale = 10i128.pow(22);
    let mu = 88_541_878_128;
    let sigma = 13;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Vacuum electric permittivity ε₀, CODATA 2018 one-sigma enclosure.
///
/// After SI 2019 this is the derived hull `1/(μ₀ c²)`, not an SI
/// defining Ratio and not P3N. Vacuum admittance `Y₀` is not stored.
/// Theories still use `physis_model` `f64` Qty.
pub fn vacuum_permittivity() -> Constant<Interval> {
    Constant::new(
        "epsilon0",
        codata_2018_epsilon0_interval(),
        "F m^{-1}",
        codata_2018_epsilon0_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 376.730313668(57) ohm.
fn codata_2018_z0_interval() -> Interval {
    let scale = 10i128.pow(9);
    let mu = 376_730_313_668;
    let sigma = 57;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Characteristic impedance of vacuum Z₀, CODATA 2018 one-sigma enclosure.
///
/// After SI 2019 this is the derived hull `μ₀ c`, not an SI defining
/// Ratio and not P3N. `Y₀ = 1/Z₀` is not stored. Theories still use
/// `physis_model` `f64` Qty.
pub fn vacuum_impedance() -> Constant<Interval> {
    Constant::new(
        "Z0",
        codata_2018_z0_interval(),
        "ohm",
        codata_2018_z0_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 7.2973525693(11)×10⁻³.
fn codata_2018_alpha_interval() -> Interval {
    let scale = 10i128.pow(13);
    let mu = 72_973_525_693;
    let sigma = 11;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Fine-structure constant α, CODATA 2018 one-sigma enclosure.
///
/// This is the zero-momentum recommended hull, not an SI defining
/// Ratio and not P3N. Running with energy is M4. Inverse-α is a
/// different recommended Interval. Theories still use `physis_model`
/// `f64` Qty.
pub fn fine_structure_constant() -> Constant<Interval> {
    Constant::new(
        "alpha",
        codata_2018_alpha_interval(),
        "1",
        codata_2018_alpha_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 137.035999084(21).
fn codata_2018_inv_alpha_interval() -> Interval {
    let scale = 10i128.pow(9);
    let mu = 137_035_999_084;
    let sigma = 21;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Inverse fine-structure constant α⁻¹, CODATA 2018 one-sigma enclosure.
///
/// This is the zero-momentum recommended hull, not an SI defining
/// Ratio and not P3N. It is a different recommended
/// value from α; the product of the two centres is not a certificate
/// that they multiply to one. Theories still use `physis_model` `f64`
/// Qty.
pub fn inverse_fine_structure_constant() -> Constant<Interval> {
    Constant::new(
        "inv_alpha",
        codata_2018_inv_alpha_interval(),
        "1",
        codata_2018_inv_alpha_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 3.2898419602508(64)×10¹⁵ Hz.
fn codata_2018_rydberg_frequency_interval() -> Interval {
    let mu = 32_898_419_602_508i128 * 100;
    let sigma = 64i128 * 100;
    Interval::new(Ratio::int(mu - sigma), Ratio::int(mu + sigma))
}

/// Rydberg frequency cR∞, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended hull in hertz, not an SI defining Ratio
/// and not P3N. It is a different recommended value from R∞; the exact
/// SI `c` does not make the stored centres a certificate that they
/// multiply. Theories still use `physis_model` `f64` Qty.
pub fn rydberg_frequency() -> Constant<Interval> {
    Constant::new(
        "cRinf",
        codata_2018_rydberg_frequency_interval(),
        "Hz",
        codata_2018_rydberg_frequency_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 2.1798723611035(42)×10⁻¹⁸ J.
fn codata_2018_rydberg_energy_interval() -> Interval {
    let scale = 10i128.pow(31);
    let mu = 21_798_723_611_035;
    let sigma = 42;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Rydberg energy equivalent hcR∞, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended hull in joules, not an SI defining Ratio,
/// not the eV conversion, and not P3N. It is a different recommended
/// value from E_h; the factor of two is not a certificate that the
/// stored centres divide. Theories still use `physis_model` `f64` Qty.
pub fn rydberg_energy_equivalent() -> Constant<Interval> {
    Constant::new(
        "hcRinf",
        codata_2018_rydberg_energy_interval(),
        "J",
        codata_2018_rydberg_energy_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 10973731.568160(21) m⁻¹.
fn codata_2018_rydberg_interval() -> Interval {
    let scale = 10i128.pow(6);
    let mu = 10_973_731_568_160;
    let sigma = 21;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Rydberg constant R∞, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended hull in inverse metres, not an SI defining
/// Ratio and not P3N. Theories still use `physis_model` `f64` Qty.
pub fn rydberg_constant() -> Constant<Interval> {
    Constant::new(
        "Rinf",
        codata_2018_rydberg_interval(),
        "m^{-1}",
        codata_2018_rydberg_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 5.29177210903(80)×10⁻¹¹ m.
fn codata_2018_bohr_interval() -> Interval {
    let scale = 10i128.pow(22);
    let mu = 529_177_210_903;
    let sigma = 80;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Bohr radius a₀, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended hull in metres, not an SI defining Ratio,
/// not the Hartree energy, and not P3N. Theories still use
/// `physis_model` `f64` Qty.
pub fn bohr_radius() -> Constant<Interval> {
    Constant::new(
        "a0",
        codata_2018_bohr_interval(),
        "m",
        codata_2018_bohr_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 4.3597447222071(85)×10⁻¹⁸ J.
fn codata_2018_hartree_interval() -> Interval {
    let scale = 10i128.pow(31);
    let mu = 43_597_447_222_071;
    let sigma = 85;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Hartree energy E_h, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended hull in joules, not an SI defining Ratio,
/// not the eV conversion, and not P3N. It is a different recommended
/// value from a₀; the algebraic relations `E_h = e²/(4πε₀ a₀) = 2 h c R∞`
/// are not a certificate that the stored centres multiply. Theories
/// still use `physis_model` `f64` Qty.
pub fn hartree_energy() -> Constant<Interval> {
    Constant::new(
        "Eh",
        codata_2018_hartree_interval(),
        "J",
        codata_2018_hartree_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 4.83633169(11)×10⁻³.
fn codata_2018_electron_muon_interval() -> Interval {
    let scale = 10i128.pow(11);
    let mu = 483_633_169;
    let sigma = 11;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Electron-muon mass ratio m_e/m_μ, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended dimensionless hull, not an SI defining Ratio,
/// not electron mass, and not P3N. The quantum of circulation `π ℏ/m_e`
/// is not stored. Theories still use `physis_model` `f64` Qty.
pub fn electron_muon_mass_ratio() -> Constant<Interval> {
    Constant::new(
        "me_mmu",
        codata_2018_electron_muon_interval(),
        "1",
        codata_2018_electron_muon_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 5.44617021487(33)×10⁻⁴.
fn codata_2018_electron_proton_interval() -> Interval {
    let scale = 10i128.pow(15);
    let mu = 544_617_021_487;
    let sigma = 33;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Electron-proton mass ratio m_e/m_p, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended dimensionless hull, not an SI defining Ratio,
/// not electron mass, and not P3N. It is a different recommended value
/// from m_e/m_μ; the muon and proton masses are not a certificate that
/// the stored centres divide. Theories still use `physis_model` `f64` Qty.
pub fn electron_proton_mass_ratio() -> Constant<Interval> {
    Constant::new(
        "me_mp",
        codata_2018_electron_proton_interval(),
        "1",
        codata_2018_electron_proton_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 5.4386734424(26)×10⁻⁴.
fn codata_2018_electron_neutron_interval() -> Interval {
    let scale = 10i128.pow(14);
    let mu = 54_386_734_424;
    let sigma = 26;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Electron-neutron mass ratio m_e/m_n, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended dimensionless hull, not an SI defining Ratio,
/// not electron mass, and not P3N. It is a different recommended value
/// from m_e/m_p; the neutron and proton masses are not a certificate
/// that the stored centres divide. Theories still use `physis_model`
/// `f64` Qty.
pub fn electron_neutron_mass_ratio() -> Constant<Interval> {
    Constant::new(
        "me_mn",
        codata_2018_electron_neutron_interval(),
        "1",
        codata_2018_electron_neutron_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 2.724437107462(96)×10⁻⁴.
fn codata_2018_electron_deuteron_interval() -> Interval {
    let scale = 10i128.pow(16);
    let mu = 2_724_437_107_462;
    let sigma = 96;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Electron-deuteron mass ratio m_e/m_d, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended dimensionless hull, not an SI defining Ratio,
/// not electron mass, and not P3N. It is a different recommended value
/// from m_e/m_n; the deuteron and neutron masses are not a certificate
/// that the stored centres divide. Theories still use `physis_model`
/// `f64` Qty.
pub fn electron_deuteron_mass_ratio() -> Constant<Interval> {
    Constant::new(
        "me_md",
        codata_2018_electron_deuteron_interval(),
        "1",
        codata_2018_electron_deuteron_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 1.819200062251(90)×10⁻⁴.
fn codata_2018_electron_triton_interval() -> Interval {
    let scale = 10i128.pow(16);
    let mu = 1_819_200_062_251;
    let sigma = 90;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Electron-triton mass ratio m_e/m_t, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended dimensionless hull, not an SI defining Ratio,
/// not electron mass, and not P3N. It is a different recommended value
/// from m_e/m_d; the triton and deuteron masses are not a certificate
/// that the stored centres divide. Theories still use `physis_model`
/// `f64` Qty.
pub fn electron_triton_mass_ratio() -> Constant<Interval> {
    Constant::new(
        "me_mt",
        codata_2018_electron_triton_interval(),
        "1",
        codata_2018_electron_triton_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 1.819543074573(79)×10⁻⁴.
fn codata_2018_electron_helion_interval() -> Interval {
    let scale = 10i128.pow(16);
    let mu = 1_819_543_074_573;
    let sigma = 79;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Electron-helion mass ratio m_e/m_h, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended dimensionless hull, not an SI defining Ratio,
/// not electron mass, and not P3N. It is a different recommended value
/// from m_e/m_t; the helion and triton masses are not a certificate
/// that the stored centres divide. Theories still use `physis_model`
/// `f64` Qty.
pub fn electron_helion_mass_ratio() -> Constant<Interval> {
    Constant::new(
        "me_mh",
        codata_2018_electron_helion_interval(),
        "1",
        codata_2018_electron_helion_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 1.370933554787(45)×10⁻⁴.
fn codata_2018_electron_alpha_interval() -> Interval {
    let scale = 10i128.pow(16);
    let mu = 1_370_933_554_787;
    let sigma = 45;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Electron to alpha particle mass ratio m_e/m_α, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended dimensionless hull, not an SI defining Ratio,
/// not electron mass, and not P3N. It is a different recommended value
/// from m_e/m_h; the alpha and helion masses are not a certificate
/// that the stored centres divide. Theories still use `physis_model`
/// `f64` Qty.
pub fn electron_alpha_mass_ratio() -> Constant<Interval> {
    Constant::new(
        "me_malpha",
        codata_2018_electron_alpha_interval(),
        "1",
        codata_2018_electron_alpha_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of −1.75882001076(53)×10¹¹ C kg⁻¹.
fn codata_2018_electron_charge_to_mass_interval() -> Interval {
    let mu = -175_882_001_076i128;
    let sigma = 53;
    Interval::new(Ratio::int(mu - sigma), Ratio::int(mu + sigma))
}

/// Electron charge to mass quotient −e/m_e, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended signed hull in C kg⁻¹, not an SI defining
/// Ratio, not electron mass, and not P3N. It is a different recommended
/// value from the SI-exact elementary charge; `e/m_e` from those
/// constructors is not this Interval. Theories still use
/// `physis_model` `f64` Qty.
pub fn electron_charge_to_mass() -> Constant<Interval> {
    Constant::new(
        "e_me",
        codata_2018_electron_charge_to_mass_interval(),
        "C kg^{-1}",
        codata_2018_electron_charge_to_mass_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 5.4857990888(17)×10⁻⁷ kg mol⁻¹.
fn codata_2018_electron_molar_mass_interval() -> Interval {
    let scale = 10i128.pow(17);
    let mu = 54_857_990_888;
    let sigma = 17;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Electron molar mass M_e, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended hull in kg mol⁻¹, not an SI defining Ratio,
/// not electron mass in kg (`10^{42}` overflows `i128`), not the
/// mass-in-u row, and not P3N. After SI 2019 this is a different
/// recommended value from `A_r(e) × 10^{-3}`. Theories still use
/// `physis_model` `f64` Qty.
pub fn electron_molar_mass() -> Constant<Interval> {
    Constant::new(
        "M_e",
        codata_2018_electron_molar_mass_interval(),
        "kg mol^{-1}",
        codata_2018_electron_molar_mass_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 3.8615926796(12)×10⁻¹³ m.
fn codata_2018_reduced_compton_interval() -> Interval {
    let scale = 10i128.pow(23);
    let mu = 38_615_926_796;
    let sigma = 12;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Reduced Compton wavelength ƛ_C, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended hull in metres, not an SI defining Ratio,
/// not a certificate that `ƛ_C = α a₀`, not the Compton wavelength
/// `λ_C`, and not P3N. Theories still use `physis_model` `f64` Qty.
pub fn reduced_compton_wavelength() -> Constant<Interval> {
    Constant::new(
        "lambdabar_C",
        codata_2018_reduced_compton_interval(),
        "m",
        codata_2018_reduced_compton_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 2.42631023867(73)×10⁻¹² m.
fn codata_2018_compton_interval() -> Interval {
    let scale = 10i128.pow(23);
    let mu = 242_631_023_867;
    let sigma = 73;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Compton wavelength λ_C, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended hull in metres, not an SI defining Ratio,
/// not a certificate that `λ_C = 2π ƛ_C`, not the reduced Compton
/// wavelength, and not P3N. Theories still use `physis_model` `f64` Qty.
pub fn compton_wavelength() -> Constant<Interval> {
    Constant::new(
        "lambda_C",
        codata_2018_compton_interval(),
        "m",
        codata_2018_compton_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 2.8179403262(13)×10⁻¹⁵ m.
fn codata_2018_classical_radius_interval() -> Interval {
    let scale = 10i128.pow(25);
    let mu = 28_179_403_262;
    let sigma = 13;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Classical electron radius r_e, CODATA 2018 one-sigma enclosure.
///
/// This is the recommended hull in metres, not an SI defining Ratio,
/// not a certificate that `r_e = α² a₀`, not the Thomson cross section,
/// and not P3N. Theories still use `physis_model` `f64` Qty.
pub fn classical_electron_radius() -> Constant<Interval> {
    Constant::new(
        "re",
        codata_2018_classical_radius_interval(),
        "m",
        codata_2018_classical_radius_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

/// CODATA 2018 one-sigma hull of 1.67262192369(51)×10⁻²⁷ kg.
fn codata_2018_proton_mass_interval() -> Interval {
    let scale = 10i128.pow(38);
    let mu = 167_262_192_369;
    let sigma = 51;
    Interval::new(Ratio::new(mu - sigma, scale), Ratio::new(mu + sigma, scale))
}

/// Proton mass m_p, CODATA 2018 one-sigma enclosure.
///
/// This is a measured hull, not an SI defining Ratio and not P3N.
/// Electron mass is not stored: `10^{42}` overflows `i128`. Theories
/// still use `physis_model` `f64` Qty.
pub fn proton_mass() -> Constant<Interval> {
    Constant::new(
        "m_p",
        codata_2018_proton_mass_interval(),
        "kg",
        codata_2018_proton_mass_source(),
        ConstantRelease::Si2019Codata2018,
    )
}

fn si_brochure_table_8(range: &str) -> SourceRecord {
    SourceRecord::new(
        Citation {
            work: "BIPM SI Brochure".into(),
            edition: "9th".into(),
        },
        "2019",
        SourceLocator {
            page: None,
            section: Some("Non-SI units accepted for use with the SI".into()),
            equation: None,
            figure: None,
            table: Some("8".into()),
            dataset_range: Some(range.into()),
            experiment: None,
        },
        ArtifactId::of(b"si-brochure-9"),
        None,
    )
    .expect("SI Brochure table 8 locator is precise")
}

/// Astronomical unit, exact, IAU 2012 Resolution B2 / BIPM table 8.
///
/// `1 au = 149 597 870 700 m` exactly. This is a conventional length,
/// not an SI defining constant. The parsec is `(648 000 / π) au` and
/// is not stored here: π means it is not a Ratio.
pub fn astronomical_unit() -> Constant<Ratio> {
    Constant::new(
        "au",
        Ratio::int(149_597_870_700),
        "m",
        si_brochure_table_8("1 au = 149 597 870 700 m"),
        ConstantRelease::Si2019Codata2018,
    )
}

/// Electronvolt, exact, SI 2019 / BIPM table 8.
///
/// `1 eV = 1.602176634×10^{-19} J` exactly, from the SI defining charge.
/// Same Ratio as [`elementary_charge`], different unit and locator.
/// Theories still evaluate with the `physis_model` Qty.
pub fn electron_volt() -> Constant<Ratio> {
    Constant::new(
        "eV",
        Ratio::new(1_602_176_634, 10i128.pow(28)),
        "J",
        si_brochure_table_8("1 eV = 1.602176634e-19 J"),
        ConstantRelease::Si2019Codata2018,
    )
}

fn iau2015_b3_table_1(range: &str) -> SourceRecord {
    SourceRecord::new(
        Citation {
            work:
                "Nominal values for selected solar and planetary quantities: IAU 2015 Resolution B3"
                    .into(),
            edition: "Astron. J. 152, 41".into(),
        },
        "2015",
        SourceLocator {
            page: None,
            section: Some("Nominal Solar and Planetary Conversion Constants".into()),
            equation: None,
            figure: None,
            table: Some("1".into()),
            dataset_range: Some(range.into()),
            experiment: None,
        },
        ArtifactId::of(b"iau-2015-b3-aj-152-41"),
        None,
    )
    .expect("IAU 2015 B3 table 1 locator is precise")
}

/// Nominal solar mass parameter (GM)_☉^N, exact, IAU 2015 Resolution B3.
///
/// `1.3271244×10^20 m³ s⁻²` is a conversion ruler, not a measured solar
/// mass and not CODATA `G`. Newtonian-gravity still evaluates with the
/// `physis_model` Qty.
pub fn solar_gm() -> Constant<Ratio> {
    Constant::new(
        "GM_sun",
        Ratio::int(13_271_244i128 * 10i128.pow(13)),
        "m^3 s^{-2}",
        iau2015_b3_table_1("(GM)_sun^N = 1.3271244e20"),
        ConstantRelease::Iau2015,
    )
}

/// Nominal solar radius R_☉^N, exact, IAU 2015 Resolution B3.
///
/// `6.957×10^8 m` is a conversion ruler, not a measured photospheric
/// radius. Newtonian-gravity still evaluates with the `physis_model`
/// Qty.
pub fn solar_radius() -> Constant<Ratio> {
    Constant::new(
        "R_sun",
        Ratio::int(695_700_000),
        "m",
        iau2015_b3_table_1("R_sun^N = 6.957e8"),
        ConstantRelease::Iau2015,
    )
}

/// Nominal solar luminosity L_☉^N, exact, IAU 2015 Resolution B3.
///
/// `3.828×10^26 W` is a conversion ruler, not a measured solar
/// luminosity. Theories still evaluate with the `physis_model` Qty.
pub fn solar_luminosity() -> Constant<Ratio> {
    Constant::new(
        "L_sun",
        Ratio::int(3_828i128 * 10i128.pow(23)),
        "W",
        iau2015_b3_table_1("L_sun^N = 3.828e26"),
        ConstantRelease::Iau2015,
    )
}

/// Independently reconstructable listing of a versioned constant.
///
/// The stored [`Constant::hash`] is not authority: rebuild via [`lookup`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConstantListing {
    /// Ledger name (`c`, `G`, `h`, …).
    pub name: String,
    /// `ratio`, `sci-exact`, or `interval`.
    pub kind: &'static str,
    /// Unit string.
    pub unit: String,
    /// Display form of the value.
    pub value: String,
    /// Release.
    pub release: ConstantRelease,
    /// Table locator, if any.
    pub table: Option<String>,
    /// Dataset range locator, if any.
    pub range: Option<String>,
    /// Content hash of the live [`Constant`].
    pub hash: ArtifactId,
    /// Provenance. Recheck independently; the stored hash is not authority.
    pub source: SourceRecord,
}

/// Ledger names in catalog order.
pub const LEDGER: &[&str] = &[
    "c",
    "delta-nu-Cs",
    "e",
    "k",
    "N_A",
    "K_cd",
    "h",
    "G",
    "mu0",
    "epsilon0",
    "Z0",
    "alpha",
    "inv_alpha",
    "cRinf",
    "hcRinf",
    "Rinf",
    "a0",
    "Eh",
    "me_mmu",
    "me_mp",
    "me_mn",
    "me_md",
    "me_mt",
    "me_mh",
    "me_malpha",
    "e_me",
    "M_e",
    "lambdabar_C",
    "lambda_C",
    "re",
    "m_p",
    "au",
    "eV",
    "GM_sun",
    "R_sun",
    "L_sun",
];

fn listing<T: std::fmt::Display>(c: Constant<T>, kind: &'static str) -> ConstantListing {
    ConstantListing {
        name: c.name,
        kind,
        unit: c.unit,
        value: c.value.to_string(),
        release: c.release,
        table: c.provenance.locator.table.clone(),
        range: c.provenance.locator.dataset_range.clone(),
        hash: c.hash,
        source: c.provenance,
    }
}

/// Rebuild a versioned constant from live constructors. Unknown names are
/// absent: this is not a claim slug lookup.
pub fn lookup(name: &str) -> Option<ConstantListing> {
    match name {
        "c" => Some(listing(speed_of_light(), "ratio")),
        "delta-nu-Cs" => Some(listing(caesium_hyperfine(), "ratio")),
        "e" => Some(listing(elementary_charge(), "ratio")),
        "k" => Some(listing(boltzmann(), "ratio")),
        "N_A" => Some(listing(avogadro(), "ratio")),
        "K_cd" => Some(listing(luminous_efficacy(), "ratio")),
        "h" => Some(listing(planck_h(), "sci-exact")),
        "G" => Some(listing(newtonian_g(), "interval")),
        "mu0" => Some(listing(vacuum_permeability(), "interval")),
        "epsilon0" => Some(listing(vacuum_permittivity(), "interval")),
        "Z0" => Some(listing(vacuum_impedance(), "interval")),
        "alpha" => Some(listing(fine_structure_constant(), "interval")),
        "inv_alpha" => Some(listing(inverse_fine_structure_constant(), "interval")),
        "cRinf" => Some(listing(rydberg_frequency(), "interval")),
        "hcRinf" => Some(listing(rydberg_energy_equivalent(), "interval")),
        "Rinf" => Some(listing(rydberg_constant(), "interval")),
        "a0" => Some(listing(bohr_radius(), "interval")),
        "Eh" => Some(listing(hartree_energy(), "interval")),
        "me_mmu" => Some(listing(electron_muon_mass_ratio(), "interval")),
        "me_mp" => Some(listing(electron_proton_mass_ratio(), "interval")),
        "me_mn" => Some(listing(electron_neutron_mass_ratio(), "interval")),
        "me_md" => Some(listing(electron_deuteron_mass_ratio(), "interval")),
        "me_mt" => Some(listing(electron_triton_mass_ratio(), "interval")),
        "me_mh" => Some(listing(electron_helion_mass_ratio(), "interval")),
        "me_malpha" => Some(listing(electron_alpha_mass_ratio(), "interval")),
        "e_me" => Some(listing(electron_charge_to_mass(), "interval")),
        "M_e" => Some(listing(electron_molar_mass(), "interval")),
        "lambdabar_C" => Some(listing(reduced_compton_wavelength(), "interval")),
        "lambda_C" => Some(listing(compton_wavelength(), "interval")),
        "re" => Some(listing(classical_electron_radius(), "interval")),
        "m_p" => Some(listing(proton_mass(), "interval")),
        "au" => Some(listing(astronomical_unit(), "ratio")),
        "eV" => Some(listing(electron_volt(), "ratio")),
        "GM_sun" => Some(listing(solar_gm(), "ratio")),
        "R_sun" => Some(listing(solar_radius(), "ratio")),
        "L_sun" => Some(listing(solar_luminosity(), "ratio")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_is_exact_and_versioned() {
        let c = speed_of_light();
        assert_eq!(c.value, Ratio::int(299_792_458));
        assert_eq!(c.release, ConstantRelease::Si2019Codata2018);
        let c2 = speed_of_light();
        assert_eq!(c.hash, c2.hash);
        assert_eq!(
            c.hash.to_hex(),
            "691eb73ea444f6d10fb223b999a1b37c0b67da92d51e43ca8bd8a6561785a3c1"
        );
    }

    #[test]
    fn si2019_defining_constants_that_fit_are_exact() {
        assert_eq!(caesium_hyperfine().value, Ratio::int(9_192_631_770));
        assert_eq!(
            elementary_charge().value,
            Ratio::new(1_602_176_634, 10i128.pow(28))
        );
        assert_eq!(boltzmann().value, Ratio::new(1_380_649, 10i128.pow(29)));
        assert_eq!(
            avogadro().value,
            Ratio::int(602_214_076i128 * 1_000_000_000_000_000i128)
        );
        assert_eq!(luminous_efficacy().value, Ratio::int(683));
        for c in [
            speed_of_light(),
            caesium_hyperfine(),
            elementary_charge(),
            boltzmann(),
            avogadro(),
            luminous_efficacy(),
        ] {
            assert_eq!(c.release, ConstantRelease::Si2019Codata2018);
            assert_eq!(c.provenance.locator.table.as_deref(), Some("1"));
            assert_eq!(c.hash, si2019_exact(&c.name, c.value, &c.unit).hash);
        }
        assert_ne!(elementary_charge().hash, speed_of_light().hash);
        assert_ne!(boltzmann().hash, avogadro().hash);
        assert_eq!(
            speed_of_light().hash.to_hex(),
            "691eb73ea444f6d10fb223b999a1b37c0b67da92d51e43ca8bd8a6561785a3c1"
        );
        assert_eq!(
            caesium_hyperfine().hash.to_hex(),
            "92d2278bbaa885fdb3b752b828d8e13c3cb65971f3b4a9a367be830d35b6e0a0"
        );
        assert_eq!(
            elementary_charge().hash.to_hex(),
            "412cb379a6bf6cca245ba89fc43539399942e644fa08000cd30bd1d9b25372a5"
        );
        assert_eq!(
            boltzmann().hash.to_hex(),
            "0d6156b1dea5afb156a9bbdcde78709fcfbac53df129a27698ea3fd76e812061"
        );
        assert_eq!(
            avogadro().hash.to_hex(),
            "410e2191c8cf7c074a32f621413239e74a7fefe735cacfaad4f503c47c9351dc"
        );
        assert_eq!(
            luminous_efficacy().hash.to_hex(),
            "236a02d738fe3bd59dd4e16a15175aefca53d9d2dfa1a906d2a52d31204ca9b7"
        );
    }

    #[test]
    fn planck_h_denominator_does_not_fit_i128() {
        assert!(
            10i128.checked_pow(42).is_none(),
            "h = 6.62607015e-34 is 662607015/10^42; that denominator overflows i128"
        );
        assert_eq!(planck_h().value.to_ratio(), None);
    }

    #[test]
    fn planck_h_is_si_exact_sci_exact() {
        let h = planck_h();
        assert_eq!(h.name, "h");
        assert_eq!(h.unit, "J s");
        assert_eq!(h.value, SciExact::new(662_607_015, -42));
        assert_eq!(h.value, SciExact::new(6_626_070_150, -43));
        assert_eq!(h.value.to_string(), "662607015e-42");
        assert_eq!(h.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(h.provenance.locator.table.as_deref(), Some("1"));
        assert_eq!(
            h.hash,
            Constant::new(
                "h",
                SciExact::new(662_607_015, -42),
                "J s",
                si_brochure(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(h.hash, speed_of_light().hash);
        assert_ne!(h.hash, newtonian_g().hash);
        assert_eq!(
            h.hash.to_hex(),
            "50a96a8715769547a90cba69b0775d8892d79f2fa32465ad13a6d73b2d111eef"
        );
        assert!(h.provenance.recheck().is_ok());
    }

    #[test]
    fn codata_2018_g_is_a_one_sigma_interval() {
        let g = newtonian_g();
        let scale = 10i128.pow(16);
        let lo = Ratio::new(667_415, scale);
        let hi = Ratio::new(667_445, scale);
        let centre = Ratio::new(667_430, scale);
        assert_eq!(g.name, "G");
        assert_eq!(g.unit, "m^3 kg^{-1} s^{-2}");
        assert_eq!(g.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(g.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(g.provenance.locator.section.as_deref(), Some("UNIVERSAL"));
        assert_eq!(
            g.provenance.locator.dataset_range.as_deref(),
            Some("G = 6.67430(15)e-11")
        );
        assert_eq!(g.value, Interval::new(lo, hi));
        assert_ne!(g.value.lo, g.value.hi, "G is measured, not SI-exact");
        assert!(g.value.contains(Interval::point(centre)));
        assert!(!g
            .value
            .contains(Interval::point(Ratio::new(667_000, scale))));
        assert_eq!(g.hash, newtonian_g().hash);
        assert_eq!(
            g.hash,
            Constant::new(
                "G",
                codata_2018_g_interval(),
                "m^3 kg^{-1} s^{-2}",
                codata_2018_g_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(g.hash, speed_of_light().hash);
        assert_eq!(
            g.hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92"
        );
        assert!(g.provenance.recheck().is_ok());
    }

    #[test]
    fn codata_2018_mu0_is_a_one_sigma_interval() {
        let mu0 = vacuum_permeability();
        let scale = 10i128.pow(17);
        let lo = Ratio::new(125_663_706_193, scale);
        let hi = Ratio::new(125_663_706_231, scale);
        let centre = Ratio::new(125_663_706_212, scale);
        assert_eq!(mu0.name, "mu0");
        assert_eq!(mu0.unit, "N A^{-2}");
        assert_eq!(mu0.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(mu0.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(mu0.provenance.locator.section.as_deref(), Some("UNIVERSAL"));
        assert_eq!(
            mu0.provenance.locator.dataset_range.as_deref(),
            Some("mu0 = 1.25663706212(19)e-6")
        );
        assert_eq!(mu0.value, Interval::new(lo, hi));
        assert_ne!(mu0.value.lo, mu0.value.hi, "mu0 is measured, not SI-exact");
        assert!(mu0.value.contains(Interval::point(centre)));
        assert!(!mu0
            .value
            .contains(Interval::point(Ratio::new(125_663_000_000, scale))));
        assert_eq!(mu0.hash, vacuum_permeability().hash);
        assert_eq!(
            mu0.hash,
            Constant::new(
                "mu0",
                codata_2018_mu0_interval(),
                "N A^{-2}",
                codata_2018_mu0_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(mu0.hash, newtonian_g().hash, "mu0 is not G");
        assert_ne!(mu0.hash, fine_structure_constant().hash, "mu0 is not alpha");
        assert_ne!(mu0.hash, proton_mass().hash, "mu0 is not m_p");
        assert_ne!(
            mu0.provenance.source_hash,
            newtonian_g().provenance.source_hash,
            "mu0 range is not the G range"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when mu0 is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when mu0 is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when mu0 is added"
        );
        assert_eq!(
            mu0.hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7"
        );
        assert!(mu0.provenance.recheck().is_ok());
        assert!(lookup("Y0").is_none());
        assert!(lookup("mu_0").is_none());
        assert!(lookup("vacuum-permeability").is_none());
    }

    #[test]
    fn codata_2018_epsilon0_is_a_one_sigma_interval() {
        let eps = vacuum_permittivity();
        let scale = 10i128.pow(22);
        let lo = Ratio::new(88_541_878_115, scale);
        let hi = Ratio::new(88_541_878_141, scale);
        let centre = Ratio::new(88_541_878_128, scale);
        assert_eq!(eps.name, "epsilon0");
        assert_eq!(eps.unit, "F m^{-1}");
        assert_eq!(eps.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(eps.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(eps.provenance.locator.section.as_deref(), Some("UNIVERSAL"));
        assert_eq!(
            eps.provenance.locator.dataset_range.as_deref(),
            Some("epsilon0 = 8.8541878128(13)e-12")
        );
        assert_eq!(eps.value, Interval::new(lo, hi));
        assert_ne!(
            eps.value.lo, eps.value.hi,
            "epsilon0 is measured, not SI-exact"
        );
        assert!(eps.value.contains(Interval::point(centre)));
        assert!(!eps
            .value
            .contains(Interval::point(Ratio::new(88_541_000_000, scale))));
        assert_eq!(eps.hash, vacuum_permittivity().hash);
        assert_eq!(
            eps.hash,
            Constant::new(
                "epsilon0",
                codata_2018_epsilon0_interval(),
                "F m^{-1}",
                codata_2018_epsilon0_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(eps.hash, vacuum_permeability().hash, "epsilon0 is not mu0");
        assert_ne!(eps.hash, newtonian_g().hash, "epsilon0 is not G");
        assert_ne!(
            eps.hash,
            fine_structure_constant().hash,
            "epsilon0 is not alpha"
        );
        assert_ne!(
            eps.provenance.source_hash,
            vacuum_permeability().provenance.source_hash,
            "epsilon0 range is not the mu0 range"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when epsilon0 is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when epsilon0 is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when epsilon0 is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when epsilon0 is added"
        );
        assert_eq!(
            eps.hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80"
        );
        assert!(eps.provenance.recheck().is_ok());
        assert!(lookup("Y0").is_none());
        assert!(lookup("Z_0").is_none());
        assert!(lookup("epsilon_0").is_none());
        assert!(lookup("eps0").is_none());
    }

    #[test]
    fn codata_2018_z0_is_a_one_sigma_interval() {
        let z0 = vacuum_impedance();
        let scale = 10i128.pow(9);
        let lo = Ratio::new(376_730_313_611, scale);
        let hi = Ratio::new(376_730_313_725, scale);
        let centre = Ratio::new(376_730_313_668, scale);
        assert_eq!(z0.name, "Z0");
        assert_eq!(z0.unit, "ohm");
        assert_eq!(z0.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(z0.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(z0.provenance.locator.section.as_deref(), Some("UNIVERSAL"));
        assert_eq!(
            z0.provenance.locator.dataset_range.as_deref(),
            Some("Z0 = 376.730313668(57)")
        );
        assert_eq!(z0.value, Interval::new(lo, hi));
        assert_ne!(z0.value.lo, z0.value.hi, "Z0 is measured, not SI-exact");
        assert!(z0.value.contains(Interval::point(centre)));
        assert!(!z0
            .value
            .contains(Interval::point(Ratio::new(376_730_000_000, scale))));
        assert_eq!(z0.hash, vacuum_impedance().hash);
        assert_eq!(
            z0.hash,
            Constant::new(
                "Z0",
                codata_2018_z0_interval(),
                "ohm",
                codata_2018_z0_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(z0.hash, vacuum_permittivity().hash, "Z0 is not epsilon0");
        assert_ne!(z0.hash, vacuum_permeability().hash, "Z0 is not mu0");
        assert_ne!(z0.hash, newtonian_g().hash, "Z0 is not G");
        assert_ne!(
            z0.provenance.source_hash,
            vacuum_permittivity().provenance.source_hash,
            "Z0 range is not the epsilon0 range"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when Z0 is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when Z0 is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when Z0 is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when Z0 is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when Z0 is added"
        );
        assert_eq!(
            z0.hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316"
        );
        assert!(z0.provenance.recheck().is_ok());
        assert!(lookup("Y0").is_none());
        assert!(lookup("Z_0").is_none());
        assert!(lookup("impedance").is_none());
    }

    #[test]
    fn codata_2018_alpha_is_a_one_sigma_interval() {
        let alpha = fine_structure_constant();
        let scale = 10i128.pow(13);
        let lo = Ratio::new(72_973_525_682, scale);
        let hi = Ratio::new(72_973_525_704, scale);
        let centre = Ratio::new(72_973_525_693, scale);
        assert_eq!(alpha.name, "alpha");
        assert_eq!(alpha.unit, "1");
        assert_eq!(alpha.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(alpha.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            alpha.provenance.locator.section.as_deref(),
            Some("ATOMIC AND NUCLEAR")
        );
        assert_eq!(
            alpha.provenance.locator.dataset_range.as_deref(),
            Some("alpha = 7.2973525693(11)e-3")
        );
        assert_eq!(alpha.value, Interval::new(lo, hi));
        assert_ne!(
            alpha.value.lo, alpha.value.hi,
            "alpha is measured, not SI-exact"
        );
        assert!(alpha.value.contains(Interval::point(centre)));
        assert!(!alpha
            .value
            .contains(Interval::point(Ratio::new(72_973_000_000, scale))));
        assert_eq!(alpha.hash, fine_structure_constant().hash);
        assert_eq!(
            alpha.hash,
            Constant::new(
                "alpha",
                codata_2018_alpha_interval(),
                "1",
                codata_2018_alpha_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(alpha.hash, newtonian_g().hash, "alpha is not G");
        assert_ne!(alpha.hash, speed_of_light().hash);
        assert_ne!(
            alpha.provenance.source_hash,
            newtonian_g().provenance.source_hash,
            "alpha range is not the G range"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when alpha is added"
        );
        assert_eq!(
            alpha.hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38"
        );
        assert!(alpha.provenance.recheck().is_ok());
        assert!(lookup("alpha-inv").is_none());
        assert!(lookup("fine-structure").is_none());
    }

    #[test]
    fn codata_2018_inv_alpha_is_a_one_sigma_interval() {
        let inv = inverse_fine_structure_constant();
        let scale = 10i128.pow(9);
        let lo = Ratio::new(137_035_999_063, scale);
        let hi = Ratio::new(137_035_999_105, scale);
        let centre = Ratio::new(137_035_999_084, scale);
        assert_eq!(inv.name, "inv_alpha");
        assert_eq!(inv.unit, "1");
        assert_eq!(inv.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(inv.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            inv.provenance.locator.section.as_deref(),
            Some("ATOMIC AND NUCLEAR")
        );
        assert_eq!(
            inv.provenance.locator.dataset_range.as_deref(),
            Some("inv_alpha = 137.035999084(21)")
        );
        assert_eq!(inv.value, Interval::new(lo, hi));
        assert_ne!(
            inv.value.lo, inv.value.hi,
            "inv_alpha is measured, not SI-exact"
        );
        assert!(inv.value.contains(Interval::point(centre)));
        assert!(!inv
            .value
            .contains(Interval::point(Ratio::new(137_035_000_000, scale))));
        assert_eq!(inv.hash, inverse_fine_structure_constant().hash);
        assert_eq!(
            inv.hash,
            Constant::new(
                "inv_alpha",
                codata_2018_inv_alpha_interval(),
                "1",
                codata_2018_inv_alpha_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(
            inv.hash,
            fine_structure_constant().hash,
            "inv_alpha is not alpha"
        );
        assert_ne!(inv.hash, newtonian_g().hash, "inv_alpha is not G");
        assert_ne!(
            inv.provenance.source_hash,
            fine_structure_constant().provenance.source_hash,
            "inv_alpha range is not the alpha range"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when inv_alpha is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when inv_alpha is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when inv_alpha is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when inv_alpha is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when inv_alpha is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when inv_alpha is added"
        );
        assert_eq!(
            inv.hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482"
        );
        assert!(inv.provenance.recheck().is_ok());
        assert!(lookup("alpha-inv").is_none());
        assert!(lookup("alpha_inv").is_none());
        assert!(lookup("inverse-alpha").is_none());
        assert!(lookup("E_h").is_none());
    }

    #[test]
    fn codata_2018_rydberg_is_a_one_sigma_interval() {
        let r = rydberg_constant();
        let scale = 10i128.pow(6);
        let lo = Ratio::new(10_973_731_568_139, scale);
        let hi = Ratio::new(10_973_731_568_181, scale);
        let centre = Ratio::new(10_973_731_568_160, scale);
        assert_eq!(r.name, "Rinf");
        assert_eq!(r.unit, "m^{-1}");
        assert_eq!(r.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(r.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            r.provenance.locator.section.as_deref(),
            Some("ATOMIC AND NUCLEAR")
        );
        assert_eq!(
            r.provenance.locator.dataset_range.as_deref(),
            Some("Rinf = 10973731.568160(21)")
        );
        assert_eq!(r.value, Interval::new(lo, hi));
        assert_ne!(r.value.lo, r.value.hi, "Rinf is measured, not SI-exact");
        assert!(r.value.contains(Interval::point(centre)));
        assert!(!r
            .value
            .contains(Interval::point(Ratio::new(10_973_000_000_000, scale))));
        assert_eq!(r.hash, rydberg_constant().hash);
        assert_eq!(
            r.hash,
            Constant::new(
                "Rinf",
                codata_2018_rydberg_interval(),
                "m^{-1}",
                codata_2018_rydberg_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(
            r.hash,
            inverse_fine_structure_constant().hash,
            "Rinf is not inv_alpha"
        );
        assert_ne!(r.hash, newtonian_g().hash, "Rinf is not G");
        assert_ne!(
            r.provenance.source_hash,
            inverse_fine_structure_constant().provenance.source_hash,
            "Rinf range is not the inv_alpha range"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when Rinf is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when Rinf is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when Rinf is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when Rinf is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when Rinf is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when Rinf is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when Rinf is added"
        );
        assert_eq!(
            r.hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090"
        );
        assert!(r.provenance.recheck().is_ok());
        assert!(lookup("R_inf").is_none());
        assert!(lookup("Rydberg").is_none());
        assert!(lookup("hcRinf_eV").is_none());
        assert!(lookup("E_h").is_none());
    }

    #[test]
    fn codata_2018_rydberg_frequency_is_a_one_sigma_interval() {
        let f = rydberg_frequency();
        let mu = 32_898_419_602_508i128 * 100;
        let sigma = 64i128 * 100;
        let lo = Ratio::int(mu - sigma);
        let hi = Ratio::int(mu + sigma);
        let centre = Ratio::int(mu);
        assert_eq!(f.name, "cRinf");
        assert_eq!(f.unit, "Hz");
        assert_eq!(f.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(f.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            f.provenance.locator.section.as_deref(),
            Some("ATOMIC AND NUCLEAR")
        );
        assert_eq!(
            f.provenance.locator.dataset_range.as_deref(),
            Some("cRinf = 3.2898419602508(64)e15")
        );
        assert_eq!(f.value, Interval::new(lo, hi));
        assert_ne!(f.value.lo, f.value.hi, "cRinf is measured, not SI-exact");
        assert!(f.value.contains(Interval::point(centre)));
        assert!(!f
            .value
            .contains(Interval::point(Ratio::int(3_289_000_000_000_000))));
        assert_eq!(f.hash, rydberg_frequency().hash);
        assert_eq!(
            f.hash,
            Constant::new(
                "cRinf",
                codata_2018_rydberg_frequency_interval(),
                "Hz",
                codata_2018_rydberg_frequency_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(f.hash, rydberg_constant().hash, "cRinf is not Rinf");
        assert_ne!(f.hash, hartree_energy().hash, "cRinf is not Eh");
        assert_ne!(f.hash, newtonian_g().hash, "cRinf is not G");
        assert_ne!(
            f.provenance.source_hash,
            rydberg_constant().provenance.source_hash,
            "cRinf range is not the Rinf range"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when cRinf is added"
        );
        assert_eq!(
            hartree_energy().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a",
            "Eh hash must stay pinned when cRinf is added"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when cRinf is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when cRinf is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when cRinf is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when cRinf is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when cRinf is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when cRinf is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when cRinf is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when cRinf is added"
        );
        assert_eq!(
            f.hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969"
        );
        assert!(f.provenance.recheck().is_ok());
        assert!(
            mu < (1i128 << 53),
            "cRinf centre in Hz fits in the f64 integer range"
        );
        assert!(lookup("c_Rinf").is_none());
        assert!(lookup("Rydberg").is_none());
        assert!(lookup("hcRinf_eV").is_none());
        assert!(lookup("cRinf_eV").is_none());
    }

    #[test]
    fn codata_2018_rydberg_energy_equivalent_is_a_one_sigma_interval() {
        let e = rydberg_energy_equivalent();
        let scale = 10i128.pow(31);
        let lo = Ratio::new(21_798_723_610_993, scale);
        let hi = Ratio::new(21_798_723_611_077, scale);
        let centre = Ratio::new(21_798_723_611_035, scale);
        assert_eq!(e.name, "hcRinf");
        assert_eq!(e.unit, "J");
        assert_eq!(e.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(e.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            e.provenance.locator.section.as_deref(),
            Some("ATOMIC AND NUCLEAR")
        );
        assert_eq!(
            e.provenance.locator.dataset_range.as_deref(),
            Some("hcRinf = 2.1798723611035(42)e-18")
        );
        assert_eq!(e.value, Interval::new(lo, hi));
        assert_ne!(e.value.lo, e.value.hi, "hcRinf is measured, not SI-exact");
        assert!(e.value.contains(Interval::point(centre)));
        assert!(!e
            .value
            .contains(Interval::point(Ratio::new(21_798_000_000_000, scale))));
        assert_eq!(e.hash, rydberg_energy_equivalent().hash);
        assert_eq!(
            e.hash,
            Constant::new(
                "hcRinf",
                codata_2018_rydberg_energy_interval(),
                "J",
                codata_2018_rydberg_energy_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(e.hash, rydberg_frequency().hash, "hcRinf is not cRinf");
        assert_ne!(e.hash, hartree_energy().hash, "hcRinf is not Eh");
        assert_ne!(e.hash, rydberg_constant().hash, "hcRinf is not Rinf");
        assert_ne!(
            e.provenance.source_hash,
            rydberg_frequency().provenance.source_hash,
            "hcRinf range is not the cRinf range"
        );
        assert_eq!(
            rydberg_frequency().hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969",
            "cRinf hash must stay pinned when hcRinf is added"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when hcRinf is added"
        );
        assert_eq!(
            hartree_energy().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a",
            "Eh hash must stay pinned when hcRinf is added"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when hcRinf is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when hcRinf is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when hcRinf is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when hcRinf is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when hcRinf is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when hcRinf is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when hcRinf is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when hcRinf is added"
        );
        assert_eq!(
            e.hash.to_hex(),
            "0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2"
        );
        assert!(e.provenance.recheck().is_ok());
        assert!(lookup("hc_Rinf").is_none());
        assert!(lookup("hcRinf_eV").is_none());
        assert!(lookup("Rydberg").is_none());
    }

    #[test]
    fn codata_2018_bohr_radius_is_a_one_sigma_interval() {
        let a0 = bohr_radius();
        let scale = 10i128.pow(22);
        let lo = Ratio::new(529_177_210_823, scale);
        let hi = Ratio::new(529_177_210_983, scale);
        let centre = Ratio::new(529_177_210_903, scale);
        assert_eq!(a0.name, "a0");
        assert_eq!(a0.unit, "m");
        assert_eq!(a0.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(a0.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            a0.provenance.locator.section.as_deref(),
            Some("ATOMIC AND NUCLEAR")
        );
        assert_eq!(
            a0.provenance.locator.dataset_range.as_deref(),
            Some("a0 = 5.29177210903(80)e-11")
        );
        assert_eq!(a0.value, Interval::new(lo, hi));
        assert_ne!(a0.value.lo, a0.value.hi, "a0 is measured, not SI-exact");
        assert!(a0.value.contains(Interval::point(centre)));
        assert!(!a0
            .value
            .contains(Interval::point(Ratio::new(529_177_000_000, scale))));
        assert_eq!(a0.hash, bohr_radius().hash);
        assert_eq!(
            a0.hash,
            Constant::new(
                "a0",
                codata_2018_bohr_interval(),
                "m",
                codata_2018_bohr_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(a0.hash, rydberg_constant().hash, "a0 is not Rinf");
        assert_ne!(a0.hash, newtonian_g().hash, "a0 is not G");
        assert_ne!(
            a0.provenance.source_hash,
            rydberg_constant().provenance.source_hash,
            "a0 range is not the Rinf range"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when a0 is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when a0 is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when a0 is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when a0 is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when a0 is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when a0 is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when a0 is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when a0 is added"
        );
        assert_eq!(
            a0.hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29"
        );
        assert!(a0.provenance.recheck().is_ok());
        assert!(lookup("a_0").is_none());
        assert!(lookup("Bohr").is_none());
        assert!(lookup("bohr").is_none());
        assert!(lookup("E_h").is_none());
    }

    #[test]
    fn codata_2018_hartree_energy_is_a_one_sigma_interval() {
        let eh = hartree_energy();
        let scale = 10i128.pow(31);
        let lo = Ratio::new(43_597_447_221_986, scale);
        let hi = Ratio::new(43_597_447_222_156, scale);
        let centre = Ratio::new(43_597_447_222_071, scale);
        assert_eq!(eh.name, "Eh");
        assert_eq!(eh.unit, "J");
        assert_eq!(eh.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(eh.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            eh.provenance.locator.section.as_deref(),
            Some("ATOMIC AND NUCLEAR")
        );
        assert_eq!(
            eh.provenance.locator.dataset_range.as_deref(),
            Some("Eh = 4.3597447222071(85)e-18")
        );
        assert_eq!(eh.value, Interval::new(lo, hi));
        assert_ne!(eh.value.lo, eh.value.hi, "Eh is measured, not SI-exact");
        assert!(eh.value.contains(Interval::point(centre)));
        assert!(!eh
            .value
            .contains(Interval::point(Ratio::new(43_597_000_000_000, scale))));
        assert_eq!(eh.hash, hartree_energy().hash);
        assert_eq!(
            eh.hash,
            Constant::new(
                "Eh",
                codata_2018_hartree_interval(),
                "J",
                codata_2018_hartree_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(eh.hash, bohr_radius().hash, "Eh is not a0");
        assert_ne!(eh.hash, rydberg_constant().hash, "Eh is not Rinf");
        assert_ne!(eh.hash, newtonian_g().hash, "Eh is not G");
        assert_ne!(
            eh.provenance.source_hash,
            bohr_radius().provenance.source_hash,
            "Eh range is not the a0 range"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when Eh is added"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when Eh is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when Eh is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when Eh is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when Eh is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when Eh is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when Eh is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when Eh is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when Eh is added"
        );
        assert_eq!(
            eh.hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a"
        );
        assert!(eh.provenance.recheck().is_ok());
        assert!(
            10i128.checked_pow(31).is_some(),
            "Eh = 4.3597447222071e-18 is 43597447222071/10^31; that denominator fits i128"
        );
        assert!(lookup("E_h").is_none());
        assert!(lookup("hartree").is_none());
        assert!(lookup("Eh_eV").is_none());
        assert!(lookup("m_e").is_none());
    }

    #[test]
    fn codata_2018_electron_muon_mass_ratio_is_a_one_sigma_interval() {
        let r = electron_muon_mass_ratio();
        let scale = 10i128.pow(11);
        let lo = Ratio::new(483_633_158, scale);
        let hi = Ratio::new(483_633_180, scale);
        let centre = Ratio::new(483_633_169, scale);
        assert_eq!(r.name, "me_mmu");
        assert_eq!(r.unit, "1");
        assert_eq!(r.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(r.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            r.provenance.locator.section.as_deref(),
            Some("Electron, e-")
        );
        assert_eq!(
            r.provenance.locator.dataset_range.as_deref(),
            Some("me/mmu = 4.83633169(11)e-3")
        );
        assert_eq!(r.value, Interval::new(lo, hi));
        assert_ne!(r.value.lo, r.value.hi, "me_mmu is measured, not SI-exact");
        assert!(r.value.contains(Interval::point(centre)));
        assert!(!r
            .value
            .contains(Interval::point(Ratio::new(483_000_000, scale))));
        assert_eq!(r.hash, electron_muon_mass_ratio().hash);
        assert_eq!(
            r.hash,
            Constant::new(
                "me_mmu",
                codata_2018_electron_muon_interval(),
                "1",
                codata_2018_electron_muon_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(r.hash, proton_mass().hash, "me_mmu is not m_p");
        assert_ne!(r.hash, hartree_energy().hash, "me_mmu is not Eh");
        assert_ne!(
            r.hash,
            rydberg_energy_equivalent().hash,
            "me_mmu is not hcRinf"
        );
        assert_ne!(
            r.provenance.source_hash,
            proton_mass().provenance.source_hash,
            "me_mmu range is not the m_p range"
        );
        assert_eq!(
            hartree_energy().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a",
            "Eh hash must stay pinned when me_mmu is added"
        );
        assert_eq!(
            rydberg_energy_equivalent().hash.to_hex(),
            "0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2",
            "hcRinf hash must stay pinned when me_mmu is added"
        );
        assert_eq!(
            rydberg_frequency().hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969",
            "cRinf hash must stay pinned when me_mmu is added"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when me_mmu is added"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when me_mmu is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when me_mmu is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when me_mmu is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when me_mmu is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when me_mmu is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when me_mmu is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when me_mmu is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when me_mmu is added"
        );
        assert_eq!(
            r.hash.to_hex(),
            "d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2"
        );
        assert!(r.provenance.recheck().is_ok());
        assert!(lookup("me/m_mu").is_none());
        assert!(lookup("m_e/m_mu").is_none());
        assert!(lookup("sigma_e").is_none());
        assert!(lookup("m_e").is_none());
    }

    #[test]
    fn codata_2018_electron_proton_mass_ratio_is_a_one_sigma_interval() {
        let r = electron_proton_mass_ratio();
        let scale = 10i128.pow(15);
        let lo = Ratio::new(544_617_021_454, scale);
        let hi = Ratio::new(544_617_021_520, scale);
        let centre = Ratio::new(544_617_021_487, scale);
        assert_eq!(r.name, "me_mp");
        assert_eq!(r.unit, "1");
        assert_eq!(r.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(r.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            r.provenance.locator.section.as_deref(),
            Some("Electron, e-")
        );
        assert_eq!(
            r.provenance.locator.dataset_range.as_deref(),
            Some("me/mp = 5.44617021487(33)e-4")
        );
        assert_eq!(r.value, Interval::new(lo, hi));
        assert_ne!(r.value.lo, r.value.hi, "me_mp is measured, not SI-exact");
        assert!(r.value.contains(Interval::point(centre)));
        assert!(!r
            .value
            .contains(Interval::point(Ratio::new(544_000_000_000, scale))));
        assert_eq!(r.hash, electron_proton_mass_ratio().hash);
        assert_eq!(
            r.hash,
            Constant::new(
                "me_mp",
                codata_2018_electron_proton_interval(),
                "1",
                codata_2018_electron_proton_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(
            r.hash,
            electron_muon_mass_ratio().hash,
            "me_mp is not me_mmu"
        );
        assert_ne!(r.hash, proton_mass().hash, "me_mp is not m_p");
        assert_ne!(r.hash, hartree_energy().hash, "me_mp is not Eh");
        assert_ne!(
            r.provenance.source_hash,
            electron_muon_mass_ratio().provenance.source_hash,
            "me_mp range is not the me_mmu range"
        );
        assert_eq!(
            electron_muon_mass_ratio().hash.to_hex(),
            "d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2",
            "me_mmu hash must stay pinned when me_mp is added"
        );
        assert_eq!(
            hartree_energy().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a",
            "Eh hash must stay pinned when me_mp is added"
        );
        assert_eq!(
            rydberg_energy_equivalent().hash.to_hex(),
            "0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2",
            "hcRinf hash must stay pinned when me_mp is added"
        );
        assert_eq!(
            rydberg_frequency().hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969",
            "cRinf hash must stay pinned when me_mp is added"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when me_mp is added"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when me_mp is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when me_mp is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when me_mp is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when me_mp is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when me_mp is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when me_mp is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when me_mp is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when me_mp is added"
        );
        assert_eq!(
            r.hash.to_hex(),
            "b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b"
        );
        assert!(r.provenance.recheck().is_ok());
        assert!(lookup("me/m_p").is_none());
        assert!(lookup("m_e/m_p").is_none());
        assert!(lookup("sigma_e").is_none());
        assert!(lookup("m_e").is_none());
    }

    #[test]
    fn codata_2018_electron_neutron_mass_ratio_is_a_one_sigma_interval() {
        let r = electron_neutron_mass_ratio();
        let scale = 10i128.pow(14);
        let lo = Ratio::new(54_386_734_398, scale);
        let hi = Ratio::new(54_386_734_450, scale);
        let centre = Ratio::new(54_386_734_424, scale);
        assert_eq!(r.name, "me_mn");
        assert_eq!(r.unit, "1");
        assert_eq!(r.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(r.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            r.provenance.locator.section.as_deref(),
            Some("Electron, e-")
        );
        assert_eq!(
            r.provenance.locator.dataset_range.as_deref(),
            Some("me/mn = 5.4386734424(26)e-4")
        );
        assert_eq!(r.value, Interval::new(lo, hi));
        assert_ne!(r.value.lo, r.value.hi, "me_mn is measured, not SI-exact");
        assert!(r.value.contains(Interval::point(centre)));
        assert!(!r
            .value
            .contains(Interval::point(Ratio::new(54_000_000_000, scale))));
        assert_eq!(r.hash, electron_neutron_mass_ratio().hash);
        assert_eq!(
            r.hash,
            Constant::new(
                "me_mn",
                codata_2018_electron_neutron_interval(),
                "1",
                codata_2018_electron_neutron_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(
            r.hash,
            electron_proton_mass_ratio().hash,
            "me_mn is not me_mp"
        );
        assert_ne!(r.hash, proton_mass().hash, "me_mn is not m_p");
        assert_ne!(
            r.hash,
            electron_muon_mass_ratio().hash,
            "me_mn is not me_mmu"
        );
        assert_ne!(
            r.provenance.source_hash,
            electron_proton_mass_ratio().provenance.source_hash,
            "me_mn range is not the me_mp range"
        );
        assert_eq!(
            electron_proton_mass_ratio().hash.to_hex(),
            "b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b",
            "me_mp hash must stay pinned when me_mn is added"
        );
        assert_eq!(
            electron_muon_mass_ratio().hash.to_hex(),
            "d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2",
            "me_mmu hash must stay pinned when me_mn is added"
        );
        assert_eq!(
            hartree_energy().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a",
            "Eh hash must stay pinned when me_mn is added"
        );
        assert_eq!(
            rydberg_energy_equivalent().hash.to_hex(),
            "0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2",
            "hcRinf hash must stay pinned when me_mn is added"
        );
        assert_eq!(
            rydberg_frequency().hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969",
            "cRinf hash must stay pinned when me_mn is added"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when me_mn is added"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when me_mn is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when me_mn is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when me_mn is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when me_mn is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when me_mn is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when me_mn is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when me_mn is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when me_mn is added"
        );
        assert_eq!(
            r.hash.to_hex(),
            "e271d2015c7b39491daebf2a1d532ebe4c4dacf8228b3f7fc4d258be7b79ecba"
        );
        assert!(r.provenance.recheck().is_ok());
        assert!(lookup("me/m_n").is_none());
        assert!(lookup("sigma_e").is_none());
        assert!(lookup("m_e").is_none());
    }

    #[test]
    fn codata_2018_electron_deuteron_mass_ratio_is_a_one_sigma_interval() {
        let r = electron_deuteron_mass_ratio();
        let scale = 10i128.pow(16);
        let lo = Ratio::new(2_724_437_107_366, scale);
        let hi = Ratio::new(2_724_437_107_558, scale);
        let centre = Ratio::new(2_724_437_107_462, scale);
        assert_eq!(r.name, "me_md");
        assert_eq!(r.unit, "1");
        assert_eq!(r.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(r.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            r.provenance.locator.section.as_deref(),
            Some("Electron, e-")
        );
        assert_eq!(
            r.provenance.locator.dataset_range.as_deref(),
            Some("me/md = 2.724437107462(96)e-4")
        );
        assert_eq!(r.value, Interval::new(lo, hi));
        assert_ne!(r.value.lo, r.value.hi, "me_md is measured, not SI-exact");
        assert!(r.value.contains(Interval::point(centre)));
        assert!(!r
            .value
            .contains(Interval::point(Ratio::new(2_700_000_000_000, scale))));
        assert_eq!(r.hash, electron_deuteron_mass_ratio().hash);
        assert_eq!(
            r.hash,
            Constant::new(
                "me_md",
                codata_2018_electron_deuteron_interval(),
                "1",
                codata_2018_electron_deuteron_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(
            r.hash,
            electron_neutron_mass_ratio().hash,
            "me_md is not me_mn"
        );
        assert_ne!(
            r.hash,
            electron_proton_mass_ratio().hash,
            "me_md is not me_mp"
        );
        assert_ne!(r.hash, proton_mass().hash, "me_md is not m_p");
        assert_ne!(
            r.hash,
            electron_muon_mass_ratio().hash,
            "me_md is not me_mmu"
        );
        assert_ne!(
            r.provenance.source_hash,
            electron_neutron_mass_ratio().provenance.source_hash,
            "me_md range is not the me_mn range"
        );
        assert_eq!(
            electron_neutron_mass_ratio().hash.to_hex(),
            "e271d2015c7b39491daebf2a1d532ebe4c4dacf8228b3f7fc4d258be7b79ecba",
            "me_mn hash must stay pinned when me_md is added"
        );
        assert_eq!(
            electron_proton_mass_ratio().hash.to_hex(),
            "b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b",
            "me_mp hash must stay pinned when me_md is added"
        );
        assert_eq!(
            electron_muon_mass_ratio().hash.to_hex(),
            "d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2",
            "me_mmu hash must stay pinned when me_md is added"
        );
        assert_eq!(
            hartree_energy().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a",
            "Eh hash must stay pinned when me_md is added"
        );
        assert_eq!(
            rydberg_energy_equivalent().hash.to_hex(),
            "0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2",
            "hcRinf hash must stay pinned when me_md is added"
        );
        assert_eq!(
            rydberg_frequency().hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969",
            "cRinf hash must stay pinned when me_md is added"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when me_md is added"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when me_md is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when me_md is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when me_md is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when me_md is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when me_md is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when me_md is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when me_md is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when me_md is added"
        );
        assert_eq!(
            r.hash.to_hex(),
            "2aa5fe69f8cdd03f44e77b006a3b6ea90d48e1b8aec71275e184c4e529f0f76c"
        );
        assert!(r.provenance.recheck().is_ok());
        assert!(lookup("me/m_d").is_none());
        assert!(lookup("sigma_e").is_none());
        assert!(lookup("m_e").is_none());
    }

    #[test]
    fn codata_2018_electron_triton_mass_ratio_is_a_one_sigma_interval() {
        let r = electron_triton_mass_ratio();
        let scale = 10i128.pow(16);
        let lo = Ratio::new(1_819_200_062_161, scale);
        let hi = Ratio::new(1_819_200_062_341, scale);
        let centre = Ratio::new(1_819_200_062_251, scale);
        assert_eq!(r.name, "me_mt");
        assert_eq!(r.unit, "1");
        assert_eq!(r.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(r.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            r.provenance.locator.section.as_deref(),
            Some("Electron, e-")
        );
        assert_eq!(
            r.provenance.locator.dataset_range.as_deref(),
            Some("me/mt = 1.819200062251(90)e-4")
        );
        assert_eq!(r.value, Interval::new(lo, hi));
        assert_ne!(r.value.lo, r.value.hi, "me_mt is measured, not SI-exact");
        assert!(r.value.contains(Interval::point(centre)));
        assert!(!r
            .value
            .contains(Interval::point(Ratio::new(1_800_000_000_000, scale))));
        assert_eq!(r.hash, electron_triton_mass_ratio().hash);
        assert_eq!(
            r.hash,
            Constant::new(
                "me_mt",
                codata_2018_electron_triton_interval(),
                "1",
                codata_2018_electron_triton_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(
            r.hash,
            electron_deuteron_mass_ratio().hash,
            "me_mt is not me_md"
        );
        assert_ne!(
            r.hash,
            electron_neutron_mass_ratio().hash,
            "me_mt is not me_mn"
        );
        assert_ne!(
            r.hash,
            electron_proton_mass_ratio().hash,
            "me_mt is not me_mp"
        );
        assert_ne!(r.hash, proton_mass().hash, "me_mt is not m_p");
        assert_ne!(
            r.hash,
            electron_muon_mass_ratio().hash,
            "me_mt is not me_mmu"
        );
        assert_ne!(
            r.provenance.source_hash,
            electron_deuteron_mass_ratio().provenance.source_hash,
            "me_mt range is not the me_md range"
        );
        assert_eq!(
            electron_deuteron_mass_ratio().hash.to_hex(),
            "2aa5fe69f8cdd03f44e77b006a3b6ea90d48e1b8aec71275e184c4e529f0f76c",
            "me_md hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            electron_neutron_mass_ratio().hash.to_hex(),
            "e271d2015c7b39491daebf2a1d532ebe4c4dacf8228b3f7fc4d258be7b79ecba",
            "me_mn hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            electron_proton_mass_ratio().hash.to_hex(),
            "b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b",
            "me_mp hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            electron_muon_mass_ratio().hash.to_hex(),
            "d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2",
            "me_mmu hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            hartree_energy().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a",
            "Eh hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            rydberg_energy_equivalent().hash.to_hex(),
            "0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2",
            "hcRinf hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            rydberg_frequency().hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969",
            "cRinf hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when me_mt is added"
        );
        assert_eq!(
            r.hash.to_hex(),
            "2f8187d744269836cf0fbc123f8cb7d60107215e65be109daf2ae67c8116afd1"
        );
        assert!(r.provenance.recheck().is_ok());
        assert!(lookup("me/m_t").is_none());
        assert!(lookup("sigma_e").is_none());
        assert!(lookup("m_e").is_none());
    }

    #[test]
    fn codata_2018_electron_helion_mass_ratio_is_a_one_sigma_interval() {
        let r = electron_helion_mass_ratio();
        let scale = 10i128.pow(16);
        let lo = Ratio::new(1_819_543_074_494, scale);
        let hi = Ratio::new(1_819_543_074_652, scale);
        let centre = Ratio::new(1_819_543_074_573, scale);
        assert_eq!(r.name, "me_mh");
        assert_eq!(r.unit, "1");
        assert_eq!(r.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(r.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            r.provenance.locator.section.as_deref(),
            Some("Electron, e-")
        );
        assert_eq!(
            r.provenance.locator.dataset_range.as_deref(),
            Some("me/mh = 1.819543074573(79)e-4")
        );
        assert_eq!(r.value, Interval::new(lo, hi));
        assert_ne!(r.value.lo, r.value.hi, "me_mh is measured, not SI-exact");
        assert!(r.value.contains(Interval::point(centre)));
        assert!(!r
            .value
            .contains(Interval::point(Ratio::new(1_800_000_000_000, scale))));
        assert_eq!(r.hash, electron_helion_mass_ratio().hash);
        assert_eq!(
            r.hash,
            Constant::new(
                "me_mh",
                codata_2018_electron_helion_interval(),
                "1",
                codata_2018_electron_helion_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(
            r.hash,
            electron_triton_mass_ratio().hash,
            "me_mh is not me_mt"
        );
        assert_ne!(
            r.hash,
            electron_deuteron_mass_ratio().hash,
            "me_mh is not me_md"
        );
        assert_ne!(
            r.hash,
            electron_neutron_mass_ratio().hash,
            "me_mh is not me_mn"
        );
        assert_ne!(
            r.hash,
            electron_proton_mass_ratio().hash,
            "me_mh is not me_mp"
        );
        assert_ne!(r.hash, proton_mass().hash, "me_mh is not m_p");
        assert_ne!(
            r.hash,
            electron_muon_mass_ratio().hash,
            "me_mh is not me_mmu"
        );
        assert_ne!(
            r.provenance.source_hash,
            electron_triton_mass_ratio().provenance.source_hash,
            "me_mh range is not the me_mt range"
        );
        assert_eq!(
            electron_triton_mass_ratio().hash.to_hex(),
            "2f8187d744269836cf0fbc123f8cb7d60107215e65be109daf2ae67c8116afd1",
            "me_mt hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            electron_deuteron_mass_ratio().hash.to_hex(),
            "2aa5fe69f8cdd03f44e77b006a3b6ea90d48e1b8aec71275e184c4e529f0f76c",
            "me_md hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            electron_neutron_mass_ratio().hash.to_hex(),
            "e271d2015c7b39491daebf2a1d532ebe4c4dacf8228b3f7fc4d258be7b79ecba",
            "me_mn hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            electron_proton_mass_ratio().hash.to_hex(),
            "b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b",
            "me_mp hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            electron_muon_mass_ratio().hash.to_hex(),
            "d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2",
            "me_mmu hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            hartree_energy().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a",
            "Eh hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            rydberg_energy_equivalent().hash.to_hex(),
            "0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2",
            "hcRinf hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            rydberg_frequency().hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969",
            "cRinf hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when me_mh is added"
        );
        assert_eq!(
            r.hash.to_hex(),
            "0fb8f5fde9e76fcf2c24d73267f36cd3a5b40ca9f27f24e47d9807ec4206055e"
        );
        assert!(r.provenance.recheck().is_ok());
        assert!(lookup("me/m_h").is_none());
        assert!(lookup("sigma_e").is_none());
        assert!(lookup("m_e").is_none());
    }

    #[test]
    fn codata_2018_electron_alpha_mass_ratio_is_a_one_sigma_interval() {
        let r = electron_alpha_mass_ratio();
        let scale = 10i128.pow(16);
        let lo = Ratio::new(1_370_933_554_742, scale);
        let hi = Ratio::new(1_370_933_554_832, scale);
        let centre = Ratio::new(1_370_933_554_787, scale);
        assert_eq!(r.name, "me_malpha");
        assert_eq!(r.unit, "1");
        assert_eq!(r.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(r.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            r.provenance.locator.section.as_deref(),
            Some("Electron, e-")
        );
        assert_eq!(
            r.provenance.locator.dataset_range.as_deref(),
            Some("me/malpha = 1.370933554787(45)e-4")
        );
        assert_eq!(r.value, Interval::new(lo, hi));
        assert_ne!(
            r.value.lo, r.value.hi,
            "me_malpha is measured, not SI-exact"
        );
        assert!(r.value.contains(Interval::point(centre)));
        assert!(!r
            .value
            .contains(Interval::point(Ratio::new(1_300_000_000_000, scale))));
        assert_eq!(r.hash, electron_alpha_mass_ratio().hash);
        assert_eq!(
            r.hash,
            Constant::new(
                "me_malpha",
                codata_2018_electron_alpha_interval(),
                "1",
                codata_2018_electron_alpha_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(
            r.hash,
            electron_helion_mass_ratio().hash,
            "me_malpha is not me_mh"
        );
        assert_ne!(
            r.hash,
            electron_triton_mass_ratio().hash,
            "me_malpha is not me_mt"
        );
        assert_ne!(
            r.hash,
            electron_deuteron_mass_ratio().hash,
            "me_malpha is not me_md"
        );
        assert_ne!(
            r.hash,
            electron_neutron_mass_ratio().hash,
            "me_malpha is not me_mn"
        );
        assert_ne!(
            r.hash,
            electron_proton_mass_ratio().hash,
            "me_malpha is not me_mp"
        );
        assert_ne!(r.hash, proton_mass().hash, "me_malpha is not m_p");
        assert_ne!(
            r.hash,
            electron_muon_mass_ratio().hash,
            "me_malpha is not me_mmu"
        );
        assert_ne!(
            r.provenance.source_hash,
            electron_helion_mass_ratio().provenance.source_hash,
            "me_malpha range is not the me_mh range"
        );
        assert_eq!(
            electron_helion_mass_ratio().hash.to_hex(),
            "0fb8f5fde9e76fcf2c24d73267f36cd3a5b40ca9f27f24e47d9807ec4206055e",
            "me_mh hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            electron_triton_mass_ratio().hash.to_hex(),
            "2f8187d744269836cf0fbc123f8cb7d60107215e65be109daf2ae67c8116afd1",
            "me_mt hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            electron_deuteron_mass_ratio().hash.to_hex(),
            "2aa5fe69f8cdd03f44e77b006a3b6ea90d48e1b8aec71275e184c4e529f0f76c",
            "me_md hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            electron_neutron_mass_ratio().hash.to_hex(),
            "e271d2015c7b39491daebf2a1d532ebe4c4dacf8228b3f7fc4d258be7b79ecba",
            "me_mn hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            electron_proton_mass_ratio().hash.to_hex(),
            "b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b",
            "me_mp hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            electron_muon_mass_ratio().hash.to_hex(),
            "d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2",
            "me_mmu hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            hartree_energy().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a",
            "Eh hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            rydberg_energy_equivalent().hash.to_hex(),
            "0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2",
            "hcRinf hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            rydberg_frequency().hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969",
            "cRinf hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when me_malpha is added"
        );
        assert_eq!(
            r.hash.to_hex(),
            "3407529f38a47a2cf983c5418482d3d9bab5243e08258bbe88c06a2f42e0baa3"
        );
        assert!(r.provenance.recheck().is_ok());
        assert!(lookup("me/m_a").is_none());
        assert!(lookup("sigma_e").is_none());
        assert!(lookup("m_e").is_none());
    }

    #[test]
    fn codata_2018_electron_charge_to_mass_is_a_one_sigma_interval() {
        let r = electron_charge_to_mass();
        let lo = Ratio::int(-175_882_001_129);
        let hi = Ratio::int(-175_882_001_023);
        let centre = Ratio::int(-175_882_001_076);
        assert_eq!(r.name, "e_me");
        assert_eq!(r.unit, "C kg^{-1}");
        assert_eq!(r.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(r.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            r.provenance.locator.section.as_deref(),
            Some("Electron, e-")
        );
        assert_eq!(
            r.provenance.locator.dataset_range.as_deref(),
            Some("-e/me = -1.75882001076(53)e11")
        );
        assert_eq!(r.value, Interval::new(lo, hi));
        assert_ne!(r.value.lo, r.value.hi, "e_me is measured, not SI-exact");
        assert!(r.value.contains(Interval::point(centre)));
        assert!(!r.value.contains(Interval::point(Ratio::int(0))));
        assert!(
            r.value.hi < Ratio::int(0),
            "CODATA −e/me is the signed electron quotient, not +e/me"
        );
        assert_eq!(r.hash, electron_charge_to_mass().hash);
        assert_eq!(
            r.hash,
            Constant::new(
                "e_me",
                codata_2018_electron_charge_to_mass_interval(),
                "C kg^{-1}",
                codata_2018_electron_charge_to_mass_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(
            r.hash,
            electron_alpha_mass_ratio().hash,
            "e_me is not me_malpha"
        );
        assert_ne!(
            r.hash,
            electron_helion_mass_ratio().hash,
            "e_me is not me_mh"
        );
        assert_ne!(r.hash, proton_mass().hash, "e_me is not m_p");
        assert_ne!(
            r.hash,
            elementary_charge().hash,
            "e_me is not the SI-exact elementary charge"
        );
        assert_ne!(
            r.provenance.source_hash,
            electron_alpha_mass_ratio().provenance.source_hash,
            "e_me range is not the me_malpha range"
        );
        assert_eq!(
            electron_alpha_mass_ratio().hash.to_hex(),
            "3407529f38a47a2cf983c5418482d3d9bab5243e08258bbe88c06a2f42e0baa3",
            "me_malpha hash must stay pinned when e_me is added"
        );
        assert_eq!(
            electron_helion_mass_ratio().hash.to_hex(),
            "0fb8f5fde9e76fcf2c24d73267f36cd3a5b40ca9f27f24e47d9807ec4206055e",
            "me_mh hash must stay pinned when e_me is added"
        );
        assert_eq!(
            electron_triton_mass_ratio().hash.to_hex(),
            "2f8187d744269836cf0fbc123f8cb7d60107215e65be109daf2ae67c8116afd1",
            "me_mt hash must stay pinned when e_me is added"
        );
        assert_eq!(
            electron_deuteron_mass_ratio().hash.to_hex(),
            "2aa5fe69f8cdd03f44e77b006a3b6ea90d48e1b8aec71275e184c4e529f0f76c",
            "me_md hash must stay pinned when e_me is added"
        );
        assert_eq!(
            electron_neutron_mass_ratio().hash.to_hex(),
            "e271d2015c7b39491daebf2a1d532ebe4c4dacf8228b3f7fc4d258be7b79ecba",
            "me_mn hash must stay pinned when e_me is added"
        );
        assert_eq!(
            electron_proton_mass_ratio().hash.to_hex(),
            "b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b",
            "me_mp hash must stay pinned when e_me is added"
        );
        assert_eq!(
            electron_muon_mass_ratio().hash.to_hex(),
            "d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2",
            "me_mmu hash must stay pinned when e_me is added"
        );
        assert_eq!(
            hartree_energy().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a",
            "Eh hash must stay pinned when e_me is added"
        );
        assert_eq!(
            rydberg_energy_equivalent().hash.to_hex(),
            "0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2",
            "hcRinf hash must stay pinned when e_me is added"
        );
        assert_eq!(
            rydberg_frequency().hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969",
            "cRinf hash must stay pinned when e_me is added"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when e_me is added"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when e_me is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when e_me is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when e_me is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when e_me is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when e_me is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when e_me is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when e_me is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when e_me is added"
        );
        assert_eq!(
            r.hash.to_hex(),
            "bfe24e8de43e90dbc8a28472f99ed206f07566fa1a4fa6c6d14356adf4e89b22"
        );
        assert!(r.provenance.recheck().is_ok());
        assert!(lookup("-e/me").is_none());
        assert!(lookup("e/me").is_none());
        assert!(lookup("sigma_e").is_none());
        assert!(lookup("m_e").is_none());
    }

    #[test]
    fn codata_2018_electron_molar_mass_is_a_one_sigma_interval() {
        let r = electron_molar_mass();
        let scale = 10i128.pow(17);
        let lo = Ratio::new(54_857_990_871, scale);
        let hi = Ratio::new(54_857_990_905, scale);
        let centre = Ratio::new(54_857_990_888, scale);
        assert_eq!(r.name, "M_e");
        assert_eq!(r.unit, "kg mol^{-1}");
        assert_eq!(r.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(r.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            r.provenance.locator.section.as_deref(),
            Some("Electron, e-")
        );
        assert_eq!(
            r.provenance.locator.dataset_range.as_deref(),
            Some("Me = 5.4857990888(17)e-7")
        );
        assert_eq!(r.value, Interval::new(lo, hi));
        assert_ne!(r.value.lo, r.value.hi, "M_e is measured, not SI-exact");
        assert!(r.value.contains(Interval::point(centre)));
        assert!(!r
            .value
            .contains(Interval::point(Ratio::new(54_000_000_000, scale))));
        assert_eq!(r.hash, electron_molar_mass().hash);
        assert_eq!(
            r.hash,
            Constant::new(
                "M_e",
                codata_2018_electron_molar_mass_interval(),
                "kg mol^{-1}",
                codata_2018_electron_molar_mass_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(r.hash, electron_charge_to_mass().hash, "M_e is not e_me");
        assert_ne!(
            r.hash,
            electron_alpha_mass_ratio().hash,
            "M_e is not me_malpha"
        );
        assert_ne!(r.hash, proton_mass().hash, "M_e is not m_p");
        assert_ne!(
            r.provenance.source_hash,
            electron_charge_to_mass().provenance.source_hash,
            "M_e range is not the e_me range"
        );
        assert_eq!(
            electron_charge_to_mass().hash.to_hex(),
            "bfe24e8de43e90dbc8a28472f99ed206f07566fa1a4fa6c6d14356adf4e89b22",
            "e_me hash must stay pinned when M_e is added"
        );
        assert_eq!(
            electron_alpha_mass_ratio().hash.to_hex(),
            "3407529f38a47a2cf983c5418482d3d9bab5243e08258bbe88c06a2f42e0baa3",
            "me_malpha hash must stay pinned when M_e is added"
        );
        assert_eq!(
            electron_helion_mass_ratio().hash.to_hex(),
            "0fb8f5fde9e76fcf2c24d73267f36cd3a5b40ca9f27f24e47d9807ec4206055e",
            "me_mh hash must stay pinned when M_e is added"
        );
        assert_eq!(
            electron_triton_mass_ratio().hash.to_hex(),
            "2f8187d744269836cf0fbc123f8cb7d60107215e65be109daf2ae67c8116afd1",
            "me_mt hash must stay pinned when M_e is added"
        );
        assert_eq!(
            electron_deuteron_mass_ratio().hash.to_hex(),
            "2aa5fe69f8cdd03f44e77b006a3b6ea90d48e1b8aec71275e184c4e529f0f76c",
            "me_md hash must stay pinned when M_e is added"
        );
        assert_eq!(
            electron_neutron_mass_ratio().hash.to_hex(),
            "e271d2015c7b39491daebf2a1d532ebe4c4dacf8228b3f7fc4d258be7b79ecba",
            "me_mn hash must stay pinned when M_e is added"
        );
        assert_eq!(
            electron_proton_mass_ratio().hash.to_hex(),
            "b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b",
            "me_mp hash must stay pinned when M_e is added"
        );
        assert_eq!(
            electron_muon_mass_ratio().hash.to_hex(),
            "d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2",
            "me_mmu hash must stay pinned when M_e is added"
        );
        assert_eq!(
            hartree_energy().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a",
            "Eh hash must stay pinned when M_e is added"
        );
        assert_eq!(
            rydberg_energy_equivalent().hash.to_hex(),
            "0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2",
            "hcRinf hash must stay pinned when M_e is added"
        );
        assert_eq!(
            rydberg_frequency().hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969",
            "cRinf hash must stay pinned when M_e is added"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when M_e is added"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when M_e is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when M_e is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when M_e is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when M_e is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when M_e is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when M_e is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when M_e is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when M_e is added"
        );
        assert_eq!(
            r.hash.to_hex(),
            "0a8b3285a4969854567b59db2ebf9449268df86ffdbb461e3b9c1db0955eb804"
        );
        assert!(r.provenance.recheck().is_ok());
        assert!(lookup("Me").is_none());
        assert!(lookup("molar_e").is_none());
        assert!(lookup("sigma_e").is_none());
        assert!(lookup("m_e").is_none());
    }

    #[test]
    fn codata_2018_reduced_compton_wavelength_is_a_one_sigma_interval() {
        let r = reduced_compton_wavelength();
        let scale = 10i128.pow(23);
        let lo = Ratio::new(38_615_926_784, scale);
        let hi = Ratio::new(38_615_926_808, scale);
        let centre = Ratio::new(38_615_926_796, scale);
        assert_eq!(r.name, "lambdabar_C");
        assert_eq!(r.unit, "m");
        assert_eq!(r.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(r.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            r.provenance.locator.section.as_deref(),
            Some("Electron, e-")
        );
        assert_eq!(
            r.provenance.locator.dataset_range.as_deref(),
            Some("lambdabar_C = 3.8615926796(12)e-13")
        );
        assert_eq!(r.value, Interval::new(lo, hi));
        assert_ne!(
            r.value.lo, r.value.hi,
            "lambdabar_C is measured, not SI-exact"
        );
        assert!(r.value.contains(Interval::point(centre)));
        assert!(!r
            .value
            .contains(Interval::point(Ratio::new(38_000_000_000, scale))));
        assert_eq!(r.hash, reduced_compton_wavelength().hash);
        assert_eq!(
            r.hash,
            Constant::new(
                "lambdabar_C",
                codata_2018_reduced_compton_interval(),
                "m",
                codata_2018_reduced_compton_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(r.hash, electron_molar_mass().hash, "lambdabar_C is not M_e");
        assert_ne!(
            r.hash,
            electron_charge_to_mass().hash,
            "lambdabar_C is not e_me"
        );
        assert_ne!(r.hash, bohr_radius().hash, "lambdabar_C is not a0");
        assert_ne!(r.hash, proton_mass().hash, "lambdabar_C is not m_p");
        assert_ne!(
            r.provenance.source_hash,
            electron_molar_mass().provenance.source_hash,
            "lambdabar_C range is not the M_e range"
        );
        assert_eq!(
            electron_molar_mass().hash.to_hex(),
            "0a8b3285a4969854567b59db2ebf9449268df86ffdbb461e3b9c1db0955eb804",
            "M_e hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            electron_charge_to_mass().hash.to_hex(),
            "bfe24e8de43e90dbc8a28472f99ed206f07566fa1a4fa6c6d14356adf4e89b22",
            "e_me hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            electron_alpha_mass_ratio().hash.to_hex(),
            "3407529f38a47a2cf983c5418482d3d9bab5243e08258bbe88c06a2f42e0baa3",
            "me_malpha hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            electron_helion_mass_ratio().hash.to_hex(),
            "0fb8f5fde9e76fcf2c24d73267f36cd3a5b40ca9f27f24e47d9807ec4206055e",
            "me_mh hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            electron_triton_mass_ratio().hash.to_hex(),
            "2f8187d744269836cf0fbc123f8cb7d60107215e65be109daf2ae67c8116afd1",
            "me_mt hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            electron_deuteron_mass_ratio().hash.to_hex(),
            "2aa5fe69f8cdd03f44e77b006a3b6ea90d48e1b8aec71275e184c4e529f0f76c",
            "me_md hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            electron_neutron_mass_ratio().hash.to_hex(),
            "e271d2015c7b39491daebf2a1d532ebe4c4dacf8228b3f7fc4d258be7b79ecba",
            "me_mn hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            electron_proton_mass_ratio().hash.to_hex(),
            "b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b",
            "me_mp hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            electron_muon_mass_ratio().hash.to_hex(),
            "d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2",
            "me_mmu hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            hartree_energy().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a",
            "Eh hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            rydberg_energy_equivalent().hash.to_hex(),
            "0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2",
            "hcRinf hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            rydberg_frequency().hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969",
            "cRinf hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when lambdabar_C is added"
        );
        assert_eq!(
            r.hash.to_hex(),
            "0ed48571f065fc19458ea3c8fd493fd00de18a7d196669f81bb93c50779bc625"
        );
        assert!(r.provenance.recheck().is_ok());
        assert!(lookup("sigma_e").is_none());
        assert!(lookup("lambdaC").is_none());
        assert!(lookup("rc").is_none());
        assert!(lookup("m_e").is_none());
    }

    #[test]
    fn codata_2018_compton_wavelength_is_a_one_sigma_interval() {
        let r = compton_wavelength();
        let scale = 10i128.pow(23);
        let lo = Ratio::new(242_631_023_794, scale);
        let hi = Ratio::new(242_631_023_940, scale);
        let centre = Ratio::new(242_631_023_867, scale);
        assert_eq!(r.name, "lambda_C");
        assert_eq!(r.unit, "m");
        assert_eq!(r.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(r.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            r.provenance.locator.section.as_deref(),
            Some("Electron, e-")
        );
        assert_eq!(
            r.provenance.locator.dataset_range.as_deref(),
            Some("lambda_C = 2.42631023867(73)e-12")
        );
        assert_eq!(r.value, Interval::new(lo, hi));
        assert_ne!(r.value.lo, r.value.hi, "lambda_C is measured, not SI-exact");
        assert!(r.value.contains(Interval::point(centre)));
        assert!(!r
            .value
            .contains(Interval::point(Ratio::new(242_000_000_000, scale))));
        assert_eq!(r.hash, compton_wavelength().hash);
        assert_eq!(
            r.hash,
            Constant::new(
                "lambda_C",
                codata_2018_compton_interval(),
                "m",
                codata_2018_compton_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(
            r.hash,
            reduced_compton_wavelength().hash,
            "lambda_C is not lambdabar_C"
        );
        assert_ne!(r.hash, electron_molar_mass().hash, "lambda_C is not M_e");
        assert_ne!(r.hash, bohr_radius().hash, "lambda_C is not a0");
        assert_ne!(r.hash, proton_mass().hash, "lambda_C is not m_p");
        assert_ne!(
            r.provenance.source_hash,
            reduced_compton_wavelength().provenance.source_hash,
            "lambda_C range is not the lambdabar_C range"
        );
        assert_eq!(
            reduced_compton_wavelength().hash.to_hex(),
            "0ed48571f065fc19458ea3c8fd493fd00de18a7d196669f81bb93c50779bc625",
            "lambdabar_C hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            electron_molar_mass().hash.to_hex(),
            "0a8b3285a4969854567b59db2ebf9449268df86ffdbb461e3b9c1db0955eb804",
            "M_e hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            electron_charge_to_mass().hash.to_hex(),
            "bfe24e8de43e90dbc8a28472f99ed206f07566fa1a4fa6c6d14356adf4e89b22",
            "e_me hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            electron_alpha_mass_ratio().hash.to_hex(),
            "3407529f38a47a2cf983c5418482d3d9bab5243e08258bbe88c06a2f42e0baa3",
            "me_malpha hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            electron_helion_mass_ratio().hash.to_hex(),
            "0fb8f5fde9e76fcf2c24d73267f36cd3a5b40ca9f27f24e47d9807ec4206055e",
            "me_mh hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            electron_triton_mass_ratio().hash.to_hex(),
            "2f8187d744269836cf0fbc123f8cb7d60107215e65be109daf2ae67c8116afd1",
            "me_mt hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            electron_deuteron_mass_ratio().hash.to_hex(),
            "2aa5fe69f8cdd03f44e77b006a3b6ea90d48e1b8aec71275e184c4e529f0f76c",
            "me_md hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            electron_neutron_mass_ratio().hash.to_hex(),
            "e271d2015c7b39491daebf2a1d532ebe4c4dacf8228b3f7fc4d258be7b79ecba",
            "me_mn hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            electron_proton_mass_ratio().hash.to_hex(),
            "b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b",
            "me_mp hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            electron_muon_mass_ratio().hash.to_hex(),
            "d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2",
            "me_mmu hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            hartree_energy().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a",
            "Eh hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            rydberg_energy_equivalent().hash.to_hex(),
            "0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2",
            "hcRinf hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            rydberg_frequency().hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969",
            "cRinf hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when lambda_C is added"
        );
        assert_eq!(
            r.hash.to_hex(),
            "6280f2b2f61adf3ae0fa3e65f3b12cfb4982f6601027d98552f541246198c3d8"
        );
        assert!(r.provenance.recheck().is_ok());
        assert!(lookup("lambdaC").is_none());
        assert!(lookup("sigma_e").is_none());
        assert!(lookup("rc").is_none());
        assert!(lookup("m_e").is_none());
    }

    #[test]
    fn codata_2018_classical_electron_radius_is_a_one_sigma_interval() {
        let r = classical_electron_radius();
        let scale = 10i128.pow(25);
        let lo = Ratio::new(28_179_403_249, scale);
        let hi = Ratio::new(28_179_403_275, scale);
        let centre = Ratio::new(28_179_403_262, scale);
        assert_eq!(r.name, "re");
        assert_eq!(r.unit, "m");
        assert_eq!(r.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(r.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(
            r.provenance.locator.section.as_deref(),
            Some("Electron, e-")
        );
        assert_eq!(
            r.provenance.locator.dataset_range.as_deref(),
            Some("re = 2.8179403262(13)e-15")
        );
        assert_eq!(r.value, Interval::new(lo, hi));
        assert_ne!(r.value.lo, r.value.hi, "re is measured, not SI-exact");
        assert!(r.value.contains(Interval::point(centre)));
        assert!(!r
            .value
            .contains(Interval::point(Ratio::new(28_000_000_000, scale))));
        assert_eq!(r.hash, classical_electron_radius().hash);
        assert_eq!(
            r.hash,
            Constant::new(
                "re",
                codata_2018_classical_radius_interval(),
                "m",
                codata_2018_classical_radius_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(r.hash, compton_wavelength().hash, "re is not lambda_C");
        assert_ne!(
            r.hash,
            reduced_compton_wavelength().hash,
            "re is not lambdabar_C"
        );
        assert_ne!(r.hash, bohr_radius().hash, "re is not a0");
        assert_ne!(r.hash, proton_mass().hash, "re is not m_p");
        assert_ne!(
            r.provenance.source_hash,
            compton_wavelength().provenance.source_hash,
            "re range is not the lambda_C range"
        );
        assert_eq!(
            compton_wavelength().hash.to_hex(),
            "6280f2b2f61adf3ae0fa3e65f3b12cfb4982f6601027d98552f541246198c3d8",
            "lambda_C hash must stay pinned when re is added"
        );
        assert_eq!(
            reduced_compton_wavelength().hash.to_hex(),
            "0ed48571f065fc19458ea3c8fd493fd00de18a7d196669f81bb93c50779bc625",
            "lambdabar_C hash must stay pinned when re is added"
        );
        assert_eq!(
            electron_molar_mass().hash.to_hex(),
            "0a8b3285a4969854567b59db2ebf9449268df86ffdbb461e3b9c1db0955eb804",
            "M_e hash must stay pinned when re is added"
        );
        assert_eq!(
            electron_charge_to_mass().hash.to_hex(),
            "bfe24e8de43e90dbc8a28472f99ed206f07566fa1a4fa6c6d14356adf4e89b22",
            "e_me hash must stay pinned when re is added"
        );
        assert_eq!(
            electron_alpha_mass_ratio().hash.to_hex(),
            "3407529f38a47a2cf983c5418482d3d9bab5243e08258bbe88c06a2f42e0baa3",
            "me_malpha hash must stay pinned when re is added"
        );
        assert_eq!(
            electron_helion_mass_ratio().hash.to_hex(),
            "0fb8f5fde9e76fcf2c24d73267f36cd3a5b40ca9f27f24e47d9807ec4206055e",
            "me_mh hash must stay pinned when re is added"
        );
        assert_eq!(
            electron_triton_mass_ratio().hash.to_hex(),
            "2f8187d744269836cf0fbc123f8cb7d60107215e65be109daf2ae67c8116afd1",
            "me_mt hash must stay pinned when re is added"
        );
        assert_eq!(
            electron_deuteron_mass_ratio().hash.to_hex(),
            "2aa5fe69f8cdd03f44e77b006a3b6ea90d48e1b8aec71275e184c4e529f0f76c",
            "me_md hash must stay pinned when re is added"
        );
        assert_eq!(
            electron_neutron_mass_ratio().hash.to_hex(),
            "e271d2015c7b39491daebf2a1d532ebe4c4dacf8228b3f7fc4d258be7b79ecba",
            "me_mn hash must stay pinned when re is added"
        );
        assert_eq!(
            electron_proton_mass_ratio().hash.to_hex(),
            "b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b",
            "me_mp hash must stay pinned when re is added"
        );
        assert_eq!(
            electron_muon_mass_ratio().hash.to_hex(),
            "d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2",
            "me_mmu hash must stay pinned when re is added"
        );
        assert_eq!(
            hartree_energy().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a",
            "Eh hash must stay pinned when re is added"
        );
        assert_eq!(
            rydberg_energy_equivalent().hash.to_hex(),
            "0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2",
            "hcRinf hash must stay pinned when re is added"
        );
        assert_eq!(
            rydberg_frequency().hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969",
            "cRinf hash must stay pinned when re is added"
        );
        assert_eq!(
            rydberg_constant().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090",
            "Rinf hash must stay pinned when re is added"
        );
        assert_eq!(
            bohr_radius().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29",
            "a0 hash must stay pinned when re is added"
        );
        assert_eq!(
            inverse_fine_structure_constant().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482",
            "inv_alpha hash must stay pinned when re is added"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when re is added"
        );
        assert_eq!(
            vacuum_impedance().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316",
            "Z0 hash must stay pinned when re is added"
        );
        assert_eq!(
            vacuum_permittivity().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80",
            "epsilon0 hash must stay pinned when re is added"
        );
        assert_eq!(
            vacuum_permeability().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7",
            "mu0 hash must stay pinned when re is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when re is added"
        );
        assert_eq!(
            proton_mass().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8",
            "m_p hash must stay pinned when re is added"
        );
        assert_eq!(
            r.hash.to_hex(),
            "1b8dfc7aa2f90183fd50dab61cf3361f57c3c906e6a221ffa3b2ef17302a38d4"
        );
        assert!(r.provenance.recheck().is_ok());
        assert!(lookup("r_e").is_none());
        assert!(lookup("rc").is_none());
        assert!(lookup("sigma_e").is_none());
        assert!(lookup("m_e").is_none());
    }

    #[test]
    fn codata_2018_proton_mass_is_a_one_sigma_interval() {
        let mp = proton_mass();
        let scale = 10i128.pow(38);
        let lo = Ratio::new(167_262_192_318, scale);
        let hi = Ratio::new(167_262_192_420, scale);
        let centre = Ratio::new(167_262_192_369, scale);
        assert_eq!(mp.name, "m_p");
        assert_eq!(mp.unit, "kg");
        assert_eq!(mp.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(mp.provenance.locator.table.as_deref(), Some("XXXI"));
        assert_eq!(mp.provenance.locator.section.as_deref(), Some("Proton, p"));
        assert_eq!(
            mp.provenance.locator.dataset_range.as_deref(),
            Some("mp = 1.67262192369(51)e-27")
        );
        assert_eq!(mp.value, Interval::new(lo, hi));
        assert_ne!(mp.value.lo, mp.value.hi, "m_p is measured, not SI-exact");
        assert!(mp.value.contains(Interval::point(centre)));
        assert!(!mp
            .value
            .contains(Interval::point(Ratio::new(167_262_000_000, scale))));
        assert_eq!(mp.hash, proton_mass().hash);
        assert_eq!(
            mp.hash,
            Constant::new(
                "m_p",
                codata_2018_proton_mass_interval(),
                "kg",
                codata_2018_proton_mass_source(),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(mp.hash, fine_structure_constant().hash, "m_p is not alpha");
        assert_ne!(mp.hash, newtonian_g().hash, "m_p is not G");
        assert_ne!(
            mp.provenance.source_hash,
            fine_structure_constant().provenance.source_hash,
            "m_p range is not the alpha range"
        );
        assert_eq!(
            fine_structure_constant().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38",
            "alpha hash must stay pinned when m_p is added"
        );
        assert_eq!(
            newtonian_g().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92",
            "G hash must stay pinned when m_p is added"
        );
        assert_eq!(
            mp.hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8"
        );
        assert!(mp.provenance.recheck().is_ok());
        assert!(
            10i128.checked_pow(38).is_some(),
            "m_p = 1.67262192369e-27 is 167262192369/10^38; that denominator fits i128"
        );
        assert!(
            10i128.checked_pow(42).is_none(),
            "m_e = 9.1093837015e-31 is 91093837015/10^42; that denominator overflows i128"
        );
        assert!(lookup("m_e").is_none());
        assert!(lookup("mp").is_none());
        assert!(lookup("proton-mass").is_none());
        assert!(lookup("electron-mass").is_none());
    }

    #[test]
    fn iau2012_au_is_an_exact_ratio() {
        let au = astronomical_unit();
        assert_eq!(au.name, "au");
        assert_eq!(au.unit, "m");
        assert_eq!(au.value, Ratio::int(149_597_870_700));
        assert_eq!(au.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(au.provenance.locator.table.as_deref(), Some("8"));
        assert_eq!(
            au.provenance.locator.dataset_range.as_deref(),
            Some("1 au = 149 597 870 700 m")
        );
        assert_eq!(
            au.provenance.locator.section.as_deref(),
            Some("Non-SI units accepted for use with the SI")
        );
        assert_eq!(au.hash, astronomical_unit().hash);
        assert_eq!(
            au.hash,
            Constant::new(
                "au",
                Ratio::int(149_597_870_700),
                "m",
                si_brochure_table_8("1 au = 149 597 870 700 m"),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(au.hash, speed_of_light().hash);
        assert_ne!(au.hash, newtonian_g().hash);
        assert_ne!(au.hash, planck_h().hash);
        assert_eq!(
            au.hash.to_hex(),
            "d3441603d75b565016c25cc955783fbb76b4050ee22befcef0c0e3896e873a0b"
        );
        assert!(au.provenance.recheck().is_ok());
        assert_ne!(
            au.provenance.source_hash,
            si_brochure().source_hash,
            "table 8 is not table 1"
        );
    }

    #[test]
    fn si2019_electronvolt_is_an_exact_ratio() {
        let ev = electron_volt();
        let value = Ratio::new(1_602_176_634, 10i128.pow(28));
        assert_eq!(ev.name, "eV");
        assert_eq!(ev.unit, "J");
        assert_eq!(ev.value, value);
        assert_eq!(ev.value, elementary_charge().value);
        assert_eq!(ev.release, ConstantRelease::Si2019Codata2018);
        assert_eq!(ev.provenance.locator.table.as_deref(), Some("8"));
        assert_eq!(
            ev.provenance.locator.dataset_range.as_deref(),
            Some("1 eV = 1.602176634e-19 J")
        );
        assert_eq!(ev.hash, electron_volt().hash);
        assert_eq!(
            ev.hash,
            Constant::new(
                "eV",
                value,
                "J",
                si_brochure_table_8("1 eV = 1.602176634e-19 J"),
                ConstantRelease::Si2019Codata2018,
            )
            .hash
        );
        assert_ne!(ev.hash, elementary_charge().hash, "eV is not e");
        assert_ne!(ev.hash, astronomical_unit().hash);
        assert_ne!(
            ev.provenance.source_hash,
            astronomical_unit().provenance.source_hash,
            "eV range is not the au range"
        );
        assert_eq!(
            ev.hash.to_hex(),
            "d5514de9cbef3f6990067899529d34f20b4349ca3b20ba18c9a5932c8c6b6c0f"
        );
        assert!(ev.provenance.recheck().is_ok());
    }

    #[test]
    fn iau2015_solar_gm_is_an_exact_ratio() {
        let gm = solar_gm();
        let value = Ratio::int(13_271_244i128 * 10i128.pow(13));
        assert_eq!(gm.name, "GM_sun");
        assert_eq!(gm.unit, "m^3 s^{-2}");
        assert_eq!(gm.value, value);
        assert_eq!(gm.release, ConstantRelease::Iau2015);
        assert_eq!(gm.provenance.locator.table.as_deref(), Some("1"));
        assert_eq!(
            gm.provenance.locator.dataset_range.as_deref(),
            Some("(GM)_sun^N = 1.3271244e20")
        );
        assert_eq!(gm.hash, solar_gm().hash);
        assert_eq!(
            gm.hash,
            Constant::new(
                "GM_sun",
                value,
                "m^3 s^{-2}",
                iau2015_b3_table_1("(GM)_sun^N = 1.3271244e20"),
                ConstantRelease::Iau2015,
            )
            .hash
        );
        assert_ne!(gm.hash, astronomical_unit().hash);
        assert_ne!(gm.hash, newtonian_g().hash);
        assert_ne!(gm.hash, speed_of_light().hash);
        assert_eq!(
            gm.hash.to_hex(),
            "636001001c4ed9cd5e6661241e5ad5e5db09c8419a3fe79790143162b7af3a58"
        );
        assert!(gm.provenance.recheck().is_ok());
        assert_ne!(
            gm.release,
            ConstantRelease::Si2019Codata2018,
            "GM_sun is IAU 2015, not an SI defining constant"
        );
    }

    #[test]
    fn iau2015_solar_radius_is_an_exact_ratio() {
        let r = solar_radius();
        assert_eq!(r.name, "R_sun");
        assert_eq!(r.unit, "m");
        assert_eq!(r.value, Ratio::int(695_700_000));
        assert_eq!(r.release, ConstantRelease::Iau2015);
        assert_eq!(r.provenance.locator.table.as_deref(), Some("1"));
        assert_eq!(
            r.provenance.locator.dataset_range.as_deref(),
            Some("R_sun^N = 6.957e8")
        );
        assert_eq!(r.hash, solar_radius().hash);
        assert_eq!(
            r.hash,
            Constant::new(
                "R_sun",
                Ratio::int(695_700_000),
                "m",
                iau2015_b3_table_1("R_sun^N = 6.957e8"),
                ConstantRelease::Iau2015,
            )
            .hash
        );
        assert_ne!(r.hash, solar_gm().hash);
        assert_ne!(r.hash, astronomical_unit().hash);
        assert_ne!(
            r.provenance.source_hash,
            solar_gm().provenance.source_hash,
            "R_sun range is not the GM_sun range"
        );
        assert_eq!(
            r.hash.to_hex(),
            "cb7f91f2d0663d2d8ff8b0e3009f6e0772a126220d04ed658fc793db7e5cc6b4"
        );
        assert!(r.provenance.recheck().is_ok());
    }

    #[test]
    fn iau2015_solar_luminosity_is_an_exact_ratio() {
        let l = solar_luminosity();
        let value = Ratio::int(3_828i128 * 10i128.pow(23));
        assert_eq!(l.name, "L_sun");
        assert_eq!(l.unit, "W");
        assert_eq!(l.value, value);
        assert_eq!(l.release, ConstantRelease::Iau2015);
        assert_eq!(l.provenance.locator.table.as_deref(), Some("1"));
        assert_eq!(
            l.provenance.locator.dataset_range.as_deref(),
            Some("L_sun^N = 3.828e26")
        );
        assert_eq!(l.hash, solar_luminosity().hash);
        assert_eq!(
            l.hash,
            Constant::new(
                "L_sun",
                value,
                "W",
                iau2015_b3_table_1("L_sun^N = 3.828e26"),
                ConstantRelease::Iau2015,
            )
            .hash
        );
        assert_ne!(l.hash, solar_gm().hash);
        assert_ne!(l.hash, solar_radius().hash);
        assert_ne!(
            l.provenance.source_hash,
            solar_radius().provenance.source_hash,
            "L_sun range is not the R_sun range"
        );
        assert_eq!(
            l.hash.to_hex(),
            "444f85fba501ddec8fb08ba403c1b869cc78a2284df5466a56a617043807bbc4"
        );
        assert!(l.provenance.recheck().is_ok());
    }

    #[test]
    fn lookup_rebuilds_the_live_ledger_and_rejects_unknown_names() {
        assert_eq!(LEDGER.len(), 36);
        for name in LEDGER {
            let live = lookup(name).expect(name);
            let again = lookup(name).expect(name);
            assert_eq!(live.hash, again.hash, "{name}");
            assert!(live.source.recheck().is_ok(), "{name}");
        }
        assert_eq!(
            lookup("c").unwrap().hash.to_hex(),
            "691eb73ea444f6d10fb223b999a1b37c0b67da92d51e43ca8bd8a6561785a3c1"
        );
        assert_eq!(
            lookup("h").unwrap().hash.to_hex(),
            "50a96a8715769547a90cba69b0775d8892d79f2fa32465ad13a6d73b2d111eef"
        );
        assert_eq!(
            lookup("G").unwrap().hash.to_hex(),
            "ebbfc13ea8fba734da50b679d9eaf236638b244cdcc350c0b14cdd6696850e92"
        );
        assert_eq!(lookup("G").unwrap().kind, "interval");
        assert_eq!(lookup("mu0").unwrap().kind, "interval");
        assert_eq!(
            lookup("mu0").unwrap().hash.to_hex(),
            "fa1264a6ce514520c9c2d9131fee2c71cacd4ce5fe615ea4dd424fd23de35cd7"
        );
        assert_eq!(lookup("epsilon0").unwrap().kind, "interval");
        assert_eq!(
            lookup("epsilon0").unwrap().hash.to_hex(),
            "fadaf2a47a8161ba2727a4c2ff6b842f7c9e6add2edd67cd5496a7a753f22d80"
        );
        assert_eq!(lookup("Z0").unwrap().kind, "interval");
        assert_eq!(
            lookup("Z0").unwrap().hash.to_hex(),
            "6f72c1c5833dc722ac6fb5223f982879499ff412157c6e6c9851d77088991316"
        );
        assert_eq!(lookup("alpha").unwrap().kind, "interval");
        assert_eq!(
            lookup("alpha").unwrap().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38"
        );
        assert_eq!(lookup("inv_alpha").unwrap().kind, "interval");
        assert_eq!(
            lookup("inv_alpha").unwrap().hash.to_hex(),
            "4b7050d77da09c5322877eaf83e94ebba7b84c99bad8ba3713b0e5fe91128482"
        );
        assert_eq!(lookup("cRinf").unwrap().kind, "interval");
        assert_eq!(
            lookup("cRinf").unwrap().hash.to_hex(),
            "c7c49f18cb4f9905decad406f7a835f59588f34483afa3e4751097451d5d9969"
        );
        assert_eq!(lookup("hcRinf").unwrap().kind, "interval");
        assert_eq!(
            lookup("hcRinf").unwrap().hash.to_hex(),
            "0d0308e874e54cb3d02570c972232b0d26c2d1d64b493880a1bb7ce4ff7827b2"
        );
        assert_eq!(lookup("Rinf").unwrap().kind, "interval");
        assert_eq!(
            lookup("Rinf").unwrap().hash.to_hex(),
            "fe5eb033872921d3fde70b701a5b1f6369cd9cde9063a995c0ee0ebc46222090"
        );
        assert_eq!(lookup("a0").unwrap().kind, "interval");
        assert_eq!(
            lookup("a0").unwrap().hash.to_hex(),
            "5d5098fcd983d3db221e4b4047e73de5061985c31a91ccdf12cd122b620eaf29"
        );
        assert_eq!(lookup("Eh").unwrap().kind, "interval");
        assert_eq!(
            lookup("Eh").unwrap().hash.to_hex(),
            "c4606c77e55763a397f633ef0f3ace1328d3e1e8781428baf97554c97f4fba5a"
        );
        assert_eq!(lookup("me_mmu").unwrap().kind, "interval");
        assert_eq!(
            lookup("me_mmu").unwrap().hash.to_hex(),
            "d57979e61fa03bae0a3b0dc5e2cff20df53cdcb76b772cf6ea2589e77c9c3cb2"
        );
        assert_eq!(lookup("me_mp").unwrap().kind, "interval");
        assert_eq!(
            lookup("me_mp").unwrap().hash.to_hex(),
            "b573fa37eb0080e54bc71e3bf41170421c2bae2911609e1d11ffc129448a2e7b"
        );
        assert_eq!(lookup("me_mn").unwrap().kind, "interval");
        assert_eq!(
            lookup("me_mn").unwrap().hash.to_hex(),
            "e271d2015c7b39491daebf2a1d532ebe4c4dacf8228b3f7fc4d258be7b79ecba"
        );
        assert_eq!(lookup("me_md").unwrap().kind, "interval");
        assert_eq!(
            lookup("me_md").unwrap().hash.to_hex(),
            "2aa5fe69f8cdd03f44e77b006a3b6ea90d48e1b8aec71275e184c4e529f0f76c"
        );
        assert_eq!(lookup("me_mt").unwrap().kind, "interval");
        assert_eq!(
            lookup("me_mt").unwrap().hash.to_hex(),
            "2f8187d744269836cf0fbc123f8cb7d60107215e65be109daf2ae67c8116afd1"
        );
        assert_eq!(lookup("me_mh").unwrap().kind, "interval");
        assert_eq!(
            lookup("me_mh").unwrap().hash.to_hex(),
            "0fb8f5fde9e76fcf2c24d73267f36cd3a5b40ca9f27f24e47d9807ec4206055e"
        );
        assert_eq!(lookup("me_malpha").unwrap().kind, "interval");
        assert_eq!(
            lookup("me_malpha").unwrap().hash.to_hex(),
            "3407529f38a47a2cf983c5418482d3d9bab5243e08258bbe88c06a2f42e0baa3"
        );
        assert_eq!(lookup("e_me").unwrap().kind, "interval");
        assert_eq!(
            lookup("e_me").unwrap().hash.to_hex(),
            "bfe24e8de43e90dbc8a28472f99ed206f07566fa1a4fa6c6d14356adf4e89b22"
        );
        assert_eq!(lookup("M_e").unwrap().kind, "interval");
        assert_eq!(
            lookup("M_e").unwrap().hash.to_hex(),
            "0a8b3285a4969854567b59db2ebf9449268df86ffdbb461e3b9c1db0955eb804"
        );
        assert_eq!(lookup("lambdabar_C").unwrap().kind, "interval");
        assert_eq!(
            lookup("lambdabar_C").unwrap().hash.to_hex(),
            "0ed48571f065fc19458ea3c8fd493fd00de18a7d196669f81bb93c50779bc625"
        );
        assert_eq!(lookup("lambda_C").unwrap().kind, "interval");
        assert_eq!(
            lookup("lambda_C").unwrap().hash.to_hex(),
            "6280f2b2f61adf3ae0fa3e65f3b12cfb4982f6601027d98552f541246198c3d8"
        );
        assert_eq!(lookup("re").unwrap().kind, "interval");
        assert_eq!(
            lookup("re").unwrap().hash.to_hex(),
            "1b8dfc7aa2f90183fd50dab61cf3361f57c3c906e6a221ffa3b2ef17302a38d4"
        );
        assert_eq!(lookup("m_p").unwrap().kind, "interval");
        assert_eq!(
            lookup("m_p").unwrap().hash.to_hex(),
            "ffd371a69f7ec3d9bac8dcf57e0126709fd3f63c35561e717d9886d2fb1f88c8"
        );
        assert_eq!(lookup("h").unwrap().kind, "sci-exact");
        assert_eq!(lookup("au").unwrap().kind, "ratio");
        assert_eq!(
            lookup("au").unwrap().hash.to_hex(),
            "d3441603d75b565016c25cc955783fbb76b4050ee22befcef0c0e3896e873a0b"
        );
        assert_eq!(lookup("eV").unwrap().kind, "ratio");
        assert_eq!(
            lookup("eV").unwrap().hash.to_hex(),
            "d5514de9cbef3f6990067899529d34f20b4349ca3b20ba18c9a5932c8c6b6c0f"
        );
        assert_eq!(lookup("GM_sun").unwrap().kind, "ratio");
        assert_eq!(
            lookup("GM_sun").unwrap().hash.to_hex(),
            "636001001c4ed9cd5e6661241e5ad5e5db09c8419a3fe79790143162b7af3a58"
        );
        assert_eq!(lookup("R_sun").unwrap().kind, "ratio");
        assert_eq!(
            lookup("R_sun").unwrap().hash.to_hex(),
            "cb7f91f2d0663d2d8ff8b0e3009f6e0772a126220d04ed658fc793db7e5cc6b4"
        );
        assert_eq!(lookup("L_sun").unwrap().kind, "ratio");
        assert_eq!(
            lookup("L_sun").unwrap().hash.to_hex(),
            "444f85fba501ddec8fb08ba403c1b869cc78a2284df5466a56a617043807bbc4"
        );
        assert!(lookup("hbar").is_none());
        assert!(lookup("m_e").is_none());
        assert!(lookup("me/m_mu").is_none());
        assert!(lookup("sigma_e").is_none());
        assert!(lookup("Y0").is_none());
        assert!(lookup("Z_0").is_none());
        assert!(lookup("epsilon_0").is_none());
        assert!(lookup("eps0").is_none());
        assert!(lookup("mu_0").is_none());
        assert!(lookup("alpha-inv").is_none());
        assert!(lookup("alpha_inv").is_none());
        assert!(lookup("inverse-alpha").is_none());
        assert!(lookup("fine-structure").is_none());
        assert!(lookup("R_inf").is_none());
        assert!(lookup("Rydberg").is_none());
        assert!(lookup("c_Rinf").is_none());
        assert!(lookup("hc_Rinf").is_none());
        assert!(lookup("hcRinf_eV").is_none());
        assert!(lookup("a_0").is_none());
        assert!(lookup("Bohr").is_none());
        assert!(lookup("E_h").is_none());
        assert!(lookup("hartree").is_none());
        assert!(lookup("Eh_eV").is_none());
        assert!(lookup("solar-gm").is_none());
        assert!(lookup("gut.weinberg-angle").is_none());
    }
}
