//! Quantum foundations: a fifth domain that puts local realism on trial.
//!
//! [`BellTest`] computes the CHSH correlator `S` for a two-qubit singlet with
//! the optimal measurement angles. The correlator `E(a,b) = −cos(a−b)` is not
//! assumed — it is the operator expectation `⟨ψ⁻|σ(a)⊗σ(b)|ψ⁻⟩`, verified
//! against the closed form by `quantum.correlator-from-operators`. Local
//! hidden-variable theories obey `|S| ≤ 2` (the Bell/CHSH bound); quantum
//! mechanics reaches `2√2 ≈ 2.828` (Tsirelson's bound). Computing `S > 2` is a
//! mechanical refutation of local realism — exactly the kind of old assumption
//! this lab exists to scrutinize.
//!
//! A `visibility` knob (Werner-state mixedness) turns the violation off: below
//! `1/√2` the correlations are reproducible by a local model.

use std::f64::consts::PI;

use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::ParameterOrigin;
use physis_model::{expectation4, spin_measurement, tensor2, Complex, Ket, World};

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;

/// The entangled state is normalized (Born rule: probabilities sum to 1).
pub const BORN_NORMALIZATION: &str = "quantum.born-normalization";
/// The CHSH correlator exceeds the local-realism bound of 2.
pub const BELL_VIOLATION: &str = "quantum.bell-violation";
/// The CHSH correlator does not exceed Tsirelson's bound 2√2.
pub const TSIRELSON_BOUND: &str = "quantum.tsirelson-bound";
/// The local-hidden-variable maximum of |S| is exactly 2.
pub const LOCAL_REALISM_BOUND: &str = "quantum.local-realism-bound";
/// The singlet correlator equals ⟨ψ|σ(a)⊗σ(b)|ψ⟩, derived from the operators.
pub const QM_CORRELATOR: &str = "quantum.correlator-from-operators";

/// Matrix rows for the quantum-foundations lab.
pub fn quantum_rows() -> [&'static str; 5] {
    [
        BORN_NORMALIZATION,
        QM_CORRELATOR,
        BELL_VIOLATION,
        TSIRELSON_BOUND,
        LOCAL_REALISM_BOUND,
    ]
}

/// Angle-grid resolution for the brute-force Tsirelson maximization.
const ANGLE_STEPS: usize = 90;

/// The signed CHSH combination `S` for the singlet correlator
/// `E(x,y) = −V·cos(x−y)` at four measurement angles. The `−cos(x−y)` form is
/// not assumed: it is verified against the operator expectation
/// `⟨ψ⁻|σ(x)⊗σ(y)|ψ⁻⟩` by the `quantum.correlator-from-operators` claim.
fn chsh_value(v: f64, a: f64, a2: f64, b: f64, b2: f64) -> f64 {
    let e = |x: f64, y: f64| -v * (x - y).cos();
    e(a, b) - e(a, b2) + e(a2, b) + e(a2, b2)
}

/// The singlet correlator `E(a,b) = ⟨ψ⁻|σ(a)⊗σ(b)|ψ⁻⟩`, computed directly from
/// the two-qubit state and the spin-measurement operators (no visibility).
fn singlet_correlator(a: f64, b: f64) -> f64 {
    let op = tensor2(spin_measurement(a), spin_measurement(b));
    expectation4(&op, &BellTest::singlet())
        .map(|c| c.re)
        .unwrap_or(f64::NAN)
}

/// Maximum `|S|` any local hidden-variable model can reach, by enumerating all
/// `2⁴` deterministic ±1 outcome assignments. The maximum is exactly 2 — the
/// CHSH bound, *derived* here rather than asserted.
fn max_chsh_local_hidden_variable() -> f64 {
    let mut best = 0.0_f64;
    for bits in 0..16u32 {
        let sign = |n: u32| -> f64 {
            if (bits >> n) & 1 == 0 {
                1.0
            } else {
                -1.0
            }
        };
        let (aa, aa2, bb, bb2) = (sign(0), sign(1), sign(2), sign(3));
        let s = (aa * bb - aa * bb2 + aa2 * bb + aa2 * bb2).abs();
        best = best.max(s);
    }
    best
}

const SPECS: &[KnobSpec] = &[KnobSpec {
    name: "visibility",
    layer: LayerId::Quantum,
    doc: "Werner-state visibility V in [0,1]. The CHSH correlator scales as V·2√2; below 1/√2 a local model suffices.",
    origin: ParameterOrigin::Chosen,
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
        chsh_value(self.visibility, 0.0, PI / 2.0, PI / 4.0, 3.0 * PI / 4.0).abs()
    }

    /// Maximize `|S|` over all measurement angles by brute-force grid search
    /// (the first angle is fixed to 0 by rotational symmetry). This mechanically
    /// checks Tsirelson's bound: for the quantum correlator no angle choice
    /// exceeds `V·2√2`, and at full visibility the maximum saturates `2√2`.
    fn max_chsh_over_angles(&self) -> f64 {
        let v = self.visibility;
        let step = PI / ANGLE_STEPS as f64;
        let mut best = 0.0_f64;
        for i in 0..ANGLE_STEPS {
            let a2 = i as f64 * step;
            for j in 0..ANGLE_STEPS {
                let b = j as f64 * step;
                for k in 0..ANGLE_STEPS {
                    let b2 = k as f64 * step;
                    best = best.max(chsh_value(v, 0.0, a2, b, b2).abs());
                }
            }
        }
        best
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
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                QM_CORRELATOR,
                "The singlet correlator equals ⟨ψ|σ(a)⊗σ(b)|ψ⟩ = −cos(a−b).",
                LayerId::Quantum,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                BELL_VIOLATION,
                "The CHSH correlator exceeds the local-realism bound of 2.",
                LayerId::Quantum,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                TSIRELSON_BOUND,
                "The CHSH correlator does not exceed Tsirelson's bound 2√2.",
                LayerId::Quantum,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                LOCAL_REALISM_BOUND,
                "The local-hidden-variable maximum of |S| is exactly 2.",
                LayerId::Quantum,
                ClaimClass::ModelInternal,
            ),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id_str() {
            BORN_NORMALIZATION => {
                let psi = Self::singlet();
                let n = psi.norm_sqr();
                let p_sum: f64 = (0..psi.dim()).filter_map(|i| psi.born(i)).sum();
                if (n - 1.0).abs() < 1e-12 && (p_sum - 1.0).abs() < 1e-12 {
                    Verdict::holds(claim, "⟨ψ|ψ⟩ = 1 and Σ pᵢ = 1")
                        .with_evidence([format!("norm² = {n:.6}, Σ pᵢ = {p_sum:.6}")])
                } else {
                    Verdict::fails(claim, "state is not normalized")
                }
            }
            QM_CORRELATOR => {
                // Verify the closed form used everywhere else is the genuine
                // quantum expectation, computed from σ(a)⊗σ(b) on the singlet.
                let mut worst = 0.0_f64;
                for (a, b) in [(0.0, 0.4), (0.3, 1.1), (1.0, 2.0), (0.0, PI / 2.0)] {
                    let from_ops = singlet_correlator(a, b);
                    let closed = -(a - b).cos();
                    worst = worst.max((from_ops - closed).abs());
                }
                if worst < 1e-12 {
                    Verdict::holds(
                        claim,
                        "⟨ψ⁻|σ(a)⊗σ(b)|ψ⁻⟩ = −cos(a−b), computed from the operators",
                    )
                    .with_evidence([format!(
                        "max |operator expectation − (−cos Δ)| = {worst:.2e} over sampled angles"
                    )])
                } else {
                    Verdict::fails(
                        claim,
                        format!("operator correlator disagrees with −cos(a−b) by {worst:.2e}"),
                    )
                }
            }
            BELL_VIOLATION => {
                let s = self.chsh_s();
                if s > 2.0 + 1e-12 {
                    Verdict::holds(
                        claim,
                        format!("CHSH S = {s:.3} > 2: local realism is refuted"),
                    )
                    .with_evidence([
                        "local hidden-variable theories obey |S| ≤ 2 (Bell/CHSH)".to_string(),
                    ])
                } else {
                    Verdict::fails(
                        claim,
                        format!(
                            "CHSH S = {s:.3} ≤ 2: reproducible by a local hidden-variable model"
                        ),
                    )
                }
            }
            TSIRELSON_BOUND => {
                // Computed, not asserted: maximize |S| over all measurement
                // angles and confirm no quantum strategy exceeds 2√2.
                let smax = self.max_chsh_over_angles();
                let tsirelson = 2.0 * 2.0_f64.sqrt();
                if smax <= tsirelson + 1e-6 {
                    Verdict::holds(claim,
                        format!(
                            "maximizing over angles gives |S|max = {smax:.4} ≤ 2√2 ≈ {tsirelson:.4}"
                        ),
                    )
                    .with_evidence([format!(
                        "brute-force over a {ANGLE_STEPS}³ angle grid; no setting exceeds 2√2 (Tsirelson)"
                    )])
                } else {
                    Verdict::fails(
                        claim,
                        format!("found |S| = {smax:.4} > 2√2 — impossible in quantum mechanics"),
                    )
                }
            }
            LOCAL_REALISM_BOUND => {
                // Derive the classical bound by enumerating deterministic models.
                let lhv = max_chsh_local_hidden_variable();
                if (lhv - 2.0).abs() < 1e-12 {
                    Verdict::holds(claim,
                        "local hidden-variable |S|max = 2, over all 2⁴ deterministic strategies",
                    )
                    .with_evidence([
                        "enumerated every ±1 outcome assignment; the CHSH bound of 2 is derived, not assumed".to_string(),
                    ])
                } else {
                    Verdict::fails(
                        claim,
                        format!("enumerated local-realism max |S| = {lhv:.3} ≠ 2"),
                    )
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by a quantum-foundations object"),
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
        let c = t.claims().into_iter().find(|c| c.id_str() == id).unwrap();
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
    fn tsirelson_bound_is_computed_by_maximizing_over_angles() {
        // No measurement setting exceeds 2√2, and full visibility saturates it.
        let t = BellTest::default();
        let smax = t.max_chsh_over_angles();
        let tsirelson = 2.0 * 2.0_f64.sqrt();
        assert!(smax <= tsirelson + 1e-6, "found |S| = {smax} > 2√2");
        assert!(
            (smax - tsirelson).abs() < 1e-2,
            "|S|max = {smax}, expected ≈ 2√2"
        );
        assert_eq!(verdict(&t, TSIRELSON_BOUND), VerdictKind::Holds);
    }

    #[test]
    fn classical_bound_of_two_is_derived_by_enumeration() {
        // The CHSH bound of 2 falls out of enumerating deterministic strategies.
        assert!((super::max_chsh_local_hidden_variable() - 2.0).abs() < 1e-12);
        assert_eq!(
            verdict(&BellTest::default(), LOCAL_REALISM_BOUND),
            VerdictKind::Holds
        );
    }

    #[test]
    fn correlator_is_derived_from_the_operators() {
        // The quantum prediction −cos(a−b) emerges from σ(a)⊗σ(b) on the singlet.
        for (a, b) in [(0.0, 0.4), (0.3, 1.1), (1.0, 2.0)] {
            assert!((super::singlet_correlator(a, b) - (-(a - b).cos())).abs() < 1e-12);
        }
        assert_eq!(
            verdict(&BellTest::default(), QM_CORRELATOR),
            VerdictKind::Holds
        );
    }

    #[test]
    fn quantum_beats_the_classical_bound() {
        // The whole point: the quantum maximum strictly exceeds the LHV maximum.
        let t = BellTest::default();
        assert!(t.max_chsh_over_angles() > super::max_chsh_local_hidden_variable() + 0.5);
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
