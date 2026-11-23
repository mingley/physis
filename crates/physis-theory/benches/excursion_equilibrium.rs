//! Time the same exact decisions on precomputed ring and near-frontier data.
use physis_numeric::Ratio;
use physis_theory::excursion::equilibrium::{equilibrium_fit, EquilibriumFit};
use std::hint::black_box;
use std::time::Instant;

fn measure(name: &str, cases: &[[Ratio; 3]], repeats: usize) {
    let start = Instant::now();
    let mut counts = [0usize; 4];
    for _ in 0..repeats {
        for &[m, r, n] in black_box(cases) {
            let index = match equilibrium_fit(m, r, n) {
                EquilibriumFit::Attained => 0,
                EquilibriumFit::ZeroInfimum => 1,
                EquilibriumFit::PositiveFloor => 2,
                EquilibriumFit::InvalidCounts => 3,
            };
            counts[index] += 1;
        }
    }
    let decisions = repeats * cases.len();
    let ns = start.elapsed().as_nanos() as f64 / decisions as f64;
    println!("{name} {decisions} decisions {ns:.1} ns/decision {counts:?}");
}

fn main() {
    let mut rings = Vec::new();
    for a in 1..=8 {
        for b in 1..=8 {
            for c in 1..=8 {
                let total: i128 = a + b + c;
                let m = Ratio::int(1) - Ratio::new(2 * a * b + c * c, total.pow(2));
                let r = m - Ratio::new(3 * c * (a * a + b * b), total.pow(3));
                rings.push([m, r, Ratio::int(2)]);
            }
        }
    }
    let mut contacts = Vec::new();
    for a in [Ratio::new(1, 3), Ratio::new(1, 2), Ratio::new(4, 5)] {
        let one = Ratio::int(1);
        let t = a.pow(2);
        let s = a.pow(3) / (one + a - t);
        for w in [Ratio::new(1, 9), Ratio::new(1, 2), Ratio::new(7, 8)] {
            let m = w * t + (one - w) * s;
            let r = w * t.pow(2) + (one - w) * s;
            let n = w * t / (one - t) + Ratio::int(2) * (one - w) * s / (one - s);
            for delta in [-1, 0, 1] {
                contacts.push([m, r, n + Ratio::new(delta, 10i128.pow(25))]);
            }
        }
    }
    measure("rings", &rings, 60);
    measure("contacts", &contacts, 120);
}
