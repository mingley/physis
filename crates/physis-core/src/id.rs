//! Stable identifiers for layers, knobs, claims, and theories.

use std::fmt;

use serde::{Deserialize, Serialize};

/// A stratum of description, from mathematical substrate to observers.
///
/// Layers are not a tower of "more true" — they are scales of *mechanism*.
/// A claim always lives on a layer. Changing a knob on one layer may
/// force verdict changes on others; that coupling is the point.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LayerId {
    /// Groups, algebras, categories, type-level structure.
    Mathematical,
    /// Manifolds, metrics, signature, dimension.
    Spacetime,
    /// Hilbert spaces, operators, amplitudes.
    Quantum,
    /// Sections of bundles, Lagrangians, local degrees of freedom.
    Field,
    /// Irreducible representations: the particle spectrum.
    Particle,
    /// Gauge groups, couplings, vertices.
    Interaction,
    /// Coarse-grained / effective descriptions.
    Effective,
    /// Ensembles, temperature, entropy.
    Statistical,
    /// Information, computation, measurement records.
    Information,
    /// Agents, experiments, journals.
    Agent,
}

impl LayerId {
    /// All layers, finest first.
    pub const ALL: [LayerId; 10] = [
        LayerId::Mathematical,
        LayerId::Spacetime,
        LayerId::Quantum,
        LayerId::Field,
        LayerId::Particle,
        LayerId::Interaction,
        LayerId::Effective,
        LayerId::Statistical,
        LayerId::Information,
        LayerId::Agent,
    ];

    /// Stable kebab-case name.
    pub const fn as_str(self) -> &'static str {
        match self {
            LayerId::Mathematical => "mathematical",
            LayerId::Spacetime => "spacetime",
            LayerId::Quantum => "quantum",
            LayerId::Field => "field",
            LayerId::Particle => "particle",
            LayerId::Interaction => "interaction",
            LayerId::Effective => "effective",
            LayerId::Statistical => "statistical",
            LayerId::Information => "information",
            LayerId::Agent => "agent",
        }
    }

    /// One-line description of what this layer is *for*.
    pub const fn doc(self) -> &'static str {
        match self {
            LayerId::Mathematical => "Typed structure: groups, algebras, dimensions, proofs.",
            LayerId::Spacetime => "Where and when: dimension, signature, metric, extra directions.",
            LayerId::Quantum => "Amplitudes and operators on Hilbert space.",
            LayerId::Field => "Local degrees of freedom and their Lagrangians.",
            LayerId::Particle => "Spectrum: spins, charges, masses, empirical status.",
            LayerId::Interaction => "Gauge groups, couplings, allowed vertices.",
            LayerId::Effective => "Descriptions valid only below a cutoff scale.",
            LayerId::Statistical => "Ensembles, temperature, thermodynamic knobs.",
            LayerId::Information => "Records, computation, entropy of states.",
            LayerId::Agent => "Observers who turn knobs and write journals.",
        }
    }
}

impl fmt::Display for LayerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Opaque claim identifier, `theory.slug` form by convention.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClaimId(pub String);

impl ClaimId {
    /// Construct from anything displayable.
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl fmt::Display for ClaimId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Opaque knob identifier, `layer.name` form by convention.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KnobId(pub String);

impl KnobId {
    /// Construct from anything displayable.
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl fmt::Display for KnobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Opaque theory identifier (`type-iib`, `standard-model`, …).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TheoryId(pub String);

impl TheoryId {
    /// Construct from anything displayable.
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
}

impl fmt::Display for TheoryId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
