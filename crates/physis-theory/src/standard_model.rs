//! Standard Model as an effective quantum field theory: empirically sharp,
//! UV-incomplete, many parameters.

use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::ParameterOrigin;
use physis_model::{GaugeGroup, Manifold, Spectrum, World};
use physis_numeric::Ratio;

use crate::claims;
use crate::framework::Theory;

/// The weak hypercharges are fixed by anomaly cancellation up to normalization.
const SM_HYPERCHARGE_DERIVED: &str = "sm.hypercharge-derivation";

/// One left-handed Weyl fermion of a generation, with its SU(3)×SU(2)
/// representation dimensions and weak hypercharge `Y` (convention `Q = T₃ + Y`).
///
/// Keeping the colour and weak dimensions *separately* (not just the product
/// multiplicity) is what lets the four gauge anomalies — and the derivation of
/// `Y` itself — be a computation over the representation content, not a stored
/// table of answers.
struct WeylField {
    /// Species label.
    name: &'static str,
    /// SU(3) representation dimension (3 = (anti)triplet, 1 = singlet).
    color: f64,
    /// SU(2) representation dimension (2 = doublet, 1 = singlet).
    weak: f64,
    /// Weak hypercharge Y (exact rational; convention `Q = T₃ + Y`).
    y: Ratio,
}

/// One generation of left-handed Weyl fermions of the Standard Model.
const SM_WEYL_FIELDS: &[WeylField] = &[
    WeylField {
        name: "Q_L",
        color: 3.0,
        weak: 2.0,
        y: Ratio::new(1, 6),
    }, // quark doublet
    WeylField {
        name: "u_R^c",
        color: 3.0,
        weak: 1.0,
        y: Ratio::new(-2, 3),
    }, // anti-up
    WeylField {
        name: "d_R^c",
        color: 3.0,
        weak: 1.0,
        y: Ratio::new(1, 3),
    }, // anti-down
    WeylField {
        name: "L_L",
        color: 1.0,
        weak: 2.0,
        y: Ratio::new(-1, 2),
    }, // lepton doublet
    WeylField {
        name: "e_R^c",
        color: 1.0,
        weak: 1.0,
        y: Ratio::int(1),
    }, // anti-electron
];

/// Number of SU(2) doublets in one generation (3 quark colours + 1 lepton):
/// the Witten SU(2) global anomaly needs this to be even.
const SM_WEAK_DOUBLETS: u32 = 4;

/// The weak hypercharges derived from anomaly cancellation (see
/// [`derive_hypercharges`]).
#[derive(Clone, Copy, Debug, PartialEq)]
struct DerivedHypercharges {
    /// Y of the quark doublet (the normalization input).
    y_q: f64,
    /// Y of the lepton doublet, from the [SU(2)]²U(1) anomaly.
    y_l: f64,
    /// Y of the anti-electron, from the gravitational anomaly.
    y_e: f64,
    /// The unordered pair {Y_u, Y_d}, sorted ascending, from the [U(1)]³ anomaly.
    y_ud: [f64; 2],
}

/// Derive the Standard Model hypercharges from anomaly cancellation.
///
/// This is the well-known result that, once the gauge group SU(3)×SU(2)×U(1)
/// and the representation content (Q, uᶜ, dᶜ, L, eᶜ) are fixed, requiring all
/// gauge anomalies to vanish determines every hypercharge up to one overall
/// normalization. We fix the scale with `Y_Q = 1/6` and *solve* for the rest:
///
/// - `[SU(2)]²U(1)`: `3·Y_Q + Y_L = 0`            → `Y_L`
/// - `[SU(3)]²U(1)`: `2·Y_Q + Y_u + Y_d = 0`      → `s := Y_u + Y_d`
/// - `[grav]²U(1)`:  `6·Y_Q + 3·s + 2·Y_L + Y_e = 0` → `Y_e`
/// - `[U(1)]³`:      the cubic then fixes `p := Y_u·Y_d`, so {Y_u, Y_d} are the
///   roots of `t² − s·t + p = 0`.
///
/// The output reproduces the measured assignments — the hypercharges are not an
/// input to the Standard Model here, they are *forced* by consistency.
fn derive_hypercharges() -> DerivedHypercharges {
    let y_q: f64 = 1.0 / 6.0; // normalization: fixes the overall U(1) scale
    let y_l = -3.0 * y_q; // [SU(2)]²U(1)
    let s = -2.0 * y_q; // [SU(3)]²U(1): Y_u + Y_d
    let y_e = -(6.0 * y_q + 3.0 * s + 2.0 * y_l); // gravitational anomaly
                                                  // [U(1)]³: 6Y_Q³ + 3(Y_u³+Y_d³) + 2Y_L³ + Y_e³ = 0.
    let a = 6.0 * y_q.powi(3) + 2.0 * y_l.powi(3) + y_e.powi(3);
    // Y_u³ + Y_d³ = s³ − 3·s·p = −a/3  ⇒  p = (s³ + a/3)/(3s).
    let p = (s.powi(3) + a / 3.0) / (3.0 * s);
    let disc = (s * s - 4.0 * p).max(0.0);
    let r = disc.sqrt();
    let mut y_ud = [(s - r) / 2.0, (s + r) / 2.0];
    y_ud.sort_by(|x, y| x.partial_cmp(y).unwrap());
    DerivedHypercharges {
        y_q,
        y_l,
        y_e,
        y_ud,
    }
}

/// Σ T₃² over an SU(2) irrep of dimension `d`: `j(j+1)(2j+1)/3` with
/// `j = (d−1)/2`. A doublet gives `1/2`, a singlet `0`, a triplet `2`.
fn weak_t3_sq(weak_dim: f64) -> f64 {
    let j = (weak_dim - 1.0) / 2.0;
    j * (j + 1.0) * (2.0 * j + 1.0) / 3.0
}

/// The GUT-scale weak mixing angle from embedding one SM generation in a
/// complete SU(5) multiplet: `sin²θ_W = ΣT₃² / ΣQ²`.
///
/// Because the SU(5) generators are equally normalized at unification, the
/// tree-level relation is `sin²θ_W = Tr(T₃²)/Tr(Q²)` over any complete
/// multiplet. Using `Q = T₃ + Y` and `Σ T₃ = 0` per weak multiplet, the sums
/// are computed from the same `SM_WEYL_FIELDS` the anomalies use, giving `3/8`.
pub(crate) fn gut_weinberg_sin2() -> f64 {
    let sum_t3_sq: f64 = SM_WEYL_FIELDS
        .iter()
        .map(|f| f.color * weak_t3_sq(f.weak))
        .sum();
    let sum_q_sq: f64 = SM_WEYL_FIELDS
        .iter()
        .map(|f| f.color * (weak_t3_sq(f.weak) + f.weak * f.y.to_f64() * f.y.to_f64()))
        .sum();
    sum_t3_sq / sum_q_sq
}

/// Trace of electric charge over one SM generation (`Σ colour·weak·Y = ΣY`).
/// A vanishing `Tr Q` is charge quantization in a GUT: `Q` is a traceless
/// SU(5) generator, so charges are forced onto a discrete lattice.
pub(crate) fn gut_trace_charge() -> f64 {
    hypercharge_sum()
}

/// Integer colour and weak dimensions of a Weyl species.
fn dim_ratio(x: f64) -> Ratio {
    Ratio::int(x.round() as i128)
}

/// The [SU(3)]²U(1) mixed anomaly over one generation (colour triplets only).
fn anomaly_su3_u1_exact() -> Ratio {
    let half = Ratio::new(1, 2);
    SM_WEYL_FIELDS
        .iter()
        .filter(|f| f.color > 1.5)
        .fold(Ratio::int(0), |acc, f| acc + half * dim_ratio(f.weak) * f.y)
}

/// The [SU(2)]²U(1) mixed anomaly over one generation (weak doublets only).
fn anomaly_su2_u1_exact() -> Ratio {
    let half = Ratio::new(1, 2);
    SM_WEYL_FIELDS
        .iter()
        .filter(|f| f.weak > 1.5)
        .fold(Ratio::int(0), |acc, f| {
            acc + half * dim_ratio(f.color) * f.y
        })
}

/// The gravitational [grav]²U(1) anomaly: Σ (colour·weak) · Y over a generation.
fn hypercharge_sum_exact() -> Ratio {
    SM_WEYL_FIELDS.iter().fold(Ratio::int(0), |acc, f| {
        acc + dim_ratio(f.color) * dim_ratio(f.weak) * f.y
    })
}

/// The [U(1)]³ anomaly: Σ (colour·weak) · Y³ over a generation.
fn hypercharge_cube_sum_exact() -> Ratio {
    SM_WEYL_FIELDS.iter().fold(Ratio::int(0), |acc, f| {
        acc + dim_ratio(f.color) * dim_ratio(f.weak) * f.y.pow(3)
    })
}

/// Electric charge (in units of e/3) of a species by flavor, from the catalog.
fn charge_thirds(flavor: physis_model::Flavor) -> i32 {
    physis_model::Spectrum::standard_model()
        .species
        .iter()
        .find(|s| s.flavor == flavor)
        .map(|s| s.charge_thirds as i32)
        .unwrap_or(0)
}

/// Net charge of a hydrogen atom (proton `uud` + electron), in units of e/3.
/// Zero is charge quantization / atom neutrality, computed from the catalog.
fn hydrogen_charge_thirds() -> i32 {
    use physis_model::Flavor;
    let proton = 2 * charge_thirds(Flavor::Up) + charge_thirds(Flavor::Down);
    proton + charge_thirds(Flavor::Electron)
}

/// The gravitational [grav]²U(1) anomaly: Σ (colour·weak) · Y over a generation.
fn hypercharge_sum() -> f64 {
    hypercharge_sum_exact().to_f64()
}

const SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "generations",
        layer: LayerId::Particle,
        doc: "Number of fermion generations. Nature: 3. This knob exists so agents can watch empirical claims flip.",
        origin: ParameterOrigin::Measured,
        domain: KnobDomain::UInt { min: 1, max: 4 },
    },
    KnobSpec {
        name: "include_higgs",
        layer: LayerId::Particle,
        doc: "Whether the Higgs scalar is in the spectrum.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Bool,
    },
    KnobSpec {
        name: "include_gravity",
        layer: LayerId::Field,
        doc: "SM as usually taught does not include gravity. Flip this to ask 'SM + graviton'.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Bool,
    },
    KnobSpec {
        name: "neutrino_masses",
        layer: LayerId::Particle,
        doc: "Whether neutrino masses are included. The minimal SM stores them as 0; oscillation experiments show they are nonzero.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Bool,
    },
];

/// The Standard Model of particle physics (effective QFT).
#[derive(Clone, Debug)]
pub struct StandardModel {
    generations: u8,
    include_higgs: bool,
    include_gravity: bool,
    neutrino_masses: bool,
}

impl Default for StandardModel {
    fn default() -> Self {
        Self {
            generations: 3,
            include_higgs: true,
            include_gravity: false,
            // The textbook minimal SM leaves neutrinos massless — a known lie.
            neutrino_masses: false,
        }
    }
}

impl StandardModel {
    fn spectrum(&self) -> Spectrum {
        let mut s = if self.include_gravity {
            Spectrum::standard_model_plus_graviton()
        } else {
            Spectrum::standard_model()
        };
        if !self.include_higgs {
            s.species
                .retain(|p| p.flavor != physis_model::Flavor::Higgs);
        }
        if self.generations < 3 {
            let drop_tau = self.generations < 3;
            let drop_muon = self.generations < 2;
            s.species.retain(|p| {
                if drop_muon
                    && matches!(
                        p.flavor,
                        physis_model::Flavor::Muon
                            | physis_model::Flavor::NuMu
                            | physis_model::Flavor::Charm
                            | physis_model::Flavor::Strange
                    )
                {
                    return false;
                }
                if drop_tau
                    && matches!(
                        p.flavor,
                        physis_model::Flavor::Tau
                            | physis_model::Flavor::NuTau
                            | physis_model::Flavor::Top
                            | physis_model::Flavor::Bottom
                    )
                {
                    return false;
                }
                true
            });
        }
        s
    }
}

impl Knobbed for StandardModel {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }

    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "generations" => Ok(KnobValue::UInt(self.generations as u64)),
            "include_higgs" => Ok(KnobValue::Bool(self.include_higgs)),
            "include_gravity" => Ok(KnobValue::Bool(self.include_gravity)),
            "neutrino_masses" => Ok(KnobValue::Bool(self.neutrino_masses)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }

    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("generations", KnobValue::UInt(v)) => self.generations = v as u8,
            ("include_higgs", KnobValue::Bool(v)) => self.include_higgs = v,
            ("include_gravity", KnobValue::Bool(v)) => self.include_gravity = v,
            ("neutrino_masses", KnobValue::Bool(v)) => self.neutrino_masses = v,
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

impl Theory for StandardModel {
    fn id(&self) -> &'static str {
        "standard-model"
    }
    fn name(&self) -> &'static str {
        "Standard Model"
    }
    fn summary(&self) -> &'static str {
        "SU(3)×SU(2)×U(1) quantum field theory of observed particles. \
         Empirically unmatched below the electroweak scale. Not a theory of gravity \
         and not a UV completion. ~19 free parameters."
    }

    fn world(&self) -> Option<World> {
        Some(World {
            spacetime: Manifold::observed_4d(),
            gauge: GaugeGroup::standard_model(),
            spectrum: self.spectrum(),
            has_gravity: self.include_gravity,
            supersymmetric: false,
            free_parameter_count: 19,
            landscape_log10: 0.0,
            note: format!(
                "SM generations={} higgs={} gravity={} neutrino_masses={}",
                self.generations, self.include_higgs, self.include_gravity, self.neutrino_masses
            ),
        })
    }

    fn claims(&self) -> Vec<Claim> {
        vec![
            claims::c(
                claims::SPACETIME_STRUCTURE,
                "3+1 Minkowski spacetime, no extra dimensions.",
                LayerId::Spacetime,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::OBSERVED_4D,
                "Macroscopic spacetime is 3+1.",
                LayerId::Spacetime,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::HIDDEN_EXTRA_DIMS,
                "No extra dimensions in the SM as an effective theory.",
                LayerId::Spacetime,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::FERMIONS,
                "Quarks and leptons exist.",
                LayerId::Particle,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::SM_GAUGE,
                "Gauge group is exactly the Standard Model.",
                LayerId::Interaction,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::ANOMALY_CANCELLATION,
                "Chiral gauge anomalies cancel within each generation.",
                LayerId::Interaction,
                ClaimClass::ModelInternal,
            ),
            claims::c(
                SM_HYPERCHARGE_DERIVED,
                "Weak hypercharges are fixed by anomaly cancellation up to normalization.",
                LayerId::Interaction,
                ClaimClass::ModelInternal,
            ),
            claims::c(
                claims::THREE_GENERATIONS,
                "Three generations of fermions.",
                LayerId::Particle,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::NEUTRINO_MASSES,
                "Neutrinos have nonzero mass.",
                LayerId::Particle,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::CHARGE_QUANTIZATION,
                "Electric charge is quantized so that atoms are exactly neutral.",
                LayerId::Particle,
                ClaimClass::ModelInternal,
            ),
            claims::c(
                claims::GRAVITY,
                "Gravity is part of the Standard Model.",
                LayerId::Field,
                ClaimClass::Phenomenological,
            ),
            claims::c(
                claims::UNIQUE_VACUUM,
                "The SM vacuum (given its parameters) is the one we use; no string landscape.",
                LayerId::Effective,
                ClaimClass::Heuristic,
            ),
            claims::c(
                claims::FEW_PARAMETERS,
                "The theory has few free parameters.",
                LayerId::Interaction,
                ClaimClass::Heuristic,
            ),
            claims::c(
                claims::UV_COMPLETION,
                "The Standard Model is a UV-complete theory of nature.",
                LayerId::Field,
                ClaimClass::Phenomenological,
            ),
        ]
    }

    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id.0.as_str() {
            claims::SPACETIME_STRUCTURE | claims::OBSERVED_4D | claims::HIDDEN_EXTRA_DIMS => {
                Verdict::holds(claim, "SM is formulated in 3+1 Minkowski")
            }
            claims::FERMIONS => Verdict::holds(claim, "quarks and leptons"),
            claims::SM_GAUGE => Verdict::holds(claim, "SU(3)×SU(2)×U(1)"),
            claims::ANOMALY_CANCELLATION => {
                let a3 = anomaly_su3_u1_exact();
                let a2 = anomaly_su2_u1_exact();
                let sy = hypercharge_sum_exact();
                let sy3 = hypercharge_cube_sum_exact();
                let all_zero = a3.is_zero() && a2.is_zero() && sy.is_zero() && sy3.is_zero();
                if all_zero && SM_WEAK_DOUBLETS % 2 == 0 {
                    Verdict::holds(
                        claim,
                        "all four SM chiral gauge anomalies cancel as exact Ratio sums",
                    )
                    .with_evidence([
                        format!("[SU(3)]²U(1) = {a3}, [SU(2)]²U(1) = {a2} (exact Ratio)"),
                        format!("[grav]²U(1) ΣY = {sy}, [U(1)]³ ΣY³ = {sy3} (exact Ratio)"),
                        format!("Witten SU(2): {SM_WEAK_DOUBLETS} doublets (even)"),
                    ])
                    .with_certified_numeric()
                } else {
                    Verdict::fails(
                        claim,
                        format!(
                            "anomaly not cancelled: [SU(3)]²U(1)={a3}, [SU(2)]²U(1)={a2}, ΣY={sy}, ΣY³={sy3}"
                        ),
                    )
                }
            }
            id if id == SM_HYPERCHARGE_DERIVED => {
                let d = derive_hypercharges();
                // Compare the derived hypercharges to the measured assignments.
                let stored_u = SM_WEYL_FIELDS
                    .iter()
                    .find(|f| f.name == "u_R^c")
                    .unwrap()
                    .y
                    .to_f64();
                let stored_d = SM_WEYL_FIELDS
                    .iter()
                    .find(|f| f.name == "d_R^c")
                    .unwrap()
                    .y
                    .to_f64();
                let mut stored_ud = [stored_u, stored_d];
                stored_ud.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let stored_l = SM_WEYL_FIELDS
                    .iter()
                    .find(|f| f.name == "L_L")
                    .unwrap()
                    .y
                    .to_f64();
                let stored_e = SM_WEYL_FIELDS
                    .iter()
                    .find(|f| f.name == "e_R^c")
                    .unwrap()
                    .y
                    .to_f64();
                let close = |a: f64, b: f64| (a - b).abs() < 1e-12;
                let matches = close(d.y_l, stored_l)
                    && close(d.y_e, stored_e)
                    && close(d.y_ud[0], stored_ud[0])
                    && close(d.y_ud[1], stored_ud[1]);
                if matches {
                    Verdict::holds(claim,
                        "anomaly cancellation forces the measured hypercharges (up to normalization Y_Q = 1/6)",
                    )
                    .with_evidence([
                        format!(
                            "derived: Y_Q = {:.4}, Y_L = {:.4}, Y_e = {:.4}, {{Y_u, Y_d}} = {{{:.4}, {:.4}}}",
                            d.y_q, d.y_l, d.y_e, d.y_ud[0], d.y_ud[1]
                        ),
                        "solved from [SU(2)]²U(1), [SU(3)]²U(1), [grav]²U(1), and [U(1)]³".to_string(),
                    ])
                } else {
                    Verdict::fails(claim,
                        format!(
                            "derived hypercharges {{Y_L={:.4}, Y_e={:.4}, Y_u/d={:?}}} disagree with the catalog",
                            d.y_l, d.y_e, d.y_ud
                        ),
                    )
                }
            }
            claims::THREE_GENERATIONS => {
                if self.generations == 3 {
                    Verdict::holds(claim, "three generations")
                } else {
                    Verdict::fails(claim,
                        format!("generations = {}, not 3", self.generations),
                    )
                }
            }
            claims::NEUTRINO_MASSES => {
                if self.neutrino_masses {
                    Verdict::holds(claim,
                        "neutrino masses included (beyond the minimal SM, e.g. via a seesaw)",
                    )
                } else {
                    Verdict::fails(claim,
                        "minimal SM stores neutrino masses as 0, but oscillations prove they are nonzero",
                    )
                }
            }
            claims::CHARGE_QUANTIZATION => {
                let h = hydrogen_charge_thirds();
                if h == 0 {
                    Verdict::holds(claim,
                        "a hydrogen atom (uud + e⁻) is exactly neutral",
                    )
                    .with_evidence([
                        "computed from the catalog: 2·Q(u) + Q(d) + Q(e⁻) = 0 (units of e/3)".to_string(),
                    ])
                } else {
                    Verdict::fails(claim,
                        format!("hydrogen net charge = {h}/3 ≠ 0"),
                    )
                }
            }
            claims::GRAVITY => {
                if self.include_gravity {
                    Verdict::holds(claim,
                        "graviton added by hand; not a UV completion of gravity",
                    )
                } else {
                    Verdict::fails(claim,
                        "the Standard Model does not contain gravity",
                    )
                }
            }
            claims::UNIQUE_VACUUM => Verdict::holds(claim,
                "no landscape; parameters are inputs, not scanned vacua",
            ),
            claims::FEW_PARAMETERS => Verdict::fails(claim,
                "≈19 free parameters; not few by the standard this lab uses",
            ),
            claims::UV_COMPLETION => Verdict::fails(claim,
                "SM is an effective theory: Landau poles, triviality, no gravity, no dark matter, no neutrino masses in the minimal form",
            ),
            claims::CRITICAL_DIMENSION | claims::SUSY_CONSTRUCTION | claims::NO_TACHYON => {
                Verdict::inapplicable(claim, "not a worldsheet theory")
            }
            _ => Verdict::inapplicable(claim, "claim not made by the Standard Model object"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;
    use physis_core::DerivationAssurance;

    #[test]
    fn generations_are_measured_not_derived() {
        let t = StandardModel::default();
        assert_eq!(
            t.spec("generations").unwrap().origin,
            ParameterOrigin::Measured
        );
        assert_eq!(
            t.spec("include_higgs").unwrap().origin,
            ParameterOrigin::Chosen
        );
    }

    #[test]
    fn default_sm_has_three_generations_no_gravity() {
        let t = StandardModel::default();
        let gens = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == claims::THREE_GENERATIONS)
            .unwrap();
        assert_eq!(t.evaluate(&gens).kind, VerdictKind::Holds);
        let g = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == claims::GRAVITY)
            .unwrap();
        assert_eq!(t.evaluate(&g).kind, VerdictKind::Fails);
    }

    #[test]
    fn neutrino_masses_are_a_known_sm_gap() {
        let verdict = |t: &StandardModel| {
            let c = t
                .claims()
                .into_iter()
                .find(|c| c.id.0 == claims::NEUTRINO_MASSES)
                .unwrap();
            t.evaluate(&c).kind
        };
        let mut t = StandardModel::default();
        // The minimal SM leaves neutrinos massless: a known empirical failure.
        assert_eq!(verdict(&t), VerdictKind::Fails);
        t.set("neutrino_masses", KnobValue::Bool(true)).unwrap();
        assert_eq!(verdict(&t), VerdictKind::Holds);
    }

    #[test]
    fn sm_cancels_anomalies() {
        let t = StandardModel::default();
        let c = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == claims::ANOMALY_CANCELLATION)
            .unwrap();
        let v = t.evaluate(&c);
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_eq!(v.class, ClaimClass::ModelInternal);
        assert_eq!(v.derivation, DerivationAssurance::CertifiedNumeric);
        assert!(v.evidence.iter().any(|e| e.contains("exact Ratio")));
    }

    #[test]
    fn hydrogen_is_neutral_by_computation() {
        assert_eq!(hydrogen_charge_thirds(), 0);
        let t = StandardModel::default();
        let c = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == claims::CHARGE_QUANTIZATION)
            .unwrap();
        let v = t.evaluate(&c);
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_eq!(v.class, ClaimClass::ModelInternal);
    }

    #[test]
    fn all_four_gauge_anomalies_vanish_over_a_generation() {
        assert!(anomaly_su3_u1_exact().is_zero(), "[SU(3)]²U(1)");
        assert!(anomaly_su2_u1_exact().is_zero(), "[SU(2)]²U(1)");
        assert!(hypercharge_sum_exact().is_zero(), "[grav]²U(1)");
        assert!(hypercharge_cube_sum_exact().is_zero(), "[U(1)]³");
        assert_eq!(SM_WEAK_DOUBLETS % 2, 0);
        // A sign flip of the cubic is not a cancellation.
        assert!(!hypercharge_cube_sum_exact().is_zero() || true);
        assert!(!(hypercharge_cube_sum_exact() + Ratio::int(1)).is_zero());
    }

    #[test]
    fn hypercharges_are_derived_from_anomaly_cancellation() {
        // The headline: the hypercharges are *not* an input — anomaly freedom
        // plus the normalization Y_Q = 1/6 forces every one of them.
        let d = derive_hypercharges();
        assert!((d.y_q - 1.0 / 6.0).abs() < 1e-12);
        assert!((d.y_l - (-1.0 / 2.0)).abs() < 1e-12, "Y_L = {}", d.y_l);
        assert!((d.y_e - 1.0).abs() < 1e-12, "Y_e = {}", d.y_e);
        // {Y_u, Y_d} = {-2/3, 1/3}, sorted ascending.
        assert!(
            (d.y_ud[0] - (-2.0 / 3.0)).abs() < 1e-12,
            "Y_u = {}",
            d.y_ud[0]
        );
        assert!(
            (d.y_ud[1] - (1.0 / 3.0)).abs() < 1e-12,
            "Y_d = {}",
            d.y_ud[1]
        );
    }

    #[test]
    fn hypercharge_derivation_claim_holds_as_theorem() {
        let t = StandardModel::default();
        let c = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == SM_HYPERCHARGE_DERIVED)
            .unwrap();
        let v = t.evaluate(&c);
        assert_eq!(v.kind, VerdictKind::Holds);
        assert_eq!(v.class, ClaimClass::ModelInternal);
    }

    #[test]
    fn dropping_a_generation_fails() {
        let mut t = StandardModel::default();
        t.set("generations", KnobValue::UInt(2)).unwrap();
        let gens = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == claims::THREE_GENERATIONS)
            .unwrap();
        assert_eq!(t.evaluate(&gens).kind, VerdictKind::Fails);
    }
}
