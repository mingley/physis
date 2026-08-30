//! Olbers' paradox: an infinite static Euclidean sky on trial.
//!
//! A standing 19th-century cosmology says space is infinite, eternal, and
//! uniformly filled with stars. Inverse-square dilution of each star is
//! cancelled by the area of the spherical shell it sits on, so the flux
//! contribution of a shell is independent of radius: `dF = ρ_L dr`. Integrate
//! to infinity and the night sky is as bright as a stellar surface.
//!
//! Two lab objects share this encoding:
//! - [`OlbersSky::static_euclidean`] — the standing theory (`olbers-static`)
//! - [`OlbersSky::finite_age`] — a horizon at `c t` (`olbers-horizon`)
//!
//! `set olbers-static finite_age true` is the finite-age resolution.
//! `set olbers-static expanding true` is Hubble dimming: `dF = ρ_L dr /
//! (1 + H r/c)²` converges as `R → ∞` to `ρ_L c / H`. The two resolutions
//! are independent knob turns. Optical depth `τ = n σ R` uses cosmic mean
//! starlight and the solar disk; `τ → ∞` only for the improper static
//! integral, not for a merely large cutoff (the same lesson as Planck's
//! improper `u_∞`).
//!
//! Inverse-square Euclidean shells live on the IR package of
//! `olbers-static`. Tired light (`add-tired-light`) is a package
//! mutation, not a `finite_age` or `expanding` knob: `dF/dr` falls
//! exponentially so cancellation fails and the energy integral
//! converges, while covering `τ = n σ R` still diverges. That is not
//! Hubble dimming (which also darkens the sky). `olbers-horizon` has
//! no package.

use std::f64::consts::PI;

use physis_core::assumption::DomainOfValidity;
use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::qty::{kelvin, seconds, Qty};
use physis_core::ParameterOrigin;
use physis_core::{Dimensionless, Irradiance, Length};
use physis_ir::{apply_mutation, parse_package, render_package, PackageMutation, TheoryPackage};
use physis_model::constants::{
    cosmic_luminosity_density, hubble_constant, solar_luminosity, solar_radius,
    stefan_boltzmann_constant, C,
};
use physis_model::World;

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;

/// Inverse-square dilution cancels shell area, so `dF/dr` is independent of `r`.
pub const SHELL_CANCELLATION: &str = "astro.shell-cancellation";
/// Integrated sky brightness stays finite as the radial cutoff is removed.
pub const SKY_FINITE: &str = "astro.sky-finite";
/// The night sky is far dimmer than a stellar photosphere (`τ ≪ 1`).
pub const NIGHT_SKY_DARK: &str = "astro.night-sky-dark";

/// Matrix rows for the Olbers lab.
pub fn olbers_rows() -> [&'static str; 3] {
    [SHELL_CANCELLATION, SKY_FINITE, NIGHT_SKY_DARK]
}

/// Julian year, seconds.
const SECONDS_PER_YEAR: f64 = 365.25 * 86_400.0;
/// Default cosmic age (years): a Hubble-time universe.
const DEFAULT_AGE_YR: f64 = 13.8e9;
/// Default radial cutoff (metres): larger than the Hubble length, so a finite
/// age actually *caps* the integral rather than the cutoff doing it.
const DEFAULT_CUTOFF_M: f64 = 1.0e28;
/// Photosphere temperature used to compare sky brightness to a star (kelvin).
const STAR_T_K: f64 = 5772.0;
/// Optical depth below this counts as a dark sky.
const TAU_DARK: f64 = 0.01;
/// Sky / photosphere irradiance ratio below this counts as dark.
const IRRADIANCE_DARK: f64 = 1.0e-6;

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "finite_age",
        layer: LayerId::Spacetime,
        doc: "If true, light has only travelled a distance c t. If false, the static Euclidean integral has no horizon. Turning this on is the finite-age resolution of Olbers' paradox. Tired light is not this knob: add-tired-light is an IR mutation on olbers-static.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Bool,
    },
    KnobSpec {
        name: "expanding",
        layer: LayerId::Spacetime,
        doc: "If true, shells are Hubble-dimmed: dF = ρ_L dr / (1 + H r/c)² (linear Hubble flow, not a full FLRW integral). An independent resolution: the improper integral saturates at ρ_L c/H. Tired light is not this knob: add-tired-light is an IR mutation on olbers-static.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Bool,
    },
    KnobSpec {
        name: "age_yr",
        layer: LayerId::Spacetime,
        doc: "Cosmic age in years. Used only when finite_age is true. Making the universe old enough that τ = n σ c t ≳ 1 makes the sky bright again.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 1.0e6,
            max: 1.0e30,
        },
    },
    KnobSpec {
        name: "cutoff_m",
        layer: LayerId::Effective,
        doc: "Radial cutoff in metres. Standing-theory verdicts use the improper R → ∞ limit, not this cutoff (a large but finite cutoff can still look dark).",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 1.0e18,
            max: 1.0e40,
        },
    },
];

/// Live Euclidean shell law on the `olbers-static` package.
const SHELL_EQ: &str = "dF = rho dr";
/// Tired-light encoding: exponential energy loss, covering still diverges.
const TIRED_EQ: &str = "tired light";

fn parse_olbers_shell(pkg: &TheoryPackage) -> Result<bool, String> {
    let mut shell = false;
    let mut tired = false;
    for eq in &pkg.equations {
        match eq.trim() {
            SHELL_EQ => shell = true,
            TIRED_EQ => tired = true,
            _ => {}
        }
    }
    if !shell {
        return Err(format!(
            "{} package has no inverse-square Euclidean shell law",
            pkg.id
        ));
    }
    Ok(tired)
}

fn shell_cancellation_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["inverse-square Euclidean shells".into()],
        vec!["dF = rho dr independent of r".into()],
        "Cancellation here is the inverse-square Euclidean shell law. Tired \
         light is a new encoding, not a silent expanding or finite_age knob.",
    )
}

/// A Euclidean universe uniformly filled with starlight: Olbers or a horizon.
///
/// Inverse-square shells live on the IR package of `olbers-static`.
/// Tired light (`add-tired-light`) is a package mutation, not a knob.
/// `finite_age` / `expanding` stay knobs. `olbers-horizon` has no package.
#[derive(Clone, Debug, PartialEq)]
pub struct OlbersSky {
    id: &'static str,
    finite_age: bool,
    expanding: bool,
    age_yr: f64,
    cutoff_m: f64,
    /// Whether the encoding is tired light (`dF ∝ e^{-Hr/c} dr`).
    tired: bool,
}

impl Default for OlbersSky {
    fn default() -> Self {
        Self::finite_age()
    }
}

impl OlbersSky {
    /// Finite cosmic age: light has only travelled `c t`.
    pub fn finite_age() -> Self {
        Self {
            id: "olbers-horizon",
            finite_age: true,
            expanding: false,
            age_yr: DEFAULT_AGE_YR,
            cutoff_m: DEFAULT_CUTOFF_M,
            tired: false,
        }
    }

    /// Infinite, eternal, static Euclidean space. The standing 19th-century theory.
    pub fn static_euclidean() -> Self {
        Self {
            id: "olbers-static",
            finite_age: false,
            expanding: false,
            age_yr: DEFAULT_AGE_YR,
            cutoff_m: DEFAULT_CUTOFF_M,
            tired: false,
        }
    }

    fn age(&self) -> Qty<physis_core::Time> {
        seconds(self.age_yr * SECONDS_PER_YEAR)
    }

    /// Light-travel horizon `c t`.
    fn age_horizon(&self) -> Qty<Length> {
        C * self.age()
    }

    /// Hubble length `c / H₀`.
    fn hubble_length(&self) -> Qty<Length> {
        C / hubble_constant()
    }

    /// Radius used by notes and dark-sky / finite-sky verdicts (not the cutoff).
    ///
    /// Finite age: `c t`. Expanding, infinite age: `c/H`. The `cutoff_m` knob
    /// is not this radius — standing-theory verdicts use the improper limit,
    /// and a finite-age sky is judged at `c t` even if the cutoff is smaller.
    fn verdict_radius(&self) -> Qty<Length> {
        if self.finite_age {
            self.age_horizon()
        } else {
            self.hubble_length()
        }
    }

    /// Unocculted flux from shells out to `r`, as typed irradiance.
    ///
    /// Static: `F = ρ_L r`. Expanding: `F = (ρ_L c/H) [1 − 1/(1 + H r/c)]`.
    /// Tired light: `F = (ρ_L c/H) [1 − e^{-H r/c}]`.
    fn flux_to(&self, r: Qty<Length>) -> Qty<Irradiance> {
        let rho = cosmic_luminosity_density();
        if self.expanding {
            let x = (hubble_constant() * r / C).value();
            if x < 1.0e-12 {
                rho * r
            } else {
                // (ρ c/H) (1 − 1/(1+x)) = ρ r * (1 − 1/(1+x)) / x
                rho * r * ((1.0 - 1.0 / (1.0 + x)) / x)
            }
        } else if self.tired {
            let x = (hubble_constant() * r / C).value();
            if x < 1.0e-12 {
                rho * r
            } else {
                // (ρ c/H) (1 − e^{-x}) = ρ r * (1 − e^{-x}) / x
                rho * r * ((1.0 - (-x).exp()) / x)
            }
        } else {
            rho * r
        }
    }

    /// Shell contribution `dF/dr` at radius `r` (irradiance per length).
    fn shell_density_at(&self, r: Qty<Length>) -> f64 {
        let rho = cosmic_luminosity_density().value();
        if self.expanding {
            let x = (hubble_constant() * r / C).value();
            rho / (1.0 + x).powi(2)
        } else if self.tired {
            let x = (hubble_constant() * r / C).value();
            rho * (-x).exp()
        } else {
            rho
        }
    }

    /// Optical depth to a stellar disk, `τ = n σ R` with `n = ρ_L / L_☉`
    /// and `σ = π R_☉²`. Dimensionless by construction: `(1/L³) · L² · L`.
    fn optical_depth(&self, r: Qty<Length>) -> Qty<Dimensionless> {
        let n = cosmic_luminosity_density() / solar_luminosity();
        let sigma = solar_radius() * solar_radius() * PI;
        n * sigma * r
    }

    fn stellar_surface(&self) -> Qty<Irradiance> {
        let t = kelvin(STAR_T_K);
        stefan_boltzmann_constant() * t * t * t * t
    }

    fn is_unbounded_static(&self) -> bool {
        !self.finite_age && !self.expanding
    }

    /// IR package for this shell encoding. Equations are `dF = rho dr`
    /// and, when forked, `tired light`. Knobs stay on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![SHELL_EQ.to_string()];
        if self.tired {
            equations.push(TIRED_EQ.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["inverse-square-euclidean-shells".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: SHELL_CANCELLATION.into(),
                statement: "Inverse-square dilution cancels shell area: dF/dr is independent of r."
                    .into(),
                layer: "spacetime".into(),
                class: "model-internal".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load a shell encoding from a package. Knobs default; overlay them
    /// from a live olbers-static object when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        if pkg.id != "olbers-static" {
            return Err(format!(
                "olbers-static package id '{}' is not olbers-static",
                pkg.id
            ));
        }
        let tired = parse_olbers_shell(pkg)?;
        Ok(Self {
            tired,
            ..Self::static_euclidean()
        })
    }

    fn tired_equation() -> String {
        TIRED_EQ.to_string()
    }
}

impl Knobbed for OlbersSky {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "finite_age" => Ok(KnobValue::Bool(self.finite_age)),
            "expanding" => Ok(KnobValue::Bool(self.expanding)),
            "age_yr" => Ok(KnobValue::Float(self.age_yr)),
            "cutoff_m" => Ok(KnobValue::Float(self.cutoff_m)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("finite_age", KnobValue::Bool(v)) => self.finite_age = v,
            ("expanding", KnobValue::Bool(v)) => self.expanding = v,
            ("age_yr", KnobValue::Float(v)) => self.age_yr = v,
            ("cutoff_m", KnobValue::Float(v)) => self.cutoff_m = v,
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

impl Theory for OlbersSky {
    fn id(&self) -> &'static str {
        self.id
    }
    fn name(&self) -> &'static str {
        match (self.finite_age, self.expanding) {
            (false, false) => "Olbers (static Euclidean)",
            (true, false) => "Finite-age Euclidean sky",
            (false, true) => "Expanding Euclidean sky",
            (true, true) => "Finite-age expanding sky",
        }
    }
    fn summary(&self) -> &'static str {
        "A Euclidean universe uniformly filled with starlight. Inverse-square \
         dilution cancels shell area, so dF = ρ_L dr and the static integral \
         diverges: the night sky should be as bright as a stellar surface. A \
         finite age caps the integral at c t; Hubble dimming saturates it at \
         ρ_L c/H. Those are independent knob turns. Tired light is an IR \
         mutation on olbers-static, not those knobs."
    }
    fn world(&self) -> Option<World> {
        None
    }
    fn note(&self) -> String {
        if self.is_unbounded_static() {
            return format!(
                "{}: improper R → ∞, F → ∞, τ → ∞ (cutoff {:.3e} m is not the verdict)",
                self.name(),
                self.cutoff_m
            );
        }
        let r = self.verdict_radius();
        let f = self.flux_to(r);
        let tau = self.optical_depth(r);
        format!(
            "{}: R_eff = {:.3e} m, F = {:.3e} W/m², τ = {:.3e}",
            self.name(),
            r.value(),
            f.value(),
            tau.value()
        )
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                SHELL_CANCELLATION,
                "Inverse-square dilution cancels shell area: dF/dr is independent of r.",
                LayerId::Spacetime,
                ClaimClass::ModelInternal,
            )
            .with_domain(shell_cancellation_domain()),
            Claim::new(
                SKY_FINITE,
                "Integrated sky brightness stays finite as the radial cutoff is removed.",
                LayerId::Spacetime,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                NIGHT_SKY_DARK,
                "The night sky is far dimmer than a stellar photosphere.",
                LayerId::Statistical,
                ClaimClass::ModelInternal,
            ),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id_str() {
            SHELL_CANCELLATION => eval_shell(self, claim),
            SKY_FINITE => eval_sky_finite(self, claim),
            NIGHT_SKY_DARK => eval_dark(self, claim),
            _ => Verdict::inapplicable(claim, "claim not made by an Olbers-sky object"),
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        if self.id != "olbers-static" {
            return None;
        }
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.tired = parsed.tired;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        if self.id != "olbers-static" || self.tired {
            return Vec::new();
        }
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(Self::tired_equation()),
        );
        if let Ok(parsed) = Self::from_package(&mutated) {
            if parsed.tired {
                let mut fork = self.clone();
                fork.tired = true;
                return vec![("add-tired-light".into(), Box::new(fork))];
            }
        }
        Vec::new()
    }
}

fn eval_shell(sky: &OlbersSky, claim: &Claim) -> Verdict {
    // Probe well inside the Hubble length so linear Hubble flow is not crazy.
    let r = sky.hubble_length() * 0.1;
    let r2 = r * 2.0;
    let d1 = sky.shell_density_at(r);
    let d2 = sky.shell_density_at(r2);
    let ratio = d2 / d1;
    if (ratio - 1.0).abs() < 0.02 {
        Verdict::holds(
            claim,
            "dF/dr is independent of r: inverse-square cancels shell area",
        )
        .with_evidence([format!(
            "dF/dr(2r)/dF/dr(r) = {ratio:.4} at r = 0.1 c/H (Euclidean cancellation)"
        )])
    } else if sky.expanding {
        Verdict::fails(
            claim,
            "Hubble dimming makes dF/dr fall with r; the standing cancellation fails",
        )
        .with_evidence([format!(
            "dF/dr(2r)/dF/dr(r) = {ratio:.4} at r = 0.1 c/H (static Euclidean requires 1)"
        )])
    } else {
        Verdict::fails(
            claim,
            "tired light makes dF/dr fall with r; the standing cancellation fails",
        )
        .with_evidence([format!(
            "dF/dr(2r)/dF/dr(r) = {ratio:.4} at r = 0.1 c/H (e^{{-Hr/c}}; static Euclidean requires 1)"
        )])
    }
}

fn eval_sky_finite(sky: &OlbersSky, claim: &Claim) -> Verdict {
    if sky.is_unbounded_static() && sky.tired {
        let r = sky.hubble_length();
        let ratio = sky.flux_to(r * 2.0).value() / sky.flux_to(r).value();
        Verdict::holds(
            claim,
            "tired light caps the improper integral; F is not proportional to R",
        )
        .with_evidence([format!(
            "F(2 c/H)/F(c/H) = {ratio:.3} (static Euclidean is 2; covering still diverges)"
        )])
    } else if sky.is_unbounded_static() {
        // Improper integral: F ∝ R, sampled by doubling a finite probe.
        // Independent of the current cutoff (a large cutoff can still look finite).
        let r = sky.age_horizon();
        let ratio = sky.flux_to(r * 2.0).value() / sky.flux_to(r).value();
        Verdict::fails(
            claim,
            "static Euclidean flux grows without bound: F(2R)/F(R) = 2",
        )
        .with_evidence([format!(
            "F(2R)/F(R) = {ratio:.3} at R = c t (Olbers catastrophe; independent of cutoff)"
        )])
    } else if sky.finite_age {
        let r = sky.age_horizon();
        let f = sky.flux_to(r);
        Verdict::holds(
            claim,
            "finite age caps the integral at R = c t; the sky brightness is finite",
        )
        .with_evidence([format!(
            "F(c t) = {:.3e} W/m² with t = {:.3e} yr (does not grow with cutoff)",
            f.value(),
            sky.age_yr
        )])
    } else {
        // Expanding, infinite age: saturates at ρ_L c/H.
        // F(c/H) = F_∞/2, F(100 c/H) ≈ F_∞, so the ratio approaches 2.
        // The static Euclidean law at the same radii would give 100.
        let f_inf = sky.flux_to(sky.hubble_length() * 100.0);
        let f_h = sky.flux_to(sky.hubble_length());
        let sat = f_inf.value() / f_h.value();
        if (1.5..2.2).contains(&sat) {
            Verdict::holds(
                claim,
                "Hubble dimming saturates the improper integral at F = ρ_L c/H",
            )
            .with_evidence([format!(
                "F(100 c/H)/F(c/H) = {sat:.3} (approaches 2 from below; static would be 100)"
            )])
        } else {
            Verdict::fails(claim, "expanding flux did not saturate as ρ_L c/H predicts")
                .with_evidence([format!("F(100 c/H)/F(c/H) = {sat:.3} (expected ≈ 2)")])
        }
    }
}

fn eval_dark(sky: &OlbersSky, claim: &Claim) -> Verdict {
    if sky.is_unbounded_static() {
        Verdict::fails(
            claim,
            "τ = n σ R → ∞ as R → ∞: every line of sight hits a star",
        )
        .with_evidence([format!(
            "covering fraction diverges; surface brightness saturates at σ T⁴ = {:.3e} W/m²",
            sky.stellar_surface().value()
        )])
    } else {
        let r = sky.verdict_radius();
        let tau = sky.optical_depth(r).value();
        let f = sky.flux_to(r);
        let star = sky.stellar_surface();
        let ratio = f.value() / star.value();
        if tau < TAU_DARK && ratio < IRRADIANCE_DARK {
            Verdict::holds(
                claim,
                "optical depth to a stellar disk is tiny; the night sky is dark",
            )
            .with_evidence([format!(
                "τ = {tau:.3e} at R_eff = {:.3e} m; F/σT⁴ = {ratio:.3e}",
                r.value()
            )])
        } else {
            Verdict::fails(
                claim,
                "the horizon is opaque: τ ≳ 1, the sky is photosphere-bright",
            )
            .with_evidence([format!(
                "τ = {tau:.3e} at R_eff = {:.3e} m; F/σT⁴ = {ratio:.3e}",
                r.value()
            )])
        }
    }
}

/// Olbers' paradox: infinite static Euclidean starlight on trial.
pub fn olbers() -> ExperimentReport {
    report_from_rows(
        "olbers",
        "Olbers lab",
        "Does an infinite, eternal, Euclidean universe uniformly filled with \
         stars have a dark night sky — or does inverse-square cancellation \
         make the integral diverge, and only a horizon (finite age) or Hubble \
         dimming keep the sky dark?",
        "Verdicts are internal to the encoding. The standing catastrophe is \
         the improper R → ∞ integral, not a large-but-finite cutoff (a cutoff \
         inside the mean free path still looks dark). Linear Hubble z = H r/c \
         is not a full FLRW integral. n and L are a cosmic-mean luminosity \
         density, not the solar neighbourhood packed out to infinity.",
        vec![
            "`astro.shell-cancellation` is the standing axiom: it holds in static Euclidean space and fails under Hubble dimming.".into(),
            "`astro.sky-finite` fails for the static improper integral (F(2R)/F(R) = 2) and holds once a horizon or Hubble saturation caps F.".into(),
            "`astro.night-sky-dark` fails for R → ∞ (τ diverges) and holds for a Hubble-time universe (τ ~ 10⁻¹⁵).".into(),
            "`hypothesize olbers-static`: add-tired-light is IR, not set. finite_age and expanding stay knobs. Covering still diverges.".into(),
            "`set olbers-horizon age_yr 1e26` makes τ ≳ 1: a finite but ancient sky is photosphere-bright.".into(),
        ],
        &olbers_rows(),
        vec![
            Box::new(OlbersSky::static_euclidean()),
            Box::new(OlbersSky::finite_age()),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;
    use physis_core::qty::meters;

    fn verdict(t: &dyn Theory, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id_str() == id).unwrap();
        t.evaluate(&c).kind
    }

    #[test]
    fn static_shell_cancellation_is_exact() {
        let s = OlbersSky::static_euclidean();
        let r = meters(1.0e24);
        let d1 = s.shell_density_at(r);
        let d2 = s.shell_density_at(r * 2.0);
        assert!((d1 / d2 - 1.0).abs() < 1e-12);
        assert_eq!(verdict(&s, SHELL_CANCELLATION), VerdictKind::Holds);
    }

    #[test]
    fn static_flux_doubles_with_radius() {
        let s = OlbersSky::static_euclidean();
        let r = s.age_horizon();
        let ratio = s.flux_to(r * 2.0).value() / s.flux_to(r).value();
        assert!((ratio - 2.0).abs() < 1e-12, "ratio = {ratio}");
        assert_eq!(verdict(&s, SKY_FINITE), VerdictKind::Fails);
        assert_eq!(verdict(&s, NIGHT_SKY_DARK), VerdictKind::Fails);
    }

    #[test]
    fn finite_age_holds_finite_and_dark() {
        let h = OlbersSky::finite_age();
        assert_eq!(verdict(&h, SHELL_CANCELLATION), VerdictKind::Holds);
        assert_eq!(verdict(&h, SKY_FINITE), VerdictKind::Holds);
        assert_eq!(verdict(&h, NIGHT_SKY_DARK), VerdictKind::Holds);
        let tau = h.optical_depth(h.age_horizon());
        assert!(
            tau.value() < 1e-10,
            "τ = {} should be tiny at a Hubble time",
            tau.value()
        );
    }

    #[test]
    fn expanding_flux_saturates_near_two_at_a_hubble_length() {
        let mut s = OlbersSky::static_euclidean();
        s.set("expanding", KnobValue::Bool(true)).unwrap();
        let f_inf = s.flux_to(s.hubble_length() * 100.0).value();
        let f_h = s.flux_to(s.hubble_length()).value();
        let sat = f_inf / f_h;
        assert!(
            (sat - 2.0).abs() < 0.05,
            "expanding F(100 c/H)/F(c/H) = {sat}, expected ≈ 2"
        );
        let static_sat = {
            let u = OlbersSky::static_euclidean();
            u.flux_to(u.hubble_length() * 100.0).value() / u.flux_to(u.hubble_length()).value()
        };
        assert!(
            (static_sat - 100.0).abs() < 1e-9,
            "static ratio = {static_sat}"
        );
    }

    #[test]
    fn expanding_breaks_cancellation_and_saves_the_sky() {
        let mut s = OlbersSky::static_euclidean();
        s.set("expanding", KnobValue::Bool(true)).unwrap();
        assert_eq!(verdict(&s, SHELL_CANCELLATION), VerdictKind::Fails);
        assert_eq!(verdict(&s, SKY_FINITE), VerdictKind::Holds);
        assert_eq!(verdict(&s, NIGHT_SKY_DARK), VerdictKind::Holds);
        assert_eq!(s.id(), "olbers-static");
    }

    #[test]
    fn finite_age_knob_flips_the_catastrophe() {
        let mut s = OlbersSky::static_euclidean();
        assert_eq!(verdict(&s, SKY_FINITE), VerdictKind::Fails);
        s.set("finite_age", KnobValue::Bool(true)).unwrap();
        assert_eq!(verdict(&s, SKY_FINITE), VerdictKind::Holds);
        assert_eq!(verdict(&s, NIGHT_SKY_DARK), VerdictKind::Holds);
        assert_eq!(verdict(&s, SHELL_CANCELLATION), VerdictKind::Holds);
    }

    #[test]
    fn an_ancient_finite_universe_is_photosphere_bright() {
        let mut h = OlbersSky::finite_age();
        h.set("age_yr", KnobValue::Float(1.0e26)).unwrap();
        assert_eq!(verdict(&h, SKY_FINITE), VerdictKind::Holds);
        assert_eq!(verdict(&h, NIGHT_SKY_DARK), VerdictKind::Fails);
    }

    #[test]
    fn tired_light_is_ir_not_a_knob() {
        let s = OlbersSky::static_euclidean();
        assert!(!s.tired);
        assert_eq!(verdict(&s, SHELL_CANCELLATION), VerdictKind::Holds);
        assert_eq!(verdict(&s, SKY_FINITE), VerdictKind::Fails);
        assert_eq!(verdict(&s, NIGHT_SKY_DARK), VerdictKind::Fails);
        assert!(
            OlbersSky::static_euclidean()
                .set("tired", KnobValue::Bool(true))
                .is_err(),
            "tired light is an IR mutation, not a knob"
        );
        assert!(
            OlbersSky::static_euclidean()
                .set("tired_light", KnobValue::Bool(true))
                .is_err(),
            "tired_light is not a knob"
        );
        assert!(
            OlbersSky::finite_age().ir_package().is_none(),
            "olbers-horizon must have no IR package"
        );
        let src = render_package(&s.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(pkg.equations.len(), 1, "live package must stay dF = rho dr");
        assert_eq!(pkg.equations[0], SHELL_EQ);
        assert_eq!(
            OlbersSky::from_package(&pkg).unwrap(),
            s,
            "IR round-trip must preserve inverse-square shells"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(OlbersSky::tired_equation()),
        );
        let parsed = OlbersSky::from_package(&mutated).unwrap();
        assert!(parsed.tired);
        let mut fork = s.clone();
        fork.tired = true;
        assert_eq!(fork.id(), "olbers-static");
        assert_eq!(verdict(&fork, SHELL_CANCELLATION), VerdictKind::Fails);
        assert_eq!(verdict(&fork, SKY_FINITE), VerdictKind::Holds);
        assert_eq!(verdict(&fork, NIGHT_SKY_DARK), VerdictKind::Fails);
        assert_eq!(verdict(&s, SHELL_CANCELLATION), VerdictKind::Holds);
        let cell = fork
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SHELL_CANCELLATION)
            .unwrap();
        let v = fork.evaluate(&cell);
        assert!(
            !v.summary.contains("Hubble") && !v.summary.contains("finite_age"),
            "tired light is not a knob: {}",
            v.summary
        );
        let ratio = {
            let r = fork.hubble_length() * 0.1;
            fork.shell_density_at(r * 2.0) / fork.shell_density_at(r)
        };
        assert!(
            (ratio - (-0.1_f64).exp()).abs() < 1e-12,
            "tired residual must be e^{{-0.1}}, got {ratio}"
        );
        assert!(
            (ratio - 1.0).abs() > 0.05,
            "tired residual must not be a unit flag, got {ratio}"
        );
        let hubble = {
            let mut e = OlbersSky::static_euclidean();
            e.set("expanding", KnobValue::Bool(true)).unwrap();
            let r = e.hubble_length() * 0.1;
            e.shell_density_at(r * 2.0) / e.shell_density_at(r)
        };
        assert!(
            (ratio - hubble).abs() > 0.04,
            "tired light must not be Hubble dimming: tired {ratio} Hubble {hubble}"
        );
        assert!(
            v.evidence
                .iter()
                .any(|e| e.contains("0.90") || e.contains("0.9")),
            "got {:?}",
            v.evidence
        );
        let mut aged = fork.clone();
        aged.set("finite_age", KnobValue::Bool(true)).unwrap();
        assert_eq!(verdict(&aged, SHELL_CANCELLATION), VerdictKind::Fails);
        assert_eq!(verdict(&aged, NIGHT_SKY_DARK), VerdictKind::Holds);
        let mut expanded = fork.clone();
        expanded.set("expanding", KnobValue::Bool(true)).unwrap();
        assert_eq!(verdict(&expanded, SHELL_CANCELLATION), VerdictKind::Fails);
        assert_eq!(verdict(&expanded, NIGHT_SKY_DARK), VerdictKind::Holds);

        let probes = OlbersSky::static_euclidean().structural_mutations();
        assert!(
            probes.iter().any(|(label, _)| label == "add-tired-light"),
            "live olbers-static must offer add-tired-light: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let probe = probes
            .iter()
            .find(|(label, _)| label == "add-tired-light")
            .expect("add-tired-light");
        assert_eq!(
            verdict(probe.1.as_ref(), SHELL_CANCELLATION),
            VerdictKind::Fails
        );
        assert_eq!(verdict(probe.1.as_ref(), SKY_FINITE), VerdictKind::Holds);
        assert_eq!(
            verdict(probe.1.as_ref(), NIGHT_SKY_DARK),
            VerdictKind::Fails
        );
        let fork_probes = fork.structural_mutations();
        assert!(
            fork_probes
                .iter()
                .all(|(label, _)| label != "add-tired-light"),
            "tired fork must not re-offer add-tired-light"
        );
        let live = OlbersSky::static_euclidean();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let mut expanded_live = OlbersSky::static_euclidean();
        expanded_live
            .set("expanding", KnobValue::Bool(true))
            .unwrap();
        let rebuilt = expanded_live.reparse_package(&parsed).unwrap();
        assert_eq!(
            rebuilt.get("expanding").unwrap(),
            KnobValue::Bool(true),
            "reparse must overlay tired IR onto live knobs"
        );
        assert_eq!(
            verdict(rebuilt.as_ref(), SHELL_CANCELLATION),
            VerdictKind::Fails,
            "expanding still Fails cancellation on the live Euclidean encoding"
        );
        assert_eq!(
            verdict(rebuilt.as_ref(), NIGHT_SKY_DARK),
            VerdictKind::Holds
        );
        let live_rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(
            verdict(live_rebuilt.as_ref(), SHELL_CANCELLATION),
            VerdictKind::Holds
        );
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SHELL_CANCELLATION)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "shell cancellation must name inverse-square Euclidean shells: {:?}",
            cell.domain()
        );
        assert!(
            OlbersSky::finite_age().ir_package().is_none(),
            "olbers-horizon has no package"
        );
        assert!(
            OlbersSky::finite_age()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-tired-light"),
            "olbers-horizon must not grow add-tired-light"
        );
        assert!(
            crate::computation::TuringMachine::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-tired-light"),
            "turing-machine must not grow add-tired-light"
        );
        assert!(
            crate::blackbody::Blackbody::planck()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-tired-light"),
            "planck must not grow add-tired-light"
        );
        assert!(
            OlbersSky::static_euclidean()
                .set("finite_age", KnobValue::Bool(true))
                .is_ok(),
            "olbers-static keeps the finite_age knob"
        );
        assert!(
            OlbersSky::static_euclidean()
                .set("expanding", KnobValue::Bool(true))
                .is_ok(),
            "olbers-static keeps the expanding knob"
        );
    }

    #[test]
    fn sky_finite_does_not_use_the_current_cutoff() {
        // A static sky with a small cutoff still fails sky-finite: the
        // catastrophe is the improper integral, not the present cutoff.
        let mut s = OlbersSky::static_euclidean();
        s.set("cutoff_m", KnobValue::Float(1.0e20)).unwrap();
        assert_eq!(verdict(&s, SKY_FINITE), VerdictKind::Fails);
        assert_eq!(verdict(&s, NIGHT_SKY_DARK), VerdictKind::Fails);
    }

    #[test]
    fn flux_is_typed_irradiance() {
        let h = OlbersSky::finite_age();
        let f: Qty<Irradiance> = h.flux_to(h.age_horizon());
        let star: Qty<Irradiance> = h.stellar_surface();
        assert!(f.value() > 0.0 && f.value() < star.value());
        let tau: Qty<Dimensionless> = h.optical_depth(h.age_horizon());
        assert!(tau.value() > 0.0 && tau.value() < 1e-10);
    }

    #[test]
    fn note_uses_the_same_radius_as_the_dark_sky_verdict() {
        // A huge age with the default cutoff still sitting at 1e28 m: the
        // note must report c t, not the cutoff, or it would look dark while
        // night-sky-dark fails.
        let mut h = OlbersSky::finite_age();
        h.set("age_yr", KnobValue::Float(1.0e26)).unwrap();
        assert_eq!(verdict(&h, NIGHT_SKY_DARK), VerdictKind::Fails);
        let r = h.verdict_radius().value();
        let ct = h.age_horizon().value();
        assert!(
            (r / ct - 1.0).abs() < 1e-12,
            "verdict radius {r} must be c t {ct}, not cutoff"
        );
        let note = h.note();
        assert!(
            note.contains(&format!("{:.3e}", ct)),
            "note must quote c t, got {note}"
        );
        assert!(
            !note.contains("1.000e28"),
            "note must not quote the cutoff as R_eff, got {note}"
        );
    }

    #[test]
    fn olbers_experiment_puts_the_static_sky_on_trial() {
        let r = olbers();
        assert_eq!(r.id, "olbers");
        let cell =
            |claim: &str, theory: &str| r.matrix.get(claim).and_then(|m| m.get(theory)).copied();
        assert_eq!(
            cell(SHELL_CANCELLATION, "olbers-static"),
            Some(VerdictKind::Holds)
        );
        assert_eq!(cell(SKY_FINITE, "olbers-static"), Some(VerdictKind::Fails));
        assert_eq!(
            cell(NIGHT_SKY_DARK, "olbers-static"),
            Some(VerdictKind::Fails)
        );
        assert_eq!(cell(SKY_FINITE, "olbers-horizon"), Some(VerdictKind::Holds));
        assert_eq!(
            cell(NIGHT_SKY_DARK, "olbers-horizon"),
            Some(VerdictKind::Holds)
        );
    }
}
