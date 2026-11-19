//! Independent checks of docs/research/excursion-activity/RESULT.md.
//! Every matrix contains balanced stationary edge traffic in arbitrary common
//! units. Normalization by departures from 0 converts it to per-excursion units.

use physis_numeric::Ratio;

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

#[test]
fn all_four_state_ring_weights_through_eight_compare_the_two_sensors() {
    let mut classifications = [[0; 2]; 2];
    for cw in 1..=8 {
        for ccw in 1..=8 {
            for opposite in 1..=8 {
                let c = measure(&ring(cw, ccw, opposite));
                assert_eq!(c.mean, Ratio::int(2));
                let short = (ONE - c.q).pow(2) > c.r;
                let activity = (ONE + c.mean) * c.q < ONE;
                classifications[usize::from(short)][usize::from(activity)] += 1;
                assert!(least_entropy(c.q, c.mean) <= c.entropy + 1e-12);
            }
        }
    }
    assert_eq!(classifications, [[238, 20], [10, 244]]);
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
