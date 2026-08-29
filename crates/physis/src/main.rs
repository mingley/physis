//! physis — turn knobs on typed theories of reality.

use physis::Lab;
use physis_agent::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut lab = Lab::standard();
    let cmd = match parse(&args) {
        Ok(c) => c,
        Err(help) => {
            eprintln!("{help}");
            std::process::exit(if args.is_empty() { 0 } else { 2 });
        }
    };
    let response = lab.exec(cmd);
    match &response {
        physis_agent::Response::Ok { text, .. } => print!("{text}"),
        physis_agent::Response::Err { message } => eprintln!("error: {message}"),
    }
    std::process::exit(response.exit_code());
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
        "experiment" => {
            let id = args
                .get(1)
                .cloned()
                .unwrap_or_else(|| "string-critique".into());
            Ok(Command::Experiment { id })
        }
        "journal" => Ok(Command::Journal),
        other => Err(format!("unknown command '{other}'\n{}", usage())),
    }
}

fn usage() -> String {
    r#"physis — mechanically verifiable models of reality

USAGE:
    physis layers
    physis theories
    physis knobs [theory]
    physis run <theory>
    physis set <theory> <knob> <value>
    physis experiment [string-critique]
    physis journal

EXAMPLES:
    physis experiment string-critique
    physis run type-iib
    physis set type-iib total_dim 9
    physis set standard-model generations 2

Theories: standard-model, general-relativity, type-iib,
          heterotic-e8e8, bosonic, observer-geometry

Docs: specs/  plans/  docs/  AGENTS.md
"#
    .into()
}
