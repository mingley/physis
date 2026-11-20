//! Independent checks of docs/research/excursion-activity/RESULT.md.
//! Every matrix contains balanced stationary edge traffic in arbitrary common
//! units. Normalization by departures from 0 converts it to per-excursion units.

use physis_core::claim::VerdictKind;
use physis_core::knob::{KnobValue, Knobbed};
use physis_numeric::Ratio;
use physis_theory::excursion::equilibrium::{equilibrium_fit, EquilibriumFit};
use physis_theory::excursion::ring::ExcursionRing;
use physis_theory::Theory;

const ZERO: Ratio = Ratio::int(0);
const ONE: Ratio = Ratio::int(1);

type Traffic = Vec<Vec<Ratio>>;

fn sum(values: impl IntoIterator<Item = Ratio>) -> Ratio {
    values.into_iter().fold(ZERO, |a, b| a + b)
}

fn visits(flow: &Traffic) -> Vec<Ratio> {
    let weights: Vec<_> = flow.iter().map(|row| sum(row.iter().copied())).collect();
    for (i, w) in weights.iter().enumerate() {
        assert_eq!(*w, sum(flow.iter().map(|row| row[i])));
        assert!(*w > ZERO);
    }
    weights
}

struct Counts {
    q: Ratio,
    mean: Ratio,
    r: Ratio,
    entropy: f64,
}

fn measure(flow: &Traffic) -> Counts {
    let h = visits(flow);
    let freq = h[0];
    let mean = sum(h.iter().skip(1).copied()) / freq - ONE;
    let q = sum((1..flow.len()).map(|i| (flow[0][i] / freq) * (flow[i][0] / h[i])));
    // Propagate the entrance distribution twice within the hidden set.
    let mut mass: Vec<_> = flow[0].iter().map(|&x| x / freq).collect();
    for _ in 0..2 {
        mass = (0..flow.len())
            .map(|j| {
                if j == 0 {
                    ZERO
                } else {
                    sum((1..flow.len()).map(|i| mass[i] * flow[i][j] / h[i]))
                }
            })
            .collect();
    }
    let mut entropy = 0.0;
    for (i, row) in flow.iter().enumerate() {
        for (j, &a) in row.iter().enumerate() {
            let b = flow[j][i];
            assert_eq!(a == ZERO, b == ZERO);
            if a != ZERO {
                entropy += (a / freq).to_f64() * (a / b).to_f64().ln();
            }
        }
    }
    Counts {
        q,
        mean,
        r: sum(mass),
        entropy,
    }
}

// Independent evaluation of the sharp witness, via its three edge contributions.
fn least_entropy(q: Ratio, mean: Ratio) -> f64 {
    let d2 = ONE - (ONE + mean) * q;
    if d2 <= ZERO {
        return 0.0;
    }
    let d = d2.to_f64().sqrt();
    let n = mean.to_f64();
    d * (2.0 * ((1.0 + d) / (1.0 - d)).ln() + ((n + d) / (n - d)).ln())
}

fn ring(clockwise: i128, anticlockwise: i128, opposite: i128) -> Traffic {
    (0..4)
        .map(|i| {
            (0..4)
                .map(|j| Ratio::int([0, clockwise, opposite, anticlockwise][(j + 4 - i) % 4]))
                .collect()
        })
        .collect()
}

// The two tests below use constructive upper witnesses and supporting lower
// planes independently of the cubic-discriminant decision in the factory.
fn support_mean(a: Ratio, m: Ratio, r: Ratio) -> Ratio {
    let two = Ratio::int(2);
    let chord = ONE + a - a.pow(2);
    let b = ONE + two * a - a.pow(2);
    let scale = (ONE + a).pow(2) * (ONE - a.pow(2)).pow(2);
    ((two * chord.pow(2) - b) * m + b * r - two * a.pow(6)) / scale
}

fn branch_mean(m: Ratio, r: Ratio, y: Ratio) -> Ratio {
    let g = m - y;
    let t = (r - y) / g;
    let w = g / t;
    let s = y / (ONE - w);
    assert!((ZERO..ONE).contains(&s) && (ZERO..ONE).contains(&t));
    assert!(w > ZERO && w < ONE);
    assert_eq!(w * t + (ONE - w) * s, m);
    assert_eq!(w * t.pow(2) + (ONE - w) * s, r);
    w * t / (ONE - t) + (ONE - w) * Ratio::int(2) * s / (ONE - s)
}

#[test]
fn exact_equilibrium_decision_agrees_with_constructive_certificates_on_all_rings() {
    let mut counts = [0; 3];
    for a in 1..=8 {
        for b in 1..=8 {
            for c in 1..=8 {
                let measured = measure(&ring(a, b, c));
                let m = ONE - measured.q;
                let r = measured.r;
                let n = measured.mean;
                let v = r - m.pow(2);
                let expected = if v < ZERO {
                    EquilibriumFit::PositiveFloor
                } else if v == ZERO {
                    assert_eq!(n * (ONE - m), m);
                    EquilibriumFit::Attained
                } else {
                    let k = ONE - Ratio::int(2) * m + r;
                    let grid = (0..16).map(|i| Ratio::new(i, 16));
                    let lower = grid.clone().any(|p| support_mean(p, m, r) > n);
                    let upper = grid.clone().any(|p| branch_mean(m, r, p * v / k) <= n);
                    assert_ne!(lower, upper, "no decisive certificate at {a},{b},{c}");
                    if lower {
                        EquilibriumFit::PositiveFloor
                    } else {
                        EquilibriumFit::Attained
                    }
                };
                assert_eq!(equilibrium_fit(m, r, n), expected);
                counts[match (expected, a == b) {
                    (EquilibriumFit::Attained, true) => 0,
                    (EquilibriumFit::Attained, false) => 1,
                    (EquilibriumFit::PositiveFloor, false) => 2,
                    _ => panic!("misclassified symmetric ring {a},{b},{c}"),
                }] += 1;
            }
        }
    }
    assert_eq!(counts, [64, 128, 320]);
}

#[test]
fn rational_contacts_and_nearby_means_include_the_double_root() {
    for a in [Ratio::new(1, 3), Ratio::new(1, 2), Ratio::new(4, 5)] {
        for w in [Ratio::new(1, 9), Ratio::new(1, 2), Ratio::new(7, 8)] {
            let t = a.pow(2);
            let s = a.pow(3) / (ONE + a - t);
            let m = w * t + (ONE - w) * s;
            let r = w * t.pow(2) + (ONE - w) * s;
            let n = w * t / (ONE - t) + (ONE - w) * Ratio::int(2) * s / (ONE - s);
            assert_eq!(support_mean(a, m, r), n);
            assert_eq!(equilibrium_fit(m, r, n), EquilibriumFit::Attained);
            let eps = Ratio::new(1, 10_i128.pow(25));
            assert_eq!(
                equilibrium_fit(m, r, n - eps),
                EquilibriumFit::PositiveFloor
            );
            assert_eq!(equilibrium_fit(m, r, n + eps), EquilibriumFit::Attained);
        }
    }
    let m = Ratio::new(23, 36);
    let r = Ratio::new(11, 24);
    assert!(support_mean(Ratio::new(20513, 25000), m, r) > Ratio::new(202408322, 100000000));
    assert!(branch_mean(m, r, Ratio::new(863, 10000)) < Ratio::new(202408323, 100000000));
}

#[test]
fn reversible_sparse_networks_including_interior_edges_admit_equilibrium() {
    for size in 3..=8 {
        for mask in 0_u32..64 {
            let mut traffic = vec![vec![ZERO; size]; size];
            for (i, row) in traffic.iter_mut().enumerate() {
                for (j, edge) in row.iter_mut().enumerate() {
                    let (lo, hi) = (i.min(j), i.max(j));
                    if lo != hi && (lo + 1 == hi || mask & (1 << ((i + j) % 6)) != 0) {
                        *edge = Ratio::int(1 + ((3 * hi + lo + mask as usize) % 7) as i128);
                    }
                }
            }
            let data = measure(&traffic);
            assert_eq!(data.entropy, 0.0);
            assert_eq!(
                equilibrium_fit(ONE - data.q, data.r, data.mean),
                EquilibriumFit::Attained
            );
        }
    }
}

#[test]
fn geometric_tail_boundary_has_zero_infimum_at_fixed_excess_mean() {
    let m = Ratio::new(1, 2);
    let r = m.pow(2);
    let n = Ratio::int(2);
    assert_eq!(equilibrium_fit(m, r, n), EquilibriumFit::ZeroInfimum);
    assert_eq!(equilibrium_fit(m, r, ONE), EquilibriumFit::Attained);
    assert_eq!(
        equilibrium_fit(m, r, Ratio::new(9, 10)),
        EquilibriumFit::PositiveFloor
    );

    let mut previous = f64::INFINITY;
    for denominator in [100, 1000, 10000, 100000] {
        let t = ONE - Ratio::new(1, denominator);
        let w = (ONE - t) * (n * (ONE - r) - m - r)
            / (t.pow(3) - r - m * (ONE - t) + n * (ONE - t) * (ONE - t.pow(2)));
        let short_m = (m - w * t) / (ONE - w);
        let short_r = (r - w * t.pow(2)) / (ONE - w);
        let short_n = (short_m + short_r) / (ONE - short_r);
        assert!(ZERO < w && w < ONE && ZERO < short_r && short_r < short_m && short_m < ONE);
        assert_eq!(
            (
                (ONE - w) * short_m + w * t,
                (ONE - w) * short_r + w * t.pow(2)
            ),
            (m, r)
        );
        assert_eq!((ONE - w) * short_n + w * t / (ONE - t), n);
        assert_eq!(short_m.pow(2) - short_r, w * ((t - m) / (ONE - w)).pow(2));
        let s = (short_m - short_r).to_f64();
        let delta = ((ONE - short_r) * (short_m.pow(2) - short_r))
            .to_f64()
            .sqrt();
        let entropy = (ONE - w).to_f64() * delta / (ONE - short_r).to_f64()
            * ((s + delta) / (s - delta)).ln();
        // Reconstruct both jump branches, then sum their directed stationary
        // edge entropies independently of the triangle minimum formula.
        let (weight, cont, r_a) = (w.to_f64(), t.to_f64(), short_r.to_f64());
        let zero_path = (1.0 - short_m.to_f64()) / 2.0;
        let (f, b) = ((s + delta) / 2.0, (s - delta) / 2.0);
        let p = (zero_path + f) / (1.0 - r_a);
        let x = (f + r_a * zero_path) / (zero_path + f);
        let y = (b + r_a * zero_path) / (zero_path + b);
        assert!((x * y - r_a).abs() < 1e-14);
        let mut edge_entropy = 0.0;
        let mut measured = [0.0; 3];
        for (mass, entry, x, y) in [(1.0 - weight, p, x, y), (weight, 0.5, cont, cont)] {
            let h1 = mass * (entry + (1.0 - entry) * y) / (1.0 - x * y);
            let h2 = mass * (1.0 - entry + entry * x) / (1.0 - x * y);
            let edges = [
                (mass * entry, h1 * (1.0 - x)),
                (mass * (1.0 - entry), h2 * (1.0 - y)),
                (h1 * x, h2 * y),
            ];
            for (forward, reverse) in edges {
                assert!(forward > 0.0 && reverse > 0.0);
                edge_entropy += (forward - reverse) * (forward / reverse).ln();
            }
            measured[0] += mass * (entry * x + (1.0 - entry) * y);
            measured[1] += mass * x * y;
            measured[2] += h1 + h2 - mass;
        }
        for (actual, target) in measured.into_iter().zip([0.5, 0.25, 2.0]) {
            assert!((actual - target).abs() < 2e-11);
        }
        assert!((edge_entropy - entropy).abs() < 1e-12);
        assert!(entropy > 0.0 && entropy < previous);
        previous = entropy;
    }
    assert!(previous < 0.00003);
    assert_eq!(
        equilibrium_fit(m, m, Ratio::int(2)),
        EquilibriumFit::Attained
    );
    assert_eq!(
        equilibrium_fit(m, m, Ratio::new(3, 2)),
        EquilibriumFit::PositiveFloor
    );
    assert_eq!(equilibrium_fit(ZERO, ZERO, ZERO), EquilibriumFit::Attained);
    for data in [(ONE, m, n), (m, ZERO, n), (m, r, m + r), (ZERO, ZERO, n)] {
        assert_eq!(
            equilibrium_fit(data.0, data.1, data.2),
            EquilibriumFit::InvalidCounts
        );
    }
}

#[test]
fn all_four_state_ring_weights_through_eight_compare_the_two_sensors() {
    let mut classifications = [[0; 2]; 2];
    let mut joint_detections_after_two_tests = 0;
    let mut joint_detections_after_all_pairs = 0;
    let mut lab_ring = ExcursionRing::default();
    let claim = lab_ring.claims().remove(0);
    for cw in 1..=8 {
        lab_ring
            .set("clockwise", KnobValue::UInt(cw as u64))
            .unwrap();
        for ccw in 1..=8 {
            lab_ring
                .set("anticlockwise", KnobValue::UInt(ccw as u64))
                .unwrap();
            for opposite in 1..=8 {
                lab_ring
                    .set("opposite", KnobValue::UInt(opposite as u64))
                    .unwrap();
                let c = measure(&ring(cw, ccw, opposite));
                assert_eq!(c.mean, Ratio::int(2));
                let short = (ONE - c.q).pow(2) > c.r;
                let activity = (ONE + c.mean) * c.q < ONE;
                classifications[usize::from(short)][usize::from(activity)] += 1;
                assert!(least_entropy(c.q, c.mean) <= c.entropy + 1e-12);
                let (gap, bound) = joint_gap_and_bound(&c);
                assert!(bound.to_f64() <= c.entropy + 1e-12);
                let max_reversible_r = c.mean / (Ratio::int(2) + c.mean);
                let all_pairs = !short && !activity && c.r <= max_reversible_r;
                assert_eq!(
                    lab_ring.evaluate(&claim).kind == VerdictKind::Holds,
                    all_pairs && gap > ZERO
                );
                if c.r > max_reversible_r {
                    let r = c.r.to_f64();
                    let cap = max_reversible_r.to_f64();
                    let kl = r * (r / cap).ln() + (1.0 - r) * ((1.0 - r) / (1.0 - cap)).ln();
                    assert!(4.0 * kl <= c.entropy + 1e-12);
                }
                if !short && !activity && gap > ZERO {
                    joint_detections_after_two_tests += 1;
                    joint_detections_after_all_pairs += usize::from(c.r <= max_reversible_r);
                }
            }
        }
    }
    assert_eq!(classifications, [[238, 20], [10, 244]]);
    assert_eq!(
        (
            joint_detections_after_two_tests,
            joint_detections_after_all_pairs
        ),
        (28, 22)
    );
    println!("joint detections after the first two tests: {joint_detections_after_two_tests}; after all three pairs: {joint_detections_after_all_pairs}");
    println!("ring counts, rows = two-tail flag false/true, columns = mean flag false/true: {classifications:?}");
    let c = measure(&ring(5, 2, 1));
    assert_eq!((c.q, c.r), (Ratio::new(21, 64), Ratio::new(257, 512)));
    assert_eq!(
        (ONE - (ONE + c.mean) * c.q).checked_sqrt(),
        Some(Ratio::new(1, 8))
    );
    let expected = (2.0 * (9.0_f64 / 7.0).ln() + (17.0_f64 / 15.0).ln()) / 8.0;
    assert!((least_entropy(c.q, c.mean) - expected).abs() < 1e-14);
}

fn joint_gap_and_bound(c: &Counts) -> (Ratio, Ratio) {
    let m = ONE - c.q;
    let gap = (ONE + m).pow(2) - (ONE + Ratio::int(2) * c.mean) * (ONE - c.r);
    let bound = if gap > ZERO {
        Ratio::int(8) * (gap / (Ratio::int(5) + Ratio::int(2) * c.mean)).pow(2)
    } else {
        ZERO
    };
    (gap, bound)
}

#[test]
fn each_pair_has_a_reversible_witness_but_the_full_triple_forces_dissipation() {
    let observed = measure(&ring(4, 2, 1));
    assert_eq!(
        (ONE - observed.q, observed.r, observed.mean),
        (Ratio::new(32, 49), Ratio::new(164, 343), Ratio::int(2))
    );
    assert_eq!(
        joint_gap_and_bound(&observed),
        (Ratio::new(296, 2401), Ratio::new(700928, 466948881))
    );
    // Each row weights the symmetric edges 01, 02, 03 and 23, in that order.
    let controls = [[465, 1920, 1920, 5248], [1, 16, 16, 33], [15, 164, 0, 179]].map(|weights| {
        let mut flow = vec![vec![ZERO; 4]; 4];
        for ((i, j), w) in [(0, 1), (0, 2), (0, 3), (2, 3)].into_iter().zip(weights) {
            flow[i][j] = Ratio::int(w);
            flow[j][i] = flow[i][j];
        }
        let data = measure(&flow);
        assert_eq!(data.entropy, 0.0);
        assert!(joint_gap_and_bound(&data).0 <= ZERO);
        data
    });
    let [mr, mn, rn] = controls;
    assert_eq!((mr.q, mr.r), (observed.q, observed.r));
    assert_eq!((mn.q, mn.mean), (observed.q, observed.mean));
    assert_eq!((rn.r, rn.mean), (observed.r, observed.mean));
    assert_ne!(mr.mean, observed.mean);
    assert_ne!(mn.r, observed.r);
    assert_ne!(rn.q, observed.q);
}

fn add_cycle(flow: &mut Traffic, route: &[usize], weight: i128) {
    for (&from, &to) in route.iter().zip(route.iter().cycle().skip(1)) {
        flow[from][to] = flow[from][to] + Ratio::int(weight);
    }
}

#[test]
fn finite_hidden_order_scan_checks_the_floor_against_edge_entropy() {
    let mut positive = 0;
    for states in 3..=7 {
        let mut base = vec![vec![ZERO; states]; states];
        for (i, row) in base.iter_mut().enumerate() {
            for (j, traffic) in row.iter_mut().enumerate() {
                if i != j {
                    let weight = (i.max(j) + 1) * (i.min(j) + 3) % 3;
                    *traffic = Ratio::int(1 + weight as i128);
                }
            }
        }
        let reverse_ring: Vec<_> = (0..states).rev().collect();
        let inner_ring: Vec<_> = (1..states).collect();
        for a in [0, 1, 2, 4, 8, 16, 32, 64] {
            for b in [0, 1, 2, 4, 8, 16, 32, 64] {
                for c in [0, 1, 2, 4, 8, 16, 32, 64] {
                    let mut flow = base.clone();
                    add_cycle(&mut flow, &[0, 1, 2], a);
                    add_cycle(&mut flow, &reverse_ring, b);
                    add_cycle(&mut flow, &inner_ring, c);
                    let x = measure(&flow);
                    let bound = least_entropy(x.q, x.mean);
                    positive += usize::from(bound > 0.0);
                    assert!(
                        bound <= x.entropy + 1e-11,
                        "states={states}, cycles={a},{b},{c}, floor={bound}, entropy={}",
                        x.entropy
                    );
                }
            }
        }
    }
    assert_eq!(positive, 2289);
    println!("positive floors in the 2560-network scan: {positive}");
}

#[test]
fn witnesses_reach_both_sides_of_the_threshold_including_equality() {
    for (mean, d) in [
        (Ratio::int(2), Ratio::new(1, 8)),
        (Ratio::new(1, 2), Ratio::new(1, 4)),
        (ONE, ZERO),
    ] {
        let f = vec![
            vec![ZERO, ONE + d, ONE - d],
            vec![ONE - d, ZERO, mean + d],
            vec![ONE + d, mean - d, ZERO],
        ];
        let x = measure(&f);
        assert_eq!(x.mean, mean);
        assert_eq!(x.q, (ONE - d * d) / (ONE + mean));
        assert!((x.entropy - least_entropy(x.q, x.mean)).abs() < 1e-13);
    }
    // The two-hidden-state ceiling at q=1/2 is 2, attained by a path.
    let path = vec![
        vec![ZERO, ONE, ZERO],
        vec![ONE, ZERO, ONE],
        vec![ZERO, ONE, ZERO],
    ];
    let x = measure(&path);
    assert_eq!(x.q, Ratio::new(1, 2));
    assert_eq!((x.r, x.mean, x.entropy), (ONE - x.q, Ratio::int(2), 0.0));
    // One more hidden state makes mean 3 possible at the same q.
    let f = vec![
        vec![ZERO, Ratio::int(4), Ratio::int(3), Ratio::int(3)],
        vec![Ratio::int(4), ZERO, ZERO, ZERO],
        vec![Ratio::int(3), ZERO, ZERO, Ratio::int(15)],
        vec![Ratio::int(3), ZERO, Ratio::int(15), ZERO],
    ];
    let x = measure(&f);
    assert_eq!(
        (x.q, x.mean, x.entropy),
        (Ratio::new(1, 2), Ratio::int(3), 0.0)
    );
}

#[test]
fn splitting_hidden_states_preserves_every_projected_rate_and_adds_the_predicted_entropy() {
    let base = ring(5, 2, 1);
    let rows = visits(&base);
    let base_counts = measure(&base);
    let exits: Vec<_> = [2, 3, 5, 7].into_iter().map(Ratio::int).collect();
    let base_state = |i: usize| if i == 0 { 0 } else { 1 + (i - 1) / 3 };
    let copy = |i: usize, k: usize| 1 + 3 * (i - 1) + k;
    let three = Ratio::int(3);
    let size = 1 + 3 * (base.len() - 1);
    for p in [Ratio::new(1, 2), Ratio::new(3, 4), Ratio::new(99, 100)] {
        let mut lift = vec![vec![ZERO; size]; size];
        for i in 1..base.len() {
            for k in 0..3 {
                let from = copy(i, k);
                lift[0][from] = base[0][i] / three;
                lift[from][0] = base[i][0] / three;
                for j in 1..base.len() {
                    lift[from][copy(j, (k + 1) % 3)] = base[i][j] * p / three;
                    lift[from][copy(j, (k + 2) % 3)] = base[i][j] * (ONE - p) / three;
                }
            }
        }
        let lifted_rows = visits(&lift);
        for (i, row) in lift.iter().enumerate() {
            let ii = base_state(i);
            let multiplicity = if i == 0 { ONE } else { three };
            assert_eq!(
                lifted_rows[i] / exits[ii],
                rows[ii] / (multiplicity * exits[ii])
            );
            for (j, &base_traffic) in base[ii].iter().enumerate() {
                let rate = sum(row
                    .iter()
                    .enumerate()
                    .filter(|(l, _)| base_state(*l) == j)
                    .map(|(_, x)| *x))
                    * exits[ii]
                    / lifted_rows[i];
                assert_eq!(rate, base_traffic * exits[ii] / rows[ii]);
            }
        }
        let lifted = measure(&lift);
        assert_eq!(
            (lifted.q, lifted.r, lifted.mean),
            (base_counts.q, base_counts.r, base_counts.mean)
        );
        let cost =
            base_counts.mean.to_f64() * (2.0 * p.to_f64() - 1.0) * (p / (ONE - p)).to_f64().ln();
        assert!((lifted.entropy - base_counts.entropy - cost).abs() < 1e-12);
    }
}
