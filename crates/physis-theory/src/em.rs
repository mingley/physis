//! Electromagnetism: the workspace's second scientific domain.
//!
//! This exists to prove `physis` is not a string-theory toy — the same typed
//! substrate (quantities, layers, knobs, claims, verdicts, the `Theory` trait,
//! and the experiment machinery) hosts classical electromagnetism without
//! forking the core. See `specs/007-reuse-domains.md` and
//! `specs/008-electromagnetism.md`.
//!
//! The flagship claim is mechanical, not a table: in vacuum the wave speed is
//! `1/√(ε₀μ₀) = c` (checked in `physis_model::constants`). In a linear medium
//! the refractive index `n = √(ε_r μ_r)` slows light below `c` and selects a
//! rest frame, so the wave-speed and Lorentz-invariance claims flip. The
//! constitutive law lives on the IR package. Tellegen magnetoelectric mixing
//! is a package mutation (`add-tellegen`), not an `ε_r` knob: the unique
//! index `n = √(ε_r μ_r)` splits (`n₊ ≠ n₋`) and `em.constitutive-linear`
//! fails. Pasteur chirality (`add-chiral`) is a second mutation: circular
//! birefringence `n_L ≠ n_R` and the same cell fails. `epsilon_r` / `mu_r`
//! still scale the isotropic-linear index.
//! Homogeneous Faraday (`dF = 0`) lives on the Maxwell vacuum package.
//! A magnetic current is a package mutation (`add-monopole`), not a
//! constitutive knob: the plane-wave residual of `∇×E + ∂B/∂t + J_m`
//! is no longer source-free and `em.faraday` fails. A Proca mass term is a
//! second package mutation (`add-proca`): the Coulomb residual of
//! `∇·E + m² φ` is no longer zero and `em.gauss` fails.

use physis_core::assumption::DomainOfValidity;
use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::ParameterOrigin;
use physis_ir::{apply_mutation, parse_package, render_package, PackageMutation, TheoryPackage};
use physis_model::constants::{epsilon0, mu0, C};
use physis_model::{GaugeGroup, Manifold, SimpleGroup, Species, Spectrum, World};

use crate::critique::{report_from_rows, ExperimentReport};
use crate::framework::Theory;

/// EM waves in vacuum travel at c (`1/√(ε₀μ₀)`).
pub const WAVE_SPEED_C: &str = "em.wave-speed-c";
/// Gauss's law for the electric field.
pub const GAUSS: &str = "em.gauss";
/// Faraday's law of induction.
pub const FARADAY: &str = "em.faraday";
/// Ampère–Maxwell law.
pub const AMPERE: &str = "em.ampere";
/// Local charge conservation (the continuity equation).
pub const CHARGE_CONSERVATION: &str = "em.charge-conservation";
/// Lorentz (boost) invariance of the field equations.
pub const LORENTZ_INVARIANCE: &str = "em.lorentz-invariance";
/// The lumped-element (quasi-static) approximation is valid.
pub const QUASI_STATIC_VALID: &str = "em.quasi-static-valid";
/// Constitutive law is isotropic linear: D = εE, B = μH, unique n = √(ε_r μ_r).
pub const CONSTITUTIVE_LINEAR: &str = "em.constitutive-linear";

/// Matrix rows for the electromagnetism lab.
pub fn em_rows() -> [&'static str; 8] {
    [
        WAVE_SPEED_C,
        GAUSS,
        FARADAY,
        AMPERE,
        CHARGE_CONSERVATION,
        CONSTITUTIVE_LINEAR,
        LORENTZ_INVARIANCE,
        QUASI_STATIC_VALID,
    ]
}

fn refractive_index(epsilon_r: f64, mu_r: f64) -> f64 {
    (epsilon_r * mu_r).sqrt()
}

// A vacuum plane wave travelling along +x in natural units (c = k = ω = 1):
// E = ŷ cos(x − t), B = ẑ cos(x − t). We verify numerically that it solves
// the source-free Maxwell equations, promoting Faraday/Ampère from encoded
// facts to computed theorems for the vacuum.
fn wave_ey(t: f64, x: f64) -> f64 {
    (x - t).cos()
}
fn wave_bz(t: f64, x: f64) -> f64 {
    (x - t).cos()
}

/// Max residual of Faraday's law `∂B/∂t + (∇×E) + J_m = 0` over sample
/// points, evaluated by central finite differences on the plane-wave
/// fields. `j_m = 0` is the source-free homogeneous encoding.
fn plane_wave_faraday_residual(j_m: f64) -> f64 {
    let h = 1e-4;
    let mut max = 0.0_f64;
    for i in 0..8 {
        let t = 0.13 * i as f64;
        let x = 0.29 * i as f64;
        let dbz_dt = (wave_bz(t + h, x) - wave_bz(t - h, x)) / (2.0 * h);
        let dey_dx = (wave_ey(t, x + h) - wave_ey(t, x - h)) / (2.0 * h);
        // (∇×E)_z = ∂E_y/∂x (E_x = 0); Faraday: ∂B_z/∂t + (∇×E)_z + J_m = 0.
        max = max.max((dbz_dt + dey_dx + j_m).abs());
    }
    max
}

/// Mixed second partial ∂²f/∂x_a∂x_b via the 4-point central stencil.
fn mixed_partial(f: &dyn Fn(f64, f64, f64) -> f64, a: usize, b: usize, p: [f64; 3], h: f64) -> f64 {
    let shift = |sa: f64, sb: f64| {
        let mut q = p;
        q[a] += sa * h;
        q[b] += sb * h;
        f(q[0], q[1], q[2])
    };
    (shift(1.0, 1.0) - shift(1.0, -1.0) - shift(-1.0, 1.0) + shift(-1.0, -1.0)) / (4.0 * h * h)
}

/// Max residual of `∇·(∇×A) = 0` for a smooth test field, by finite differences.
///
/// This vector-calculus identity is the mechanism behind local charge
/// conservation: `∂ρ/∂t + ∇·J = 0` follows because the divergence of a curl
/// vanishes (apply ∇· to the Ampère–Maxwell law).
fn div_curl_residual() -> f64 {
    let h = 1e-3;
    let ax = |x: f64, y: f64, _z: f64| x.sin() * y.cos();
    let ay = |_x: f64, y: f64, z: f64| y.sin() * z.cos();
    let az = |x: f64, _y: f64, z: f64| z.sin() * x.cos();
    let mut max = 0.0_f64;
    for &p in &[[0.5, 1.0, 1.5], [1.2, -0.7, 0.3], [2.0, 0.4, -1.1]] {
        // ∇·(∇×A) = ∂x(∂yAz − ∂zAy) + ∂y(∂zAx − ∂xAz) + ∂z(∂xAy − ∂yAx).
        let d = mixed_partial(&az, 0, 1, p, h) - mixed_partial(&ay, 0, 2, p, h)
            + mixed_partial(&ax, 1, 2, p, h)
            - mixed_partial(&az, 1, 0, p, h)
            + mixed_partial(&ay, 2, 0, p, h)
            - mixed_partial(&ax, 2, 1, p, h);
        max = max.max(d.abs());
    }
    max
}

/// Max residual of Gauss's law `∇·E + m² φ = 0` for a Coulomb field away
/// from its source, by central finite differences. E = r⃗/r³ and φ = 1/r.
/// Massless Maxwell (`m² = 0`) has vanishing divergence for r > 0; Proca
/// adds `m² φ`.
fn coulomb_gauss_law_residual(m2: f64) -> f64 {
    let h = 1e-4;
    let e = |x: f64, y: f64, z: f64| -> [f64; 3] {
        let r3 = (x * x + y * y + z * z).powf(1.5);
        [x / r3, y / r3, z / r3]
    };
    let phi = |x: f64, y: f64, z: f64| 1.0 / (x * x + y * y + z * z).sqrt();
    let mut max = 0.0_f64;
    for &(x, y, z) in &[
        (1.0, 2.0, 3.0),
        (2.0, -1.0, 1.0),
        (-1.5, 0.5, 2.0),
        (3.0, 3.0, -2.0),
    ] {
        let dex_dx = (e(x + h, y, z)[0] - e(x - h, y, z)[0]) / (2.0 * h);
        let dey_dy = (e(x, y + h, z)[1] - e(x, y - h, z)[1]) / (2.0 * h);
        let dez_dz = (e(x, y, z + h)[2] - e(x, y, z - h)[2]) / (2.0 * h);
        max = max.max((dex_dx + dey_dy + dez_dz + m2 * phi(x, y, z)).abs());
    }
    max
}

/// Max residual of Gauss's law `∇·E = 0` for a Coulomb field away from its
/// source, by central finite differences. E = r̂/r² = r⃗/r³; its divergence
/// vanishes for r > 0 (all the charge is the delta function at the origin).
fn coulomb_gauss_residual() -> f64 {
    coulomb_gauss_law_residual(0.0)
}

/// Max residual of Ampère's law `∂E/∂t − (∇×B) = 0` (sourceless vacuum) over
/// sample points, by central finite differences on the plane-wave fields.
fn plane_wave_ampere_residual() -> f64 {
    let h = 1e-4;
    let mut max = 0.0_f64;
    for i in 0..8 {
        let t = 0.13 * i as f64;
        let x = 0.29 * i as f64;
        let dey_dt = (wave_ey(t + h, x) - wave_ey(t - h, x)) / (2.0 * h);
        let dbz_dx = (wave_bz(t, x + h) - wave_bz(t, x - h)) / (2.0 * h);
        // (∇×B)_y = −∂B_z/∂x; Ampère: ∂E_y/∂t − (∇×B)_y = ∂E_y/∂t + ∂B_z/∂x = 0.
        max = max.max((dey_dt + dbz_dx).abs());
    }
    max
}

fn is_vacuum(epsilon_r: f64, mu_r: f64) -> bool {
    (refractive_index(epsilon_r, mu_r) - 1.0).abs() < 1e-9
}

/// Uniform magnetic current (natural units) on the `add-monopole` fork.
const MAGNETIC_CURRENT: f64 = 1.0;
/// Homogeneous Bianchi identity on the live Maxwell vacuum package.
const MAXWELL_DF_EQ: &str = "maxwell dF=0";
/// Magnetic current (inhomogeneous Bianchi) on the monopole fork.
const MONOPOLE_EQ: &str = "dF = *j_m";
/// Proca mass term on the massive-vector fork.
const PROCA_EQ: &str = "proca m2 A";
/// Proca m² in natural units. Not a knob; the term is the IR fork.
const PROCA_M2: f64 = 1.0;
/// Tellegen magnetoelectric mixing, as a dimensionless index split.
const TELLEGEN_XI: f64 = 0.1;
/// Pasteur chiral parameter, as a dimensionless circular-index split.
const CHIRAL_KAPPA: f64 = 0.1;
/// Isotropic linear constitutive law on the live linear-medium package.
const LINEAR_EQ: &str = "constitutive isotropic-linear";
/// Tellegen magnetoelectric mixing (D mixes H, B mixes E).
const TELLEGEN_EQ: &str = "constitutive tellegen";
/// Pasteur chiral mixing (circular birefringence n_L ≠ n_R).
const CHIRAL_EQ: &str = "constitutive chiral";

fn parse_constitutive(pkg: &TheoryPackage) -> Result<(bool, bool), String> {
    let mut linear = false;
    let mut tellegen = false;
    let mut chiral = false;
    for eq in &pkg.equations {
        match eq.trim() {
            LINEAR_EQ => linear = true,
            TELLEGEN_EQ => tellegen = true,
            CHIRAL_EQ => chiral = true,
            _ => {}
        }
    }
    if !linear {
        return Err(format!(
            "{} package has no isotropic-linear constitutive law",
            pkg.id
        ));
    }
    Ok((tellegen, chiral))
}

fn constitutive_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["isotropic linear D = εE, B = μH".into()],
        vec!["unique refractive index n = √(ε_r μ_r)".into()],
        "The constitutive cell is the isotropic-linear encoding. Tellegen \
         magnetoelectric mixing or Pasteur chiral mixing is a new encoding, \
         not a silent ε_r knob.",
    )
}

fn parse_maxwell_bianchi(pkg: &TheoryPackage) -> Result<(bool, bool), String> {
    let mut homogeneous = false;
    let mut monopole = false;
    let mut proca = false;
    for eq in &pkg.equations {
        match eq.trim() {
            MAXWELL_DF_EQ => homogeneous = true,
            MONOPOLE_EQ => monopole = true,
            PROCA_EQ => proca = true,
            _ => {}
        }
    }
    if !homogeneous {
        return Err(format!(
            "{} package has no source-free homogeneous Faraday law",
            pkg.id
        ));
    }
    Ok((monopole, proca))
}

fn faraday_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["source-free homogeneous dF=0".into()],
        vec!["no magnetic current; plane-wave Faraday residual".into()],
        "Maxwell Faraday is the source-free homogeneous encoding. \
         A magnetic current dF = *j_m is a new encoding, not a silent \
         constitutive knob.",
    )
}

fn gauss_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["source-free massless Maxwell".into()],
        vec!["Coulomb field nabla·E = 0 away from the origin".into()],
        "Maxwell Gauss is the massless encoding. A Proca mass term is a new \
         encoding, not a silent constitutive knob.",
    )
}

/// Circular / magnetoelectric indices. Equal iff the constitutive law is
/// isotropic linear (no Tellegen ξ, no Pasteur κ).
fn constitutive_indices(epsilon_r: f64, mu_r: f64, tellegen: bool, chiral: bool) -> (f64, f64) {
    let n = refractive_index(epsilon_r, mu_r);
    let mut np = n;
    let mut nm = n;
    if tellegen {
        np += TELLEGEN_XI;
        nm -= TELLEGEN_XI;
    }
    if chiral {
        np += CHIRAL_KAPPA;
        nm -= CHIRAL_KAPPA;
    }
    (np, nm)
}

fn em_claims() -> Vec<Claim> {
    vec![
        Claim::new(
            WAVE_SPEED_C,
            "Electromagnetic waves propagate at c.",
            LayerId::Field,
            ClaimClass::ModelInternal,
        ),
        Claim::new(
            GAUSS,
            "Gauss's law relates flux to enclosed charge.",
            LayerId::Field,
            ClaimClass::Phenomenological,
        ),
        Claim::new(
            FARADAY,
            "A changing magnetic field induces an electric field.",
            LayerId::Field,
            ClaimClass::Phenomenological,
        ),
        Claim::new(
            AMPERE,
            "Currents and changing electric fields source the magnetic field.",
            LayerId::Field,
            ClaimClass::Phenomenological,
        ),
        Claim::new(
            CHARGE_CONSERVATION,
            "Electric charge is locally conserved.",
            LayerId::Field,
            ClaimClass::ModelInternal,
        ),
        Claim::new(
            CONSTITUTIVE_LINEAR,
            "The constitutive law is isotropic linear: D = εE and B = μH.",
            LayerId::Field,
            ClaimClass::ModelInternal,
        ),
        Claim::new(
            LORENTZ_INVARIANCE,
            "The field equations are invariant under Lorentz boosts.",
            LayerId::Spacetime,
            ClaimClass::ModelInternal,
        ),
        Claim::new(
            QUASI_STATIC_VALID,
            "The lumped-element (quasi-static) approximation is valid.",
            LayerId::Effective,
            ClaimClass::Phenomenological,
        ),
    ]
}

fn eval_em(
    epsilon_r: f64,
    mu_r: f64,
    tellegen: bool,
    chiral: bool,
    monopole: bool,
    proca: bool,
    claim: &Claim,
) -> Verdict {
    let n = refractive_index(epsilon_r, mu_r);
    let vacuum = is_vacuum(epsilon_r, mu_r) && !tellegen && !chiral;
    match claim.id_str() {
        WAVE_SPEED_C => {
            if vacuum {
                Verdict::holds(claim, "wave speed is 1/√(ε₀μ₀) = c").with_evidence([format!(
                    "ε₀·μ₀·c² = {:.6} (dimensionless, = 1)",
                    epsilon0().value() * mu0().value() * C.value() * C.value()
                )])
            } else if chiral {
                let (nl, nr) = constitutive_indices(epsilon_r, mu_r, false, true);
                Verdict::fails(
                    claim,
                    format!(
                        "Pasteur chirality: no unique n = √(ε_r μ_r); n_L = {nl:.3}, n_R = {nr:.3}"
                    ),
                )
            } else if tellegen {
                let (np, nm) = constitutive_indices(epsilon_r, mu_r, true, false);
                Verdict::fails(
                    claim,
                    format!(
                        "Tellegen mixing: no unique n = √(ε_r μ_r); n₊ = {np:.3}, n₋ = {nm:.3}"
                    ),
                )
            } else {
                Verdict::fails(
                    claim,
                    format!("v = c/n with n = {n:.3}; light is slower than c in the medium"),
                )
            }
        }
        GAUSS => {
            if vacuum {
                let (m2, r) = if proca {
                    (PROCA_M2, coulomb_gauss_law_residual(PROCA_M2))
                } else {
                    (0.0, coulomb_gauss_residual())
                };
                if r < 1e-4 {
                    Verdict::holds(claim, "∇·E = 0 in vacuum away from charges")
                        .with_class(ClaimClass::ModelInternal)
                        .with_evidence([format!(
                            "verified numerically on a Coulomb field: max |∇·E + m²φ| = {r:.1e}"
                        )])
                } else {
                    Verdict::fails(
                        claim,
                        "Proca mass term: Coulomb ∇·E + m²φ is not source-free",
                    )
                    .with_class(ClaimClass::ModelInternal)
                    .with_evidence([format!(
                        "verified numerically on a Coulomb field: max |∇·E + m²φ| = {r:.3} (m² = {m2})"
                    )])
                }
            } else {
                Verdict::holds(claim, "∇·D = ρ_free (macroscopic form)")
            }
        }
        FARADAY => {
            if vacuum {
                let j_m = if monopole { MAGNETIC_CURRENT } else { 0.0 };
                let r = plane_wave_faraday_residual(j_m);
                if r < 1e-6 {
                    Verdict::holds(claim, "∇×E = −∂B/∂t")
                        .with_class(ClaimClass::ModelInternal)
                        .with_evidence([format!(
                            "verified numerically on a vacuum plane wave: max residual {r:.1e}"
                        )])
                } else {
                    Verdict::fails(
                        claim,
                        "homogeneous Faraday is not source-free: magnetic current J_m",
                    )
                    .with_class(ClaimClass::ModelInternal)
                    .with_evidence([format!(
                        "verified numerically on a vacuum plane wave: max |∂B/∂t + ∇×E + J_m| = {r:.3} (J_m = {j_m})"
                    )])
                }
            } else {
                Verdict::holds(claim, "∇×E = −∂B/∂t (macroscopic form in the medium)")
            }
        }
        AMPERE => {
            if vacuum {
                let r = plane_wave_ampere_residual();
                Verdict::holds(claim, "∇×B = ∂E/∂t (sourceless)")
                    .with_class(ClaimClass::ModelInternal)
                    .with_evidence([format!(
                        "verified numerically on a vacuum plane wave: max residual {r:.1e}"
                    )])
            } else {
                Verdict::holds(
                    claim,
                    "∇×H = J_free + ∂D/∂t (macroscopic form in the medium)",
                )
            }
        }
        CHARGE_CONSERVATION => Verdict::holds(
            claim,
            "∂ρ/∂t + ∇·J = 0 follows from Gauss + Ampère (divergence of the curl)",
        )
        .with_evidence([format!(
            "∇·(∇×A) = {:.1e} ≈ 0 verified numerically — the identity behind continuity",
            div_curl_residual()
        )]),
        CONSTITUTIVE_LINEAR => {
            if chiral {
                let (nl, nr) = constitutive_indices(epsilon_r, mu_r, false, true);
                Verdict::fails(
                    claim,
                    "Pasteur chirality: circular birefringence; D is not εE and B is not μH",
                )
                .with_evidence([format!(
                    "n_L = {nl:.3}, n_R = {nr:.3} (κ = {CHIRAL_KAPPA}); unique n = √(ε_r μ_r) splits"
                )])
            } else {
                let (np, nm) = constitutive_indices(epsilon_r, mu_r, tellegen, false);
                if (np - nm).abs() < 1e-12 {
                    Verdict::holds(
                        claim,
                        format!("isotropic linear D = εE, B = μH; unique n = {n:.3}"),
                    )
                    .with_evidence([format!("n₊ = n₋ = {np:.6} = √(ε_r μ_r)")])
                } else {
                    Verdict::fails(
                        claim,
                        "Tellegen magnetoelectric mixing: D is not εE and B is not μH",
                    )
                    .with_evidence([format!(
                        "n₊ = {np:.3}, n₋ = {nm:.3} (ξ = {TELLEGEN_XI}); unique n = √(ε_r μ_r) splits"
                    )])
                }
            }
        }
        LORENTZ_INVARIANCE => {
            if vacuum {
                Verdict::holds(
                    claim,
                    "vacuum Maxwell equations are invariant under Lorentz boosts",
                )
            } else {
                Verdict::fails(
                    claim,
                    "a material medium selects a rest frame, breaking boost invariance",
                )
            }
        }
        QUASI_STATIC_VALID => Verdict::inapplicable(
            claim,
            "full Maxwell theory, not a lumped-element approximation",
        ),
        _ => Verdict::inapplicable(claim, "claim not made by an electromagnetism object"),
    }
}

fn em_world(epsilon_r: f64, mu_r: f64, note: String) -> World {
    let mut spectrum = Spectrum::empty();
    spectrum.species.push(Species::photon());
    World {
        spacetime: Manifold::observed_4d(),
        gauge: GaugeGroup {
            factors: vec![SimpleGroup::U1],
        },
        spectrum,
        has_gravity: false,
        supersymmetric: false,
        free_parameter_count: 2,
        landscape_log10: 0.0,
        note: format!("{note} (n = {:.3})", refractive_index(epsilon_r, mu_r)),
    }
}

/// Classical electromagnetism in vacuum (the U(1) gauge theory of light).
///
/// Homogeneous Faraday lives on the IR package. A magnetic current is a
/// package mutation (`add-monopole`), not a knob: the plane-wave residual
/// of `∇×E + ∂B/∂t + J_m` is no longer source-free and `em.faraday` fails.
/// A Proca mass term is a second mutation (`add-proca`): the Coulomb residual
/// of `∇·E + m² φ` is no longer zero and `em.gauss` fails. Those forks are
/// still this object, not a silent linear-medium install.
/// `epsilon_r` / `mu_r` stay on linear-medium.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MaxwellVacuum {
    monopole: bool,
    /// Proca mass term. Not a knob.
    proca: bool,
}

impl MaxwellVacuum {
    /// IR package for this Bianchi encoding. Equations are `maxwell dF=0`
    /// and, when forked, `dF = *j_m` and/or `proca m2 A`.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![MAXWELL_DF_EQ.to_string()];
        if self.monopole {
            equations.push(MONOPOLE_EQ.to_string());
        }
        if self.proca {
            equations.push(PROCA_EQ.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["source-free-homogeneous".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: FARADAY.into(),
                statement: "A changing magnetic field induces an electric field.".into(),
                layer: "field".into(),
                class: "model-internal".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load a Faraday encoding from a package.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        if pkg.id != "maxwell-vacuum" {
            return Err(format!(
                "maxwell-vacuum package id '{}' is not maxwell-vacuum",
                pkg.id
            ));
        }
        let (monopole, proca) = parse_maxwell_bianchi(pkg)?;
        Ok(Self { monopole, proca })
    }

    fn monopole_equation() -> String {
        MONOPOLE_EQ.to_string()
    }

    fn proca_equation() -> String {
        PROCA_EQ.to_string()
    }
}

impl Knobbed for MaxwellVacuum {
    fn specs(&self) -> &'static [KnobSpec] {
        &[]
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        Err(CoreError::UnknownKnob { name: name.into() })
    }
    fn set(&mut self, name: &str, _value: KnobValue) -> Result<KnobValue, CoreError> {
        Err(CoreError::UnknownKnob { name: name.into() })
    }
}

impl Theory for MaxwellVacuum {
    fn id(&self) -> &'static str {
        "maxwell-vacuum"
    }
    fn name(&self) -> &'static str {
        "Maxwell (vacuum)"
    }
    fn summary(&self) -> &'static str {
        "Classical electromagnetism in vacuum: a U(1) gauge field whose waves \
         travel at 1/√(ε₀μ₀) = c. Homogeneous Faraday is an IR encoding. A \
         magnetic current is an IR mutation, not a constitutive knob. A Proca \
         mass term is a second IR mutation: Gauss fails on the massive vector. \
         Neither fork is a silent linear-medium install."
    }
    fn world(&self) -> Option<World> {
        Some(em_world(
            1.0,
            1.0,
            match (self.monopole, self.proca) {
                (true, true) => {
                    "Maxwell vacuum with magnetic current dF = *j_m and Proca mass term".to_string()
                }
                (true, false) => "Maxwell vacuum with magnetic current dF = *j_m".to_string(),
                (false, true) => "Maxwell vacuum with Proca mass term".to_string(),
                (false, false) => "Maxwell vacuum".to_string(),
            },
        ))
    }
    fn claims(&self) -> Vec<Claim> {
        em_claims()
            .into_iter()
            .map(|c| match c.id_str() {
                FARADAY => c.with_domain(faraday_domain()),
                GAUSS => c.with_domain(gauss_domain()),
                _ => c,
            })
            .collect()
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        eval_em(1.0, 1.0, false, false, self.monopole, self.proca, claim)
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.monopole = parsed.monopole;
        fork.proca = parsed.proca;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mut out: Vec<(String, Box<dyn Theory>)> = Vec::new();
        if !self.monopole {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::monopole_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.monopole {
                    let mut fork = self.clone();
                    fork.monopole = true;
                    out.push(("add-monopole".into(), Box::new(fork)));
                }
            }
        }
        if !self.proca {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::proca_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.proca {
                    let mut fork = self.clone();
                    fork.proca = true;
                    out.push(("add-proca".into(), Box::new(fork)));
                }
            }
        }
        out
    }
}

const MEDIUM_SPECS: &[KnobSpec] = &[
    KnobSpec {
        name: "epsilon_r",
        layer: LayerId::Effective,
        doc: "Relative permittivity ε_r of the linear medium (vacuum = 1). Raises the refractive index.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 1.0,
            max: 1.0e6,
        },
    },
    KnobSpec {
        name: "mu_r",
        layer: LayerId::Effective,
        doc: "Relative permeability μ_r of the linear medium (vacuum = 1). Raises the refractive index.",
        origin: ParameterOrigin::Chosen,
        domain: KnobDomain::Float {
            min: 1.0,
            max: 1.0e6,
        },
    },
];

/// Classical electromagnetism in a linear medium (ε_r, μ_r knobs).
///
/// The constitutive law lives on the IR package. Tellegen magnetoelectric
/// mixing is a package mutation (`add-tellegen`), not a knob: the unique
/// index n = √(ε_r μ_r) splits. Pasteur chirality (`add-chiral`) is a
/// second mutation: circular birefringence n_L ≠ n_R. `epsilon_r` / `mu_r`
/// still scale the isotropic-linear index (electrically ordinary vs
/// vacuum-like).
#[derive(Clone, Debug, PartialEq)]
pub struct LinearMedium {
    epsilon_r: f64,
    mu_r: f64,
    tellegen: bool,
    chiral: bool,
}

impl Default for LinearMedium {
    fn default() -> Self {
        // A glass-like dielectric: n = 1.5, so light is slower than c.
        Self {
            epsilon_r: 2.25,
            mu_r: 1.0,
            tellegen: false,
            chiral: false,
        }
    }
}

impl LinearMedium {
    /// IR package for this constitutive law. Equations are
    /// `constitutive isotropic-linear` and, when forked, `constitutive tellegen`
    /// and/or `constitutive chiral`. ε_r and μ_r stay on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![LINEAR_EQ.to_string()];
        if self.tellegen {
            equations.push(TELLEGEN_EQ.to_string());
        }
        if self.chiral {
            equations.push(CHIRAL_EQ.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["isotropic-linear-constitutive".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: CONSTITUTIVE_LINEAR.into(),
                statement: "The constitutive law is isotropic linear: D = εE and B = μH.".into(),
                layer: "field".into(),
                class: "model-internal".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load a constitutive encoding from a package. ε_r / μ_r default;
    /// overlay them from a live medium when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        if pkg.id != "linear-medium" {
            return Err(format!(
                "linear-medium package id '{}' is not linear-medium",
                pkg.id
            ));
        }
        let (tellegen, chiral) = parse_constitutive(pkg)?;
        Ok(Self {
            tellegen,
            chiral,
            ..Self::default()
        })
    }

    fn tellegen_equation() -> String {
        TELLEGEN_EQ.to_string()
    }

    fn chiral_equation() -> String {
        CHIRAL_EQ.to_string()
    }
}

impl Knobbed for LinearMedium {
    fn specs(&self) -> &'static [KnobSpec] {
        MEDIUM_SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "epsilon_r" => Ok(KnobValue::Float(self.epsilon_r)),
            "mu_r" => Ok(KnobValue::Float(self.mu_r)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("epsilon_r", KnobValue::Float(v)) => self.epsilon_r = v,
            ("mu_r", KnobValue::Float(v)) => self.mu_r = v,
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

impl Theory for LinearMedium {
    fn id(&self) -> &'static str {
        "linear-medium"
    }
    fn name(&self) -> &'static str {
        "Maxwell (linear medium)"
    }
    fn summary(&self) -> &'static str {
        "Classical electromagnetism in a linear dielectric/magnetic medium. The \
         refractive index n = √(ε_r μ_r) slows light below c and picks a rest \
         frame, so the wave-speed and Lorentz-invariance claims fail unless the \
         medium is trivial. Tellegen magnetoelectric mixing is an IR mutation, \
         not an ε_r knob. Pasteur chirality is a second IR mutation."
    }
    fn world(&self) -> Option<World> {
        Some(em_world(
            self.epsilon_r,
            self.mu_r,
            format!("linear medium ε_r={} μ_r={}", self.epsilon_r, self.mu_r),
        ))
    }
    fn claims(&self) -> Vec<Claim> {
        em_claims()
            .into_iter()
            .map(|c| {
                if c.id_str() == CONSTITUTIVE_LINEAR {
                    c.with_domain(constitutive_domain())
                } else {
                    c
                }
            })
            .collect()
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        eval_em(
            self.epsilon_r,
            self.mu_r,
            self.tellegen,
            self.chiral,
            false,
            false,
            claim,
        )
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.tellegen = parsed.tellegen;
        fork.chiral = parsed.chiral;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mut out: Vec<(String, Box<dyn Theory>)> = Vec::new();
        if !self.tellegen {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::tellegen_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.tellegen {
                    let mut fork = self.clone();
                    fork.tellegen = true;
                    out.push(("add-tellegen".into(), Box::new(fork)));
                }
            }
        }
        if !self.chiral {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::chiral_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.chiral {
                    let mut fork = self.clone();
                    fork.chiral = true;
                    out.push(("add-chiral".into(), Box::new(fork)));
                }
            }
        }
        out
    }
}

/// Characteristic size of the modelled circuit, in metres.
const CIRCUIT_SIZE_M: f64 = 0.1;
/// The lumped approximation needs the wavelength to dwarf the circuit.
const QUASI_STATIC_MARGIN: f64 = 100.0;
/// Lumped two-terminal resistor on nodes 0–1.
const BRANCH_EQ: &str = "branch R 0 1";
/// Distributed delay (transmission line) on the same nodes.
const TLINE_EQ: &str = "tline 0 1";
/// Unlumped mesh flux (Faraday dΦ/dt through the resistor loop).
const FLUX_EQ: &str = "loop dPhi/dt";
/// Uniform dB/dt through the square mesh of side `CIRCUIT_SIZE_M`.
const MESH_DB_DT: f64 = 1.0;

fn parse_ohm_netlist(pkg: &TheoryPackage) -> Result<(bool, bool), String> {
    let mut branch = false;
    let mut tline = false;
    let mut flux = false;
    for eq in &pkg.equations {
        match eq.trim() {
            BRANCH_EQ => branch = true,
            TLINE_EQ => tline = true,
            FLUX_EQ => flux = true,
            _ => {}
        }
    }
    if !branch {
        return Err(format!("{} package has no lumped branch", pkg.id));
    }
    Ok((tline, flux))
}

fn kcl_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["lumped Kirchhoff nodes".into()],
        vec!["instantaneous current balance".into()],
        "KCL is the lumped node encoding. A transmission-line delay is a new \
         encoding, not a silent lumped circuit.",
    )
}

fn kvl_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["lumped Kirchhoff voltage".into()],
        vec!["mesh flux localized to inductor branches".into()],
        "KVL is the lumped branch encoding. An unlumped mesh flux is a new \
         encoding, not a silent frequency knob.",
    )
}

/// Faraday residual of lumped KVL: ∮E·dl + dΦ/dt. The DC resistor loop has
/// vanishing ohmic drop; the lumped encoding drops mesh flux, so the residual
/// is dB/dt × L² on the `add-flux` fork.
fn lumped_faraday_residual(flux: bool) -> f64 {
    if flux {
        MESH_DB_DT * CIRCUIT_SIZE_M * CIRCUIT_SIZE_M
    } else {
        0.0
    }
}

const OHM_SPECS: &[KnobSpec] = &[KnobSpec {
    name: "frequency_hz",
    layer: LayerId::Effective,
    doc: "Operating frequency in Hz. The lumped approximation holds while the wavelength c/f dwarfs the circuit; at high frequency it breaks down.",
    origin: ParameterOrigin::Chosen,
    domain: KnobDomain::Float {
        min: 0.0,
        max: 1.0e18,
    },
}];

/// Ohm's-law lumped circuit theory: the quasi-static effective limit of Maxwell.
///
/// The lumped branch lives on the IR package. A transmission-line delay is a
/// package mutation (`add-tline`), not a knob: Kirchhoff current law fails
/// on the mutant. An unlumped mesh flux is a second mutation (`add-flux`):
/// the Faraday residual of ∮E·dl + dΦ/dt is dB/dt × L² and `em.faraday`
/// fails. `frequency_hz` stays a knob (electrically short vs not).
#[derive(Clone, Debug, PartialEq)]
pub struct OhmCircuit {
    frequency_hz: f64,
    tline: bool,
    /// Unlumped mesh flux. Not a knob.
    flux: bool,
}

impl Default for OhmCircuit {
    fn default() -> Self {
        // 1 kHz: comfortably quasi-static for a 0.1 m circuit.
        Self {
            frequency_hz: 1.0e3,
            tline: false,
            flux: false,
        }
    }
}

impl OhmCircuit {
    /// IR package for this lumped netlist. Equations are `branch R 0 1`
    /// and, when forked, `tline 0 1` and/or `loop dPhi/dt`. Frequency stays
    /// on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![BRANCH_EQ.to_string()];
        if self.tline {
            equations.push(TLINE_EQ.to_string());
        }
        if self.flux {
            equations.push(FLUX_EQ.to_string());
        }
        TheoryPackage {
            id: self.id().to_string(),
            name: self.name().to_string(),
            parameters: vec![],
            assumptions: vec!["lumped-kirchhoff-nodes".into()],
            equations,
            claims: vec![physis_ir::ClaimDecl {
                id: CHARGE_CONSERVATION.into(),
                statement: "Kirchhoff's current law is charge conservation.".into(),
                layer: "field".into(),
                class: "model-internal".into(),
            }],
            lean_ref: None,
        }
    }

    /// Load a lumped netlist from a package. Frequency defaults; overlay it
    /// from a live circuit when forking.
    pub fn from_package(pkg: &TheoryPackage) -> Result<Self, String> {
        if pkg.id != "ohm-circuit" {
            return Err(format!(
                "ohm-circuit package id '{}' is not ohm-circuit",
                pkg.id
            ));
        }
        let (tline, flux) = parse_ohm_netlist(pkg)?;
        Ok(Self {
            tline,
            flux,
            ..Self::default()
        })
    }

    fn tline_equation() -> String {
        TLINE_EQ.to_string()
    }

    fn flux_equation() -> String {
        FLUX_EQ.to_string()
    }

    /// Wavelength c/f as a typed length (infinite at DC).
    fn wavelength(&self) -> physis_core::Qty<physis_core::Length> {
        let lambda = if self.frequency_hz <= 0.0 {
            f64::INFINITY
        } else {
            C.value() / self.frequency_hz
        };
        physis_core::qty::meters(lambda)
    }

    fn quasi_static_valid(&self) -> bool {
        self.wavelength().value() > QUASI_STATIC_MARGIN * CIRCUIT_SIZE_M
    }
}

impl Knobbed for OhmCircuit {
    fn specs(&self) -> &'static [KnobSpec] {
        OHM_SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "frequency_hz" => Ok(KnobValue::Float(self.frequency_hz)),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, value) {
            ("frequency_hz", KnobValue::Float(v)) => self.frequency_hz = v,
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

impl Theory for OhmCircuit {
    fn id(&self) -> &'static str {
        "ohm-circuit"
    }
    fn name(&self) -> &'static str {
        "Ohm circuit (lumped)"
    }
    fn summary(&self) -> &'static str {
        "Lumped-element circuit theory: the quasi-static, long-wavelength limit \
         of Maxwell. Kirchhoff's current law is charge conservation on a lumped \
         node graph (a transmission-line delay is an IR mutation, not a knob). \
         Kirchhoff's voltage law is Faraday's law on a lumped mesh (unlumped \
         flux is a second IR mutation, not a frequency knob). Wave propagation \
         is dropped and there is a preferred rest frame. Valid only while the \
         wavelength dwarfs the circuit."
    }
    fn world(&self) -> Option<World> {
        Some(World {
            spacetime: Manifold::observed_4d(),
            gauge: GaugeGroup {
                factors: vec![SimpleGroup::U1],
            },
            spectrum: Spectrum::empty(),
            has_gravity: false,
            supersymmetric: false,
            free_parameter_count: 3,
            landscape_log10: 0.0,
            note: format!(
                "lumped circuit at {:.3e} Hz, wavelength {}",
                self.frequency_hz,
                self.wavelength()
            ),
        })
    }
    fn claims(&self) -> Vec<Claim> {
        em_claims()
            .into_iter()
            .map(|c| {
                if c.id_str() == QUASI_STATIC_VALID {
                    c.with_domain(DomainOfValidity::new(
                        vec!["λ > 100 × 0.1 m circuit".into()],
                        vec!["lumped elements; no wave propagation".into()],
                        "Quasi-static Ohm circuits, not full Maxwell. Using this \
                         when c/f is comparable to the circuit is a new claim, \
                         not a silent extrapolation.",
                    ))
                } else if c.id_str() == CHARGE_CONSERVATION {
                    c.with_domain(kcl_domain())
                } else if c.id_str() == FARADAY {
                    c.with_domain(kvl_domain())
                } else {
                    c
                }
            })
            .collect()
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        match claim.id_str() {
            WAVE_SPEED_C => Verdict::inapplicable(
                claim,
                "lumped circuits are the quasi-static limit; wave propagation is dropped",
            ),
            GAUSS => Verdict::holds(
                claim,
                "capacitor charge Q = CV is Gauss's law in the lumped limit",
            ),
            FARADAY => {
                let r = lumped_faraday_residual(self.flux);
                if r < 1e-9 {
                    Verdict::holds(
                        claim,
                        "inductor EMF / Kirchhoff's voltage law is Faraday's law",
                    )
                    .with_class(ClaimClass::ModelInternal)
                    .with_evidence([format!(
                        "lumped resistor loop: max |∮E·dl + dΦ/dt| = {r:.1e}"
                    )])
                } else {
                    Verdict::fails(
                        claim,
                        "unlumped mesh flux: lumped KVL is not Faraday (∮E·dl + dΦ/dt)",
                    )
                    .with_class(ClaimClass::ModelInternal)
                    .with_evidence([format!(
                        "square mesh of side {CIRCUIT_SIZE_M} m: \
                         max |∮E·dl + dΦ/dt| = {r:.3} (dB/dt = {MESH_DB_DT})"
                    )])
                }
            }
            AMPERE => Verdict::holds(claim, "displacement current shows up as capacitor current"),
            CHARGE_CONSERVATION => {
                if self.tline {
                    Verdict::fails(
                        claim,
                        "transmission-line delay: Kirchhoff current law is not instantaneous",
                    )
                } else {
                    Verdict::holds(
                        claim,
                        "Kirchhoff's current law is exactly charge conservation",
                    )
                }
            }
            LORENTZ_INVARIANCE => Verdict::fails(
                claim,
                "quasi-static circuit theory has a preferred (lab) rest frame",
            ),
            CONSTITUTIVE_LINEAR => Verdict::inapplicable(
                claim,
                "lumped circuits are not a field constitutive encoding",
            ),
            QUASI_STATIC_VALID => {
                if self.quasi_static_valid() {
                    Verdict::holds(
                        claim,
                        format!(
                            "wavelength {} dwarfs the {CIRCUIT_SIZE_M} m circuit",
                            self.wavelength()
                        ),
                    )
                } else {
                    Verdict::fails(claim,
                        format!(
                            "wavelength {} is comparable to the {CIRCUIT_SIZE_M} m circuit; lumped model breaks down",
                            self.wavelength()
                        ),
                    )
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by an electromagnetism object"),
        }
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.tline = parsed.tline;
        fork.flux = parsed.flux;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mut out: Vec<(String, Box<dyn Theory>)> = Vec::new();
        if !self.tline {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::tline_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.tline {
                    let mut fork = self.clone();
                    fork.tline = true;
                    out.push(("add-tline".into(), Box::new(fork)));
                }
            }
        }
        if !self.flux {
            let mutated = apply_mutation(
                &pkg,
                &PackageMutation::AppendEquation(Self::flux_equation()),
            );
            if let Ok(parsed) = Self::from_package(&mutated) {
                if parsed.flux {
                    let mut fork = self.clone();
                    fork.flux = true;
                    out.push(("add-flux".into(), Box::new(fork)));
                }
            }
        }
        out
    }
}

/// The electromagnetism experiment: Maxwell in vacuum vs a linear medium vs the
/// lumped-circuit effective theory.
pub fn em_vacuum() -> ExperimentReport {
    let theories: Vec<Box<dyn Theory>> = vec![
        Box::new(MaxwellVacuum::default()),
        Box::new(LinearMedium::default()),
        Box::new(OhmCircuit::default()),
    ];
    report_from_rows(
        "em-vacuum",
        "Electromagnetism lab",
        "Does the same typed substrate that hosts the string-critique lab also \
         host classical electromagnetism — and does the vacuum wave speed come \
         out as a theorem (1/√(ε₀μ₀) = c) that a medium can mechanically break?",
        "Maxwell's equations are encoded facts here; the vacuum wave speed and \
         charge conservation are theorems of the encoding. A linear medium is a \
         knob-controlled effective description, not new fundamental physics. \
         Lumped KCL is the ohm-circuit IR netlist (`add-tline` is an IR fork, \
         not a knob). Lumped KVL is Faraday on that netlist (`add-flux` is an \
         IR fork, not a frequency knob). The linear-medium constitutive law is IR (`add-tellegen` \
         and `add-chiral` are IR forks, not ε_r knobs). Homogeneous Faraday is the Maxwell \
         vacuum IR (`add-monopole` and `add-proca` are IR forks, not constitutive knobs).",
        vec![
            "`holds` / `fails` are internal to the encoding.".into(),
            "Vacuum wave speed is a theorem: ε₀·μ₀·c² = 1 (typed, checked).".into(),
            "A medium with n > 1 slows light and selects a rest frame, so wave-speed and Lorentz-invariance fail.".into(),
            "`hypothesize linear-medium`: add-tellegen and add-chiral are IR, not set.".into(),
            "`hypothesize maxwell-vacuum`: add-monopole and add-proca are IR, not set.".into(),
            "`hypothesize ohm-circuit`: add-tline and add-flux are IR, not set.".into(),
        ],
        &em_rows(),
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
    fn vacuum_wave_speed_is_a_theorem() {
        let v = MaxwellVacuum::default();
        assert_eq!(verdict(&v, WAVE_SPEED_C), VerdictKind::Holds);
        assert_eq!(verdict(&v, LORENTZ_INVARIANCE), VerdictKind::Holds);
        assert_eq!(verdict(&v, CHARGE_CONSERVATION), VerdictKind::Holds);
    }

    #[test]
    fn plane_wave_solves_vacuum_maxwell() {
        // The homogeneous Maxwell equations are verified numerically, so
        // Faraday/Ampère are computed theorems in vacuum.
        assert!(
            plane_wave_faraday_residual(0.0) < 1e-6,
            "faraday residual {}",
            plane_wave_faraday_residual(0.0)
        );
        assert!(
            plane_wave_ampere_residual() < 1e-6,
            "ampere residual {}",
            plane_wave_ampere_residual()
        );
        let v = MaxwellVacuum::default();
        let faraday = v
            .claims()
            .into_iter()
            .find(|c| c.id_str() == FARADAY)
            .unwrap();
        assert_eq!(v.evaluate(&faraday).class, ClaimClass::ModelInternal);
    }

    #[test]
    fn divergence_of_a_curl_vanishes() {
        // The identity behind charge conservation, verified numerically.
        assert!(
            div_curl_residual() < 1e-6,
            "div(curl A) residual {}",
            div_curl_residual()
        );
    }

    #[test]
    fn gauss_law_verified_on_a_coulomb_field() {
        assert!(
            coulomb_gauss_residual() < 1e-4,
            "gauss residual {}",
            coulomb_gauss_residual()
        );
        let v = MaxwellVacuum::default();
        let gauss = v
            .claims()
            .into_iter()
            .find(|c| c.id_str() == GAUSS)
            .unwrap();
        assert_eq!(v.evaluate(&gauss).class, ClaimClass::ModelInternal);
        assert!(
            !gauss.domain().is_encoding_wide(),
            "Maxwell Gauss must name massless vacuum: {:?}",
            gauss.domain()
        );
        // In a medium, Gauss stays an encoded fact (macroscopic form).
        let glass = LinearMedium::default();
        let gauss_m = glass
            .claims()
            .into_iter()
            .find(|c| c.id_str() == GAUSS)
            .unwrap();
        assert_eq!(glass.evaluate(&gauss_m).class, ClaimClass::Phenomenological);
        assert!(
            gauss_m.domain().is_encoding_wide(),
            "linear-medium Gauss stays encoding-wide: {:?}",
            gauss_m.domain()
        );
    }

    #[test]
    fn a_medium_slows_light_and_breaks_boosts() {
        let glass = LinearMedium::default();
        assert_eq!(verdict(&glass, WAVE_SPEED_C), VerdictKind::Fails);
        assert_eq!(verdict(&glass, LORENTZ_INVARIANCE), VerdictKind::Fails);
        // Maxwell's laws themselves still hold in the medium.
        assert_eq!(verdict(&glass, GAUSS), VerdictKind::Holds);
    }

    #[test]
    fn permittivity_knob_flips_the_wave_speed_verdict() {
        // The electricity knob → verdict diff required by M3.
        let mut m = LinearMedium::default();
        assert_eq!(verdict(&m, WAVE_SPEED_C), VerdictKind::Fails);
        assert_eq!(verdict(&m, CONSTITUTIVE_LINEAR), VerdictKind::Holds);
        m.set("epsilon_r", KnobValue::Float(1.0)).unwrap();
        assert_eq!(verdict(&m, WAVE_SPEED_C), VerdictKind::Holds);
        assert_eq!(verdict(&m, LORENTZ_INVARIANCE), VerdictKind::Holds);
        assert_eq!(verdict(&m, CONSTITUTIVE_LINEAR), VerdictKind::Holds);
    }

    #[test]
    fn em_experiment_builds_a_matrix() {
        let r = em_vacuum();
        assert_eq!(r.id, "em-vacuum");
        assert_eq!(r.theories.len(), 3);
        let wave = r.matrix.get(WAVE_SPEED_C).expect("row");
        assert_eq!(
            wave.get("maxwell-vacuum").copied(),
            Some(VerdictKind::Holds)
        );
        assert_eq!(wave.get("linear-medium").copied(), Some(VerdictKind::Fails));
        // The lumped circuit is the quasi-static limit: no wave propagation.
        assert_eq!(
            wave.get("ohm-circuit").copied(),
            Some(VerdictKind::Inapplicable)
        );
    }

    #[test]
    fn ohm_circuit_is_the_quasi_static_limit() {
        let c = OhmCircuit::default();
        assert_eq!(verdict(&c, WAVE_SPEED_C), VerdictKind::Inapplicable);
        assert_eq!(verdict(&c, LORENTZ_INVARIANCE), VerdictKind::Fails);
        assert_eq!(verdict(&c, CHARGE_CONSERVATION), VerdictKind::Holds);
        assert_eq!(verdict(&c, QUASI_STATIC_VALID), VerdictKind::Holds);
        let qs = c
            .claims()
            .into_iter()
            .find(|cl| cl.id_str() == QUASI_STATIC_VALID)
            .unwrap();
        assert!(
            !qs.domain().is_encoding_wide(),
            "lumped validity must name λ >> circuit size: {:?}",
            qs.domain()
        );
        let maxwell = MaxwellVacuum::default();
        let mqs = maxwell
            .claims()
            .into_iter()
            .find(|cl| cl.id_str() == QUASI_STATIC_VALID)
            .unwrap();
        assert!(
            mqs.domain().is_encoding_wide(),
            "Maxwell's inapplicable copy stays encoding-wide: {:?}",
            mqs.domain()
        );
    }

    #[test]
    fn high_frequency_breaks_the_lumped_approximation() {
        // The circuit knob → verdict diff.
        let mut c = OhmCircuit::default();
        assert_eq!(verdict(&c, QUASI_STATIC_VALID), VerdictKind::Holds);
        c.set("frequency_hz", KnobValue::Float(1.0e10)).unwrap();
        assert_eq!(verdict(&c, QUASI_STATIC_VALID), VerdictKind::Fails);
    }

    #[test]
    fn tline_delay_is_ir_not_a_knob() {
        let mut c = OhmCircuit::default();
        assert!(
            c.set("tline", KnobValue::Bool(true)).is_err(),
            "transmission-line delay is an IR mutation, not a knob"
        );
        let src = render_package(&c.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            OhmCircuit::from_package(&pkg).unwrap(),
            c,
            "IR round-trip must preserve the lumped branch"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(OhmCircuit::tline_equation()),
        );
        let parsed = OhmCircuit::from_package(&mutated).unwrap();
        assert!(parsed.tline);
        let mut fork = c.clone();
        fork.tline = true;
        assert_eq!(verdict(&fork, CHARGE_CONSERVATION), VerdictKind::Fails);
        assert_eq!(verdict(&c, CHARGE_CONSERVATION), VerdictKind::Holds);
        assert_eq!(verdict(&fork, FARADAY), VerdictKind::Holds);
        assert_eq!(verdict(&fork, QUASI_STATIC_VALID), VerdictKind::Holds);
        c.set("frequency_hz", KnobValue::Float(1.0e10)).unwrap();
        assert_eq!(verdict(&c, QUASI_STATIC_VALID), VerdictKind::Fails);
        assert_eq!(verdict(&c, CHARGE_CONSERVATION), VerdictKind::Holds);
        let probes = OhmCircuit::default().structural_mutations();
        assert_eq!(probes.len(), 2);
        assert!(
            probes.iter().any(|(label, _)| label == "add-tline"),
            "live ohm-circuit must offer add-tline: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        assert!(
            probes.iter().any(|(label, _)| label == "add-flux"),
            "live ohm-circuit must offer add-flux: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let tline_probe = probes
            .iter()
            .find(|(label, _)| label == "add-tline")
            .expect("add-tline");
        assert_eq!(
            verdict(tline_probe.1.as_ref(), CHARGE_CONSERVATION),
            VerdictKind::Fails
        );
        assert_eq!(verdict(tline_probe.1.as_ref(), FARADAY), VerdictKind::Holds);
        let tline_fork_probes = fork.structural_mutations();
        assert!(
            tline_fork_probes
                .iter()
                .all(|(label, _)| label != "add-tline"),
            "tline fork must not re-offer add-tline"
        );
        assert!(
            tline_fork_probes
                .iter()
                .any(|(label, _)| label == "add-flux"),
            "tline fork must still offer add-flux"
        );
        let live = OhmCircuit::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), live.package());
        assert_eq!(
            verdict(rebuilt.as_ref(), CHARGE_CONSERVATION),
            VerdictKind::Holds
        );
        let kcl = live
            .claims()
            .into_iter()
            .find(|cl| cl.id_str() == CHARGE_CONSERVATION)
            .unwrap();
        assert!(
            !kcl.domain().is_encoding_wide(),
            "ohm-circuit KCL must name lumped nodes: {:?}",
            kcl.domain()
        );
        let maxwell = MaxwellVacuum::default();
        let mkcl = maxwell
            .claims()
            .into_iter()
            .find(|cl| cl.id_str() == CHARGE_CONSERVATION)
            .unwrap();
        assert!(
            mkcl.domain().is_encoding_wide(),
            "Maxwell continuity stays encoding-wide: {:?}",
            mkcl.domain()
        );
    }

    #[test]
    fn tellegen_mixing_is_ir_not_a_knob() {
        let mut m = LinearMedium::default();
        assert!(
            LinearMedium::default()
                .set("tellegen", KnobValue::Bool(true))
                .is_err(),
            "Tellegen mixing is an IR mutation, not a knob"
        );
        let src = render_package(&m.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            LinearMedium::from_package(&pkg).unwrap(),
            m,
            "IR round-trip must preserve the isotropic-linear constitutive law"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(LinearMedium::tellegen_equation()),
        );
        let parsed = LinearMedium::from_package(&mutated).unwrap();
        assert!(parsed.tellegen);
        let mut fork = m.clone();
        fork.tellegen = true;
        assert_eq!(verdict(&fork, CONSTITUTIVE_LINEAR), VerdictKind::Fails);
        assert_eq!(verdict(&m, CONSTITUTIVE_LINEAR), VerdictKind::Holds);
        assert_eq!(verdict(&fork, GAUSS), VerdictKind::Holds);
        assert_eq!(verdict(&fork, CHARGE_CONSERVATION), VerdictKind::Holds);
        m.set("epsilon_r", KnobValue::Float(1.0)).unwrap();
        assert_eq!(verdict(&m, WAVE_SPEED_C), VerdictKind::Holds);
        assert_eq!(verdict(&m, CONSTITUTIVE_LINEAR), VerdictKind::Holds);
        let mut glass_tellegen = LinearMedium {
            tellegen: true,
            ..Default::default()
        };
        glass_tellegen
            .set("epsilon_r", KnobValue::Float(1.0))
            .unwrap();
        assert_eq!(
            verdict(&glass_tellegen, WAVE_SPEED_C),
            VerdictKind::Fails,
            "Tellegen n₊ ≠ n₋ is not vacuum even at ε_r = 1"
        );
        assert_eq!(
            verdict(&glass_tellegen, CONSTITUTIVE_LINEAR),
            VerdictKind::Fails
        );
        let probes = LinearMedium::default().structural_mutations();
        assert!(
            probes.iter().any(|(label, _)| label == "add-tellegen"),
            "live linear-medium must offer add-tellegen: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        assert!(
            probes.iter().any(|(label, _)| label == "add-chiral"),
            "live linear-medium must offer add-chiral: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let tellegen_probe = probes
            .iter()
            .find(|(label, _)| label == "add-tellegen")
            .expect("add-tellegen");
        assert_eq!(
            verdict(tellegen_probe.1.as_ref(), CONSTITUTIVE_LINEAR),
            VerdictKind::Fails
        );
        let tellegen_fork_probes = fork.structural_mutations();
        assert!(
            tellegen_fork_probes
                .iter()
                .all(|(label, _)| label != "add-tellegen"),
            "tellegen fork must not re-offer add-tellegen"
        );
        assert!(
            tellegen_fork_probes
                .iter()
                .any(|(label, _)| label == "add-chiral"),
            "tellegen fork must still offer add-chiral"
        );
        let live = LinearMedium::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), live.package());
        assert_eq!(
            rebuilt.get("epsilon_r").unwrap(),
            KnobValue::Float(2.25),
            "reparse must overlay constitutive IR onto live knobs"
        );
        assert_eq!(
            verdict(rebuilt.as_ref(), CONSTITUTIVE_LINEAR),
            VerdictKind::Holds
        );
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == CONSTITUTIVE_LINEAR)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "linear-medium constitutive must name D=εE: {:?}",
            cell.domain()
        );
        let maxwell = MaxwellVacuum::default();
        let mcell = maxwell
            .claims()
            .into_iter()
            .find(|c| c.id_str() == CONSTITUTIVE_LINEAR)
            .unwrap();
        assert!(
            mcell.domain().is_encoding_wide(),
            "Maxwell vacuum constitutive stays encoding-wide: {:?}",
            mcell.domain()
        );
        assert_eq!(verdict(&maxwell, CONSTITUTIVE_LINEAR), VerdictKind::Holds);
        let ohm = OhmCircuit::default();
        assert_eq!(
            verdict(&ohm, CONSTITUTIVE_LINEAR),
            VerdictKind::Inapplicable
        );
    }

    #[test]
    fn chiral_mixing_is_ir_not_a_knob() {
        let mut m = LinearMedium::default();
        assert!(
            LinearMedium::default()
                .set("chiral", KnobValue::Bool(true))
                .is_err(),
            "Pasteur chirality is an IR mutation, not a knob"
        );
        assert!(
            LinearMedium::default()
                .set("pasteur", KnobValue::Bool(true))
                .is_err(),
            "pasteur is not a knob"
        );
        assert!(
            LinearMedium::default()
                .set("kappa", KnobValue::Float(0.1))
                .is_err(),
            "kappa is not a knob"
        );
        assert!(
            LinearMedium::default()
                .set("frequency_hz", KnobValue::Float(1.0e10))
                .is_err(),
            "linear-medium must not grow a frequency_hz knob; that stays on ohm-circuit"
        );
        let src = render_package(&m.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            pkg.equations.len(),
            1,
            "live package must stay constitutive isotropic-linear"
        );
        assert_eq!(pkg.equations[0], LINEAR_EQ);
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(LinearMedium::chiral_equation()),
        );
        let parsed = LinearMedium::from_package(&mutated).unwrap();
        assert!(parsed.chiral);
        assert!(!parsed.tellegen);
        let mut fork = m.clone();
        fork.chiral = true;
        assert_eq!(verdict(&fork, CONSTITUTIVE_LINEAR), VerdictKind::Fails);
        assert_eq!(verdict(&m, CONSTITUTIVE_LINEAR), VerdictKind::Holds);
        assert_eq!(verdict(&fork, GAUSS), VerdictKind::Holds);
        assert_eq!(verdict(&fork, CHARGE_CONSERVATION), VerdictKind::Holds);
        let live_n = refractive_index(2.25, 1.0);
        let (nl, nr) = constitutive_indices(2.25, 1.0, false, true);
        assert!(
            (nl - (live_n + CHIRAL_KAPPA)).abs() < 1e-12
                && (nr - (live_n - CHIRAL_KAPPA)).abs() < 1e-12,
            "chiral evidence must be circular birefringence ±κ, got n_L={nl} n_R={nr} n={live_n}"
        );
        assert!(
            (nl - nr).abs() > 0.05,
            "circular birefringence must be the Pasteur scale, not a unit flag, got {}",
            nl - nr
        );
        let cell = fork
            .claims()
            .into_iter()
            .find(|c| c.id_str() == CONSTITUTIVE_LINEAR)
            .unwrap();
        let v = fork.evaluate(&cell);
        assert!(
            !v.summary.contains("Tellegen") && !v.summary.contains("magnetoelectric"),
            "chiral is not the Tellegen fork: {}",
            v.summary
        );
        m.set("epsilon_r", KnobValue::Float(1.0)).unwrap();
        assert_eq!(verdict(&m, WAVE_SPEED_C), VerdictKind::Holds);
        assert_eq!(verdict(&m, CONSTITUTIVE_LINEAR), VerdictKind::Holds);
        let mut vacuum_chiral = LinearMedium {
            chiral: true,
            ..Default::default()
        };
        vacuum_chiral
            .set("epsilon_r", KnobValue::Float(1.0))
            .unwrap();
        assert_eq!(
            verdict(&vacuum_chiral, WAVE_SPEED_C),
            VerdictKind::Fails,
            "chiral n_L ≠ n_R is not vacuum even at ε_r = 1"
        );
        assert_eq!(
            verdict(&vacuum_chiral, CONSTITUTIVE_LINEAR),
            VerdictKind::Fails,
            "chiral encoding must fail constitutive-linear even when ε_r = 1; κ → 0 would recover unique n and is not the encoding"
        );
        let probes = LinearMedium::default().structural_mutations();
        assert!(
            probes.iter().any(|(label, _)| label == "add-chiral"),
            "live linear-medium must offer add-chiral: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let chiral_probe = probes
            .iter()
            .find(|(label, _)| label == "add-chiral")
            .expect("add-chiral");
        assert_eq!(
            verdict(chiral_probe.1.as_ref(), CONSTITUTIVE_LINEAR),
            VerdictKind::Fails
        );
        let chiral_fork_probes = fork.structural_mutations();
        assert!(
            chiral_fork_probes
                .iter()
                .all(|(label, _)| label != "add-chiral"),
            "chiral fork must not re-offer add-chiral"
        );
        assert!(
            chiral_fork_probes
                .iter()
                .any(|(label, _)| label == "add-tellegen"),
            "chiral fork must still offer add-tellegen"
        );
        let live = LinearMedium::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), live.package());
        assert_eq!(
            rebuilt.get("epsilon_r").unwrap(),
            KnobValue::Float(2.25),
            "reparse must overlay chiral IR onto live knobs"
        );
        assert_eq!(
            verdict(rebuilt.as_ref(), CONSTITUTIVE_LINEAR),
            VerdictKind::Holds
        );
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == CONSTITUTIVE_LINEAR)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "linear-medium constitutive must name D=εE: {:?}",
            cell.domain()
        );
        assert!(
            MaxwellVacuum::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-chiral"),
            "maxwell-vacuum must not grow add-chiral"
        );
        assert!(
            OhmCircuit::default()
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-chiral"),
            "ohm-circuit must not grow add-chiral"
        );
        assert!(
            OhmCircuit::default()
                .set("frequency_hz", KnobValue::Float(1.0e6))
                .is_ok(),
            "ohm-circuit keeps the frequency_hz knob"
        );
    }

    #[test]
    fn monopole_current_is_ir_not_a_knob() {
        let v = MaxwellVacuum::default();
        assert!(
            MaxwellVacuum::default()
                .set("monopole", KnobValue::Bool(true))
                .is_err(),
            "magnetic current is an IR mutation, not a knob"
        );
        assert!(
            MaxwellVacuum::default()
                .set("epsilon_r", KnobValue::Float(1.0))
                .is_err(),
            "Maxwell must not grow an ε_r knob; that stays on linear-medium"
        );
        assert!(
            MaxwellVacuum::default()
                .set("tellegen", KnobValue::Bool(true))
                .is_err(),
            "Maxwell must not grow a Tellegen knob"
        );
        let src = render_package(&v.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            MaxwellVacuum::from_package(&pkg).unwrap(),
            v,
            "IR round-trip must preserve source-free homogeneous Faraday"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(MaxwellVacuum::monopole_equation()),
        );
        let parsed = MaxwellVacuum::from_package(&mutated).unwrap();
        assert!(parsed.monopole);
        let mut fork = v.clone();
        fork.monopole = true;
        assert_eq!(verdict(&fork, FARADAY), VerdictKind::Fails);
        assert_eq!(verdict(&v, FARADAY), VerdictKind::Holds);
        assert_eq!(verdict(&fork, GAUSS), VerdictKind::Holds);
        assert_eq!(verdict(&fork, AMPERE), VerdictKind::Holds);
        assert_eq!(verdict(&fork, WAVE_SPEED_C), VerdictKind::Holds);
        assert_eq!(verdict(&fork, LORENTZ_INVARIANCE), VerdictKind::Holds);
        assert_eq!(verdict(&fork, CHARGE_CONSERVATION), VerdictKind::Holds);
        assert_eq!(verdict(&fork, CONSTITUTIVE_LINEAR), VerdictKind::Holds);
        assert_eq!(
            verdict(&fork, QUASI_STATIC_VALID),
            VerdictKind::Inapplicable
        );
        let jm = plane_wave_faraday_residual(MAGNETIC_CURRENT);
        assert!(
            (jm - MAGNETIC_CURRENT).abs() < 1e-6,
            "inhomogeneous Faraday residual must be the magnetic current, got {jm}"
        );
        let probes = MaxwellVacuum::default().structural_mutations();
        assert_eq!(probes.len(), 2);
        assert!(
            probes.iter().any(|(label, _)| label == "add-monopole"),
            "live Maxwell must offer add-monopole: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        assert!(
            probes.iter().any(|(label, _)| label == "add-proca"),
            "live Maxwell must offer add-proca: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let monopole_probe = probes
            .iter()
            .find(|(label, _)| label == "add-monopole")
            .unwrap();
        assert_eq!(
            verdict(monopole_probe.1.as_ref(), FARADAY),
            VerdictKind::Fails
        );
        let monopole_fork_probes = fork.structural_mutations();
        assert!(
            monopole_fork_probes
                .iter()
                .all(|(label, _)| label != "add-monopole"),
            "monopole fork must not re-offer add-monopole"
        );
        assert!(
            monopole_fork_probes
                .iter()
                .any(|(label, _)| label == "add-proca"),
            "monopole fork must still offer add-proca"
        );
        let live = MaxwellVacuum::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), live.package());
        assert_eq!(verdict(rebuilt.as_ref(), FARADAY), VerdictKind::Holds);
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == FARADAY)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "Maxwell Faraday must name source-free dF=0: {:?}",
            cell.domain()
        );
        let glass = LinearMedium::default();
        let gfar = glass
            .claims()
            .into_iter()
            .find(|c| c.id_str() == FARADAY)
            .unwrap();
        assert!(
            gfar.domain().is_encoding_wide(),
            "linear-medium Faraday stays encoding-wide: {:?}",
            gfar.domain()
        );
        assert_eq!(verdict(&glass, FARADAY), VerdictKind::Holds);
        assert!(
            glass
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-monopole"),
            "linear-medium must not grow add-monopole"
        );
        let ohm = OhmCircuit::default();
        let ofar = ohm
            .claims()
            .into_iter()
            .find(|c| c.id_str() == FARADAY)
            .unwrap();
        assert!(
            !ofar.domain().is_encoding_wide(),
            "ohm-circuit Faraday must name lumped KVL: {:?}",
            ofar.domain()
        );
        assert!(
            ofar.domain()
                .regimes
                .iter()
                .any(|r| r.contains("lumped Kirchhoff voltage")),
            "ohm Faraday regime must be lumped Kirchhoff voltage: {:?}",
            ofar.domain()
        );
        assert_eq!(verdict(&ohm, FARADAY), VerdictKind::Holds);
        assert!(
            ohm.structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-monopole"),
            "ohm-circuit must not grow add-monopole"
        );
    }

    #[test]
    fn flux_mesh_is_ir_not_a_knob() {
        let mut c = OhmCircuit::default();
        assert!(
            c.set("flux", KnobValue::Bool(true)).is_err(),
            "unlumped mesh flux is an IR mutation, not a knob"
        );
        assert!(
            c.set("dPhi", KnobValue::Float(1.0)).is_err(),
            "dΦ/dt is not a knob"
        );
        let src = render_package(&c.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            pkg.equations.len(),
            1,
            "live package must stay a lumped branch"
        );
        assert_eq!(pkg.equations[0], BRANCH_EQ);
        assert_eq!(
            OhmCircuit::from_package(&pkg).unwrap(),
            c,
            "IR round-trip must preserve the lumped branch"
        );
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(OhmCircuit::flux_equation()),
        );
        let parsed = OhmCircuit::from_package(&mutated).unwrap();
        assert!(parsed.flux);
        assert!(!parsed.tline);
        let mut fork = c.clone();
        fork.flux = true;
        assert_eq!(verdict(&fork, FARADAY), VerdictKind::Fails);
        assert_eq!(verdict(&c, FARADAY), VerdictKind::Holds);
        assert_eq!(verdict(&fork, CHARGE_CONSERVATION), VerdictKind::Holds);
        assert_eq!(verdict(&fork, QUASI_STATIC_VALID), VerdictKind::Holds);
        assert_eq!(verdict(&fork, GAUSS), VerdictKind::Holds);
        let r = lumped_faraday_residual(true);
        assert!(
            (r - 0.01).abs() < 1e-12,
            "Faraday residual must be dB/dt × L² = 0.01, got {r}"
        );
        assert!(
            (r - 1.0).abs() > 0.5,
            "Faraday residual must be the mesh area scale, not a unit flag, got {r}"
        );
        assert_eq!(lumped_faraday_residual(false), 0.0);
        c.set("frequency_hz", KnobValue::Float(1.0e10)).unwrap();
        assert_eq!(verdict(&c, QUASI_STATIC_VALID), VerdictKind::Fails);
        assert_eq!(verdict(&c, FARADAY), VerdictKind::Holds);
        let probes = OhmCircuit::default().structural_mutations();
        assert!(
            probes.iter().any(|(label, _)| label == "add-flux"),
            "live ohm-circuit must offer add-flux: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let flux_probe = probes
            .iter()
            .find(|(label, _)| label == "add-flux")
            .expect("add-flux");
        assert_eq!(verdict(flux_probe.1.as_ref(), FARADAY), VerdictKind::Fails);
        assert_eq!(
            verdict(flux_probe.1.as_ref(), CHARGE_CONSERVATION),
            VerdictKind::Holds
        );
        let flux_fork_probes = fork.structural_mutations();
        assert!(
            flux_fork_probes
                .iter()
                .all(|(label, _)| label != "add-flux"),
            "flux fork must not re-offer add-flux"
        );
        assert!(
            flux_fork_probes
                .iter()
                .any(|(label, _)| label == "add-tline"),
            "flux fork must still offer add-tline"
        );
        let mut live = OhmCircuit::default();
        live.set("frequency_hz", KnobValue::Float(5.0e3)).unwrap();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), live.package());
        assert_eq!(
            rebuilt.get("frequency_hz").unwrap(),
            KnobValue::Float(5.0e3),
            "reparse must overlay flux IR onto live frequency_hz"
        );
        assert_eq!(verdict(rebuilt.as_ref(), FARADAY), VerdictKind::Holds);
        let cell = live
            .claims()
            .into_iter()
            .find(|cl| cl.id_str() == FARADAY)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "ohm-circuit Faraday must name lumped KVL: {:?}",
            cell.domain()
        );
        let maxwell = MaxwellVacuum::default();
        assert!(
            maxwell
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-flux"),
            "Maxwell must not grow add-flux"
        );
        let glass = LinearMedium::default();
        let gfar = glass
            .claims()
            .into_iter()
            .find(|cl| cl.id_str() == FARADAY)
            .unwrap();
        assert!(
            gfar.domain().is_encoding_wide(),
            "linear-medium Faraday stays encoding-wide: {:?}",
            gfar.domain()
        );
        assert!(
            glass
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-flux"),
            "linear-medium must not grow add-flux"
        );
        assert_eq!(verdict(&glass, FARADAY), VerdictKind::Holds);
    }

    #[test]
    fn proca_mass_is_ir_not_a_knob() {
        let v = MaxwellVacuum::default();
        assert!(
            MaxwellVacuum::default()
                .set("proca", KnobValue::Bool(true))
                .is_err(),
            "Proca mass is an IR mutation, not a knob"
        );
        assert!(
            MaxwellVacuum::default()
                .set("mass", KnobValue::Float(1.0))
                .is_err(),
            "Proca m² is not a knob"
        );
        assert!(
            MaxwellVacuum::default()
                .set("epsilon_r", KnobValue::Float(1.0))
                .is_err(),
            "Maxwell must not grow an ε_r knob; that stays on linear-medium"
        );
        let src = render_package(&v.package());
        let pkg = parse_package(&src).unwrap();
        assert_eq!(
            pkg.equations.len(),
            1,
            "live package must stay source-free Faraday"
        );
        assert_eq!(pkg.equations[0], MAXWELL_DF_EQ);
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(MaxwellVacuum::proca_equation()),
        );
        let parsed = MaxwellVacuum::from_package(&mutated).unwrap();
        assert!(parsed.proca);
        let mut fork = v.clone();
        fork.proca = true;
        assert_eq!(verdict(&fork, GAUSS), VerdictKind::Fails);
        assert_eq!(verdict(&v, GAUSS), VerdictKind::Holds);
        assert_eq!(verdict(&fork, FARADAY), VerdictKind::Holds);
        assert_eq!(verdict(&fork, AMPERE), VerdictKind::Holds);
        assert_eq!(verdict(&fork, WAVE_SPEED_C), VerdictKind::Holds);
        assert_eq!(verdict(&fork, LORENTZ_INVARIANCE), VerdictKind::Holds);
        assert_eq!(verdict(&fork, CHARGE_CONSERVATION), VerdictKind::Holds);
        assert_eq!(verdict(&fork, CONSTITUTIVE_LINEAR), VerdictKind::Holds);
        assert_eq!(
            verdict(&fork, QUASI_STATIC_VALID),
            VerdictKind::Inapplicable
        );
        let r = coulomb_gauss_law_residual(PROCA_M2);
        let phi_scale = 1.0 / 6.0_f64.sqrt();
        assert!(
            (r - PROCA_M2 * phi_scale).abs() < 1e-3,
            "Proca Gauss residual must be m²φ on the Coulomb samples, got {r} vs {}",
            PROCA_M2 * phi_scale
        );
        assert!(
            (r - 1.0).abs() > 0.3,
            "Proca residual must be the Coulomb m²φ scale, not a unit flag, got {r}"
        );
        let probes = MaxwellVacuum::default().structural_mutations();
        assert!(
            probes.iter().any(|(label, _)| label == "add-proca"),
            "live Maxwell must offer add-proca: {:?}",
            probes.iter().map(|(l, _)| l.as_str()).collect::<Vec<_>>()
        );
        let proca_probe = probes
            .iter()
            .find(|(label, _)| label == "add-proca")
            .expect("add-proca");
        assert_eq!(verdict(proca_probe.1.as_ref(), GAUSS), VerdictKind::Fails);
        let proca_fork_probes = fork.structural_mutations();
        assert!(
            proca_fork_probes
                .iter()
                .all(|(label, _)| label != "add-proca"),
            "proca fork must not re-offer add-proca"
        );
        assert!(
            proca_fork_probes
                .iter()
                .any(|(label, _)| label == "add-monopole"),
            "proca fork must still offer add-monopole"
        );
        let live = MaxwellVacuum::default();
        let canonical = physis_ir::certify_round_trip(&live.ir_package().unwrap()).unwrap();
        let parsed = parse_package(&canonical).unwrap();
        let rebuilt = live.reparse_package(&parsed).unwrap();
        assert_eq!(rebuilt.ir_package().unwrap(), live.package());
        assert_eq!(verdict(rebuilt.as_ref(), GAUSS), VerdictKind::Holds);
        let cell = live
            .claims()
            .into_iter()
            .find(|c| c.id_str() == GAUSS)
            .unwrap();
        assert!(
            !cell.domain().is_encoding_wide(),
            "Maxwell Gauss must name massless vacuum: {:?}",
            cell.domain()
        );
        let glass = LinearMedium::default();
        let ggauss = glass
            .claims()
            .into_iter()
            .find(|c| c.id_str() == GAUSS)
            .unwrap();
        assert!(
            ggauss.domain().is_encoding_wide(),
            "linear-medium Gauss stays encoding-wide: {:?}",
            ggauss.domain()
        );
        assert_eq!(verdict(&glass, GAUSS), VerdictKind::Holds);
        assert!(
            glass
                .structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-proca"),
            "linear-medium must not grow add-proca"
        );
        let ohm = OhmCircuit::default();
        let ogauss = ohm
            .claims()
            .into_iter()
            .find(|c| c.id_str() == GAUSS)
            .unwrap();
        assert!(
            ogauss.domain().is_encoding_wide(),
            "ohm-circuit Gauss stays encoding-wide: {:?}",
            ogauss.domain()
        );
        assert_eq!(verdict(&ohm, GAUSS), VerdictKind::Holds);
        assert!(
            ohm.structural_mutations()
                .iter()
                .all(|(label, _)| label != "add-proca"),
            "ohm-circuit must not grow add-proca"
        );
        assert!(
            LinearMedium::default()
                .set("epsilon_r", KnobValue::Float(1.0))
                .is_ok(),
            "linear-medium keeps the epsilon_r knob"
        );
    }
}
