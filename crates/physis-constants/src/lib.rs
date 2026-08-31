//! Versioned physical constants. Never scatter magic floats in theories.
//!
//! SI 2019 defining constants that fit in [`physis_numeric::Ratio`] are
//! `c`, `Δν_Cs`, `e`, `k`, `N_A`, and `K_cd`. Planck's `h` is SI-exact
//! [`physis_numeric::SciExact`] `662607015e-42` J s: the reduced
//! denominator does not fit in `i128`, so it is not a Ratio. `ħ` is not
//! a terminating decimal. CODATA 2018 Newtonian `G` is a one-sigma
//! [`Interval`], not an exact Ratio. Theories still use `physis_model`
//! `f64` Qty constants. This crate does not mint a kernel proof.

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
}

impl ConstantRelease {
    /// Stable name.
    pub const fn as_str(self) -> &'static str {
        match self {
            ConstantRelease::Si2019Codata2018 => "si-2019-codata-2018",
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

fn codata_2018_g_source() -> SourceRecord {
    SourceRecord::new(
        Citation {
            work: "CODATA recommended values of the fundamental physical constants: 2018".into(),
            edition: "J. Phys. Chem. Ref. Data 50, 033105".into(),
        },
        "2018",
        SourceLocator {
            page: None,
            section: Some("UNIVERSAL".into()),
            equation: None,
            figure: None,
            table: Some("XXXI".into()),
            dataset_range: Some("G = 6.67430(15)e-11".into()),
            experiment: None,
        },
        ArtifactId::of(b"codata-2018-jpcrd-50-033105"),
        None,
    )
    .expect("CODATA 2018 G locator names a table and range")
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
pub const LEDGER: &[&str] = &["c", "delta-nu-Cs", "e", "k", "N_A", "K_cd", "h", "G"];

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
    fn lookup_rebuilds_the_live_ledger_and_rejects_unknown_names() {
        assert_eq!(LEDGER.len(), 8);
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
        assert_eq!(lookup("h").unwrap().kind, "sci-exact");
        assert!(lookup("hbar").is_none());
        assert!(lookup("gut.weinberg-angle").is_none());
    }
}
