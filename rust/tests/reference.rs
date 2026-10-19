use concrete_calculator_benchmark::{cases, reference_volume_cuft, run, Shape, CUFT_PER_CUYD};

const REFERENCE_CSV: &str = include_str!("../data/reference.csv");

#[test]
fn closed_form_matches_every_reference_column() {
    let cs = cases();
    assert_eq!(cs.len(), 6);
    let mut rows = 0;
    for line in REFERENCE_CSV.lines().skip(1).map(str::trim_end).filter(|l| !l.is_empty()) {
        let f: Vec<&str> = line.split(',').collect();
        let c = cs.iter().find(|c| c.id == f[0]).expect("case for reference row");
        let v = reference_volume_cuft(c);
        // engine_cuft, independent_cuft, reference_cuft
        for col in [3, 4, 6] {
            let r: f64 = f[col].parse().unwrap();
            assert!((v - r).abs() < 1e-9, "{} column {col}: {v} vs {r}", c.id);
        }
        let cuyd: f64 = f[7].parse().unwrap();
        assert!((v / CUFT_PER_CUYD - cuyd).abs() < 1e-9, "{} cu yd", c.id);
        assert!((c.reference_cuft - v).abs() < 1e-9);
        assert!((c.reference_cuyd - cuyd).abs() == 0.0);
        rows += 1;
    }
    assert_eq!(rows, 6);
}

#[test]
fn shapes_in_file_order() {
    let shapes: Vec<Shape> = cases().iter().map(|c| c.shape).collect();
    use Shape::*;
    assert_eq!(shapes, [Slab, Slab, Slab, StripFooting, RoundColumn, Slab]);
}

#[test]
fn run_reports_gaps() {
    assert!(run(reference_volume_cuft).passes(0.0));
    let with_waste = run(|c| reference_volume_cuft(c) * 1.1);
    assert!(!with_waste.passes(1e-9));
    assert_eq!(with_waste.per_case.len(), 6);
    let nan = run(|_| f64::NAN);
    assert!(nan.max_abs_diff.is_infinite());
    assert!(!nan.passes(1e9));
}
