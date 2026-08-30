# Agent protocol

See `specs/006-agent-protocol.md` and `AGENTS.md`.

## CLI

```
cargo run -p physis -- layers
cargo run -p physis -- theories
cargo run -p physis -- knobs type-iib
cargo run -p physis -- run type-iib
cargo run -p physis -- set type-iib total_dim 9
cargo run -p physis -- hypothesize type-iib
cargo run -p physis -- evidence predictivity.unique-vacuum
cargo run -p physis -- experiment string-critique
cargo run -p physis -- score heterotic-e8e8
cargo run -p physis -- journal

# Persist a session across process runs, then replay + verify it:
cargo run -p physis -- --journal session.jsonl set type-iib total_dim 9
cargo run -p physis -- --journal session.jsonl set type-iib supersymmetry false
cargo run -p physis -- replay session.jsonl

# Structured JSON output for agents (typed matrix + diffs):
cargo run -p physis -- --json experiment string-critique
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

JSONL events. Append only. In-memory by default. `Journal::file` persists to
disk, and the CLI `--journal <file.jsonl>` flag records a session across process
runs (restoring prior state each run so the session stays coherent).

`physis replay <file.jsonl>` re-applies the recorded `set` events onto a fresh
`Lab::standard()`, recomputes the verdict diffs, and verifies they match what
was recorded. Kind triples always compare; derivation / empirical / judgment
strings compare only when the journal record carries them (pre-axis JSONL
still certifies). The check exits non-zero on any mismatch. See
`physis_agent::replay::replay_journal`.

## Long time horizons

The bet is that an agent can live here for a long time *because* the state is small, typed, and diffable. If you need a 10⁹-degree-of-freedom lattice, you are in a different product. Build that as a later crate that still *reports* into this claim system, rather than replacing it.
