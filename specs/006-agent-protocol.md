# 006 — Agent protocol

Status: active
Layer: agent

## Commands

| op | meaning |
|---|---|
| `layers` | list `LayerId` with docs |
| `theories` | list lab theories |
| `knobs [theory]` | snapshot knobs |
| `run <theory>` | evaluate all claims |
| `set <theory> <knob> <value>` | turn a knob, print Δverdicts |
| `experiments` | list the available experiments |
| `experiment <id>` | canonical experiment (fresh defaults) |
| `score <theory>` | grade a theory against the empirical-target fixture |
| `journal` | dump JSONL |
| `replay <path>` | replay a recorded JSONL journal and verify it reproduces |

CLI tokens map 1:1 onto `physis_agent::Command`.

## Responses

`Response::Ok { text, report, diffs }` or `Response::Err { message }`.

Text is for humans and for agents that just want a buffer. `report` / `diffs` are structured.

The CLI `--json` flag emits the whole `Response` as JSON (status, text, and the
structured `report`/`diffs`), so an agent gets typed matrices and verdict diffs
rather than parsing prose. Example: `physis --json set type-iib total_dim 9`.

## Journal events

- `boot` — theory ids at lab creation
- `set-knob` — from, to, diffs
- `run` — holds/fails/other counts
- `experiment` — experiment id

Append-only. Optional file backend (`Journal::file`).

## Long-horizon use

Agents should persist the journal, not the vibes. A later agent must be able to replay “what was tried” from JSONL without the original session.

Replay of `set` events onto a fresh `Lab::standard()` is implemented (M1.1).
`replay_journal` re-applies every recorded `set-knob`, recomputes the verdict
diffs, and checks them against the recorded diffs. A faithful replay is a
mechanical proof that the session reproduces; a mismatch (or a failed turn)
proves the journal or the encoding drifted, and the CLI exits non-zero.

Journals must be **coherent sessions**: the diffs of a `set` are computed
against the state left by the previous `set`. The one-shot CLI evaluates each
line against fresh defaults, so to persist a real session across process runs,
pass `--journal <file.jsonl>`: the lab loads the file and *restores* prior state
(`Lab::restore_from_journal`) before the new turn, so the accumulated file
replays faithfully.

Timestamps (`t`) are Unix milliseconds as `u64` — 128-bit integers do not
survive serde's internally tagged round-trip, which would silently drop every
event on reload.

## Non-goals

- Natural language as a command
- Unbounded Python in the loop
- Networked multi-agent consensus
