//! Newtonian gravity on trial against general relativity.
//!
//! Two solar-system theorems that ended the 19th-century inverse-square
//! monopoly:
//!
//! - **Eddington / Soldner light deflection.** A photon grazing the Sun is
//!   bent by `2 GM/(c² R)` in Newtonian corpuscular gravity (Soldner 1801,
//!   Einstein 1911) and by `4 GM/(c² R)` on a Schwarzschild null geodesic
//!   (Einstein 1915). Eddington 1919 measured the latter.
//! - **Mercury's perihelion.** Inverse-square gravity produces a closed
//!   ellipse (Newton). The Schwarzschild Binet term `3 (GM/c²) u²` advances
//!   perihelion by `6π GM / (c² a (1−e²))` per orbit — 43″ per century.
//!
//! Both numbers are *integrated*, not tabulated: a shared RK4 stepper on the
//! Binet equation `u'' + u = rhs(u)`. Newtonian light uses `rhs = (GM/c²)/b²`;
//! GR light uses `rhs = 3 (GM/c²) u²`. Planets use `GM/h²` with or without
//! the GR term.
//!
//! [`NewtonianGravity`] is the standing theory. [`crate::relativity::GeneralRelativity`]
//! is the 1915 resolution. `set general-relativity dim 5` makes the 4D solar
//! tests inapplicable.

use std::f64::consts::{FRAC_PI_2, PI};

use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobSpec, KnobValue, Knobbed};
use physis_core::{Length, Qty};
use physis_model::constants::{
    mercury_eccentricity, mercury_orbits_per_century, mercury_semi_major, solar_gm, solar_radius, C,
};
use physis_model::{GaugeGroup, Manifold, Spectrum, World};

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;
use crate::relativity::GeneralRelativity;

/// Grazing solar light deflection is 1.75″ (Eddington / full GR).
pub const EDDINGTON: &str = "gr.eddington-deflection";
/// Mercury's extra perihelion advance is 43″ per century.
pub const MERCURY_PERIHELION: &str = "gr.mercury-perihelion";
/// Light deflection is the Newtonian/1911 value 2 GM/(c² b).
pub const NEWTON_HALF: &str = "gr.newton-half-deflection";

/// Matrix rows for the gravity lab.
pub fn gravity_rows() -> [&'static str; 3] {
    [NEWTON_HALF, EDDINGTON, MERCURY_PERIHELION]
}

const ARCSEC_PER_RAD: f64 = 180.0 / PI * 3600.0;
/// Eddington's GR target, arcseconds (grazing Sun).
const EDDINGTON_ARCSEC: f64 = 1.75;
/// Observed GR perihelion remainder for Mercury, arcseconds per century.
const MERCURY_ARCSEC_PER_CENTURY: f64 = 42.98;

/// One RK4 step of `u'' + u = rhs(u)` with `y = (u, u')`.
fn rk4_binet(u: f64, v: f64, h: f64, rhs: impl Fn(f64) -> f64) -> (f64, f64) {
    let acc = |uu: f64| rhs(uu) - uu;
    let k1u = v;
    let k1v = acc(u);
    let k2u = v + 0.5 * h * k1v;
    let k2v = acc(u + 0.5 * h * k1u);
    let k3u = v + 0.5 * h * k2v;
    let k3v = acc(u + 0.5 * h * k2u);
    let k4u = v + h * k3v;
    let k4v = acc(u + h * k3u);
    (
        u + (h / 6.0) * (k1u + 2.0 * k2u + 2.0 * k3u + k4u),
        v + (h / 6.0) * (k1v + 2.0 * k2v + 2.0 * k3v + k4v),
    )
}

/// Solar `GM/c²` as a typed length.
pub fn solar_m() -> Qty<Length> {
    solar_gm() / (C * C)
}

/// Integrate a light ray from periapsis (`u = 1/b`, `u' = 0`) until `u = 0`.
/// The deflection is `2 (φ_asymptote − π/2)`.
fn light_deflection_rad(m: f64, b: f64, gr: bool) -> f64 {
    let mut phi = 0.0_f64;
    let mut u = 1.0 / b;
    let mut v = 0.0_f64;
    let n = 80_000_usize;
    let h = FRAC_PI_2 / (n as f64);
    let rhs = |uu: f64| {
        if gr {
            3.0 * m * uu * uu
        } else {
            m / (b * b)
        }
    };
    for _ in 0..n * 2 {
        let (u2, v2) = rk4_binet(u, v, h, rhs);
        if u2 <= 0.0 {
            let frac = u / (u - u2);
            phi += h * frac;
            break;
        }
        u = u2;
        v = v2;
        phi += h;
        if phi > PI {
            break;
        }
    }
    2.0 * (phi - FRAC_PI_2)
}

/// Extra perihelion advance per orbit, radians (0 for a closed Newtonian ellipse).
fn perihelion_advance_rad(m: f64, a: f64, e: f64, gr: bool) -> f64 {
    let p = a * (1.0 - e * e);
    let gm_h2 = 1.0 / p; // GM/h² = 1/(a(1−e²)), in 1/metres
    let mut phi = 0.0_f64;
    // Perihelion of the Kepler ellipse: r_min = a(1−e), so u = 1/(a(1−e)).
    let mut u = 1.0 / (a * (1.0 - e));
    let mut v = 0.0_f64;
    let n = 120_000_usize;
    let h = (2.0 * PI) / (n as f64);
    let rhs = |uu: f64| {
        let kepler = gm_h2;
        if gr {
            kepler + 3.0 * m * uu * uu
        } else {
            kepler
        }
    };
    // Leave perihelion (v becomes negative), pass aphelion, return.
    let mut seen_aphelion = false;
    for _ in 0..n * 2 {
        let v_prev = v;
        let (u2, v2) = rk4_binet(u, v, h, rhs);
        u = u2;
        v = v2;
        phi += h;
        if !seen_aphelion && phi > 1.0 && v_prev < 0.0 && v >= 0.0 {
            seen_aphelion = true;
        } else if seen_aphelion && v_prev > 0.0 && v <= 0.0 {
            let frac = v_prev / (v_prev - v);
            phi += h * (frac - 1.0);
            return phi - 2.0 * PI;
        }
        if phi > 3.0 * PI {
            break;
        }
    }
    0.0
}

/// Grazing solar deflection in radians.
pub fn solar_deflection_rad(gr: bool) -> f64 {
    light_deflection_rad(solar_m().value(), solar_radius().value(), gr)
}

/// Grazing solar deflection in arcseconds.
pub fn solar_deflection_arcsec(gr: bool) -> f64 {
    solar_deflection_rad(gr) * ARCSEC_PER_RAD
}

/// Mercury extra perihelion, arcseconds per century.
pub fn mercury_arcsec_per_century(gr: bool) -> f64 {
    let m = solar_m().value();
    let a = mercury_semi_major().value();
    let e = mercury_eccentricity().value();
    perihelion_advance_rad(m, a, e, gr) * mercury_orbits_per_century() * ARCSEC_PER_RAD
}

/// Analytic GR perihelion: `6π GM / (c² a (1−e²))` per orbit, in arcsec/century.
pub fn mercury_analytic_arcsec_per_century() -> f64 {
    let m = solar_m().value();
    let a = mercury_semi_major().value();
    let e = mercury_eccentricity().value();
    let per_orbit = 6.0 * PI * m / (a * (1.0 - e * e));
    per_orbit * mercury_orbits_per_century() * ARCSEC_PER_RAD
}

fn solar_ready(dim: u8) -> bool {
    dim == 4
}

/// Evaluate the three solar-system claims for Newton (`gr = false`) or GR.
pub fn eval_solar(gr: bool, dim: u8, claim: &Claim) -> Verdict {
    if !solar_ready(dim) {
        return Verdict::inapplicable(
            claim,
            "solar-system tests are 4D Schwarzschild / inverse-square",
        );
    }
    match claim.id_str() {
        NEWTON_HALF => {
            let delta = solar_deflection_arcsec(gr);
            let newton = solar_deflection_arcsec(false);
            // Compare against the computed Newtonian integral, not a table.
            if (delta - newton).abs() / newton.max(1e-12) < 0.02 && newton > 0.5 {
                Verdict::holds(claim,
                    "grazing deflection is the Newtonian 2 GM/(c² R)",
                )
                .with_evidence([format!(
                    "δ = {delta:.4}\" (Newtonian integral {newton:.4}\"; analytic 2GM/c²R = {:.4}\")",
                    2.0 * solar_m().value() / solar_radius().value() * ARCSEC_PER_RAD
                )])
            } else {
                Verdict::fails(
                    claim,
                    "deflection is not the Newtonian half-angle (spatial curvature doubles it)",
                )
                .with_evidence([format!(
                    "δ = {delta:.4}\" vs Newtonian {newton:.4}\" (GR analytic 4GM/c²R = {:.4}\")",
                    4.0 * solar_m().value() / solar_radius().value() * ARCSEC_PER_RAD
                )])
            }
        }
        EDDINGTON => {
            let delta = solar_deflection_arcsec(gr);
            if (delta - EDDINGTON_ARCSEC).abs() / EDDINGTON_ARCSEC < 0.03 {
                Verdict::holds(
                    claim,
                    "grazing solar deflection is 1.75″ (Eddington / Schwarzschild)",
                )
                .with_evidence([format!(
                    "δ = {delta:.4}\" (analytic 4GM/c²R = {:.4}\")",
                    4.0 * solar_m().value() / solar_radius().value() * ARCSEC_PER_RAD
                )])
            } else {
                Verdict::fails(
                    claim,
                    "grazing deflection is not 1.75″ (Newtonian / 1911 half-angle)",
                )
                .with_evidence([format!(
                    "δ = {delta:.4}\" (Eddington 1.75\"; Newtonian 2GM/c²R = {:.4}\")",
                    2.0 * solar_m().value() / solar_radius().value() * ARCSEC_PER_RAD
                )])
            }
        }
        MERCURY_PERIHELION => {
            let extra = mercury_arcsec_per_century(gr);
            let analytic = mercury_analytic_arcsec_per_century();
            if (extra - MERCURY_ARCSEC_PER_CENTURY).abs() < 1.5 {
                Verdict::holds(claim, "Mercury's extra perihelion is 43″ per century")
                    .with_evidence([format!(
                    "Δω = {extra:.2}\" / century (analytic 6π GM/(c²a(1−e²)) = {analytic:.2}\"/cy)"
                )])
            } else {
                Verdict::fails(
                    claim,
                    "inverse-square ellipses do not precess; the 43″ remainder is missing",
                )
                .with_evidence([format!(
                    "Δω = {extra:.3}\" / century (GR analytic {analytic:.2}\"/cy)"
                )])
            }
        }
        _ => Verdict::inapplicable(claim, "claim not made by a solar-system gravity object"),
    }
}

pub(crate) fn solar_claims() -> Vec<Claim> {
    vec![
        Claim::new(
            NEWTON_HALF,
            "Grazing solar light deflection is the Newtonian value 2 GM/(c² R) ≈ 0.87″.",
            LayerId::Spacetime,
            ClaimClass::ModelInternal,
        ),
        Claim::new(
            EDDINGTON,
            "Grazing solar light deflection is 1.75″ (Eddington 1919 / full GR).",
            LayerId::Spacetime,
            ClaimClass::ModelInternal,
        ),
        Claim::new(
            MERCURY_PERIHELION,
            "Mercury's extra perihelion advance is 43″ per century.",
            LayerId::Spacetime,
            ClaimClass::ModelInternal,
        ),
    ]
}

/// Inverse-square gravity with corpuscular light (Soldner / Newton).
#[derive(Clone, Debug, Default)]
pub struct NewtonianGravity;

impl Knobbed for NewtonianGravity {
    fn specs(&self) -> &'static [KnobSpec] {
        &[]
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        Err(CoreError::UnknownKnob { name: name.into() })
    }
    fn set(&mut self, name: &str, _value: KnobValue) -> Result<KnobValue, CoreError> {
        Err(CoreError::UnknownKnob { name: name.into() })
    }
}

impl Theory for NewtonianGravity {
    fn id(&self) -> &'static str {
        "newtonian-gravity"
    }
    fn name(&self) -> &'static str {
        "Newtonian gravity"
    }
    fn summary(&self) -> &'static str {
        "Inverse-square gravity and corpuscular light. Grazing solar deflection \
         is 2 GM/(c² R) ≈ 0.87″ (Soldner); bound orbits are closed ellipses. \
         Eddington's 1.75″ and Mercury's 43″ remainder both fail — the standing \
         19th-century theory, mechanized."
    }
    fn world(&self) -> Option<World> {
        Some(World {
            spacetime: Manifold::observed_4d(),
            gauge: GaugeGroup::trivial(),
            spectrum: Spectrum::empty(),
            has_gravity: true,
            supersymmetric: false,
            free_parameter_count: 1, // G
            landscape_log10: 0.0,
            note: "Newtonian inverse-square gravity, corpuscular light".into(),
        })
    }
    fn claims(&self) -> Vec<Claim> {
        solar_claims()
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        eval_solar(false, 4, claim)
    }
}

/// Solar-system gravity lab: Newton vs Einstein.
pub fn gravity() -> ExperimentReport {
    report_from_rows(
        "gravity",
        "Solar-system gravity lab",
        "Does inverse-square gravity survive contact with Eddington's 1.75″ \
         solar deflection and Mercury's 43″ perihelion remainder — or does the \
         standing Newtonian theory fail those theorems, while Schwarzschild \
         geodesics hold them?",
        "Both numbers are RK4 integrals of the Binet equation, checked against \
         2 GM/(c² R), 4 GM/(c² R), and 6π GM/(c² a (1−e²)). Verdicts are \
         internal to the encoding. The 43″ is the *remainder* after Newtonian \
         planetary perturbations, which this lab does not integrate.",
        vec![
            "`gr.newton-half-deflection` is the standing Soldner/1911 claim: it holds for Newton and fails for GR (spatial curvature doubles the angle).".into(),
            "`gr.eddington-deflection` and `gr.mercury-perihelion` are the observations Newtonian gravity fails.".into(),
            "`set general-relativity dim 5` makes the 4D solar tests inapplicable.".into(),
            "GM_☉ is the IAU standard gravitational parameter, so GM/c² is a typed length.".into(),
        ],
        &gravity_rows(),
        vec![
            Box::new(NewtonianGravity),
            Box::new(GeneralRelativity::default()),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;
    use physis_core::Dimensionless;

    fn verdict(t: &dyn Theory, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id_str() == id).unwrap();
        t.evaluate(&c).kind
    }

    #[test]
    fn gm_over_c2_is_a_typed_length() {
        let m: Qty<Length> = solar_m();
        let dimensionless: Qty<Dimensionless> = m / solar_radius();
        assert!(dimensionless.value() > 0.0);
    }

    #[test]
    fn newtonian_light_matches_two_gm_over_c2_r() {
        let num = solar_deflection_rad(false);
        let analytic = 2.0 * solar_m().value() / solar_radius().value();
        assert!(
            (num - analytic).abs() / analytic < 0.01,
            "Newton δ = {num:.6e} vs 2GM/c²R = {analytic:.6e}"
        );
    }

    #[test]
    fn gr_light_matches_four_gm_over_c2_r() {
        let num = solar_deflection_rad(true);
        let analytic = 4.0 * solar_m().value() / solar_radius().value();
        assert!(
            (num - analytic).abs() / analytic < 0.02,
            "GR δ = {num:.6e} vs 4GM/c²R = {analytic:.6e}"
        );
        assert!(
            (num / solar_deflection_rad(false) - 2.0).abs() < 0.03,
            "GR should be twice Newton"
        );
    }

    #[test]
    fn newtonian_orbit_does_not_precess() {
        let extra = mercury_arcsec_per_century(false);
        assert!(
            extra.abs() < 0.5,
            "Newton Δω = {extra:.3} \"/cy (should be ~0)"
        );
    }

    #[test]
    fn gr_mercury_matches_six_pi_formula() {
        let num = mercury_arcsec_per_century(true);
        let analytic = mercury_analytic_arcsec_per_century();
        assert!(
            (num - analytic).abs() < 1.0,
            "GR Δω = {num:.2} vs analytic {analytic:.2}"
        );
        assert!(
            (num - MERCURY_ARCSEC_PER_CENTURY).abs() < 1.5,
            "GR Δω = {num:.2} vs 42.98"
        );
    }

    #[test]
    fn newton_holds_half_angle_and_fails_the_observations() {
        let n = NewtonianGravity;
        assert_eq!(verdict(&n, NEWTON_HALF), VerdictKind::Holds);
        assert_eq!(verdict(&n, EDDINGTON), VerdictKind::Fails);
        assert_eq!(verdict(&n, MERCURY_PERIHELION), VerdictKind::Fails);
    }

    #[test]
    fn gr_fails_half_angle_and_holds_the_observations() {
        let g = GeneralRelativity::default();
        assert_eq!(verdict(&g, NEWTON_HALF), VerdictKind::Fails);
        assert_eq!(verdict(&g, EDDINGTON), VerdictKind::Holds);
        assert_eq!(verdict(&g, MERCURY_PERIHELION), VerdictKind::Holds);
    }

    #[test]
    fn gravity_experiment_puts_newton_on_trial() {
        let r = gravity();
        assert_eq!(r.id, "gravity");
        let cell =
            |claim: &str, theory: &str| r.matrix.get(claim).and_then(|m| m.get(theory)).copied();
        assert_eq!(
            cell(EDDINGTON, "newtonian-gravity"),
            Some(VerdictKind::Fails)
        );
        assert_eq!(
            cell(EDDINGTON, "general-relativity"),
            Some(VerdictKind::Holds)
        );
        assert_eq!(
            cell(NEWTON_HALF, "newtonian-gravity"),
            Some(VerdictKind::Holds)
        );
        assert_eq!(
            cell(NEWTON_HALF, "general-relativity"),
            Some(VerdictKind::Fails)
        );
    }
}
