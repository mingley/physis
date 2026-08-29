# 001 — Type system

Status: active
Layer: mathematical

## Quantities

A quantity is `Qty<D>` where `D` is an SI dimension vector encoded with `typenum` integers:

```
SI<M, L, T, I, Θ, N, J>
```

- Addition and subtraction require identical `D`.
- Multiplication *adds* exponents (type-level `Add`).
- Division *subtracts* exponents (type-level `Sub`).
- Scaling by `f64` preserves `D`.

Named derived dimensions used by the lab include `Action` (`h`, `ħ`),
`EnergyDensity` (same exponents as `Pressure`: J/m³ = Pa),
`SpectralEnergyDensity` (J m⁻³ Hz⁻¹), `StefanBoltzmann` (W m⁻² K⁻⁴), and
`RadiationConstant` (`a` in `u = a T⁴`). Energy density is *not* energy:
assigning one to the other is a compile-fail contract.

Numeric payload is `f64` in SI base units. Uncertainty is not in `Qty`; it belongs on claims.

## Why typenum

Stable Rust does not allow `{M1+M2}` in a type without `generic_const_exprs`. `typenum` gives type-level integers and `Add`/`Sub` on stable. That is the mechanical core of “you cannot add mass to length.”

## Compile-fail contract

This must not compile:

```rust,ignore
use physis_core::qty::{kg, meters};
let _ = kg(1.0) + meters(1.0);
```

This must compile, and equal 9 J:

```rust
use physis_core::qty::{kg, meters_per_second};
let k = kg(2.0) * meters_per_second(3.0) * meters_per_second(3.0) * 0.5;
assert!((k.value() - 9.0).abs() < 1e-12);
```

## Knobs vs types

Some structure is type-level (dimensions). Some structure is runtime (an agent sets `total_dim` to 9). Both are required:

- type-level: illegal *kinds* of mixing
- runtime knobs: illegal *values* inside a kind, with domain checks, producing verdict diffs

Do not collapse one into the other. A const-generic `Superstring<10>` is a fine *additional* encoding; it does not replace the agent-facing knob.

## Identifiers

`LayerId`, `ClaimId`, `KnobId`, `TheoryId` are the stable names journals and agents use. Claim ids that appear in experiments are shared strings (see `physis_theory::claims`) so matrices align.

## Serialization

`KnobValue` is the protocol scalar. Domains are static (`&'static KnobSpec`). Do not deserialize a domain from an agent; look it up from the theory.
