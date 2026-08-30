//! In-tree Physlib source. The verifier still treats it as untrusted bytes.

/// The Physlib module checked by Lean kernel + nanoda.
pub const PHYSLIB_SOURCE: &str = include_str!("../../../formal/physlib/Physlib.lean");

/// Collapse type strings so catalog `β*x` matches Lean `β * x` without
/// merging identifiers (`a b c` stays three binders).
pub fn compact_lean_type(s: &str) -> String {
    let spaced: String = s.split_whitespace().collect::<Vec<_>>().join(" ");
    let chars: Vec<char> = spaced.chars().collect();
    let ops = ['*', '^', '+', '-', '=', ',', '(', ')', ':'];
    let mut out = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if c == ' ' {
            let prev = i.checked_sub(1).and_then(|j| chars.get(j).copied());
            let next = chars.get(i + 1).copied();
            if prev.is_some_and(|p| ops.contains(&p)) || next.is_some_and(|n| ops.contains(&n)) {
                continue;
            }
        }
        out.push(c);
    }
    out
}

/// One `theorem` extracted from Lean source (conservative, not a parser).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExtractedTheorem {
    /// Theorem name.
    pub name: String,
    /// Reconstructed `∀ binders, type`.
    pub ty: String,
}

/// Extract `theorem` names and types. Fails closed on source it cannot read.
pub fn extract_theorems(source: &str) -> Vec<ExtractedTheorem> {
    let mut out = Vec::new();
    for (i, _) in source.match_indices("theorem ") {
        if i > 0 {
            if let Some(before) = source[..i].chars().last() {
                if before.is_ascii_alphanumeric() || before == '_' {
                    continue;
                }
            }
        }
        let after_kw = &source[i + "theorem ".len()..];
        let after_name = after_kw.trim_start();
        let name_len = after_name
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '.'))
            .unwrap_or(after_name.len());
        if name_len == 0 {
            continue;
        }
        let name = after_name[..name_len].to_string();
        let after = &after_name[name_len..];
        let Some((binders, ty_body)) = split_binders_and_type(after) else {
            continue;
        };
        let ty = if binders.is_empty() {
            ty_body.to_string()
        } else {
            format!("∀ {binders}, {ty_body}")
        };
        out.push(ExtractedTheorem { name, ty });
    }
    out
}

/// Split ` (a b c : Int) : type :=` at the last colon that is not inside
/// parentheses. The first `:` in a binder list is not the type separator.
fn split_binders_and_type(after_name: &str) -> Option<(&str, &str)> {
    let s = after_name.trim_start();
    let colon_eq = s.find(":=")?;
    let before = &s[..colon_eq];
    let mut depth = 0i32;
    let mut last_colon = None;
    for (idx, c) in before.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ':' if depth == 0 => last_colon = Some(idx),
            _ => {}
        }
    }
    let colon = last_colon?;
    let binders = before[..colon].trim();
    let ty_body = before[colon + 1..].trim();
    if ty_body.is_empty() {
        return None;
    }
    Some((binders, ty_body))
}

/// True when the source contains a theorem whose compacted type matches.
pub fn source_matches_challenge(source: &str, lean_type: &str) -> bool {
    let want = compact_lean_type(lean_type);
    extract_theorems(source)
        .iter()
        .any(|t| compact_lean_type(&t.ty) == want)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::CATALOG;

    #[test]
    fn physlib_matches_every_catalog_lean_type() {
        for spec in CATALOG {
            assert!(
                source_matches_challenge(PHYSLIB_SOURCE, spec.lean_type),
                "{} type {} not in Physlib; extracted {:?}",
                spec.claim_id,
                spec.lean_type,
                extract_theorems(PHYSLIB_SOURCE)
                    .iter()
                    .map(|t| (t.name.as_str(), compact_lean_type(&t.ty)))
                    .collect::<Vec<_>>()
            );
            assert!(
                extract_theorems(PHYSLIB_SOURCE)
                    .iter()
                    .any(|t| t.name == spec.lean_theorem),
                "missing theorem {}",
                spec.lean_theorem
            );
        }
    }

    #[test]
    fn true_is_not_d2() {
        let src = "theorem T : True := trivial\n";
        assert!(!source_matches_challenge(
            src,
            "∀ (a b c : Int), (b - a) - (c - a) + (c - b) = 0"
        ));
    }
}
