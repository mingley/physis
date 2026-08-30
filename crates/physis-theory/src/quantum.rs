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
//! `1/√2` the correlations are reproducible by a local model. The ket lives on
//! the IR package. A product-state encoding is a package mutation
//! (`add-product`), not a knob: the singlet correlator and the Bell violation
//! fail on the mutant.

use std::f64::consts::PI;

use physis_core::assumption::DomainOfValidity;
use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::ParameterOrigin;
use physis_ir::{apply_mutation, parse_package, render_package, PackageMutation, TheoryPackage};
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
/// Two-qubit singlet ket on the IR package.
const SINGLET_EQ: &str = "state singlet";
/// Product-state encoding (computational |01⟩).
const PRODUCT_EQ: &str = "state product";

fn parse_bell_state(pkg: &TheoryPackage) -> Result<bool, String> {
    let mut singlet = false;
    let mut product = false;
    for eq in &pkg.equations {
        match eq.trim() {
            SINGLET_EQ => singlet = true,
            PRODUCT_EQ => product = true,
            _ => {}
        }
    }
    if !singlet {
        return Err(format!("{} package has no singlet ket", pkg.id));
    }
    Ok(product)
}

fn singlet_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["two-qubit singlet".into()],
        vec!["CHSH on |ψ⁻⟩".into()],
        "Bell violation here is the singlet encoding. A product ket is a new \
         encoding, not a silent Werner mixture.",
    )
}

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

/// A CHSH Bell test. The ket lives on the IR package.
///
/// Default encoding is a two-qubit singlet. A product state is a package
/// mutation (`add-product`), not a knob: the singlet correlator and Bell
/// violation fail on the mutant. `visibility` stays a Werner mixedness knob.
#[derive(Clone, Debug, PartialEq)]
pub struct BellTest {
    visibility: f64,
    product: bool,
}

impl Default for BellTest {
    fn default() -> Self {
        Self {
            visibility: 1.0,
            product: false,
        }
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

    /// Computational-basis product |01⟩.
    fn product_ket() -> Ket {
        Ket {
            amps: vec![Complex::ZERO, Complex::ONE, Complex::ZERO, Complex::ZERO],
        }
    }

    fn ket(&self) -> Ket {
        if self.product {
            Self::product_ket()
        } else {
            Self::singlet()
        }
    }

    /// IR package for this ket. Equations are `state singlet` and, when
    /// forked, `state product`. Visibility stays on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![SINGLET_EQ.to_string()];
        if self.product {
            equations.push(PRODUCT_EQ.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["two-qubit-singlet".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: BELL_VIOLATION.into(),
                statement: "The CHSH correlator exceeds the local-realism bound of 2.".into(),
                layer: "quantum".into(),
                class: "model-internal".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load a ket encoding from a package. Visibility defaults; overlay it
    /// from a live test when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        if pkg.id != "bell-test" {
            return Err(format!(
                "bell-test package id '{}' is not bell-test",
                pkg.id
            ));
        }
        Ok(Self {
            product: parse_bell_state(pkg)?,
            ..Self::default()
        })
    }

    fn product_equation() -> String {
        PRODUCT_EQ.to_string()
    }

    /// The CHSH correlator |S| at the optimal singlet angles.
    fn chsh_s(&self) -> f64 {
        if self.product {
            product_chsh().abs()
        } else {
            chsh_value(self.visibility, 0.0, PI / 2.0, PI / 4.0, 3.0 * PI / 4.0).abs()
        }
    }

    /// Maximize `|S|` over all measurement angles by brute-force grid search
    /// (the first angle is fixed to 0 by rotational symmetry). This mechanically
    /// checks Tsirelson's bound: for the quantum correlator no angle choice
    /// exceeds `V·2√2`, and at full visibility the maximum saturates `2√2`.
    fn max_chsh_over_angles(&self) -> f64 {
        if self.product {
            return self.chsh_s();
        }
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

fn product_correlator(a: f64, b: f64) -> f64 {
    let op = tensor2(spin_measurement(a), spin_measurement(b));
    expectation4(&op, &BellTest::product_ket())
        .map(|c| c.re)
        .unwrap_or(f64::NAN)
}

fn product_chsh() -> f64 {
    let e = product_correlator;
    e(0.0, PI / 4.0) - e(0.0, 3.0 * PI / 4.0) + e(PI / 2.0, PI / 4.0) + e(PI / 2.0, 3.0 * PI / 4.0)
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
        "A CHSH test on a two-qubit ket. Local hidden-variable theories obey \
         |S| ≤ 2; the singlet encoding computes |S| = 2√2, mechanically \
         refuting local realism. A product ket is an IR mutation, not a knob. \
         A visibility knob turns the singlet violation off."
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
            )
            .with_domain(singlet_domain()),
            Claim::new(
                BELL_VIOLATION,
                "The CHSH correlator exceeds the local-realism bound of 2.",
                LayerId::Quantum,
                ClaimClass::ModelInternal,
            )
            .with_domain(singlet_domain()),
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
                let psi = self.ket();
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
                if self.product {
                    Verdict::fails(
                        claim,
                        "product state: ⟨ψ|σ(a)⊗σ(b)|ψ⟩ is not the singlet −cos(a−b)",
                    )
                } else {
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
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.product = parsed.product;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        if self.product {
            return Vec::new();
        }
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(Self::product_equation()),
        );
        match Self::from_package(&mutated) {
            Ok(parsed) if parsed.product => {
                let mut fork = self.clone();
                fork.product = true;
                vec![("add-product".into(), Box::new(fork))]
            }
            _ => Vec::new(),
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
         the two-qubit ket. Local realism is a falsifiable assumption here, and \
         it fails on the singlet. A product ket is an IR fork (`add-product`), \
         not a visibility knob.",
        vec![
            "`holds` / `fails` are internal to the encoding.".into(),
            "S is computed at the optimal CHSH angles; the classical bound is 2, the quantum (Tsirelson) bound is 2√2.".into(),
            "`set bell-test visibility 0.5` drops S below 2 — a local model then suffices.".into(),
            "`hypothesize bell-test`: add-product is IR, not set.".into(),
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

    #[test]
    fn product_state_is_ir_not_a_knob() {
        let mut t = BellTest::default();
        assert!(
            t.set("product", KnobValue::Bool(true)).is_err(),
            "product ket is an IR mutation, not a knob"
        );
        let src = render_package(&t.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            BellTest::from_package(&pkg).unwrap(),
            t,
            "IR round-trip must preserve the singlet ket"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(BellTest::product_equation()),
        );
        let parsed = BellTest::from_package(&mutated).unwrap();
        assert!(parsed.product);
        let mut fork = t.clone();
        fork.product = true;
        assert_eq!(verdict(&fork, BELL_VIOLATION), VerdictKind::Fails);
        assert_eq!(verdict(&fork, QM_CORRELATOR), VerdictKind::Fails);
        assert_eq!(verdict(&fork, BORN_NORMALIZATION), VerdictKind::Holds);
        assert_eq!(verdict(&fork, TSIRELSON_BOUND), VerdictKind::Holds);
        assert_eq!(verdict(&fork, LOCAL_REALISM_BOUND), VerdictKind::Holds);
        assert_eq!(verdict(&t, BELL_VIOLATION), VerdictKind::Holds);
        assert_eq!(verdict(&t, QM_CORRELATOR), VerdictKind::Holds);
        t.set("visibility", KnobValue::Float(0.5)).unwrap();
        assert_eq!(verdict(&t, BELL_VIOLATION), VerdictKind::Fails);
        assert_eq!(verdict(&t, QM_CORRELATOR), VerdictKind::Holds);
        let probes = BellTest::default().structural_mutations();
        assert_eq!(probes.len(), 1);
        assert_eq!(probes[0].0, "add-product");
        assert_eq!(
            verdict(probes[0].1.as_ref(), BELL_VIOLATION),
            VerdictKind::Fails
        );
        assert!(fork.structural_mutations().is_empty());
        let live = BellTest::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), live.package());
        assert_eq!(
            verdict(rebuilt.as_ref(), BELL_VIOLATION),
            VerdictKind::Holds
        );
        for (a, b) in [(0.0, 0.4), (1.0, 2.0)] {
            let ops = super::product_correlator(a, b);
            let closed = -a.cos() * b.cos();
            assert!((ops - closed).abs() < 1e-12, "a={a} b={b} ops={ops}");
        }
        assert!((fork.chsh_s() - 2.0_f64.sqrt()).abs() < 1e-9);
        let bell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == BELL_VIOLATION)
            .unwrap();
        assert!(
            !bell.domain().is_encoding_wide(),
            "Bell violation must name the singlet: {:?}",
            bell.domain()
        );
    }
}
