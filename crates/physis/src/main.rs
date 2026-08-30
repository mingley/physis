//! physis — turn knobs on typed theories of reality.

use physis::{Journal, Lab};
use physis_agent::Command;

fn main() {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let (journal_path, json, args) = extract_opts(&raw);

    let mut lab = Lab::standard();
    if let Some(path) = &journal_path {
        match Journal::file(path) {
            // Persist this session's new events to the file; existing history
            // is loaded and replayed so `set` turns accumulate across process
            // runs into one coherent, replayable session.
            Ok(journal) => {
                *lab.journal_mut() = journal;
                lab.restore_from_journal();
            }
            Err(e) => {
                eprintln!("error: cannot open journal '{path}': {e}");
                std::process::exit(2);
            }
        }
    }

    let cmd = match parse(&args) {
        Ok(c) => c,
        Err(help) => {
            eprintln!("{help}");
            std::process::exit(if args.is_empty() { 0 } else { 2 });
        }
    };
    let response = lab.exec(cmd);
    if json {
        // Structured output for agents: the full typed response (matrix,
        // diffs, verdicts) as JSON, not just the human-readable text.
        match serde_json::to_string_pretty(&response) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("error: could not serialize response: {e}"),
        }
    } else {
        match &response {
            physis_agent::Response::Ok { text, .. } => print!("{text}"),
            physis_agent::Response::Err { message } => eprintln!("error: {message}"),
        }
    }
    std::process::exit(response.exit_code());
}

/// Strip leading global options (`--journal <path>`/`-j <path>` and `--json`),
/// in any order, returning the journal path, whether JSON output was requested,
/// and the remaining args. Options must precede the subcommand.
fn extract_opts(args: &[String]) -> (Option<String>, bool, Vec<String>) {
    let mut journal = None;
    let mut json = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--journal" | "-j" if i + 1 < args.len() => {
                journal = Some(args[i + 1].clone());
                i += 2;
            }
            "--json" => {
                json = true;
                i += 1;
            }
            _ => break,
        }
    }
    (journal, json, args[i..].to_vec())
}

fn parse(args: &[String]) -> Result<Command, String> {
    if args.is_empty() || args[0] == "help" || args[0] == "--help" || args[0] == "-h" {
        return Err(usage());
    }
    match args[0].as_str() {
        "layers" => Ok(Command::Layers),
        "theories" => Ok(Command::Theories),
        "knobs" => Ok(Command::Knobs {
            theory: args.get(1).cloned(),
        }),
        "run" => {
            let theory = args.get(1).ok_or_else(usage)?.clone();
            Ok(Command::Run { theory })
        }
        "set" => {
            if args.len() < 4 {
                return Err(usage());
            }
            Ok(Command::Set {
                theory: args[1].clone(),
                knob: args[2].clone(),
                value: args[3].clone(),
            })
        }
        "experiments" => Ok(Command::Experiments),
        "epistemics" => Ok(Command::Epistemics),
        "why" => {
            let claim = args.get(1).ok_or_else(usage)?.clone();
            Ok(Command::Why { claim })
        }
        "experiment" => {
            let id = args
                .get(1)
                .cloned()
                .unwrap_or_else(|| "string-critique".into());
            Ok(Command::Experiment { id })
        }
        "journal" => Ok(Command::Journal),
        "score" => {
            let theory = args.get(1).ok_or_else(usage)?.clone();
            Ok(Command::Score { theory })
        }
        "replay" => {
            let path = args.get(1).ok_or_else(usage)?.clone();
            Ok(Command::Replay { path })
        }
        "prove" => {
            let claim = args.get(1).ok_or_else(usage)?.clone();
            Ok(Command::Prove { claim })
        }
        "falsify" => {
            let claim = args.get(1).ok_or_else(usage)?.clone();
            Ok(Command::Falsify { claim })
        }
        "sweep" => {
            if args.len() < 4 {
                return Err(usage());
            }
            let values = args[3]
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            Ok(Command::Sweep {
                theory: args[1].clone(),
                knob: args[2].clone(),
                values,
            })
        }
        "branch" => {
            let name = args.get(1).ok_or_else(usage)?.clone();
            Ok(Command::Branch { name })
        }
        "checkout" => {
            let name = args.get(1).ok_or_else(usage)?.clone();
            Ok(Command::Checkout { name })
        }
        "compare" => {
            if args.len() < 3 {
                return Err(usage());
            }
            Ok(Command::Compare {
                a: args[1].clone(),
                b: args[2].clone(),
            })
        }
        "audit" => Ok(Command::Audit),
        "design" => {
            if args.len() < 3 {
                return Err(usage());
            }
            Ok(Command::Design {
                theories: args[1..].to_vec(),
            })
        }
        "sensitivity" => {
            if args.len() < 3 {
                return Err(usage());
            }
            Ok(Command::Sensitivity {
                theory: args[1].clone(),
                knob: args[2].clone(),
            })
        }
        other => Err(format!("unknown command '{other}'\n{}", usage())),
    }
}

fn usage() -> String {
    r#"physis — mechanically verifiable models of reality

USAGE:
    physis [--journal <file.jsonl>] [--json] <command>
    physis layers
    physis theories
    physis knobs [theory]
    physis run <theory>
    physis set <theory> <knob> <value>
    physis score <theory>
    physis epistemics
    physis why <claim-id>
    physis prove <claim-id>
    physis falsify <claim-id>
    physis sweep <theory> <knob> <v1,v2,...>
    physis branch <name>
    physis checkout <name>
    physis compare <theory-a> <theory-b>
    physis design <theory> <theory> [...]
    physis sensitivity <theory> <knob>
    physis audit
    physis experiments
    physis experiment [string-critique | em-vacuum | computation | field-modes | gauge-lattice | thermo | blackbody | solid | gravity | olbers | bell]
    physis journal
    physis replay <journal.jsonl>

EXAMPLES:
    physis experiment string-critique
    physis run type-iib
    physis set type-iib total_dim 9
    physis set standard-model generations 2
    physis --journal session.jsonl set type-iib total_dim 9
    physis replay session.jsonl

Theories: standard-model, general-relativity, type-iib,
          heterotic-e8e8, bosonic, observer-geometry

Docs: specs/  plans/  docs/  AGENTS.md
"#
    .into()
}
