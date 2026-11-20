//! Evaluate the exact equilibrium criterion on three canonical rational inputs.
use physis_numeric::Ratio;
use physis_theory::excursion::equilibrium::{equilibrium_fit, EquilibriumFit};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let [m, r, n] = args.as_slice() else {
        eprintln!("usage: excursion_equilibrium <P(N>=1)> <P(N>=2)> <mean N>");
        return ExitCode::FAILURE;
    };
    let [Some(m), Some(r), Some(n)] = [m, r, n].map(|x| Ratio::parse_display(x)) else {
        eprintln!("use integers or reduced fractions with a positive denominator");
        return ExitCode::FAILURE;
    };
    let fit = equilibrium_fit(m, r, n);
    println!("{}", fit.as_str());
    if fit == EquilibriumFit::InvalidCounts {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
