# 009 — Computation

Status: active
Layer: information / mathematical
Id: `computation`

## Purpose

Computation is the **second domain reuse** (after electromagnetism), and the
third scientific domain overall. It further proves the substrate is
general: the same knobs, claims, verdicts, `Theory` trait, and experiment
matrix host models of computation.

Its payoff is the **halting problem**. The lab already has an `Undecidable`
verdict kind; here it means what Turing meant. An unbounded-tape Turing
machine's `comp.halts` claim is `Undecidable`, and bounding the tape
mechanically restores decidability — a clean knob → verdict diff.

## Objects

| id | object |
|---|---|
| `combinational-circuit` | a finite, acyclic boolean circuit |
| `turing-machine` | a deterministic Turing machine (`tape_bound` knob) |
| `landauer-engine` | a computation coupled to a heat bath (Landauer/Bennett) |

## Knobs

| theory | knob | effect |
|---|---|---|
| `turing-machine` | `tape_bound` | tape length in cells; `0` = unbounded. A finite bound makes the machine a finite automaton. |
| `turing-machine` | `nondeterministic` | whether the transition relation allows nondeterministic branching; flips `comp.deterministic`. |
| `landauer-engine` | `temperature_k` | bath temperature (K); sets the energy scale `k_B·T·ln2`. The `ln2` factor is not this knob: `add-kt` is an IR mutation. |
| `landauer-engine` | `bits_erased` | number of logical bits irreversibly erased. |
| `landauer-engine` | `reversible` | logical reversibility (Bennett): erases nothing, so the process can be free. This stays a knob. |

`combinational-circuit` has no knobs. Its NAND netlist lives on the IR
package. `add-feedback` appends a cyclic NAND and is an IR mutation.
`add-contention` appends a second NAND on the same output wire and is
a second IR mutation: `comp.deterministic` fails. That is still
`combinational-circuit`, not a silent Turing-machine install.
`turing-machine` keeps `nondeterministic`.

## Claims

| id | meaning |
|---|---|
| `comp.halts` | the machine halts on every input |
| `comp.turing-complete` | the model is Turing complete |
| `comp.deterministic` | the transition function is single-valued. On `combinational-circuit` this names unique NAND drivers. `add-contention` appends a second NAND on the same wire and the cell fails. That is not a knob. TM's copy stays encoding-wide; `nondeterministic` stays a knob |
| `comp.decidable-equivalence` | equivalence of two instances is decidable |
| `comp.resource-bounded` | the computation runs within an a priori resource bound |
| `comp.feasible-decision` | a resource-feasible procedure in this lab decides the instance |
| `comp.p-equals-np` | P = NP — encoded as `undecidable`/`open`, an honest unknown |
| `comp.acyclic` | NAND gate graph has no cycle (combinational-circuit IR netlist) |
| `info.landauer-cost` | erasing a bit dissipates at least `k_B·T·ln2` (named domain: kT ln2 Landauer bound). **holds** on the live encoding (`erase kT ln2`). `add-kt` appends `erase kT` and the encoding energy is `N kT`, so this cell fails. That is not a knob. `reversible` / `bits_erased` stay knobs |
| `info.thermodynamically-free` | the process erases nothing and can dissipate no heat |

## Landauer's principle: the computation ↔ thermodynamics bridge

`landauer-engine` is the first object that reuses substrate from **two**
domains at once. Landauer's principle (1961) says erasing one logical bit
dissipates at least `k_B·T·ln2` of energy; Bennett (1973) showed a logically
reversible computation erases nothing and can approach zero dissipation.

The energy is **computed from the typed Boltzmann constant**, so its units are
checked at compile time: `k_boltzmann()` carries `J/K` and `kelvin(T)` carries
`K`, so the product `E_min = N·k_B·T·ln2` is a `Qty<Energy>` — a mass added to a
length would not compile. `info.landauer-cost` holds on the live `kT ln2`
encoding as a computed bound (one bit at 300 K is `2.871e-21 J`), with a
named DomainOfValidity. Dropping `ln2` is a package mutation (`add-kt`):
the encoding energy is `N kT` and the cell fails. That is not a knob.
When `effective_bits` is zero both encodings give zero, so the cost still
holds; hypothesize is asserted at the default irreversible one-bit engine.

The knob → verdict diff is cross-domain:

```
physis run landauer-engine        # info.thermodynamically-free: fails (erases 1 bit)
physis set landauer-engine reversible true
physis hypothesize landauer-engine  # add-kt is IR, not set
```

flips `info.thermodynamically-free` `fails → holds`: a reversible computation
erases nothing, so the Landauer floor is zero and the process can be free.
Setting `bits_erased 0` is the other route to a free process; raising
`bits_erased` or `temperature_k` scales the computed dissipation linearly.
`add-kt` flips `info.landauer-cost` `holds → fails` without converting
`reversible`.

## Honest unknowns: P vs NP

`comp.p-equals-np` is `undecidable` with epistemic tag `open` for the Turing
machine. This is deliberate: the lab refuses to record `holds` or `fails` for a
famous open problem. It is `inapplicable` to a single fixed circuit (P vs NP is
about uniform machine models). This is the epistemic honesty the whole project
is built on — an `open` verdict is a first-class, respected outcome.

## The knob → verdict diff

```
physis experiment computation
physis set turing-machine tape_bound 1000
```

Setting `tape_bound: 0 → 1000` flips:

- `comp.halts` `undecidable → holds` (a finite tape has finitely many
  configurations; halting is decidable by cycle detection),
- `comp.turing-complete` `holds → fails` (a bounded machine is a finite
  automaton),
- `comp.decidable-equivalence` `undecidable → holds`,
- `comp.resource-bounded` `fails → holds`.
- `comp.feasible-decision` `inapplicable → undecidable` (the configuration
  graph is now finite, so the problem is in range, but this lab does not
  enumerate it; `GapReason::ComputationallyIntractable`).

The combinational circuit is a finite NAND netlist described by an IR
package (`equation nand a b -> out`). Acyclicity is a graph property of
that netlist (`comp.acyclic`). Unique drivers are a graph property of
the same netlist (`comp.deterministic`). `physis hypothesize combinational-circuit`
forks the package with a feedback equation or a second driver on an
existing net; those are not knobs (`set combinational-circuit feedback`
and `set combinational-circuit contention` are unknown). A cyclic
encoding makes `comp.halts` inapplicable (out of the combinational
domain). A multi-driven encoding leaves the graph acyclic and fails
determinism. No SAT solver and no tape simulator are run.

## No borrowed spacetime

Computation has no spacetime, gauge, or spectrum, so these theories return
`None` from `Theory::world()` (whose return type is `Option<World>`) and
describe themselves via `Theory::note()`. The `physis score` command reports a
computational object as a non-physics domain rather than grading it against the
physics empirical target. Earlier drafts used a degenerate placeholder world;
that rough edge has been removed.

## Non-goals (this milestone)

- An interpreter / simulator that evaluates NAND gates or tape steps.
  Cycle detection on the netlist is topology, not simulation.
- Complexity-class claims (P, NP, …) as `holds` / `fails` verdicts. `comp.p-equals-np`
  stays `open`. `comp.feasible-decision` is the first complexity *gap*:
  coNP-complete / exponential search is `ComputationallyIntractable`, not Rice.

## Related

- `specs/007-reuse-domains.md` — how domains are added
- `specs/008-electromagnetism.md` — the first reuse
- `plans/004-m3-domain-reuse.md` — the milestone
