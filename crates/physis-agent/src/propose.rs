//! Untrusted candidate encodings.
//!
//! An agent may submit a dimensioned polynomial. Both expanders run, the
//! unit is checked against the theory's observable, and the record is
//! stored as an agent action. The baseline theory is not modified and
//! nothing here mints a receipt.

use physis_proof::expand::dual_expand;
use physis_proof::parse::parse_expr;

/// Closed observable units a candidate may name.
pub fn allowed_units(theory: &str) -> &'static [&'static str] {
    match theory {
        "newtonian-gravity" | "general-relativity" => &["arcsec", "arcsec cy-1"],
        "planck" | "rayleigh-jeans" => &["W m-2 K-4", "m"],
        "de-rham" => &["1"],
        _ => &["1"],
    }
}

/// Result of a dual expansion that has not been installed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    /// Theory the unit was checked against.
    pub theory: String,
    /// Declared unit.
    pub unit: String,
    /// Source expression.
    pub expression: String,
    /// Both expanders agreed this is the zero polynomial.
    pub identically_zero: bool,
}

/// Parse and dual-expand `expression` for `theory` in `unit`.
///
/// A unit outside [`allowed_units`] is refused. Expander disagreement is
/// refused. The returned record is not a receipt.
pub fn evaluate(theory: &str, unit: &str, expression: &str) -> Result<Candidate, String> {
    let allowed = allowed_units(theory);
    if !allowed.contains(&unit) {
        return Err(format!(
            "unit '{unit}' is not an observable of {theory} ({})",
            allowed.join(", ")
        ));
    }
    let expr = parse_expr(expression).map_err(|e| format!("candidate parse: {e}"))?;
    let identically_zero = dual_expand(&expr).map_err(|e| format!("candidate expand: {e}"))?;
    Ok(Candidate {
        theory: theory.to_string(),
        unit: unit.to_string(),
        expression: expression.to_string(),
        identically_zero,
    })
}

/// Canonical payload stored in the bundle and the lab action node.
pub fn payload(candidate: &Candidate) -> String {
    format!(
        "candidate-encoding\ntheory {}\nunit {}\nexpression {}\nidentically-zero {}\ninstalled false\nreceipt false\n",
        candidate.theory,
        candidate.unit,
        candidate.expression,
        candidate.identically_zero
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_variable_minus_itself_is_zero_in_arcsec() {
        let c = evaluate("newtonian-gravity", "arcsec", "(a - a)").unwrap();
        assert!(c.identically_zero);
        let body = payload(&c);
        assert!(body.contains("installed false"));
        assert!(body.contains("receipt false"));
    }

    #[test]
    fn a_nonzero_polynomial_is_stored_as_not_zero() {
        let c = evaluate("newtonian-gravity", "arcsec", "(a + 1)").unwrap();
        assert!(!c.identically_zero);
    }

    #[test]
    fn metre_is_refused_for_a_deflection_theory() {
        let err = evaluate("newtonian-gravity", "metre", "(a - a)").unwrap_err();
        assert!(err.contains("not an observable"), "{err}");
    }

    #[test]
    fn an_unparsable_expression_is_refused() {
        assert!(evaluate("de-rham", "1", "???").is_err());
    }

    #[test]
    fn propose_stores_the_record_and_does_not_install_it() {
        use crate::coordinator::Campaign;
        use crate::lab::Lab;
        use crate::protocol::Command;

        let mut lab = Lab::standard();
        let dir = std::env::temp_dir().join(format!("physis-propose-{}.d", std::process::id()));
        std::fs::remove_dir_all(&dir).ok();
        let text = crate::coordinator::test_manifest(
            &["newtonian-gravity"],
            &["gr.newton-half-deflection"],
            &["hypothesize"],
        );
        {
            let campaign = Campaign::open(&mut lab, &text, &dir).unwrap();
            drop(campaign);
        }
        let before = lab
            .theory("newtonian-gravity")
            .unwrap()
            .ir_package()
            .map(|p| physis_ir::render_package(&p));
        let out = lab.exec(Command::Propose {
            theory: "newtonian-gravity".into(),
            unit: "arcsec".into(),
            expression: "(a - a)".into(),
            campaign: dir.display().to_string(),
        });
        assert_eq!(out.exit_code(), 0, "{}", out.text());
        assert!(out.text().contains("installed false"), "{}", out.text());
        assert!(out.text().contains("receipt false"), "{}", out.text());
        assert!(
            out.text().contains("identically-zero true"),
            "{}",
            out.text()
        );
        let after = lab
            .theory("newtonian-gravity")
            .unwrap()
            .ir_package()
            .map(|p| physis_ir::render_package(&p));
        assert_eq!(before, after);
        let bad = lab.exec(Command::Propose {
            theory: "newtonian-gravity".into(),
            unit: "metre".into(),
            expression: "(a - a)".into(),
            campaign: dir.display().to_string(),
        });
        assert_eq!(bad.exit_code(), 1, "{}", bad.text());
        std::fs::remove_dir_all(&dir).ok();
    }
}
