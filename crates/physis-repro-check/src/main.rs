//! Independent checker executable: obligation file in, verdict file out.
//!
//! Usage: `physis-repro-check <obligation.json> <verdict.json>`.
//! Exit `0` means a verdict file was written (any disposition).
//! Anything else means no verdict exists and the host must map the
//! run to `execution-failed`.

#![forbid(unsafe_code)]

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: physis-repro-check <obligation.json> <verdict.json>");
        return ExitCode::from(2);
    }
    let bytes = match std::fs::read(&args[1]) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("cannot read obligation '{}': {e}", args[1]);
            return ExitCode::from(1);
        }
    };
    let exe = std::env::var("WORKER_EXE_DIGEST").unwrap_or_default();
    let verdict = physis_repro_check::check(&bytes, &exe);
    match std::fs::write(&args[2], verdict.to_json()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("cannot write verdict '{}': {e}", args[2]);
            ExitCode::from(1)
        }
    }
}
