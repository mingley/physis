//! Grand unification: the Georgi–Glashow SU(5) GUT as a first-class theory.
//!
//! This is the empirical-contact counterpart to the string-critique's
//! "accommodate vs derive" theme, one layer above the Standard Model. Embedding
//! one SM generation into a complete SU(5) multiplet (`5̄ + 10`) *derives* two
//! numbers that the SM merely accommodates:
//!
//! - **charge quantization**: `Q` is a traceless SU(5) generator, so
//!   `Tr Q = 0` over the multiplet (`gut.charge-quantization`). That sum is
//!   `ΣY`, the gravitational anomaly already certified as P3N on the SM;
//!   the GUT cell stays `executed`, not a second certificate, and
//! - **the weak mixing angle**: `sin²θ_W = Tr(T₃²)/Tr(Q²) = 3/8` at the
//!   unification scale (`gut.weinberg-angle`, exact `Ratio` / P3N). Georgi–Quinn–
//!   Weinberg running of that boundary condition down to `M_Z` is a separate
//!   claim (`gut.weinberg-angle-mz`): minimal SU(5) predicts ≈0.21 and **fails**;
//!   the MSSM predicts ≈0.231 and holds as a heuristic. The companion
//!   `gut.weinberg-angle-mz-interval` asks the empirical axis: the same 3%
//!   band as an enclosure against the PDG hull. Overlap is not containment.
//!   `gut.proton-lifetime-sk` is the empirical proton-lifetime cell: no
//!   Super-Kamiokande Dataset is registered, so it stays untested.
//!
//! It is also where the lab is honest about *failure*: minimal (non-SUSY)
//! SU(5) does not unify the gauge couplings and predicts proton decay at a rate
//! already excluded by Super-Kamiokande. Those claims `fail`. A `supersymmetric`
//! knob revives unification as a `heuristic`, at the price of unobserved
//! superpartners.

use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::ParameterOrigin;
use physis_data::{pdg_2024_sin2theta, super_kamiokande_proton_lifetime, EmpiricalReceipt};
use physis_model::{GaugeGroup, Manifold, Spectrum, World};
use physis_numeric::{Interval, Ratio};

use crate::framework::Theory;
use crate::rge::GaugeRunning;
use crate::standard_model::{gut_trace_charge_exact, gut_weinberg_traces_exact};

/// Relative mismatch the heuristic GQW cell treats as a hit. The empirical
/// enclosure uses the same 3% as a theory band, not a two-loop remainder
/// certificate.
const GQW_HEURISTIC_BAND: f64 = 0.03;

/// SM fermions fill complete SU(5) multiplets (`5̄ + 10` per generation).
pub const GUT_SM_EMBEDDING: &str = "gut.sm-embedding";
/// Electric charge is quantized because `Q` is a traceless SU(5) generator.
pub const GUT_CHARGE_QUANTIZATION: &str = "gut.charge-quantization";
/// The GUT-scale weak mixing angle is `sin²θ_W = 3/8`.
pub const GUT_WEINBERG_ANGLE: &str = "gut.weinberg-angle";
/// The GQW-evolved `sin²θ_W(M_Z)` matches the measured electroweak value.
pub const GUT_WEINBERG_ANGLE_MZ: &str = "gut.weinberg-angle-mz";
/// One-loop GQW `sin²θ_W(M_Z)` enclosed by the heuristic 3% band, vs PDG.
pub const GUT_WEINBERG_ANGLE_MZ_INTERVAL: &str = "gut.weinberg-angle-mz-interval";
/// The three SM gauge couplings meet at a single unification scale.
pub const GUT_COUPLING_UNIFICATION: &str = "gut.coupling-unification";
/// The predicted proton lifetime is consistent with experiment.
pub const GUT_PROTON_DECAY_VIABLE: &str = "gut.proton-decay-viable";
/// Predicted proton lifetime compared to Super-Kamiokande as a dataset.
pub const GUT_PROTON_LIFETIME_SK: &str = "gut.proton-lifetime-sk";

const SPECS: &[KnobSpec] = &[KnobSpec {
    name: "supersymmetric",
    layer: LayerId::Field,
    doc: "Whether the GUT is supersymmetric (MSSM matter). SUSY revives gauge-coupling unification and raises the proton-decay scale, at the price of unobserved superpartners.",
    origin: ParameterOrigin::Chosen,
    domain: KnobDomain::Bool,
}];

/// The Georgi–Glashow SU(5) grand unified theory.
#[derive(Clone, Debug, Default)]
pub struct Su5Gut {
    /// Whether the theory is supersymmetric (SUSY SU(5) / MSSM).
    supersymmetric: bool,
}

impl Knobbed for Su5Gut {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "supersymmetric" => Ok(KnobValue::Bool(self.supersymmetric)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("supersymmetric", KnobValue::Bool(v)) => self.supersymmetric = v,
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

impl Theory for Su5Gut {
    fn id(&self) -> &'static str {
        "su5-gut"
    }
    fn name(&self) -> &'static str {
        "SU(5) grand unified theory"
    }
    fn summary(&self) -> &'static str {
        "Georgi–Glashow SU(5): one gauge group containing SU(3)×SU(2)×U(1). It \
         derives charge quantization and sin²θ_W = 3/8 at unification. Running \
         that 3/8 down to M_Z with Georgi–Quinn–Weinberg, minimal SU(5) misses \
         the measured 0.231; a supersymmetric knob revives the heuristic match. \
         The PDG interval comparison is a separate empirical cell."
    }
    fn world(&self) -> Option<World> {
        Some(World {
            spacetime: Manifold::observed_4d(),
            gauge: GaugeGroup::su5(),
            spectrum: Spectrum::standard_model(),
            has_gravity: false,
            supersymmetric: self.supersymmetric,
            // One unified gauge coupling instead of three, plus a GUT scale and
            // symmetry-breaking sector: a heuristic count, fewer than the SM's 19.
            free_parameter_count: if self.supersymmetric { 18 } else { 15 },
            landscape_log10: 0.0,
            note: format!(
                "SU(5) GUT, supersymmetric={} (SM ⊂ SU(5))",
                self.supersymmetric
            ),
        })
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                GUT_SM_EMBEDDING,
                "The Standard Model fermions fill complete SU(5) multiplets (5̄ + 10).",
                LayerId::Interaction,
                ClaimClass::Phenomenological,
            ),
            Claim::new(
                GUT_CHARGE_QUANTIZATION,
                "Electric charge is quantized (Q is a traceless SU(5) generator).",
                LayerId::Particle,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                GUT_WEINBERG_ANGLE,
                "The unification-scale weak mixing angle is sin²θ_W = 3/8.",
                LayerId::Interaction,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                GUT_WEINBERG_ANGLE_MZ,
                "Georgi–Quinn–Weinberg running of 3/8 down to M_Z matches the measured sin²θ_W.",
                LayerId::Effective,
                ClaimClass::Heuristic,
            ),
            Claim::new(
                GUT_WEINBERG_ANGLE_MZ_INTERVAL,
                "The one-loop GQW prediction of sin²θ_W(M_Z), enclosed by the heuristic 3% \
                 truncation band, lies inside the PDG measurement.",
                LayerId::Effective,
                ClaimClass::EmpiricalPrediction,
            ),
            Claim::new(
                GUT_COUPLING_UNIFICATION,
                "The three SM gauge couplings meet at a single scale.",
                LayerId::Interaction,
                ClaimClass::Heuristic,
            ),
            Claim::new(
                GUT_PROTON_DECAY_VIABLE,
                "The predicted proton lifetime is consistent with experiment.",
                LayerId::Effective,
                ClaimClass::Heuristic,
            ),
            Claim::new(
                GUT_PROTON_LIFETIME_SK,
                "The predicted proton lifetime is compared to Super-Kamiokande as a registered dataset.",
                LayerId::Effective,
                ClaimClass::EmpiricalPrediction,
            ),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id.0.as_str() {
            GUT_SM_EMBEDDING => match GaugeGroup::su5().verified_contains_sm() {
                Some(chain) => {
                    Verdict::holds(claim, "SU(5) ⊃ SU(3)×SU(2)×U(1); one generation = 5̄ ⊕ 10")
                        .with_evidence([format!("verified chain: {}", chain.join(" ⊃ "))])
                }
                None => Verdict::fails(claim, "no verified SM embedding"),
            },
            GUT_CHARGE_QUANTIZATION => {
                let tr_q = gut_trace_charge_exact();
                if tr_q.is_zero() {
                    Verdict::holds(
                        claim,
                        "Tr Q = 0 over a complete SU(5) multiplet (ΣY, already the grav anomaly)",
                    )
                    .with_evidence([
                        format!("computed Tr Q over one generation = {tr_q} (= ΣY)"),
                        "Q = T₃ + Y and Σ T₃ = 0, so Tr Q is the gravitational [grav]²U(1) sum already certified as P3N on consistency.anomaly-cancellation; not a second certificate"
                            .to_string(),
                    ])
                } else {
                    Verdict::fails(
                        claim,
                        format!("Tr Q = {tr_q} ≠ 0: charge not quantized by SU(5)"),
                    )
                }
            }
            GUT_WEINBERG_ANGLE => match gut_weinberg_traces_exact() {
                Some((t3, q2)) => {
                    let s2 = t3 / q2;
                    if s2 == Ratio::new(3, 8) {
                        Verdict::holds(
                            claim,
                            "sin²θ_W = 3/8 at the unification scale (exact Tr(T₃²)/Tr(Q²))",
                        )
                        .with_evidence([
                            format!("Σ T₃² = {t3}, Σ Q² = {q2}, sin²θ_W = {s2} at M_GUT"),
                            "the low-energy value is gut.weinberg-angle-mz (GQW running), not this cell"
                                .to_string(),
                        ])
                        .with_certified_numeric()
                    } else {
                        Verdict::fails(claim, format!("computed sin²θ_W = {s2} ≠ 3/8"))
                    }
                }
                None => Verdict::fails(claim, "Tr(Q²) vanished; sin²θ_W is undefined"),
            },
            GUT_WEINBERG_ANGLE_MZ => {
                // Georgi–Quinn–Weinberg: α_em and α_s predict sin²θ_W(M_Z)
                // assuming one-loop unification. This does not use the
                // measured mixing angle, so it is not tautological with 3/8.
                let run = if self.supersymmetric {
                    GaugeRunning::mssm()
                } else {
                    GaugeRunning::standard_model()
                };
                let pred = run.predicted_sin2_mz();
                let meas = run.measured_sin2_mz();
                let mismatch = run.sin2_mismatch();
                let evidence = [format!(
                    "GQW one-loop: predicted sin²θ_W(M_Z) = {pred:.4} vs measured {meas:.4} \
                     (mismatch {:.1}%), M_U ≈ {:.2e} GeV",
                    100.0 * mismatch,
                    run.gqw_unification_scale_gev()
                )];
                if mismatch < GQW_HEURISTIC_BAND {
                    Verdict::holds(
                        claim,
                        "GQW running of unification lands on the measured sin²θ_W(M_Z)",
                    )
                    .with_evidence(evidence)
                } else {
                    Verdict::fails(
                        claim,
                        "GQW running of unification misses the measured sin²θ_W(M_Z)",
                    )
                    .with_evidence(evidence)
                }
            }
            GUT_WEINBERG_ANGLE_MZ_INTERVAL => {
                // Same one-loop centre as the heuristic cell, enclosed by that
                // cell's 3% hit threshold. The band is not a remainder
                // certificate. 3/8 at M_GUT is a different claim.
                let run = if self.supersymmetric {
                    GaugeRunning::mssm()
                } else {
                    GaugeRunning::standard_model()
                };
                let pred = run.predicted_sin2_mz();
                let envelope =
                    Interval::from_f64_approx(pred).relative_envelope(Ratio::new(3, 100));
                let dataset = pdg_2024_sin2theta();
                let rec = EmpiricalReceipt::compare(envelope, &dataset);
                let evidence = [
                    format!(
                        "one-loop GQW centre {pred:.5} ± 3% heuristic band vs {} hull",
                        dataset.id
                    ),
                    format!(
                        "receipt excluded={} compatible={} inconclusive={} (interval-subset; 3% is not a remainder certificate)",
                        rec.excluded, rec.compatible, rec.inconclusive
                    ),
                    "not the GUT-scale 3/8; that is gut.weinberg-angle".to_string(),
                ];
                let v = if rec.excluded {
                    Verdict::fails(
                        claim,
                        "GQW truncation envelope is disjoint from the PDG hull",
                    )
                } else if rec.compatible {
                    Verdict::holds(
                        claim,
                        "GQW truncation envelope is contained in the PDG hull",
                    )
                } else {
                    Verdict::undecidable(
                        claim,
                        "GQW truncation envelope overlaps the PDG hull but is not contained in it",
                    )
                };
                v.with_empirical(rec.status()).with_evidence(evidence)
            }
            GUT_COUPLING_UNIFICATION => {
                // Computed at one loop: run α_1, α_2, α_3 from M_Z, fix the
                // unification point from the electroweak lines, and predict
                // α_3(M_Z). Small mismatch ⇒ the three couplings meet.
                let run = if self.supersymmetric {
                    GaugeRunning::mssm()
                } else {
                    GaugeRunning::standard_model()
                };
                let mismatch = run.unification_mismatch();
                let evidence = [
                    format!(
                        "one-loop: predicted α_3(M_Z) = {:.4} vs measured {:.4} (mismatch {:.1}%), M_GUT ≈ {:.2e} GeV",
                        run.predicted_alpha3_mz(),
                        run.measured_alpha3_mz(),
                        100.0 * mismatch,
                        run.unification_scale_gev()
                    ),
                    format!(
                        "two-loop (RK4): α_3⁻¹ gap {:.1}% at M_GUT ≈ {:.2e} GeV",
                        100.0 * run.two_loop_unification_mismatch(),
                        run.two_loop_unification_scale_gev()
                    ),
                ];
                let two_loop = run.two_loop_unification_mismatch();
                if mismatch < 0.03 {
                    Verdict::holds(claim,
                        format!(
                            "the three couplings meet (1-loop {:.0}%, 2-loop {:.0}% gap) — a celebrated near-success",
                            100.0 * mismatch,
                            100.0 * two_loop
                        ),
                    )
                    .with_evidence(evidence)
                } else {
                    Verdict::fails(claim,
                        format!(
                            "the couplings miss unification (1-loop {:.0}%, 2-loop {:.0}% off in α_3)",
                            100.0 * mismatch,
                            100.0 * two_loop
                        ),
                    )
                    .with_evidence(evidence)
                }
            }
            GUT_PROTON_DECAY_VIABLE => {
                // Tie the verdict to the computed unification scale: the
                // dimension-6 rate scales as M_GUT⁻⁴, so a low M_GUT (minimal
                // SU(5)) means a short, already-excluded lifetime.
                let run = if self.supersymmetric {
                    GaugeRunning::mssm()
                } else {
                    GaugeRunning::standard_model()
                };
                let m_gut = run.unification_scale_gev();
                if self.supersymmetric {
                    Verdict::holds(claim,
                        "SUSY raises M_GUT, pushing p → e⁺π⁰ above current limits (not excluded, not seen)",
                    )
                    .with_evidence([format!("computed one-loop M_GUT ≈ {m_gut:.2e} GeV")])
                } else {
                    Verdict::fails(claim,
                        "minimal SU(5) predicts τ_p ~ 10³¹ yr, excluded by Super-Kamiokande (τ > 2.4×10³⁴ yr)",
                    )
                    .with_evidence([format!(
                        "low computed M_GUT ≈ {m_gut:.2e} GeV drives the too-fast decay (rate ∝ M_GUT⁻⁴)"
                    )])
                }
            }
            GUT_PROTON_LIFETIME_SK => {
                // Super-K is cited as heuristic prose on gut.proton-decay-viable.
                // It is not a physis-data Dataset. Do not mint a lifetime
                // number to close this gap.
                match super_kamiokande_proton_lifetime() {
                    None => Verdict::undecidable(
                        claim,
                        "no Super-Kamiokande Dataset is registered",
                    )
                    .with_evidence([
                        "gut.proton-decay-viable quotes Super-K as M_GUT heuristic prose, not as an artifact"
                            .to_string(),
                    ]),
                    Some(_) => Verdict::undecidable(
                        claim,
                        "a Super-Kamiokande Dataset is registered but no lifetime enclosure is encoded",
                    ),
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by the SU(5) GUT object"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;
    use physis_core::DerivationAssurance;

    fn verdict(t: &dyn Theory, id: &str) -> Verdict {
        let c = t.claims().into_iter().find(|c| c.id.0 == id).unwrap();
        t.evaluate(&c)
    }

    #[test]
    fn weinberg_angle_is_exactly_three_eighths() {
        let (t3, q2) = gut_weinberg_traces_exact().expect("Tr(Q²) is nonzero");
        assert_eq!((t3, q2), (Ratio::int(2), Ratio::new(16, 3)));
        assert_eq!(t3 / q2, Ratio::new(3, 8));
        let g = Su5Gut::default();
        let v = verdict(&g, GUT_WEINBERG_ANGLE);
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_eq!(v.class, ClaimClass::ModelInternal);
        assert_eq!(v.derivation, DerivationAssurance::CertifiedNumeric);
        assert!(
            v.evidence
                .iter()
                .any(|e| e.contains("Σ T₃² = 2") && e.contains("Σ Q² = 16/3")),
            "evidence must quote the exact traces, not a float: {:?}",
            v.evidence
        );
        // The GUT-scale cell must not pretend 3/8 is the M_Z value.
        assert!(
            !v.evidence
                .iter()
                .any(|e| e.contains("0.231") && e.contains("3/8")),
            "do not mix the M_Z measurement into the 3/8 hold evidence: {:?}",
            v.evidence
        );
    }

    #[test]
    fn gqw_running_fails_minimal_su5_and_holds_for_susy() {
        let mut g = Su5Gut::default();
        let v = verdict(&g, GUT_WEINBERG_ANGLE_MZ);
        assert_eq!(v.kind, VerdictKind::Fails);
        assert_eq!(v.class, ClaimClass::Heuristic);
        assert!(
            v.evidence
                .iter()
                .any(|e| e.contains("predicted sin²θ_W(M_Z)")),
            "evidence: {:?}",
            v.evidence
        );
        // The fail evidence quotes the low-energy prediction, not 3/8 as if it
        // were the M_Z value.
        assert!(
            v.evidence
                .iter()
                .any(|e| e.contains("0.20") || e.contains("0.21")),
            "SM GQW should quote ~0.21, got {:?}",
            v.evidence
        );
        g.set("supersymmetric", KnobValue::Bool(true)).unwrap();
        let u = verdict(&g, GUT_WEINBERG_ANGLE_MZ);
        assert_eq!(u.kind, VerdictKind::Holds);
        assert_eq!(u.class, ClaimClass::Heuristic);
        // A hold must not claim 3/8 at M_Z.
        assert!(
            !u.evidence.iter().any(|e| e.contains("3/8")),
            "MSSM hold must not quote 3/8 as the M_Z value: {:?}",
            u.evidence
        );
    }

    #[test]
    fn gqw_interval_excludes_minimal_su5_and_is_too_coarse_for_mssm() {
        use physis_core::EmpiricalStatus;
        let mut g = Su5Gut::default();
        let v = verdict(&g, GUT_WEINBERG_ANGLE_MZ_INTERVAL);
        assert_eq!(v.kind, VerdictKind::Fails);
        assert_eq!(v.class, ClaimClass::EmpiricalPrediction);
        assert_eq!(v.empirical, EmpiricalStatus::Excluded);
        assert!(
            v.evidence.iter().any(|e| e.contains("pdg-2024-sin2theta")),
            "evidence: {:?}",
            v.evidence
        );
        assert!(
            !v.evidence
                .iter()
                .any(|e| e.contains("3/8") && e.contains("0.231")),
            "do not mix GUT-scale 3/8 into the M_Z interval evidence: {:?}",
            v.evidence
        );
        g.set("supersymmetric", KnobValue::Bool(true)).unwrap();
        let u = verdict(&g, GUT_WEINBERG_ANGLE_MZ_INTERVAL);
        assert_eq!(u.kind, VerdictKind::Undecidable);
        assert_eq!(u.empirical, EmpiricalStatus::Inconclusive);
        // Heuristic folklore can still hold while the interval receipt cannot.
        assert_eq!(verdict(&g, GUT_WEINBERG_ANGLE_MZ).kind, VerdictKind::Holds);
    }

    #[test]
    fn proton_lifetime_sk_stays_untested_without_a_dataset() {
        use physis_core::EmpiricalStatus;
        let mut g = Su5Gut::default();
        let v = verdict(&g, GUT_PROTON_LIFETIME_SK);
        assert_eq!(v.kind, VerdictKind::Undecidable);
        assert_eq!(v.class, ClaimClass::EmpiricalPrediction);
        assert_eq!(v.empirical, EmpiricalStatus::Untested);
        g.set("supersymmetric", KnobValue::Bool(true)).unwrap();
        let u = verdict(&g, GUT_PROTON_LIFETIME_SK);
        assert_eq!(u.kind, VerdictKind::Undecidable);
        assert_eq!(u.empirical, EmpiricalStatus::Untested);
        assert_eq!(
            verdict(&g, GUT_PROTON_DECAY_VIABLE).kind,
            VerdictKind::Holds
        );
    }

    #[test]
    fn charge_is_quantized_by_traceless_generator() {
        let g = Su5Gut::default();
        let v = verdict(&g, GUT_CHARGE_QUANTIZATION);
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_eq!(v.class, ClaimClass::ModelInternal);
        assert_eq!(v.derivation, DerivationAssurance::Executed);
        assert_ne!(v.derivation, DerivationAssurance::CertifiedNumeric);
        assert!(gut_trace_charge_exact().is_zero());
        assert!(
            v.evidence
                .iter()
                .any(|e| e.contains("consistency.anomaly-cancellation")),
            "Tr Q must admit it is the grav anomaly, not a second P3N: {:?}",
            v.evidence
        );
    }

    #[test]
    fn minimal_su5_is_falsified_but_susy_revives_it() {
        // Minimal SU(5): couplings miss and proton decay is excluded.
        let mut g = Su5Gut::default();
        assert_eq!(
            verdict(&g, GUT_COUPLING_UNIFICATION).kind,
            VerdictKind::Fails
        );
        assert_eq!(
            verdict(&g, GUT_PROTON_DECAY_VIABLE).kind,
            VerdictKind::Fails
        );
        assert_eq!(verdict(&g, GUT_WEINBERG_ANGLE_MZ).kind, VerdictKind::Fails);

        // The knob → verdict diff: SUSY flips both to holds (as heuristics).
        g.set("supersymmetric", KnobValue::Bool(true)).unwrap();
        let u = verdict(&g, GUT_COUPLING_UNIFICATION);
        assert_eq!(u.kind, VerdictKind::Holds);
        assert_eq!(u.class, ClaimClass::Heuristic);
        assert_eq!(
            verdict(&g, GUT_PROTON_DECAY_VIABLE).kind,
            VerdictKind::Holds
        );
        assert_eq!(verdict(&g, GUT_WEINBERG_ANGLE_MZ).kind, VerdictKind::Holds);
    }

    #[test]
    fn sm_embeds_in_su5() {
        let g = Su5Gut::default();
        assert_eq!(verdict(&g, GUT_SM_EMBEDDING).kind, VerdictKind::Holds);
    }

    #[test]
    fn coupling_unification_verdict_carries_computed_numbers() {
        // The verdict is backed by an actual one-loop RGE prediction, not a
        // stored sentence: the evidence must quote a predicted α_3 and M_GUT.
        let g = Su5Gut::default();
        let v = verdict(&g, GUT_COUPLING_UNIFICATION);
        assert_eq!(v.kind, VerdictKind::Fails);
        assert!(
            v.evidence.iter().any(|e| e.contains("predicted α_3")),
            "evidence: {:?}",
            v.evidence
        );
        assert!(v.evidence.iter().any(|e| e.contains("M_GUT")));
    }
}
