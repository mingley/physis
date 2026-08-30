//! Special relativity as mechanized kinematics — and a knob that turns it off.
//!
//! Einstein's 1905 kinematics is not asserted here; it is *computed*. Three
//! invariants are checked directly:
//!
//! - the spacetime interval `s² = (cΔt)² − Δx²` is unchanged by a boost,
//! - composing two subluminal velocities stays subluminal, and
//! - the energy–momentum invariant `E² − (pc)² = (mc²)²` is frame-independent,
//!   built from *typed* quantities so `pc` and `mc²` are forced to be energies.
//!
//! The Lorentz boost lives on the IR package. A truncated binomial γ
//! (`add-binomial-gamma`) is a package mutation, not the `absolute_time`
//! knob: interval and mass-shell fail on that fork. `absolute_time` still
//! switches exact Lorentz to Galilean. Velocity composition stays Einstein
//! on the binomial fork.

use physis_core::claim::{Claim, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::ParameterOrigin;
use physis_core::{Energy, Momentum, Qty};
use physis_ir::{apply_mutation, parse_package, render_package, PackageMutation, TheoryPackage};
use physis_model::constants::{electron_mass, C};
use physis_model::{GaugeGroup, Manifold, Spectrum, World};
use physis_proof::lookup;

use crate::framework::Theory;

/// The spacetime interval is invariant under the boost.
pub const SR_INVARIANT_INTERVAL: &str = "sr.invariant-interval";
/// Composing two subluminal velocities stays subluminal.
pub const SR_SUBLUMINAL_COMPOSITION: &str = "sr.subluminal-composition";
/// The energy–momentum invariant `E² − (pc)² = (mc²)²` is frame-independent.
pub const SR_ENERGY_MOMENTUM: &str = "sr.energy-momentum-invariant";

/// The demonstration boost speed, as a fraction of `c`.
const BETA: f64 = 0.6;
/// Tiny boost where a truncated γ can look Lorentzian in a sample.
#[cfg(test)]
const BETA_VANISHING: f64 = 1.0e-6;
/// Exact Lorentz boost on the live SR package.
const BOOST_LORENTZ: &str = "boost lorentz";
/// Truncated binomial γ = 1 + β²/2.
const BOOST_BINOMIAL: &str = "boost binomial-gamma";

fn parse_sr_boost(pkg: &TheoryPackage) -> Result<bool, String> {
    let mut lorentz = false;
    let mut binomial = false;
    for eq in &pkg.equations {
        match eq.trim() {
            BOOST_LORENTZ => lorentz = true,
            BOOST_BINOMIAL => binomial = true,
            _ => {}
        }
    }
    if !lorentz {
        return Err(format!("{} package has no Lorentz boost", pkg.id));
    }
    Ok(binomial)
}

fn gamma_lorentz(beta: f64) -> f64 {
    1.0 / (1.0 - beta * beta).sqrt()
}

fn gamma_binomial(beta: f64) -> f64 {
    1.0 + 0.5 * beta * beta
}

/// Residual γ_L − γ_bin. Evidence, not the encoding. β → 0 recovers
/// Lorentz and the interval cell still fails.
fn binomial_gamma_residual(beta: f64) -> f64 {
    gamma_lorentz(beta) - gamma_binomial(beta)
}

const SPECS: &[KnobSpec] = &[KnobSpec {
    name: "absolute_time",
    layer: LayerId::Spacetime,
    doc: "If true, boosts are Galilean (time is absolute) instead of Lorentzian. Turning this on breaks every relativistic invariant — the pre-1905 worldview.",
    origin: ParameterOrigin::Chosen,
    domain: KnobDomain::Bool,
}];

/// Special relativity: flat Minkowski kinematics with a Galilean-toggle knob.
///
/// The Lorentz boost lives on the IR package. Truncated binomial γ
/// (`add-binomial-gamma`) is a package mutation, not a knob: interval
/// and mass-shell fail. `absolute_time` still selects Galilean boosts.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SpecialRelativity {
    /// If true, use Galilean boosts (absolute time) instead of Lorentzian.
    absolute_time: bool,
    /// Whether the encoding uses γ = 1 + β²/2 instead of exact Lorentz.
    binomial_gamma: bool,
}

impl SpecialRelativity {
    /// IR package for this boost encoding. Equations are `boost lorentz`
    /// and, when forked, `boost binomial-gamma`. `absolute_time` stays
    /// on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![BOOST_LORENTZ.to_string()];
        if self.binomial_gamma {
            equations.push(BOOST_BINOMIAL.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["lorentz-boost".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: SR_INVARIANT_INTERVAL.into(),
                statement: "The spacetime interval s² = (cΔt)² − Δx² is invariant under a boost."
                    .into(),
                layer: "spacetime".into(),
                class: "model-internal".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load a boost encoding from a package. Knobs default; overlay them
    /// from a live SR object when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        if pkg.id != "special-relativity" {
            return Err(format!(
                "special-relativity package id '{}' is not special-relativity",
                pkg.id
            ));
        }
        let binomial_gamma = parse_sr_boost(pkg)?;
        Ok(Self {
            binomial_gamma,
            ..Self::default()
        })
    }

    fn binomial_equation() -> String {
        BOOST_BINOMIAL.to_string()
    }

    /// Boost a two-vector `(a0, a1)` by `beta` (in units of `c`).
    ///
    /// Binomial γ is an IR encoding. Otherwise Lorentz, or Galilean when
    /// `absolute_time` is set. Both `(ct, x)` and `(E, pc)` are 4-vectors.
    fn boost(&self, a0: f64, a1: f64, beta: f64) -> (f64, f64) {
        if self.binomial_gamma {
            let gamma = gamma_binomial(beta);
            (gamma * (a0 - beta * a1), gamma * (a1 - beta * a0))
        } else if self.absolute_time {
            (a0, a1 - beta * a0)
        } else {
            let gamma = gamma_lorentz(beta);
            (gamma * (a0 - beta * a1), gamma * (a1 - beta * a0))
        }
    }

    /// Relativistic (or, under the knob, Galilean) composition of two speeds
    /// given as fractions of `c`.
    fn compose_speeds(&self, u: f64, v: f64) -> f64 {
        if self.absolute_time {
            u + v
        } else {
            (u + v) / (1.0 + u * v)
        }
    }
}

impl Knobbed for SpecialRelativity {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "absolute_time" => Ok(KnobValue::Bool(self.absolute_time)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("absolute_time", KnobValue::Bool(v)) => self.absolute_time = v,
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

impl Theory for SpecialRelativity {
    fn id(&self) -> &'static str {
        "special-relativity"
    }
    fn name(&self) -> &'static str {
        "Special relativity"
    }
    fn summary(&self) -> &'static str {
        "Flat Minkowski kinematics: the invariant interval, subluminal velocity \
         composition, and the mass shell E² = (pc)² + (mc²)², all computed. The \
         Lorentz boost is an IR encoding. Truncated binomial γ is an IR mutation, \
         not the absolute_time knob. That knob still replaces exact Lorentz with \
         Galilean boosts."
    }
    fn world(&self) -> Option<World> {
        Some(World {
            spacetime: Manifold::observed_4d(),
            gauge: GaugeGroup::trivial(),
            spectrum: Spectrum::empty(),
            has_gravity: false,
            supersymmetric: false,
            free_parameter_count: 0, // c is a unit conversion, not a free parameter
            landscape_log10: 0.0,
            note: format!(
                "special relativity, boosts = {}",
                if self.binomial_gamma {
                    "binomial-γ"
                } else if self.absolute_time {
                    "Galilean"
                } else {
                    "Lorentzian"
                }
            ),
        })
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            lookup(SR_INVARIANT_INTERVAL)
                .expect("interval is a catalog identity")
                .lab_claim(),
            lookup(SR_SUBLUMINAL_COMPOSITION)
                .expect("composition is a catalog identity")
                .lab_claim()
                .with_dependencies(&[SR_INVARIANT_INTERVAL]),
            lookup(SR_ENERGY_MOMENTUM)
                .expect("mass shell is a catalog identity")
                .lab_claim()
                .with_dependencies(&[SR_INVARIANT_INTERVAL]),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        let c = C.value();
        match claim.id_str() {
            SR_INVARIANT_INTERVAL => {
                let ct0 = c * 1.0e-8;
                let x0 = 2.0;
                let s0 = ct0 * ct0 - x0 * x0;
                let (ct1, x1) = self.boost(ct0, x0, BETA);
                let s1 = ct1 * ct1 - x1 * x1;
                if self.binomial_gamma {
                    let residual = binomial_gamma_residual(BETA);
                    Verdict::fails(claim, "binomial γ: the interval is not Lorentz-invariant")
                        .with_evidence([format!(
                        "γ_L − γ_bin = {residual:.6} at β = {BETA}; s² = {s0:.4e} m² → {s1:.4e} m²"
                    )])
                } else {
                    let invariant = (s0 - s1).abs() <= 1e-9 * s0.abs();
                    if invariant {
                        Verdict::holds(claim, "s² is unchanged by the Lorentz boost").with_evidence(
                            [format!("s² = {s0:.4e} m² before and {s1:.4e} m² after")],
                        )
                    } else {
                        Verdict::fails(
                            claim,
                            "the interval is not invariant under a Galilean boost",
                        )
                        .with_evidence([format!("s² = {s0:.4e} m² → {s1:.4e} m² (changed)")])
                    }
                }
            }
            SR_SUBLUMINAL_COMPOSITION => {
                let (u, v) = (0.8, 0.7);
                let w = self.compose_speeds(u, v);
                if w < 1.0 {
                    Verdict::holds(claim, "relativistic composition keeps the result below c")
                        .with_evidence([format!("0.8c ⊕ 0.7c = {w:.4}c < c")])
                } else {
                    Verdict::fails(claim, "Galilean addition exceeds c")
                        .with_evidence([format!("0.8c + 0.7c = {w:.4}c ≥ c")])
                }
            }
            SR_ENERGY_MOMENTUM => {
                let m = electron_mass();
                let mc2: Qty<Energy> = m * C * C;
                let (e1, pc1) = self.boost(mc2.value(), 0.0, BETA);
                let shell = e1 * e1 - pc1 * pc1;
                let rest = mc2.value() * mc2.value();
                let p1: Qty<Momentum> = Qty::new(pc1 / c);
                if self.binomial_gamma {
                    let residual = binomial_gamma_residual(BETA);
                    Verdict::fails(
                        claim,
                        "binomial γ: the mass shell is not Lorentz-invariant",
                    )
                    .with_evidence([format!(
                        "γ_L − γ_bin = {residual:.6} at β = {BETA}; E² − (pc)² = {shell:.4e} J² vs (mc²)² = {rest:.4e} J²"
                    )])
                } else {
                    let invariant = (shell - rest).abs() <= 1e-9 * rest.abs();
                    if invariant {
                        Verdict::holds(claim, "E² − (pc)² equals (mc²)² in the boosted frame")
                            .with_evidence([
                                format!(
                                    "mc² = {:.4e} J, boosted |p| = {:.4e} kg·m/s",
                                    mc2.value(),
                                    p1.value()
                                ),
                                format!("E² − (pc)² = {shell:.4e} J² vs (mc²)² = {rest:.4e} J²"),
                            ])
                    } else {
                        Verdict::fails(claim, "the mass shell is not preserved by a Galilean boost")
                            .with_evidence([format!(
                                "E² − (pc)² = {shell:.4e} J² ≠ (mc²)² = {rest:.4e} J²"
                            )])
                    }
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by the special-relativity object"),
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.binomial_gamma = parsed.binomial_gamma;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mut out: Vec<(String, Box<dyn Theory>)> = Vec::new();
        if !self.binomial_gamma {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::binomial_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.binomial_gamma {
                    let mut fork = self.clone();
                    fork.binomial_gamma = true;
                    out.push(("add-binomial-gamma".into(), Box::new(fork)));
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

    fn kind(t: &dyn Theory, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id_str() == id).unwrap();
        t.evaluate(&c).kind
    }

    #[test]
    fn lorentz_kinematics_holds_all_invariants() {
        let sr = SpecialRelativity::default();
        assert!(!sr.absolute_time);
        assert_eq!(kind(&sr, SR_INVARIANT_INTERVAL), VerdictKind::Holds);
        assert_eq!(kind(&sr, SR_SUBLUMINAL_COMPOSITION), VerdictKind::Holds);
        assert_eq!(kind(&sr, SR_ENERGY_MOMENTUM), VerdictKind::Holds);
    }

    #[test]
    fn absolute_time_knob_breaks_every_invariant() {
        // The Galilean→Einstein revolution as one knob turn: flipping to
        // absolute time makes all three relativistic theorems fail.
        let mut sr = SpecialRelativity::default();
        sr.set("absolute_time", KnobValue::Bool(true)).unwrap();
        assert_eq!(kind(&sr, SR_INVARIANT_INTERVAL), VerdictKind::Fails);
        assert_eq!(kind(&sr, SR_SUBLUMINAL_COMPOSITION), VerdictKind::Fails);
        assert_eq!(kind(&sr, SR_ENERGY_MOMENTUM), VerdictKind::Fails);
    }

    #[test]
    fn mass_shell_depends_on_the_interval() {
        let sr = SpecialRelativity::default();
        let em = sr
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_ENERGY_MOMENTUM)
            .unwrap();
        assert_eq!(em.depends_on[0].0, SR_INVARIANT_INTERVAL);
    }

    #[test]
    fn velocity_composition_numbers_are_right() {
        let sr = SpecialRelativity::default();
        // 0.8c ⊕ 0.7c = 1.5 / 1.56 ≈ 0.9615c.
        assert!((sr.compose_speeds(0.8, 0.7) - 1.5 / 1.56).abs() < 1e-12);
        // Galilean would give 1.5c, superluminal.
        let mut g = SpecialRelativity::default();
        g.set("absolute_time", KnobValue::Bool(true)).unwrap();
        assert!((g.compose_speeds(0.8, 0.7) - 1.5).abs() < 1e-12);
    }

    #[test]
    fn interval_invariance_is_exact_under_lorentz() {
        let sr = SpecialRelativity::default();
        let c = C.value();
        let (ct0, x0) = (c * 1.0e-8, 2.0);
        let (ct1, x1) = sr.boost(ct0, x0, BETA);
        let s0 = ct0 * ct0 - x0 * x0;
        let s1 = ct1 * ct1 - x1 * x1;
        assert!((s0 - s1).abs() <= 1e-9 * s0.abs());
    }

    #[test]
    fn binomial_gamma_is_ir_not_a_knob() {
        let t = SpecialRelativity::default();
        assert!(
            SpecialRelativity::default()
                .set("binomial_gamma", KnobValue::Bool(true))
                .is_err(),
            "binomial γ is an IR mutation, not a knob"
        );
        assert!(SpecialRelativity::default()
            .set("gamma", KnobValue::Bool(true))
            .is_err());
        assert_eq!(
            t.get("absolute_time").unwrap(),
            KnobValue::Bool(false),
            "absolute_time stays a knob"
        );
        let src = render_package(&t.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            SpecialRelativity::from_package(&pkg).unwrap(),
            t,
            "IR round-trip must preserve the Lorentz boost"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(SpecialRelativity::binomial_equation()),
        );
        let parsed = SpecialRelativity::from_package(&mutated).unwrap();
        assert!(parsed.binomial_gamma);
        let mut fork = t.clone();
        fork.binomial_gamma = true;
        assert_eq!(fork.id(), "special-relativity");
        assert_eq!(kind(&fork, SR_INVARIANT_INTERVAL), VerdictKind::Fails);
        assert_eq!(kind(&fork, SR_ENERGY_MOMENTUM), VerdictKind::Fails);
        assert_eq!(
            kind(&fork, SR_SUBLUMINAL_COMPOSITION),
            VerdictKind::Holds,
            "binomial γ is not the Galilean composition fork"
        );
        assert_eq!(kind(&t, SR_INVARIANT_INTERVAL), VerdictKind::Holds);
        let interval = fork
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_INVARIANT_INTERVAL)
            .unwrap();
        let v = fork.evaluate(&interval);
        assert!(
            !v.summary.contains("Galilean"),
            "binomial γ is not the absolute_time knob: {}",
            v.summary
        );
        let residual = binomial_gamma_residual(BETA);
        assert!(
            residual.abs() > 0.05,
            "residual must be the γ mismatch, not a unit flag, got {residual}"
        );
        assert!(
            v.evidence.iter().any(|e| e.contains("γ_L − γ_bin")),
            "got {:?}",
            v.evidence
        );
        let c = C.value();
        let (ct0, x0) = (c * 1.0e-8, 2.0);
        let (ct1, x1) = fork.boost(ct0, x0, BETA_VANISHING);
        let s0 = ct0 * ct0 - x0 * x0;
        let s1 = ct1 * ct1 - x1 * x1;
        assert!(
            (s0 - s1).abs() <= 1e-9 * s0.abs(),
            "tiny β makes the sample look Lorentzian; residual is not the encoding"
        );
        assert_eq!(
            kind(&fork, SR_INVARIANT_INTERVAL),
            VerdictKind::Fails,
            "binomial encoding must fail the interval even when a tiny-β sample looks invariant"
        );
        let mut galilean = SpecialRelativity::default();
        galilean
            .set("absolute_time", KnobValue::Bool(true))
            .unwrap();
        assert_eq!(kind(&galilean, SR_INVARIANT_INTERVAL), VerdictKind::Fails);
        assert_eq!(
            kind(&galilean, SR_SUBLUMINAL_COMPOSITION),
            VerdictKind::Fails
        );
        let probes = SpecialRelativity::default().structural_mutations();
        assert!(
            probes
                .iter()
                .any(|(label, _)| label == "add-binomial-gamma"),
            "live SR must offer add-binomial-gamma: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let probe = probes
            .iter()
            .find(|(label, _)| label == "add-binomial-gamma")
            .expect("add-binomial-gamma");
        assert_eq!(
            kind(probe.1.as_ref(), SR_INVARIANT_INTERVAL),
            VerdictKind::Fails
        );
        let fork_probes = fork.structural_mutations();
        assert!(
            fork_probes
                .iter()
                .all(|(label, _)| label != "add-binomial-gamma"),
            "binomial fork must not re-offer add-binomial-gamma"
        );
        let live = SpecialRelativity::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let mut abs = SpecialRelativity::default();
        abs.set("absolute_time", KnobValue::Bool(true)).unwrap();
        let rebuilt = abs.reparse_package(&parsed).unwrap();
        assert_eq!(
            rebuilt.get("absolute_time").unwrap(),
            KnobValue::Bool(true),
            "reparse must overlay binomial IR onto live knobs"
        );
        assert_eq!(
            kind(rebuilt.as_ref(), SR_INVARIANT_INTERVAL),
            VerdictKind::Fails,
            "absolute_time still Fails interval on the live Lorentz encoding"
        );
        let live_rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(
            kind(live_rebuilt.as_ref(), SR_INVARIANT_INTERVAL),
            VerdictKind::Holds
        );
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == SR_INVARIANT_INTERVAL)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "interval must keep the catalog Minkowski domain: {:?}",
            cell.domain()
        );
        assert!(
            crate::relativity::GeneralRelativity::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-binomial-gamma"),
            "general-relativity must not grow add-binomial-gamma"
        );
        assert!(
            crate::computation::TuringMachine::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-binomial-gamma"),
            "turing-machine must not grow add-binomial-gamma"
        );
    }
}
