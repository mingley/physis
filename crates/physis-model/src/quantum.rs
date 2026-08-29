//! Finite-dimensional Hilbert space, kets, Born rule, Pauli operators.
//!
//! This is the *smallest honest quantum layer*: amplitudes, inner products,
//! and two-level systems. QFT continuum limits are a later milestone.

use physis_core::id::LayerId;
use physis_core::layer::Layer;

use crate::complex::Complex;

/// A finite Hilbert space of dimension `n`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hilbert {
    /// Complex dimension.
    pub dim: usize,
}

/// A ket `|ψ⟩` as a column of amplitudes.
#[derive(Clone, Debug, PartialEq)]
pub struct Ket {
    /// Amplitudes in the computational basis.
    pub amps: Vec<Complex>,
}

impl Ket {
    /// Computational-basis state `|i⟩`.
    pub fn basis(dim: usize, i: usize) -> Option<Self> {
        if i >= dim {
            return None;
        }
        let mut amps = vec![Complex::ZERO; dim];
        amps[i] = Complex::ONE;
        Some(Self { amps })
    }

    /// Dimension.
    pub fn dim(&self) -> usize {
        self.amps.len()
    }

    /// Inner product `⟨self|other⟩`.
    pub fn inner(&self, other: &Self) -> Option<Complex> {
        if self.dim() != other.dim() {
            return None;
        }
        let mut acc = Complex::ZERO;
        for (a, b) in self.amps.iter().zip(other.amps.iter()) {
            acc = acc + a.conj() * *b;
        }
        Some(acc)
    }

    /// `⟨ψ|ψ⟩`.
    pub fn norm_sqr(&self) -> f64 {
        self.amps.iter().map(|a| a.norm_sqr()).sum()
    }

    /// Normalize in place. Returns false if the ket is ~0.
    pub fn normalize(&mut self) -> bool {
        let n = self.norm_sqr().sqrt();
        if n < 1e-18 {
            return false;
        }
        for a in &mut self.amps {
            *a = *a * (1.0 / n);
        }
        true
    }

    /// Born probability of basis outcome `i`.
    pub fn born(&self, i: usize) -> Option<f64> {
        let n = self.norm_sqr();
        if n < 1e-18 || i >= self.dim() {
            return None;
        }
        Some(self.amps[i].norm_sqr() / n)
    }
}

/// 2×2 complex matrix (row-major).
pub type Mat2 = [[Complex; 2]; 2];

/// Apply a 2×2 operator to a qubit ket.
pub fn apply_mat2(op: Mat2, ket: &Ket) -> Option<Ket> {
    if ket.dim() != 2 {
        return None;
    }
    Some(Ket {
        amps: vec![
            op[0][0] * ket.amps[0] + op[0][1] * ket.amps[1],
            op[1][0] * ket.amps[0] + op[1][1] * ket.amps[1],
        ],
    })
}

/// Pauli σ_x.
pub fn pauli_x() -> Mat2 {
    [[Complex::ZERO, Complex::ONE], [Complex::ONE, Complex::ZERO]]
}

/// Pauli σ_y.
pub fn pauli_y() -> Mat2 {
    [
        [Complex::ZERO, Complex::new(0.0, -1.0)],
        [Complex::new(0.0, 1.0), Complex::ZERO],
    ]
}

/// Pauli σ_z.
pub fn pauli_z() -> Mat2 {
    [
        [Complex::ONE, Complex::ZERO],
        [Complex::ZERO, Complex::from_re(-1.0)],
    ]
}

/// 4×4 complex matrix (row-major): an operator on two qubits.
pub type Mat4 = [[Complex; 4]; 4];

/// Spin measurement operator along angle `θ` in the x–z plane,
/// `cos θ · σ_z + sin θ · σ_x`. Hermitian, with eigenvalues ±1.
pub fn spin_measurement(theta: f64) -> Mat2 {
    let (c, s) = (theta.cos(), theta.sin());
    [
        [Complex::from_re(c), Complex::from_re(s)],
        [Complex::from_re(s), Complex::from_re(-c)],
    ]
}

/// Kronecker (tensor) product `A ⊗ B` of two single-qubit operators, as a 4×4
/// operator on the two-qubit space in basis order `|00⟩, |01⟩, |10⟩, |11⟩`.
pub fn tensor2(a: Mat2, b: Mat2) -> Mat4 {
    let mut out = [[Complex::ZERO; 4]; 4];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                for l in 0..2 {
                    out[2 * i + k][2 * j + l] = a[i][j] * b[k][l];
                }
            }
        }
    }
    out
}

/// Apply a 4×4 operator to a two-qubit ket.
pub fn apply_mat4(op: &Mat4, ket: &Ket) -> Option<Ket> {
    if ket.dim() != 4 {
        return None;
    }
    let mut amps = vec![Complex::ZERO; 4];
    for (i, amp) in amps.iter_mut().enumerate() {
        let mut acc = Complex::ZERO;
        for (j, kj) in ket.amps.iter().enumerate() {
            acc = acc + op[i][j] * *kj;
        }
        *amp = acc;
    }
    Some(Ket { amps })
}

/// Expectation value `⟨ψ|M|ψ⟩` of a two-qubit operator. For a Hermitian `M`
/// (like a tensor of spin measurements) the result is real up to rounding.
pub fn expectation4(op: &Mat4, ket: &Ket) -> Option<Complex> {
    let m = apply_mat4(op, ket)?;
    ket.inner(&m)
}

/// A two-level system used as the default quantum observable.
#[derive(Clone, Debug, PartialEq)]
pub struct Qubit {
    /// State.
    pub ket: Ket,
}

impl Qubit {
    /// `|0⟩`.
    pub fn zero() -> Self {
        Self {
            ket: Ket::basis(2, 0).expect("dim"),
        }
    }

    /// `|1⟩`.
    pub fn one() -> Self {
        Self {
            ket: Ket::basis(2, 1).expect("dim"),
        }
    }

    /// Hadamard-like equal superposition (unnormalized then normalized).
    pub fn plus() -> Self {
        let mut ket = Ket {
            amps: vec![Complex::from_re(1.0), Complex::from_re(1.0)],
        };
        ket.normalize();
        Self { ket }
    }
}

impl Layer for Qubit {
    const ID: LayerId = LayerId::Quantum;
    type Observable = Ket;
    fn observe(&self) -> Self::Observable {
        self.ket.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn born_on_basis() {
        let k = Ket::basis(2, 0).unwrap();
        assert!((k.born(0).unwrap() - 1.0).abs() < 1e-12);
        assert!(k.born(1).unwrap().abs() < 1e-12);
    }

    #[test]
    fn pauli_x_flips() {
        let k = Ket::basis(2, 0).unwrap();
        let out = apply_mat2(pauli_x(), &k).unwrap();
        assert!((out.born(1).unwrap() - 1.0).abs() < 1e-12);
    }

    #[test]
    fn plus_is_fifty_fifty() {
        let q = Qubit::plus();
        let p = q.ket.born(0).unwrap();
        assert!((p - 0.5).abs() < 1e-12);
    }

    #[test]
    fn spin_measurement_has_unit_eigenvalues() {
        // cos²+sin² = 1, so det = −1 and the eigenvalues are ±1 for any θ.
        for theta in [0.0, 0.7, 1.3, 2.5] {
            let m = spin_measurement(theta);
            let det = (m[0][0] * m[1][1] - m[0][1] * m[1][0]).re;
            assert!((det + 1.0).abs() < 1e-12, "θ={theta}");
        }
    }

    #[test]
    fn singlet_correlator_is_minus_cosine() {
        // ⟨ψ⁻|σ(a)⊗σ(b)|ψ⁻⟩ = −cos(a−b), the quantum prediction, computed here
        // straight from the operators rather than assumed.
        let s = 1.0 / 2.0_f64.sqrt();
        let singlet = Ket {
            amps: vec![
                Complex::ZERO,
                Complex::from_re(s),
                Complex::from_re(-s),
                Complex::ZERO,
            ],
        };
        for (a, b) in [(0.0, 0.4), (0.3, 1.1), (1.0, 2.0), (0.0, PI_OVER_2)] {
            let op = tensor2(spin_measurement(a), spin_measurement(b));
            let e = expectation4(&op, &singlet).unwrap();
            assert!(e.im.abs() < 1e-12, "expectation should be real");
            assert!((e.re - (-(a - b).cos())).abs() < 1e-12, "a={a}, b={b}");
        }
    }

    const PI_OVER_2: f64 = std::f64::consts::FRAC_PI_2;
}
