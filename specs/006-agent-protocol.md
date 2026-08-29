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
| `experiment <id>` | canonical experiment (fresh defaults) |
| `journal` | dump JSONL |

CLI tokens map 1:1 onto `physis_agent::Command`.

## Responses

`Response::Ok { text, report, diffs }` or `Response::Err { message }`.

Text is for humans and for agents that just want a buffer. `report` / `diffs` are structured.

## Journal events

- `boot` — theory ids at lab creation
- `set-knob` — from, to, diffs
- `run` — holds/fails/other counts
- `experiment` — experiment id

Append-only. Optional file backend (`Journal::file`).

## Long-horizon use

Agents should persist the journal, not the vibes. A later agent must be able to replay “what was tried” from JSONL without the original session.

Replay of `set` events onto a fresh `Lab::standard()` is a planned milestone (`plans/002`). v0 records; it does not replay.

## Non-goals

- Natural language as a command
- Unbounded Python in the loop
- Networked multi-agent consensus
