# Agent protocol

See `specs/006-agent-protocol.md` and `AGENTS.md`.

For task selection and handoffs, use [TODO.md](../TODO.md) and
[the research workflow](RESEARCH-LAB.md). The proposed campaign protocol is
in [spec 022](../specs/022-research-campaigns.md); its `research` commands
are not available yet. Existing role flags are application permissions,
not process isolation.

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

## Global flags

| Flag | Meaning |
|---|---|
| `--journal <file.jsonl>` | Record the session across process runs; each run restores prior state first |
| `--json` | Structured output for agents (typed matrices + verdict diffs) |
| `--role <role>` | Run as a named role (default `lab`); the lab refuses commands outside the role |
| `--budget prove=N,review=N,set=N` | Cap those actions per lab; application-level counts, not time/memory limits |

## Roles

Thirteen roles (`lab` default). Roles are application permissions, not process
isolation or authenticated identity. Each row names the role's characteristic
command; refusals name the protocol verb.

| Role | Does | Cannot |
|---|---|---|
| `lab` | full protocol | — |
| `explorer` | observe, inspect, compare, hypothesize | set, prove, review, score, audit; cannot mint |
| `formalizer` | emit untrusted encoding (`formalize`) | prove |
| `proof-searcher` | request a dual-check mint (`prove`) | remint a stored receipt |
| `falsifier` | search failing evaluations (`falsify`) | prove, review |
| `reviewer` | request encoding review (`review`) | prove |
| `auditor` | run the red-team corpus (`audit`) | prove, review |
| `replication-agent` | remint a stored receipt in-process (`reproduce`, not P4) | prove |
| `empirical-analyst` | score a theory (`score`) | prove |
| `numerical-verifier` | parse a `CertifiedNumeric` enclosure (`enclose`) | prove |
| `provenance-auditor` | rebuild a SourceRecord / Constant (`cite`, `constant`) | prove, review |
| `encoding-auditor` | round-trip an IR package (`encode`) | prove, review |
| `judge` | rebuild a `from_lab` judgment (`judge`) | prove; JSON cannot mint `logical proved` |

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
