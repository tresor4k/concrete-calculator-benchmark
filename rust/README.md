# concrete-calculator-benchmark

Rust crate with the input cases and reference volumes of the Concrete Calculator Benchmark (v1, 2026-10-01 and
2026-10-02), published by TakeoffMetric Editorial ([takeoffmetric.com](https://takeoffmetric.com/)). No dependencies.

## Cases and reference
Six cases in US units (`data/cases.csv`): slabs of 10 x 10 ft x 4 in, 24 x 16 ft x 5 in, 12 x 12 ft x 6 in and
10 x 10 ft x 4.5 in, a strip footing of 20 ft x 16 in x 8 in, and a round column of 12 in diameter x 4 ft.

Each volume is computed with the TakeoffMetric engine and again with an independent closed-form formula in US units
(length x width x thickness; pi x r^2 x h). The computation stops if the two differ by more than 1e-9 cu ft. The
largest observed gap is 8.9e-16 cu ft (C05). The reference is the exact geometric volume, with no waste and no
rounding.

| file | content |
|---|---|
| `data/cases.csv` | the 6 input cases (4 slabs, 1 straight strip footing, 1 round column) |
| `data/reference.csv` | reference volume per case, in cu ft and cu yd |

The tool readings (`data/bench_*.csv`) and `data/tools.csv` are in the
[dataset repository](https://github.com/tresor4k/concrete-calculator-benchmark).

## Use

```rust
use concrete_calculator_benchmark::{cases, reference_volume_cuft, run};

let all = cases();                        // 6 cases, with reference_cuft and reference_cuyd
let v = reference_volume_cuft(&all[0]);   // closed-form volume, cu ft
let report = run(|c| reference_volume_cuft(c)); // replace with the volume function to check
assert!(report.passes(1e-9));
```

`cargo run --example check` prints the cases and their reference volumes.

## Links
- [Concrete calculator](https://takeoffmetric.com/concrete/concrete-calculator/)
- [Footing calculator](https://takeoffmetric.com/concrete/footing-calculator/)
- [Methodology](https://takeoffmetric.com/methodology/)
- [Corrections](https://takeoffmetric.com/corrections/)
- [Dataset repository](https://github.com/tresor4k/concrete-calculator-benchmark)

## Licence
- Code: MIT, see `LICENSE`.
- License of the data (`data/`): Creative Commons Attribution 4.0 International (CC BY 4.0), full text in
  `data/LICENSE`.
- Suggested citation: TakeoffMetric Editorial, Concrete calculator benchmark, 2026, takeoffmetric.com.
- Contact: contact@takeoffmetric.com.

## Updating the data
`python sync_data.py` copies `cases.csv`, `reference.csv` and the data licence from the dataset repository
(`../data`, or the folder given as first argument) into `data/`. It accepts only the published v1 headers. Then run
`cargo test`.
