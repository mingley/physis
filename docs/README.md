# physis documentation

## Start here

- **New to the lab?** Read the [README](../README.md), then [RESEARCH-LAB.md](RESEARCH-LAB.md), then run the [string lab](STRING-EXPERIMENT.md).
- **Implementing a task?** Pick one up from [TODO.md](../TODO.md), follow [the research workflow](RESEARCH-LAB.md), and check [ARCHITECTURE.md](ARCHITECTURE.md) for the crate you touch.
- **Driving the CLI as an agent?** See [AGENT-PROTOCOL.md](AGENT-PROTOCOL.md) for commands, flags, roles, and journals.

## Page index

| Doc | Contents |
|---|---|
| [RESEARCH-LAB.md](RESEARCH-LAB.md) | Research approach, current CLI session, evidence reading and agent handoff |
| [ARCHITECTURE.md](ARCHITECTURE.md) | Crate diagram and knob-turn data flow |
| [LAYERS.md](LAYERS.md) | Ontology tower and smallest empirical stuff |
| [KNOBS.md](KNOBS.md) | Inventory of default-lab knobs |
| [THEORIES.md](THEORIES.md) | SM, GR, strings, observer-geometry |
| [AGENT-PROTOCOL.md](AGENT-PROTOCOL.md) | CLI and library protocol |
| [STRING-EXPERIMENT.md](STRING-EXPERIMENT.md) | How to run and read the first lab |

Contracts live in `/specs`. Sequencing lives in `/plans`. Standing orders live in `/AGENTS.md`.

For implementation, start at [TODO.md](../TODO.md). The next-stage contracts
are [independent reproduction](../specs/021-independent-reproduction.md)
and [research campaigns](../specs/022-research-campaigns.md); both describe
work still to be implemented, not additional current CLI commands.
