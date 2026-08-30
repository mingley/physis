//! Knobs: named, typed, domain-bounded parameters of a system.
//!
//! A knob is the thing an agent turns. The type system plus domain checks
//! make the turn *mechanical*: illegal values are rejected, legal values
//! produce downstream verdict changes that the lab can diff.

use serde::{Deserialize, Serialize};

use crate::error::CoreError;
use crate::id::LayerId;
use crate::judgment::ParameterOrigin;

/// Runtime value of a knob.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "kebab-case")]
pub enum KnobValue {
    /// Boolean switch.
    Bool(bool),
    /// Signed integer.
    Int(i64),
    /// Unsigned integer (dimensions, counts).
    UInt(u64),
    /// Floating point in a documented unit.
    Float(f64),
    /// Discrete named choice.
    Choice(String),
}

impl KnobValue {
    /// Kind name for error messages.
    pub fn kind_name(&self) -> &'static str {
        match self {
            KnobValue::Bool(_) => "bool",
            KnobValue::Int(_) => "int",
            KnobValue::UInt(_) => "uint",
            KnobValue::Float(_) => "float",
            KnobValue::Choice(_) => "choice",
        }
    }

    /// Human display.
    pub fn display(&self) -> String {
        match self {
            KnobValue::Bool(v) => v.to_string(),
            KnobValue::Int(v) => v.to_string(),
            KnobValue::UInt(v) => v.to_string(),
            KnobValue::Float(v) => format!("{v}"),
            KnobValue::Choice(v) => v.clone(),
        }
    }

    /// Parse a CLI/agent token into a value of the given domain.
    pub fn parse_in_domain(raw: &str, domain: &KnobDomain) -> Result<KnobValue, CoreError> {
        match domain {
            KnobDomain::Bool => {
                let v = match raw.to_ascii_lowercase().as_str() {
                    "true" | "1" | "yes" | "on" => true,
                    "false" | "0" | "no" | "off" => false,
                    _ => {
                        return Err(CoreError::TypeMismatch {
                            name: String::new(),
                            expected: "bool".into(),
                            got: raw.into(),
                        });
                    }
                };
                Ok(KnobValue::Bool(v))
            }
            KnobDomain::Int { min, max } => {
                let v: i64 = raw.parse().map_err(|_| CoreError::TypeMismatch {
                    name: String::new(),
                    expected: "int".into(),
                    got: raw.into(),
                })?;
                if v < *min || v > *max {
                    return Err(CoreError::Domain {
                        name: String::new(),
                        reason: format!("{v} not in [{min}, {max}]"),
                    });
                }
                Ok(KnobValue::Int(v))
            }
            KnobDomain::UInt { min, max } => {
                let v: u64 = raw.parse().map_err(|_| CoreError::TypeMismatch {
                    name: String::new(),
                    expected: "uint".into(),
                    got: raw.into(),
                })?;
                if v < *min || v > *max {
                    return Err(CoreError::Domain {
                        name: String::new(),
                        reason: format!("{v} not in [{min}, {max}]"),
                    });
                }
                Ok(KnobValue::UInt(v))
            }
            KnobDomain::Float { min, max } => {
                let v: f64 = raw.parse().map_err(|_| CoreError::TypeMismatch {
                    name: String::new(),
                    expected: "float".into(),
                    got: raw.into(),
                })?;
                if v < *min || v > *max {
                    return Err(CoreError::Domain {
                        name: String::new(),
                        reason: format!("{v} not in [{min}, {max}]"),
                    });
                }
                Ok(KnobValue::Float(v))
            }
            KnobDomain::Choice(options) => {
                if options.contains(&raw) {
                    Ok(KnobValue::Choice(raw.to_string()))
                } else {
                    Err(CoreError::Domain {
                        name: String::new(),
                        reason: format!("'{raw}' not in {options:?}"),
                    })
                }
            }
        }
    }
}

/// Allowed set of values for a knob.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum KnobDomain {
    /// `true` / `false`.
    Bool,
    /// Inclusive integer range.
    Int {
        /// Minimum.
        min: i64,
        /// Maximum.
        max: i64,
    },
    /// Inclusive unsigned range.
    UInt {
        /// Minimum.
        min: u64,
        /// Maximum.
        max: u64,
    },
    /// Inclusive float range.
    Float {
        /// Minimum.
        min: f64,
        /// Maximum.
        max: f64,
    },
    /// Named discrete options.
    Choice(&'static [&'static str]),
}

impl KnobDomain {
    /// Kind name matching [`KnobValue::kind_name`].
    pub fn kind_name(&self) -> &'static str {
        match self {
            KnobDomain::Bool => "bool",
            KnobDomain::Int { .. } => "int",
            KnobDomain::UInt { .. } => "uint",
            KnobDomain::Float { .. } => "float",
            KnobDomain::Choice(_) => "choice",
        }
    }

    /// Reject a value that is the wrong kind or out of range.
    pub fn check(&self, name: &str, value: &KnobValue) -> Result<(), CoreError> {
        match (self, value) {
            (KnobDomain::Bool, KnobValue::Bool(_)) => Ok(()),
            (KnobDomain::Int { min, max }, KnobValue::Int(v)) => {
                if v < min || v > max {
                    Err(CoreError::Domain {
                        name: name.into(),
                        reason: format!("{v} not in [{min}, {max}]"),
                    })
                } else {
                    Ok(())
                }
            }
            (KnobDomain::UInt { min, max }, KnobValue::UInt(v)) => {
                if v < min || v > max {
                    Err(CoreError::Domain {
                        name: name.into(),
                        reason: format!("{v} not in [{min}, {max}]"),
                    })
                } else {
                    Ok(())
                }
            }
            (KnobDomain::Float { min, max }, KnobValue::Float(v)) => {
                if v < min || v > max || !v.is_finite() {
                    Err(CoreError::Domain {
                        name: name.into(),
                        reason: format!("{v} not in [{min}, {max}]"),
                    })
                } else {
                    Ok(())
                }
            }
            (KnobDomain::Choice(options), KnobValue::Choice(v)) => {
                if options.contains(&v.as_str()) {
                    Ok(())
                } else {
                    Err(CoreError::Domain {
                        name: name.into(),
                        reason: format!("'{v}' not in {options:?}"),
                    })
                }
            }
            (domain, got) => Err(CoreError::TypeMismatch {
                name: name.into(),
                expected: domain.kind_name().into(),
                got: got.kind_name().into(),
            }),
        }
    }
}

/// Static description of a knob (name, layer, domain, documentation).
#[derive(Clone, Debug, PartialEq)]
pub struct KnobSpec {
    /// Stable name, unique per theory.
    pub name: &'static str,
    /// Layer this knob belongs to.
    pub layer: LayerId,
    /// What turning this knob *means*.
    pub doc: &'static str,
    /// Whether this number is derived, measured, fitted, or chosen.
    pub origin: ParameterOrigin,
    /// Allowed values.
    pub domain: KnobDomain,
}

/// An object whose parameters are exposed as knobs.
pub trait Knobbed {
    /// Specs in a stable order.
    fn specs(&self) -> &'static [KnobSpec];

    /// Current value of a named knob.
    fn get(&self, name: &str) -> Result<KnobValue, CoreError>;

    /// Set a named knob. Returns the previous value.
    fn set(&mut self, name: &str, value: KnobValue) -> Result<KnobValue, CoreError>;

    /// All knobs with current values.
    fn snapshot(&self) -> Vec<(&'static KnobSpec, KnobValue)> {
        self.specs()
            .iter()
            .filter_map(|s| self.get(s.name).ok().map(|v| (s, v)))
            .collect()
    }

    /// Look up a spec by name.
    fn spec(&self, name: &str) -> Result<&'static KnobSpec, CoreError> {
        self.specs()
            .iter()
            .find(|s| s.name == name)
            .ok_or_else(|| CoreError::UnknownKnob { name: name.into() })
    }
}
