//! A small scientific IR. Not a replacement for Lean. Points at a formal
//! backend for sophisticated mathematics.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use physis_core::assurance::ClaimClass;
use physis_core::id::LayerId;
use serde::{Deserialize, Serialize};

/// A declarative theory package (the contents of `theory.toml` plus
/// `.physis` fragments, parsed).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TheoryPackage {
    /// Package id (`my-theory`).
    pub id: String,
    /// Human title.
    pub name: String,
    /// Parameters.
    pub parameters: Vec<ParameterDecl>,
    /// Assumptions.
    pub assumptions: Vec<String>,
    /// Equations (as text; dimensional check is later).
    pub equations: Vec<String>,
    /// Claims.
    pub claims: Vec<ClaimDecl>,
    /// Optional Lean/Physlib theorem reference.
    pub lean_ref: Option<String>,
}

/// A declared parameter.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterDecl {
    /// Name.
    pub name: String,
    /// Origin: `fundamental-input`, `fitted`, …
    pub origin: String,
}

/// A declared claim.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimDecl {
    /// Id.
    pub id: String,
    /// Statement.
    pub statement: String,
    /// Layer name.
    pub layer: String,
    /// Claim class name.
    pub class: String,
}

/// Parse a minimal `theory.toml`-like listing.
///
/// Format (line oriented, not full TOML — agents can emit this without a
/// TOML crate in the trusted kernel):
///
/// ```text
/// id = my-theory
/// name = My Theory
/// parameter fundamental-input alpha
/// assumption locality
/// equation dF = 0
/// claim mathematical spacetime math.d2 : d^2 = 0
/// lean_ref Physlib.Relativity.Interval.invariant
/// ```
pub fn parse_package(src: &str) -> Result<TheoryPackage, String> {
    let mut pkg = TheoryPackage {
        id: String::new(),
        name: String::new(),
        parameters: Vec::new(),
        assumptions: Vec::new(),
        equations: Vec::new(),
        claims: Vec::new(),
        lean_ref: None,
    };
    for (i, line) in src.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (key, rest) = line
            .split_once(' ')
            .or_else(|| line.split_once('='))
            .ok_or_else(|| format!("L{}: expected key value", i + 1))?;
        let key = key.trim().trim_end_matches('=');
        let rest = rest.trim().trim_start_matches('=').trim();
        match key {
            "id" => pkg.id = rest.to_string(),
            "name" => pkg.name = rest.to_string(),
            "parameter" => {
                let (origin, name) = rest
                    .split_once(' ')
                    .ok_or_else(|| format!("L{}: parameter <origin> <name>", i + 1))?;
                pkg.parameters.push(ParameterDecl {
                    origin: origin.to_string(),
                    name: name.to_string(),
                });
            }
            "assumption" => pkg.assumptions.push(rest.to_string()),
            "equation" => pkg.equations.push(rest.to_string()),
            "claim" => {
                // claim <class> <layer> <id> : <statement>
                let (head, stmt) = rest
                    .split_once(':')
                    .ok_or_else(|| format!("L{}: claim class layer id : statement", i + 1))?;
                let parts: Vec<&str> = head.split_whitespace().collect();
                if parts.len() != 3 {
                    return Err(format!("L{}: claim class layer id : statement", i + 1));
                }
                pkg.claims.push(ClaimDecl {
                    class: parts[0].into(),
                    layer: parts[1].into(),
                    id: parts[2].into(),
                    statement: stmt.trim().into(),
                });
            }
            "lean_ref" => pkg.lean_ref = Some(rest.to_string()),
            other => return Err(format!("L{}: unknown key '{other}'", i + 1)),
        }
    }
    if pkg.id.is_empty() {
        return Err("missing id".into());
    }
    Ok(pkg)
}

/// Known layer names (rejects typos rather than inventing a layer).
pub fn parse_layer(s: &str) -> Result<LayerId, String> {
    LayerId::ALL
        .iter()
        .copied()
        .find(|l| l.as_str() == s)
        .ok_or_else(|| format!("unknown layer '{s}'"))
}

/// Known class names.
pub fn parse_class(s: &str) -> Result<ClaimClass, String> {
    use ClaimClass::*;
    let c = match s {
        "mathematical" => Mathematical,
        "model-internal" => ModelInternal,
        "phenomenological" => Phenomenological,
        "empirical-prediction" => EmpiricalPrediction,
        "measurement" => Measurement,
        "conjecture" => Conjecture,
        "heuristic" => Heuristic,
        "open-problem" => OpenProblem,
        _ => return Err(format!("unknown class '{s}'")),
    };
    Ok(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fork_add_claim_round_trip() {
        let src = r#"
id = my-theory
name = A fork of Maxwell
parameter fundamental-input epsilon0
assumption locality
equation dF = 0
claim mathematical spacetime dec.d-squared-zero : d^2 = 0
lean_ref Physlib.Exterior.d_squared
"#;
        let pkg = parse_package(src).unwrap();
        assert_eq!(pkg.id, "my-theory");
        assert_eq!(pkg.parameters[0].name, "epsilon0");
        assert_eq!(pkg.claims[0].id, "dec.d-squared-zero");
        assert!(parse_class(&pkg.claims[0].class).is_ok());
        assert!(parse_layer("spacetime").is_ok());
        assert!(parse_layer("not-a-layer").is_err());
    }
}
