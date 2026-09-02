//! One-loop renormalization-group running of the gauge couplings.
//!
//! This turns "do the couplings unify?" from an asserted encoded fact into a
//! *computation*. At one loop each inverse coupling runs linearly in
//! `t = ln(μ/M_Z)`:
//!
//! ```text
//! α_i⁻¹(μ) = α_i⁻¹(M_Z) − (b_i / 2π) · t
//! ```
//!
//! with the standard one-loop coefficients `b_i` (GUT-normalized U(1)). Given
//! the measured couplings at `M_Z`, we fix the unification point from the two
//! electroweak lines (`α_1`, `α_2`), then *predict* `α_3(M_Z)` and compare it to
//! experiment. The minimal Standard Model misses by tens of percent; the MSSM
//! agrees to about a percent — the celebrated (approximate) unification.
//!
//! The same one-loop betas give the Georgi–Quinn–Weinberg prediction of
//! `sin²θ_W(M_Z)` from `α_em` and `α_s` alone (no measured mixing angle):
//! minimal SU(5) lands near 0.21 and misses; the MSSM lands on 0.231.
//! The GQW *centre* is an exact `Ratio`: `2π` cancels, so the prediction
//! is a rational function of recorded PDG decimals and the one-loop
//! betas. The empirical sibling encloses that function by the sourced
//! PDG 2022 one-sigma hulls of `α_em⁻¹(M_Z)` and `α_s(M_Z)` — not a 3%
//! remainder certificate. Complementary one-loop unification predicts
//! `α_3(M_Z)` from `α_em⁻¹` and `sin²θ_W`: `2π` cancels the same way,
//! so that centre is also a `Ratio`. The empirical sibling encloses it
//! by the sourced PDG 2024 mixing-angle hull and the PDG 2022
//! `α_em⁻¹` hull, versus the PDG `α_s` hull — not a 3% remainder
//! certificate, and not the GQW mixing-angle cell. The third vertex
//! predicts `α_em⁻¹(M_Z)` from `α_s` and `sin²θ_W`: `2π` cancels
//! the same way. That enclosure is compared to the tight PDG
//! `α_em⁻¹` hull — one-loop MSSM is still disjoint there, because
//! `α_em` is known much more precisely than `α_s`. `M_U` still uses
//! `ln`/`exp` and is approximate evidence.
//!
//! One loop is an approximation: two-loop terms and SUSY thresholds shift the
//! numbers at the percent level. The verdicts that consume this are therefore
//! tagged `Heuristic`, with the computed numbers as evidence.

use std::f64::consts::PI;

use physis_data::{pdg_2022_alpha_s_mz, pdg_2022_inv_alpha_em_mz, pdg_2024_sin2theta};
use physis_model::constants::{
    inverse_alpha_em_mz, strong_coupling_mz, weak_mixing_angle_sin2_mz, z_mass_gev,
};
use physis_numeric::{Interval, Ratio};

/// SM one-loop betas `(41/10, −19/6, −7)`, GUT-normalized U(1).
const SM_ONE_LOOP_B: [Ratio; 3] = [Ratio::new(41, 10), Ratio::new(-19, 6), Ratio::int(-7)];
/// MSSM one-loop betas `(33/5, 1, −3)`, GUT-normalized U(1).
const MSSM_ONE_LOOP_B: [Ratio; 3] = [Ratio::new(33, 5), Ratio::int(1), Ratio::int(-3)];

/// Recorded PDG `α_em⁻¹(M_Z) = 127.951` as a Ratio. This is the Gaussian
/// centre of the sourced PDG 2022 listing, not a certificate of the `f64`
/// bits.
fn inverse_alpha_em_mz_ratio() -> Ratio {
    pdg_2022_inv_alpha_em_mz()
        .gaussian_mu()
        .expect("PDG 2022 α_em^{-1} is a Gaussian")
}

/// Recorded PDG `α_s(M_Z) = 0.1179` as a Ratio.
fn strong_coupling_mz_ratio() -> Ratio {
    pdg_2022_alpha_s_mz()
        .gaussian_mu()
        .expect("PDG 2022 α_s is a Gaussian")
}

/// Recorded PDG 2024 MS-bar `sin²θ_W(M_Z) = 0.23122` as a Ratio.
fn weak_mixing_angle_sin2_mz_ratio() -> Ratio {
    pdg_2024_sin2theta()
        .gaussian_mu()
        .expect("PDG 2024 sin2theta is a Gaussian")
}

/// A running of the three SM gauge couplings from `M_Z` (one- and two-loop).
#[derive(Clone, Copy, Debug)]
pub struct GaugeRunning {
    /// One-loop beta coefficients `(b_1, b_2, b_3)`, GUT-normalized U(1).
    pub b: [f64; 3],
    /// Two-loop (gauge) beta matrix `b_ij`, GUT-normalized.
    pub b2: [[f64; 3]; 3],
    /// Inverse couplings at `M_Z`: `[α_1⁻¹, α_2⁻¹, α_3⁻¹]`.
    pub inv_alpha_mz: [f64; 3],
}

impl GaugeRunning {
    /// Inverse couplings at `M_Z` computed from the measured electroweak inputs.
    ///
    /// `α_2⁻¹ = sin²θ_W · α_em⁻¹`, `α_1⁻¹ = (3/5)·cos²θ_W · α_em⁻¹` (GUT
    /// normalization `α_1 = 5/3·α_Y`), and `α_3⁻¹ = 1/α_s`.
    fn measured_inv_alpha_mz() -> [f64; 3] {
        let inv_em = inverse_alpha_em_mz().value();
        let s2 = weak_mixing_angle_sin2_mz().value();
        let c2 = 1.0 - s2;
        let a1 = 0.6 * c2 * inv_em;
        let a2 = s2 * inv_em;
        let a3 = 1.0 / strong_coupling_mz().value();
        [a1, a2, a3]
    }

    /// The minimal Standard Model: one-loop `b = (41/10, −19/6, −7)` and the
    /// standard two-loop gauge matrix (GUT-normalized).
    pub fn standard_model() -> Self {
        Self {
            b: [
                SM_ONE_LOOP_B[0].to_f64(),
                SM_ONE_LOOP_B[1].to_f64(),
                SM_ONE_LOOP_B[2].to_f64(),
            ],
            b2: [
                [199.0 / 50.0, 27.0 / 10.0, 44.0 / 5.0],
                [9.0 / 10.0, 35.0 / 6.0, 12.0],
                [11.0 / 10.0, 9.0 / 2.0, -26.0],
            ],
            inv_alpha_mz: Self::measured_inv_alpha_mz(),
        }
    }

    /// The MSSM (supersymmetric SM): one-loop `b = (33/5, 1, −3)` and the
    /// standard two-loop gauge matrix (GUT-normalized).
    pub fn mssm() -> Self {
        Self {
            b: [
                MSSM_ONE_LOOP_B[0].to_f64(),
                MSSM_ONE_LOOP_B[1].to_f64(),
                MSSM_ONE_LOOP_B[2].to_f64(),
            ],
            b2: [
                [199.0 / 25.0, 27.0 / 5.0, 88.0 / 5.0],
                [9.0 / 5.0, 25.0, 24.0],
                [11.0 / 5.0, 9.0, 14.0],
            ],
            inv_alpha_mz: Self::measured_inv_alpha_mz(),
        }
    }

    /// `t = ln(μ/M_Z)` where the two electroweak lines (`α_1`, `α_2`) cross.
    pub fn unification_log(&self) -> f64 {
        2.0 * PI * (self.inv_alpha_mz[0] - self.inv_alpha_mz[1]) / (self.b[0] - self.b[1])
    }

    /// Inverse coupling of line `i` at running parameter `t`.
    fn inv_alpha_at(&self, i: usize, t: f64) -> f64 {
        self.inv_alpha_mz[i] - self.b[i] / (2.0 * PI) * t
    }

    /// The unified inverse coupling `α_GUT⁻¹`, from the `α_1`/`α_2` crossing.
    pub fn alpha_gut_inv(&self) -> f64 {
        self.inv_alpha_at(0, self.unification_log())
    }

    /// The unification scale `M_GUT` in GeV, `M_Z · exp(t*)`.
    pub fn unification_scale_gev(&self) -> f64 {
        z_mass_gev().value() * self.unification_log().exp()
    }

    /// Dimension-6 proton lifetime in units of `10^31` years.
    ///
    /// `τ / 10^31 yr = (M_GUT / 10^14 GeV)^4`. This is the Georgi–Glashow
    /// order-of-magnitude estimate, normalized so `10^14 GeV` yields the
    /// textbook `10^31 yr`. It is not a lattice matrix element and not a
    /// dimension-5 operator.
    pub fn dim6_proton_lifetime_units(&self) -> f64 {
        let x = self.unification_scale_gev() / 1.0e14;
        x * x * x * x
    }

    /// Predict `α_3(M_Z)` by demanding line 3 pass through the `α_1`/`α_2`
    /// crossing, then running it back down to `M_Z`.
    pub fn predicted_alpha3_mz(&self) -> f64 {
        let t = self.unification_log();
        // Force line 3 through (t*, α_GUT⁻¹): α_3⁻¹(M_Z) = α_GUT⁻¹ + (b_3/2π)·t*.
        let inv_a3_pred = self.alpha_gut_inv() + self.b[2] / (2.0 * PI) * t;
        1.0 / inv_a3_pred
    }

    /// The measured `α_3(M_Z)` for comparison.
    pub fn measured_alpha3_mz(&self) -> f64 {
        1.0 / self.inv_alpha_mz[2]
    }

    /// Fractional disagreement between the predicted and measured `α_3(M_Z)`.
    /// Small means the three couplings (nearly) meet at one point.
    pub fn unification_mismatch(&self) -> f64 {
        (self.predicted_alpha3_mz() / self.measured_alpha3_mz() - 1.0).abs()
    }

    /// One-loop `α_3(M_Z)` from `α_em⁻¹` and `sin²θ_W` as an exact Ratio.
    ///
    /// `2π` cancels: `α_i⁻¹(M_Z) = α_U⁻¹ + (b_i/2π) t` with `t ∝ 2π`
    /// from the `α_1`/`α_2` crossing, so the predicted strong coupling
    /// is a rational function of recorded `α_em⁻¹`, `sin²θ_W`, and the
    /// one-loop betas. This does **not** use the measured `α_s`. That
    /// is the comparison datum. Complementary to
    /// [`Self::predicted_sin2_mz_exact`], which uses `α_s` and does
    /// not use the mixing angle. Not P3N: the inputs are recorded PDG
    /// decimals, not a remainder certificate.
    pub fn predicted_alpha3_mz_exact(&self) -> Ratio {
        let [b1, b2, b3] = self.one_loop_betas_ratio();
        let inv_em = inverse_alpha_em_mz_ratio();
        let s2 = weak_mixing_angle_sin2_mz_ratio();
        let c2 = Ratio::int(1) - s2;
        let a1 = Ratio::new(3, 5) * c2 * inv_em;
        let a2 = s2 * inv_em;
        let inv_a3 = a1 - ((b1 - b3) / (b1 - b2)) * (a1 - a2);
        Ratio::int(1) / inv_a3
    }

    /// One-loop `α_3(M_Z)` as an interval, propagating the sourced
    /// PDG 2024 one-sigma hull of `sin²θ_W` and the PDG 2022 hull of
    /// `α_em⁻¹` through the exact rational function. This is input
    /// uncertainty, not a two-loop remainder certificate, and not
    /// P3N. Complementary to [`Self::predicted_sin2_mz_interval`].
    pub fn predicted_alpha3_mz_interval(&self) -> Interval {
        let [b1, b2, b3] = self.one_loop_betas_ratio();
        let inv_em = pdg_2022_inv_alpha_em_mz().statistical;
        let s2 = pdg_2024_sin2theta().statistical;
        let one = Interval::point(Ratio::int(1));
        let three_fifth = Interval::point(Ratio::new(3, 5));
        let c2 = one - s2;
        let a1 = three_fifth * c2 * inv_em;
        let a2 = s2 * inv_em;
        let b1i = Interval::point(b1);
        let b2i = Interval::point(b2);
        let b3i = Interval::point(b3);
        let inv_a3 = a1 - ((b1i - b3i) / (b1i - b2i)) * (a1 - a2);
        Interval::point(Ratio::int(1)) / inv_a3
    }

    /// One-loop `α_em⁻¹(M_Z)` from `α_s` and `sin²θ_W` as an exact Ratio.
    ///
    /// Inverse of [`Self::predicted_alpha3_mz_exact`]: the same
    /// π-free algebra with the comparison datum swapped. This does
    /// **not** use the measured `α_em⁻¹`. That is the comparison
    /// datum. Complementary to GQW (predicts the mixing angle) and
    /// coupling unification (predicts `α_3`). Not P3N: the inputs
    /// are recorded PDG decimals, not a remainder certificate.
    pub fn predicted_inv_alpha_em_mz_exact(&self) -> Ratio {
        let [b1, b2, b3] = self.one_loop_betas_ratio();
        let alpha_s = strong_coupling_mz_ratio();
        let s2 = weak_mixing_angle_sin2_mz_ratio();
        let c2 = Ratio::int(1) - s2;
        let r = (b1 - b3) / (b1 - b2);
        let three_fifth = Ratio::new(3, 5);
        let coeff = three_fifth * c2 - r * (three_fifth * c2 - s2);
        (Ratio::int(1) / alpha_s) / coeff
    }

    /// One-loop `α_em⁻¹(M_Z)` as an interval, propagating the sourced
    /// PDG 2022 one-sigma hull of `α_s` and the PDG 2024 hull of
    /// `sin²θ_W` through the exact rational function. This is input
    /// uncertainty, not a two-loop remainder certificate, and not
    /// P3N. Complementary to [`Self::predicted_alpha3_mz_interval`].
    pub fn predicted_inv_alpha_em_mz_interval(&self) -> Interval {
        let [b1, b2, b3] = self.one_loop_betas_ratio();
        let alpha_s = pdg_2022_alpha_s_mz().statistical;
        let s2 = pdg_2024_sin2theta().statistical;
        let one = Interval::point(Ratio::int(1));
        let three_fifth = Interval::point(Ratio::new(3, 5));
        let c2 = one - s2;
        let b1i = Interval::point(b1);
        let b2i = Interval::point(b2);
        let b3i = Interval::point(b3);
        let r = (b1i - b3i) / (b1i - b2i);
        let coeff = three_fifth * c2 - r * (three_fifth * c2 - s2);
        (one / alpha_s) / coeff
    }

    /// Measured `sin²θ_W(M_Z)` (PDG MS-bar). Input to `α_1`/`α_2`; *not* used
    /// by [`Self::predicted_sin2_mz`].
    pub fn measured_sin2_mz(&self) -> f64 {
        weak_mixing_angle_sin2_mz().value()
    }

    fn one_loop_betas_ratio(&self) -> [Ratio; 3] {
        if (self.b[2] - SM_ONE_LOOP_B[2].to_f64()).abs() < 1e-12 {
            SM_ONE_LOOP_B
        } else if (self.b[2] - MSSM_ONE_LOOP_B[2].to_f64()).abs() < 1e-12 {
            MSSM_ONE_LOOP_B
        } else {
            panic!(
                "GQW exact centre needs SM or MSSM one-loop betas, got b3={}",
                self.b[2]
            )
        }
    }

    /// `t = ln(M_U/M_Z)` implied by `α_em` + `α_s` under one-loop unification.
    fn gqw_log(&self) -> f64 {
        let inv_em = inverse_alpha_em_mz().value();
        let inv_s = self.inv_alpha_mz[2];
        let [b1, b2, b3] = self.b;
        let denom = (5.0 / 3.0) * (b1 - b3) + (b2 - b3);
        2.0 * PI * (inv_em - (8.0 / 3.0) * inv_s) / denom
    }

    /// Georgi–Quinn–Weinberg `sin²θ_W(M_Z)` as an exact Ratio.
    ///
    /// `2π` cancels: `α_i⁻¹(M_Z) = α_U⁻¹ + (b_i/2π) t` with
    /// `t ∝ 2π`, so the mixing angle is a rational function of
    /// recorded `α_em⁻¹`, `α_s`, and the one-loop betas. This does
    /// **not** use the measured mixing angle. At unification the
    /// same algebra is identically `3/8`; here it is the low-energy
    /// value after running. Not P3N: the inputs are recorded PDG
    /// decimals, not a remainder certificate.
    pub fn predicted_sin2_mz_exact(&self) -> Ratio {
        let [b1, b2, b3] = self.one_loop_betas_ratio();
        let inv_em = inverse_alpha_em_mz_ratio();
        let inv_s = Ratio::int(1) / strong_coupling_mz_ratio();
        let five_thirds = Ratio::new(5, 3);
        let eight_thirds = Ratio::new(8, 3);
        let denom = five_thirds * (b1 - b3) + (b2 - b3);
        let num = denom * inv_s + (b2 - b3) * (inv_em - eight_thirds * inv_s);
        num / (denom * inv_em)
    }

    /// One-loop GQW `sin²θ_W(M_Z)` as an interval, propagating the sourced
    /// PDG 2022 one-sigma hulls of `α_em⁻¹` and `α_s` through the exact
    /// rational function. This is input uncertainty, not a two-loop
    /// remainder certificate, and not P3N.
    pub fn predicted_sin2_mz_interval(&self) -> Interval {
        let [b1, b2, b3] = self.one_loop_betas_ratio();
        let inv_em = pdg_2022_inv_alpha_em_mz().statistical;
        let alpha_s = pdg_2022_alpha_s_mz().statistical;
        let inv_s = Interval::point(Ratio::int(1)) / alpha_s;
        let five_thirds = Interval::point(Ratio::new(5, 3));
        let eight_thirds = Interval::point(Ratio::new(8, 3));
        let b1i = Interval::point(b1);
        let b2i = Interval::point(b2);
        let b3i = Interval::point(b3);
        let denom = five_thirds * (b1i - b3i) + (b2i - b3i);
        let num = denom * inv_s + (b2i - b3i) * (inv_em - eight_thirds * inv_s);
        num / (denom * inv_em)
    }

    /// IEEE-754 view of [`Self::predicted_sin2_mz_exact`]. Not a
    /// certificate of the float; use the Ratio for threshold cells.
    pub fn predicted_sin2_mz(&self) -> f64 {
        self.predicted_sin2_mz_exact().to_f64()
    }

    /// Unification scale implied by `α_em` + `α_s` (GQW), in GeV.
    pub fn gqw_unification_scale_gev(&self) -> f64 {
        z_mass_gev().value() * self.gqw_log().exp()
    }

    /// Fractional disagreement between GQW-predicted and measured `sin²θ_W(M_Z)`.
    pub fn sin2_mismatch(&self) -> f64 {
        (self.predicted_sin2_mz() / self.measured_sin2_mz() - 1.0).abs()
    }

    /// Two-loop RGE derivative `d(α_i⁻¹)/dt` at inverse couplings `y`:
    /// `−b_i/2π − (1/8π²) Σ_j b_ij α_j`, with `α_j = 1/y_j`.
    fn two_loop_deriv(&self, y: [f64; 3]) -> [f64; 3] {
        let mut d = [0.0; 3];
        for (i, di) in d.iter_mut().enumerate() {
            let two_loop: f64 = (0..3).map(|j| self.b2[i][j] / y[j]).sum();
            *di = -self.b[i] / (2.0 * PI) - two_loop / (8.0 * PI * PI);
        }
        d
    }

    /// One RK4 step of size `h` on the inverse-coupling vector.
    fn rk4_step(&self, y: [f64; 3], h: f64) -> [f64; 3] {
        let add =
            |a: [f64; 3], b: [f64; 3], s: f64| [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s];
        let k1 = self.two_loop_deriv(y);
        let k2 = self.two_loop_deriv(add(y, k1, h / 2.0));
        let k3 = self.two_loop_deriv(add(y, k2, h / 2.0));
        let k4 = self.two_loop_deriv(add(y, k3, h));
        [
            y[0] + h / 6.0 * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]),
            y[1] + h / 6.0 * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]),
            y[2] + h / 6.0 * (k1[2] + 2.0 * k2[2] + 2.0 * k3[2] + k4[2]),
        ]
    }

    /// Integrate the two-loop RGEs up from `M_Z`, find the scale where `α_1⁻¹`
    /// and `α_2⁻¹` cross, and return `(ln(M_GUT/M_Z), α_GUT⁻¹, α_3⁻¹ there)`.
    fn two_loop_crossing(&self) -> (f64, f64, f64) {
        let h = 0.01;
        let mut y = self.inv_alpha_mz;
        let mut t = 0.0;
        for _ in 0..6000 {
            let y_next = self.rk4_step(y, h);
            let (d_now, d_next) = (y[0] - y[1], y_next[0] - y_next[1]);
            if d_now.signum() != d_next.signum() && d_now != 0.0 {
                // Linear interpolation to the crossing within this step.
                let frac = d_now / (d_now - d_next);
                let interp = |a: f64, b: f64| a + (b - a) * frac;
                let a1 = interp(y[0], y_next[0]);
                let a3 = interp(y[2], y_next[2]);
                return (t + h * frac, a1, a3);
            }
            y = y_next;
            t += h;
        }
        (t, y[0], y[2]) // no crossing found within range
    }

    /// Two-loop unification scale `M_GUT` in GeV.
    pub fn two_loop_unification_scale_gev(&self) -> f64 {
        z_mass_gev().value() * self.two_loop_crossing().0.exp()
    }

    /// Two-loop unification mismatch: the fractional gap between `α_3⁻¹` and the
    /// `α_1⁻¹ = α_2⁻¹` meeting value, at the two-loop crossing scale. Small means
    /// all three couplings meet there.
    pub fn two_loop_unification_mismatch(&self) -> f64 {
        let (_, a12, a3) = self.two_loop_crossing();
        (a3 - a12).abs() / a12.abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_loop_mssm_unifies_far_better_than_the_sm() {
        let sm = GaugeRunning::standard_model();
        let mssm = GaugeRunning::mssm();
        // Two-loop RK4 integration: SM misses, MSSM unifies to a few percent.
        let sm_m = sm.two_loop_unification_mismatch();
        let mssm_m = mssm.two_loop_unification_mismatch();
        assert!(sm_m > 0.08, "SM two-loop mismatch = {sm_m}");
        assert!(mssm_m < 0.05, "MSSM two-loop mismatch = {mssm_m}");
        assert!(mssm_m < sm_m / 2.0, "SUSY should unify much better");
        // Two-loop MSSM scale is the phenomenological ~2–3×10^16 GeV; the SM's is
        // far too low (~10^13 GeV), the reason its proton decay is excluded.
        let mssm_gut = mssm.two_loop_unification_scale_gev();
        assert!(
            (1e16..1e17).contains(&mssm_gut),
            "MSSM M_GUT = {mssm_gut:.2e}"
        );
        assert!((1e12..1e14).contains(&sm.two_loop_unification_scale_gev()));
    }

    #[test]
    fn two_loop_refines_one_loop() {
        // The two-loop scale differs measurably from the one-loop estimate.
        let mssm = GaugeRunning::mssm();
        let one = mssm.unification_scale_gev();
        let two = mssm.two_loop_unification_scale_gev();
        assert!(
            (one / two - 1.0).abs() > 0.05,
            "one={one:.2e}, two={two:.2e}"
        );
    }

    #[test]
    fn standard_model_misses_unification() {
        let sm = GaugeRunning::standard_model();
        // The SM predicts α_3(M_Z) tens of percent away from the measured value.
        assert!(
            sm.unification_mismatch() > 0.2,
            "SM mismatch = {}",
            sm.unification_mismatch()
        );
        // Its low unification scale (~10^13 GeV) is why proton decay is too fast.
        let m_gut = sm.unification_scale_gev();
        assert!((1e12..1e14).contains(&m_gut), "SM M_GUT = {m_gut:.2e} GeV");
        let tau = sm.dim6_proton_lifetime_units();
        assert!(
            tau < 2400.0,
            "minimal SU(5) dim-6 lifetime must sit below Super-K: {tau:.3e} × 10^31 yr"
        );
    }

    #[test]
    fn mssm_unifies_to_about_a_percent() {
        let mssm = GaugeRunning::mssm();
        // The celebrated near-success: predicted α_3 matches to a few percent.
        assert!(
            mssm.unification_mismatch() < 0.03,
            "MSSM mismatch = {}",
            mssm.unification_mismatch()
        );
        // With a unification scale near 2×10^16 GeV.
        let m_gut = mssm.unification_scale_gev();
        assert!(
            (5e15..1e17).contains(&m_gut),
            "MSSM M_GUT = {m_gut:.2e} GeV"
        );
        let tau = mssm.dim6_proton_lifetime_units();
        assert!(
            tau > 2400.0,
            "MSSM dim-6 lifetime must sit above Super-K: {tau:.3e} × 10^31 yr"
        );
    }

    #[test]
    fn susy_beats_the_sm_at_unification() {
        // The whole point: SUSY unification is dramatically better than the SM.
        let sm = GaugeRunning::standard_model();
        let mssm = GaugeRunning::mssm();
        assert!(mssm.unification_mismatch() < sm.unification_mismatch() / 5.0);
    }

    #[test]
    fn gqw_sin2_misses_in_the_sm_and_hits_in_the_mssm() {
        // Georgi–Quinn–Weinberg: α_em + α_s predict sin²θ_W(M_Z), without
        // using the measured mixing angle. Minimal SU(5) lands near 0.21;
        // the MSSM lands on 0.231.
        let sm = GaugeRunning::standard_model();
        let mssm = GaugeRunning::mssm();
        let measured = sm.measured_sin2_mz();
        assert!(
            (measured - 0.231_21).abs() < 1e-6,
            "PDG input drifted: {measured}"
        );
        let sm_pred = sm.predicted_sin2_mz();
        let mssm_pred = mssm.predicted_sin2_mz();
        assert!(
            sm_pred < 0.22 && sm_pred > 0.20,
            "SM GQW sin²θ_W = {sm_pred}"
        );
        assert!(
            (mssm_pred - measured).abs() < 0.005,
            "MSSM GQW sin²θ_W = {mssm_pred} vs {measured}"
        );
        assert!(sm.sin2_mismatch() > 0.05);
        assert!(mssm.sin2_mismatch() < 0.03);
        // The GQW scale for the MSSM is the phenomenological ~2×10^16 GeV.
        let mssm_u = mssm.gqw_unification_scale_gev();
        assert!(
            (1e16..1e17).contains(&mssm_u),
            "MSSM GQW M_U = {mssm_u:.2e}"
        );
        // And it is *not* the tautological 3/8: that is the GUT-scale value.
        assert!((sm_pred - 0.375).abs() > 0.1);
        assert!((mssm_pred - 0.375).abs() > 0.1);
    }

    #[test]
    fn gqw_sin2_is_an_exact_pi_free_ratio() {
        assert!(
            (inverse_alpha_em_mz_ratio().to_f64() - inverse_alpha_em_mz().value()).abs() < 1e-12
        );
        assert!((strong_coupling_mz_ratio().to_f64() - strong_coupling_mz().value()).abs() < 1e-12);
        let sm = GaugeRunning::standard_model();
        let mssm = GaugeRunning::mssm();
        let sm_exact = sm.predicted_sin2_mz_exact();
        let mssm_exact = mssm.predicted_sin2_mz_exact();
        assert_eq!(sm_exact, Ratio::new(12588941801, 60643400058));
        assert_eq!(mssm_exact, Ratio::new(522562687, 2262813435));
        assert_ne!(sm_exact, Ratio::new(3, 8));
        assert_ne!(mssm_exact, Ratio::new(3, 8));
        assert_eq!(sm_exact.round_to(100_000), Ratio::new(20759, 100000));
        assert_eq!(mssm_exact.round_to(100_000), Ratio::new(23093, 100000));
        // The closed form must not need 2π: rebuild it here from the same
        // recorded decimals and betas.
        let inv_em = inverse_alpha_em_mz_ratio();
        let inv_s = Ratio::int(1) / strong_coupling_mz_ratio();
        let [b1, b2, b3] = SM_ONE_LOOP_B;
        let denom = Ratio::new(5, 3) * (b1 - b3) + (b2 - b3);
        let rebuilt =
            (denom * inv_s + (b2 - b3) * (inv_em - Ratio::new(8, 3) * inv_s)) / (denom * inv_em);
        assert_eq!(rebuilt, sm_exact);
        assert!((sm_exact.to_f64() - sm.predicted_sin2_mz()).abs() < 1e-15);
    }

    #[test]
    fn gqw_input_interval_contains_the_centre_and_misses_the_pdg_hull() {
        let sm = GaugeRunning::standard_model();
        let mssm = GaugeRunning::mssm();
        let sm_i = sm.predicted_sin2_mz_interval();
        let mssm_i = mssm.predicted_sin2_mz_interval();
        assert!(sm_i.contains(sm.predicted_sin2_mz_exact().enclosure()));
        assert!(mssm_i.contains(mssm.predicted_sin2_mz_exact().enclosure()));
        let pdg = Interval::new(Ratio::new(23121, 100000), Ratio::new(23123, 100000));
        assert!(
            sm_i.disjoint(pdg),
            "SU(5) input interval {sm_i} vs PDG {pdg}"
        );
        assert_eq!(
            Interval::parse_display(&sm_i.to_string()),
            Some(sm_i),
            "GQW input interval Display must independently parse"
        );
        assert_eq!(Interval::parse_display(&mssm_i.to_string()), Some(mssm_i));
        assert!(
            !mssm_i.disjoint(pdg) && !pdg.contains(mssm_i),
            "one-loop MSSM with sourced PDG input σ overlaps 10^-5 but is not contained: {mssm_i} vs {pdg}"
        );
        assert!(!sm_i.contains(pdg) && !pdg.contains(sm_i));
        // The 3% folklore band is not this enclosure.
        let folklore = sm
            .predicted_sin2_mz_exact()
            .enclosure()
            .relative_envelope(Ratio::new(3, 100));
        assert!(folklore.contains(sm_i));
        assert!(!sm_i.contains(folklore));
    }

    #[test]
    fn coupling_unification_alpha3_is_an_exact_pi_free_ratio() {
        let sm = GaugeRunning::standard_model();
        let mssm = GaugeRunning::mssm();
        let sm_exact = sm.predicted_alpha3_mz_exact();
        let mssm_exact = mssm.predicted_alpha3_mz_exact();
        assert_eq!(sm_exact, Ratio::new(5450000000, 76612068711));
        assert_eq!(mssm_exact, Ratio::new(10000000, 85599219));
        assert_eq!(sm_exact.round_to(10_000), Ratio::new(711, 10_000));
        assert_eq!(mssm_exact.round_to(10_000), Ratio::new(1168, 10_000));
        let inv_em = inverse_alpha_em_mz_ratio();
        let s2 = weak_mixing_angle_sin2_mz_ratio();
        let c2 = Ratio::int(1) - s2;
        let [b1, b2, b3] = SM_ONE_LOOP_B;
        let a1 = Ratio::new(3, 5) * c2 * inv_em;
        let a2 = s2 * inv_em;
        let rebuilt = Ratio::int(1) / (a1 - ((b1 - b3) / (b1 - b2)) * (a1 - a2));
        assert_eq!(rebuilt, sm_exact);
        let als = pdg_2022_alpha_s_mz().statistical;
        assert!(als.disjoint(sm_exact.enclosure()));
        assert!(
            als.disjoint(mssm_exact.enclosure()),
            "one-loop MSSM α_3 centre sits below the PDG 1σ hull; overlap is the input-interval cell"
        );
    }

    #[test]
    fn coupling_unification_input_interval_contains_the_centre_and_misses_the_pdg_hull() {
        let sm = GaugeRunning::standard_model();
        let mssm = GaugeRunning::mssm();
        let sm_i = sm.predicted_alpha3_mz_interval();
        let mssm_i = mssm.predicted_alpha3_mz_interval();
        assert!(sm_i.contains(sm.predicted_alpha3_mz_exact().enclosure()));
        assert!(mssm_i.contains(mssm.predicted_alpha3_mz_exact().enclosure()));
        let pdg = pdg_2022_alpha_s_mz().statistical;
        assert!(
            sm_i.disjoint(pdg),
            "SU(5) input interval {sm_i} vs PDG α_s {pdg}"
        );
        assert_eq!(
            Interval::parse_display(&sm_i.to_string()),
            Some(sm_i),
            "coupling-unification input interval Display must independently parse"
        );
        assert_eq!(Interval::parse_display(&mssm_i.to_string()), Some(mssm_i));
        assert!(
            !mssm_i.disjoint(pdg) && !pdg.contains(mssm_i),
            "one-loop MSSM with sourced PDG input σ overlaps α_s but is not contained: {mssm_i} vs {pdg}"
        );
        assert!(!sm_i.contains(pdg) && !pdg.contains(sm_i));
        let folklore = sm
            .predicted_alpha3_mz_exact()
            .enclosure()
            .relative_envelope(Ratio::new(3, 100));
        assert!(folklore.contains(sm_i));
        assert!(!sm_i.contains(folklore));
    }

    #[test]
    fn inverse_alpha_em_is_an_exact_pi_free_ratio() {
        let sm = GaugeRunning::standard_model();
        let mssm = GaugeRunning::mssm();
        let sm_exact = sm.predicted_inv_alpha_em_mz_exact();
        let mssm_exact = mssm.predicted_inv_alpha_em_mz_exact();
        assert_eq!(sm_exact, Ratio::new(54500000000, 705939219));
        assert_eq!(mssm_exact, Ratio::new(100000000, 788751));
        assert_eq!(sm_exact.round_to(1000), Ratio::new(38601, 500));
        assert_eq!(mssm_exact.round_to(1000), Ratio::new(126783, 1000));
        let alpha_s = strong_coupling_mz_ratio();
        let s2 = weak_mixing_angle_sin2_mz_ratio();
        let c2 = Ratio::int(1) - s2;
        let [b1, b2, b3] = SM_ONE_LOOP_B;
        let r = (b1 - b3) / (b1 - b2);
        let three_fifth = Ratio::new(3, 5);
        let coeff = three_fifth * c2 - r * (three_fifth * c2 - s2);
        let rebuilt = (Ratio::int(1) / alpha_s) / coeff;
        assert_eq!(rebuilt, sm_exact);
        let pdg = pdg_2022_inv_alpha_em_mz().statistical;
        assert!(pdg.disjoint(sm_exact.enclosure()));
        assert!(
            pdg.disjoint(mssm_exact.enclosure()),
            "one-loop MSSM inv-alpha-em centre sits below the PDG 1σ hull; the input-interval cell is also disjoint"
        );
    }

    #[test]
    fn inverse_alpha_em_input_interval_contains_the_centre_and_misses_the_pdg_hull() {
        let sm = GaugeRunning::standard_model();
        let mssm = GaugeRunning::mssm();
        let sm_i = sm.predicted_inv_alpha_em_mz_interval();
        let mssm_i = mssm.predicted_inv_alpha_em_mz_interval();
        assert!(sm_i.contains(sm.predicted_inv_alpha_em_mz_exact().enclosure()));
        assert!(mssm_i.contains(mssm.predicted_inv_alpha_em_mz_exact().enclosure()));
        let pdg = pdg_2022_inv_alpha_em_mz().statistical;
        assert!(
            sm_i.disjoint(pdg),
            "SU(5) input interval {sm_i} vs PDG inv-alpha-em {pdg}"
        );
        assert_eq!(
            Interval::parse_display(&sm_i.to_string()),
            Some(sm_i),
            "inverse-alpha input interval Display must independently parse"
        );
        assert_eq!(Interval::parse_display(&mssm_i.to_string()), Some(mssm_i));
        assert!(
            mssm_i.disjoint(pdg),
            "one-loop MSSM with sourced PDG input σ is still disjoint from the tight inv-alpha-em hull: {mssm_i} vs {pdg}"
        );
        assert!(!sm_i.contains(pdg) && !pdg.contains(sm_i));
        let folklore = sm
            .predicted_inv_alpha_em_mz_exact()
            .enclosure()
            .relative_envelope(Ratio::new(3, 100));
        assert!(folklore.contains(sm_i));
        assert!(!sm_i.contains(folklore));
    }
}
