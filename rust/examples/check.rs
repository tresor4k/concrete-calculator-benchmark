//! Prints each case with its reference volume, then checks a volume function against them.
//!
//! cargo run --example check
use concrete_calculator_benchmark::{cases, reference_volume_cuft, run, CUFT_PER_CUYD};

fn main() {
    for c in cases() {
        let cuft = reference_volume_cuft(&c);
        println!("{}  {:<52} {:>14.9} cu ft {:>12.9} cu yd", c.id, c.input_summary, cuft, cuft / CUFT_PER_CUYD);
    }
    // Replace the closure with the volume function to check.
    let report = run(reference_volume_cuft);
    println!("max abs diff: {:e} cu ft, passes 1e-9: {}", report.max_abs_diff, report.passes(1e-9));
}
