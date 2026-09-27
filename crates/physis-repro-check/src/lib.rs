//! Independent integer-identity checker (C6.2, spec 021).
//!
//! This crate intentionally links no `physis-*` crate: its parser and its
//! grid-evaluation algorithm are implemented here, independently of the
//! production infix parser, catalog expression constructors, and
//! recursive/postfix expanders. Shared with production: only the wire
//! format documented in `WIRE.md`, standard serialization (`serde`,
//! `serde_json`), and the hash library (`sha2`). See `RATIONALE.md` for
//! the algorithm-independence argument and declared common-mode risks,
//! and `ALLOWLIST.md` for the maintainer-reviewed assessment record.
//!
//! [`check`] is total: every input maps to a [`Verdict`]. There is no
//! `Holds` to read, no receipt to trust, and no network, clock, or
//! environment input beyond the obligation bytes and `WORKER_EXE_DIGEST`.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;

/// Obligation schema tag.
pub const OBLIGATION_SCHEMA: &str = "physis-int-obligation";
/// Obligation schema version.
pub const OBLIGATION_VERSION: u64 = 1;
/// Verdict schema tag.
pub const VERDICT_SCHEMA: &str = "physis-int-verdict";
/// Verdict schema version.
pub const VERDICT_VERSION: u64 = 1;
/// Checker identity echoed in verdicts.
pub const CHECKER_ID: &str = "dec-int-checker";
/// Checker version echoed in verdicts.
pub const CHECKER_VERSION: u64 = 1;
/// Largest obligation file accepted, in bytes.
pub const MAX_OBLIGATION_BYTES: usize = 1_000_000;
/// Deepest accepted expression tree. Small enough to stay well under
/// the JSON recursion limit at every layer (each tree level spans
/// several JSON levels) and to keep recursive evaluation shallow.
pub const MAX_TREE_DEPTH: usize = 32;
/// Most accepted tree nodes.
pub const MAX_TREE_NODES: usize = 100_000;
/// Most accepted symbols.
pub const MAX_SYMBOLS: usize = 16;
/// Most grid points evaluated before giving up.
pub const MAX_GRID_POINTS: u64 = 1_000_000;

/// Parsed obligation: what the checker was asked to decide.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Obligation {
    /// Opaque run binding, echoed in the verdict.
    pub run_id: String,
    /// Declared symbols, sorted and unique.
    pub symbols: Vec<String>,
    /// Claimed-zero expression tree.
    pub tree: Tree,
}

/// Integer expression tree. Deliberately structural: no sharing, no
/// interning, no simplification. Evaluation is direct.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tree {
    /// Integer constant.
    Const(i128),
    /// Declared symbol.
    Var(String),
    /// Sum.
    Add(Box<Tree>, Box<Tree>),
    /// Difference.
    Sub(Box<Tree>, Box<Tree>),
    /// Product.
    Mul(Box<Tree>, Box<Tree>),
    /// Negation.
    Neg(Box<Tree>),
    /// Base to a non-negative integer power.
    Pow(Box<Tree>, u32),
}

/// Checker disposition. `execution-failed` exists only on the host side:
/// the checker either writes one of these or writes nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disposition {
    /// The exact obligation checked out on the full grid.
    Agree,
    /// Well-formed obligation with a counterexample witness.
    Disagree,
    /// Schema or obligation kind the checker does not implement.
    Unsupported,
    /// Malformed, tampered, or mismatched input.
    Invalid,
    /// A cap (grid, overflow, depth) stopped evaluation. Unresolved,
    /// never disproof.
    ResourceExhausted,
}

impl Disposition {
    /// Wire spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Disposition::Agree => "agree",
            Disposition::Disagree => "disagree",
            Disposition::Unsupported => "unsupported",
            Disposition::Invalid => "invalid",
            Disposition::ResourceExhausted => "resource-exhausted",
        }
    }
}

/// Verdict the checker writes. All bindings the host re-verifies.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Verdict {
    /// Echo of the obligation's run id.
    pub run_id: String,
    /// SHA-256 of the obligation bytes as read, hex.
    pub obligation_sha256: String,
    /// Echo of `WORKER_EXE_DIGEST` (empty when unset).
    pub exe_sha256: String,
    /// Disposition.
    pub disposition: Disposition,
    /// Human diagnostic; the `disagree` witness in text form.
    pub detail: String,
    /// Grid points evaluated.
    pub evaluated_points: u64,
    /// Syntactic degree used, when computed.
    pub grid_degree: Option<u64>,
    /// First counterexample, when found.
    pub witness: Option<Vec<(String, i128)>>,
}

/// Check obligation bytes to a verdict. Total: malformed input, overflow,
/// and unsupported kinds all map to verdicts, never panics.
pub fn check(obligation_bytes: &[u8], exe_digest: &str) -> Verdict {
    let obligation_sha256 = hex_sha256(obligation_bytes);
    let mut base = Verdict {
        run_id: String::new(),
        obligation_sha256,
        exe_sha256: exe_digest.to_string(),
        disposition: Disposition::Invalid,
        detail: String::new(),
        evaluated_points: 0,
        grid_degree: None,
        witness: None,
    };
    if obligation_bytes.len() > MAX_OBLIGATION_BYTES {
        base.detail = format!(
            "obligation is {} bytes, cap is {MAX_OBLIGATION_BYTES}",
            obligation_bytes.len()
        );
        return base;
    }
    let text = match std::str::from_utf8(obligation_bytes) {
        Ok(t) => t,
        Err(_) => {
            base.detail = "obligation is not utf-8".into();
            return base;
        }
    };
    if let Err(e) = reject_duplicate_keys(text) {
        base.detail = e;
        return base;
    }
    let value: serde_json::Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => {
            base.detail = format!("malformed json: {e}");
            return base;
        }
    };
    let obligation = match parse_obligation(&value) {
        Ok(o) => o,
        Err(Fail::Invalid(e)) => {
            base.detail = e;
            // Invalid still binds the run when the id parsed, so the
            // host can attribute the refusal to its run.
            if let Some(id) = value.get("run_id").and_then(|v| v.as_str()) {
                base.run_id = id.to_string();
            }
            return base;
        }
        Err(Fail::Unsupported(e)) => {
            base.disposition = Disposition::Unsupported;
            base.detail = e;
            // Unsupported still binds the run when the id parsed.
            if let Some(id) = value.get("run_id").and_then(|v| v.as_str()) {
                base.run_id = id.to_string();
            }
            return base;
        }
    };
    base.run_id = obligation.run_id.clone();
    match evaluate(&obligation) {
        EvalOutcome::Zero { points, degree } => {
            base.disposition = Disposition::Agree;
            base.detail = format!("all {points} grid points vanish (degree {degree})");
            base.evaluated_points = points;
            base.grid_degree = Some(degree);
        }
        EvalOutcome::Witness {
            point,
            points,
            degree,
        } => {
            base.disposition = Disposition::Disagree;
            base.detail = format!(
                "counterexample {}",
                point
                    .iter()
                    .map(|(s, x)| format!("{s}={x}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            base.evaluated_points = points;
            base.grid_degree = Some(degree);
            base.witness = Some(point);
        }
        EvalOutcome::Resource(reason) => {
            base.disposition = Disposition::ResourceExhausted;
            base.detail = reason;
        }
    }
    base
}

impl Verdict {
    /// Minimal-JSON verdict bytes for the verdict file.
    pub fn to_json(&self) -> String {
        let witness = self.witness.as_ref().map(|point| {
            point
                .iter()
                .map(|(s, x)| (s.clone(), serde_json::json!(*x)))
                .collect::<serde_json::Map<String, serde_json::Value>>()
        });
        serde_json::json!({
            "schema": VERDICT_SCHEMA,
            "version": VERDICT_VERSION,
            "run_id": self.run_id,
            "obligation_sha256": self.obligation_sha256,
            "exe_sha256": self.exe_sha256,
            "checker_id": CHECKER_ID,
            "checker_version": CHECKER_VERSION,
            "disposition": self.disposition.as_str(),
            "detail": self.detail,
            "evaluated_points": self.evaluated_points,
            "grid_degree": self.grid_degree,
            "witness": witness,
        })
        .to_string()
    }
}

/// SHA-256 hex of bytes.
fn hex_sha256(bytes: &[u8]) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut hex = String::with_capacity(64);
    for b in digest {
        hex.push_str(&format!("{b:02x}"));
    }
    hex
}

/// Parse failure: schema violation or unimplemented kind.
enum Fail {
    Invalid(String),
    Unsupported(String),
}

fn invalid<T>(detail: impl Into<String>) -> Result<T, Fail> {
    Err(Fail::Invalid(detail.into()))
}

/// Reject duplicate object keys via the deserializer itself: entries
/// arrive in document order with escapes decoded, so `a` and `\u0061`
/// compare equal. Malformed JSON also fails here; the main parse reports it.
fn reject_duplicate_keys(text: &str) -> Result<(), String> {
    use serde::de::{self, MapAccess, SeqAccess, Visitor};
    use serde::Deserialize;
    use std::fmt;
    use std::marker::PhantomData;

    struct NoDup;

    struct Walk(PhantomData<()>);

    impl<'de> Visitor<'de> for Walk {
        type Value = ();
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("any json value")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
            let mut seen = BTreeSet::new();
            while let Some(key) = map.next_key::<String>()? {
                if !seen.insert(key.clone()) {
                    return Err(de::Error::custom(format!("duplicate key \"{key}\"")));
                }
                map.next_value::<NoDup>()?;
            }
            Ok(())
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
            while seq.next_element::<NoDup>()?.is_some() {}
            Ok(())
        }
        fn visit_bool<E: de::Error>(self, _v: bool) -> Result<(), E> {
            Ok(())
        }
        fn visit_i64<E: de::Error>(self, _v: i64) -> Result<(), E> {
            Ok(())
        }
        fn visit_u64<E: de::Error>(self, _v: u64) -> Result<(), E> {
            Ok(())
        }
        fn visit_f64<E: de::Error>(self, _v: f64) -> Result<(), E> {
            Ok(())
        }
        fn visit_str<E: de::Error>(self, _v: &str) -> Result<(), E> {
            Ok(())
        }
        fn visit_none<E: de::Error>(self) -> Result<(), E> {
            Ok(())
        }
        fn visit_unit<E: de::Error>(self) -> Result<(), E> {
            Ok(())
        }
    }

    impl<'de> serde::Deserialize<'de> for NoDup {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            d.deserialize_any(Walk(PhantomData))?;
            Ok(NoDup)
        }
    }

    let mut de = serde_json::Deserializer::from_str(text);
    NoDup::deserialize(&mut de).map_err(|e| format!("duplicate key or malformed json: {e}"))?;
    de.end().map_err(|e| format!("trailing bytes: {e}"))?;
    Ok(())
}

/// Strict obligation parse against the wire schema.
fn parse_obligation(value: &serde_json::Value) -> Result<Obligation, Fail> {
    let obj = value
        .as_object()
        .ok_or_else(|| Fail::Invalid("obligation must be an object".into()))?;
    for key in obj.keys() {
        match key.as_str() {
            "schema" | "version" | "run_id" | "symbols" | "expect" | "tree" => {}
            _ => return invalid(format!("unknown obligation key '{key}'")),
        }
    }
    let get = |key: &str| {
        obj.get(key)
            .ok_or_else(|| Fail::Invalid(format!("missing obligation key '{key}'")))
    };
    if get("schema")?.as_str() != Some(OBLIGATION_SCHEMA) {
        return invalid("obligation schema must be physis-int-obligation");
    }
    if get("version")?.as_u64() != Some(OBLIGATION_VERSION) {
        return invalid("unknown obligation version");
    }
    let run_id = match get("run_id")?.as_str() {
        Some(id) if !id.is_empty() => id.to_string(),
        _ => return invalid("run_id must be a non-empty string"),
    };
    let symbols = match get("symbols")? {
        serde_json::Value::Array(items) if !items.is_empty() => {
            let mut names = Vec::with_capacity(items.len());
            for item in items {
                match item.as_str() {
                    Some(name) if is_name(name) => names.push(name.to_string()),
                    _ => return invalid("symbols must be valid names"),
                }
            }
            names
        }
        _ => return invalid("symbols must be a non-empty array"),
    };
    let mut sorted = symbols.clone();
    sorted.sort();
    if sorted != symbols || {
        let mut dedup = sorted.clone();
        dedup.dedup();
        dedup.len() != sorted.len()
    } {
        return invalid("symbols must be sorted and unique");
    }
    match get("expect")?.as_str() {
        Some("zero") => {}
        Some(other) => {
            return Err(Fail::Unsupported(format!(
                "unsupported expectation '{other}'"
            )));
        }
        None => return invalid("expect must be a string"),
    }
    let mut nodes = 0usize;
    let tree = parse_tree(get("tree")?, 0, &mut nodes, &symbols)?;
    Ok(Obligation {
        run_id,
        symbols,
        tree,
    })
}

/// `[A-Za-z_][A-Za-z0-9_]*`, non-empty.
fn is_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Strict tree parse with depth and node budgets.
fn parse_tree(
    value: &serde_json::Value,
    depth: usize,
    nodes: &mut usize,
    symbols: &[String],
) -> Result<Tree, Fail> {
    if depth > MAX_TREE_DEPTH {
        return invalid("expression tree too deep");
    }
    *nodes += 1;
    if *nodes > MAX_TREE_NODES {
        return invalid("expression tree has too many nodes");
    }
    let obj = value
        .as_object()
        .ok_or_else(|| Fail::Invalid("tree node must be an object".into()))?;
    if let Some(n) = obj.get("const") {
        if obj.len() != 1 {
            return invalid("const node takes only const");
        }
        return n
            .as_i64()
            .map(|x| Tree::Const(x as i128))
            .or_else(|| n.as_u64().map(|x| Tree::Const(i128::from(x))))
            .ok_or_else(|| Fail::Invalid("const must be an integer within i128".into()));
    }
    if let Some(name) = obj.get("var") {
        if obj.len() != 1 {
            return invalid("var node takes only var");
        }
        return match name.as_str() {
            Some(var) if symbols.iter().any(|s| s == var) => Ok(Tree::Var(var.to_string())),
            _ => invalid("var must name a declared symbol"),
        };
    }
    let op = match obj.get("op").and_then(|v| v.as_str()) {
        Some(op) => op,
        None => return invalid("node needs const, var, or op"),
    };
    let args = obj
        .get("args")
        .and_then(|v| v.as_array())
        .ok_or_else(|| Fail::Invalid(format!("op '{op}' needs an args array")))?;
    let mut children = Vec::with_capacity(args.len());
    for arg in args {
        children.push(parse_tree(arg, depth + 1, nodes, symbols)?);
    }
    match op {
        "add" | "sub" | "mul" => {
            if obj.len() != 2 || children.len() != 2 {
                return invalid(format!("op '{op}' takes exactly 2 args"));
            }
            let [a, b]: [Tree; 2] = children
                .try_into()
                .map_err(|_| Fail::Invalid(format!("op '{op}' takes exactly 2 args")))?;
            Ok(match op {
                "add" => Tree::Add(Box::new(a), Box::new(b)),
                "sub" => Tree::Sub(Box::new(a), Box::new(b)),
                _ => Tree::Mul(Box::new(a), Box::new(b)),
            })
        }
        "neg" => {
            if obj.len() != 2 || children.len() != 1 {
                return invalid("op neg takes exactly 1 arg");
            }
            Ok(Tree::Neg(Box::new(children.into_iter().next().unwrap())))
        }
        "pow" => {
            if obj.len() != 3 || children.len() != 1 {
                return invalid("op pow takes 1 arg plus exp");
            }
            let exp = match obj.get("exp").and_then(|v| v.as_u64()) {
                Some(e) if e <= u32::MAX as u64 => e as u32,
                _ => return invalid("pow exp must be a non-negative integer"),
            };
            Ok(Tree::Pow(
                Box::new(children.into_iter().next().unwrap()),
                exp,
            ))
        }
        _ => invalid(format!("unknown op '{op}'")),
    }
}

/// Grid-evaluation outcome.
enum EvalOutcome {
    /// Every grid point vanished.
    Zero {
        /// Points evaluated.
        points: u64,
        /// Degree used.
        degree: u64,
    },
    /// First counterexample.
    Witness {
        /// Symbol values in declaration order.
        point: Vec<(String, i128)>,
        /// Points evaluated including the witness.
        points: u64,
        /// Degree used.
        degree: u64,
    },
    /// A cap stopped evaluation.
    Resource(String),
}

/// Evaluate the obligation on the full `{0..=d}^n` grid, where `d` is the
/// recomputed syntactic total degree. Exact: a nonzero integer polynomial
/// of total degree `d` cannot vanish on all `(d+1)^n` grid points.
fn evaluate(obligation: &Obligation) -> EvalOutcome {
    let degree = match total_degree(&obligation.tree) {
        Some(d) => d,
        None => return EvalOutcome::Resource("degree computation overflowed".into()),
    };
    let base = match degree.checked_add(1) {
        Some(b) => b,
        None => return EvalOutcome::Resource("degree + 1 overflowed".into()),
    };
    let mut count: u64 = 1;
    for _ in &obligation.symbols {
        count = match count.checked_mul(base) {
            Some(c) => c,
            None => {
                return EvalOutcome::Resource("grid point count overflowed".into());
            }
        };
    }
    if count > MAX_GRID_POINTS {
        return EvalOutcome::Resource(format!("grid has {count} points, cap is {MAX_GRID_POINTS}"));
    }
    let mut digits = vec![0i128; obligation.symbols.len()];
    for idx in 0..count {
        let value = match eval(&obligation.tree, &obligation.symbols, &digits) {
            Some(v) => v,
            None => {
                return EvalOutcome::Resource("integer overflow during evaluation".into());
            }
        };
        if value != 0 {
            let point: Vec<(String, i128)> = obligation
                .symbols
                .iter()
                .cloned()
                .zip(digits.iter().copied())
                .collect();
            return EvalOutcome::Witness {
                point,
                points: idx + 1,
                degree,
            };
        }
        odometer_next(&mut digits, base as i128);
    }
    EvalOutcome::Zero {
        points: count,
        degree,
    }
}

/// Syntactic total degree with checked arithmetic. `None` on overflow.
fn total_degree(tree: &Tree) -> Option<u64> {
    match tree {
        Tree::Const(_) => Some(0),
        Tree::Var(_) => Some(1),
        Tree::Add(a, b) | Tree::Sub(a, b) => {
            let x = total_degree(a)?;
            let y = total_degree(b)?;
            Some(x.max(y))
        }
        Tree::Mul(a, b) => total_degree(a)?.checked_add(total_degree(b)?),
        Tree::Neg(a) => total_degree(a),
        Tree::Pow(a, e) => total_degree(a)?.checked_mul(*e as u64),
    }
}

/// Evaluate at one point with checked i128 arithmetic. `None` on overflow.
fn eval(tree: &Tree, symbols: &[String], digits: &[i128]) -> Option<i128> {
    match tree {
        Tree::Const(c) => Some(*c),
        Tree::Var(name) => symbols
            .iter()
            .position(|s| s == name)
            .and_then(|i| digits.get(i).copied()),
        Tree::Add(a, b) => eval(a, symbols, digits)?.checked_add(eval(b, symbols, digits)?),
        Tree::Sub(a, b) => eval(a, symbols, digits)?.checked_sub(eval(b, symbols, digits)?),
        Tree::Mul(a, b) => eval(a, symbols, digits)?.checked_mul(eval(b, symbols, digits)?),
        Tree::Neg(a) => eval(a, symbols, digits)?.checked_neg(),
        Tree::Pow(a, e) => checked_pow(eval(a, symbols, digits)?, *e),
    }
}

/// Checked integer power by squaring.
fn checked_pow(mut base: i128, mut exp: u32) -> Option<i128> {
    let mut acc: i128 = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            acc = acc.checked_mul(base)?;
        }
        exp >>= 1;
        if exp > 0 {
            base = base.checked_mul(base)?;
        }
    }
    Some(acc)
}

/// Advance odometer digits (least significant first) in base `base`.
fn odometer_next(digits: &mut [i128], base: i128) {
    for d in digits.iter_mut() {
        *d += 1;
        if *d < base {
            return;
        }
        *d = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Canonical triangle obligation: `(b-a)-(c-a)+(c-b) = 0`.
    fn triangle() -> Vec<u8> {
        br#"{"expect":"zero","run_id":"t1","schema":"physis-int-obligation","symbols":["a","b","c"],"tree":{"args":[{"args":[{"args":[{"var":"b"},{"var":"a"}],"op":"sub"},{"args":[{"var":"c"},{"var":"a"}],"op":"sub"}],"op":"sub"},{"args":[{"var":"c"},{"var":"b"}],"op":"sub"}],"op":"add"},"version":1}"#.to_vec()
    }

    fn disposition_of(bytes: &[u8]) -> Disposition {
        check(bytes, "exe").disposition
    }

    #[test]
    fn triangle_agrees_on_eight_points() {
        let v = check(&triangle(), "exe");
        assert_eq!(v.disposition, Disposition::Agree);
        assert_eq!(v.evaluated_points, 8);
        assert_eq!(v.grid_degree, Some(1));
        assert_eq!(v.run_id, "t1");
        assert!(v.witness.is_none());
    }

    #[test]
    fn flipped_sign_disagrees_with_witness() {
        // (b+a)-(c-a)+(c-b) = 2a: nonzero at a=1.
        let bytes = br#"{"expect":"zero","run_id":"t2","schema":"physis-int-obligation","symbols":["a","b","c"],"tree":{"args":[{"args":[{"args":[{"var":"b"},{"var":"a"}],"op":"add"},{"args":[{"var":"c"},{"var":"a"}],"op":"sub"}],"op":"sub"},{"args":[{"var":"c"},{"var":"b"}],"op":"sub"}],"op":"add"},"version":1}"#;
        let v = check(bytes, "exe");
        assert_eq!(v.disposition, Disposition::Disagree);
        let witness = v.witness.expect("counterexample");
        assert!(v.detail.contains('a'), "{}", v.detail);
        assert_eq!(
            witness.iter().find(|(s, _)| s == "a").map(|(_, x)| *x),
            Some(1)
        );
    }

    #[test]
    fn nonzero_constant_disagrees() {
        let bytes = br#"{"expect":"zero","run_id":"t3","schema":"physis-int-obligation","symbols":["a"],"tree":{"const":7},"version":1}"#;
        let v = check(bytes, "exe");
        assert_eq!(v.disposition, Disposition::Disagree);
        assert!(v.evaluated_points >= 1);
    }

    #[test]
    fn quadratic_identity_holds_only_on_its_grid() {
        // x^2 - x vanishes on {0,1} but degree 2 forces {0,1,2}: witness x=2.
        let bytes = br#"{"expect":"zero","run_id":"t4","schema":"physis-int-obligation","symbols":["x"],"tree":{"args":[{"args":[{"var":"x"}],"op":"pow","exp":2},{"var":"x"}],"op":"sub"},"version":1}"#;
        let v = check(bytes, "exe");
        assert_eq!(v.disposition, Disposition::Disagree);
        assert_eq!(v.witness, Some(vec![("x".to_string(), 2)]));
    }

    #[test]
    fn overflow_is_resource_exhausted() {
        // 10^40 overflows i128 at the single grid point. Small literals:
        // huge literals are not representable JSON integers (see WIRE.md).
        let bytes = br#"{"expect":"zero","run_id":"t5","schema":"physis-int-obligation","symbols":["a"],"tree":{"args":[{"const":10}],"op":"pow","exp":40},"version":1}"#;
        assert_eq!(disposition_of(bytes), Disposition::ResourceExhausted);
    }

    #[test]
    fn oversized_grid_is_resource_exhausted_without_evaluating() {
        // 16 variables, degree 2 (sum of squares): 3^16 points, refused
        // before evaluation. Zero-padded names stay lexicographically sorted.
        let mut sum = String::from("{\"args\":[{\"var\":\"x00\"}],\"op\":\"pow\",\"exp\":2}");
        for i in 1..16 {
            sum = format!(
                "{{\"args\":[{sum},{{\"args\":[{{\"var\":\"x{i:02}\"}}],\"op\":\"pow\",\"exp\":2}}],\"op\":\"add\"}}"
            );
        }
        let symbols: Vec<String> = (0..16).map(|i| format!("\"x{i:02}\"")).collect();
        let bytes = format!(
            "{{\"expect\":\"zero\",\"run_id\":\"t6\",\"schema\":\"physis-int-obligation\",\"symbols\":[{}],\"tree\":{},\"version\":1}}",
            symbols.join(","),
            sum
        );
        let v = check(bytes.as_bytes(), "exe");
        assert_eq!(v.disposition, Disposition::ResourceExhausted);
        assert_eq!(v.evaluated_points, 0);
    }

    #[test]
    fn unsupported_expect_is_unsupported() {
        let bytes = br#"{"expect":"nonzero","run_id":"t7","schema":"physis-int-obligation","symbols":["a"],"tree":{"const":0},"version":1}"#;
        assert_eq!(disposition_of(bytes), Disposition::Unsupported);
    }

    #[test]
    fn refuses_duplicate_keys() {
        let bytes = br#"{"expect":"zero","run_id":"t8","schema":"physis-int-obligation","symbols":["a"],"tree":{"const":0},"version":1,"version":2}"#;
        assert_eq!(disposition_of(bytes), Disposition::Invalid);
    }

    #[test]
    fn refuses_unknown_version() {
        let bytes = br#"{"expect":"zero","run_id":"t9","schema":"physis-int-obligation","symbols":["a"],"tree":{"const":0},"version":2}"#;
        assert_eq!(disposition_of(bytes), Disposition::Invalid);
    }

    #[test]
    fn refuses_missing_field() {
        let bytes = br#"{"expect":"zero","run_id":"t10","schema":"physis-int-obligation","symbols":["a"],"version":1}"#;
        assert_eq!(disposition_of(bytes), Disposition::Invalid);
    }

    #[test]
    fn invalid_echoes_run_id_when_it_parsed() {
        let bytes = br#"{"expect":"zero","run_id":"t10b","schema":"physis-int-obligation","symbols":["a"],"version":1}"#;
        let v = check(bytes, "");
        assert_eq!(v.disposition, Disposition::Invalid);
        assert_eq!(v.run_id, "t10b");
    }

    #[test]
    fn refuses_unknown_op() {
        let bytes = br#"{"expect":"zero","run_id":"t11","schema":"physis-int-obligation","symbols":["a"],"tree":{"args":[{"var":"a"}],"op":"sqrt"},"version":1}"#;
        assert_eq!(disposition_of(bytes), Disposition::Invalid);
    }

    #[test]
    fn refuses_float_const() {
        let bytes = br#"{"expect":"zero","run_id":"t12","schema":"physis-int-obligation","symbols":["a"],"tree":{"const":2.0},"version":1}"#;
        assert_eq!(disposition_of(bytes), Disposition::Invalid);
    }

    #[test]
    fn refuses_undeclared_var() {
        let bytes = br#"{"expect":"zero","run_id":"t13","schema":"physis-int-obligation","symbols":["a"],"tree":{"var":"zzz"},"version":1}"#;
        assert_eq!(disposition_of(bytes), Disposition::Invalid);
    }

    #[test]
    fn refuses_unsorted_or_dup_symbols() {
        for symbols in [r#"["b","a"]"#, r#"["a","a"]"#, "[]"] {
            let bytes = format!(
                "{{\"expect\":\"zero\",\"run_id\":\"t14\",\"schema\":\"physis-int-obligation\",\"symbols\":{symbols},\"tree\":{{\"const\":0}},\"version\":1}}"
            );
            assert_eq!(
                disposition_of(bytes.as_bytes()),
                Disposition::Invalid,
                "{symbols}"
            );
        }
    }

    #[test]
    fn refuses_deep_tree() {
        let mut tree = String::from("{\"var\":\"a\"}");
        for _ in 0..300 {
            tree = format!("{{\"args\":[{tree}],\"op\":\"neg\"}}");
        }
        let bytes = format!(
            "{{\"expect\":\"zero\",\"run_id\":\"t15\",\"schema\":\"physis-int-obligation\",\"symbols\":[\"a\"],\"tree\":{tree},\"version\":1}}"
        );
        assert_eq!(disposition_of(bytes.as_bytes()), Disposition::Invalid);
    }

    #[test]
    fn refuses_overlarge_file() {
        let mut bytes = triangle();
        bytes.extend(vec![b' '; MAX_OBLIGATION_BYTES]);
        assert_eq!(disposition_of(&bytes), Disposition::Invalid);
    }

    #[test]
    fn refuses_bad_symbol_names() {
        for name in ["a-b", "9a", "", "a b", "a.b"] {
            let bytes = format!(
                "{{\"expect\":\"zero\",\"run_id\":\"t17\",\"schema\":\"physis-int-obligation\",\"symbols\":[\"{name}\"],\"tree\":{{\"const\":0}},\"version\":1}}"
            );
            assert_eq!(
                disposition_of(bytes.as_bytes()),
                Disposition::Invalid,
                "{name}"
            );
        }
    }

    #[test]
    fn refuses_bad_arity() {
        let bytes = br#"{"expect":"zero","run_id":"t18","schema":"physis-int-obligation","symbols":["a"],"tree":{"args":[{"var":"a"}],"op":"add"},"version":1}"#;
        assert_eq!(disposition_of(bytes), Disposition::Invalid);
    }

    #[test]
    fn refuses_malformed_json() {
        assert_eq!(disposition_of(b"{torn"), Disposition::Invalid);
        assert_eq!(disposition_of(b""), Disposition::Invalid);
    }

    #[test]
    fn bindings_echo_run_and_hash() {
        let bytes = triangle();
        let v = check(&bytes, "deadbeef");
        assert_eq!(v.run_id, "t1");
        assert_eq!(v.exe_sha256, "deadbeef");
        assert_eq!(v.obligation_sha256.len(), 64);
        assert!(v.obligation_sha256.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
