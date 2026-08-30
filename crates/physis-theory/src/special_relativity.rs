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
//! The `absolute_time` knob switches the boost from Lorentz to **Galilean**
//! (`t` absolute). Every one of these theorems then fails — the interval is not
//! invariant, velocities add past `c`, and the mass shell is not preserved.
//! This is the Galilean→Einstein revolution as a single mechanical knob turn.

use physis_core::claim::{Claim, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::ParameterOrigin;
use physis_core::{Energy, Momentum, Qty};
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

const SPECS: &[KnobSpec] = &[KnobSpec {
    name: "absolute_time",
    layer: LayerId::Spacetime,
    doc: "If true, boosts are Galilean (time is absolute) instead of Lorentzian. Turning this on breaks every relativistic invariant — the pre-1905 worldview.",
    origin: ParameterOrigin::Chosen,
    domain: KnobDomain::Bool,
}];

/// Special relativity: flat Minkowski kinematics with a Galilean-toggle knob.
#[derive(Clone, Debug, Default)]
pub struct SpecialRelativity {
    /// If true, use Galilean boosts (absolute time) instead of Lorentzian.
    absolute_time: bool,
}

impl SpecialRelativity {
    /// Boost a two-vector `(a0, a1)` by `beta` (in units of `c`), using the
    /// Lorentz transform, or the Galilean one when `absolute_time` is set.
    ///
    /// Both the spacetime coordinates `(ct, x)` and the energy–momentum
    /// `(E, pc)` are 4-vectors, so the same transform applies to each.
    fn boost(&self, a0: f64, a1: f64, beta: f64) -> (f64, f64) {
        if self.absolute_time {
            // Galilean: the timelike/energy component is absolute; only the
            // spacelike/momentum component shears. (a0' , a1' − β·a0').
            (a0, a1 - beta * a0)
        } else {
            let gamma = 1.0 / (1.0 - beta * beta).sqrt();
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
         composition, and the mass shell E² = (pc)² + (mc²)², all computed. An \
         absolute_time knob replaces Lorentz boosts with Galilean ones and \
         breaks every invariant — the pre-relativistic worldview, mechanized."
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
                if self.absolute_time {
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
                // A timelike event, boosted by BETA.
                let ct0 = c * 1.0e-8; // c·(10 ns)
                let x0 = 2.0; // metres
                let s0 = ct0 * ct0 - x0 * x0;
                let (ct1, x1) = self.boost(ct0, x0, BETA);
                let s1 = ct1 * ct1 - x1 * x1;
                let invariant = (s0 - s1).abs() <= 1e-9 * s0.abs();
                if invariant {
                    Verdict::holds(claim, "s² is unchanged by the Lorentz boost")
                        .with_evidence([format!("s² = {s0:.4e} m² before and {s1:.4e} m² after")])
                } else {
                    Verdict::fails(
                        claim,
                        "the interval is not invariant under a Galilean boost",
                    )
                    .with_evidence([format!("s² = {s0:.4e} m² → {s1:.4e} m² (changed)")])
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
                // Typed quantities: mc² and pc are *forced* to be energies.
                let m = electron_mass();
                let mc2: Qty<Energy> = m * C * C;
                // A particle at rest has (E, pc) = (mc², 0); boost it.
                let (e1, pc1) = self.boost(mc2.value(), 0.0, BETA);
                let shell = e1 * e1 - pc1 * pc1;
                let rest = mc2.value() * mc2.value();
                // The momentum after the boost, as a typed quantity for the note.
                let p1: Qty<Momentum> = Qty::new(pc1 / c);
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
            _ => Verdict::inapplicable(claim, "claim not made by the special-relativity object"),
        }
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
}
