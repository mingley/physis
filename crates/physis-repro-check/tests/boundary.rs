//! Automated dependency-boundary checks (spec 021, C6.2).
//!
//! The checker must not link to or copy the production proof stack:
//! only the documented wire format, standard serialization, and hash
//! libraries are shared. These tests fail the build if the boundary
//! drifts; they do not replace the maintainer-reviewed assessment in
//! `ALLOWLIST.md`.

use std::path::PathBuf;

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Dependency keys the checker may use, nothing else.
fn allowed_deps() -> Vec<&'static str> {
    vec!["serde", "serde_json", "sha2"]
}

fn manifest() -> String {
    std::fs::read_to_string(crate_dir().join("Cargo.toml")).expect("checker Cargo.toml")
}

/// Keys of a `[section]` table: lines starting a `key =` entry.
fn section_keys(text: &str, section: &str) -> Vec<String> {
    let mut keys = Vec::new();
    let mut inside = false;
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            inside = t == section;
            continue;
        }
        if inside && !t.is_empty() && !t.starts_with('#') {
            if let Some(key) = t.split(['=', ' ', '.']).next() {
                keys.push(key.to_string());
            }
        }
    }
    keys
}

#[test]
fn checker_deps_stay_in_the_allowlist() {
    let text = manifest();
    for key in section_keys(&text, "[dependencies]") {
        assert!(
            allowed_deps().contains(&key.as_str()),
            "checker dependency '{key}' is outside the allowlist"
        );
    }
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() || t.starts_with('#') || t.starts_with('[') {
            continue;
        }
        if let Some(key) = t.split(['=', ' ']).next() {
            assert!(
                !key.starts_with("physis"),
                "checker manifest must not reference physis crates: {t}"
            );
        }
    }
    let mut section = String::new();
    for line in text.lines() {
        let t = line.trim();
        if t.starts_with('[') {
            section = t.to_string();
            continue;
        }
        if section.contains("dependencies") || section.contains("patch") {
            assert!(
                !t.contains("path =") && !t.contains("path="),
                "checker manifest must not use path dependencies: {t}"
            );
        }
    }
}

#[test]
fn checker_dev_deps_stay_empty() {
    let text = manifest();
    assert!(
        section_keys(&text, "[dev-dependencies]").is_empty(),
        "checker tests use std only; keep it that way"
    );
}

fn source_files() -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![crate_dir().join("src")];
    while let Some(dir) = stack.pop() {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(&dir)
            .expect("read src")
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    assert!(!out.is_empty(), "checker has no sources?");
    out
}

#[test]
fn checker_source_uses_no_physis() {
    for path in source_files() {
        let text = std::fs::read_to_string(&path).expect("read source");
        for (n, line) in text.lines().enumerate() {
            for needle in ["use physis", "physis::", "extern crate physis"] {
                assert!(
                    !line.contains(needle),
                    "{}:{} uses production code: {line}",
                    path.display(),
                    n + 1
                );
            }
        }
    }
}

#[test]
fn checker_is_unsafe_free() {
    for path in source_files() {
        let text = std::fs::read_to_string(&path).expect("read source");
        if path
            .file_name()
            .is_some_and(|n| n == "lib.rs" || n == "main.rs")
        {
            assert!(
                text.contains("#![forbid(unsafe_code)]"),
                "{} must forbid unsafe",
                path.display()
            );
        }
        for (n, line) in text.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("//") || t.starts_with("//!") {
                continue;
            }
            for needle in ["unsafe ", "unsafe{", "unsafe(", "unsafe;"] {
                assert!(
                    !line.contains(needle),
                    "{}:{} uses unsafe: {line}",
                    path.display(),
                    n + 1
                );
            }
        }
    }
}

#[test]
fn no_physis_crate_links_the_checker() {
    // Walk the workspace siblings: no physis crate may depend on the
    // checker in any section (linking it would put both implementations
    // in one binary, defeating independence).
    let crates = crate_dir().parent().expect("crates dir").to_path_buf();
    let mut checked = 0;
    let mut entries: Vec<PathBuf> = std::fs::read_dir(&crates)
        .expect("read crates")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();
    for dir in entries {
        let manifest = dir.join("Cargo.toml");
        if !manifest.is_file() || dir.file_name().is_some_and(|n| n == "physis-repro-check") {
            continue;
        }
        let text = std::fs::read_to_string(&manifest).expect("read manifest");
        if !text.contains("name = \"physis") {
            continue;
        }
        checked += 1;
        assert!(
            !text.contains("physis-repro-check") && !text.contains("physis_repro_check"),
            "{} links the checker",
            manifest.display()
        );
    }
    assert!(
        checked >= 10,
        "boundary walk found too few crates: {checked}"
    );
}
