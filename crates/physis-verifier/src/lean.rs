//! Lean kernel compile + nanoda replay of a `lean4export` file.
//!
//! Both checkers must run. Missing tools is [`crate::VerifyError::LeanPipelineNotWired`],
//! not a mint.

use std::io::Write;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::process::Command;

use physis_proof::{compact_lean_type, extract_theorems, Challenge};

use crate::{CheckerReceipt, VerifyError};

const LEAN_TOOLCHAIN: &str = "leanprover/lean4:v4.34.0-rc2";
const NANODA_VERSION: &str = "0.4.16";
const EXPORTER_VERSION: &str = "lean4export-3.1.0";

/// Locations of the two checkers. Absence of either means the pipeline is
/// not wired.
#[derive(Clone, Debug)]
pub struct LeanTools {
    lean: PathBuf,
    lake: PathBuf,
    lean4export: PathBuf,
}

/// Discover `lean`, `lake`, and `lean4export` (`LEAN4EXPORT` or PATH).
pub fn discover_tools() -> Option<LeanTools> {
    let lean = which("lean")?;
    let lake = which("lake")?;
    let lean4export = std::env::var_os("LEAN4EXPORT")
        .map(PathBuf::from)
        .filter(|p| p.is_file())
        .or_else(|| which("lean4export"))
        .or_else(|| {
            let p = PathBuf::from("/tmp/lean4export/.lake/build/bin/lean4export");
            p.is_file().then_some(p)
        })?;
    Some(LeanTools {
        lean,
        lake,
        lean4export,
    })
}

fn which(name: &str) -> Option<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::env::var_os("PATH")
        .map(|p| std::env::split_paths(&p).collect())
        .unwrap_or_default();
    if let Some(home) = std::env::var_os("HOME") {
        dirs.push(PathBuf::from(home).join(".elan/bin"));
    }
    for dir in dirs {
        let p = dir.join(name);
        if p.is_file() {
            return Some(p);
        }
    }
    None
}

/// Compile untrusted Lean with the Lean kernel and replay the export with
/// nanoda. The source must contain a theorem whose type matches the challenge.
pub(crate) fn check_source(
    challenge: &Challenge,
    source: &str,
) -> Result<(CheckerReceipt, CheckerReceipt), VerifyError> {
    let want = compact_lean_type(&challenge.lean_type);
    let thm = extract_theorems(source)
        .into_iter()
        .find(|t| compact_lean_type(&t.ty) == want)
        .ok_or(VerifyError::StatementMismatch)?;
    let tools = discover_tools().ok_or(VerifyError::LeanPipelineNotWired)?;

    let lake_env = |cmd: &mut Command| {
        cmd.env("ELAN_TOOLCHAIN", LEAN_TOOLCHAIN);
        if let Some(home) = std::env::var_os("HOME") {
            let elan = PathBuf::from(home).join(".elan/bin");
            if let Some(joined) = std::env::var_os("PATH")
                .and_then(|p| {
                    let mut v = vec![elan.clone()];
                    v.extend(std::env::split_paths(&p));
                    std::env::join_paths(v).ok()
                })
                .or_else(|| Some(elan.into_os_string()))
            {
                cmd.env("PATH", joined);
            }
        }
    };

    let tmp = tempfile_dir().map_err(VerifyError::LeanKernelRejected)?;
    write_sandbox(&tmp, source).map_err(VerifyError::LeanKernelRejected)?;

    let mut lean_cmd = Command::new(&tools.lake);
    lean_cmd.arg("build").current_dir(&tmp);
    lake_env(&mut lean_cmd);
    let lean_status = lean_cmd
        .output()
        .map_err(|e| VerifyError::LeanKernelRejected(e.to_string()))?;
    if !lean_status.status.success() {
        let err = String::from_utf8_lossy(&lean_status.stderr);
        let out = String::from_utf8_lossy(&lean_status.stdout);
        let _ = std::fs::remove_dir_all(&tmp);
        return Err(VerifyError::LeanKernelRejected(format!("{err}{out}")));
    }
    let primary = CheckerReceipt::ran("lean-kernel", &lean_version(&tools.lean), true);

    let export_path = tmp.join("export.ndjson");
    let mut export_cmd = Command::new(&tools.lake);
    export_cmd
        .arg("env")
        .arg(&tools.lean4export)
        .arg("Physlib")
        .arg("--")
        .arg(&thm.name)
        .current_dir(&tmp);
    lake_env(&mut export_cmd);
    let export_out = export_cmd
        .output()
        .map_err(|e| VerifyError::LeanKernelRejected(e.to_string()))?;
    if !export_out.status.success() {
        let err = String::from_utf8_lossy(&export_out.stderr);
        let _ = std::fs::remove_dir_all(&tmp);
        return Err(VerifyError::LeanKernelRejected(format!(
            "lean4export: {err}"
        )));
    }
    std::fs::write(&export_path, &export_out.stdout)
        .map_err(|e| VerifyError::LeanKernelRejected(e.to_string()))?;

    nanoda_check(&export_path).inspect_err(|_| {
        let _ = std::fs::remove_dir_all(&tmp);
    })?;
    let secondary = CheckerReceipt::ran(
        "nanoda",
        &format!("{NANODA_VERSION}+{EXPORTER_VERSION}"),
        true,
    );

    let _ = std::fs::remove_dir_all(&tmp);
    Ok((primary, secondary))
}

fn lean_version(lean: &Path) -> String {
    Command::new(lean)
        .arg("--version")
        .env("ELAN_TOOLCHAIN", LEAN_TOOLCHAIN)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| {
            s.lines()
                .next()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
        })
        .unwrap_or_else(|| LEAN_TOOLCHAIN.to_string())
}

fn nanoda_check(export_path: &Path) -> Result<(), VerifyError> {
    let cfg: nanoda_lib::util::Config = serde_json::from_value(serde_json::json!({
        "export_file_path": export_path,
        "permitted_axioms": ["propext", "Quot.sound", "Classical.choice"],
        "unpermitted_axiom_hard_error": true,
        "print_success_message": false,
        "print_axioms": false,
        "nat_extension": true,
        "string_extension": true,
        "num_threads": 1usize
    }))
    .map_err(|e| VerifyError::NanodaRejected(e.to_string()))?;
    let (export_file, skipped) = cfg
        .to_export_file()
        .map_err(|e| VerifyError::NanodaRejected(e.to_string()))?;
    if !skipped.is_empty() {
        return Err(VerifyError::NanodaRejected(format!(
            "skipped unpermitted axioms {skipped:?}"
        )));
    }
    let ok = catch_unwind(AssertUnwindSafe(|| {
        export_file.check_all_declars();
    }));
    match ok {
        Ok(()) => Ok(()),
        Err(_) => Err(VerifyError::NanodaRejected(
            "nanoda panicked while typechecking the export".into(),
        )),
    }
}

fn tempfile_dir() -> Result<PathBuf, String> {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let dir = std::env::temp_dir().join(format!("physis-lean-{}-{}", std::process::id(), nanos));
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn write_sandbox(dir: &Path, source: &str) -> Result<(), String> {
    std::fs::write(dir.join("lean-toolchain"), format!("{LEAN_TOOLCHAIN}\n"))
        .map_err(|e| e.to_string())?;
    std::fs::write(
        dir.join("lakefile.toml"),
        "name = \"physlib\"\ndefaultTargets = [\"Physlib\"]\n\n[[lean_lib]]\nname = \"Physlib\"\n",
    )
    .map_err(|e| e.to_string())?;
    let mut f = std::fs::File::create(dir.join("Physlib.lean")).map_err(|e| e.to_string())?;
    f.write_all(source.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}
