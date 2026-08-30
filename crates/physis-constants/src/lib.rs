//! Versioned physical constants. Never scatter magic floats in theories.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use physis_core::artifact::ArtifactId;
use physis_numeric::Ratio;
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

/// Speed of light, exact, SI 2019.
pub fn speed_of_light() -> Constant<Ratio> {
    Constant::new(
        "c",
        Ratio::int(299_792_458),
        "m/s",
        si_brochure(),
        ConstantRelease::Si2019Codata2018,
    )
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
    }
}
