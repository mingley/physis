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
//! fails. `epsilon_r` / `mu_r` still scale the isotropic-linear index.
//! Homogeneous Faraday (`dF = 0`) lives on the Maxwell vacuum package.
//! A magnetic current is a package mutation (`add-monopole`), not a
//! constitutive knob: the plane-wave residual of `∇×E + ∂B/∂t + J_m`
//! is no longer source-free and `em.faraday` fails.

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

/// Max residual of Gauss's law `∇·E = 0` for a Coulomb field away from its
/// source, by central finite differences. E = r̂/r² = r⃗/r³; its divergence
/// vanishes for r > 0 (all the charge is the delta function at the origin).
fn coulomb_gauss_residual() -> f64 {
    let h = 1e-4;
    let e = |x: f64, y: f64, z: f64| -> [f64; 3] {
        let r3 = (x * x + y * y + z * z).powf(1.5);
        [x / r3, y / r3, z / r3]
    };
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
        max = max.max((dex_dx + dey_dy + dez_dz).abs());
    }
    max
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
/// Tellegen magnetoelectric mixing, as a dimensionless index split.
const TELLEGEN_XI: f64 = 0.1;
/// Isotropic linear constitutive law on the live linear-medium package.
const LINEAR_EQ: &str = "constitutive isotropic-linear";
/// Tellegen magnetoelectric mixing (D mixes H, B mixes E).
const TELLEGEN_EQ: &str = "constitutive tellegen";

fn parse_constitutive(pkg: &TheoryPackage) -> Result<bool, String> {
    let mut linear = false;
    let mut tellegen = false;
    for eq in &pkg.equations {
        match eq.trim() {
            LINEAR_EQ => linear = true,
            TELLEGEN_EQ => tellegen = true,
            _ => {}
        }
    }
    if !linear {
        return Err(format!(
            "{} package has no isotropic-linear constitutive law",
            pkg.id
        ));
    }
    Ok(tellegen)
}

fn constitutive_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["isotropic linear D = εE, B = μH".into()],
        vec!["unique refractive index n = √(ε_r μ_r)".into()],
        "The constitutive cell is the isotropic-linear encoding. Tellegen \
         magnetoelectric mixing is a new encoding, not a silent ε_r knob.",
    )
}

fn parse_maxwell_bianchi(pkg: &TheoryPackage) -> Result<bool, String> {
    let mut homogeneous = false;
    let mut monopole = false;
    for eq in &pkg.equations {
        match eq.trim() {
            MAXWELL_DF_EQ => homogeneous = true,
            MONOPOLE_EQ => monopole = true,
            _ => {}
        }
    }
    if !homogeneous {
        return Err(format!(
            "{} package has no source-free homogeneous Faraday law",
            pkg.id
        ));
    }
    Ok(monopole)
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

/// Circular / magnetoelectric indices. Equal iff the constitutive law is
/// isotropic linear (no Tellegen ξ).
fn constitutive_indices(epsilon_r: f64, mu_r: f64, tellegen: bool) -> (f64, f64) {
    let n = refractive_index(epsilon_r, mu_r);
    if tellegen {
        (n + TELLEGEN_XI, n - TELLEGEN_XI)
    } else {
        (n, n)
    }
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

fn eval_em(epsilon_r: f64, mu_r: f64, tellegen: bool, monopole: bool, claim: &Claim) -> Verdict {
    let n = refractive_index(epsilon_r, mu_r);
    let vacuum = is_vacuum(epsilon_r, mu_r) && !tellegen;
    match claim.id_str() {
        WAVE_SPEED_C => {
            if vacuum {
                Verdict::holds(claim, "wave speed is 1/√(ε₀μ₀) = c").with_evidence([format!(
                    "ε₀·μ₀·c² = {:.6} (dimensionless, = 1)",
                    epsilon0().value() * mu0().value() * C.value() * C.value()
                )])
            } else if tellegen {
                let (np, nm) = constitutive_indices(epsilon_r, mu_r, true);
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
                let r = coulomb_gauss_residual();
                Verdict::holds(claim, "∇·E = 0 in vacuum away from charges")
                    .with_class(ClaimClass::ModelInternal)
                    .with_evidence([format!(
                        "verified numerically on a Coulomb field: max |∇·E| = {r:.1e}"
                    )])
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
            let (np, nm) = constitutive_indices(epsilon_r, mu_r, tellegen);
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
/// That fork is still this object, not a silent linear-medium install.
/// `epsilon_r` / `mu_r` stay on linear-medium.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MaxwellVacuum {
    monopole: bool,
}

impl MaxwellVacuum {
    /// IR package for this Bianchi encoding. Equations are `maxwell dF=0`
    /// and, when forked, `dF = *j_m`.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![MAXWELL_DF_EQ.to_string()];
        if self.monopole {
            equations.push(MONOPOLE_EQ.to_string());
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
        Ok(Self {
            monopole: parse_maxwell_bianchi(pkg)?,
        })
    }

    fn monopole_equation() -> String {
        MONOPOLE_EQ.to_string()
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
         magnetic current is an IR mutation, not a constitutive knob, and not \
         a silent linear-medium install."
    }
    fn world(&self) -> Option<World> {
        Some(em_world(
            1.0,
            1.0,
            if self.monopole {
                "Maxwell vacuum with magnetic current dF = *j_m".to_string()
            } else {
                "Maxwell vacuum".to_string()
            },
        ))
    }
    fn claims(&self) -> Vec<Claim> {
        em_claims()
            .into_iter()
            .map(|c| {
                if c.id_str() == FARADAY {
                    c.with_domain(faraday_domain())
                } else {
                    c
                }
            })
            .collect()
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        eval_em(1.0, 1.0, false, self.monopole, claim)
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.monopole = parsed.monopole;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        if self.monopole {
            return Vec::new();
        }
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(Self::monopole_equation()),
        );
        match Self::from_package(&mutated) {
            Ok(parsed) if parsed.monopole => {
                let mut fork = self.clone();
                fork.monopole = true;
                vec![("add-monopole".into(), Box::new(fork))]
            }
            _ => Vec::new(),
        }
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
/// index n = √(ε_r μ_r) splits. `epsilon_r` / `mu_r` still scale the
/// isotropic-linear index (electrically ordinary vs vacuum-like).
#[derive(Clone, Debug, PartialEq)]
pub struct LinearMedium {
    epsilon_r: f64,
    mu_r: f64,
    tellegen: bool,
}

impl Default for LinearMedium {
    fn default() -> Self {
        // A glass-like dielectric: n = 1.5, so light is slower than c.
        Self {
            epsilon_r: 2.25,
            mu_r: 1.0,
            tellegen: false,
        }
    }
}

impl LinearMedium {
    /// IR package for this constitutive law. Equations are
    /// `constitutive isotropic-linear` and, when forked, `constitutive tellegen`.
    /// ε_r and μ_r stay on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![LINEAR_EQ.to_string()];
        if self.tellegen {
            equations.push(TELLEGEN_EQ.to_string());
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
        Ok(Self {
            tellegen: parse_constitutive(pkg)?,
            ..Self::default()
        })
    }

    fn tellegen_equation() -> String {
        TELLEGEN_EQ.to_string()
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
         not an ε_r knob."
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
        eval_em(self.epsilon_r, self.mu_r, self.tellegen, false, claim)
    }
    fn ir_package(&self) -> Option<TheoryPackage> {
        Some(self.package())
    }
    fn reparse_package(&self, pkg: &TheoryPackage) -> Result<Box<dyn Theory>, String> {
        let parsed = Self::from_package(pkg)?;
        let mut fork = self.clone();
        fork.tellegen = parsed.tellegen;
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        if self.tellegen {
            return Vec::new();
        }
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(Self::tellegen_equation()),
        );
        match Self::from_package(&mutated) {
            Ok(parsed) if parsed.tellegen => {
                let mut fork = self.clone();
                fork.tellegen = true;
                vec![("add-tellegen".into(), Box::new(fork))]
            }
            _ => Vec::new(),
        }
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

fn parse_ohm_netlist(pkg: &TheoryPackage) -> Result<bool, String> {
    let mut branch = false;
    let mut tline = false;
    for eq in &pkg.equations {
        match eq.trim() {
            BRANCH_EQ => branch = true,
            TLINE_EQ => tline = true,
            _ => {}
        }
    }
    if !branch {
        return Err(format!("{} package has no lumped branch", pkg.id));
    }
    Ok(tline)
}

fn kcl_domain() -> DomainOfValidity {
    DomainOfValidity::new(
        vec!["lumped Kirchhoff nodes".into()],
        vec!["instantaneous current balance".into()],
        "KCL is the lumped node encoding. A transmission-line delay is a new \
         encoding, not a silent lumped circuit.",
    )
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
/// on the mutant. `frequency_hz` stays a knob (electrically short vs not).
#[derive(Clone, Debug, PartialEq)]
pub struct OhmCircuit {
    frequency_hz: f64,
    tline: bool,
}

impl Default for OhmCircuit {
    fn default() -> Self {
        // 1 kHz: comfortably quasi-static for a 0.1 m circuit.
        Self {
            frequency_hz: 1.0e3,
            tline: false,
        }
    }
}

impl OhmCircuit {
    /// IR package for this lumped netlist. Equations are `branch R 0 1`
    /// and, when forked, `tline 0 1`. Frequency stays on the struct.
    pub fn package(&self) -> TheoryPackage {
        let mut equations = vec![BRANCH_EQ.to_string()];
        if self.tline {
            equations.push(TLINE_EQ.to_string());
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
        Ok(Self {
            tline: parse_ohm_netlist(pkg)?,
            ..Self::default()
        })
    }

    fn tline_equation() -> String {
        TLINE_EQ.to_string()
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
         Wave propagation is dropped and there is a preferred rest frame. \
         Valid only while the wavelength dwarfs the circuit."
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
            FARADAY => Verdict::holds(
                claim,
                "inductor EMF / Kirchhoff's voltage law is Faraday's law",
            ),
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
        Ok(Box::new(fork))
    }
    fn structural_mutations(&self) -> Vec<(String, Box<dyn Theory>)> {
        if self.tline {
            return Vec::new();
        }
        let src = render_package(&self.package());
        let Ok(pkg) = parse_package(&src) else {
            return Vec::new();
        };
        let mutated = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation(Self::tline_equation()),
        );
        match Self::from_package(&mutated) {
            Ok(parsed) if parsed.tline => {
                let mut fork = self.clone();
                fork.tline = true;
                vec![("add-tline".into(), Box::new(fork))]
            }
            _ => Vec::new(),
        }
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
         not a knob). The linear-medium constitutive law is IR (`add-tellegen` \
         is an IR fork, not an ε_r knob). Homogeneous Faraday is the Maxwell \
         vacuum IR (`add-monopole` is an IR fork, not a constitutive knob).",
        vec![
            "`holds` / `fails` are internal to the encoding.".into(),
            "Vacuum wave speed is a theorem: ε₀·μ₀·c² = 1 (typed, checked).".into(),
            "A medium with n > 1 slows light and selects a rest frame, so wave-speed and Lorentz-invariance fail.".into(),
            "`hypothesize linear-medium`: add-tellegen is IR, not set.".into(),
            "`hypothesize maxwell-vacuum`: add-monopole is IR, not set.".into(),
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
        // In a medium, Gauss stays an encoded fact (macroscopic form).
        let glass = LinearMedium::default();
        let gauss_m = glass
            .claims()
            .into_iter()
            .find(|c| c.id_str() == GAUSS)
            .unwrap();
        assert_eq!(glass.evaluate(&gauss_m).class, ClaimClass::Phenomenological);
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
        assert_eq!(verdict(&fork, QUASI_STATIC_VALID), VerdictKind::Holds);
        c.set("frequency_hz", KnobValue::Float(1.0e10)).unwrap();
        assert_eq!(verdict(&c, QUASI_STATIC_VALID), VerdictKind::Fails);
        assert_eq!(verdict(&c, CHARGE_CONSERVATION), VerdictKind::Holds);
        let probes = OhmCircuit::default().structural_mutations();
        assert_eq!(probes.len(), 1);
        assert_eq!(probes[0].0, "add-tline");
        assert_eq!(
            verdict(probes[0].1.as_ref(), CHARGE_CONSERVATION),
            VerdictKind::Fails
        );
        assert!(fork.structural_mutations().is_empty());
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
        assert_eq!(probes.len(), 1);
        assert_eq!(probes[0].0, "add-tellegen");
        assert_eq!(
            verdict(probes[0].1.as_ref(), CONSTITUTIVE_LINEAR),
            VerdictKind::Fails
        );
        assert!(fork.structural_mutations().is_empty());
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
        assert_eq!(probes.len(), 1);
        assert_eq!(probes[0].0, "add-monopole");
        assert_eq!(verdict(probes[0].1.as_ref(), FARADAY), VerdictKind::Fails);
        assert!(fork.structural_mutations().is_empty());
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
            ofar.domain().is_encoding_wide(),
            "ohm-circuit Faraday stays encoding-wide: {:?}",
            ofar.domain()
        );
        assert_eq!(verdict(&ohm, FARADAY), VerdictKind::Holds);
    }
}
