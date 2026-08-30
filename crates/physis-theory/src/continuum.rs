//! Continuum (M4): a field as an actual local object, not a flag.
//!
//! [`KleinGordonField`] is a real scalar field on a finite 1D periodic lattice.
//! Its degrees of freedom are the N lattice sites, coupled by a nearest-
//! neighbour discrete Laplacian, so its normal modes are *computed*:
//!
//! ```text
//! ω_j² = m² + (4/a²) · sin²(π j / N),   j = 0 … N-1
//! ```
//!
//! The claims are theorems of that computation rather than tabulated facts: the
//! long-wavelength dispersion matches the continuum `ω² = m² + k²`, and a
//! negative `mass_squared` produces a genuine tachyonic mode (`min ω² < 0`) —
//! the same instability notion as the string bosonic tachyon, but computed.

use std::f64::consts::PI;

use physis_core::assumption::DomainOfValidity;
use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::EmpiricalStatus;
use physis_core::ParameterOrigin;
use physis_model::{GaugeGroup, Manifold, Spectrum, World};

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;

/// The field has a finite number of normal modes.
pub const FINITE_MODES: &str = "field.finite-modes";
/// The lattice dispersion matches the continuum ω² = m² + k² at long wavelength.
pub const DISPERSION: &str = "field.dispersion-continuum-limit";
/// No tachyonic mode: min ω² ≥ 0.
pub const STABLE: &str = "field.stable";
/// The group velocity is bounded by c.
pub const CAUSAL: &str = "field.causal";
/// The coupling is local (nearest-neighbour).
pub const LOCAL: &str = "field.local";
/// The discretization is second-order accurate (error ∝ a²).
pub const SECOND_ORDER: &str = "field.second-order-accurate";

/// Long-wavelength domain for the O(a²) identity: `|k a| < 1` at the
/// convergence probe. Outside it, Richardson's `p` is not a verdict on the
/// stencil — the lattice does not resolve the probe.
const ASYMPTOTIC_KA: f64 = 1.0;

/// Matrix rows for the field lab.
pub fn field_rows() -> [&'static str; 6] {
    [
        FINITE_MODES,
        DISPERSION,
        STABLE,
        CAUSAL,
        LOCAL,
        SECOND_ORDER,
    ]
}

/// Fixed physical wavenumber used to probe the discretization's accuracy order.
const CONVERGENCE_PROBE_K: f64 = 0.1;

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "sites",
        layer: LayerId::Field,
        doc: "Number of lattice sites N (the field's local degrees of freedom).",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 2, max: 4096 },
    },
    KnobSpec {
        name: "mass_squared",
        layer: LayerId::Field,
        doc: "Mass-squared m² in natural units. Negative values make the zero mode tachyonic.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: -100.0,
            max: 100.0,
        },
    },
    KnobSpec {
        name: "spacing",
        layer: LayerId::Field,
        doc: "Lattice spacing a in natural units.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 1.0e-3,
            max: 1.0e3,
        },
    },
];

/// A real scalar (Klein–Gordon) field on a finite 1D periodic lattice.
#[derive(Clone, Debug)]
pub struct KleinGordonField {
    sites: u32,
    mass_squared: f64,
    spacing: f64,
}

impl Default for KleinGordonField {
    fn default() -> Self {
        Self {
            sites: 16,
            mass_squared: 1.0,
            spacing: 1.0,
        }
    }
}

impl KleinGordonField {
    /// ω² for normal mode `j` (0-based) from the discrete Laplacian.
    fn omega_sq(&self, j: u32) -> f64 {
        let n = self.sites as f64;
        let a = self.spacing;
        let s = (PI * j as f64 / n).sin();
        self.mass_squared + (4.0 / (a * a)) * s * s
    }

    /// Wavenumber of mode `j`.
    fn k(&self, j: u32) -> f64 {
        2.0 * PI * j as f64 / (self.sites as f64 * self.spacing)
    }

    /// Minimum ω² over all modes (the stability-controlling value).
    fn min_omega_sq(&self) -> f64 {
        (0..self.sites)
            .map(|j| self.omega_sq(j))
            .fold(f64::INFINITY, f64::min)
    }

    /// Largest group velocity dω/dk over the modes.
    ///
    /// A tachyonic mode (ω² < 0) has an imaginary frequency and no sensible,
    /// causal group velocity, so the field is treated as non-causal. The
    /// massless zero mode (ω² = 0) is fine: its group velocity is 0.
    fn max_group_velocity(&self) -> f64 {
        let a = self.spacing;
        let mut max = 0.0_f64;
        for j in 0..self.sites {
            let w2 = self.omega_sq(j);
            if w2 < 0.0 {
                return f64::INFINITY;
            }
            // d(ω²)/dk = (2/a) sin(ka); v_g = that / (2ω). The zero mode has
            // sin(0) = 0, so v_g = 0 even as ω → 0.
            let ka = self.k(j) * a;
            let vg = if w2 <= 0.0 {
                0.0
            } else {
                ((2.0 / a) * ka.sin()).abs() / (2.0 * w2.sqrt())
            };
            max = max.max(vg);
        }
        max
    }

    /// Absolute dispersion error of the discrete Laplacian at a fixed physical
    /// wavenumber `k`, for lattice spacing `a`: `|(4/a²) sin²(ka/2) − k²|`.
    fn dispersion_abs_error(&self, k: f64, a: f64) -> f64 {
        let s = (k * a / 2.0).sin();
        ((4.0 / (a * a)) * s * s - k * k).abs()
    }

    /// Empirical convergence order p, from the error at spacing `a` vs `a/2`:
    /// `p = log2(err(a) / err(a/2))`. A second-order scheme gives p ≈ 2.
    fn convergence_order(&self) -> f64 {
        let a = self.spacing;
        let e1 = self.dispersion_abs_error(CONVERGENCE_PROBE_K, a);
        let e2 = self.dispersion_abs_error(CONVERGENCE_PROBE_K, a / 2.0);
        if e2 <= 0.0 || e1 <= 0.0 {
            // Exact at this probe: treat as (at least) second order.
            return 2.0;
        }
        (e1 / e2).log2()
    }

    /// Probe wavenumber times spacing. The O(a²) expansion of
    /// `(4/a²) sin²(ka/2)` is a long-wavelength statement.
    fn probe_ka(&self) -> f64 {
        CONVERGENCE_PROBE_K * self.spacing
    }

    /// True when the convergence probe is outside the long-wavelength domain.
    fn too_coarse_for_order(&self) -> bool {
        self.probe_ka().abs() >= ASYMPTOTIC_KA
    }

    /// Relative error between the longest-wavelength mode and the continuum
    /// `ω² = m² + k²`.
    fn long_wavelength_rel_error(&self) -> f64 {
        let j = 1; // longest non-zero wavelength
        let lattice = self.omega_sq(j);
        let k = self.k(j);
        let continuum = self.mass_squared + k * k;
        if continuum.abs() < 1e-12 {
            (lattice - continuum).abs()
        } else {
            (lattice - continuum).abs() / continuum.abs()
        }
    }
}

impl Knobbed for KleinGordonField {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "sites" => Ok(KnobValue::UInt(self.sites as u64)),
            "mass_squared" => Ok(KnobValue::Float(self.mass_squared)),
            "spacing" => Ok(KnobValue::Float(self.spacing)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("sites", KnobValue::UInt(v)) => self.sites = v as u32,
            ("mass_squared", KnobValue::Float(v)) => self.mass_squared = v,
            ("spacing", KnobValue::Float(v)) => self.spacing = v,
            _ => {
                return Err(CoreError::TypeMismatch {
                    name: name.into(),
                    expected: spec.domain.kind_name().into(),
                    got: old.kind_name().into(),
                });
            }
        }
        Ok(old)
    }
}

impl Theory for KleinGordonField {
    fn id(&self) -> &'static str {
        "klein-gordon"
    }
    fn name(&self) -> &'static str {
        "Klein–Gordon field (1D lattice)"
    }
    fn summary(&self) -> &'static str {
        "A real scalar field as an actual local object: N lattice sites coupled \
         by a nearest-neighbour Laplacian, with computed normal modes. Its \
         stability and dispersion are theorems of the computation, not flags."
    }
    fn world(&self) -> Option<World> {
        // A 1+1 D field: one time direction, one spatial lattice direction.
        Some(World {
            spacetime: Manifold {
                dim: 2,
                signature: physis_model::Signature { time: 1, space: 1 },
                compact_extra: 0,
                compact_radius_planck: 0.0,
                topology: physis_model::Topology::Minkowski,
                convention: physis_model::SignConvention::MostlyPlus,
            },
            gauge: GaugeGroup::trivial(),
            spectrum: Spectrum::empty(),
            has_gravity: false,
            supersymmetric: false,
            free_parameter_count: 3,
            landscape_log10: 0.0,
            note: format!(
                "Klein–Gordon scalar on {} sites, m²={}, a={}, min ω²={:.4}",
                self.sites,
                self.mass_squared,
                self.spacing,
                self.min_omega_sq()
            ),
        })
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                FINITE_MODES,
                "The field has a finite number of normal modes.",
                LayerId::Field,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                DISPERSION,
                "The long-wavelength dispersion matches the continuum ω² = m² + k².",
                LayerId::Field,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                STABLE,
                "There is no tachyonic mode (min ω² ≥ 0).",
                LayerId::Field,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                CAUSAL,
                "The group velocity is bounded by c.",
                LayerId::Field,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                LOCAL,
                "The coupling is local (nearest-neighbour).",
                LayerId::Field,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                SECOND_ORDER,
                "The discretization is second-order accurate (error ∝ a²).",
                LayerId::Field,
                ClaimClass::ModelInternal,
            )
            .with_domain(DomainOfValidity::new(
                vec!["|k a| < 1 at the Richardson probe".into()],
                vec!["O(a^2) stencil at long wavelength".into()],
                "Outside |k a| < 1 the Richardson order is not a stencil verdict. \
                 That spacing is undecidable / insufficient-precision, not a failed theorem.",
            )),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id.0.as_str() {
            FINITE_MODES => {
                Verdict::holds(claim, format!("{} normal modes on the lattice", self.sites))
            }
            DISPERSION => {
                let err = self.long_wavelength_rel_error();
                if err < 0.05 {
                    Verdict::holds(claim, "long-wavelength mode matches continuum ω² = m² + k²")
                        .with_evidence([format!(
                            "relative error {:.2}% at the longest wavelength",
                            err * 100.0
                        )])
                } else {
                    Verdict::fails(
                        claim,
                        format!(
                            "lattice too coarse: {:.1}% error vs the continuum dispersion",
                            err * 100.0
                        ),
                    )
                }
            }
            STABLE => {
                let m = self.min_omega_sq();
                if m >= -1e-12 {
                    Verdict::holds(claim, format!("min ω² = {m:.4} ≥ 0"))
                } else {
                    Verdict::fails(
                        claim,
                        format!("tachyonic mode: min ω² = {m:.4} < 0 (unstable)"),
                    )
                    .with_evidence([
                        "same instability notion as the bosonic-string tachyon, here computed"
                            .to_string(),
                    ])
                }
            }
            CAUSAL => {
                let v = self.max_group_velocity();
                if v <= 1.0 + 1e-9 {
                    Verdict::holds(claim, format!("max group velocity {v:.4} ≤ c"))
                } else {
                    Verdict::fails(claim, format!("superluminal group velocity {v:.4} > c"))
                }
            }
            LOCAL => Verdict::holds(claim, "nearest-neighbour Laplacian: the coupling is local"),
            SECOND_ORDER => {
                let p = self.convergence_order();
                let ka = self.probe_ka();
                if self.too_coarse_for_order() {
                    Verdict::undecidable(
                        claim,
                        format!(
                            "probe |k a| = {ka:.3} ≥ {ASYMPTOTIC_KA}: too coarse to certify O(a²)"
                        ),
                    )
                    .with_empirical(EmpiricalStatus::Inconclusive)
                    .with_evidence([
                        format!("Richardson p = {p:.3} is not a stencil verdict outside |k a| < 1"),
                        "the discrete Laplacian is still O(a²) at long wavelength; this spacing does not resolve the probe"
                            .to_string(),
                    ])
                } else if (1.8..=2.2).contains(&p) {
                    Verdict::holds(
                        claim,
                        format!("measured convergence order p = {p:.3} ≈ 2 (error ∝ a²)"),
                    )
                    .with_evidence([
                        "computed by halving the lattice spacing at a fixed physical wavenumber"
                            .to_string(),
                        format!("probe |k a| = {ka:.3} is inside the long-wavelength domain"),
                    ])
                } else {
                    Verdict::fails(
                        claim,
                        format!("measured order p = {p:.3} ≠ 2 inside the long-wavelength domain"),
                    )
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by a field object"),
        }
    }
}

/// The field-modes experiment: a scalar field's computed spectrum and stability.
pub fn field_modes() -> ExperimentReport {
    let theories: Vec<Box<dyn Theory>> = vec![Box::new(KleinGordonField::default())];
    report_from_rows(
        "field-modes",
        "Field modes lab",
        "Can a field be an actual local object — N lattice sites with a computed \
         spectrum — so that stability and the continuum dispersion are theorems \
         of the computation, and a negative mass² produces a real tachyon?",
        "The normal modes are computed from the discrete Laplacian, not \
         tabulated. `field.stable` reads the sign of the minimum ω²; a negative \
         mass_squared knob makes it fail, the same instability as the bosonic \
         string's tachyon.",
        vec![
            "`holds` / `fails` are internal to the encoding.".into(),
            "Modes and dispersion are computed: ω_j² = m² + (4/a²) sin²(π j / N).".into(),
            "`set klein-gordon mass_squared -1` makes the zero mode tachyonic and `field.stable` fails.".into(),
        ],
        &field_rows(),
        theories,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;

    fn verdict(t: &dyn Theory, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id.0 == id).unwrap();
        t.evaluate(&c).kind
    }

    #[test]
    fn default_field_is_stable_causal_and_dispersive() {
        let f = KleinGordonField::default();
        assert_eq!(verdict(&f, STABLE), VerdictKind::Holds);
        assert_eq!(verdict(&f, CAUSAL), VerdictKind::Holds);
        assert_eq!(verdict(&f, DISPERSION), VerdictKind::Holds);
        assert_eq!(verdict(&f, LOCAL), VerdictKind::Holds);
    }

    #[test]
    fn negative_mass_squared_is_a_computed_tachyon() {
        // The field knob → verdict diff: a negative mass² destabilizes the field
        // and, being an imaginary frequency, also makes it non-causal.
        let mut f = KleinGordonField::default();
        assert_eq!(verdict(&f, STABLE), VerdictKind::Holds);
        assert_eq!(verdict(&f, CAUSAL), VerdictKind::Holds);
        f.set("mass_squared", KnobValue::Float(-1.0)).unwrap();
        assert_eq!(verdict(&f, STABLE), VerdictKind::Fails);
        assert_eq!(verdict(&f, CAUSAL), VerdictKind::Fails);
    }

    #[test]
    fn even_a_slightly_negative_mass_squared_flips_both() {
        // Regression: when only the zero mode is tachyonic, both stable and
        // causal must still fail (the j=0 mode must not be skipped).
        for &m2 in &[-0.05_f64, -1.0] {
            let mut f = KleinGordonField::default();
            f.set("mass_squared", KnobValue::Float(m2)).unwrap();
            assert_eq!(verdict(&f, STABLE), VerdictKind::Fails, "m²={m2}");
            assert_eq!(verdict(&f, CAUSAL), VerdictKind::Fails, "m²={m2}");
        }
        // And on a tiny lattice where only the zero mode can be tachyonic.
        let mut small = KleinGordonField::default();
        small.set("sites", KnobValue::UInt(2)).unwrap();
        small.set("mass_squared", KnobValue::Float(-1.0)).unwrap();
        assert_eq!(verdict(&small, STABLE), VerdictKind::Fails);
        assert_eq!(verdict(&small, CAUSAL), VerdictKind::Fails);
    }

    #[test]
    fn massless_field_is_stable_and_causal() {
        // The zero mode has ω² = 0; that is fine, not a tachyon.
        let mut f = KleinGordonField::default();
        f.set("mass_squared", KnobValue::Float(0.0)).unwrap();
        assert_eq!(verdict(&f, STABLE), VerdictKind::Holds);
        assert_eq!(verdict(&f, CAUSAL), VerdictKind::Holds);
    }

    #[test]
    fn discretization_is_second_order_accurate() {
        // The discrete Laplacian converges at O(a²): computed order ≈ 2.
        let f = KleinGordonField::default();
        let v = {
            let c = f
                .claims()
                .into_iter()
                .find(|c| c.id.0 == SECOND_ORDER)
                .unwrap();
            f.evaluate(&c)
        };
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_eq!(v.empirical, EmpiricalStatus::NotApplicable);
        let order_claim = f
            .claims()
            .into_iter()
            .find(|c| c.id.0 == SECOND_ORDER)
            .unwrap();
        assert!(
            order_claim
                .domain
                .regimes
                .iter()
                .any(|r| r.contains("|k a| < 1")),
            "second-order is a long-wavelength claim, not encoding-wide: {:?}",
            order_claim.domain
        );
        assert!((f.convergence_order() - 2.0).abs() < 0.1);
        // An absurdly coarse lattice is a resolution gap, not a failed stencil.
        let mut coarse = KleinGordonField::default();
        coarse.set("spacing", KnobValue::Float(100.0)).unwrap();
        let c = coarse
            .claims()
            .into_iter()
            .find(|c| c.id.0 == SECOND_ORDER)
            .unwrap();
        let u = coarse.evaluate(&c);
        assert_eq!(u.kind, VerdictKind::Undecidable);
        assert_eq!(u.empirical, EmpiricalStatus::Inconclusive);
        assert_ne!(u.kind, VerdictKind::Fails);
        assert_ne!(
            u.derivation,
            physis_core::DerivationAssurance::CertifiedNumeric
        );
    }

    #[test]
    fn mode_count_follows_the_sites_knob() {
        let mut f = KleinGordonField::default();
        f.set("sites", KnobValue::UInt(32)).unwrap();
        assert_eq!(f.claims().len(), 6);
        // 32 sites → 32 modes; the minimum ω² is the zero mode = mass².
        assert!((f.min_omega_sq() - f.mass_squared).abs() < 1e-9);
    }

    #[test]
    fn field_experiment_builds_a_matrix() {
        let r = field_modes();
        assert_eq!(r.id, "field-modes");
        assert_eq!(r.theories.len(), 1);
        assert_eq!(
            r.matrix
                .get(STABLE)
                .and_then(|m| m.get("klein-gordon"))
                .copied(),
            Some(VerdictKind::Holds)
        );
    }
}
