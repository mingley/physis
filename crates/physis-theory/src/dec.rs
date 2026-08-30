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

use physis_core::claim::{Claim, ClaimClass, Verdict};
use physis_core::error::CoreError;
use physis_core::id::LayerId;
use physis_core::knob::{KnobDomain, KnobSpec, KnobValue, Knobbed};
use physis_core::ParameterOrigin;
use physis_model::World;
use physis_proof::lookup;

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

    /// A triangulated Klein bottle: a 4×4 grid glued into a torus in one
    /// direction and with a *flip* in the other (the top edge is identified with
    /// the bottom edge reversed). Non-orientable closed surface: over ℝ,
    /// `b₀ = 1`, `b₁ = 1`, `b₂ = 0`, `χ = 0`. The ℤ₂ torsion in `H₁` is invisible
    /// to real coefficients, and `b₂ = 0` records the non-orientability.
    pub fn klein_bottle() -> Self {
        let n = 4usize;
        // Logical (row a, col b) → canonical vertex index. Columns are glued
        // straight (periodic in b); rows are glued with a flip that mirrors b.
        let vert = |a: usize, b: usize| -> usize {
            let bb = b % n;
            if a == n {
                (n - bb) % n // seam: row n ≡ row 0, column mirrored
            } else {
                (a % n) * n + bb
            }
        };
        let mut edge_set = std::collections::BTreeSet::new();
        let mut triangles = Vec::new();
        let mut add_edge = |a: usize, b: usize| {
            if a != b {
                edge_set.insert((a.min(b), a.max(b)));
            }
        };
        for a in 0..n {
            for b in 0..n {
                let (c00, c01, c10, c11) = (
                    vert(a, b),
                    vert(a, b + 1),
                    vert(a + 1, b),
                    vert(a + 1, b + 1),
                );
                for tri in [[c00, c01, c11], [c00, c10, c11]] {
                    // Skip any degenerate triangle (repeated vertex).
                    if tri[0] != tri[1] && tri[1] != tri[2] && tri[0] != tri[2] {
                        add_edge(tri[0], tri[1]);
                        add_edge(tri[1], tri[2]);
                        add_edge(tri[0], tri[2]);
                        let mut t = tri;
                        t.sort_unstable();
                        triangles.push(t);
                    }
                }
            }
        }
        let edges = edge_set.into_iter().map(|(a, b)| [a, b]).collect();
        Self::new(n * n, edges, triangles)
    }

    /// The boundary of a tetrahedron: a triangulated 2-sphere `S²`.
    /// Closed orientable surface with `b₀ = 1`, `b₁ = 0`, `b₂ = 1`, `χ = 2`.
    pub fn sphere() -> Self {
        Self::new(
            4,
            vec![[0, 1], [0, 2], [0, 3], [1, 2], [1, 3], [2, 3]],
            vec![[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]],
        )
    }

    /// True if every edge borders exactly two triangles — the combinatorial
    /// signature of a closed surface (a validity check for the constructions).
    pub fn is_closed_surface(&self) -> bool {
        let mut count: HashMap<(usize, usize), usize> = HashMap::new();
        for t in &self.triangles {
            for (a, b) in [(t[0], t[1]), (t[0], t[2]), (t[1], t[2])] {
                *count.entry((a.min(b), a.max(b))).or_insert(0) += 1;
            }
        }
        !count.is_empty() && count.values().all(|&c| c == 2)
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
    /// Triangulated Klein bottle (non-orientable; over ℝ, `b₁ = 1`, `b₂ = 0`).
    Klein,
    /// Boundary of a tetrahedron — a 2-sphere (`b₁ = 0`, `b₂ = 1`, `χ = 2`).
    Sphere,
}

impl Shape {
    fn name(self) -> &'static str {
        match self {
            Shape::Disk => "disk",
            Shape::Circle => "circle",
            Shape::Torus => "torus",
            Shape::Klein => "klein",
            Shape::Sphere => "sphere",
        }
    }
    fn from_name(s: &str) -> Option<Self> {
        match s {
            "disk" => Some(Shape::Disk),
            "circle" => Some(Shape::Circle),
            "torus" => Some(Shape::Torus),
            "klein" => Some(Shape::Klein),
            "sphere" => Some(Shape::Sphere),
            _ => None,
        }
    }
    /// Expected first Betti number (over ℝ), the textbook value this shape yields.
    fn expected_b1(self) -> usize {
        match self {
            Shape::Disk => 0,
            Shape::Circle => 1,
            Shape::Torus => 2,
            Shape::Klein => 1, // ℤ₂ torsion in H₁ is invisible over ℝ
            Shape::Sphere => 0,
        }
    }
    fn complex(self) -> Complex {
        match self {
            Shape::Disk => Complex::disk(),
            Shape::Circle => Complex::circle(),
            Shape::Torus => Complex::torus(),
            Shape::Klein => Complex::klein_bottle(),
            Shape::Sphere => Complex::sphere(),
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
/// - `dec.fundamental-class`: `b₂ = 1` (an orientable closed surface over ℝ).
///
/// The `shape` knob (`disk`, `circle`, `torus`, `klein`, `sphere`) selects the complex.
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
/// `dec.fundamental-class`: `b₂ = 1` over ℝ.
pub const FUNDAMENTAL_CLASS: &str = "dec.fundamental-class";

const SHAPE_OPTIONS: &[&str] = &["disk", "circle", "torus", "klein", "sphere"];

const SPECS: &[KnobSpec] = &[KnobSpec {
    name: "shape",
    layer: LayerId::Mathematical,
    doc: "The simplicial complex to evaluate on: disk (b₁=0, b₂=0), circle (b₁=1), torus (b₁=2, b₂=1), klein (Klein bottle, b₁=1, b₂=0), or sphere (S², b₁=0, b₂=1, χ=2). Changing it changes the topology.",
    origin: ParameterOrigin::Chosen,
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
         theorem of the coboundary; Betti numbers, computed from incidence ranks, \
         count holes and voids; Poincaré (closed = exact) and the fundamental \
         class (b₂ = 1) detect topology — flipped by the `shape` knob."
    }
    fn world(&self) -> Option<World> {
        None // pure mathematics: no spacetime/gauge/spectrum projection
    }
    fn note(&self) -> String {
        let c = self.complex();
        format!(
            "simplicial complex: {} ({} vertices, {} edges, {} triangles), \
             b₀ = {}, b₁ = {}, b₂ = {}, χ = {}",
            self.shape.name(),
            c.n_vertices,
            c.edges.len(),
            c.triangles.len(),
            c.betti0(),
            c.betti1(),
            c.betti2(),
            c.euler_from_cells()
        )
    }
    fn claims(&self) -> Vec<Claim> {
        vec![
            lookup(D_SQUARED_ZERO)
                .expect("d² is a catalog identity")
                .lab_claim(),
            Claim::new(
                FIRST_BETTI,
                "The first Betti number counts independent 1-cycles (holes).",
                LayerId::Mathematical,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                CLOSED_EQUALS_EXACT,
                "Every closed 1-form is exact (the Poincaré lemma).",
                LayerId::Mathematical,
                ClaimClass::ModelInternal,
            )
            .with_dependencies(&[D_SQUARED_ZERO]),
            Claim::new(
                EULER_POINCARE,
                "The Euler characteristic V−E+F equals b₀−b₁+b₂.",
                LayerId::Mathematical,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                HODGE_HARMONIC,
                "The dimension of harmonic 1-forms equals b₁ (Hodge theorem).",
                LayerId::Mathematical,
                ClaimClass::ModelInternal,
            ),
            Claim::new(
                FUNDAMENTAL_CLASS,
                "The complex has a fundamental class over ℝ: b₂ = 1.",
                LayerId::Mathematical,
                ClaimClass::ModelInternal,
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
                    Verdict::holds(claim, "d₁∘d₀ = 0 on every basis 0-form (curl grad = 0)")
                        .with_evidence([format!(
                            "max |d(d f)| = {worst:.2e} over all {} basis functions",
                            c.n_vertices
                        )])
                } else {
                    Verdict::fails(claim, format!("d∘d ≠ 0: max = {worst:.2e}"))
                }
            }
            FIRST_BETTI => {
                let b1 = c.betti1();
                let expected = self.shape.expected_b1();
                if b1 == expected {
                    Verdict::holds(
                        claim,
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
                    Verdict::fails(claim, format!("computed b₁ = {b1}, expected {expected}"))
                }
            }
            CLOSED_EQUALS_EXACT => {
                let b1 = c.betti1();
                if b1 == 0 {
                    Verdict::holds(
                        claim,
                        format!(
                            "b₁ = 0: every closed 1-form is exact (Poincaré lemma holds on the {})",
                            self.shape.name()
                        ),
                    )
                } else {
                    Verdict::fails(claim,
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
                        claim,
                        format!("χ = V−E+F = {chi_cells} = b₀−b₁+b₂ (Euler–Poincaré)"),
                    )
                    .with_evidence([
                        format!(
                            "V−E+F = {chi_cells}; b₀−b₁+b₂ = {chi_betti} (b₀={}, b₁={}, b₂={})",
                            c.betti0(),
                            c.betti1(),
                            c.betti2()
                        ),
                        "rank-nullity cancellation: b₀−b₁+b₂ ≡ V−E+F for these Betti formulas; not a second path"
                            .to_string(),
                    ])
                } else {
                    Verdict::fails(
                        claim,
                        format!("χ mismatch: cells {chi_cells} ≠ Betti {chi_betti}"),
                    )
                }
            }
            HODGE_HARMONIC => {
                let harmonic = c.harmonic1_dim();
                let b1 = c.betti1();
                if harmonic == b1 {
                    Verdict::holds(
                        claim,
                        format!(
                            "dim(harmonic 1-forms) = {harmonic} = b₁ (Hodge: harmonic ≅ cohomology)"
                        ),
                    )
                    .with_evidence([format!(
                        "nullity of Δ₁ = d₀d₀ᵀ + d₁ᵀd₁ is {harmonic}, matching b₁ = {b1}"
                    )])
                    .with_cross_checked()
                } else {
                    Verdict::fails(claim, format!("harmonic 1-forms dim {harmonic} ≠ b₁ {b1}"))
                }
            }
            FUNDAMENTAL_CLASS => {
                let b2 = c.betti2();
                if b2 == 1 {
                    Verdict::holds(
                        claim,
                        format!(
                            "b₂ = 1: the {} has a fundamental class over ℝ",
                            self.shape.name()
                        ),
                    )
                    .with_evidence([format!(
                        "closed={}, χ = {}, b₀ = {}, b₁ = {}, b₂ = {b2}",
                        c.is_closed_surface(),
                        c.euler_from_cells(),
                        c.betti0(),
                        c.betti1()
                    )])
                } else {
                    let why = if self.shape == Shape::Klein {
                        "b₂ = 0: non-orientable surfaces have no fundamental class over ℝ"
                    } else {
                        "b₂ ≠ 1: no 2-dimensional fundamental class over ℝ"
                    };
                    Verdict::fails(claim, why).with_evidence([format!(
                        "{}: closed={}, χ = {}, b₀ = {}, b₁ = {}, b₂ = {b2}",
                        self.shape.name(),
                        c.is_closed_surface(),
                        c.euler_from_cells(),
                        c.betti0(),
                        c.betti1()
                    )])
                }
            }
            _ => Verdict::inapplicable(claim, "claim not made by the de Rham object"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use physis_core::claim::VerdictKind;
    use physis_core::{DerivationAssurance, Quantifier};

    fn kind(t: &dyn Theory, id: &str) -> VerdictKind {
        let c = t.claims().into_iter().find(|c| c.id.0 == id).unwrap();
        t.evaluate(&c).kind
    }

    fn derivation(t: &dyn Theory, id: &str) -> DerivationAssurance {
        let c = t.claims().into_iter().find(|c| c.id.0 == id).unwrap();
        t.evaluate(&c).derivation
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
    fn klein_bottle_real_homology() {
        // Non-orientable closed surface. Over ℝ the ℤ₂ torsion in H₁ is
        // invisible, so b₁ = 1 (not 2 like the torus), and b₂ = 0 records the
        // non-orientability, yet χ = 0 like the torus.
        let k = Complex::klein_bottle();
        assert!(
            k.is_closed_surface(),
            "each edge must border exactly 2 faces"
        );
        assert_eq!(k.betti0(), 1);
        assert_eq!(k.betti1(), 1); // ℝ can't see the ℤ₂ torsion
        assert_eq!(k.betti2(), 0); // non-orientable: no fundamental class over ℝ
        assert_eq!(k.euler_from_cells(), 0);
        assert_eq!(k.euler_from_cells(), k.euler_from_betti());
        assert_eq!(k.harmonic1_dim(), 1);
        // The torus and Klein bottle share χ = 0 but differ in b₁ and b₂.
        let t = Complex::torus();
        assert_eq!(t.euler_from_cells(), k.euler_from_cells());
        assert_ne!(t.betti1(), k.betti1());
        assert_ne!(t.betti2(), k.betti2());
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
        // The torus has holes, so closed ≠ exact, but it is orientable: b₂ = 1.
        assert_eq!(kind(&t, CLOSED_EQUALS_EXACT), VerdictKind::Fails);
        assert_eq!(kind(&t, FUNDAMENTAL_CLASS), VerdictKind::Holds);
        // An unknown shape is rejected by the domain.
        assert!(t.set("shape", KnobValue::Choice("mobius".into())).is_err());
    }

    #[test]
    fn klein_bottle_via_the_shape_knob() {
        let mut t = DeRham::default();
        t.set("shape", KnobValue::Choice("klein".into())).unwrap();
        assert_eq!(kind(&t, FIRST_BETTI), VerdictKind::Holds); // b₁ = 1 over ℝ
        assert_eq!(kind(&t, D_SQUARED_ZERO), VerdictKind::Holds);
        assert_eq!(kind(&t, EULER_POINCARE), VerdictKind::Holds);
        assert_eq!(kind(&t, HODGE_HARMONIC), VerdictKind::Holds);
        assert_eq!(kind(&t, CLOSED_EQUALS_EXACT), VerdictKind::Fails); // it has a hole
        assert_eq!(kind(&t, FUNDAMENTAL_CLASS), VerdictKind::Fails); // no class over ℝ
    }

    #[test]
    fn sphere_is_the_tetrahedron_boundary() {
        let s = Complex::sphere();
        assert!(
            s.is_closed_surface(),
            "each edge of S² must border exactly 2 faces"
        );
        assert_eq!(s.n_vertices, 4);
        assert_eq!(s.edges.len(), 6);
        assert_eq!(s.triangles.len(), 4);
        assert_eq!(s.betti0(), 1);
        assert_eq!(s.betti1(), 0); // simply connected
        assert_eq!(s.betti2(), 1); // fundamental class
        assert_eq!(s.euler_from_cells(), 2); // χ(S²) = 2
        assert_eq!(s.euler_from_cells(), s.euler_from_betti());
        assert_eq!(s.harmonic1_dim(), 0);
        // d² = 0 on a 4-vertex 0-form.
        let f = Cochain::<G0>::new(vec![0.3, -1.7, 2.9, 0.5]);
        assert!(s.d1(&s.d0(&f)).is_zero(1e-12));
        // Disk shares b₁ = 0 (Poincaré) but not χ, b₂, or closedness.
        let d = Complex::disk();
        assert_eq!(d.betti1(), s.betti1());
        assert_ne!(d.betti2(), s.betti2());
        assert_ne!(d.euler_from_cells(), s.euler_from_cells());
        assert!(!d.is_closed_surface());
    }

    #[test]
    fn sphere_via_the_shape_knob_gains_a_fundamental_class() {
        let mut t = DeRham::default();
        assert_eq!(kind(&t, FUNDAMENTAL_CLASS), VerdictKind::Fails); // disk, b₂ = 0
        assert_eq!(kind(&t, CLOSED_EQUALS_EXACT), VerdictKind::Holds);
        t.set("shape", KnobValue::Choice("sphere".into())).unwrap();
        assert_eq!(kind(&t, FUNDAMENTAL_CLASS), VerdictKind::Holds); // S², b₂ = 1
        assert_eq!(kind(&t, CLOSED_EQUALS_EXACT), VerdictKind::Holds); // still b₁ = 0
        assert_eq!(kind(&t, FIRST_BETTI), VerdictKind::Holds);
        assert_eq!(kind(&t, EULER_POINCARE), VerdictKind::Holds);
        assert_eq!(kind(&t, HODGE_HARMONIC), VerdictKind::Holds);
        assert_eq!(kind(&t, D_SQUARED_ZERO), VerdictKind::Holds);
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
    fn euler_poincare_is_rank_cancellation_not_a_second_path() {
        // b0 = V-r0, b1 = E-r1-r0, b2 = F-r1 ⇒ b0-b1+b2 ≡ V-E+F.
        // A broken matrix_rank still satisfies Euler-Poincaré, so that
        // agreement must not mint CrossChecked.
        for cx in [
            Complex::disk(),
            Complex::circle(),
            Complex::torus(),
            Complex::klein_bottle(),
            Complex::sphere(),
        ] {
            assert_eq!(cx.euler_from_cells(), cx.euler_from_betti());
        }
    }

    #[test]
    fn forgetting_a_laplacian_term_is_not_hodge() {
        // Circle has no faces: Δ = d0 d0^T. The zero matrix has nullity E,
        // not b1, so Hodge is a genuine second matrix, not rank-cancellation.
        let circle = Complex::circle();
        let n = circle.edges.len();
        let zero = vec![vec![0.0; n]; n];
        let zero_nullity = n - matrix_rank(zero);
        assert_eq!(zero_nullity, n);
        assert_eq!(circle.betti1(), 1);
        assert_eq!(circle.harmonic1_dim(), 1);
        assert_ne!(zero_nullity, circle.betti1());

        let torus = Complex::torus();
        let d0 = torus.d0_matrix();
        let down_only = matmul(&d0, &transpose(&d0));
        let dim_down = torus.edges.len() - matrix_rank(down_only);
        assert_ne!(
            dim_down,
            torus.betti1(),
            "d0 d0^T nullity is not b1 when faces exist"
        );
        assert_eq!(torus.harmonic1_dim(), torus.betti1());
    }

    #[test]
    fn euler_and_hodge_claims_hold_under_the_knob() {
        let mut t = DeRham::default();
        assert_eq!(kind(&t, EULER_POINCARE), VerdictKind::Holds);
        assert_eq!(kind(&t, HODGE_HARMONIC), VerdictKind::Holds);
        assert_eq!(
            derivation(&t, EULER_POINCARE),
            DerivationAssurance::Executed,
            "b0-b1+b2 is rank-cancellation of V-E+F, not a second path"
        );
        assert_eq!(
            derivation(&t, HODGE_HARMONIC),
            DerivationAssurance::CrossChecked
        );
        assert_eq!(
            derivation(&t, D_SQUARED_ZERO),
            DerivationAssurance::Executed,
            "d² = 0 is a single-path identity, not a two-path cross-check"
        );
        let d2 = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == D_SQUARED_ZERO)
            .unwrap();
        assert_eq!(d2.commitments.quantifier, Quantifier::ForAll);
        assert!(d2
            .commitments
            .formal_libraries
            .iter()
            .any(|l| l == "physlib:unversioned"));
        let poincare = t
            .claims()
            .into_iter()
            .find(|c| c.id.0 == CLOSED_EQUALS_EXACT)
            .unwrap();
        assert_eq!(poincare.commitments.quantifier, Quantifier::Unspecified);
        assert!(
            poincare.commitments.formal_libraries.is_empty(),
            "Poincaré is not a catalog polynomial"
        );
        assert_eq!(
            derivation(&t, CLOSED_EQUALS_EXACT),
            DerivationAssurance::Executed,
            "Poincaré is b₁ = 0, not two independent χ computations"
        );
        // Both are identities: they still hold on the circle and the torus.
        t.set("shape", KnobValue::Choice("circle".into())).unwrap();
        assert_eq!(kind(&t, EULER_POINCARE), VerdictKind::Holds);
        assert_eq!(kind(&t, HODGE_HARMONIC), VerdictKind::Holds);
        assert_eq!(
            derivation(&t, EULER_POINCARE),
            DerivationAssurance::Executed
        );
        t.set("shape", KnobValue::Choice("torus".into())).unwrap();
        assert_eq!(kind(&t, EULER_POINCARE), VerdictKind::Holds);
        assert_eq!(kind(&t, HODGE_HARMONIC), VerdictKind::Holds);
        assert_eq!(
            derivation(&t, CLOSED_EQUALS_EXACT),
            DerivationAssurance::Executed
        );
        assert_eq!(
            derivation(&t, HODGE_HARMONIC),
            DerivationAssurance::CrossChecked
        );
        t.set("shape", KnobValue::Choice("sphere".into())).unwrap();
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
