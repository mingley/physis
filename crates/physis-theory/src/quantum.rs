//! Quantum foundations: a fifth domain that puts local realism on trial.
//!
//! [`BellTest`] computes the CHSH correlator `S` for a two-qubit singlet with
//! the optimal measurement angles. Local hidden-variable theories obey
//! `|S| ≤ 2` (the Bell/CHSH bound); quantum mechanics reaches `2√2 ≈ 2.828`
//! (Tsirelson's bound). Computing `S > 2` is a mechanical refutation of local
//! realism — exactly the kind of old assumption this lab exists to scrutinize.
//!
//! A `visibility` knob (Werner-state mixedness) turns the violation off: below
//! `1/√2` the correlations are reproducible by a local model.

use std::f64::consts::PI;

use physis_core::claim::{Claim, Epistemic, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_model::{Complex, Ket, World};

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;

/// The entangled state is normalized (Born rule: probabilities sum to 1).
pub const BORN_NORMALIZATION: &str = "quantum.born-normalization";
/// The CHSH correlator exceeds the local-realism bound of 2.
pub const BELL_VIOLATION: &str = "quantum.bell-violation";
/// The CHSH correlator does not exceed Tsirelson's bound 2√2.
pub const TSIRELSON_BOUND: &str = "quantum.tsirelson-bound";

/// Matrix rows for the quantum-foundations lab.
pub fn quantum_rows() -> [&'static str; 3] {
    [BORN_NORMALIZATION, BELL_VIOLATION, TSIRELSON_BOUND]
}

const SPECS: &[KnobSpec] = &[KnobSpec {
    name: "visibility",
    layer: LayerId::Quantum,
    doc: "Werner-state visibility V in [0,1]. The CHSH correlator scales as V·2√2; below 1/√2 a local model suffices.",
    domain: KnobDomain::Float { min: 0.0, max: 1.0 },
}];

/// A CHSH Bell test on a two-qubit singlet.
#[derive(Clone, Debug)]
pub struct BellTest {
    visibility: f64,
}

impl Default for BellTest {
    fn default() -> Self {
        Self { visibility: 1.0 }
    }
}

impl BellTest {
    /// The singlet state |ψ⁻⟩ = (|01⟩ − |10⟩)/√2 as a 4-dimensional ket.
    fn singlet() -> Ket {
        let s = 1.0 / 2.0_f64.sqrt();
        Ket {
            amps: vec![
                Complex::ZERO,
                Complex::from_re(s),
                Complex::from_re(-s),
                Complex::ZERO,
            ],
        }
    }

    /// The CHSH correlator |S| for the singlet at the optimal angles, scaled by
    /// the visibility. E(a,b) = −V·cos(2(a−b)); optimal angles give |S| = V·2√2.
    fn chsh_s(&self) -> f64 {
        let v = self.visibility;
        let e = |a: f64, b: f64| -v * (2.0 * (a - b)).cos();
        let (a, a2, b, b2) = (0.0, PI / 4.0, PI / 8.0, 3.0 * PI / 8.0);
        (e(a, b) - e(a, b2) + e(a2, b) + e(a2, b2)).abs()
    }
}

impl Knobbed for BellTest {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "visibility" => Ok(KnobValue::Float(self.visibility)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("visibility", KnobValue::Float(v)) => self.visibility = v,
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

impl Theory for BellTest {
    fn id(&self) -> &'static str {
        "bell-test"
    }
    fn name(&self) -> &'static str {
        "CHSH Bell test"
    }
    fn summary(&self) -> &'static str {
        "A CHSH test on a two-qubit singlet. Local hidden-variable theories obey \
         |S| ≤ 2; quantum mechanics computes |S| = 2√2, mechanically refuting \
         local realism. A visibility knob turns the violation off."
    }
    fn world(&self) -> Option<World> {
        None // quantum foundations live on the quantum/information layers
    }
    fn note(&self) -> String {
        format!(
            "CHSH singlet, visibility = {}, S = {:.3}",
            self.visibility,
            self.chsh_s()
        )
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                BORN_NORMALIZATION,
                "The entangled state is normalized.",
                LayerId::Quantum,
                Epistemic::Theorem,
            ),
            Claim::new(
                BELL_VIOLATION,
                "The CHSH correlator exceeds the local-realism bound of 2.",
                LayerId::Quantum,
                Epistemic::Theorem,
            ),
            Claim::new(
                TSIRELSON_BOUND,
                "The CHSH correlator does not exceed Tsirelson's bound 2√2.",
                LayerId::Quantum,
                Epistemic::Theorem,
            ),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id.0.as_str() {
            BORN_NORMALIZATION => {
                let psi = Self::singlet();
                let n = psi.norm_sqr();
                let p_sum: f64 = (0..psi.dim()).filter_map(|i| psi.born(i)).sum();
                if (n - 1.0).abs() < 1e-12 && (p_sum - 1.0).abs() < 1e-12 {
                    Verdict::holds(Epistemic::Theorem, "⟨ψ|ψ⟩ = 1 and Σ pᵢ = 1")
                        .with_evidence([format!("norm² = {n:.6}, Σ pᵢ = {p_sum:.6}")])
                } else {
                    Verdict::fails(Epistemic::Theorem, "state is not normalized")
                }
            }
            BELL_VIOLATION => {
                let s = self.chsh_s();
                if s > 2.0 + 1e-12 {
                    Verdict::holds(
                        Epistemic::Theorem,
                        format!("CHSH S = {s:.3} > 2: local realism is refuted"),
                    )
                    .with_evidence([
                        "local hidden-variable theories obey |S| ≤ 2 (Bell/CHSH)".to_string(),
                    ])
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        format!(
                            "CHSH S = {s:.3} ≤ 2: reproducible by a local hidden-variable model"
                        ),
                    )
                }
            }
            TSIRELSON_BOUND => {
                let s = self.chsh_s();
                let tsirelson = 2.0 * 2.0_f64.sqrt();
                if s <= tsirelson + 1e-9 {
                    Verdict::holds(
                        Epistemic::Theorem,
                        format!("CHSH S = {s:.3} ≤ 2√2 ≈ {tsirelson:.3} (Tsirelson bound)"),
                    )
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        "CHSH S exceeds Tsirelson's bound — impossible in quantum mechanics",
                    )
                }
            }
            _ => Verdict::inapplicable("claim not made by a quantum-foundations object"),
        }
    }
}

/// The quantum-foundations experiment: a CHSH Bell test.
pub fn bell() -> ExperimentReport {
    let theories: Vec<Box<dyn Theory>> = vec![Box::new(BellTest::default())];
    report_from_rows(
        "bell",
        "Quantum foundations lab",
        "Can local realism survive? The CHSH correlator for a singlet is computed \
         from the quantum state; |S| > 2 refutes local hidden variables, and the \
         quantum value saturates Tsirelson's bound 2√2.",
        "The Born rule, the CHSH value, and Tsirelson's bound are computed from \
         the two-qubit state. Local realism is a falsifiable assumption here, and \
         it fails.",
        vec![
            "`holds` / `fails` are internal to the encoding.".into(),
            "S is computed at the optimal CHSH angles; the classical bound is 2, the quantum (Tsirelson) bound is 2√2.".into(),
            "`set bell-test visibility 0.5` drops S below 2 — a local model then suffices.".into(),
        ],
        &quantum_rows(),
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
    fn singlet_saturates_tsirelson_and_violates_bell() {
        let t = BellTest::default();
        assert!((t.chsh_s() - 2.0 * 2.0_f64.sqrt()).abs() < 1e-9);
        assert_eq!(verdict(&t, BELL_VIOLATION), VerdictKind::Holds);
        assert_eq!(verdict(&t, TSIRELSON_BOUND), VerdictKind::Holds);
        assert_eq!(verdict(&t, BORN_NORMALIZATION), VerdictKind::Holds);
    }

    #[test]
    fn low_visibility_restores_local_realism() {
        // The quantum knob → verdict diff.
        let mut t = BellTest::default();
        assert_eq!(verdict(&t, BELL_VIOLATION), VerdictKind::Holds);
        t.set("visibility", KnobValue::Float(0.5)).unwrap();
        assert_eq!(verdict(&t, BELL_VIOLATION), VerdictKind::Fails);
        // Tsirelson still holds (S only got smaller).
        assert_eq!(verdict(&t, TSIRELSON_BOUND), VerdictKind::Holds);
    }

    #[test]
    fn quantum_experiment_builds_a_matrix() {
        let r = bell();
        assert_eq!(r.id, "bell");
        assert_eq!(
            r.matrix
                .get(BELL_VIOLATION)
                .and_then(|m| m.get("bell-test"))
                .copied(),
            Some(VerdictKind::Holds)
        );
    }
}
