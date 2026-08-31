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
//! UNIVERSAL): `1/(μ₀ c²)` after SI 2019, not an exact Ratio. `Z₀` is
//! not stored. CODATA 2018 fine-structure `α` is
//! a one-sigma [`Interval`] `7.2973525693(11)×10^{-3}` (JPCRD table
//! XXXI, ATOMIC AND NUCLEAR): a measured hull, not an SI defining
//! Ratio. Inverse-α is not stored. CODATA 2018 proton mass `m_p` is a
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

fn codata_2018_alpha_source() -> SourceRecord {
    codata_2018_jpcrd("ATOMIC AND NUCLEAR", "alpha = 7.2973525693(11)e-3")
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
/// Ratio and not P3N. `Z₀ = μ₀ c` is not stored. Theories still
/// use `physis_model` `f64` Qty.
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
/// defining Ratio and not P3N. `Z₀` is not stored. Theories still
/// use `physis_model` `f64` Qty.
pub fn vacuum_permittivity() -> Constant<Interval> {
    Constant::new(
        "epsilon0",
        codata_2018_epsilon0_interval(),
        "F m^{-1}",
        codata_2018_epsilon0_source(),
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
/// Ratio, not inverse-α, and not P3N. Running with energy is M4.
/// Theories still use `physis_model` `f64` Qty.
pub fn fine_structure_constant() -> Constant<Interval> {
    Constant::new(
        "alpha",
        codata_2018_alpha_interval(),
        "1",
        codata_2018_alpha_source(),
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
    "alpha",
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
        "alpha" => Some(listing(fine_structure_constant(), "interval")),
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
        assert!(lookup("Z0").is_none());
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
        assert!(lookup("Z0").is_none());
        assert!(lookup("Z_0").is_none());
        assert!(lookup("epsilon_0").is_none());
        assert!(lookup("eps0").is_none());
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
        assert_eq!(LEDGER.len(), 17);
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
        assert_eq!(lookup("alpha").unwrap().kind, "interval");
        assert_eq!(
            lookup("alpha").unwrap().hash.to_hex(),
            "cef64589acdbd1ed4cb5f5f631658978c01477248f334b1d3563e57314644b38"
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
        assert!(lookup("Z0").is_none());
        assert!(lookup("Z_0").is_none());
        assert!(lookup("epsilon_0").is_none());
        assert!(lookup("eps0").is_none());
        assert!(lookup("mu_0").is_none());
        assert!(lookup("alpha-inv").is_none());
        assert!(lookup("fine-structure").is_none());
        assert!(lookup("solar-gm").is_none());
        assert!(lookup("gut.weinberg-angle").is_none());
    }
}
