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

    /// Heterotic / Type I SO(32).
    pub fn so32() -> Self {
        Self {
            factors: vec![SimpleGroup::So(32)],
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

    /// How the Standard Model sits in this group, as encoded textbook facts.
    pub fn sm_embed(&self) -> Embed {
        if self == &Self::standard_model() {
            return Embed::Equal;
        }
        // Known chain: SM ⊂ SU(5) ⊂ SO(10) ⊂ E6 ⊂ E8, and SM ⊂ SO(32), SM ⊂ E8×E8.
        let facts = [
            (Self::su5(), Embed::KnownEmbedding),
            (Self::so10(), Embed::KnownEmbedding),
            (Self::e6(), Embed::KnownEmbedding),
            (Self::e8e8(), Embed::KnownEmbedding),
            (Self::so32(), Embed::KnownEmbedding),
            (Self::spin10(), Embed::KnownEmbedding),
        ];
        for (g, e) in facts {
            if self == &g {
                return e;
            }
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
}
