//! A small scientific IR. Not a replacement for Lean. Points at a formal
//! backend for sophisticated mathematics. Packages round-trip through
//! [`parse_package`] / [`render_package`]; [`certify_round_trip`] is an
//! independent parse check, not a kernel proof. [`apply_mutation`] is a
//! constrained encoding fork, not a kernel proof.

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

/// Render a package in the line-oriented dialect [`parse_package`] accepts.
pub fn render_package(pkg: &TheoryPackage) -> String {
    let mut out = String::new();
    out.push_str(&format!("id = {}\n", pkg.id));
    if !pkg.name.is_empty() {
        out.push_str(&format!("name = {}\n", pkg.name));
    }
    for p in &pkg.parameters {
        out.push_str(&format!("parameter {} {}\n", p.origin, p.name));
    }
    for a in &pkg.assumptions {
        out.push_str(&format!("assumption {a}\n"));
    }
    for e in &pkg.equations {
        out.push_str(&format!("equation {e}\n"));
    }
    for c in &pkg.claims {
        out.push_str(&format!(
            "claim {} {} {} : {}\n",
            c.class, c.layer, c.id, c.statement
        ));
    }
    if let Some(r) = &pkg.lean_ref {
        out.push_str(&format!("lean_ref {r}\n"));
    }
    out
}

/// Constrained structural edits of a package. These are encoding forks,
/// not knobs and not a kernel proof.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PackageMutation {
    /// Append one equation line (a netlist gate, an identity, …).
    AppendEquation(String),
    /// Drop the first assumption, if any.
    DropAssumption,
    /// Replace the first ` - ` in the first equation with ` + ` (a sign flip).
    FlipFirstMinus,
}

/// Apply a constrained mutation. The result is a different package; it is
/// not automatically a live lab theory and it does not mint.
pub fn apply_mutation(pkg: &TheoryPackage, mutation: &PackageMutation) -> TheoryPackage {
    let mut out = pkg.clone();
    match mutation {
        PackageMutation::AppendEquation(eq) => out.equations.push(eq.clone()),
        PackageMutation::DropAssumption => {
            if !out.assumptions.is_empty() {
                out.assumptions.remove(0);
            }
        }
        PackageMutation::FlipFirstMinus => {
            if let Some(eq) = out.equations.first_mut() {
                if let Some(i) = eq.find(" - ") {
                    eq.replace_range(i..i + 3, " + ");
                }
            }
        }
    }
    out
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

/// Independently parse and round-trip a package through the line dialect.
///
/// Returns the canonical render when `parse(render(pkg))` equals `pkg`,
/// the second render is stable, and every claim names a known class and
/// layer. This is not a kernel proof, not semantic review, and not a
/// mutation.
pub fn certify_round_trip(pkg: &TheoryPackage) -> Result<String, String> {
    if pkg.id.is_empty() {
        return Err("missing id".into());
    }
    let rendered = render_package(pkg);
    let parsed = parse_package(&rendered)?;
    if parsed != *pkg {
        return Err("round-trip parse does not match the live package".into());
    }
    let again = render_package(&parsed);
    if again != rendered {
        return Err("render is not canonical".into());
    }
    for c in &pkg.claims {
        parse_class(&c.class)?;
        parse_layer(&c.layer)?;
    }
    Ok(rendered)
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
        let again = parse_package(&render_package(&pkg)).unwrap();
        assert_eq!(again, pkg);
    }

    #[test]
    fn sign_flip_is_a_new_package() {
        let pkg = parse_package(
            "id = discrete-d2\nequation (b - a) - (c - a) + (c - b)\nassumption coboundary\n",
        )
        .unwrap();
        let flipped = apply_mutation(&pkg, &PackageMutation::FlipFirstMinus);
        assert_ne!(flipped.equations[0], pkg.equations[0]);
        assert!(flipped.equations[0].contains("b + a"));
        let dropped = apply_mutation(&pkg, &PackageMutation::DropAssumption);
        assert!(dropped.assumptions.is_empty());
        let extra = apply_mutation(
            &pkg,
            &PackageMutation::AppendEquation("nand 2 2 -> 0".into()),
        );
        assert_eq!(extra.equations.len(), 2);
        assert_eq!(
            parse_package(&render_package(&extra)).unwrap().equations,
            extra.equations
        );
    }

    #[test]
    fn certify_round_trip_accepts_a_stable_package() {
        let pkg = parse_package(
            "id = combinational-circuit\n\
             name = Combinational circuit\n\
             assumption finite-nand-netlist\n\
             equation nand 0 1 -> 2\n\
             claim model-internal information comp.acyclic : The gate graph is acyclic.\n",
        )
        .unwrap();
        let canonical = certify_round_trip(&pkg).unwrap();
        assert_eq!(parse_package(&canonical).unwrap(), pkg);
        assert!(canonical.contains("nand 0 1 -> 2"));
    }

    #[test]
    fn certify_round_trip_rejects_an_unknown_class() {
        let mut pkg = parse_package("id = fork\nequation nand 0 1 -> 2\n").unwrap();
        pkg.claims.push(ClaimDecl {
            id: "x".into(),
            statement: "nope".into(),
            layer: "information".into(),
            class: "not-a-class".into(),
        });
        let err = certify_round_trip(&pkg).unwrap_err();
        assert!(err.contains("unknown class"), "{err}");
    }
}
