# Agent protocol

See `specs/006-agent-protocol.md` and `AGENTS.md`.

## CLI

```
cargo run -p physis -- layers
cargo run -p physis -- theories
cargo run -p physis -- knobs type-iib
cargo run -p physis -- run type-iib
cargo run -p physis -- set type-iib total_dim 9
cargo run -p physis -- experiment string-critique
cargo run -p physis -- journal
```

## Library

```rust,ignore
use physis_agent::{Command, Lab};

let mut lab = Lab::standard();
let r = lab.exec(Command::Set {
    theory: "type-iib".into(),
    knob: "total_dim".into(),
    value: "9".into(),
});
println!("{}", r.text());
```

## Journal

JSONL events. Append only. In-memory by default. `Journal::file` exists for persistence; CLI wiring of `--journal` is M1.

## Long time horizons

The bet is that an agent can live here for a long time *because* the state is small, typed, and diffable. If you need a 10⁹-degree-of-freedom lattice, you are in a different product. Build that as a later crate that still *reports* into this claim system, rather than replacing it.
