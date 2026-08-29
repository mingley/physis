# String critique lab

See `specs/005-string-critique.md`.

## How to run

```
cargo run -p physis -- experiment string-critique
```

## How to poke it

```
cargo run -p physis -- set type-iib total_dim 9
cargo run -p physis -- set type-iib flux_bits 0
cargo run -p physis -- set type-iib supersymmetry false
cargo run -p physis -- set bosonic total_dim 10
cargo run -p physis -- set observer-geometry unique_vacuum false
cargo run -p physis -- set observer-geometry derive_gauge false
cargo run -p physis -- set standard-model generations 2
```

Each line is a one-shot lab (fresh defaults). Library users keep a `Lab` alive to sequence turns.

## How to read the matrix

- `holds` + `theorem` — trust this cell as a fact about the *encoding of the construction*
- `holds` + `encoded-fact` — textbook result stored as data
- `holds` + `heuristic` — folklore, knob-sensitive
- `holds` + `conjecture` — the program asserts it; we did not derive it
- `fails` — the current knobs violate the claim
- `undecidable` — we refused to guess (e.g. generation count from CY topology)
- `inapplicable` — the claim is about a different kind of object

## The Weinstein-shaped question

Public argument, compressed: string theory traded unique geometry for a landscape; that was a wrong turn.

In this lab that argument becomes:

- default superstring knobs → `predictivity.unique-vacuum` **fails** (heuristic)
- observer-geometry default knobs → `predictivity.unique-vacuum` **holds** (conjecture/axiom)
- observer-geometry empirical contact → **undecidable / conjecture**, not earned

### "Why three generations?", made mechanical

`empirical.three-generations` is `undecidable` for a superstring until you choose
a compactification topology via the `euler_number` knob. Then generations =
`|χ|/2` is a real topological count:

```
cargo run -p physis -- set heterotic-e8e8 euler_number 6   # |χ|/2 = 3: holds
cargo run -p physis -- set heterotic-e8e8 euler_number 8   # |χ|/2 = 4: fails
```

This is exactly the accommodate-vs-derive distinction: the theory can *fit* three
generations by choosing χ = ±6, but nothing here *derives* why χ = ±6. The knob
is the choice the critique says is unexplained.

So the critique is *visible*, and the alternative has not yet *won*. That is the correct v0 state. Agents are invited to change it by encoding more mechanism, not by turning the uniqueness axiom on harder.
