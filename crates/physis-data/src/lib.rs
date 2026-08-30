//! Empirical data is an artifact, not a naked number.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use physis_core::artifact::ArtifactId;
use physis_core::EmpiricalStatus;
use physis_numeric::{Interval, Ratio};
use physis_provenance::{Citation, SourceLocator, SourceRecord};
use serde::{Deserialize, Serialize};

/// A registered dataset.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dataset {
    /// Stable id (`pdg-2024-sin2theta`, …).
    pub id: String,
    /// Content hash of the canonical listing.
    pub artifact_hash: ArtifactId,
    /// Observable name.
    pub observable: String,
    /// Unit, as a string until the IR owns it.
    pub unit: String,
    /// Central interval (statistical).
    pub statistical: Interval,
    /// Systematic interval, if given as an enclosure.
    pub systematic: Option<Interval>,
    /// Provenance.
    pub source: SourceRecord,
}

impl Dataset {
    /// Build and hash.
    pub fn new(
        id: impl Into<String>,
        observable: impl Into<String>,
        unit: impl Into<String>,
        statistical: Interval,
        systematic: Option<Interval>,
        source: SourceRecord,
    ) -> Self {
        let id = id.into();
        let observable = observable.into();
        let unit = unit.into();
        let mut buf = String::new();
        buf.push_str(&id);
        buf.push('\n');
        buf.push_str(&observable);
        buf.push('\n');
        buf.push_str(&unit);
        buf.push_str(&format!("{:?}\n{:?}\n", statistical, systematic));
        buf.push_str(&source.source_hash.to_hex());
        Self {
            artifact_hash: ArtifactId::of(buf.as_bytes()),
            id,
            observable,
            unit,
            statistical,
            systematic,
            source,
        }
    }

    /// Combined uncertainty hull (stat ± syst, treated as independent
    /// interval sum — not a covariance). Honest and conservative.
    pub fn combined(&self) -> Interval {
        match self.systematic {
            None => self.statistical,
            Some(sys) => Interval::new(
                Ratio::new(
                    self.statistical.lo.num - (sys.hi.num - sys.lo.num).abs(),
                    self.statistical.lo.den,
                ),
                Ratio::new(
                    self.statistical.hi.num + (sys.hi.num - sys.lo.num).abs(),
                    self.statistical.hi.den,
                ),
            ),
        }
    }
}

/// PDG 2024 MS-bar `sin²θ_W(M_Z)` as a versioned dataset artifact.
///
/// The hull is `0.23122 ± 0.00001` written as rationals. This is the
/// *low-energy* mixing angle, not the GUT-scale `3/8`.
pub fn pdg_2024_sin2theta() -> Dataset {
    let source = SourceRecord::new(
        Citation {
            work: "PDG Review of Particle Physics".into(),
            edition: "2024".into(),
        },
        "2024",
        SourceLocator {
            page: None,
            section: Some("Electroweak".into()),
            equation: None,
            figure: None,
            table: Some("sin2thetaW".into()),
            dataset_range: None,
            experiment: None,
        },
        ArtifactId::of(b"pdg-2024-sin2"),
        None,
    )
    .expect("PDG locator names a section and table");
    Dataset::new(
        "pdg-2024-sin2theta",
        "sin^2 theta_W(M_Z)",
        "1",
        Interval::new(Ratio::new(23121, 100000), Ratio::new(23123, 100000)),
        None,
        source,
    )
}

/// Super-Kamiokande proton lifetime limit. Not a registered artifact.
///
/// The GUT heuristic cell quotes Super-K as prose. That sentence is not
/// a [`Dataset`]. Returning `None` is the catalog hole MissingDataset
/// is for. Do not mint a lifetime number to fill it.
pub fn super_kamiokande_proton_lifetime() -> Option<Dataset> {
    None
}

/// Receipt of an empirical comparison. Exclusion is this object, not
/// `prediction != known_number`.
///
/// Decision rule (`interval-subset`):
/// - `excluded`: prediction disjoint from the data hull
/// - `compatible`: prediction ⊆ data hull
/// - `inconclusive`: they overlap, but the prediction is not contained
///
/// A wide theory envelope that merely overlaps a tight measurement is
/// therefore not compatible. Overlap is not agreement.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmpiricalReceipt {
    /// Prediction enclosure hash.
    pub prediction_hash: ArtifactId,
    /// Dataset hash.
    pub dataset_hash: ArtifactId,
    /// Likelihood / decision-rule identifier.
    pub analysis_hash: ArtifactId,
    /// Whether the prediction interval is disjoint from the data hull.
    pub excluded: bool,
    /// Whether every predicted value lies in the data hull.
    pub compatible: bool,
    /// Overlap without containment: the prediction is too coarse to decide.
    pub inconclusive: bool,
}

impl EmpiricalReceipt {
    /// Compare a theory prediction interval to a dataset under the
    /// interval-subset decision rule.
    pub fn compare(prediction: Interval, dataset: &Dataset) -> Self {
        let data = dataset.combined();
        let excluded = prediction.disjoint(data);
        let compatible = data.contains(prediction);
        let inconclusive = !excluded && !compatible;
        let mut buf = String::new();
        buf.push_str(&format!("{prediction:?}\n{data:?}\ninterval-subset"));
        Self {
            prediction_hash: ArtifactId::of(format!("{prediction:?}").as_bytes()),
            dataset_hash: dataset.artifact_hash,
            analysis_hash: ArtifactId::of(buf.as_bytes()),
            excluded,
            compatible,
            inconclusive,
        }
    }

    /// Project the receipt onto the empirical axis.
    pub fn status(&self) -> EmpiricalStatus {
        if self.excluded {
            EmpiricalStatus::Excluded
        } else if self.compatible {
            EmpiricalStatus::Compatible
        } else {
            EmpiricalStatus::Inconclusive
        }
    }
}

#[cfg(test)]
mod tests {
    use physis_numeric::{Interval, Ratio};

    use super::*;

    #[test]
    fn su5_three_eighths_is_excluded_by_the_mz_measurement() {
        let pred = Interval::point(Ratio::new(3, 8));
        let rec = EmpiricalReceipt::compare(pred, &pdg_2024_sin2theta());
        assert!(rec.excluded);
        assert!(!rec.compatible);
        assert!(!rec.inconclusive);
        assert_eq!(rec.status(), EmpiricalStatus::Excluded);
    }

    #[test]
    fn a_prediction_on_the_measurement_is_compatible() {
        let pred = Interval::point(Ratio::new(23122, 100000));
        let rec = EmpiricalReceipt::compare(pred, &pdg_2024_sin2theta());
        assert!(!rec.excluded);
        assert!(rec.compatible);
        assert!(!rec.inconclusive);
        assert_eq!(rec.status(), EmpiricalStatus::Compatible);
    }

    #[test]
    fn a_wide_envelope_that_overlaps_is_inconclusive_not_compatible() {
        let pred = Interval::point(Ratio::new(23122, 100000)).relative_envelope(Ratio::new(3, 100));
        let rec = EmpiricalReceipt::compare(pred, &pdg_2024_sin2theta());
        assert!(!rec.excluded);
        assert!(!rec.compatible);
        assert!(rec.inconclusive);
        assert_eq!(rec.status(), EmpiricalStatus::Inconclusive);
    }

    #[test]
    fn super_kamiokande_proton_lifetime_is_not_registered() {
        assert!(super_kamiokande_proton_lifetime().is_none());
    }
}
