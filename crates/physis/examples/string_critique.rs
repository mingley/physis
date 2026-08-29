//! Print the string-critique matrix.

fn main() {
    let report = physis::theory::string_critique();
    print!("{}", report.render());
}
