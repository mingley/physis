//! Gauge groups as typed algebraic objects, plus known SM embeddings.
//!
//! Full root-system branching is a later milestone. Today, embeddings of
//! the Standard Model into GUT and heterotic groups are *encoded facts*
//! — standard textbook results stored as data, labelled `EncodedFact`.

use serde::{Deserialize, Serialize};

/// Simple compact Lie group (the building blocks).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "family", content = "n", rename_all = "kebab-case")]
pub enum SimpleGroup {
    /// U(1).
    U1,
    /// SU(n), n ≥ 2.
    Su(u8),
    /// SO(n), n ≥ 3.
    So(u8),
    /// Sp(n) (compact symplectic).
    Sp(u8),
    /// G₂.
    G2,
    /// F₄.
    F4,
    /// E₆.
    E6,
    /// E₇.
    E7,
    /// E₈.
    E8,
    /// Spin(n), the simply connected cover of SO(n).
    Spin(u8),
}

/// One step along the standard GUT chain toward the Standard Model.
enum SmStep {
    /// Descend to a smaller group that is a maximal subgroup on the chain.
    Down(SimpleGroup),
    /// The next step is the Standard Model gauge group itself.
    StandardModel,
}

impl SimpleGroup {
    /// Rank (Cartan dimension), when well-defined.
    pub fn rank(self) -> Option<u8> {
        match self {
            SimpleGroup::U1 => Some(1),
            SimpleGroup::Su(n) if n >= 2 => Some(n - 1),
            SimpleGroup::So(n) if n >= 3 => Some(n / 2),
            SimpleGroup::Sp(n) if n >= 1 => Some(n),
            SimpleGroup::G2 => Some(2),
            SimpleGroup::F4 => Some(4),
            SimpleGroup::E6 => Some(6),
            SimpleGroup::E7 => Some(7),
            SimpleGroup::E8 => Some(8),
            SimpleGroup::Spin(n) if n >= 3 => Some(n / 2),
            _ => None,
        }
    }

    /// One textbook maximal-subgroup step toward the Standard Model, if this
    /// group sits on the standard GUT chain. Returns the next group down (or a
    /// signal that the Standard Model itself is the next step).
    fn step_toward_sm(self) -> Option<SmStep> {
        Some(match self {
            SimpleGroup::E8 => SmStep::Down(SimpleGroup::E6),
            SimpleGroup::E7 => SmStep::Down(SimpleGroup::E6),
            SimpleGroup::E6 => SmStep::Down(SimpleGroup::So(10)),
            SimpleGroup::So(10) | SimpleGroup::Spin(10) => SmStep::Down(SimpleGroup::Su(5)),
            SimpleGroup::So(n) | SimpleGroup::Spin(n) if n > 10 => {
                SmStep::Down(SimpleGroup::So(10))
            }
            SimpleGroup::Su(5) => SmStep::StandardModel,
            _ => return None,
        })
    }

    /// Verify — not discover — that the Standard Model embeds in this simple
    /// group by walking the standard maximal-subgroup chain and checking the
    /// necessary rank and dimension inequalities at every link.
    ///
    /// This is stronger than group equality but is **not** a full proof: rank
    /// and dimension containment are necessary, not sufficient, conditions, and
    /// the chain of maximal subgroups is encoded from the literature. Returns
    /// the chain of group names on success.
    fn sm_embedding_chain(self) -> Option<Vec<String>> {
        let mut chain = vec![self.name()];
        let mut cur = self;
        for _ in 0..16 {
            match cur.step_toward_sm()? {
                SmStep::Down(next) => {
                    if next.rank()? > cur.rank()? || next.dimension()? > cur.dimension()? {
                        return None;
                    }
                    chain.push(next.name());
                    cur = next;
                }
                SmStep::StandardModel => {
                    let sm = GaugeGroup::standard_model();
                    if sm.rank()? > cur.rank()? as u32 || sm.dimension()? > cur.dimension()? {
                        return None;
                    }
                    chain.push(sm.name());
                    return Some(chain);
                }
            }
        }
        None
    }

    /// Dimension of the group (number of generators), when well-defined.
    pub fn dimension(self) -> Option<u32> {
        Some(match self {
            SimpleGroup::U1 => 1,
            SimpleGroup::Su(n) if n >= 2 => (n as u32) * (n as u32) - 1,
            SimpleGroup::So(n) if n >= 3 => (n as u32) * (n as u32 - 1) / 2,
            SimpleGroup::Sp(n) if n >= 1 => (n as u32) * (2 * n as u32 + 1),
            SimpleGroup::G2 => 14,
            SimpleGroup::F4 => 52,
            SimpleGroup::E6 => 78,
            SimpleGroup::E7 => 133,
            SimpleGroup::E8 => 248,
            SimpleGroup::Spin(n) if n >= 3 => (n as u32) * (n as u32 - 1) / 2,
            _ => return None,
        })
    }

    /// Short name.
    pub fn name(self) -> String {
        match self {
            SimpleGroup::U1 => "U(1)".into(),
            SimpleGroup::Su(n) => format!("SU({n})"),
            SimpleGroup::So(n) => format!("SO({n})"),
            SimpleGroup::Sp(n) => format!("Sp({n})"),
            SimpleGroup::G2 => "G2".into(),
            SimpleGroup::F4 => "F4".into(),
            SimpleGroup::E6 => "E6".into(),
            SimpleGroup::E7 => "E7".into(),
            SimpleGroup::E8 => "E8".into(),
            SimpleGroup::Spin(n) => format!("Spin({n})"),
        }
    }
}

/// A compact gauge group: a product of simple (or U(1)) factors.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GaugeGroup {
    /// Factors, left to right.
    pub factors: Vec<SimpleGroup>,
}

impl GaugeGroup {
    /// No gauge symmetry.
    pub fn trivial() -> Self {
        Self { factors: vec![] }
    }

    /// Standard Model: SU(3)×SU(2)×U(1).
    pub fn standard_model() -> Self {
        Self {
            factors: vec![SimpleGroup::Su(3), SimpleGroup::Su(2), SimpleGroup::U1],
        }
    }

    /// Georgi–Glashow SU(5).
    pub fn su5() -> Self {
        Self {
            factors: vec![SimpleGroup::Su(5)],
        }
    }

    /// SO(10) GUT.
    pub fn so10() -> Self {
        Self {
            factors: vec![SimpleGroup::So(10)],
        }
    }

    /// E₆ GUT.
    pub fn e6() -> Self {
        Self {
            factors: vec![SimpleGroup::E6],
        }
    }

    /// Heterotic E₈×E₈.
    pub fn e8e8() -> Self {
        Self {
            factors: vec![SimpleGroup::E8, SimpleGroup::E8],
        }
    }

    /// A single E₈. Not a 10D Green–Schwarz solution (dimension 248 ≠ 496).
    pub fn e8() -> Self {
        Self {
            factors: vec![SimpleGroup::E8],
        }
    }

    /// Heterotic / Type I SO(32).
    pub fn so32() -> Self {
        Self {
            factors: vec![SimpleGroup::So(32)],
        }
    }

    /// A single SO(16). Not a 10D Green–Schwarz solution (dimension 120 ≠ 496).
    pub fn so16() -> Self {
        Self {
            factors: vec![SimpleGroup::So(16)],
        }
    }

    /// Spin(10), a common "geometry yields this" hypothesis.
    pub fn spin10() -> Self {
        Self {
            factors: vec![SimpleGroup::Spin(10)],
        }
    }

    /// Display name.
    pub fn name(&self) -> String {
        if self.factors.is_empty() {
            return "1".into();
        }
        self.factors
            .iter()
            .map(|f| f.name())
            .collect::<Vec<_>>()
            .join(" × ")
    }

    /// Rank (sum of factor ranks).
    pub fn rank(&self) -> Option<u32> {
        self.factors
            .iter()
            .map(|f| f.rank().map(|r| r as u32))
            .sum()
    }

    /// Total dimension (sum of factor dimensions), when all factors are valid.
    pub fn dimension(&self) -> Option<u32> {
        self.factors.iter().map(|f| f.dimension()).sum()
    }

    /// Whether this group cancels the 10D N=1 anomaly via Green–Schwarz.
    ///
    /// Green–Schwarz cancellation in ten-dimensional N=1 supergravity coupled
    /// to super-Yang–Mills requires a dimension-496 gauge group with the right
    /// trace identities. The non-abelian solutions realized by consistent
    /// string constructions are SO(32) (Type I, heterotic) and E₈×E₈
    /// (heterotic). Encoded as a textbook fact — this is *not* a re-derivation
    /// of the anomaly polynomial, which is a later milestone.
    pub fn gs_anomaly_free_10d(&self) -> bool {
        self.dimension() == Some(496) && (self == &Self::so32() || self == &Self::e8e8())
    }

    /// Verify by code that the Standard Model embeds in this group, returning
    /// the maximal-subgroup chain that witnesses it.
    ///
    /// A group contains the SM if any of its simple factors reduces to the SM
    /// through the standard chain (e.g. E₈ ⊃ E₆ ⊃ SO(10) ⊃ SU(5) ⊃ SM), with
    /// the necessary rank and dimension inequalities checked at each step. This
    /// replaces the old "is this group literally SU(5)?" equality table.
    pub fn verified_contains_sm(&self) -> Option<Vec<String>> {
        if self == &Self::standard_model() {
            return Some(vec![Self::standard_model().name()]);
        }
        self.factors.iter().find_map(|f| f.sm_embedding_chain())
    }

    /// How the Standard Model sits in this group, verified by the maximal-
    /// subgroup chain rather than stored as an equality table.
    pub fn sm_embed(&self) -> Embed {
        if self == &Self::standard_model() {
            return Embed::Equal;
        }
        if self.verified_contains_sm().is_some() {
            return Embed::KnownEmbedding;
        }
        if self.factors.is_empty() {
            Embed::None
        } else if self
            .factors
            .iter()
            .any(|f| matches!(f, SimpleGroup::Su(3) | SimpleGroup::Su(2) | SimpleGroup::U1))
        {
            Embed::Partial
        } else {
            Embed::Unknown
        }
    }
}

/// How G contains the Standard Model gauge group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Embed {
    /// G is exactly SM.
    Equal,
    /// A standard embedding G ⊃ SM is in the literature and encoded here.
    KnownEmbedding,
    /// Some SM factors present, not a complete embedding.
    Partial,
    /// No embedding.
    None,
    /// Not in the table; do not guess.
    Unknown,
}

impl Embed {
    /// True if SM can sit inside G according to this encoding.
    pub const fn contains_sm(self) -> bool {
        matches!(self, Embed::Equal | Embed::KnownEmbedding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sm_equals_itself() {
        assert_eq!(GaugeGroup::standard_model().sm_embed(), Embed::Equal);
        assert_eq!(GaugeGroup::e8e8().sm_embed(), Embed::KnownEmbedding);
        assert_eq!(GaugeGroup::trivial().sm_embed(), Embed::None);
    }

    #[test]
    fn su3_rank() {
        assert_eq!(SimpleGroup::Su(3).rank(), Some(2));
        assert_eq!(SimpleGroup::E8.rank(), Some(8));
    }

    #[test]
    fn group_dimensions() {
        assert_eq!(SimpleGroup::Su(3).dimension(), Some(8));
        assert_eq!(SimpleGroup::E8.dimension(), Some(248));
        assert_eq!(SimpleGroup::So(32).dimension(), Some(496));
        assert_eq!(GaugeGroup::e8e8().dimension(), Some(496));
        assert_eq!(GaugeGroup::so32().dimension(), Some(496));
        assert_eq!(GaugeGroup::standard_model().dimension(), Some(12));
    }

    #[test]
    fn sm_embedding_is_verified_by_chain_not_equality() {
        // Each GUT group verifies SM containment by walking a real chain.
        for g in [
            GaugeGroup::su5(),
            GaugeGroup::so10(),
            GaugeGroup::e6(),
            GaugeGroup::e8e8(),
            GaugeGroup::so32(),
            GaugeGroup::spin10(),
        ] {
            let chain = g
                .verified_contains_sm()
                .unwrap_or_else(|| panic!("{} should verify SM containment", g.name()));
            // The chain must terminate at the Standard Model.
            assert_eq!(chain.last().unwrap(), &GaugeGroup::standard_model().name());
            assert!(g.sm_embed().contains_sm());
        }

        // The canonical Georgi–Glashow chain, spelled out.
        assert_eq!(
            GaugeGroup::e6().verified_contains_sm().unwrap(),
            vec![
                "E6".to_string(),
                "SO(10)".to_string(),
                "SU(5)".to_string(),
                "SU(3) × SU(2) × U(1)".to_string(),
            ]
        );
    }

    #[test]
    fn groups_without_an_sm_chain_do_not_verify() {
        for g in [
            GaugeGroup::trivial(),
            GaugeGroup {
                factors: vec![SimpleGroup::G2],
            },
            GaugeGroup {
                factors: vec![SimpleGroup::F4],
            },
            GaugeGroup {
                factors: vec![SimpleGroup::So(9)],
            },
        ] {
            assert!(g.verified_contains_sm().is_none(), "{}", g.name());
            assert!(!g.sm_embed().contains_sm());
        }
    }

    #[test]
    fn embedding_chain_rank_and_dimension_are_monotonic() {
        // Necessary conditions: each subgroup has rank and dimension no larger
        // than its parent, all the way down to the SM.
        let names = GaugeGroup::e8e8().verified_contains_sm().unwrap();
        assert_eq!(names.first().unwrap(), "E8");
        assert_eq!(names.last().unwrap(), &GaugeGroup::standard_model().name());
    }

    #[test]
    fn green_schwarz_solutions_are_exactly_so32_and_e8e8() {
        // The two consistent 10D N=1 gauge groups.
        assert!(GaugeGroup::so32().gs_anomaly_free_10d());
        assert!(GaugeGroup::e8e8().gs_anomaly_free_10d());

        // Everything else must fail — including a plausible-looking "fake"
        // gauge choice. Green–Schwarz is the reason, not a menu.
        for g in [
            GaugeGroup::standard_model(),
            GaugeGroup::su5(),
            GaugeGroup::so10(),
            GaugeGroup::e6(),
            GaugeGroup::e8(),
            GaugeGroup::so16(),
            GaugeGroup::trivial(),
            GaugeGroup {
                factors: vec![SimpleGroup::Su(3)],
            },
        ] {
            assert!(
                !g.gs_anomaly_free_10d(),
                "{} must not be a GS solution",
                g.name()
            );
        }
        assert!(
            GaugeGroup::e8().sm_embed().contains_sm(),
            "a single E8 still contains SM; GS is the dimension-496 identity"
        );
        assert!(
            GaugeGroup::so16().sm_embed().contains_sm(),
            "SO(16) still contains SM via SO(10); GS is the dimension-496 identity"
        );
    }
}
