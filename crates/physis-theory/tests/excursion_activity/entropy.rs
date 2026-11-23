//! Independently propagate jump probabilities and sum entropy on edges.
use super::{equilibrium_fit, measure, ring, sum, visits, EquilibriumFit, Traffic, ONE, ZERO};
use num_rational::BigRational;
use num_traits::ToPrimitive;
use physis_core::claim::VerdictKind;
use physis_core::knob::{KnobValue, Knobbed};
use physis_numeric::Ratio;
use physis_theory::excursion::entropy::{
    one_jump_absent_minimum, support_plane_entropy, two_tail_entropy,
};
use physis_theory::excursion::square::ExcursionSquare;
use physis_theory::Theory;

// Row sums are stationary visits per excursion, reconstructed from the jump law.
fn square(p: Ratio, z: Ratio, x: Ratio, y: Ratio) -> Traffic {
    for v in [p, z, x, y] {
        assert!(v > ZERO && v < ONE);
    }
    let b = (p * x + (ONE - p) * y) / (ONE - z * x - (ONE - z) * y);
    let left = p + b * z;
    let right = ONE + b - left;
    vec![
        vec![ZERO, p, ZERO, ONE - p],
        vec![left * (ONE - x), ZERO, left * x, ZERO],
        vec![ZERO, b * z, ZERO, b * (ONE - z)],
        vec![right * (ONE - y), ZERO, right * y, ZERO],
    ]
}

fn tails(flow: &Traffic, count: usize) -> Vec<Ratio> {
    let h = visits(flow);
    let mut mass: Vec<_> = flow[0].iter().map(|&x| x / h[0]).collect();
    (0..count)
        .map(|_| {
            mass = (0..h.len())
                .map(|j| {
                    if j == 0 {
                        ZERO
                    } else {
                        sum((1..h.len()).map(|i| mass[i] * flow[i][j] / h[i]))
                    }
                })
                .collect();
            sum(mass.iter().copied())
        })
        .collect()
}

#[test]
fn every_square_bias_pair_matches_independent_flux_entropy_and_equality() {
    let mut model = ExcursionSquare::default();
    let claims = model.claims();
    let half = Ratio::new(1, 2);
    let mut positives = 0;
    let mut minima = 0;
    for u in -11..=11 {
        model.set("continuation_bias", KnobValue::Int(u)).unwrap();
        for v in -11..=11 {
            model.set("entry_bias", KnobValue::Int(v)).unwrap();
            let u = Ratio::new(u.into(), 24);
            let v = Ratio::new(v.into(), 24);
            let f = square(half + v, half - v, half + u, half - u);
            assert_eq!(visits(&f), vec![ONE; 4]);
            let data = measure(&f);
            let m = ONE - data.q;
            assert_eq!((m, data.mean), (data.r, Ratio::int(2)));
            let floor = one_jump_absent_minimum(m, data.mean).unwrap();
            assert!(data.entropy + 1e-12 >= floor);
            let counted = [floor > 0.0, (floor - data.entropy).abs() < 1e-12];
            positives += usize::from(counted[0]);
            minima += usize::from(counted[1]);
            for (claim, holds) in claims.iter().zip(counted) {
                assert_eq!(model.evaluate(claim).kind == VerdictKind::Holds, holds);
            }
        }
    }
    assert_eq!((positives, minima), (242, 45));
}

#[test]
fn a_fixed_square_clock_and_all_four_masses_survive_the_drive_in_each_regime() {
    let half = Ratio::new(1, 2);
    for (b, m) in [
        (half, Ratio::new(3, 8)),
        (ONE, Ratio::new(5, 8)),
        (Ratio::int(2), Ratio::new(3, 4)),
        (Ratio::int(3), Ratio::new(1, 8)),
        (Ratio::new(2, 3), Ratio::new(1, 3)),
        (Ratio::int(2), Ratio::new(2, 3)),
    ] {
        let c = b / (ONE + b);
        let t = ONE - m / b;
        let (weight, initial_spread) = if m > c {
            // Each example here has rational minimizing winding.
            let d = if b == half { Ratio::new(1, 4) } else { half };
            assert_eq!(d.pow(2), ONE - (ONE + b) * (ONE - m));
            (half, Ratio::int(2) * d / (ONE + b))
        } else if m < c {
            let y = m * half;
            let x = (b * (ONE - y) - m) / (b * (ONE - y) - y);
            let p = (m - y) / (x - y);
            (p / ((ONE - x) * (ONE + b)), x - y)
        } else {
            (half, ZERO)
        };
        let limit = (c / weight).min((ONE - c) / (ONE - weight));
        let mut previous = -1.0;
        for step in [ZERO, half, Ratio::new(9, 10), Ratio::new(999, 1000)] {
            let spread = initial_spread + (limit - initial_spread) * step;
            let x = c + (ONE - weight) * spread;
            let y = c - weight * spread;
            let (p, z) = if spread == ZERO {
                (weight, weight)
            } else {
                ((m - y) / spread, (t - y) / spread)
            };
            let f = square(p, z, x, y);
            assert_eq!(
                visits(&f),
                [ONE, weight * (ONE + b), b, (ONE - weight) * (ONE + b)]
            );
            let measured = measure(&f);
            assert_eq!(
                (ONE - measured.q, measured.r, measured.mean),
                (m, m, Ratio::int(2) * b)
            );
            for (i, prob) in tails(&f, 8).into_iter().enumerate() {
                assert_eq!(prob, m * t.pow((i / 2) as u32));
            }
            if step == ZERO {
                assert!(
                    (measured.entropy - one_jump_absent_minimum(m, measured.mean).unwrap()).abs()
                        < 2e-13
                );
            }
            assert!(measured.entropy > previous);
            previous = measured.entropy;
        }
    }
}

#[test]
fn a_missing_single_jump_allows_longer_odd_excursions_and_still_bounds_entropy() {
    let mut positive = 0;
    for winding in 0..16 {
        for extra in 1..=8 {
            let mut flow = ring(16 + winding, 16 - winding, 0);
            for row in &mut flow {
                row.push(ZERO);
            }
            flow.push(vec![ZERO; 5]);
            for i in [1, 2] {
                flow[i][4] = Ratio::int(extra);
                flow[4][i] = Ratio::int(extra);
            }
            // The 1-2-4-1 cycle is wholly hidden. Its directed traffic
            // preserves stationarity and includes a positive three-jump word.
            for (i, j) in [(1, 2), (2, 4), (4, 1)] {
                flow[i][j] = flow[i][j] + Ratio::int(extra);
            }
            let data = measure(&flow);
            let tail = tails(&flow, 4);
            assert_eq!(tail[0], tail[1]);
            assert!(tail[2] > tail[3]);
            let floor = one_jump_absent_minimum(ONE - data.q, data.mean).unwrap();
            positive += usize::from(floor > 0.0);
            assert!(data.entropy + 1e-12 >= floor);
        }
    }
    assert!(positive > 0 && positive < 128);
}

#[test]
fn the_three_jump_probability_rules_out_an_otherwise_feasible_equilibrium() {
    // Four-state ring (clockwise 4, reverse 2), joined to a fifth site.
    let mut f = ring(4, 2, 0);
    for row in &mut f {
        row.push(ZERO);
    }
    f.push(vec![ZERO; 5]);
    for i in [1, 2] {
        f[i][4] = ONE;
        f[4][i] = ONE;
    }
    for (i, j) in [(1, 2), (2, 4), (4, 1)] {
        f[i][j] = f[i][j] + ONE;
    }
    let data = measure(&f);
    let tail = tails(&f, 4);
    let m = ONE - data.q;
    let odd = tail[2] - tail[3];
    assert_eq!(
        (m, data.r, data.mean, odd),
        (
            Ratio::new(11, 18),
            m,
            Ratio::new(19, 6),
            Ratio::new(19, 576)
        )
    );
    assert_eq!(equilibrium_fit(m, m, data.mean), EquilibriumFit::Attained);
    let lower = one_jump_absent_minimum(m, data.mean - odd).unwrap();
    assert!(lower > 0.01169 && lower < 0.01170 && data.entropy > lower);
}

#[test]
fn symmetrized_networks_reproduce_both_quadratic_tail_changes() {
    // Asymmetric currents on a connected ring and on a hidden triangle.
    // All symmetric traffic varies by edge, including edges remote from 0.
    let mut positive = 0;
    for size in 4..=7 {
        for seed in 1..=32 {
            let mut f = vec![vec![ZERO; size]; size];
            for (i, row) in f.iter_mut().enumerate() {
                for (j, edge) in row.iter_mut().enumerate() {
                    let (lo, hi) = (i.min(j), i.max(j));
                    if hi == lo + 1
                        || (lo == 0 && hi + 1 == size)
                        || (lo == 1 && hi == 3)
                        || (hi != lo && (lo + hi + seed) % 3 == 0)
                    {
                        *edge = Ratio::int((1 + (lo + 3 * hi + seed) % 5) as i128);
                    }
                }
            }
            for (i, row) in f.iter_mut().enumerate() {
                let next = (i + 1) % size;
                row[next] = row[next] + Ratio::int((seed % 13) as i128);
            }
            for (i, j) in [(1, 2), (2, 3), (3, 1)] {
                f[i][j] = f[i][j] + Ratio::int((seed % 7) as i128);
            }
            let h = visits(&f);
            let sym: Traffic = (0..size)
                .map(|i| {
                    (0..size)
                        .map(|j| (f[i][j] + f[j][i]) / Ratio::int(2))
                        .collect()
                })
                .collect();
            let base = measure(&sym);
            let data = measure(&f);
            let s: Vec<_> = (0..size).map(|i| sym[0][i] / h[i]).collect();
            let j: Vec<_> = (0..size).map(|i| (f[0][i] - sym[0][i]) / h[i]).collect();
            let x = sum((1..size).map(|i| h[i] * j[i].pow(2))) / h[0];
            assert_eq!(base.q - data.q, x);
            let mut shift = ZERO;
            for i in 1..size {
                for k in 1..size {
                    shift = shift
                        + j[i]
                            * (sym[i][k] * j[k]
                                - (f[i][k] - sym[i][k]) * (Ratio::int(2) * s[k] - ONE));
                }
            }
            assert_eq!(data.r - base.r, shift / h[0]);
            assert_eq!(base.mean, data.mean);
            let two = two_tail_entropy(ONE - data.q, data.r)
                .unwrap()
                .to_f64()
                .unwrap();
            assert!(data.entropy + 1e-12 >= two);
            for a in [Ratio::new(1, 2), Ratio::new(3, 4), Ratio::new(9, 11)] {
                let lower = support_plane_entropy(ONE - data.q, data.r, data.mean, a)
                    .unwrap()
                    .to_f64()
                    .unwrap();
                assert!(data.entropy + 1e-12 >= lower);
                positive += usize::from(lower > 0.0);
            }
        }
    }
    assert!(positive > 0);
}

#[test]
fn plane_lower_bound_has_linear_order_along_a_driven_reversible_contact() {
    let a = Ratio::new(1, 2);
    let t = a.pow(2);
    let s = a.pow(3) / (ONE + a - t);
    let w = Ratio::new(1, 2);
    let mut previous = f64::INFINITY;
    for divisor in [50, 100, 1000, 10000] {
        let eps = Ratio::new(1, divisor);
        let mut f = vec![vec![ZERO; 5]; 5];
        for (i, j, g) in [
            (0, 1, w / Ratio::int(2)),
            (1, 2, w * t / (Ratio::int(2) * (ONE - t))),
            (2, 0, w / Ratio::int(2)),
        ] {
            f[i][j] = g + eps;
            f[j][i] = g - eps;
        }
        for (i, j, g) in [(0, 3, ONE - w), (3, 4, (ONE - w) * s / (ONE - s))] {
            f[i][j] = g;
            f[j][i] = g;
        }
        let data = measure(&f);
        let x = Ratio::int(4) * (ONE - t) * eps.pow(2) / w;
        let m = ONE - data.q;
        assert_eq!(m, w * t + (ONE - w) * s + x);
        assert_eq!(data.r, w * t.pow(2) + (ONE - w) * s - (ONE - t) * x);
        assert_eq!(data.mean, Ratio::new(5, 18));
        let expected = Ratio::int(32) * a.pow(2) * (ONE - a).pow(2) * x
            / (Ratio::int(11) * (ONE + Ratio::int(2) * a - t));
        let lower = support_plane_entropy(m, data.r, data.mean, a).unwrap();
        assert_eq!(lower, BigRational::from(expected));
        assert!(expected > ZERO && expected.to_f64() < data.entropy && data.entropy < previous);
        assert!(data.entropy / eps.pow(2).to_f64() < 100.0);
        previous = data.entropy;
    }
    let ring = measure(&ring(3, 2, 1));
    let lower = support_plane_entropy(ONE - ring.q, ring.r, ring.mean, Ratio::new(9, 11)).unwrap();
    assert_eq!(lower, BigRational::from(Ratio::new(2189228, 172485621)));
    assert!(lower.to_f64().unwrap() < ring.entropy);
    let error = Ratio::new(1, 1000);
    let conservative = support_plane_entropy(
        ONE - ring.q - error,
        ring.r - error,
        ring.mean + error,
        Ratio::new(9, 11),
    )
    .unwrap();
    assert_eq!(
        conservative,
        BigRational::from(Ratio::new(177971224, 21560702625))
    );
}

#[test]
fn two_tails_alone_certify_linear_entropy_cost_at_a_geometric_contact() {
    let two = Ratio::int(2);
    let target = two_tail_entropy(Ratio::new(2, 5), Ratio::new(1, 8)).unwrap();
    assert_eq!(target, BigRational::from(Ratio::new(28, 275)));
    for t in [Ratio::new(1, 4), Ratio::new(3, 4)] {
        for eps in [Ratio::new(1, 100), Ratio::new(1, 1000)] {
            let mut f = vec![vec![ZERO; 3]; 3];
            for (i, j, g) in [
                (0, 1, ONE / two),
                (1, 2, t / (two * (ONE - t))),
                (2, 0, ONE / two),
            ] {
                f[i][j] = g + eps;
                f[j][i] = g - eps;
            }
            let data = measure(&f);
            let m = ONE - data.q;
            let x = Ratio::int(4) * (ONE - t) * eps.pow(2);
            let deficit = m.pow(2) - data.r;
            assert_eq!(deficit, (ONE + t) * x + x.pow(2));
            let lower = two_tail_entropy(m, data.r).unwrap().to_f64().unwrap();
            assert!(lower >= 2.0 * deficit.to_f64() && lower < data.entropy);
            assert!(data.entropy < 25.0 * deficit.to_f64());
        }
    }
}

#[test]
fn exact_decisions_and_displays_handle_unreduced_and_extreme_counts() {
    let wide = i128::MAX / 160;
    let scaled = |n: i128, d: i128| Ratio {
        num: n * wide,
        den: d * wide,
    };
    assert_eq!(
        equilibrium_fit(scaled(7, 40), scaled(13, 160), scaled(5, 18)),
        EquilibriumFit::Attained
    );
    assert_eq!(
        equilibrium_fit(scaled(23, 36), scaled(11, 24), scaled(2, 1)),
        EquilibriumFit::PositiveFloor
    );
    let tiny = Ratio::new(1, i128::MAX);
    let near_one = ONE - tiny;
    let s = one_jump_absent_minimum(near_one, Ratio::int(2)).unwrap();
    assert!(s.is_finite() && s > 100.0);
    assert_eq!(one_jump_absent_minimum(tiny, Ratio::int(2) * tiny), None);
    assert_eq!(one_jump_absent_minimum(ZERO, ZERO), Some(0.0));
    let bad = Ratio { num: 1, den: 0 };
    assert_eq!(
        equilibrium_fit(bad, ONE, ONE),
        EquilibriumFit::InvalidCounts
    );
    assert_eq!(one_jump_absent_minimum(bad, ONE), None);
    assert_eq!(support_plane_entropy(tiny, tiny, ONE, bad), None);
}
