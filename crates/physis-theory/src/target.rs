//! The empirical target as **data**, not a function.
//!
//! `data/empirical-world.json` states what any candidate theory must reproduce
//! at low energy: 3+1 spacetime, a gauge sector containing the Standard Model,
//! chiral fermions in three generations, and gravity. Theories are *scored*
//! against this fixture by projecting their [`World`](physis_model::World) and
//! comparing, so the requirements live in a checked-in file rather than in a
//! hand-written `empirical_target()` body.

use serde::Deserialize;

use crate::framework::Theory;

/// The low-energy requirements a theory is scored against.
#[derive(Clone, Debug, Deserialize)]
pub struct EmpiricalTarget {
    /// Observed (non-compact) spacetime dimension.
    pub observed_dim: i16,
    /// Whether the gauge sector must contain the Standard Model.
    pub gauge_contains_sm: bool,
    /// Whether low-energy fermions are required.
    pub has_fermions: bool,
    /// Required number of charged-lepton generations.
    pub charged_lepton_generations: usize,
    /// Whether gravity (a massless spin-2) is required.
    pub has_gravity: bool,
    /// Human-readable description of the target.
    pub note: String,
}

/// Load the empirical target from the committed JSON fixture.
///
/// The fixture is embedded at build time, so the requirements are versioned
/// data in `data/empirical-world.json`, parsed by `serde`, not a code literal.
pub fn empirical_target() -> EmpiricalTarget {
    serde_json::from_str(include_str!("../../../data/empirical-world.json"))
        .expect("data/empirical-world.json is a valid EmpiricalTarget fixture")
}

/// One requirement checked against a theory.
#[derive(Clone, Debug)]
pub struct Check {
    /// Requirement name.
    pub name: String,
    /// Required value (as text).
    pub required: String,
    /// The theory's actual value (as text).
    pub actual: String,
    /// Whether the requirement is met.
    pub pass: bool,
}

/// A theory's score against the empirical target.
#[derive(Clone, Debug)]
pub struct Scorecard {
    /// Theory id.
    pub theory: String,
    /// One entry per requirement.
    pub checks: Vec<Check>,
}

impl Scorecard {
    /// Number of requirements met.
    pub fn passed(&self) -> usize {
        self.checks.iter().filter(|c| c.pass).count()
    }

    /// Total number of requirements.
    pub fn total(&self) -> usize {
        self.checks.len()
    }

    /// Whether every requirement is met.
    pub fn perfect(&self) -> bool {
        self.checks.iter().all(|c| c.pass)
    }

    /// Human-readable scorecard.
    pub fn render(&self) -> String {
        let mut out = format!("score {} against empirical target\n", self.theory);
        for c in &self.checks {
            out.push_str(&format!(
                "  [{}] {:<28} required {:<6} actual {}\n",
                if c.pass { "ok" } else { "XX" },
                c.name,
                c.required,
                c.actual
            ));
        }
        out.push_str(&format!(
            "\n{}/{} requirements met{}\n",
            self.passed(),
            self.total(),
            if self.perfect() {
                " — reproduces the low-energy world"
            } else {
                ""
            }
        ));
        out
    }
}

/// Score a theory's projected world against the empirical target.
pub fn score(target: &EmpiricalTarget, theory: &dyn Theory) -> Scorecard {
    let mut checks = Vec::new();
    let Some(w) = theory.world() else {
        // A non-physics domain (e.g. computation) has no world to grade against
        // the physics target; say so honestly rather than faking a score.
        checks.push(Check {
            name: "physics-domain".into(),
            required: "physics world".into(),
            actual: "none (non-physics domain)".into(),
            pass: false,
        });
        return Scorecard {
            theory: theory.id().into(),
            checks,
        };
    };

    let observed = w.spacetime.observed_dim();
    checks.push(Check {
        name: "observed_dim".into(),
        required: target.observed_dim.to_string(),
        actual: observed.to_string(),
        pass: observed == target.observed_dim,
    });

    let gauge_sm = w.gauge.sm_embed().contains_sm();
    checks.push(Check {
        name: "gauge_contains_sm".into(),
        required: target.gauge_contains_sm.to_string(),
        actual: gauge_sm.to_string(),
        pass: gauge_sm == target.gauge_contains_sm,
    });

    let fermions = w.spectrum.has_fermions();
    checks.push(Check {
        name: "has_fermions".into(),
        required: target.has_fermions.to_string(),
        actual: fermions.to_string(),
        pass: fermions == target.has_fermions,
    });

    let gens = w.spectrum.charged_lepton_generations();
    checks.push(Check {
        name: "charged_lepton_generations".into(),
        required: target.charged_lepton_generations.to_string(),
        actual: gens.to_string(),
        pass: gens == target.charged_lepton_generations,
    });

    checks.push(Check {
        name: "has_gravity".into(),
        required: target.has_gravity.to_string(),
        actual: w.has_gravity.to_string(),
        pass: w.has_gravity == target.has_gravity,
    });

    Scorecard {
        theory: theory.id().into(),
        checks,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::ObserverGeometry;
    use crate::relativity::GeneralRelativity;
    use crate::standard_model::StandardModel;
    use crate::strings::StringTheory;

    #[test]
    fn fixture_parses_from_json() {
        let t = empirical_target();
        assert_eq!(t.observed_dim, 4);
        assert!(t.gauge_contains_sm);
        assert_eq!(t.charged_lepton_generations, 3);
    }

    #[test]
    fn standard_model_misses_only_gravity() {
        // The SM reproduces the low-energy world except gravity — exactly its
        // famous deficiency, now surfaced mechanically by the scorecard.
        let card = score(&empirical_target(), &StandardModel::default());
        assert!(!card.perfect(), "{}", card.render());
        assert_eq!(card.passed(), card.total() - 1);
        assert!(card
            .checks
            .iter()
            .any(|c| c.name == "has_gravity" && !c.pass));
    }

    #[test]
    fn standard_model_plus_gravity_reproduces_the_target() {
        use physis_core::knob::{KnobValue, Knobbed};
        let mut sm = StandardModel::default();
        sm.set("include_gravity", KnobValue::Bool(true)).unwrap();
        assert!(score(&empirical_target(), &sm).perfect());
    }

    #[test]
    fn heterotic_reproduces_the_target_but_type_iib_lacks_gauge() {
        let target = empirical_target();
        // Heterotic E8xE8 carries an SM-embedding gauge sector: it meets every
        // low-energy requirement in this coarse encoding (the critique is about
        // predictivity/uniqueness, not low-energy content).
        assert!(score(&target, &StringTheory::heterotic_e8()).perfect());
        // Type IIB has no perturbative 10D GUT gauge group, so it misses the
        // gauge requirement.
        let iib = score(&target, &StringTheory::type_iib());
        assert!(!iib.perfect());
        assert!(iib
            .checks
            .iter()
            .any(|c| c.name == "gauge_contains_sm" && !c.pass));
    }

    #[test]
    fn controls_and_bosonic_miss_requirements() {
        let target = empirical_target();
        // GR has no SM gauge or matter; the bosonic string has no fermions.
        assert!(!score(&target, &GeneralRelativity::default()).perfect());
        assert!(!score(&target, &StringTheory::bosonic()).perfect());
    }

    #[test]
    fn observer_geometry_asserts_the_low_energy_content() {
        // The scaffold *claims* SM content (via its Spin(10) conjecture), so it
        // meets the low-energy fixture — the critique is about predictivity and
        // whether that content is derived, not about the content itself.
        assert!(score(&empirical_target(), &ObserverGeometry::default()).perfect());
    }
}
