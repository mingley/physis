//! Discrete exterior calculus: `d² = 0`, and topology from the coboundary.
//!
//! Differential-form *grade* is carried at the Rust type level, so the compiler
//! forbids adding a 1-form to a 2-form and the exterior derivative `d` provably
//! raises grade by exactly one. On a simplicial complex the coboundary makes
//! `d ∘ d = 0` an *exact* identity — the algebra behind `curl grad = 0` and
//! behind the homogeneous Maxwell equations `dF = 0` when `F = dA`.
//!
//! The same coboundary computes topology: the first Betti number
//! `b₁ = dim ker d₁ − dim im d₀` counts holes. On a filled triangle (a disk)
//! `b₁ = 0` and every closed 1-form is exact (Poincaré lemma); remove the face
//! and the hollow triangle is a circle with `b₁ = 1`, carrying a closed form
//! that is *not* exact. A `filled` knob flips this — topology as a knob → verdict
//! diff.
//!
//! Grade is a type error to mix:
//!
//! ```compile_fail
//! use physis_theory::dec::{Cochain, G0, G1};
//! let a = Cochain::<G0>::zero(3);
//! let b = Cochain::<G1>::zero(3);
//! let _ = a + b; // different grades: does not type-check
//! ```

use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Add;

use physis_core::claim::{Claim, Epistemic, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_model::World;

use crate::framework::Theory;

/// A type-level differential-form grade.
pub trait Grade {
    /// The grade `k` as a value.
    const K: usize;
}

/// Grade 0 (functions on vertices).
#[derive(Clone, Copy, Debug)]
pub struct G0;
/// Grade 1 (forms on edges).
#[derive(Clone, Copy, Debug)]
pub struct G1;
/// Grade 2 (forms on triangles).
#[derive(Clone, Copy, Debug)]
pub struct G2;

impl Grade for G0 {
    const K: usize = 0;
}
impl Grade for G1 {
    const K: usize = 1;
}
impl Grade for G2 {
    const K: usize = 2;
}

/// A `k`-cochain: one real value per `k`-simplex, tagged with its grade so the
/// compiler tracks which space it lives in.
#[derive(Clone, Debug, PartialEq)]
pub struct Cochain<G: Grade> {
    /// Coefficients, one per `k`-simplex in the complex's fixed ordering.
    pub values: Vec<f64>,
    _grade: PhantomData<G>,
}

impl<G: Grade> Cochain<G> {
    /// A cochain from explicit coefficients.
    pub fn new(values: Vec<f64>) -> Self {
        Self {
            values,
            _grade: PhantomData,
        }
    }
    /// The zero `k`-cochain of length `n`.
    pub fn zero(n: usize) -> Self {
        Self::new(vec![0.0; n])
    }
    /// True if every coefficient is within `tol` of zero.
    pub fn is_zero(&self, tol: f64) -> bool {
        self.values.iter().all(|v| v.abs() <= tol)
    }
}

impl<G: Grade> Add for Cochain<G> {
    type Output = Cochain<G>;
    fn add(self, rhs: Cochain<G>) -> Cochain<G> {
        Cochain::new(
            self.values
                .iter()
                .zip(rhs.values.iter())
                .map(|(a, b)| a + b)
                .collect(),
        )
    }
}

/// A 2-dimensional simplicial complex (vertices, oriented edges, triangles).
#[derive(Clone, Debug)]
pub struct Complex {
    /// Number of vertices.
    pub n_vertices: usize,
    /// Oriented edges `[tail, head]` with `tail < head`.
    pub edges: Vec<[usize; 2]>,
    /// Oriented triangles `[a, b, c]` with `a < b < c`.
    pub triangles: Vec<[usize; 3]>,
    edge_index: HashMap<(usize, usize), usize>,
}

impl Complex {
    /// Build a complex, indexing edges for coboundary lookups.
    pub fn new(n_vertices: usize, edges: Vec<[usize; 2]>, triangles: Vec<[usize; 3]>) -> Self {
        let edge_index = edges
            .iter()
            .enumerate()
            .map(|(i, e)| ((e[0], e[1]), i))
            .collect();
        Self {
            n_vertices,
            edges,
            triangles,
            edge_index,
        }
    }

    /// The filled triangle: a triangulated disk (contractible, `b₁ = 0`).
    pub fn disk() -> Self {
        Self::new(3, vec![[0, 1], [0, 2], [1, 2]], vec![[0, 1, 2]])
    }

    /// The hollow triangle: a triangulated circle `S¹` (a hole, `b₁ = 1`).
    pub fn circle() -> Self {
        Self::new(3, vec![[0, 1], [0, 2], [1, 2]], vec![])
    }

    /// A triangulated torus `T²` as a 3×3 flat (periodic) grid, each square cut
    /// by its diagonal into two triangles. Closed orientable surface with
    /// `b₀ = 1`, `b₁ = 2`, `b₂ = 1`, `χ = 0` — a non-trivial homology check.
    pub fn torus() -> Self {
        let n = 3usize;
        let idx = |i: usize, j: usize| (i % n) * n + (j % n);
        let mut edge_set = std::collections::BTreeSet::new();
        let mut triangles = Vec::new();
        for i in 0..n {
            for j in 0..n {
                let (v, right, up, diag) =
                    (idx(i, j), idx(i, j + 1), idx(i + 1, j), idx(i + 1, j + 1));
                for (a, b) in [(v, right), (v, up), (v, diag)] {
                    edge_set.insert((a.min(b), a.max(b)));
                }
                // Split square (i,j) along the (v, diag) diagonal.
                let mut lower = [v, right, diag];
                lower.sort_unstable();
                let mut upper = [v, up, diag];
                upper.sort_unstable();
                triangles.push(lower);
                triangles.push(upper);
            }
        }
        let edges = edge_set.into_iter().map(|(a, b)| [a, b]).collect();
        Self::new(n * n, edges, triangles)
    }

    fn edge_of(&self, a: usize, b: usize) -> usize {
        self.edge_index[&(a.min(b), a.max(b))]
    }

    /// The exterior derivative `d₀: 0-forms → 1-forms` (discrete gradient):
    /// `(d₀f)[a,b] = f[b] − f[a]`.
    pub fn d0(&self, f: &Cochain<G0>) -> Cochain<G1> {
        let values = self
            .edges
            .iter()
            .map(|e| f.values[e[1]] - f.values[e[0]])
            .collect();
        Cochain::new(values)
    }

    /// The exterior derivative `d₁: 1-forms → 2-forms` (discrete curl):
    /// `(d₁ω)[a,b,c] = ω[a,b] − ω[a,c] + ω[b,c]` (the signed face boundary).
    pub fn d1(&self, w: &Cochain<G1>) -> Cochain<G2> {
        let values = self
            .triangles
            .iter()
            .map(|t| {
                let (a, b, c) = (t[0], t[1], t[2]);
                w.values[self.edge_of(a, b)] - w.values[self.edge_of(a, c)]
                    + w.values[self.edge_of(b, c)]
            })
            .collect();
        Cochain::new(values)
    }

    /// The incidence matrix of `d₀` (edges × vertices).
    fn d0_matrix(&self) -> Vec<Vec<f64>> {
        self.edges
            .iter()
            .map(|e| {
                let mut row = vec![0.0; self.n_vertices];
                row[e[0]] = -1.0;
                row[e[1]] = 1.0;
                row
            })
            .collect()
    }

    /// The incidence matrix of `d₁` (triangles × edges).
    fn d1_matrix(&self) -> Vec<Vec<f64>> {
        self.triangles
            .iter()
            .map(|t| {
                let mut row = vec![0.0; self.edges.len()];
                let (a, b, c) = (t[0], t[1], t[2]);
                row[self.edge_of(a, b)] += 1.0;
                row[self.edge_of(a, c)] -= 1.0;
                row[self.edge_of(b, c)] += 1.0;
                row
            })
            .collect()
    }

    /// Number of connected components, `b₀ = n_vertices − rank(d₀)`.
    pub fn betti0(&self) -> usize {
        self.n_vertices - matrix_rank(self.d0_matrix())
    }

    /// The first Betti number (independent 1-cycles / holes),
    /// `b₁ = dim ker d₁ − dim im d₀ = n_edges − rank(d₁) − rank(d₀)`.
    pub fn betti1(&self) -> usize {
        let r0 = matrix_rank(self.d0_matrix());
        let r1 = matrix_rank(self.d1_matrix());
        self.edges.len() - r1 - r0
    }

    /// The second Betti number (enclosed voids). With no 3-cells,
    /// `b₂ = n_triangles − rank(d₁)`.
    pub fn betti2(&self) -> usize {
        self.triangles.len() - matrix_rank(self.d1_matrix())
    }

    /// The Euler characteristic as the alternating sum of cell counts,
    /// `χ = V − E + F`.
    pub fn euler_from_cells(&self) -> i64 {
        self.n_vertices as i64 - self.edges.len() as i64 + self.triangles.len() as i64
    }

    /// The Euler characteristic as the alternating sum of Betti numbers,
    /// `χ = b₀ − b₁ + b₂` (the Euler–Poincaré theorem).
    pub fn euler_from_betti(&self) -> i64 {
        self.betti0() as i64 - self.betti1() as i64 + self.betti2() as i64
    }

    /// Dimension of the space of harmonic 1-forms, `dim ker Δ₁`, where the
    /// combinatorial Hodge Laplacian is `Δ₁ = d₀ d₀ᵀ + d₁ᵀ d₁`. By the Hodge
    /// theorem this equals `b₁`.
    pub fn harmonic1_dim(&self) -> usize {
        let n = self.edges.len();
        let d0 = self.d0_matrix(); // edges × vertices
        let down = matmul(&d0, &transpose(&d0)); // edges × edges
                                                 // The "up" term d₁ᵀd₁ is the n×n zero matrix when there are no faces.
        let up = if self.triangles.is_empty() {
            vec![vec![0.0; n]; n]
        } else {
            let d1 = self.d1_matrix(); // triangles × edges
            matmul(&transpose(&d1), &d1) // edges × edges
        };
        let laplacian = matadd(&down, &up);
        n - matrix_rank(laplacian)
    }
}

/// Transpose of a real matrix.
fn transpose(m: &[Vec<f64>]) -> Vec<Vec<f64>> {
    if m.is_empty() {
        return Vec::new();
    }
    let (rows, cols) = (m.len(), m[0].len());
    (0..cols)
        .map(|c| (0..rows).map(|r| m[r][c]).collect())
        .collect()
}

/// Product of two real matrices (`a` is `p×q`, `b` is `q×r`).
fn matmul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    if a.is_empty() || b.is_empty() {
        return Vec::new();
    }
    let (p, q, r) = (a.len(), b.len(), b[0].len());
    (0..p)
        .map(|i| {
            (0..r)
                .map(|j| (0..q).map(|k| a[i][k] * b[k][j]).sum())
                .collect()
        })
        .collect()
}

/// Entrywise sum of two equally-shaped matrices.
fn matadd(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    a.iter()
        .zip(b.iter())
        .map(|(ra, rb)| ra.iter().zip(rb.iter()).map(|(x, y)| x + y).collect())
        .collect()
}

/// Rank of a real matrix by Gaussian elimination with partial pivoting.
fn matrix_rank(mut m: Vec<Vec<f64>>) -> usize {
    let rows = m.len();
    if rows == 0 {
        return 0;
    }
    let cols = m[0].len();
    let mut rank = 0;
    let mut pivot_row = 0;
    for col in 0..cols {
        if pivot_row >= rows {
            break;
        }
        let mut sel = pivot_row;
        for r in (pivot_row + 1)..rows {
            if m[r][col].abs() > m[sel][col].abs() {
                sel = r;
            }
        }
        if m[sel][col].abs() < 1e-9 {
            continue;
        }
        m.swap(pivot_row, sel);
        let pivot_vals = m[pivot_row].clone();
        let pivot = pivot_vals[col];
        for (r, row) in m.iter_mut().enumerate() {
            if r != pivot_row {
                let factor = row[col] / pivot;
                if factor != 0.0 {
                    for (c, pv) in pivot_vals.iter().enumerate().skip(col) {
                        row[c] -= factor * pv;
                    }
                }
            }
        }
        pivot_row += 1;
        rank += 1;
    }
    rank
}

/// The simplicial complex a [`DeRham`] object is evaluated on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    /// Filled triangle — a disk (`b₁ = 0`).
    Disk,
    /// Hollow triangle — a circle (`b₁ = 1`).
    Circle,
    /// Triangulated 3×3 flat torus (`b₁ = 2`).
    Torus,
}

impl Shape {
    fn name(self) -> &'static str {
        match self {
            Shape::Disk => "disk",
            Shape::Circle => "circle",
            Shape::Torus => "torus",
        }
    }
    fn from_name(s: &str) -> Option<Self> {
        match s {
            "disk" => Some(Shape::Disk),
            "circle" => Some(Shape::Circle),
            "torus" => Some(Shape::Torus),
            _ => None,
        }
    }
    /// Expected first Betti number, the textbook value this shape should yield.
    fn expected_b1(self) -> usize {
        match self {
            Shape::Disk => 0,
            Shape::Circle => 1,
            Shape::Torus => 2,
        }
    }
    fn complex(self) -> Complex {
        match self {
            Shape::Disk => Complex::disk(),
            Shape::Circle => Complex::circle(),
            Shape::Torus => Complex::torus(),
        }
    }
}

/// De Rham cohomology on a simplicial complex, via the exterior derivative.
///
/// Named claims:
/// - `dec.d-squared-zero`: `d ∘ d = 0` (an exact theorem of the coboundary),
/// - `dec.first-betti-number`: the computed number of holes `b₁`,
/// - `dec.closed-equals-exact`: every closed 1-form is exact (Poincaré),
/// - `dec.euler-poincare`: `V−E+F = b₀−b₁+b₂`,
/// - `dec.hodge-harmonic`: `dim(harmonic 1-forms) = b₁`.
///
/// The `shape` knob (`disk`, `circle`, `torus`) selects the complex.
#[derive(Clone, Debug)]
pub struct DeRham {
    /// Which simplicial complex to evaluate on.
    shape: Shape,
}

impl Default for DeRham {
    fn default() -> Self {
        // The disk is the contractible default.
        Self { shape: Shape::Disk }
    }
}

impl DeRham {
    fn complex(&self) -> Complex {
        self.shape.complex()
    }
}

/// `dec.d-squared-zero`.
pub const D_SQUARED_ZERO: &str = "dec.d-squared-zero";
/// `dec.first-betti-number`.
pub const FIRST_BETTI: &str = "dec.first-betti-number";
/// `dec.closed-equals-exact`.
pub const CLOSED_EQUALS_EXACT: &str = "dec.closed-equals-exact";
/// `dec.euler-poincare`.
pub const EULER_POINCARE: &str = "dec.euler-poincare";
/// `dec.hodge-harmonic`.
pub const HODGE_HARMONIC: &str = "dec.hodge-harmonic";

const SHAPE_OPTIONS: &[&str] = &["disk", "circle", "torus"];

const SPECS: &[KnobSpec] = &[KnobSpec {
    name: "shape",
    layer: LayerId::Mathematical,
    doc: "The simplicial complex to evaluate on: disk (b₁=0), circle (b₁=1), or torus (b₁=2). Changing it changes the topology and the Poincaré verdict.",
    domain: KnobDomain::Choice(SHAPE_OPTIONS),
}];

impl Knobbed for DeRham {
    fn specs(&self) -> &'static [KnobSpec] {
        SPECS
    }
    fn get(&self, name: &str) -> Result<KnobValue, CoreError> {
        match name {
            "shape" => Ok(KnobValue::Choice(self.shape.name().to_string())),
            _ => Err(CoreError::UnknownKnob { name: name.into() }),
        }
    }
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError> {
        let spec = self.spec(name)?;
        spec.domain.check(name, &value)?;
        let old = self.get(name)?;
        match (name, &value) {
            ("shape", KnobValue::Choice(v)) => {
                self.shape = Shape::from_name(v).ok_or_else(|| CoreError::Domain {
                    name: name.into(),
                    reason: format!("unknown shape '{v}'"),
                })?;
            }
            _ => {
                return Err(CoreError::TypeMismatch {
                    name: name.into(),
                    expected: spec.domain.kind_name().into(),
                    got: value.kind_name().into(),
                });
            }
        }
        Ok(old)
    }
}

impl Theory for DeRham {
    fn id(&self) -> &'static str {
        "de-rham"
    }
    fn name(&self) -> &'static str {
        "De Rham cohomology"
    }
    fn summary(&self) -> &'static str {
        "Discrete exterior calculus on a simplicial complex. d²=0 is an exact \
         theorem of the coboundary; the first Betti number, computed from the \
         incidence ranks, counts holes; and whether every closed 1-form is exact \
         (Poincaré) detects the topology — flipped by the `filled` knob."
    }
    fn world(&self) -> Option<World> {
        None // pure mathematics: no spacetime/gauge/spectrum projection
    }
    fn note(&self) -> String {
        let c = self.complex();
        format!(
            "simplicial complex: {} ({} vertices, {} edges, {} triangles), b₁ = {}",
            self.shape.name(),
            c.n_vertices,
            c.edges.len(),
            c.triangles.len(),
            c.betti1()
        )
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            Claim::new(
                D_SQUARED_ZERO,
                "The exterior derivative is nilpotent: d ∘ d = 0.",
                LayerId::Mathematical,
                Epistemic::Theorem,
            ),
            Claim::new(
                FIRST_BETTI,
                "The first Betti number counts independent 1-cycles (holes).",
                LayerId::Mathematical,
                Epistemic::Theorem,
            ),
            Claim::new(
                CLOSED_EQUALS_EXACT,
                "Every closed 1-form is exact (the Poincaré lemma).",
                LayerId::Mathematical,
                Epistemic::Theorem,
            ),
            Claim::new(
                EULER_POINCARE,
                "The Euler characteristic V−E+F equals b₀−b₁+b₂.",
                LayerId::Mathematical,
                Epistemic::Theorem,
            ),
            Claim::new(
                HODGE_HARMONIC,
                "The dimension of harmonic 1-forms equals b₁ (Hodge theorem).",
                LayerId::Mathematical,
                Epistemic::Theorem,
            ),
        ]
    }
    fn evaluate(&self, claim: &Claim) -> Verdict {
        let c = self.complex();
        match claim.id.0.as_str() {
            D_SQUARED_ZERO => {
                // d₁(d₀ f) = 0 for a basis of 0-cochains ⇒ d∘d = 0 exactly.
                let mut worst = 0.0_f64;
                for i in 0..c.n_vertices {
                    let mut f = Cochain::<G0>::zero(c.n_vertices);
                    f.values[i] = 1.0;
                    let ddf = c.d1(&c.d0(&f));
                    worst = worst.max(ddf.values.iter().fold(0.0, |m, v| m.max(v.abs())));
                }
                if worst < 1e-12 {
                    Verdict::holds(
                        Epistemic::Theorem,
                        "d₁∘d₀ = 0 on every basis 0-form (curl grad = 0)",
                    )
                    .with_evidence([format!(
                        "max |d(d f)| = {worst:.2e} over all {} basis functions",
                        c.n_vertices
                    )])
                } else {
                    Verdict::fails(Epistemic::Theorem, format!("d∘d ≠ 0: max = {worst:.2e}"))
                }
            }
            FIRST_BETTI => {
                let b1 = c.betti1();
                let expected = self.shape.expected_b1();
                if b1 == expected {
                    Verdict::holds(
                        Epistemic::Theorem,
                        format!(
                            "b₁ = {b1} for the {} — computed from n_edges − rank(d₁) − rank(d₀)",
                            self.shape.name()
                        ),
                    )
                    .with_evidence([format!(
                        "b₀ = {}, b₁ = {b1}, b₂ = {}",
                        c.betti0(),
                        c.betti2()
                    )])
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        format!("computed b₁ = {b1}, expected {expected}"),
                    )
                }
            }
            CLOSED_EQUALS_EXACT => {
                let b1 = c.betti1();
                if b1 == 0 {
                    Verdict::holds(
                        Epistemic::Theorem,
                        "b₁ = 0: every closed 1-form is exact (Poincaré lemma holds on the disk)",
                    )
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        format!(
                            "b₁ = {b1}: a closed 1-form that is not exact exists (the {} has holes)",
                            self.shape.name()
                        ),
                    )
                    .with_evidence([format!(
                        "closed ≠ exact detects topology: {} has {b1} independent 1-cycle(s)",
                        self.shape.name()
                    )])
                }
            }
            EULER_POINCARE => {
                let chi_cells = c.euler_from_cells();
                let chi_betti = c.euler_from_betti();
                if chi_cells == chi_betti {
                    Verdict::holds(
                        Epistemic::Theorem,
                        format!("χ = V−E+F = {chi_cells} = b₀−b₁+b₂ (Euler–Poincaré)"),
                    )
                    .with_evidence([format!(
                        "V−E+F = {chi_cells}; b₀−b₁+b₂ = {chi_betti} (b₀={}, b₁={}, b₂={})",
                        c.betti0(),
                        c.betti1(),
                        c.betti2()
                    )])
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        format!("χ mismatch: cells {chi_cells} ≠ Betti {chi_betti}"),
                    )
                }
            }
            HODGE_HARMONIC => {
                let harmonic = c.harmonic1_dim();
                let b1 = c.betti1();
                if harmonic == b1 {
                    Verdict::holds(
                        Epistemic::Theorem,
                        format!(
                            "dim(harmonic 1-forms) = {harmonic} = b₁ (Hodge: harmonic ≅ cohomology)"
                        ),
                    )
                    .with_evidence([format!(
                        "nullity of Δ₁ = d₀d₀ᵀ + d₁ᵀd₁ is {harmonic}, matching b₁ = {b1}"
                    )])
                } else {
                    Verdict::fails(
                        Epistemic::Theorem,
                        format!("harmonic 1-forms dim {harmonic} ≠ b₁ {b1}"),
                    )
                }
            }
            _ => Verdict::inapplicable("claim not made by the de Rham object"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;

    fn kind(t: &dyn Theory, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id.0 == id).unwrap();
        t.evaluate(&c).kind
    }

    #[test]
    fn d_squared_is_exactly_zero() {
        // curl grad = 0, on both complexes, for an arbitrary 0-form.
        for cx in [Complex::disk(), Complex::circle()] {
            let f = Cochain::<G0>::new(vec![0.3, -1.7, 2.9]);
            let ddf = cx.d1(&cx.d0(&f));
            assert!(ddf.is_zero(1e-12), "d² should vanish");
        }
    }

    #[test]
    fn betti_numbers_are_correct() {
        let disk = Complex::disk();
        assert_eq!(disk.betti0(), 1); // connected
        assert_eq!(disk.betti1(), 0); // no hole
        let circle = Complex::circle();
        assert_eq!(circle.betti0(), 1);
        assert_eq!(circle.betti1(), 1); // one hole
    }

    #[test]
    fn shape_knob_flips_poincare_and_betti() {
        // Topology as a knob → verdict diff.
        let mut t = DeRham::default();
        assert_eq!(t.shape, Shape::Disk);
        assert_eq!(kind(&t, CLOSED_EQUALS_EXACT), VerdictKind::Holds);
        assert_eq!(kind(&t, FIRST_BETTI), VerdictKind::Holds);

        t.set("shape", KnobValue::Choice("circle".into())).unwrap();
        assert_eq!(kind(&t, CLOSED_EQUALS_EXACT), VerdictKind::Fails);
        // The Betti claim still *holds* (b₁ = 1 is the correct expected value).
        assert_eq!(kind(&t, FIRST_BETTI), VerdictKind::Holds);
    }

    #[test]
    fn torus_has_two_holes() {
        // A non-trivial homology check beyond the minimal disk/circle.
        let t = Complex::torus();
        assert_eq!(t.n_vertices, 9);
        assert_eq!(t.edges.len(), 27);
        assert_eq!(t.triangles.len(), 18);
        assert_eq!(t.betti0(), 1); // connected
        assert_eq!(t.betti1(), 2); // two independent 1-cycles
        assert_eq!(t.betti2(), 1); // one enclosed void (closed surface)
        assert_eq!(t.euler_from_cells(), 0); // χ(T²) = 0
        assert_eq!(t.euler_from_cells(), t.euler_from_betti());
        assert_eq!(t.harmonic1_dim(), 2); // Hodge: matches b₁
    }

    #[test]
    fn torus_via_the_shape_knob() {
        let mut t = DeRham::default();
        t.set("shape", KnobValue::Choice("torus".into())).unwrap();
        assert_eq!(kind(&t, FIRST_BETTI), VerdictKind::Holds);
        assert_eq!(kind(&t, D_SQUARED_ZERO), VerdictKind::Holds);
        assert_eq!(kind(&t, EULER_POINCARE), VerdictKind::Holds);
        assert_eq!(kind(&t, HODGE_HARMONIC), VerdictKind::Holds);
        // The torus has holes, so closed ≠ exact.
        assert_eq!(kind(&t, CLOSED_EQUALS_EXACT), VerdictKind::Fails);
        // An unknown shape is rejected by the domain.
        assert!(t.set("shape", KnobValue::Choice("klein".into())).is_err());
    }

    #[test]
    fn d_squared_zero_claim_holds() {
        assert_eq!(kind(&DeRham::default(), D_SQUARED_ZERO), VerdictKind::Holds);
    }

    #[test]
    fn euler_characteristic_agrees_two_ways() {
        // V−E+F equals b₀−b₁+b₂ on both complexes (Euler–Poincaré).
        for cx in [Complex::disk(), Complex::circle()] {
            assert_eq!(cx.euler_from_cells(), cx.euler_from_betti());
        }
        assert_eq!(Complex::disk().euler_from_cells(), 1); // disk: χ = 1
        assert_eq!(Complex::circle().euler_from_cells(), 0); // circle: χ = 0
    }

    #[test]
    fn hodge_harmonic_dimension_equals_betti1() {
        // The Hodge theorem: dim ker Δ₁ = b₁, computed from the Laplacian.
        assert_eq!(Complex::disk().harmonic1_dim(), Complex::disk().betti1());
        assert_eq!(
            Complex::circle().harmonic1_dim(),
            Complex::circle().betti1()
        );
        assert_eq!(Complex::circle().harmonic1_dim(), 1);
        assert_eq!(Complex::disk().harmonic1_dim(), 0);
    }

    #[test]
    fn euler_and_hodge_claims_hold_under_the_knob() {
        let mut t = DeRham::default();
        assert_eq!(kind(&t, EULER_POINCARE), VerdictKind::Holds);
        assert_eq!(kind(&t, HODGE_HARMONIC), VerdictKind::Holds);
        // Both are identities: they still hold on the circle and the torus.
        t.set("shape", KnobValue::Choice("circle".into())).unwrap();
        assert_eq!(kind(&t, EULER_POINCARE), VerdictKind::Holds);
        assert_eq!(kind(&t, HODGE_HARMONIC), VerdictKind::Holds);
        t.set("shape", KnobValue::Choice("torus".into())).unwrap();
        assert_eq!(kind(&t, EULER_POINCARE), VerdictKind::Holds);
        assert_eq!(kind(&t, HODGE_HARMONIC), VerdictKind::Holds);
    }

    #[test]
    fn transpose_and_matmul_are_correct() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]; // 2×3
        let at = transpose(&a);
        assert_eq!(at, vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]]);
        // a·aᵀ = [[14,32],[32,77]]
        let prod = matmul(&a, &at);
        assert_eq!(prod, vec![vec![14.0, 32.0], vec![32.0, 77.0]]);
    }

    #[test]
    fn matrix_rank_basic() {
        assert_eq!(matrix_rank(vec![vec![1.0, 0.0], vec![0.0, 1.0]]), 2);
        assert_eq!(matrix_rank(vec![vec![1.0, 1.0], vec![2.0, 2.0]]), 1);
        assert_eq!(matrix_rank(vec![vec![0.0, 0.0], vec![0.0, 0.0]]), 0);
    }
}
