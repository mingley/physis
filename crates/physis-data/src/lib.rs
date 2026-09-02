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
    /// Central interval (statistical), or the allowed hull of a
    /// one-sided limit. A Super-K lower limit is `[τ_min, open-end]`,
    /// not a measurement of a finite lifetime.
    pub statistical: Interval,
    /// Systematic interval, if given as an enclosure.
    pub systematic: Option<Interval>,
    /// Likelihood model. A two-sided measurement may carry a Gaussian;
    /// a one-sided limit must not.
    #[serde(default)]
    pub likelihood: LikelihoodModel,
    /// Provenance.
    pub source: SourceRecord,
}

/// How a dataset enters a likelihood, if at all.
///
/// Interval-subset is a hull comparison, not a Gaussian. Super-K's
/// lower limit is that kind of object. A PDG mixing-angle measurement
/// with a published central value and σ is a Gaussian.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LikelihoodModel {
    /// Hull comparison only. One-sided limits and open-end placeholders.
    #[default]
    IntervalSubset,
    /// Two-sided Gaussian: NLL = `(x − μ)² / (2σ²)` as an exact Ratio.
    Gaussian {
        /// Central value.
        mu: Ratio,
        /// One-sigma width. Must be positive.
        sigma: Ratio,
    },
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
        buf.push_str(&format!("{:?}\n", LikelihoodModel::IntervalSubset));
        buf.push_str(&source.source_hash.to_hex());
        Self {
            artifact_hash: ArtifactId::of(buf.as_bytes()),
            id,
            observable,
            unit,
            statistical,
            systematic,
            likelihood: LikelihoodModel::IntervalSubset,
            source,
        }
    }

    /// Overlay a two-sided Gaussian. Rebuilds the listing hash.
    /// `sigma` must be positive. Does not mint a kernel proof.
    pub fn with_gaussian(mut self, mu: Ratio, sigma: Ratio) -> Self {
        assert!(sigma > Ratio::int(0), "gaussian sigma must be positive");
        self.likelihood = LikelihoodModel::Gaussian { mu, sigma };
        let mut buf = String::new();
        buf.push_str(&self.id);
        buf.push('\n');
        buf.push_str(&self.observable);
        buf.push('\n');
        buf.push_str(&self.unit);
        buf.push_str(&format!("{:?}\n{:?}\n", self.statistical, self.systematic));
        buf.push_str(&format!("{:?}\n", self.likelihood));
        buf.push_str(&self.source.source_hash.to_hex());
        self.artifact_hash = ArtifactId::of(buf.as_bytes());
        self
    }

    /// Combined uncertainty hull (stat ± syst, treated as independent
    /// interval sum — not a covariance). `systematic` is a shift
    /// enclosure, typically centred at 0. Honest and conservative.
    pub fn combined(&self) -> Interval {
        match self.systematic {
            None => self.statistical,
            Some(sys) => self.statistical + sys,
        }
    }

    /// Gaussian centre when this listing is a two-sided measurement.
    pub fn gaussian_mu(&self) -> Option<Ratio> {
        match self.likelihood {
            LikelihoodModel::Gaussian { mu, .. } => Some(mu),
            LikelihoodModel::IntervalSubset => None,
        }
    }
}

/// PDG 2024 MS-bar `sin²θ_W(M_Z)` as a versioned dataset artifact.
///
/// The hull is `0.23122 ± 0.00001` written as rationals, and the
/// likelihood is that same Gaussian (`μ = 23122/100000`, `σ = 1/100000`).
/// This is the *low-energy* mixing angle, not the GUT-scale `3/8`.
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
    .with_gaussian(Ratio::new(23122, 100000), Ratio::new(1, 100000))
}

/// Dataset id for PDG 2022 `α_s(M_Z)`.
pub const PDG_2022_ALPHA_S_MZ: &str = "pdg-2022-alpha-s-mz";

/// PDG 2022 Physical Constants `α_s(m_Z) = 0.1179(9)`.
///
/// This is the recorded centre already stored as `0.1179` in physis-model,
/// with the published one-sigma width `0.0009`, not a last-digit guess.
pub fn pdg_2022_alpha_s_mz() -> Dataset {
    let source = SourceRecord::new(
        Citation {
            work: "PDG Review of Particle Physics".into(),
            edition: "2022".into(),
        },
        "2022",
        SourceLocator {
            page: None,
            section: Some("Physical Constants".into()),
            equation: None,
            figure: None,
            table: Some("alpha_s(mZ)".into()),
            dataset_range: Some("0.1179(9)".into()),
            experiment: None,
        },
        ArtifactId::of(b"pdg-2022-phys-constants-alpha-s-mz"),
        None,
    )
    .expect("PDG 2022 α_s locator names a section, table, and range");
    Dataset::new(
        PDG_2022_ALPHA_S_MZ,
        "alpha_s(M_Z)",
        "1",
        Interval::new(Ratio::new(1170, 10000), Ratio::new(1188, 10000)),
        None,
        source,
    )
    .with_gaussian(Ratio::new(1179, 10000), Ratio::new(9, 10000))
}

/// Dataset id for PDG 2022 `α_em⁻¹(M_Z)`.
pub const PDG_2022_INV_ALPHA_EM_MZ: &str = "pdg-2022-inv-alpha-em-mz";

/// PDG 2022 Electroweak `α^{(5)}(M_Z)^{-1} = 127.951 ± 0.009`.
///
/// This is the recorded centre already stored as `127.951` in physis-model,
/// with the published one-sigma width `0.009` from the MS-bar five-flavour
/// running coupling, not a last-digit guess.
pub fn pdg_2022_inv_alpha_em_mz() -> Dataset {
    let source = SourceRecord::new(
        Citation {
            work: "PDG Review of Particle Physics".into(),
            edition: "2022".into(),
        },
        "2022",
        SourceLocator {
            page: None,
            section: Some("Electroweak Model and Constraints on New Physics".into()),
            equation: None,
            figure: None,
            table: None,
            dataset_range: Some("alpha^(5)(MZ)^{-1} = 127.951 ± 0.009".into()),
            experiment: None,
        },
        ArtifactId::of(b"pdg-2022-electroweak-inv-alpha-em-mz"),
        None,
    )
    .expect("PDG 2022 α_em locator names a section and range");
    Dataset::new(
        PDG_2022_INV_ALPHA_EM_MZ,
        "alpha_em^{-1}(M_Z)",
        "1",
        Interval::new(Ratio::new(127942, 1000), Ratio::new(127960, 1000)),
        None,
        source,
    )
    .with_gaussian(Ratio::new(127951, 1000), Ratio::new(9, 1000))
}

/// Dataset id for Super-Kamiokande `p → e⁺π⁰` (Takenaka et al. 2020).
pub const SK_2020_P_E_PI0: &str = "sk-2020-p-e-pi0";

/// Super-Kamiokande 90% CL lower limit `τ/B(p → e⁺π⁰) > 2.4×10³⁴ yr`.
///
/// Units are `10^31 yr`, so the published bound is the exact rational
/// `2400`. Super-K does not measure an upper lifetime; the hull high
/// end is an open-end placeholder (`10^12` in these units, `10^43 yr`),
/// not a Super-K observation. Interval-subset then treats the allowed
/// region as a closed interval. This is Takenaka et al., Phys. Rev. D
/// **102**, 112011 (2020) (arXiv:2010.16098), not an invented number
/// and not a dimension-5 operator bound.
pub fn super_kamiokande_proton_lifetime() -> Dataset {
    let source = SourceRecord::new(
        Citation {
            work: "Takenaka et al. (Super-Kamiokande Collaboration), \
                 Search for proton decay via p→e+π0 and p→μ+π0 with an \
                 enlarged fiducial volume in Super-Kamiokande I-IV"
                .into(),
            edition: "Phys. Rev. D 102, 112011 (2020); arXiv:2010.16098".into(),
        },
        "2020",
        SourceLocator {
            page: None,
            section: Some("Results".into()),
            equation: None,
            figure: None,
            table: None,
            dataset_range: Some("tau/B(p→e+π0) 90% CL; 450 kton·year".into()),
            experiment: Some("Super-Kamiokande I-IV".into()),
        },
        ArtifactId::of(b"sk-prd-d-102-112011-pepi0"),
        None,
    )
    .expect("Super-K locator names experiment and dataset range");
    Dataset::new(
        SK_2020_P_E_PI0,
        "tau/B(p→e+π0)",
        "10^31 yr",
        Interval::new(Ratio::int(2400), Ratio::int(1_000_000_000_000)),
        None,
        source,
    )
}

/// Live dataset whose [`SourceRecord`] is the empirical provenance of
/// `claim_id`. The GUT-scale `3/8` cell is not this registry. GQW at `M_Z`
/// compares to the PDG mixing-angle listing; one-loop `α_3(M_Z)` compares
/// to the PDG 2022 `α_s` listing. The complementary PDG listings are
/// *inputs* to those predictions, not this map.
pub fn dataset_for_claim(claim_id: &str) -> Option<Dataset> {
    match claim_id {
        "gut.weinberg-angle-mz-interval" => Some(pdg_2024_sin2theta()),
        "gut.coupling-unification-interval" => Some(pdg_2022_alpha_s_mz()),
        "gut.proton-lifetime-sk" => Some(super_kamiokande_proton_lifetime()),
        _ => None,
    }
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
///
/// When the dataset carries a Gaussian and the caller supplies a point
/// prediction, `nll` is the exact Ratio `(x − μ)² / (2σ²)`. That object
/// is not interval-subset and is not inferred from a one-sided hull.
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
    /// Exact Gaussian NLL of a supplied point, when the dataset is Gaussian.
    pub nll: Option<Ratio>,
}

impl EmpiricalReceipt {
    /// Compare a theory prediction interval to a dataset under the
    /// interval-subset decision rule. Does not compute a Gaussian NLL.
    pub fn compare(prediction: Interval, dataset: &Dataset) -> Self {
        Self::compare_gaussian(prediction, dataset, None)
    }

    /// Interval-subset, plus exact Gaussian NLL of `x` when the dataset
    /// is a two-sided measurement. `x` is ignored on an interval-subset
    /// dataset (Super-K): a one-sided limit is not a Gaussian.
    pub fn compare_gaussian(prediction: Interval, dataset: &Dataset, x: Option<Ratio>) -> Self {
        let data = dataset.combined();
        let excluded = prediction.disjoint(data);
        let compatible = data.contains(prediction);
        let inconclusive = !excluded && !compatible;
        let nll = match (x, dataset.likelihood) {
            (Some(x), LikelihoodModel::Gaussian { mu, sigma }) => Some(x.gaussian_nll(mu, sigma)),
            _ => None,
        };
        let mut buf = String::new();
        buf.push_str(&format!("{prediction:?}\n{data:?}\ninterval-subset"));
        if let Some(nll) = nll {
            buf.push_str(&format!("\ngaussian-nll {nll}"));
        }
        Self {
            prediction_hash: ArtifactId::of(format!("{prediction:?}").as_bytes()),
            dataset_hash: dataset.artifact_hash,
            analysis_hash: ArtifactId::of(buf.as_bytes()),
            excluded,
            compatible,
            inconclusive,
            nll,
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
    fn super_kamiokande_p_e_pi0_is_a_registered_lower_limit() {
        let d = super_kamiokande_proton_lifetime();
        assert_eq!(d.id, SK_2020_P_E_PI0);
        assert_eq!(d.unit, "10^31 yr");
        assert_eq!(d.statistical.lo, Ratio::int(2400));
        assert!(d.statistical.hi > d.statistical.lo);
        assert!(d.source.locator.experiment.as_deref() == Some("Super-Kamiokande I-IV"));
        assert!(d.source.citation.work.contains("Takenaka"));
        assert!(!d.source.citation.work.to_lowercase().contains("textbook"));
    }

    #[test]
    fn minimal_su5_scaling_is_excluded_by_super_k() {
        let pred = Interval::point(Ratio::int(1));
        let rec = EmpiricalReceipt::compare(pred, &super_kamiokande_proton_lifetime());
        assert!(rec.excluded);
        assert!(!rec.compatible);
        assert_eq!(rec.status(), EmpiricalStatus::Excluded);
    }

    #[test]
    fn mssm_dim6_scaling_is_compatible_with_super_k() {
        let pred = Interval::point(Ratio::int(1_600_000_000));
        let rec = EmpiricalReceipt::compare(pred, &super_kamiokande_proton_lifetime());
        assert!(rec.compatible);
        assert!(!rec.excluded);
        assert_eq!(rec.status(), EmpiricalStatus::Compatible);
    }

    #[test]
    fn an_envelope_that_crosses_the_limit_is_inconclusive() {
        let pred = Interval::new(Ratio::int(1), Ratio::int(1_000_000_000));
        let rec = EmpiricalReceipt::compare(pred, &super_kamiokande_proton_lifetime());
        assert!(rec.inconclusive);
        assert!(!rec.excluded);
        assert!(!rec.compatible);
        assert_eq!(rec.status(), EmpiricalStatus::Inconclusive);
    }

    #[test]
    fn pdg_mixing_angle_is_a_gaussian_measurement() {
        let d = pdg_2024_sin2theta();
        assert!(matches!(
            d.likelihood,
            LikelihoodModel::Gaussian { mu, sigma }
                if mu == Ratio::new(23122, 100000) && sigma == Ratio::new(1, 100000)
        ));
        let x = Ratio::new(23122, 100000);
        let rec = EmpiricalReceipt::compare_gaussian(Interval::point(x), &d, Some(x));
        assert_eq!(rec.nll, Some(Ratio::int(0)));
        assert!(rec.compatible);
        let three_eighths = Ratio::new(3, 8);
        let far = EmpiricalReceipt::compare_gaussian(
            Interval::point(three_eighths),
            &d,
            Some(three_eighths),
        );
        assert!(far.excluded);
        assert_eq!(
            far.nll,
            Some(three_eighths.gaussian_nll(Ratio::new(23122, 100000), Ratio::new(1, 100000)))
        );
        assert!(far.nll.unwrap() > Ratio::int(0));
        let hull_only = EmpiricalReceipt::compare(Interval::point(x), &d);
        assert!(hull_only.nll.is_none());
    }

    #[test]
    fn super_k_is_not_a_gaussian() {
        let d = super_kamiokande_proton_lifetime();
        assert_eq!(d.likelihood, LikelihoodModel::IntervalSubset);
        let rec = EmpiricalReceipt::compare_gaussian(
            Interval::point(Ratio::int(1)),
            &d,
            Some(Ratio::int(1)),
        );
        assert!(rec.nll.is_none());
        assert!(rec.excluded);
    }

    #[test]
    fn dataset_for_claim_is_the_hashed_source_not_a_slogan() {
        let pdg = dataset_for_claim("gut.weinberg-angle-mz-interval").expect("PDG cell");
        assert_eq!(pdg.id, "pdg-2024-sin2theta");
        assert!(pdg.source.recheck().is_ok());
        let sk = dataset_for_claim("gut.proton-lifetime-sk").expect("Super-K cell");
        assert_eq!(sk.id, SK_2020_P_E_PI0);
        assert!(sk.source.recheck().is_ok());
        assert!(dataset_for_claim("predictivity.unique-vacuum").is_none());
        assert!(dataset_for_claim("gut.weinberg-angle").is_none());
        assert!(dataset_for_claim("gut.coupling-unification").is_none());
        assert!(dataset_for_claim("dec.d-squared-zero").is_none());
        let als = dataset_for_claim("gut.coupling-unification-interval").expect("α_s cell");
        assert_eq!(als.id, PDG_2022_ALPHA_S_MZ);
        assert!(als.source.recheck().is_ok());
    }

    #[test]
    fn pdg_2022_couplings_are_the_recorded_centres_with_published_sigma() {
        let als = pdg_2022_alpha_s_mz();
        assert_eq!(als.id, PDG_2022_ALPHA_S_MZ);
        assert_eq!(als.gaussian_mu(), Some(Ratio::new(1179, 10000)));
        assert!(als
            .statistical
            .contains(Interval::point(Ratio::new(1179, 10000))));
        assert_eq!(
            als.statistical,
            Interval::new(Ratio::new(1170, 10000), Ratio::new(1188, 10000))
        );
        assert!(als.source.recheck().is_ok());
        assert!(als.source.locator.table.as_deref() == Some("alpha_s(mZ)"));

        let inv = pdg_2022_inv_alpha_em_mz();
        assert_eq!(inv.id, PDG_2022_INV_ALPHA_EM_MZ);
        assert_eq!(inv.gaussian_mu(), Some(Ratio::new(127951, 1000)));
        assert!(inv
            .statistical
            .contains(Interval::point(Ratio::new(127951, 1000))));
        assert_eq!(
            inv.statistical,
            Interval::new(Ratio::new(127942, 1000), Ratio::new(127960, 1000))
        );
        assert!(inv.source.recheck().is_ok());
        assert!(
            dataset_for_claim("gut.weinberg-angle-mz-interval")
                .unwrap()
                .id
                != PDG_2022_ALPHA_S_MZ
        );
    }

    #[test]
    fn combined_uses_interval_sum_not_a_shared_denominator() {
        let d = Dataset::new(
            "toy",
            "x",
            "1",
            Interval::new(Ratio::new(1, 2), Ratio::new(3, 2)),
            Some(Interval::new(Ratio::new(-1, 3), Ratio::new(1, 3))),
            pdg_2024_sin2theta().source,
        );
        assert_eq!(
            d.combined(),
            Interval::new(Ratio::new(1, 6), Ratio::new(11, 6))
        );
    }
}
