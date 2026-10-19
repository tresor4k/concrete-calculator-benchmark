"""Copy the benchmark cases, reference volumes and data licence into data/ (embedded by the crate).

Usage: python sync_data.py [bench_data_dir]
Default source: ../data (the data/ folder of the concrete-calculator-benchmark repository)
Only the published v1 headers are accepted. Then run: cargo test
"""
import csv
import pathlib
import shutil
import sys

HERE = pathlib.Path(__file__).resolve().parent
SRC = pathlib.Path(sys.argv[1]) if len(sys.argv) > 1 else HERE.parent / "data"
HEADERS = {
    "cases.csv": "case_id,shape,length_ft,width_ft,width_in,thickness_in,diameter_in,height_ft,input_summary,in_pilot",
    "reference.csv": "case_id,shape,input_summary,engine_cuft,independent_cuft,abs_diff_cuft,reference_cuft,reference_cuyd",
}
dst_dir = HERE / "data"
dst_dir.mkdir(exist_ok=True)
for name, header in HEADERS.items():
    src = SRC / name
    with open(src, encoding="utf-8", newline="") as f:
        rows = list(csv.reader(f))
    if ",".join(rows[0]) != header:
        sys.exit(f"unexpected header in {src}: {rows[0]}")
    shutil.copyfile(src, dst_dir / name)
    print(f"synced {len(rows) - 1} rows from {src} to {dst_dir / name}")
licence = SRC.parent / "LICENSE"
shutil.copyfile(licence, dst_dir / "LICENSE")
print(f"copied {licence} to {dst_dir / 'LICENSE'}")
