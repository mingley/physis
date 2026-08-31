//! General relativity as a classical spacetime theory.
//!
//! The Einstein–Hilbert action lives on the IR package. A quadratic
//! curvature term (`add-r-squared`) is a package mutation, not a `dim`
//! knob: uniqueness of Einstein gravity plus Λ fails on that fork.
//! Solar tests still hold (the Schwarzschild integrals). A Brans–Dicke
//! scalar (`add-brans-dicke`) is a second package mutation: PPN
//! `γ = (ω+1)/(ω+2)` at `ω = 1` is not 1, so Eddington and Mercury fail
//! while uniqueness also fails. That is not the `dim` knob and not a
//! silent Newton install. `dim` / `cosmological_constant` stay knobs.

use physis_core::assumption::DomainOfValidity;
use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::ParameterOrigin;
use physis_ir::{apply_mutation, parse_package, render_package, PackageMutation, TheoryPackage};
use physis_model::{GaugeGroup, Manifold, Signature, Spectrum, Topology, World};

use crate::claims;
use crate::framework::Theory;
use crate::gravity::{
    eval_solar, mercury_analytic_arcsec_per_century, mercury_arcsec_per_century, solar_claims,
    solar_deflection_arcsec, EDDINGTON, EDDINGTON_ARCSEC, MERCURY_ARCSEC_PER_CENTURY,
    MERCURY_PERIHELION, NEWTON_HALF,
};

/// Einstein–Hilbert action on the live GR package.
const ACTION_EH: &str = "action einstein-hilbert";
/// Starobinsky / f(R) quadratic curvature term.
const ACTION_R2: &str = "action r-squared";
/// Brans–Dicke scalar-tensor term.
const ACTION_BD: &str = "action brans-dicke";
/// Quadratic coupling used as residual evidence. ξ → 0 recovers
/// Einstein–Hilbert and the uniqueness cell still fails.
const QUADRATIC_XI: f64 = 1.0;
/// Brans–Dicke ω used as residual evidence. ω → ∞ recovers GR
/// solar tests and the uniqueness cell still fails.
const BRANS_DICKE_OMEGA: f64 = 1.0;

fn parse_gr_action(pkg: &TheoryPackage) -> Result<(bool, bool), String> {
    let mut eh = false;
    let mut r_squared = false;
    let mut brans_dicke = false;
    for eq in &pkg.equations {
        match eq.trim() {
            ACTION_EH => eh = true,
            ACTION_R2 => r_squared = true,
            ACTION_BD => brans_dicke = true,
            _ => {}
        }
    }
    if !eh {
        return Err(format!("{} package has no Einstein-Hilbert action", pkg.id));
    }
    Ok((r_squared, brans_dicke))
}

fn brans_dicke_gamma(omega: f64) -> f64 {
    (omega + 1.0) / (omega + 2.0)
}

fn brans_dicke_light_factor(gamma: f64) -> f64 {
    (1.0 + gamma) / 2.0
}

fn brans_dicke_perihelion_factor(gamma: f64) -> f64 {
    (2.0 + 2.0 * gamma - 1.0) / 3.0
}

fn eval_solar_brans_dicke(dim: u8, claim: &Claim) -> Verdict {
    if dim != 4 {
        return Verdict::inapplicable(
            claim,
            "solar-system tests are 4D Schwarzschild / inverse-square",
        );
    }
    let omega = BRANS_DICKE_OMEGA;
    let gamma = brans_dicke_gamma(omega);
    let light = brans_dicke_light_factor(gamma);
    let peri = brans_dicke_perihelion_factor(gamma);
    match claim.id_str() {
        NEWTON_HALF => {
            let newton = solar_deflection_arcsec(false);
            let delta = solar_deflection_arcsec(true) * light;
            Verdict::fails(claim, "Brans-Dicke PPN deflection is not 2GM/(c²R)").with_evidence([
                format!(
                    "δ = {delta:.4}\" vs half-angle {newton:.4}\"; γ = {gamma:.4} at ω = {omega}"
                ),
            ])
        }
        EDDINGTON => {
            let gr = solar_deflection_arcsec(true);
            let delta = gr * light;
            if (delta - EDDINGTON_ARCSEC).abs() / EDDINGTON_ARCSEC < 0.03 {
                Verdict::holds(claim, "Brans-Dicke PPN still matches 1.75″").with_evidence([
                    format!("δ = {delta:.4}\" (GR integral {gr:.4}\" × (1+γ)/2 = {light:.4})"),
                ])
            } else {
                Verdict::fails(
                    claim,
                    "Brans-Dicke PPN: sampled deflection is not 1.75″",
                )
                .with_evidence([format!(
                    "δ = {delta:.4}\" (GR {gr:.4}\" × (1+γ)/2 = {light:.4}; γ = {gamma:.4} at ω = {omega})"
                )])
            }
        }
        MERCURY_PERIHELION => {
            let gr = mercury_arcsec_per_century(true);
            let extra = gr * peri;
            let analytic = mercury_analytic_arcsec_per_century();
            if (extra - MERCURY_ARCSEC_PER_CENTURY).abs() < 1.5 {
                Verdict::holds(claim, "Brans-Dicke PPN still matches 43″ per century")
                    .with_evidence([format!(
                        "Δω = {extra:.2}\" / century (GR {gr:.2}\" × (2+2γ−β)/3 = {peri:.4})"
                    )])
            } else {
                Verdict::fails(claim, "Brans-Dicke PPN: perihelion is not 43″ per century")
                    .with_evidence([format!(
                        "Δω = {extra:.2}\" / century (GR {gr:.2}\" × (2+2γ−β)/3 = {peri:.4}; \
                     analytic GR {analytic:.2}\"/cy; γ = {gamma:.4} at ω = {omega})"
                    )])
            }
        }
        _ => Verdict::inapplicable(claim, "claim not made by a solar-system gravity object"),
    }
}

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "dim",
        layer: LayerId::Spacetime,
        doc: "Spacetime dimension. Empirical GR is 4. Higher-D GR is a well-defined classical theory. Quadratic curvature is not this knob: add-r-squared is an IR mutation. Brans-Dicke is not this knob: add-brans-dicke is an IR mutation.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::UInt { min: 2, max: 26 },
    },
    KnobSpec {
        name: "cosmological_constant",
        layer: LayerId::Spacetime,
        doc: "Λ in Planck units (order-of-magnitude knob, not a precision cosmology fit).",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: -1.0,
            max: 1.0,
        },
    },
];

/// Einstein gravity.
///
/// The Einstein–Hilbert action lives on the IR package. Quadratic
/// curvature (`add-r-squared`) is a package mutation, not a knob:
/// uniqueness of Einstein gravity plus Λ fails. Brans–Dicke
/// (`add-brans-dicke`) is a second package mutation: PPN γ is not 1,
/// so Eddington and Mercury fail. `dim` / `cosmological_constant`
/// stay knobs. The fork is still this object, not a silent Newton
/// install.
#[derive(Clone, Debug, PartialEq)]
pub struct GeneralRelativity {
    dim: u8,
    cosmological_constant: f64,
    /// Whether the encoding includes an R² curvature term.
    r_squared: bool,
    /// Whether the encoding includes a Brans–Dicke scalar.
    brans_dicke: bool,
}

impl Default for GeneralRelativity {
    fn default() -> Self {
        Self {
            dim: 4,
            cosmological_constant: 0.0,
            r_squared: false,
            brans_dicke: false,
        }
    }
}

impl Knobbed for GeneralRelativity {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }

    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "dim" => Ok(KnobValue::UInt(self.dim as u64)),
            "cosmological_constant" => Ok(KnobValue::Float(self.cosmological_constant)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }

    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("dim", KnobValue::UInt(v)) => self.dim = v as u8,
            ("cosmological_constant", KnobValue::Float(v)) => self.cosmological_constant = v,
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

impl GeneralRelativity {
    /// IR package for this gravitational action. Equations are
    /// `action einstein-hilbert` and, when forked, `action r-squared`
    /// and/or `action brans-dicke`. Dimension and Λ stay on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![ACTION_EH.to_string()];
        if self.r_squared {
            equations.push(ACTION_R2.to_string());
        }
        if self.brans_dicke {
            equations.push(ACTION_BD.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["einstein-hilbert".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: claims::UNIQUE_VACUUM.into(),
                statement:
                    "Einstein gravity plus Λ is a unique classical theory (not a landscape).".into(),
                layer: "spacetime".into(),
                class: "heuristic".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load an action encoding from a package. Knobs default; overlay
    /// them from a live GR object when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        if pkg.id != "general-relativity" {
            return Err(format!(
                "general-relativity package id '{}' is not general-relativity",
                pkg.id
            ));
        }
        let (r_squared, brans_dicke) = parse_gr_action(pkg)?;
        Ok(Self {
            r_squared,
            brans_dicke,
            ..Self::default()
        })
    }

    fn r_squared_equation() -> String {
        ACTION_R2.to_string()
    }

    fn brans_dicke_equation() -> String {
        ACTION_BD.to_string()
    }

    fn build_world(&self) -> World {
        let space = self.dim.saturating_sub(1);
        World {
            spacetime: Manifold {
                dim: self.dim,
                signature: Signature { time: 1, space },
                compact_extra: 0,
                compact_radius_planck: 0.0,
                topology: Topology::Minkowski,
                convention: physis_model::SignConvention::MostlyMinus,
            },
            gauge: GaugeGroup::trivial(),
            spectrum: {
                let mut s = Spectrum::empty();
                s.species.push(physis_model::Species::graviton());
                s
            },
            has_gravity: true,
            supersymmetric: false,
            free_parameter_count: 2, // G and Λ, roughly
            landscape_log10: 0.0,
            note: if self.brans_dicke && self.r_squared {
                format!(
                    "GR in D={} Λ={} with R² curvature and Brans-Dicke",
                    self.dim, self.cosmological_constant
                )
            } else if self.brans_dicke {
                format!(
                    "GR in D={} Λ={} with Brans-Dicke",
                    self.dim, self.cosmological_constant
                )
            } else if self.r_squared {
                format!(
                    "GR in D={} Λ={} with R² curvature",
                    self.dim, self.cosmological_constant
                )
            } else {
                format!("GR in D={} Λ={}", self.dim, self.cosmological_constant)
            },
        }
    }
}

impl Theory for GeneralRelativity {
    fn id(&self) -> &'static str {
        "general-relativity"
    }
    fn name(&self) -> &'static str {
        if self.brans_dicke {
            "Brans–Dicke gravity"
        } else {
            "General relativity"
        }
    }
    fn summary(&self) -> &'static str {
        "Classical dynamical spacetime. Matches gravity from tabletop to cosmology. \
         Grazing solar deflection (1.75″) and Mercury's 43″ perihelion are computed \
         Schwarzschild integrals, not slogans. The Einstein–Hilbert action is an \
         IR encoding. Quadratic curvature is an IR mutation, not a dim knob. \
         Brans-Dicke is a second IR mutation: PPN γ is not 1. \
         Not a quantum theory. Not a theory of the Standard Model spectrum."
    }

    fn world(&self) -> Option<World> {
        Some(self.build_world())
    }

    fn claims(&self) -> Vec<Claim> {
        let mut c = vec![
            claims::c(
                claims::SPACETIME_STRUCTURE,
                "Lorentzian manifold of the chosen dimension.",
                LayerId::Spacetime,
                ClaimClass::ModelInternal,
            ),
            claims::c(
                claims::OBSERVED_4D,
                "Spacetime dimension is 4.",
                LayerId::Spacetime,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::GRAVITY,
                "Gravity is dynamical spacetime.",
                LayerId::Spacetime,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::FERMIONS,
                "GR contains the Standard Model fermions.",
                LayerId::Particle,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::SM_GAUGE,
                "GR contains the Standard Model gauge group.",
                LayerId::Interaction,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::UV_COMPLETION,
                "GR is UV-complete as a quantum theory.",
                LayerId::Quantum,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::UNIQUE_VACUUM,
                "Einstein gravity plus Λ is a unique classical theory (not a landscape).",
                LayerId::Spacetime,
                ClaimClass::Heuristic,
            )
            .with_domain(DomainOfValidity::new(
                vec!["classical Einstein-Hilbert plus Λ".into()],
                vec![
                    "uniqueness of the classical action given D and Λ, not a quantum vacuum count"
                        .into(),
                ],
                "A unique classical Lagrangian is not a unique quantum vacuum. Using \
                 this as a string-landscape theorem is a new claim.",
            )),
        ];
        c.extend(solar_claims());
        c
    }

    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id_str() {
            claims::SPACETIME_STRUCTURE => {
                if self.build_world().spacetime.structurally_ok() {
                    Verdict::holds(claim, "Lorentzian, consistent dim")
                } else {
                    Verdict::fails(claim, "inconsistent manifold numbers")
                }
            }
            claims::OBSERVED_4D => {
                if self.dim == 4 {
                    Verdict::holds(claim, "D=4")
                } else {
                    Verdict::fails(claim, format!("D={}, not 4", self.dim))
                }
            }
            claims::GRAVITY => {
                if self.brans_dicke {
                    Verdict::holds(claim, "Brans-Dicke scalar-tensor gravity")
                } else {
                    Verdict::holds(claim, "Einstein-Hilbert gravity")
                }
            }
            claims::FERMIONS | claims::SM_GAUGE => {
                Verdict::fails(claim, "GR has no Standard Model matter content")
            }
            claims::UV_COMPLETION => Verdict::fails(
                claim,
                "perturbative quantum GR is not renormalizable; not a UV completion",
            ),
            claims::UNIQUE_VACUUM => {
                if self.r_squared {
                    let xi = QUADRATIC_XI;
                    Verdict::fails(
                        claim,
                        "R² curvature: the Einstein-Hilbert action is not unique",
                    )
                    .with_evidence([format!(
                        "ξ = {xi}; R + ξ R² is not Einstein-Hilbert (ξ → 0 recovers EH)"
                    )])
                } else if self.brans_dicke {
                    let omega = BRANS_DICKE_OMEGA;
                    let gamma = brans_dicke_gamma(omega);
                    Verdict::fails(
                        claim,
                        "Brans-Dicke: the Einstein-Hilbert action is not unique",
                    )
                    .with_evidence([format!(
                        "ω = {omega}; γ = (ω+1)/(ω+2) = {gamma:.4} (ω → ∞ recovers GR)"
                    )])
                } else {
                    Verdict::holds(
                        claim,
                        "classical GR is a unique theory given D and Λ, not a landscape of 10^500 vacua",
                    )
                }
            }
            NEWTON_HALF | EDDINGTON | MERCURY_PERIHELION => {
                if self.brans_dicke {
                    eval_solar_brans_dicke(self.dim, claim)
                } else {
                    eval_solar(true, self.dim, claim)
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by the GR object"),
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.r_squared = parsed.r_squared;
        fork.brans_dicke = parsed.brans_dicke;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mut out: Vec<(String, Box<dyn Theory>)> = Vec::new();
        if !self.r_squared {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::r_squared_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.r_squared {
                    let mut fork = self.clone();
                    fork.r_squared = true;
                    out.push(("add-r-squared".into(), Box::new(fork)));
                }
            }
        }
        if !self.brans_dicke {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::brans_dicke_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.brans_dicke {
                    let mut fork = self.clone();
                    fork.brans_dicke = true;
                    out.push(("add-brans-dicke".into(), Box::new(fork)));
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;

    use crate::gravity::NewtonianGravity;
    use crate::standard_model::StandardModel;

    fn verdict(t: &dyn Theory, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id_str() == id).unwrap();
        t.evaluate(&c).kind
    }

    #[test]
    fn r_squared_curvature_is_ir_not_a_knob() {
        let t = GeneralRelativity::default();
        assert!(
            GeneralRelativity::default()
                .set("r_squared", KnobValue::Bool(true))
                .is_err(),
            "R² curvature is an IR mutation, not a knob"
        );
        assert!(
            GeneralRelativity::default()
                .set("starobinsky", KnobValue::Bool(true))
                .is_err(),
            "Starobinsky R² is an IR mutation, not a knob"
        );
        assert_eq!(
            t.get("dim").unwrap(),
            KnobValue::UInt(4),
            "dim stays a knob"
        );
        let src = render_package(&t.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            GeneralRelativity::from_package(&pkg).unwrap(),
            t,
            "IR round-trip must preserve the Einstein-Hilbert action"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(GeneralRelativity::r_squared_equation()),
        );
        let parsed = GeneralRelativity::from_package(&mutated).unwrap();
        assert!(parsed.r_squared);
        let mut fork = t.clone();
        fork.r_squared = true;
        assert_eq!(fork.id(), "general-relativity");
        assert_eq!(verdict(&fork, claims::UNIQUE_VACUUM), VerdictKind::Fails);
        assert_eq!(verdict(&t, claims::UNIQUE_VACUUM), VerdictKind::Holds);
        assert_eq!(verdict(&fork, EDDINGTON), VerdictKind::Holds);
        assert_eq!(verdict(&fork, MERCURY_PERIHELION), VerdictKind::Holds);
        assert_eq!(verdict(&fork, NEWTON_HALF), VerdictKind::Fails);
        assert_eq!(
            verdict(&fork, claims::GRAVITY),
            VerdictKind::Holds,
            "R² is still dynamical spacetime"
        );
        let uniq = fork
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::UNIQUE_VACUUM)
            .unwrap();
        let v = fork.evaluate(&uniq);
        assert!(
            !v.summary.contains("3GM") && !v.summary.contains("Yukawa"),
            "R² is not the Newton Binet fork: {}",
            v.summary
        );
        assert!(
            !v.summary.contains("2x1") && !v.summary.contains("rectangle"),
            "R² is not the Wilson rectangle fork: {}",
            v.summary
        );
        assert!(
            v.evidence.iter().any(|e| e.contains("ξ = 1")),
            "residual must be the quadratic coupling, got {:?}",
            v.evidence
        );
        let mut high_d = GeneralRelativity::default();
        high_d.set("dim", KnobValue::UInt(5)).unwrap();
        assert_eq!(verdict(&high_d, claims::UNIQUE_VACUUM), VerdictKind::Holds);
        assert_eq!(verdict(&high_d, EDDINGTON), VerdictKind::Inapplicable);
        high_d.r_squared = true;
        assert_eq!(
            verdict(&high_d, claims::UNIQUE_VACUUM),
            VerdictKind::Fails,
            "R² encoding must fail uniqueness even in D=5 where solar tests are inapplicable"
        );
        let mut with_lambda = GeneralRelativity::default();
        with_lambda
            .set("cosmological_constant", KnobValue::Float(0.0))
            .unwrap();
        with_lambda.r_squared = true;
        assert_eq!(
            verdict(&with_lambda, claims::UNIQUE_VACUUM),
            VerdictKind::Fails,
            "R² encoding must fail uniqueness even at Λ = 0 where ξ → 0 recovers EH"
        );
        assert_eq!(QUADRATIC_XI, 1.0);
        let probes = GeneralRelativity::default().structural_mutations();
        assert!(
            probes.iter().any(|(label, _)| label == "add-r-squared"),
            "live GR must offer add-r-squared: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        assert!(
            probes.iter().any(|(label, _)| label == "add-brans-dicke"),
            "live GR must still offer add-brans-dicke"
        );
        let r2_probe = probes
            .iter()
            .find(|(label, _)| label == "add-r-squared")
            .expect("add-r-squared");
        assert_eq!(
            verdict(r2_probe.1.as_ref(), claims::UNIQUE_VACUUM),
            VerdictKind::Fails
        );
        let r2_fork_probes = fork.structural_mutations();
        assert!(
            r2_fork_probes
                .iter()
                .all(|(label, _)| label != "add-r-squared"),
            "R² fork must not re-offer add-r-squared"
        );
        assert!(
            r2_fork_probes
                .iter()
                .any(|(label, _)| label == "add-brans-dicke"),
            "R² fork must still offer add-brans-dicke"
        );
        let live = GeneralRelativity::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let mut dim5 = GeneralRelativity::default();
        dim5.set("dim", KnobValue::UInt(5)).unwrap();
        let rebuilt = dim5.reparse_package(&parsed).unwrap();
        assert_eq!(
            rebuilt.get("dim").unwrap(),
            KnobValue::UInt(5),
            "reparse must overlay R² IR onto live knobs"
        );
        assert_eq!(
            verdict(rebuilt.as_ref(), claims::UNIQUE_VACUUM),
            VerdictKind::Holds
        );
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::UNIQUE_VACUUM)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "GR unique-vacuum must name Einstein-Hilbert plus Λ: {:?}",
            cell.domain()
        );
        assert!(
            NewtonianGravity::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-r-squared"),
            "newtonian-gravity must not grow add-r-squared"
        );
        assert!(
            StandardModel::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-r-squared"),
            "standard-model must not grow add-r-squared; include_higgs stays a knob"
        );
        let mut sm = StandardModel::default();
        sm.set("include_higgs", KnobValue::Bool(false)).unwrap();
        assert_eq!(sm.get("include_higgs").unwrap(), KnobValue::Bool(false));
    }

    #[test]
    fn brans_dicke_scalar_tensor_is_ir_not_a_knob() {
        let t = GeneralRelativity::default();
        assert!(
            GeneralRelativity::default()
                .set("brans_dicke", KnobValue::Bool(true))
                .is_err(),
            "Brans-Dicke is an IR mutation, not a knob"
        );
        assert!(
            GeneralRelativity::default()
                .set("omega", KnobValue::Float(1.0))
                .is_err(),
            "Brans-Dicke omega is not a knob"
        );
        assert!(
            GeneralRelativity::default()
                .set("add-brans-dicke", KnobValue::Bool(true))
                .is_err(),
            "add-brans-dicke is not a knob"
        );
        assert_eq!(
            t.get("dim").unwrap(),
            KnobValue::UInt(4),
            "dim stays a knob"
        );
        let src = render_package(&t.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            pkg.equations.len(),
            1,
            "live package must stay action einstein-hilbert"
        );
        assert_eq!(pkg.equations[0], ACTION_EH);
        assert_eq!(
            GeneralRelativity::from_package(&pkg).unwrap(),
            t,
            "IR round-trip must preserve the Einstein-Hilbert action"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(GeneralRelativity::brans_dicke_equation()),
        );
        let parsed = GeneralRelativity::from_package(&mutated).unwrap();
        assert!(parsed.brans_dicke);
        assert!(
            !parsed.r_squared,
            "Brans-Dicke mutation must not install R²"
        );
        let mut fork = t.clone();
        fork.brans_dicke = true;
        assert_eq!(fork.id(), "general-relativity");
        assert_eq!(verdict(&fork, claims::UNIQUE_VACUUM), VerdictKind::Fails);
        assert_eq!(verdict(&fork, EDDINGTON), VerdictKind::Fails);
        assert_eq!(verdict(&fork, MERCURY_PERIHELION), VerdictKind::Fails);
        assert_eq!(verdict(&fork, NEWTON_HALF), VerdictKind::Fails);
        assert_eq!(verdict(&t, claims::UNIQUE_VACUUM), VerdictKind::Holds);
        assert_eq!(verdict(&t, EDDINGTON), VerdictKind::Holds);
        assert_eq!(verdict(&t, MERCURY_PERIHELION), VerdictKind::Holds);
        assert_eq!(
            verdict(&fork, claims::GRAVITY),
            VerdictKind::Holds,
            "Brans-Dicke is still dynamical spacetime"
        );
        let uniq = fork
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::UNIQUE_VACUUM)
            .unwrap();
        let v = fork.evaluate(&uniq);
        assert!(
            !v.summary.contains("3GM")
                && !v.summary.contains("Yukawa")
                && !v.summary.contains("R²")
                && !v.summary.contains("Newton"),
            "Brans-Dicke is not the Newton or R² fork: {}",
            v.summary
        );
        assert!(
            v.evidence.iter().any(|e| e.contains("ω = 1")),
            "residual must be the Brans-Dicke coupling, got {:?}",
            v.evidence
        );
        let edd = fork
            .claims()
            .into_iter()
            .find(|c| c.id_str() == EDDINGTON)
            .unwrap();
        let ev = fork.evaluate(&edd);
        assert!(
            !ev.summary.contains("Newtonian") && !ev.summary.contains("1911"),
            "Brans-Dicke is not the half-angle fork: {}",
            ev.summary
        );
        assert!(
            ev.evidence.iter().any(|e| e.contains("(1+γ)/2")),
            "Eddington residual must be the PPN light factor, got {:?}",
            ev.evidence
        );
        let gamma = brans_dicke_gamma(BRANS_DICKE_OMEGA);
        let light = brans_dicke_light_factor(gamma);
        let peri = brans_dicke_perihelion_factor(gamma);
        assert!((gamma - 2.0 / 3.0).abs() < 1e-12);
        assert!((light - 5.0 / 6.0).abs() < 1e-12);
        assert!((peri - 7.0 / 9.0).abs() < 1e-12);
        let delta = solar_deflection_arcsec(true) * light;
        assert!(
            (delta - EDDINGTON_ARCSEC).abs() / EDDINGTON_ARCSEC > 0.03,
            "ω = 1 must miss 1.75 arcsec: {delta}"
        );
        assert!(
            (brans_dicke_gamma(1.0e12) - 1.0).abs() < 1e-12,
            "ω → ∞ must recover γ = 1"
        );
        let mut high_d = GeneralRelativity::default();
        high_d.set("dim", KnobValue::UInt(5)).unwrap();
        assert_eq!(verdict(&high_d, claims::UNIQUE_VACUUM), VerdictKind::Holds);
        assert_eq!(verdict(&high_d, EDDINGTON), VerdictKind::Inapplicable);
        high_d.brans_dicke = true;
        assert_eq!(
            verdict(&high_d, claims::UNIQUE_VACUUM),
            VerdictKind::Fails,
            "Brans-Dicke must fail uniqueness even in D=5 where solar tests are inapplicable"
        );
        assert_eq!(verdict(&high_d, EDDINGTON), VerdictKind::Inapplicable);
        let mut with_lambda = GeneralRelativity::default();
        with_lambda
            .set("cosmological_constant", KnobValue::Float(0.0))
            .unwrap();
        with_lambda.brans_dicke = true;
        assert_eq!(
            verdict(&with_lambda, claims::UNIQUE_VACUUM),
            VerdictKind::Fails,
            "Brans-Dicke must fail uniqueness even at Λ = 0 where ω → ∞ recovers GR"
        );
        assert_eq!(BRANS_DICKE_OMEGA, 1.0);
        let probes = GeneralRelativity::default().structural_mutations();
        assert!(
            probes.iter().any(|(label, _)| label == "add-brans-dicke"),
            "live GR must offer add-brans-dicke: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        assert!(
            probes.iter().any(|(label, _)| label == "add-r-squared"),
            "live GR must still offer add-r-squared"
        );
        let probe = probes
            .iter()
            .find(|(label, _)| label == "add-brans-dicke")
            .expect("add-brans-dicke");
        assert_eq!(
            verdict(probe.1.as_ref(), claims::UNIQUE_VACUUM),
            VerdictKind::Fails
        );
        assert_eq!(verdict(probe.1.as_ref(), EDDINGTON), VerdictKind::Fails);
        assert_eq!(
            verdict(probe.1.as_ref(), MERCURY_PERIHELION),
            VerdictKind::Fails
        );
        let fork_probes = fork.structural_mutations();
        assert!(
            fork_probes
                .iter()
                .all(|(label, _)| label != "add-brans-dicke"),
            "Brans-Dicke fork must not re-offer add-brans-dicke"
        );
        assert!(
            fork_probes
                .iter()
                .any(|(label, _)| label == "add-r-squared"),
            "Brans-Dicke fork must still offer add-r-squared"
        );
        let live = GeneralRelativity::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed_live = parse_package(&canonical).unwrap();
        let mut dim5 = GeneralRelativity::default();
        dim5.set("dim", KnobValue::UInt(5)).unwrap();
        let rebuilt = dim5.reparse_package(&parsed_live).unwrap();
        assert_eq!(
            rebuilt.get("dim").unwrap(),
            KnobValue::UInt(5),
            "reparse must overlay Brans-Dicke IR onto live knobs"
        );
        assert_eq!(
            verdict(rebuilt.as_ref(), claims::UNIQUE_VACUUM),
            VerdictKind::Holds,
            "dim still Holds uniqueness on the live Einstein-Hilbert encoding"
        );
        let live_rebuilt = live.reparse_package(&parsed_live).unwrap();
        assert_eq!(
            verdict(live_rebuilt.as_ref(), EDDINGTON),
            VerdictKind::Holds
        );
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == claims::UNIQUE_VACUUM)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "GR unique-vacuum must keep the Einstein-Hilbert domain: {:?}",
            cell.domain()
        );
        assert!(
            NewtonianGravity::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-brans-dicke"),
            "newtonian-gravity must not grow add-brans-dicke"
        );
        assert!(
            StandardModel::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-brans-dicke"),
            "standard-model must not grow add-brans-dicke"
        );
        assert!(
            crate::special_relativity::SpecialRelativity::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-brans-dicke"),
            "special-relativity must not grow add-brans-dicke"
        );
    }
}
