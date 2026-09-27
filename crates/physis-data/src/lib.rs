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
    /// Reuse license of the source listing, when audited
    /// (`public-domain`, …). Existing listings keep `None` (not
    /// audited) and their hashes: the license enters the listing hash
    /// only when set.
    #[serde(default)]
    pub license: Option<String>,
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
        let mut out = Self {
            artifact_hash: ArtifactId::of(b"pending"),
            id,
            observable,
            unit,
            statistical,
            systematic,
            likelihood: LikelihoodModel::IntervalSubset,
            source,
            license: None,
        };
        out.rehash();
        out
    }

    /// Rebind the listing hash to the live fields. Byte-identical to the
    /// historical layout while `license` is `None`; a set license appends
    /// one line so licensed and unlicensed listings never share a hash.
    fn rehash(&mut self) {
        let mut buf = String::new();
        buf.push_str(&self.id);
        buf.push('\n');
        buf.push_str(&self.observable);
        buf.push('\n');
        buf.push_str(&self.unit);
        buf.push_str(&format!("{:?}\n{:?}\n", self.statistical, self.systematic));
        buf.push_str(&format!("{:?}\n", self.likelihood));
        buf.push_str(&self.source.source_hash.to_hex());
        if let Some(license) = &self.license {
            buf.push('\n');
            buf.push_str(license);
        }
        self.artifact_hash = ArtifactId::of(buf.as_bytes());
    }

    /// Overlay a two-sided Gaussian. Rebuilds the listing hash.
    /// `sigma` must be positive. Does not mint a kernel proof.
    pub fn with_gaussian(mut self, mu: Ratio, sigma: Ratio) -> Self {
        assert!(sigma > Ratio::int(0), "gaussian sigma must be positive");
        self.likelihood = LikelihoodModel::Gaussian { mu, sigma };
        self.rehash();
        self
    }

    /// Annotate the reuse license of the source listing and rebind the
    /// hash. The license is a reuse annotation, not a second measurement:
    /// it never changes the hull, the likelihood, or the comparison rule.
    pub fn with_license(mut self, license: impl Into<String>) -> Self {
        self.license = Some(license.into());
        self.rehash();
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

/// Dataset id for Dyson et al. 1920 Sobral 4-inch deflection.
pub const DYSON_1920_SOBRAL_DEFLECTION: &str = "dyson-1920-sobral-deflection";

/// Dyson, Eddington & Davidson 1920 Sobral 4-inch result: 1.98″ with
/// published probable error 0.12″, i.e. the reported standard deviation
/// σ = 0.18″.
///
/// The stored hull is the honest 1σ `[1.80, 2.16]`, not widened to force
/// compatibility: the GR prediction sits ~1.3σ low, so 1σ
/// interval-subset would spuriously exclude it (see
/// [`EmpiricalReceipt::compare_two_sigma`]). The 1919 paper gives a
/// single probable-error figure, so there is no separate systematic
/// enclosure. The companion Principe result (1.61″ ± 0.30″ probable
/// error) is not registered; comparisons against this listing use the
/// Sobral 4-inch plates only. License `public-domain`: published January
/// 1920 (Phil. Trans. R. Soc. Lond. A 220, 291–333;
/// doi:10.1098/rsta.1920.0009), and the listing itself is a fact, not
/// expression.
pub fn dyson_1920_sobral_deflection() -> Dataset {
    let source = SourceRecord::new(
        Citation {
            work: "Dyson, Eddington & Davidson, A Determination of the Deflection \
                 of Light by the Sun's Gravitational Field, from Observations Made \
                 at the Total Eclipse of May 29, 1919"
                .into(),
            edition: "Phil. Trans. R. Soc. Lond. A 220, 291–333 (1920); doi:10.1098/rsta.1920.0009"
                .into(),
        },
        "1920",
        SourceLocator {
            page: None,
            section: None,
            equation: None,
            figure: None,
            table: None,
            dataset_range: Some("Sobral 4-inch: 1.98 ± 0.12 probable error; σ = 0.18".into()),
            experiment: Some("Sobral expedition, May 29, 1919 eclipse".into()),
        },
        ArtifactId::of(b"dyson-1920-sobral-4inch-deflection"),
        None,
    )
    .expect("Sobral locator names experiment and dataset range");
    Dataset::new(
        DYSON_1920_SOBRAL_DEFLECTION,
        "grazing solar light deflection",
        "arcsec",
        Interval::new(Ratio::new(180, 100), Ratio::new(216, 100)),
        None,
        source,
    )
    .with_gaussian(Ratio::new(198, 100), Ratio::new(18, 100))
    .with_license("public-domain")
}

/// Dataset id for Clemence 1947 Mercury residual precession.
pub const CLEMENCE_1947_MERCURY_RESIDUAL: &str = "clemence-1947-mercury-residual";

/// Clemence 1947 Mercury excess: 42.56 ± 0.94 ″/Julian century.
///
/// Observed 5599.74±0.41 minus computed Newtonian 5557.18±0.85
/// (planetary perturbations plus precession), combined in quadrature by
/// the source. Frame: moving equinox at 1850. Time: Julian century of
/// 36,525 mean solar days. The ±0.94 is a probable error under
/// Clemence's stated convention (50% inclusion; σ = 0.94/0.6745), so
/// the Gaussian is μ = 42.56, σ = 1880/1349. This listing is the
/// MEASURED residual, not a GR prediction: Clemence's own Table I
/// theoretical value is 43.03 (modern constants give 42.98; see Nobili
/// & Will 1986). There is no separate systematic enclosure: the source
/// gives one combined figure. License `public-domain`: the listing
/// records only the numerical residual, its uncertainty, and the
/// bibliographic locator (facts, not expression); the 1947 paper itself
/// remains under its publisher's copyright.
pub fn clemence_1947_mercury_residual() -> Dataset {
    let mu = Ratio::new(4256, 100);
    let sigma = Ratio::new(1880, 1349);
    let source = SourceRecord::new(
        Citation {
            work: "Clemence, The Relativity Effect in Planetary Motions".into(),
            edition: "Rev. Mod. Phys. 19, 361 (1947); doi:10.1103/RevModPhys.19.361".into(),
        },
        "1947",
        SourceLocator {
            page: None,
            section: None,
            equation: None,
            figure: None,
            table: None,
            dataset_range: Some(
                "residual 42.56 ± 0.94 probable error (observed 5599.74±0.41 minus computed 5557.18±0.85)"
                    .into(),
            ),
            experiment: Some(
                "Mercury meridian-observation solution at 1850, moving equinox, Julian century of 36,525 mean solar days"
                    .into(),
            ),
        },
        ArtifactId::of(b"clemence-1947-mercury-residual"),
        None,
    )
    .expect("Clemence locator names experiment and dataset range");
    Dataset::new(
        CLEMENCE_1947_MERCURY_RESIDUAL,
        "Mercury excess perihelion precession (observed minus Newtonian-planetary; moving equinox; Julian century)",
        "arcsec cy^{-1}",
        Interval::new(mu - sigma, mu + sigma),
        None,
        source,
    )
    .with_gaussian(mu, sigma)
    .with_license("public-domain")
}

/// Dataset id for Quinn & Martin 1985 Stefan-Boltzmann constant.
pub const QUINN_MARTIN_1985_SIGMA: &str = "quinn-martin-1985-sigma";

/// Quinn & Martin 1985 Stefan-Boltzmann constant: σ = 5.66967 ± 0.00076
/// (×10⁻⁸ W m⁻² K⁻⁴), relative standard uncertainty 130 ppm.
///
/// Total radiant exitance of a blackbody at the triple point of water
/// (T_tp = 273.16 K), measured with the NPL cryogenic radiometer
/// (absolute radiation thermometer; electrical-substitution
/// calibration, independent of σ). This listing is the MEASUREMENT,
/// not the SI-exact combination 2π⁵k⁴/(15h³c²): that combination uses
/// defining constants, while this value was radiometrically determined
/// and disagrees with it by ~1σ. Fit and calibration inputs are
/// separate by construction: the radiometer scale comes from electrical
/// power, the temperature reference is the defined triple point, and
/// the lab prediction uses only h/c/k floats plus numerical quadrature.
/// There is no separate systematic enclosure: the source gives one
/// combined standard uncertainty. License `public-domain`: the listing
/// records only the numerical value, its uncertainty, and the
/// bibliographic locator (facts, not expression); the 1985 paper itself
/// remains under its publisher's copyright.
pub fn quinn_martin_1985_sigma() -> Dataset {
    let mu = Ratio::new(566967, 10i128.pow(13));
    let sigma = Ratio::new(76, 10i128.pow(13));
    let source = SourceRecord::new(
        Citation {
            work: "Quinn & Martin, A radiometric determination of the Stefan-Boltzmann constant and thermodynamic temperatures between -40 C and +100 C"
                .into(),
            edition: "Phil. Trans. R. Soc. Lond. A 316, 85 (1985); doi:10.1098/rsta.1985.0058"
                .into(),
        },
        "1985",
        SourceLocator {
            page: None,
            section: None,
            equation: None,
            figure: None,
            table: None,
            dataset_range: Some(
                "σ = (5.66967 ± 0.00076)e-8 at T_tp (relative standard uncertainty 130 ppm)"
                    .into(),
            ),
            experiment: Some(
                "NPL cryogenic radiometer; blackbody total exitance at T_tp = 273.16 K".into(),
            ),
        },
        ArtifactId::of(b"quinn-martin-1985-sigma"),
        None,
    )
    .expect("Quinn-Martin locator names experiment and dataset range");
    Dataset::new(
        QUINN_MARTIN_1985_SIGMA,
        "Stefan-Boltzmann constant (blackbody total exitance at T_tp; cryogenic radiometry)",
        "W m^{-2} K^{-4}",
        Interval::new(mu - sigma, mu + sigma),
        None,
        source,
    )
    .with_gaussian(mu, sigma)
    .with_license("public-domain")
}

/// Live dataset whose [`SourceRecord`] is the empirical provenance of
/// `claim_id`. The GUT-scale `3/8` cell is not this registry. GQW at `M_Z`
/// compares to the PDG mixing-angle listing; one-loop `α_3(M_Z)` compares
/// to the PDG 2022 `α_s` listing. One-loop `α_em⁻¹(M_Z)` compares to the
/// PDG 2022 `α_em⁻¹` listing. The complementary PDG listings are
/// *inputs* to those predictions, not this map. Solar light deflection
/// compares to the Dyson et al. 1920 Sobral 4-inch listing. Mercury
/// extra-precession compares to the Clemence 1947 residual listing.
/// Stefan-Boltzmann σ compares to the Quinn & Martin 1985 listing.
pub fn dataset_for_claim(claim_id: &str) -> Option<Dataset> {
    match claim_id {
        "gut.weinberg-angle-mz-interval" => Some(pdg_2024_sin2theta()),
        "gut.coupling-unification-interval" => Some(pdg_2022_alpha_s_mz()),
        "gut.inverse-alpha-em-mz-interval" => Some(pdg_2022_inv_alpha_em_mz()),
        "gut.proton-lifetime-sk" => Some(super_kamiokande_proton_lifetime()),
        "gr.eddington-deflection" => Some(dyson_1920_sobral_deflection()),
        "gr.mercury-perihelion" => Some(clemence_1947_mercury_residual()),
        "thermo.stefan-boltzmann" => Some(quinn_martin_1985_sigma()),
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

    /// Exclusion-grade comparison for two-sided Gaussian measurements:
    /// interval-subset against the μ±2σ (≈95%) hull, plus the exact
    /// Gaussian NLL of `x`.
    ///
    /// A 1σ hull is not an exclusion rule: a prediction 1.3σ from the
    /// centre would be spuriously excluded by [`Self::compare`]. The 2σ
    /// threshold is the predeclared verdict rule for two-sided
    /// historical measurements, matching standard exclusion practice
    /// (cf. the Super-K 90% CL limit). The dataset keeps its honest 1σ
    /// hull and Gaussian; only this rule reads the 2σ hull. Refuses
    /// [`LikelihoodModel::IntervalSubset`] datasets: a one-sided limit
    /// has no σ. The analysis hash commits to the `two-sigma` rule, so
    /// these receipts are distinguishable from 1σ receipts.
    pub fn compare_two_sigma(
        prediction: Interval,
        dataset: &Dataset,
        x: Option<Ratio>,
    ) -> Result<Self, String> {
        let (mu, sigma) = match dataset.likelihood {
            LikelihoodModel::Gaussian { mu, sigma } => (mu, sigma),
            LikelihoodModel::IntervalSubset => {
                return Err(format!(
                    "two-sigma comparison refused: dataset {} is a one-sided limit, not a Gaussian",
                    dataset.id
                ));
            }
        };
        let two = Ratio::int(2);
        let hull = Interval::new(mu - two * sigma, mu + two * sigma);
        let excluded = prediction.disjoint(hull);
        let compatible = hull.contains(prediction);
        let inconclusive = !excluded && !compatible;
        let nll = x.map(|x| x.gaussian_nll(mu, sigma));
        let mut buf = String::new();
        buf.push_str(&format!("{prediction:?}\n{hull:?}\ntwo-sigma"));
        if let Some(nll) = nll {
            buf.push_str(&format!("\ngaussian-nll {nll}"));
        }
        Ok(Self {
            prediction_hash: ArtifactId::of(format!("{prediction:?}").as_bytes()),
            dataset_hash: dataset.artifact_hash,
            analysis_hash: ArtifactId::of(buf.as_bytes()),
            excluded,
            compatible,
            inconclusive,
            nll,
        })
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
        let invem = dataset_for_claim("gut.inverse-alpha-em-mz-interval").expect("α_em cell");
        assert_eq!(invem.id, PDG_2022_INV_ALPHA_EM_MZ);
        assert!(invem.source.recheck().is_ok());
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

    #[test]
    fn sobral_1919_is_a_registered_gaussian_deflection() {
        let d = dyson_1920_sobral_deflection();
        assert_eq!(d.id, DYSON_1920_SOBRAL_DEFLECTION);
        assert_eq!(d.unit, "arcsec");
        // Honest 1σ hull, not widened to force compatibility.
        assert_eq!(
            d.statistical,
            Interval::new(Ratio::new(180, 100), Ratio::new(216, 100))
        );
        assert!(matches!(
            d.likelihood,
            LikelihoodModel::Gaussian { mu, sigma }
                if mu == Ratio::new(198, 100) && sigma == Ratio::new(18, 100)
        ));
        assert_eq!(d.license.as_deref(), Some("public-domain"));
        assert!(d.source.citation.work.contains("Dyson"));
        assert!(d.source.citation.edition.contains("10.1098/rsta.1920.0009"));
        assert!(!d.source.citation.work.to_lowercase().contains("textbook"));
        assert!(d.source.locator.experiment.is_some());
        assert!(d.source.recheck().is_ok());
        // Pre-existing listings are not license-audited; None is honest.
        assert_eq!(pdg_2024_sin2theta().license, None);
        assert_eq!(super_kamiokande_proton_lifetime().license, None);
        let edd = dataset_for_claim("gr.eddington-deflection").expect("Eddington cell");
        assert_eq!(edd.id, DYSON_1920_SOBRAL_DEFLECTION);
    }

    #[test]
    fn two_sigma_rule_compares_gr_and_soldner_to_sobral() {
        let d = dyson_1920_sobral_deflection();
        // Live RK4 centres snapped to 0.001″ (representation, not physics).
        let gr_x = Ratio::new(1751, 1000);
        let gr = EmpiricalReceipt::compare_two_sigma(Interval::point(gr_x), &d, Some(gr_x))
            .expect("Sobral is Gaussian");
        assert!(gr.compatible);
        assert!(!gr.excluded);
        assert!(!gr.inconclusive);
        assert_eq!(gr.status(), EmpiricalStatus::Compatible);
        assert_eq!(gr.nll, Some(Ratio::new(52441, 64800)));
        let newton_x = Ratio::new(876, 1000);
        let newton =
            EmpiricalReceipt::compare_two_sigma(Interval::point(newton_x), &d, Some(newton_x))
                .expect("Sobral is Gaussian");
        assert!(newton.excluded);
        assert!(!newton.compatible);
        assert!(!newton.inconclusive);
        assert_eq!(newton.status(), EmpiricalStatus::Excluded);
        assert_eq!(newton.nll, Some(Ratio::new(4232, 225)));
        assert!(gr.nll.unwrap() < newton.nll.unwrap());
        assert_eq!(gr.dataset_hash, d.artifact_hash);
        assert_eq!(newton.dataset_hash, d.artifact_hash);
    }

    #[test]
    fn one_sigma_hull_is_not_the_exclusion_rule() {
        // This is why compare_two_sigma exists: the honest 1σ hull
        // narrowly excludes the GR prediction, so 1σ interval-subset
        // cannot be the verdict rule for two-sided measurements.
        let d = dyson_1920_sobral_deflection();
        let rec = EmpiricalReceipt::compare(Interval::point(Ratio::new(1751, 1000)), &d);
        assert!(rec.excluded);
        assert!(rec.nll.is_none());
    }

    #[test]
    fn clemence_1947_is_a_registered_gaussian_residual() {
        let d = clemence_1947_mercury_residual();
        assert_eq!(d.id, CLEMENCE_1947_MERCURY_RESIDUAL);
        assert_eq!(d.unit, "arcsec cy^{-1}");
        let mu = Ratio::new(4256, 100);
        let sigma = Ratio::new(1880, 1349);
        assert_eq!(d.statistical, Interval::new(mu - sigma, mu + sigma));
        assert!((d.statistical.lo.to_f64() - 41.16636).abs() < 1e-4);
        assert!((d.statistical.hi.to_f64() - 43.95364).abs() < 1e-4);
        assert!(matches!(
            d.likelihood,
            LikelihoodModel::Gaussian { mu: m, sigma: s }
                if m == mu && s == sigma
        ));
        assert_eq!(d.license.as_deref(), Some("public-domain"));
        assert!(d.source.citation.work.contains("Clemence"));
        assert!(d
            .source
            .citation
            .edition
            .contains("10.1103/RevModPhys.19.361"));
        assert!(!d.source.citation.work.to_lowercase().contains("textbook"));
        assert!(d.source.locator.experiment.is_some());
        assert!(d.source.recheck().is_ok());
        let merc = dataset_for_claim("gr.mercury-perihelion").expect("Mercury cell");
        assert_eq!(merc.id, CLEMENCE_1947_MERCURY_RESIDUAL);
    }

    #[test]
    fn two_sigma_rule_compares_gr_and_newton_to_clemence() {
        let d = clemence_1947_mercury_residual();
        // Live RK4 centres snapped to 0.001 (representation, not physics).
        let gr_x = Ratio::new(42981, 1000);
        let gr = EmpiricalReceipt::compare_two_sigma(Interval::point(gr_x), &d, Some(gr_x))
            .expect("Clemence is Gaussian");
        assert!(gr.compatible);
        assert!(!gr.excluded);
        assert!(!gr.inconclusive);
        assert_eq!(gr.status(), EmpiricalStatus::Compatible);
        assert_eq!(gr.nll, Some(Ratio::new(322543349041, 7068800000000)));
        let newton_x = Ratio::new(1, 1000);
        let newton =
            EmpiricalReceipt::compare_two_sigma(Interval::point(newton_x), &d, Some(newton_x))
                .expect("Clemence is Gaussian");
        assert!(newton.excluded);
        assert!(!newton.compatible);
        assert!(!newton.inconclusive);
        assert_eq!(newton.status(), EmpiricalStatus::Excluded);
        assert_eq!(
            newton.nll,
            Some(Ratio::new(3296148192992281, 7068800000000))
        );
        assert!(gr.nll.unwrap() < newton.nll.unwrap());
        assert_eq!(gr.dataset_hash, d.artifact_hash);
        assert_eq!(newton.dataset_hash, d.artifact_hash);
    }

    #[test]
    fn clemence_margin_is_comfortable_even_at_one_sigma() {
        // Unlike Sobral 1919, the Clemence residual accommodates GR at 1σ
        // too; the predeclared rule stays two-sigma either way.
        let d = clemence_1947_mercury_residual();
        let rec = EmpiricalReceipt::compare(Interval::point(Ratio::new(42981, 1000)), &d);
        assert!(rec.compatible);
        assert!(!rec.excluded);
    }

    #[test]
    fn quinn_martin_1985_is_a_registered_gaussian_sigma() {
        let d = quinn_martin_1985_sigma();
        assert_eq!(d.id, QUINN_MARTIN_1985_SIGMA);
        assert_eq!(d.unit, "W m^{-2} K^{-4}");
        let mu = Ratio::new(566967, 10i128.pow(13));
        let sigma = Ratio::new(76, 10i128.pow(13));
        assert_eq!(d.statistical, Interval::new(mu - sigma, mu + sigma));
        assert!((d.statistical.lo.to_f64() - 5.66891e-8).abs() < 1e-13);
        assert!((d.statistical.hi.to_f64() - 5.67043e-8).abs() < 1e-13);
        assert!(matches!(
            d.likelihood,
            LikelihoodModel::Gaussian { mu: m, sigma: s }
                if m == mu && s == sigma
        ));
        assert_eq!(d.license.as_deref(), Some("public-domain"));
        assert!(d.source.citation.work.contains("Quinn"));
        assert!(d.source.citation.edition.contains("10.1098/rsta.1985.0058"));
        assert!(!d.source.citation.work.to_lowercase().contains("textbook"));
        assert!(d.source.locator.experiment.is_some());
        assert!(d.source.recheck().is_ok());
        let sb = dataset_for_claim("thermo.stefan-boltzmann").expect("Stefan cell");
        assert_eq!(sb.id, QUINN_MARTIN_1985_SIGMA);
    }

    #[test]
    fn two_sigma_rule_compares_planck_and_wien_to_quinn_martin() {
        let d = quinn_martin_1985_sigma();
        // Live integral ratios snapped to 1e-13 (representation, not physics).
        let planck_x = Ratio::new(567037, 10i128.pow(13));
        let planck =
            EmpiricalReceipt::compare_two_sigma(Interval::point(planck_x), &d, Some(planck_x))
                .expect("Quinn-Martin is Gaussian");
        assert!(planck.compatible);
        assert!(!planck.excluded);
        assert!(!planck.inconclusive);
        assert_eq!(planck.status(), EmpiricalStatus::Compatible);
        assert_eq!(planck.nll, Some(Ratio::new(1225, 2888)));
        let wien_x = Ratio::new(523908, 10i128.pow(13));
        let wien = EmpiricalReceipt::compare_two_sigma(Interval::point(wien_x), &d, Some(wien_x))
            .expect("Quinn-Martin is Gaussian");
        assert!(wien.excluded);
        assert!(!wien.compatible);
        assert!(!wien.inconclusive);
        assert_eq!(wien.status(), EmpiricalStatus::Excluded);
        assert_eq!(wien.nll, Some(Ratio::new(1854077481, 11552)));
        assert!(planck.nll.unwrap() < wien.nll.unwrap());
        assert_eq!(planck.dataset_hash, d.artifact_hash);
        assert_eq!(wien.dataset_hash, d.artifact_hash);
    }

    #[test]
    fn two_sigma_refuses_one_sided_limits() {
        let d = super_kamiokande_proton_lifetime();
        let err = EmpiricalReceipt::compare_two_sigma(
            Interval::point(Ratio::int(1)),
            &d,
            Some(Ratio::int(1)),
        )
        .unwrap_err();
        assert!(err.contains("one-sided"), "{err}");
    }
}
