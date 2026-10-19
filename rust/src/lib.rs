//! The input cases and reference volumes of the concrete calculator benchmark published by
//! TakeoffMetric Editorial ([takeoffmetric.com](https://takeoffmetric.com/)), with a runner that
//! checks a volume function against them.
//!
//! Six cases in US units (`data/cases.csv`): slabs of 10 x 10 ft x 4 in, 24 x 16 ft x 5 in,
//! 12 x 12 ft x 6 in and 10 x 10 ft x 4.5 in, a strip footing of 20 ft x 16 in x 8 in, and a
//! round column of 12 in diameter x 4 ft.
//!
//! Each volume is computed with the TakeoffMetric engine and again with an independent
//! closed-form formula in US units (length x width x thickness; pi x r^2 x h). The computation
//! stops if the two differ by more than 1e-9 cu ft. The largest observed gap is 8.9e-16 cu ft
//! (C05). The reference is the exact geometric volume, with no waste and no rounding.
//!
//! The crate embeds `data/cases.csv` and `data/reference.csv`. [`cases`] returns the cases,
//! [`reference_volume_cuft`] is the closed-form formula, and [`run`] compares a volume function
//! with it. The tool readings (`data/bench_*.csv`) and `data/tools.csv` are in the
//! [dataset repository](https://github.com/tresor4k/concrete-calculator-benchmark).
//!
//! ```
//! use concrete_calculator_benchmark::{cases, run, Case, Shape, CUFT_PER_CUYD};
//!
//! fn my_volume_cuft(c: &Case) -> f64 {
//!     match c.shape {
//!         Shape::Slab => c.length_ft.unwrap() * c.width_ft.unwrap() * c.thickness_in.unwrap() / 12.0,
//!         Shape::StripFooting => {
//!             c.length_ft.unwrap() * (c.width_in.unwrap() / 12.0) * (c.thickness_in.unwrap() / 12.0)
//!         }
//!         Shape::RoundColumn => {
//!             let r_ft = c.diameter_in.unwrap() / 24.0;
//!             std::f64::consts::PI * r_ft * r_ft * c.height_ft.unwrap()
//!         }
//!     }
//! }
//!
//! assert_eq!(cases().len(), 6);
//! let report = run(my_volume_cuft);
//! assert!(report.passes(1e-9));
//! let c01 = &report.per_case[0];
//! assert_eq!(c01.id, "C01");
//! assert!((c01.expected_cuft / CUFT_PER_CUYD - 1.234567901).abs() < 1e-9);
//! ```
//!
//! Links: [concrete calculator](https://takeoffmetric.com/concrete/concrete-calculator/),
//! [footing calculator](https://takeoffmetric.com/concrete/footing-calculator/),
//! [methodology](https://takeoffmetric.com/methodology/),
//! [corrections](https://takeoffmetric.com/corrections/).
//!
//! Code: MIT. Data (`data/`): Creative Commons Attribution 4.0 International (CC BY 4.0), full
//! text in `data/LICENSE`. Suggested citation: TakeoffMetric Editorial, Concrete calculator
//! benchmark, 2026, takeoffmetric.com.
#![forbid(unsafe_code)]

const CASES_CSV: &str = include_str!("../data/cases.csv");
const REFERENCE_CSV: &str = include_str!("../data/reference.csv");

/// Cubic feet in one cubic yard.
pub const CUFT_PER_CUYD: f64 = 27.0;

/// Shape of a case, as in the `shape` column of `data/cases.csv`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// `slab`: `length_ft`, `width_ft`, `thickness_in`.
    Slab,
    /// `strip_footing`: `length_ft`, `width_in`, `thickness_in`.
    StripFooting,
    /// `round_column`: `diameter_in`, `height_ft`.
    RoundColumn,
}

/// One row of `data/cases.csv`, joined with its row of `data/reference.csv`.
///
/// A dimension that does not apply to the shape is `None`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Case {
    /// `case_id`, `C01` to `C06`.
    pub id: &'static str,
    /// `shape`.
    pub shape: Shape,
    /// `length_ft`.
    pub length_ft: Option<f64>,
    /// `width_ft`.
    pub width_ft: Option<f64>,
    /// `width_in`.
    pub width_in: Option<f64>,
    /// `thickness_in`.
    pub thickness_in: Option<f64>,
    /// `diameter_in`.
    pub diameter_in: Option<f64>,
    /// `height_ft`.
    pub height_ft: Option<f64>,
    /// `input_summary`, for example `slab 10 ft x 10 ft x 4 in`.
    pub input_summary: &'static str,
    /// `in_pilot`.
    pub in_pilot: bool,
    /// `reference_cuft` of `data/reference.csv` (9 decimals).
    pub reference_cuft: f64,
    /// `reference_cuyd` of `data/reference.csv` (9 decimals).
    pub reference_cuyd: f64,
}

fn num(field: &str) -> Option<f64> {
    if field.is_empty() {
        None
    } else {
        Some(field.parse().expect("embedded CSV: number"))
    }
}

/// The 6 cases, in file order.
pub fn cases() -> Vec<Case> {
    CASES_CSV
        .lines()
        .skip(1)
        .map(str::trim_end)
        .filter(|l| !l.is_empty())
        .map(|line| {
            let f: Vec<&'static str> = line.split(',').collect();
            assert_eq!(f.len(), 10, "embedded cases.csv: 10 columns");
            let shape = match f[1] {
                "slab" => Shape::Slab,
                "strip_footing" => Shape::StripFooting,
                "round_column" => Shape::RoundColumn,
                other => panic!("embedded cases.csv: unknown shape {other}"),
            };
            let r: Vec<&str> = REFERENCE_CSV
                .lines()
                .map(str::trim_end)
                .find(|l| l.split(',').next() == Some(f[0]))
                .expect("embedded reference.csv: row for every case")
                .split(',')
                .collect();
            assert_eq!(r.len(), 8, "embedded reference.csv: 8 columns");
            Case {
                id: f[0],
                shape,
                length_ft: num(f[2]),
                width_ft: num(f[3]),
                width_in: num(f[4]),
                thickness_in: num(f[5]),
                diameter_in: num(f[6]),
                height_ft: num(f[7]),
                input_summary: f[8],
                in_pilot: f[9] == "yes",
                reference_cuft: num(r[6]).expect("reference_cuft"),
                reference_cuyd: num(r[7]).expect("reference_cuyd"),
            }
        })
        .collect()
}

/// Closed-form volume in cu ft (length x width x thickness; pi x r^2 x h), with no waste and no
/// rounding.
///
/// # Panics
///
/// If a dimension required by the shape is `None` (never the case for the embedded cases).
pub fn reference_volume_cuft(case: &Case) -> f64 {
    let d = |v: Option<f64>| v.expect("dimension required by the shape");
    match case.shape {
        Shape::Slab => d(case.length_ft) * d(case.width_ft) * (d(case.thickness_in) / 12.0),
        Shape::StripFooting => {
            d(case.length_ft) * (d(case.width_in) / 12.0) * (d(case.thickness_in) / 12.0)
        }
        Shape::RoundColumn => {
            let r_ft = d(case.diameter_in) / 2.0 / 12.0;
            std::f64::consts::PI * r_ft * r_ft * d(case.height_ft)
        }
    }
}

/// Result of one case in a [`Report`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CaseResult {
    /// `case_id`.
    pub id: &'static str,
    /// [`reference_volume_cuft`] of the case.
    pub expected_cuft: f64,
    /// Value returned by the function under test.
    pub got_cuft: f64,
    /// `|got_cuft - expected_cuft|`; infinite when `got_cuft` is not finite.
    pub abs_diff_cuft: f64,
}

/// Output of [`run`].
#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    /// One result per case, in file order.
    pub per_case: Vec<CaseResult>,
    /// Largest `abs_diff_cuft`, in cu ft.
    pub max_abs_diff: f64,
}

impl Report {
    /// `true` when every case is within `tol_cuft` cu ft of the reference.
    pub fn passes(&self, tol_cuft: f64) -> bool {
        self.max_abs_diff <= tol_cuft
    }
}

/// Runs `volume_cuft` on every case and compares it with [`reference_volume_cuft`].
pub fn run(volume_cuft: impl Fn(&Case) -> f64) -> Report {
    let per_case: Vec<CaseResult> = cases()
        .iter()
        .map(|c| {
            let expected_cuft = reference_volume_cuft(c);
            let got_cuft = volume_cuft(c);
            let abs_diff_cuft = if got_cuft.is_finite() {
                (got_cuft - expected_cuft).abs()
            } else {
                f64::INFINITY
            };
            CaseResult { id: c.id, expected_cuft, got_cuft, abs_diff_cuft }
        })
        .collect();
    let max_abs_diff = per_case.iter().map(|r| r.abs_diff_cuft).fold(0.0, f64::max);
    Report { per_case, max_abs_diff }
}
