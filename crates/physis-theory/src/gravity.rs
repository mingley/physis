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
//! the GR term. Default steps are `n = 80_000` (light, `h = π/(2n)`) and
//! `n = 120_000` (Mercury, `h = 2π/n`). Evidence names input (IAU `GM_sun` /
//! `R_sun` exact Ratio conversion rulers; Mercury `a,e` point Qty), rounding
//! (`f64` RK4 / [`physis_numeric::NumericTier::Approximate`]), and
//! integration/truncation. The RK4 remainder is not a certificate: wrapping
//! a coarse point estimate in an interval is not `CertifiedNumeric`. A
//! truncation hull that overlaps the 3% / 1.5″ decision band without
//! containment is `Undecidable` (`numeric unresolved`), not a theorem.
//!
//! The inverse-square rhs lives on the IR package. A Schwarzschild
//! `3 (GM/c²) u²` term is a package mutation (`add-schwarzschild`), not a
//! knob: the Newtonian half-angle fails on the mutant, and Eddington /
//! Mercury hold — still as `newtonian-gravity`, not a silent GR install.
//! A Yukawa `e^{-μr}/r` potential is a second mutation (`add-yukawa`):
//! the impulse-approximation Soldner angle is suppressed by `μR K_1(μR)`
//! and the half-angle fails, while Eddington / Mercury still fail.
//! [`crate::relativity::GeneralRelativity`] stays a separate object;
//! `set general-relativity dim 5` still makes the 4D solar tests
//! inapplicable.

use std::f64::consts::{FRAC_PI_2, PI};

use physis_core::assumption::DomainOfValidity;
use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobSpec, KnobValue, Knobbed};
use physis_core::{EmpiricalStatus, Length, Qty};
use physis_ir::{apply_mutation, parse_package, render_package, PackageMutation, TheoryPackage};
use physis_model::constants::{
    mercury_eccentricity, mercury_orbits_per_century, mercury_semi_major, solar_gm, solar_radius, C,
};
use physis_model::{GaugeGroup, Manifold, Spectrum, World};
use physis_numeric::{residual_relation, Interval, NumericTier, Ratio, ResidualRelation};

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
/// Default RK4 steps for grazing light (`h = π/(2n)`).
pub(crate) const DEFAULT_LIGHT_STEPS: usize = 80_000;
/// Default RK4 steps for Mercury (`h = 2π/n`).
pub(crate) const DEFAULT_MERCURY_STEPS: usize = 120_000;
/// Snap for relative residuals (Eddington 3% band). Not a certificate.
const REL_SNAP: i128 = 1_000_000;
/// Snap for arcsecond residuals (Mercury 1.5″ band). Not a certificate.
const ARCSEC_SNAP: i128 = 1_000;
/// Eddington's GR target, arcseconds (grazing Sun).
pub(crate) const EDDINGTON_ARCSEC: f64 = 1.75;
/// Observed GR perihelion remainder for Mercury, arcseconds per century.
pub(crate) const MERCURY_ARCSEC_PER_CENTURY: f64 = 42.98;
/// Inverse-square Binet rhs on the live Newton package.
const BINET_INVERSE_SQUARE: &str = "binet inverse-square";
/// Schwarzschild Binet term `3 (GM/c²) u²`.
const BINET_SCHWARZSCHILD: &str = "binet 3GM u^2";
/// Yukawa `e^{-μr}/r` potential (impulse-approximation Soldner suppression).
const POTENTIAL_YUKAWA: &str = "potential yukawa";
/// Grazing `μR` for the Yukawa fork. `μR K_1(μR)` at this value is ~0.602,
/// not a unit flag, and is evidence: the encoding fails even if a sampled
/// residual vanished (μ → 0 recovers Newton).
const YUKAWA_MU_R: f64 = 1.0;
/// Euler–Mascheroni constant (A&S 9.6.11).
const EULER_GAMMA: f64 = 0.577_215_664_901_532_9;

fn parse_newton_binet(pkg: &TheoryPackage) -> Result<(bool, bool), String> {
    let mut inverse_square = false;
    let mut schwarzschild = false;
    let mut yukawa = false;
    for eq in &pkg.equations {
        match eq.trim() {
            BINET_INVERSE_SQUARE => inverse_square = true,
            BINET_SCHWARZSCHILD => schwarzschild = true,
            POTENTIAL_YUKAWA => yukawa = true,
            _ => {}
        }
    }
    if !inverse_square {
        return Err(format!(
            "{} package has no inverse-square Binet rhs",
            pkg.id
        ));
    }
    Ok((schwarzschild, yukawa))
}

fn inverse_square_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["inverse-square Binet rhs".into()],
        vec!["Soldner / 1911 corpuscular light; closed Kepler ellipses".into()],
        "Solar-system cells here are the inverse-square Binet encoding. \
         A 3GM u² Schwarzschild term or a Yukawa e^{-μr}/r potential is a \
         new encoding, not a silent GR install.",
    )
}

/// Modified Bessel `K_1(x)` from A&S 9.6.10–11.
fn bessel_k1(x: f64) -> f64 {
    let half = x / 2.0;
    let z = half * half;
    let mut i1_sum = 0.0;
    let mut psi_weighted = 0.0;
    let mut term = 1.0;
    let mut harmonic = 0.0;
    for k in 0..200 {
        let kf = k as f64;
        let h_kp1 = harmonic + 1.0 / (kf + 1.0);
        psi_weighted += (-2.0 * EULER_GAMMA + harmonic + h_kp1) * term;
        i1_sum += term;
        harmonic = h_kp1;
        let next_k = kf + 1.0;
        let next = term * z / (next_k * (next_k + 1.0));
        if k > 8 && next.abs() <= 1e-18 * i1_sum.abs() {
            break;
        }
        term = next;
    }
    let i1 = half * i1_sum;
    i1 * half.ln() + 1.0 / x - (x / 4.0) * psi_weighted
}

/// Impulse-approximation Soldner factor `μR K_1(μR)`. Newton is the μ → 0
/// limit where this tends to 1.
fn yukawa_soldner_factor(mu_r: f64) -> f64 {
    mu_r * bessel_k1(mu_r)
}

/// Residual of the inverse-square half-angle: `1 − μR K_1(μR)` on the
/// Yukawa encoding, 0 on inverse-square. Evidence, not the encoding.
fn yukawa_soldner_residual(yukawa: bool) -> f64 {
    if yukawa {
        1.0 - yukawa_soldner_factor(YUKAWA_MU_R)
    } else {
        0.0
    }
}

fn yukawa_newton_half(claim: &Claim) -> Verdict {
    let x = YUKAWA_MU_R;
    let residual = yukawa_soldner_residual(true);
    let factor = 1.0 - residual;
    Verdict::fails(
        claim,
        "Yukawa e^{-μr}/r suppresses the inverse-square Soldner angle",
    )
    .with_evidence([
        format!("μR K_1(μR) = {factor:.6} (μR = {x}); 1 − μR K_1 = {residual:.6}"),
        "Yukawa Soldner is the K_1 series, not RK4-certified".to_string(),
    ])
}

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
///
/// `n` is the number of RK4 steps in `π/2` of true anomaly
/// (`h = π/(2n)`). Default production value is [`DEFAULT_LIGHT_STEPS`].
fn light_deflection_rad(m: f64, b: f64, gr: bool, n: usize) -> f64 {
    let mut phi = 0.0_f64;
    let mut u = 1.0 / b;
    let mut v = 0.0_f64;
    let n = n.max(1);
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
///
/// `n` is the number of RK4 steps in `2π` of true anomaly (`h = 2π/n`).
/// Default production value is [`DEFAULT_MERCURY_STEPS`].
fn perihelion_advance_rad(m: f64, a: f64, e: f64, gr: bool, n: usize) -> f64 {
    let p = a * (1.0 - e * e);
    let gm_h2 = 1.0 / p; // GM/h² = 1/(a(1−e²)), in 1/metres
    let mut phi = 0.0_f64;
    // Perihelion of the Kepler ellipse: r_min = a(1−e), so u = 1/(a(1−e)).
    let mut u = 1.0 / (a * (1.0 - e));
    let mut v = 0.0_f64;
    let n = n.max(1);
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

/// Grazing solar deflection in radians at the default light step count.
pub fn solar_deflection_rad(gr: bool) -> f64 {
    solar_deflection_rad_n(gr, DEFAULT_LIGHT_STEPS)
}

fn solar_deflection_rad_n(gr: bool, n: usize) -> f64 {
    light_deflection_rad(solar_m().value(), solar_radius().value(), gr, n)
}

/// Grazing solar deflection in arcseconds at the default light step count.
pub fn solar_deflection_arcsec(gr: bool) -> f64 {
    solar_deflection_arcsec_n(gr, DEFAULT_LIGHT_STEPS)
}

/// Grazing solar deflection in arcseconds at an explicit light step count.
pub(crate) fn solar_deflection_arcsec_n(gr: bool, n: usize) -> f64 {
    solar_deflection_rad_n(gr, n) * ARCSEC_PER_RAD
}

/// Mercury extra perihelion, arcseconds per century, at the default step count.
pub fn mercury_arcsec_per_century(gr: bool) -> f64 {
    mercury_arcsec_per_century_n(gr, DEFAULT_MERCURY_STEPS)
}

fn mercury_arcsec_per_century_n(gr: bool, n: usize) -> f64 {
    let m = solar_m().value();
    let a = mercury_semi_major().value();
    let e = mercury_eccentricity().value();
    perihelion_advance_rad(m, a, e, gr, n) * mercury_orbits_per_century() * ARCSEC_PER_RAD
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

fn analytic_two_gm_arcsec() -> f64 {
    2.0 * solar_m().value() / solar_radius().value() * ARCSEC_PER_RAD
}

fn analytic_four_gm_arcsec() -> f64 {
    4.0 * solar_m().value() / solar_radius().value() * ARCSEC_PER_RAD
}

/// Diagnostic RK4 truncation half-width: one `h^4` radian in the Binet
/// independent variable, converted to the observable. This is the RK4
/// global-order scaling, **not** a remainder certificate.
fn light_truncation_arcsec(n: usize) -> f64 {
    let h = FRAC_PI_2 / (n.max(1) as f64);
    h.powi(4) * ARCSEC_PER_RAD
}

fn mercury_truncation_arcsec(n: usize) -> f64 {
    let h = (2.0 * PI) / (n.max(1) as f64);
    h.powi(4) * mercury_orbits_per_century() * ARCSEC_PER_RAD
}

fn snapped_hull(center: f64, halfwidth: f64, den: i128) -> Interval {
    let c = Ratio::nearest(center, den);
    let mut w = Ratio::nearest(halfwidth.abs(), den);
    if w.is_zero() {
        w = Ratio::new(1, den);
    }
    Interval::new(c - w, c + w)
}

fn relative_residual_hull(value: f64, target: f64, abs_halfwidth: f64) -> Interval {
    let scale = target.abs().max(1e-12);
    snapped_hull((value - target) / scale, abs_halfwidth / scale, REL_SNAP)
}

fn arcsec_residual_hull(value: f64, target: f64, halfwidth: f64) -> Interval {
    snapped_hull(value - target, halfwidth, ARCSEC_SNAP)
}

fn eddington_rel_band() -> Interval {
    Interval::new(Ratio::new(-3, 100), Ratio::new(3, 100))
}

fn mercury_abs_band() -> Interval {
    Interval::new(Ratio::new(-3, 2), Ratio::new(3, 2))
}

fn solar_error_budget_evidence(n_light: usize, n_mercury: usize) -> Vec<String> {
    let gm = physis_constants::lookup("GM_sun").expect("GM_sun is on LEDGER");
    let rsun = physis_constants::lookup("R_sun").expect("R_sun is on LEDGER");
    vec![
        format!(
            "input: IAU GM_sun exact Ratio conversion ruler (not measured solar mass); {} {} hash {}",
            gm.kind,
            gm.value,
            gm.hash.to_hex()
        ),
        format!(
            "input: IAU R_sun exact Ratio conversion ruler (not measured solar radius); {} {} hash {}",
            rsun.kind,
            rsun.value,
            rsun.hash.to_hex()
        ),
        "input: Mercury a,e are point Qty, no hull, not a dataset".into(),
        format!(
            "rounding: f64 RK4 / NumericTier::Approximate ({}); not a certificate",
            NumericTier::Approximate.as_str()
        ),
        format!(
            "integration/truncation: light n={n_light}, h=π/(2n); Mercury n={n_mercury}, h=2π/n; RK4 remainder is not a certificate"
        ),
    ]
}

fn decide_truncation_residual(
    claim: &Claim,
    relation: ResidualRelation,
    residual: Interval,
    hold_summary: impl Into<String>,
    fail_summary: impl Into<String>,
    evidence: impl IntoIterator<Item = String>,
) -> Verdict {
    let v = match relation {
        ResidualRelation::Contained => Verdict::holds(claim, hold_summary),
        ResidualRelation::Disjoint => Verdict::fails(claim, fail_summary),
        ResidualRelation::OverlapsWithoutContainment => Verdict::undecidable(
            claim,
            "truncation residual overlaps the decision band without containment; overlap is not equality",
        )
        .with_empirical(EmpiricalStatus::Inconclusive),
    };
    v.with_evidence(evidence)
        .with_interval_enclosure(residual.lo.to_string(), residual.hi.to_string())
}

/// Evaluate the three solar-system claims for Newton (`gr = false`) or GR.
pub fn eval_solar(gr: bool, dim: u8, claim: &Claim) -> Verdict {
    eval_solar_with_steps(gr, dim, claim, DEFAULT_LIGHT_STEPS, DEFAULT_MERCURY_STEPS)
}

/// Solar-system evaluator with explicit RK4 step counts.
///
/// Production [`eval_solar`] uses [`DEFAULT_LIGHT_STEPS`] / [`DEFAULT_MERCURY_STEPS`].
/// A coarse `n` is a truncation control, not a knob: overlap of the diagnostic
/// remainder hull with the 3% / 1.5″ band without containment is `Undecidable`,
/// never [`physis_core::DerivationAssurance::CertifiedNumeric`].
pub(crate) fn eval_solar_with_steps(
    gr: bool,
    dim: u8,
    claim: &Claim,
    n_light: usize,
    n_mercury: usize,
) -> Verdict {
    if !solar_ready(dim) {
        return Verdict::inapplicable(
            claim,
            "solar-system tests are 4D Schwarzschild / inverse-square",
        );
    }
    match claim.id_str() {
        NEWTON_HALF => {
            let delta = solar_deflection_arcsec_n(gr, n_light);
            let newton = solar_deflection_arcsec_n(false, n_light);
            let mut evidence = solar_error_budget_evidence(n_light, n_mercury);
            // Compare against the computed Newtonian integral at the same n,
            // not a table. Truncation is shared, so this is not a remainder
            // certificate of either integral.
            if (delta - newton).abs() / newton.max(1e-12) < 0.02 && newton > 0.5 {
                evidence.push(format!(
                    "δ = {delta:.4}\" (Newtonian integral {newton:.4}\"; analytic 2GM/c²R = {:.4}\")",
                    analytic_two_gm_arcsec()
                ));
                Verdict::holds(claim, "grazing deflection is the Newtonian 2 GM/(c² R)")
                    .with_evidence(evidence)
            } else {
                evidence.push(format!(
                    "δ = {delta:.4}\" vs Newtonian {newton:.4}\" (GR analytic 4GM/c²R = {:.4}\")",
                    analytic_four_gm_arcsec()
                ));
                Verdict::fails(
                    claim,
                    "deflection is not the Newtonian half-angle (spatial curvature doubles it)",
                )
                .with_evidence(evidence)
            }
        }
        EDDINGTON => {
            let delta = solar_deflection_arcsec_n(gr, n_light);
            let residual =
                relative_residual_hull(delta, EDDINGTON_ARCSEC, light_truncation_arcsec(n_light));
            let band = eddington_rel_band();
            let relation = residual_relation(residual, band);
            let mut evidence = solar_error_budget_evidence(n_light, n_mercury);
            evidence.push(format!(
                "δ = {delta:.4}\" (analytic 4GM/c²R = {:.4}\"; Newtonian 2GM/c²R = {:.4}\")",
                analytic_four_gm_arcsec(),
                analytic_two_gm_arcsec()
            ));
            evidence.push(format!(
                "truncation residual hull {residual} vs relative 3% Eddington band {band} ({relation}); overlap is not equality"
            ));
            decide_truncation_residual(
                claim,
                relation,
                residual,
                "grazing solar deflection is 1.75″ (Eddington / Schwarzschild)",
                "grazing deflection is not 1.75″ (Newtonian / 1911 half-angle)",
                evidence,
            )
        }
        MERCURY_PERIHELION => {
            let extra = mercury_arcsec_per_century_n(gr, n_mercury);
            let analytic = mercury_analytic_arcsec_per_century();
            let residual = arcsec_residual_hull(
                extra,
                MERCURY_ARCSEC_PER_CENTURY,
                mercury_truncation_arcsec(n_mercury),
            );
            let band = mercury_abs_band();
            let relation = residual_relation(residual, band);
            let mut evidence = solar_error_budget_evidence(n_light, n_mercury);
            evidence.push(format!(
                "Δω = {extra:.2}\" / century (analytic 6π GM/(c²a(1−e²)) = {analytic:.2}\"/cy)"
            ));
            evidence.push(format!(
                "truncation residual hull {residual} vs 1.5″ Mercury band {band} ({relation}); overlap is not equality"
            ));
            decide_truncation_residual(
                claim,
                relation,
                residual,
                "Mercury's extra perihelion is 43″ per century",
                "inverse-square ellipses do not precess; the 43″ remainder is missing",
                evidence,
            )
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
///
/// The Binet rhs lives on the IR package. A Schwarzschild `3GM u²` term
/// is a package mutation (`add-schwarzschild`), not a knob: the half-angle
/// fails on the mutant and Eddington / Mercury hold. A Yukawa `e^{-μr}/r`
/// potential is a second mutation (`add-yukawa`): `μR K_1(μR)` suppresses
/// Soldner and the half-angle fails, while Eddington / Mercury still fail.
/// Those forks are still this object, not a silent GR install. GR keeps `dim`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct NewtonianGravity {
    schwarzschild: bool,
    yukawa: bool,
}

impl NewtonianGravity {
    /// IR package for this Binet rhs. Equations are `binet inverse-square`
    /// and, when forked, `binet 3GM u^2` and/or `potential yukawa`.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![BINET_INVERSE_SQUARE.to_string()];
        if self.schwarzschild {
            equations.push(BINET_SCHWARZSCHILD.to_string());
        }
        if self.yukawa {
            equations.push(POTENTIAL_YUKAWA.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["inverse-square-binet".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: NEWTON_HALF.into(),
                statement:
                    "Grazing solar light deflection is the Newtonian value 2 GM/(c² R) ≈ 0.87″."
                        .into(),
                layer: "spacetime".into(),
                class: "model-internal".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load a Binet encoding from a package.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        if pkg.id != "newtonian-gravity" {
            return Err(format!(
                "newtonian-gravity package id '{}' is not newtonian-gravity",
                pkg.id
            ));
        }
        let (schwarzschild, yukawa) = parse_newton_binet(pkg)?;
        Ok(Self {
            schwarzschild,
            yukawa,
        })
    }

    fn schwarzschild_equation() -> String {
        BINET_SCHWARZSCHILD.to_string()
    }

    fn yukawa_equation() -> String {
        POTENTIAL_YUKAWA.to_string()
    }
}

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
         Eddington's 1.75″ and Mercury's 43″ remainder both fail. A \
         Schwarzschild 3GM u² term is an IR mutation, not a knob. A Yukawa \
         e^{-μr}/r potential is a second IR mutation: it suppresses Soldner \
         and is not a silent GR install."
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
            note: if self.yukawa {
                "Newtonian encoding with Yukawa e^{-μr}/r potential".into()
            } else if self.schwarzschild {
                "Newtonian encoding with Schwarzschild 3GM u² Binet term".into()
            } else {
                "Newtonian inverse-square gravity, corpuscular light".into()
            },
        })
    }
    fn claims(&self) -> Vec<Claim> {
        solar_claims()
            .into_iter()
            .map(|c| c.with_domain(inverse_square_domain()))
            .collect()
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        if self.yukawa {
            match claim.id_str() {
                NEWTON_HALF => yukawa_newton_half(claim),
                _ => eval_solar(false, 4, claim),
            }
        } else {
            eval_solar(self.schwarzschild, 4, claim)
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.schwarzschild = parsed.schwarzschild;
        fork.yukawa = parsed.yukawa;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mut out: Vec<(String, Box<dyn Theory>)> = Vec::new();
        if !self.schwarzschild {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::schwarzschild_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.schwarzschild {
                    let mut fork = self.clone();
                    fork.schwarzschild = true;
                    out.push(("add-schwarzschild".into(), Box::new(fork)));
                }
            }
        }
        if !self.yukawa {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::yukawa_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.yukawa {
                    let mut fork = self.clone();
                    fork.yukawa = true;
                    out.push(("add-yukawa".into(), Box::new(fork)));
                }
            }
        }
        out
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
         2 GM/(c² R), 4 GM/(c² R), and 6π GM/(c² a (1−e²)). Evidence names \
         input, rounding, and truncation; the RK4 remainder is not a \
         certificate. Verdicts are internal to the encoding. The 43″ is the \
         *remainder* after Newtonian planetary perturbations, which this lab \
         does not integrate.",
        vec![
            "`gr.newton-half-deflection` is the standing Soldner/1911 claim: it holds for Newton and fails for GR (spatial curvature doubles the angle).".into(),
            "`gr.eddington-deflection` and `gr.mercury-perihelion` are the observations Newtonian gravity fails.".into(),
            "`hypothesize newtonian-gravity`: add-schwarzschild and add-yukawa are IR, not set. GR stays a separate object.".into(),
            "`hypothesize general-relativity`: add-r-squared and add-brans-dicke are IR, not set. dim stays on GR.".into(),
            "`set general-relativity dim 5` makes the 4D solar tests inapplicable.".into(),
            "GM_☉ is the IAU standard gravitational parameter, so GM/c² is a typed length.".into(),
        ],
        &gravity_rows(),
        vec![
            Box::new(NewtonianGravity::default()),
            Box::new(GeneralRelativity::default()),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;
    use physis_core::{DerivationAssurance, Dimensionless, EmpiricalStatus};
    use physis_numeric::{Interval, Ratio};

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
        let n = NewtonianGravity::default();
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

    #[test]
    fn schwarzschild_binet_is_ir_not_a_knob() {
        let t = NewtonianGravity::default();
        assert!(
            NewtonianGravity::default()
                .set("schwarzschild", KnobValue::Bool(true))
                .is_err(),
            "Schwarzschild Binet term is an IR mutation, not a knob"
        );
        assert!(
            NewtonianGravity::default()
                .set("dim", KnobValue::UInt(5))
                .is_err(),
            "Newton must not grow a dim knob; that stays on GR"
        );
        let src = render_package(&t.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            NewtonianGravity::from_package(&pkg).unwrap(),
            t,
            "IR round-trip must preserve the inverse-square Binet rhs"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(NewtonianGravity::schwarzschild_equation()),
        );
        let parsed = NewtonianGravity::from_package(&mutated).unwrap();
        assert!(parsed.schwarzschild);
        let mut fork = t.clone();
        fork.schwarzschild = true;
        assert_eq!(fork.id(), "newtonian-gravity");
        assert_eq!(verdict(&fork, NEWTON_HALF), VerdictKind::Fails);
        assert_eq!(verdict(&fork, EDDINGTON), VerdictKind::Holds);
        assert_eq!(verdict(&fork, MERCURY_PERIHELION), VerdictKind::Holds);
        assert_eq!(verdict(&t, NEWTON_HALF), VerdictKind::Holds);
        assert_eq!(verdict(&t, EDDINGTON), VerdictKind::Fails);
        assert_eq!(verdict(&t, MERCURY_PERIHELION), VerdictKind::Fails);
        let probes = NewtonianGravity::default().structural_mutations();
        assert_eq!(probes.len(), 2);
        assert!(
            probes.iter().any(|(label, _)| label == "add-schwarzschild"),
            "live Newton must offer add-schwarzschild: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        assert!(
            probes.iter().any(|(label, _)| label == "add-yukawa"),
            "live Newton must offer add-yukawa: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let schwarzschild_probe = probes
            .iter()
            .find(|(label, _)| label == "add-schwarzschild")
            .expect("add-schwarzschild");
        assert_eq!(schwarzschild_probe.1.id(), "newtonian-gravity");
        assert_eq!(
            verdict(schwarzschild_probe.1.as_ref(), NEWTON_HALF),
            VerdictKind::Fails
        );
        assert_eq!(
            verdict(schwarzschild_probe.1.as_ref(), EDDINGTON),
            VerdictKind::Holds
        );
        let schwarzschild_fork_probes = fork.structural_mutations();
        assert!(
            schwarzschild_fork_probes
                .iter()
                .all(|(label, _)| label != "add-schwarzschild"),
            "schwarzschild fork must not re-offer add-schwarzschild"
        );
        assert!(
            schwarzschild_fork_probes
                .iter()
                .any(|(label, _)| label == "add-yukawa"),
            "schwarzschild fork must still offer add-yukawa"
        );
        let live = NewtonianGravity::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), live.package());
        assert_eq!(verdict(rebuilt.as_ref(), NEWTON_HALF), VerdictKind::Holds);
        let half = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == NEWTON_HALF)
            .unwrap();
        assert!(
            !half.domain().is_encoding_wide(),
            "Newton half-angle must name the inverse-square Binet rhs: {:?}",
            half.domain()
        );
        let gr = GeneralRelativity::default();
        let gr_half = gr
            .claims()
            .into_iter()
            .find(|c| c.id_str() == NEWTON_HALF)
            .unwrap();
        assert!(
            gr_half.domain().is_encoding_wide(),
            "GR solar cells stay encoding-wide: {:?}",
            gr_half.domain()
        );
        let mut high_d = GeneralRelativity::default();
        high_d.set("dim", KnobValue::UInt(5)).unwrap();
        assert_eq!(verdict(&high_d, EDDINGTON), VerdictKind::Inapplicable);
        assert_eq!(verdict(&live, EDDINGTON), VerdictKind::Fails);
    }

    #[test]
    fn bessel_k1_matches_known_values() {
        assert!(
            (bessel_k1(1.0) - 0.601_907_230_197_234_6).abs() < 1e-12,
            "K_1(1) = {}",
            bessel_k1(1.0)
        );
        assert!(
            (bessel_k1(2.0) - 0.139_865_881_816_522_4).abs() < 1e-12,
            "K_1(2) = {}",
            bessel_k1(2.0)
        );
        let tiny = 1e-9 * bessel_k1(1e-9);
        assert!(
            (tiny - 1.0).abs() < 1e-6,
            "μR K_1(μR) → 1 as μ → 0, got {tiny}"
        );
    }

    #[test]
    fn yukawa_potential_is_ir_not_a_knob() {
        let t = NewtonianGravity::default();
        assert!(
            NewtonianGravity::default()
                .set("yukawa", KnobValue::Bool(true))
                .is_err(),
            "Yukawa potential is an IR mutation, not a knob"
        );
        assert!(
            NewtonianGravity::default()
                .set("mu", KnobValue::Float(1.0))
                .is_err(),
            "μ is not a knob"
        );
        assert!(
            NewtonianGravity::default()
                .set("dim", KnobValue::UInt(5))
                .is_err(),
            "Newton must not grow a dim knob; that stays on GR"
        );
        let src = render_package(&t.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            pkg.equations.len(),
            1,
            "live package must stay inverse-square Binet"
        );
        assert_eq!(pkg.equations[0], BINET_INVERSE_SQUARE);
        assert_eq!(
            NewtonianGravity::from_package(&pkg).unwrap(),
            t,
            "IR round-trip must preserve the inverse-square Binet rhs"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(NewtonianGravity::yukawa_equation()),
        );
        let parsed = NewtonianGravity::from_package(&mutated).unwrap();
        assert!(parsed.yukawa);
        assert!(!parsed.schwarzschild);
        let mut fork = t.clone();
        fork.yukawa = true;
        assert_eq!(fork.id(), "newtonian-gravity");
        assert_eq!(verdict(&fork, NEWTON_HALF), VerdictKind::Fails);
        assert_eq!(verdict(&fork, EDDINGTON), VerdictKind::Fails);
        assert_eq!(verdict(&fork, MERCURY_PERIHELION), VerdictKind::Fails);
        assert_eq!(verdict(&t, NEWTON_HALF), VerdictKind::Holds);
        assert_eq!(verdict(&t, EDDINGTON), VerdictKind::Fails);
        assert_eq!(verdict(&t, MERCURY_PERIHELION), VerdictKind::Fails);
        let r = yukawa_soldner_residual(true);
        assert!(
            (r - 0.398_092_769_802_765_4).abs() < 1e-9,
            "Soldner residual must be 1 − μR K_1(μR) at μR=1, got {r}"
        );
        assert!(
            (r - 1.0).abs() > 0.3,
            "Soldner residual must be the K_1 scale, not a unit flag, got {r}"
        );
        assert_eq!(yukawa_soldner_residual(false), 0.0);
        let half = fork
            .claims()
            .into_iter()
            .find(|c| c.id_str() == NEWTON_HALF)
            .unwrap();
        let v = fork.evaluate(&half);
        assert!(
            !v.summary.contains("spatial curvature"),
            "Yukawa is not GR spatial curvature: {}",
            v.summary
        );
        assert!(
            v.evidence.iter().any(|e| e.contains("K_1")),
            "Yukawa evidence must report μR K_1: {:?}",
            v.evidence
        );
        let probes = NewtonianGravity::default().structural_mutations();
        assert!(
            probes.iter().any(|(label, _)| label == "add-yukawa"),
            "live Newton must offer add-yukawa: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let yukawa_probe = probes
            .iter()
            .find(|(label, _)| label == "add-yukawa")
            .expect("add-yukawa");
        assert_eq!(
            verdict(yukawa_probe.1.as_ref(), NEWTON_HALF),
            VerdictKind::Fails
        );
        assert_eq!(
            verdict(yukawa_probe.1.as_ref(), EDDINGTON),
            VerdictKind::Fails
        );
        let yukawa_fork_probes = fork.structural_mutations();
        assert!(
            yukawa_fork_probes
                .iter()
                .all(|(label, _)| label != "add-yukawa"),
            "yukawa fork must not re-offer add-yukawa"
        );
        assert!(
            yukawa_fork_probes
                .iter()
                .any(|(label, _)| label == "add-schwarzschild"),
            "yukawa fork must still offer add-schwarzschild"
        );
        let live = NewtonianGravity::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), live.package());
        assert_eq!(verdict(rebuilt.as_ref(), NEWTON_HALF), VerdictKind::Holds);
        let half = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == NEWTON_HALF)
            .unwrap();
        assert!(
            !half.domain().is_encoding_wide(),
            "Newton half-angle must name the inverse-square Binet rhs: {:?}",
            half.domain()
        );
        let gr = GeneralRelativity::default();
        assert!(
            gr.structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-yukawa"),
            "GR must not grow add-yukawa"
        );
        assert!(
            gr.structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-schwarzschild"),
            "GR must not grow add-schwarzschild"
        );
        assert!(
            gr.structural_mutations()
                .iter()
                .any(|(label, _)| label == "add-r-squared"),
            "GR must offer add-r-squared on its own Einstein-Hilbert package"
        );
        let mut high_d = GeneralRelativity::default();
        high_d.set("dim", KnobValue::UInt(5)).unwrap();
        assert_eq!(verdict(&high_d, EDDINGTON), VerdictKind::Inapplicable);
        assert_eq!(
            high_d.get("dim").unwrap(),
            KnobValue::UInt(5),
            "dim stays on GR"
        );
    }

    fn solar_claim(id: &str) -> Claim {
        solar_claims()
            .into_iter()
            .find(|c| c.id_str() == id)
            .unwrap()
    }

    fn evidence_blob(v: &Verdict) -> String {
        v.evidence.join("\n")
    }

    fn assert_solar_error_budget(blob: &str, n_light: usize, n_mercury: usize) {
        assert!(
            blob.contains("GM_sun") && blob.contains("R_sun"),
            "input must name IAU GM_sun and R_sun: {blob}"
        );
        assert!(
            blob.contains("conversion ruler") || blob.contains("conversion rulers"),
            "GM_sun/R_sun are conversion rulers, not measured mass/radius: {blob}"
        );
        assert!(
            blob.contains("not measured"),
            "IAU rulers are not measured solar mass/radius: {blob}"
        );
        assert!(
            blob.contains("exact Ratio"),
            "GM_sun/R_sun are exact Ratio rulers: {blob}"
        );
        assert!(
            blob.contains("Mercury")
                && blob.contains("point")
                && blob.contains("no hull")
                && blob.contains("not a dataset"),
            "Mercury a,e are point Qty, no hull, not a dataset: {blob}"
        );
        assert!(
            blob.contains("rounding")
                && blob.contains("f64")
                && (blob.contains("Approximate") || blob.contains("approximate")),
            "rounding must name f64 RK4 / NumericTier::Approximate: {blob}"
        );
        assert!(
            blob.contains(&format!("n={n_light}")) && blob.contains("π/(2n)"),
            "light truncation must name n={n_light}, h=π/(2n): {blob}"
        );
        assert!(
            blob.contains(&format!("n={n_mercury}")) && blob.contains("2π/n"),
            "Mercury truncation must name n={n_mercury}, h=2π/n: {blob}"
        );
        assert!(
            blob.contains("not a certificate"),
            "RK4 remainder is not a certificate: {blob}"
        );
    }

    fn assert_not_certified(v: &Verdict) {
        assert_eq!(v.derivation(), DerivationAssurance::Executed);
        assert_ne!(v.derivation(), DerivationAssurance::CertifiedNumeric);
        assert_ne!(v.empirical(), EmpiricalStatus::Compatible);
    }

    #[test]
    fn default_solar_evaluations_name_input_rounding_and_truncation() {
        let newton = NewtonianGravity::default();
        let gr = GeneralRelativity::default();
        for (theory, id) in [
            (&newton as &dyn Theory, NEWTON_HALF),
            (&newton, EDDINGTON),
            (&newton, MERCURY_PERIHELION),
            (&gr, NEWTON_HALF),
            (&gr, EDDINGTON),
            (&gr, MERCURY_PERIHELION),
        ] {
            let c = theory
                .claims()
                .into_iter()
                .find(|c| c.id_str() == id)
                .unwrap();
            let v = theory.evaluate(&c);
            assert_not_certified(&v);
            assert_solar_error_budget(
                &evidence_blob(&v),
                DEFAULT_LIGHT_STEPS,
                DEFAULT_MERCURY_STEPS,
            );
        }
        let n = NewtonianGravity::default();
        assert_eq!(verdict(&n, NEWTON_HALF), VerdictKind::Holds);
        assert_eq!(verdict(&n, EDDINGTON), VerdictKind::Fails);
        assert_eq!(verdict(&n, MERCURY_PERIHELION), VerdictKind::Fails);
        let g = GeneralRelativity::default();
        assert_eq!(verdict(&g, NEWTON_HALF), VerdictKind::Fails);
        assert_eq!(verdict(&g, EDDINGTON), VerdictKind::Holds);
        assert_eq!(verdict(&g, MERCURY_PERIHELION), VerdictKind::Holds);
    }

    #[test]
    fn coarse_rk4_step_is_not_certified_numeric() {
        let eddington = solar_claim(EDDINGTON);
        let mercury = solar_claim(MERCURY_PERIHELION);
        let newton_half = solar_claim(NEWTON_HALF);

        let coarse_gr_light = eval_solar_with_steps(true, 4, &eddington, 50, DEFAULT_MERCURY_STEPS);
        assert_not_certified(&coarse_gr_light);
        assert_solar_error_budget(&evidence_blob(&coarse_gr_light), 50, DEFAULT_MERCURY_STEPS);
        assert_eq!(
            coarse_gr_light.kind,
            VerdictKind::Undecidable,
            "coarse GR Eddington truncation must straddle the 3% band: {} {:?}",
            coarse_gr_light.summary,
            coarse_gr_light.evidence
        );
        assert_eq!(coarse_gr_light.empirical(), EmpiricalStatus::Inconclusive);
        let j = physis_core::judgment::Judgment::from_lab(
            coarse_gr_light.class,
            coarse_gr_light.kind,
            coarse_gr_light.empirical(),
            coarse_gr_light.derivation(),
            false,
            coarse_gr_light.numeric_lo(),
            coarse_gr_light.numeric_hi(),
            coarse_gr_light.statistical_nll(),
        );
        assert_eq!(j.label(), "numeric unresolved");

        let coarse_newton_eddington =
            eval_solar_with_steps(false, 4, &eddington, 50, DEFAULT_MERCURY_STEPS);
        assert_not_certified(&coarse_newton_eddington);
        assert_eq!(
            coarse_newton_eddington.kind,
            VerdictKind::Fails,
            "coarse Newton-on-Eddington stays disjoint from 1.75″: {} {:?}",
            coarse_newton_eddington.summary,
            coarse_newton_eddington.evidence
        );

        let coarse_gr_mercury = eval_solar_with_steps(true, 4, &mercury, DEFAULT_LIGHT_STEPS, 200);
        assert_not_certified(&coarse_gr_mercury);
        assert_eq!(
            coarse_gr_mercury.kind,
            VerdictKind::Undecidable,
            "coarse GR Mercury truncation must straddle the 1.5″ band: {} {:?}",
            coarse_gr_mercury.summary,
            coarse_gr_mercury.evidence
        );
        assert_eq!(coarse_gr_mercury.empirical(), EmpiricalStatus::Inconclusive);

        let coarse_newton_half =
            eval_solar_with_steps(false, 4, &newton_half, 50, DEFAULT_MERCURY_STEPS);
        assert_not_certified(&coarse_newton_half);

        let delta = solar_deflection_arcsec_n(true, 50);
        let wrapped = Interval::point(Ratio::nearest(delta, 100));
        let forged = Verdict::holds(&eddington, "coarse point wrap")
            .with_certified_numeric(wrapped.lo.to_string(), wrapped.hi.to_string());
        assert_eq!(forged.derivation(), DerivationAssurance::CertifiedNumeric);
        assert_ne!(
            coarse_gr_light.derivation(),
            forged.derivation(),
            "shipped coarse path must not mint CertifiedNumeric by wrapping a point"
        );
        assert_ne!(coarse_gr_light.kind, VerdictKind::Holds);
        let _ = light_deflection_rad(solar_m().value(), solar_radius().value(), true, 50);
        let _ = perihelion_advance_rad(
            solar_m().value(),
            mercury_semi_major().value(),
            mercury_eccentricity().value(),
            true,
            200,
        );
    }
}
