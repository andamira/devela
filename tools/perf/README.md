# Compilation performance

Tools for measuring and understanding devela's compilation costs.
Run from the repository root.

## Compile

`compile.sh` measures repeated compilation times using Hyperfine.

````sh
# Feature-less baseline
./tools/perf/compile.sh

# One additional feature
./tools/perf/compile.sh --features ratio

# Compare num and num_all against the baseline
./tools/perf/compile.sh --modules num --levels both

# Preview all root-module scenarios without compiling
./tools/perf/compile.sh --modules all --levels both --list
````

By default, the runner performs `cargo check` with default features disabled,
dependencies cached, one warmup and five measured runs. Results are saved
in a timestamped directory under `/tmp`.

Requires Bash, Hyperfine and the workspace `./x` launcher.

See `./tools/perf/compile.sh --help` for more options.

## Profile rustc

`profile.sh` records nightly rustc phase and query timings, and compares runs.
Profiling adds overhead; use `compile.sh` to measure ordinary compilation time.

````sh
./tools/perf/profile.sh --label baseline
./tools/perf/profile.sh --label num-all --features num_all
./tools/perf/profile.sh compare BASELINE_DIR EXPERIMENT_DIR
````

Requires Bash, Python 3.9+, nightly Rust, `./x` and Measureme `summarize`.
The script repairs a known invalid escape in affected profile data if needed.
See `./tools/perf/profile.sh --help` for options and output details.

## Inspect LLVM IR

`llvm-tree.py` groups generated LLVM IR by module
to reveal implementation breadth and function copies.

````sh
./x L --lib -p devela --no-default-features > /tmp/devela-llvm.txt

python3 tools/perf/llvm-tree.py /tmp/devela-llvm.txt --depth 2
python3 tools/perf/llvm-tree.py /tmp/devela-llvm.txt --focus num --depth 4
````

Requires `cargo-llvm-lines` to generate reports and Python 3.10+ to analyze them.

See `python3 tools/perf/llvm-tree.py --help` for comparison, sorting and export options.

## Comparing changes

Keep the same toolchain, features, build settings and clean policy.
Change one thing at a time and repeat the measurements.

Compilation time measures cost; LLVM IR size helps explain implementation breadth.
They are not interchangeable.
