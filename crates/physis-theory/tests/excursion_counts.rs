//! Independent finite checks for docs/research/excursion-counts/RESULT.md.
use physis_numeric::Ratio;

const ONE: Ratio = Ratio::int(1);
const ZERO: Ratio = Ratio::int(0);

#[derive(Clone, Copy)]
struct Triangle {
    // Row i lists the two off-diagonal rates in increasing destination order.
    rates: [[Ratio; 2]; 3],
}

impl Triangle {
    fn exits(self) -> [Ratio; 3] {
        self.rates.map(|[a, b]| a + b)
    }

    // Directed spanning trees rooted at each state.
    fn stationary(self) -> [Ratio; 3] {
        let [[a, b], [c, u], [v, d]] = self.rates;
        let weights = [
            c * v + c * d + u * v,
            a * d + a * v + b * d,
            b * u + b * c + a * u,
        ];
        let sum = weights.into_iter().fold(ZERO, |s, w| s + w);
        weights.map(|w| w / sum)
    }

    // Return the tail probabilities from actual steps of the killed jump chain.
    fn count_tails(self) -> [Ratio; 7] {
        let [on, left, right] = self.exits();
        let [mut a, mut b] = self.rates[0].map(|rate| rate / on);
        std::array::from_fn(|_| {
            let mass = a + b;
            (a, b) = (b * self.rates[2][1] / right, a * self.rates[1][1] / left);
            mass
        })
    }

    fn zero_hop_weights(self) -> [Ratio; 2] {
        let [on, left, right] = self.exits();
        [
            self.rates[0][0] / on * self.rates[1][0] / left,
            self.rates[0][1] / on * self.rates[2][0] / right,
        ]
    }
}

fn generator(p: Ratio, x: Ratio, y: Ratio, exits: [Ratio; 3]) -> Triangle {
    let [on, left, right] = exits;
    Triangle {
        rates: [
            [on * p, on * (ONE - p)],
            [left * (ONE - x), left * x],
            [right * (ONE - y), right * y],
        ],
    }
}

// Returns no split when the sampled survival curve has a single component.
fn recovered_weight_product([h0, h1, h2, h3]: [Ratio; 4]) -> Option<Ratio> {
    let det = h0 * h2 - h1.pow(2);
    if det.is_zero() {
        return None;
    }
    let sum = (h0 * h3 - h1 * h2) / det;
    let product = (h1 * h3 - h2.pow(2)) / det;
    Some(det / (sum.pow(2) - Ratio::int(4) * product))
}

#[test]
fn reconstructed_current_and_affinity_match_stationary_fluxes() {
    // All 3^6 off-diagonal rate choices, including equal exits and equilibrium.
    for code in 0..3_i128.pow(6) {
        let mut digits = code;
        let rates = std::array::from_fn(|_| {
            std::array::from_fn(|_| {
                let rate = Ratio::int(1 + digits % 3);
                digits /= 3;
                rate
            })
        });
        let model = Triangle { rates };
        let [_, m, r, tail3, tail4, tail5, tail6] = model.count_tails();
        assert_eq!(
            [tail3, tail4, tail5, tail6],
            [m * r, r.pow(2), m * r.pow(2), r.pow(3)]
        );

        let [a, b] = model.zero_hop_weights();
        let [on, left, right] = model.exits();
        // At lag ln(2) the survival factors for the integer exit rates are rational.
        let z = |rate: Ratio| ONE / Ratio::int(2).pow(rate.num.try_into().unwrap());
        let samples =
            std::array::from_fn(|j| a * z(left).pow(j as u32) + b * z(right).pow(j as u32));
        let split = recovered_weight_product(samples);
        assert_eq!(split, (left != right).then_some(a * b));

        let s = m - r;
        let d2 = s.pow(2) - Ratio::int(4) * r * a * b;
        assert_eq!(d2 - (ONE - r) * (m.pow(2) - r), r * (a - b).pow(2));
        let d = d2.checked_sqrt().unwrap();
        let pi = model.stationary();
        let frequency = pi[0] * on;
        let edge_current = pi[0] * rates[0][0] - pi[1] * rates[1][0];
        assert_eq!((frequency * d / (ONE - r)).pow(2), edge_current.pow(2));
        let odds =
            rates[0][0] * rates[1][1] * rates[2][0] / (rates[0][1] * rates[2][1] * rates[1][0]);
        assert_eq!((s + d) / (s - d), odds.max(ONE / odds));

        // Rebuild both possible assignments of the two one-hop paths.
        for (f, rev) in [
            ((s + d) / Ratio::int(2), (s - d) / Ratio::int(2)),
            ((s - d) / Ratio::int(2), (s + d) / Ratio::int(2)),
        ] {
            let rebuilt = generator(
                (a + f) / (ONE - r),
                (f + r * a) / (a + f),
                (rev + r * b) / (b + rev),
                model.exits(),
            );
            assert_eq!(rebuilt.count_tails(), model.count_tails());
            assert_eq!(rebuilt.zero_hop_weights(), [a, b]);
        }
    }
}

#[test]
fn equal_exit_clock_preserves_an_equilibrium_and_a_driven_record() {
    let frac = Ratio::new;
    let exits = [ONE, Ratio::int(2), Ratio::int(2)];
    let equilibrium = generator(frac(1, 4), frac(1, 2), frac(1, 4), exits);
    let driven = generator(frac(1, 4), frac(3, 4), frac(1, 6), exits);
    assert_eq!(equilibrium.count_tails(), driven.count_tails());
    assert_eq!(&driven.count_tails()[1..3], &[frac(5, 16), frac(1, 8)]);
    for (model, current, odds) in [
        (equilibrium, ZERO, ONE),
        (driven, frac(4, 49), Ratio::int(5)),
    ] {
        let [[a, b], [c, u], [v, d]] = model.rates;
        let pi = model.stationary();
        assert_eq!(pi[0], frac(4, 7));
        assert_eq!(pi[0] * a - pi[1] * c, current);
        assert_eq!(a * u * v / (b * d * c), odds);
    }
}

#[test]
fn positive_entropy_floor_is_attained_and_geometric_boundary_allows_equilibrium() {
    let frac = Ratio::new;
    let exits = [ONE, Ratio::int(2), Ratio::int(2)];
    let minimizer = generator(frac(3, 5), frac(1, 2), frac(1, 4), exits);
    let [_, m, r, ..] = minimizer.count_tails();
    assert_eq!((m, r), (frac(2, 5), frac(1, 8)));
    let [a, b] = minimizer.zero_hop_weights();
    assert_eq!(a, b);
    let [[out, other], [back, hop], [end, reverse]] = minimizer.rates;
    let pi = minimizer.stationary();
    assert_eq!(pi[0] * out - pi[1] * back, frac(1, 9));
    assert_eq!(out * hop * end / (other * reverse * back), frac(9, 2));

    let geometric = generator(frac(1, 2), frac(1, 3), frac(1, 3), exits);
    let [_, m, r, ..] = geometric.count_tails();
    assert_eq!(m.pow(2), r);
    let pi = geometric.stationary();
    assert_eq!(pi[0] * geometric.rates[0][0], pi[1] * geometric.rates[1][0]);
}

// Advance the off-state mass without reintroducing paths that have returned to 0.
fn hidden_jump_tails<const STATES: usize, const STEPS: usize>(
    weights: &[[i128; STATES]; STATES],
) -> [Ratio; STEPS] {
    let exits = weights.map(|row| row.into_iter().sum::<i128>());
    let mut mass = weights[0].map(|f| Ratio::new(f, exits[0]));
    std::array::from_fn(|_| {
        mass = std::array::from_fn(|j| {
            if j == 0 {
                return ZERO;
            }
            (1..STATES)
                .map(|i| mass[i] * Ratio::new(weights[i][j], exits[i]))
                .fold(ZERO, |a, b| a + b)
        });
        mass.iter().copied().fold(ZERO, |a, b| a + b)
    })
}

#[test]
fn count_deficit_detects_circulation_with_three_hidden_states() {
    for circulation in [0, 2, 8, 32, 128] {
        let c = circulation;
        let flux = [
            [0, 4 + c, 1, 2],
            [4, 0, 3 + c, 1],
            [1 + c, 3, 0, 2],
            [2, 1, 2, 0],
        ];
        let [m, r] = hidden_jump_tails(&flux);
        assert_eq!(m.pow(2) > r, c >= 8);
        // Multiplying the reversible comparator's fluxes by two preserves its jump chain.
        let sym: [[i128; 4]; 4] =
            std::array::from_fn(|i| std::array::from_fn(|j| flux[i][j] + flux[j][i]));
        let [sym_m, sym_r] = hidden_jump_tails(&sym);
        assert!(sym_m.pow(2) <= sym_r);

        if m.pow(2) > r {
            let t = (m + r) / (ONE + m);
            let q = [ONE - m, m - r, r];
            let b = [ONE - t, t * (ONE - t), t * t];
            let divergence: f64 = q
                .into_iter()
                .zip(b)
                .map(|(p, v)| p.to_f64() * (p / v).to_f64().ln())
                .sum();
            let frequency: i128 = flux[0].iter().sum();
            let count_bound = 4.0 * frequency as f64 * divergence;
            let odds = Ratio::new((4 + c) * (3 + c) * (1 + c), 4 * 3);
            let true_entropy = c as f64 * odds.to_f64().ln();
            assert!(0.0 < count_bound && count_bound < true_entropy);
        }
    }
}

#[test]
fn five_states_undercut_every_triangle_matching_the_two_count_tails() {
    let frac = Ratio::new;
    // Each row is normalized separately. The last two states form a reversible branch.
    let weights = [
        [0, 3, 1, 198, 198],
        [1, 0, 3, 0, 0],
        [3, 1, 0, 0, 0],
        [1, 0, 0, 0, 9999],
        [1, 0, 0, 9999, 0],
    ];
    let [m, r, _, fourth] = hidden_jump_tails(&weights);
    assert_eq!(m, frac(996151, 1000000));
    assert_eq!(r, frac(9916770099, 10000000000));
    assert!(fourth > r.pow(2)); // The full count law exposes the extra hidden states.
    let triangle_floor_lower_bound = Ratio::int(2) * (m.pow(2) - r) / (m - r);
    assert!(triangle_floor_lower_bound > frac(7, 25));

    let forward = frac(3, 400) * frac(3, 4) * frac(3, 4);
    let reverse = frac(1, 400) * frac(1, 4) * frac(1, 4);
    let winding = (forward - reverse) / (ONE - frac(3, 16));
    assert_eq!(winding, frac(1, 200));
    assert_eq!(forward / reverse, Ratio::int(27));
    // log(27) < 4, so the true entropy per excursion is below 1/50.
    assert_eq!(Ratio::int(4) * winding, frac(1, 50));
}
