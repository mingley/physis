//! Evaluate the sharp no-one-jump minimum or rational lower bounds from two or three observations.
use physis_numeric::Ratio;
use physis_theory::excursion::entropy::{
    one_jump_absent_minimum, support_plane_entropy, two_tail_entropy,
};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let Some((mode, inputs)) = args.split_first() else {
        return usage();
    };
    let Some(inputs) = inputs
        .iter()
        .map(|s| Ratio::parse_display(s))
        .collect::<Option<Vec<_>>>()
    else {
        return usage();
    };
    match (mode.as_str(), inputs.as_slice()) {
        ("no-one-jump", &[m, n]) => {
            let Some(minimum) = one_jump_absent_minimum(m, n) else {
                return usage();
            };
            println!("minimum k_B/excursion ~ {minimum:.12e}");
        }
        ("two-tail", &[m, r]) => {
            let Some(bound) = two_tail_entropy(m, r) else {
                return usage();
            };
            println!("lower bound k_B/excursion >= {bound}");
        }
        ("plane", &[m, r, n, a]) => {
            let Some(bound) = support_plane_entropy(m, r, n, a) else {
                return usage();
            };
            println!("lower bound k_B/excursion >= {bound}");
        }
        _ => return usage(),
    }
    ExitCode::SUCCESS
}

fn usage() -> ExitCode {
    eprintln!("use no-one-jump <m> <n>, two-tail <m> <r>, or plane <m> <r> <n> <a> with valid rational inputs");
    ExitCode::FAILURE
}
