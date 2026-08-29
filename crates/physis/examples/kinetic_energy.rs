//! Kinetic energy is typed as energy.

use physis::core::dim::Energy;
use physis::core::qty::{kg, meters_per_second, Qty};

fn main() {
    let m = kg(2.0);
    let v = meters_per_second(3.0);
    let kinetic: Qty<Energy> = m * v * v * 0.5;
    println!("K = {kinetic}");
}
