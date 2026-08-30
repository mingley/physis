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
//! fail on the mutant. A PR-box correlator is a second mutation
//! (`add-pr-box`): the CHSH combination of `E = (−1)^{xy}` is 4, which
//! exceeds Tsirelson's `2√2`, so `quantum.tsirelson-bound` fails.

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
/// Popescu–Rohrlich no-signalling correlator on the PR-box fork.
const PRBOX_EQ: &str = "correlator pr-box";

fn parse_bell_state(pkg: &TheoryPackage) -> Result<(bool, bool), String> {
    let mut singlet = false;
    let mut product = false;
    let mut prbox = false;
    for eq in &pkg.equations {
        match eq.trim() {
            SINGLET_EQ => singlet = true,
            PRODUCT_EQ => product = true,
            PRBOX_EQ => prbox = true,
            _ => {}
        }
    }
    if !singlet {
        return Err(format!("{} package has no singlet ket", pkg.id));
    }
    Ok((product, prbox))
}

fn singlet_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["two-qubit singlet".into()],
        vec!["CHSH on |ψ⁻⟩".into()],
        "Bell violation here is the singlet encoding. A product ket is a new \
         encoding, not a silent Werner mixture.",
    )
}

fn tsirelson_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["Hilbert-space CHSH (Tsirelson 2√2)".into()],
        vec!["quantum correlator E = -cos(a-b); |S|max = 2√2".into()],
        "Tsirelson is the Hilbert-space encoding. A PR-box correlator is a new \
         encoding, not a silent visibility knob.",
    )
}

/// PR-box correlator `E(x,y) = (−1)^{xy}` on bits. Not a ket.
fn pr_box_e(x: i32, y: i32) -> f64 {
    if x * y == 1 {
        -1.0
    } else {
        1.0
    }
}

/// CHSH combination `E00 + E01 + E10 − E11` of the PR-box table. Equals 4.
fn pr_box_chsh() -> f64 {
    (pr_box_e(0, 0) + pr_box_e(0, 1) + pr_box_e(1, 0) - pr_box_e(1, 1)).abs()
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
/// violation fail on the mutant. A PR-box correlator is a second mutation
/// (`add-pr-box`): `E = (−1)^{xy}` gives CHSH `S = 4`, so Tsirelson's bound
/// fails. `visibility` stays a Werner mixedness knob.
#[derive(Clone, Debug, PartialEq)]
pub struct BellTest {
    visibility: f64,
    product: bool,
    /// PR-box correlator. Not a knob.
    prbox: bool,
}

impl Default for BellTest {
    fn default() -> Self {
        Self {
            visibility: 1.0,
            product: false,
            prbox: false,
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
    /// forked, `state product` and/or `correlator pr-box`. Visibility stays
    /// on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![SINGLET_EQ.to_string()];
        if self.product {
            equations.push(PRODUCT_EQ.to_string());
        }
        if self.prbox {
            equations.push(PRBOX_EQ.to_string());
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
        let (product, prbox) = parse_bell_state(pkg)?;
        Ok(Self {
            product,
            prbox,
            ..Self::default()
        })
    }

    fn product_equation() -> String {
        PRODUCT_EQ.to_string()
    }

    fn prbox_equation() -> String {
        PRBOX_EQ.to_string()
    }

    /// The CHSH correlator |S|. PR-box uses the bit table; product uses
    /// operator expectations on |01⟩; otherwise the singlet at optimal angles.
    fn chsh_s(&self) -> f64 {
        if self.prbox {
            pr_box_chsh()
        } else if self.product {
            product_chsh().abs()
        } else {
            chsh_value(self.visibility, 0.0, PI / 2.0, PI / 4.0, 3.0 * PI / 4.0).abs()
        }
    }

    /// Maximize `|S|` over all measurement angles by brute-force grid search
    /// (the first angle is fixed to 0 by rotational symmetry). This mechanically
    /// checks Tsirelson's bound: for the quantum correlator no angle choice
    /// exceeds `V·2√2`, and at full visibility the maximum saturates `2√2`.
    /// A PR-box is not an angle-parameterized quantum correlator; its CHSH
    /// value is the bit-table combination, identically 4.
    fn max_chsh_over_angles(&self) -> f64 {
        if self.prbox {
            return pr_box_chsh();
        }
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
         A PR-box correlator is a second IR mutation: S = 4 exceeds Tsirelson. \
         A visibility knob turns the singlet violation off."
    }
    fn world(&self) -> Option<World> {
        None // quantum foundations live on the quantum/information layers
    }
    fn note(&self) -> String {
        format!(
            "CHSH {}, visibility = {}, S = {:.3}",
            match (self.product, self.prbox) {
                (true, true) => "product+PR-box",
                (true, false) => "product",
                (false, true) => "PR-box",
                (false, false) => "singlet",
            },
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
            )
            .with_domain(tsirelson_domain()),
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
                if self.prbox {
                    let residual = (pr_box_e(0, 0) - singlet_correlator(0.0, 0.0)).abs();
                    Verdict::fails(
                        claim,
                        "PR-box E = (−1)^{xy} is not ⟨ψ⁻|σ(a)⊗σ(b)|ψ⁻⟩ = −cos(a−b)",
                    )
                    .with_evidence([format!(
                        "max |E_PR − ⟨σ⊗σ⟩| = {residual:.3} at (0,0); CHSH S_PR = {:.3}",
                        pr_box_chsh()
                    )])
                } else if self.product {
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
        fork.prbox = parsed.prbox;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mut out: Vec<(String, Box<dyn Theory>)> = Vec::new();
        if !self.product {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::product_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.product {
                    let mut fork = self.clone();
                    fork.product = true;
                    out.push(("add-product".into(), Box::new(fork)));
                }
            }
        }
        if !self.prbox {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::prbox_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.prbox {
                    let mut fork = self.clone();
                    fork.prbox = true;
                    out.push(("add-pr-box".into(), Box::new(fork)));
                }
            }
        }
        out
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
         not a visibility knob. A PR-box correlator is a second IR fork \
         (`add-pr-box`): S = 4 exceeds Tsirelson.",
        vec![
            "`holds` / `fails` are internal to the encoding.".into(),
            "S is computed at the optimal CHSH angles; the classical bound is 2, the quantum (Tsirelson) bound is 2√2.".into(),
            "`set bell-test visibility 0.5` drops S below 2 — a local model then suffices.".into(),
            "`hypothesize bell-test`: add-product and add-pr-box are IR, not set.".into(),
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
        assert_eq!(probes.len(), 2);
        assert!(
            probes.iter().any(|(label, _)| label == "add-product"),
            "live Bell must offer add-product: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        assert!(
            probes.iter().any(|(label, _)| label == "add-pr-box"),
            "live Bell must offer add-pr-box: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let product_probe = probes
            .iter()
            .find(|(label, _)| label == "add-product")
            .unwrap();
        assert_eq!(
            verdict(product_probe.1.as_ref(), BELL_VIOLATION),
            VerdictKind::Fails
        );
        let product_fork_probes = fork.structural_mutations();
        assert!(
            product_fork_probes
                .iter()
                .all(|(label, _)| label != "add-product"),
            "product fork must not re-offer add-product"
        );
        assert!(
            product_fork_probes
                .iter()
                .any(|(label, _)| label == "add-pr-box"),
            "product fork must still offer add-pr-box"
        );
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

    #[test]
    fn pr_box_correlator_is_ir_not_a_knob() {
        let mut t = BellTest::default();
        assert!(
            BellTest::default()
                .set("prbox", KnobValue::Bool(true))
                .is_err(),
            "PR-box correlator is an IR mutation, not a knob"
        );
        assert!(
            BellTest::default()
                .set("pr-box", KnobValue::Bool(true))
                .is_err(),
            "PR-box is not a knob"
        );
        let src = render_package(&t.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            pkg.equations.len(),
            1,
            "live package must stay the singlet ket"
        );
        assert_eq!(pkg.equations[0], SINGLET_EQ);
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(BellTest::prbox_equation()),
        );
        let parsed = BellTest::from_package(&mutated).unwrap();
        assert!(parsed.prbox);
        let mut fork = t.clone();
        fork.prbox = true;
        assert_eq!(verdict(&fork, TSIRELSON_BOUND), VerdictKind::Fails);
        assert_eq!(verdict(&t, TSIRELSON_BOUND), VerdictKind::Holds);
        assert_eq!(verdict(&fork, BELL_VIOLATION), VerdictKind::Holds);
        assert_eq!(verdict(&fork, QM_CORRELATOR), VerdictKind::Fails);
        assert_eq!(verdict(&fork, BORN_NORMALIZATION), VerdictKind::Holds);
        assert_eq!(verdict(&fork, LOCAL_REALISM_BOUND), VerdictKind::Holds);
        let s = pr_box_chsh();
        assert!(
            (s - 4.0).abs() < 1e-12,
            "PR-box CHSH must be the bit-table combination 4, got {s}"
        );
        let tsirelson = 2.0 * 2.0_f64.sqrt();
        assert!(
            s > tsirelson + 0.5,
            "PR-box S must exceed 2√2, got {s} vs {tsirelson}"
        );
        let residual = (pr_box_e(0, 0) - singlet_correlator(0.0, 0.0)).abs();
        assert!(
            (residual - 2.0).abs() < 1e-12,
            "PR E(0,0)=1 vs singlet ⟨σ⊗σ⟩=−1 must differ by 2, got {residual}"
        );
        t.set("visibility", KnobValue::Float(0.5)).unwrap();
        assert_eq!(verdict(&t, BELL_VIOLATION), VerdictKind::Fails);
        assert_eq!(verdict(&t, TSIRELSON_BOUND), VerdictKind::Holds);
        let mut noisy = fork.clone();
        noisy.set("visibility", KnobValue::Float(0.5)).unwrap();
        assert_eq!(
            verdict(&noisy, TSIRELSON_BOUND),
            VerdictKind::Fails,
            "visibility must not convert a PR-box into Hilbert-space CHSH"
        );
        assert_eq!(verdict(&noisy, BELL_VIOLATION), VerdictKind::Holds);
        let probes = BellTest::default().structural_mutations();
        let p = probes
            .iter()
            .find(|(label, _)| label == "add-pr-box")
            .expect("add-pr-box");
        assert_eq!(verdict(p.1.as_ref(), TSIRELSON_BOUND), VerdictKind::Fails);
        let prbox_probes = fork.structural_mutations();
        assert!(
            prbox_probes.iter().all(|(l, _)| l != "add-pr-box"),
            "PR-box fork must not re-offer add-pr-box"
        );
        assert!(
            prbox_probes.iter().any(|(l, _)| l == "add-product"),
            "PR-box fork must still offer add-product"
        );
        let live = BellTest::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), live.package());
        assert_eq!(
            rebuilt.get("visibility").unwrap(),
            KnobValue::Float(1.0),
            "reparse must overlay correlator IR onto live knobs"
        );
        assert_eq!(
            verdict(rebuilt.as_ref(), TSIRELSON_BOUND),
            VerdictKind::Holds
        );
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == TSIRELSON_BOUND)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "Bell Tsirelson must name Hilbert-space CHSH: {:?}",
            cell.domain()
        );
        assert!(
            crate::em::MaxwellVacuum::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-pr-box"),
            "maxwell-vacuum must not grow add-pr-box"
        );
        assert!(
            BellTest::default()
                .set("visibility", KnobValue::Float(0.7))
                .is_ok(),
            "bell-test keeps the visibility knob"
        );
    }
}
