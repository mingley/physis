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
//! One loop is an approximation: two-loop terms and SUSY thresholds shift the
//! numbers at the percent level. The verdicts that consume this are therefore
//! tagged `Heuristic`, with the computed numbers as evidence.

use std::f64::consts::PI;

use physis_model::constants::{
    inverse_alpha_em_mz, strong_coupling_mz, weak_mixing_angle_sin2_mz, z_mass_gev,
};

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
            b: [41.0 / 10.0, -19.0 / 6.0, -7.0],
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
            b: [33.0 / 5.0, 1.0, -3.0],
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
    }

    #[test]
    fn susy_beats_the_sm_at_unification() {
        // The whole point: SUSY unification is dramatically better than the SM.
        let sm = GaugeRunning::standard_model();
        let mssm = GaugeRunning::mssm();
        assert!(mssm.unification_mismatch() < sm.unification_mismatch() / 5.0);
    }
}
