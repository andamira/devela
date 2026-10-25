#!/usr/bin/env bash
# devela compiler diagnostics (rustc self-profile)
#
# Purpose: locate time spent in compiler phases and queries during check/build;
#          compare saved reports without rebuilding.
# Requires: Bash, Python 3.9+, nightly Rust, workspace ./x, measureme `summarize`.
# Use:     ./tools/perf/profile.sh [--label NAME] [--features CSV] [-v]
#          ./tools/perf/profile.sh compare BASELINE_DIR EXPERIMENT_DIR
# Defaults: check devela without default features; 3 independently cleaned runs.
# Output:  uniquely named /tmp/devela-profile-* directory (or --out DIR), with
#          phase, query and summary reports. See --help.
# Note:    instrumentation adds overhead; use compile.sh for wall-time benchmarks.
#          Applies an in-place, byte-length-preserving escape repair to profiler
#          data if affected. Use --keep-original to retain an unmodified copy.

set -euo pipefail

# Cargo calls RUSTC_WRAPPER with the rustc executable as argv[1]. Re-entering
# this same, fixed-path script avoids changing Cargo fingerprints every run.
if [[ ${DEVELA_PROFILE_WRAPPER:-0} == 1 ]]; then
    rustc=$1
    shift
    matched=0
    previous=''
    for arg in "$@"; do
        if [[ $previous == --crate-name && $arg == devela ]]; then matched=1; fi
        if [[ $arg == --crate-name=devela ]]; then matched=1; fi
        previous=$arg
    done
    if ((matched)); then
        exec "$rustc" "$@" -Ztime-passes "-Zself-profile=${DEVELA_PROFILE_DIR:?}"
    else
        exec "$rustc" "$@"
    fi
fi

usage() {
    cat <<'HELP'
Usage: ./tools/perf/profile.sh [options]
       ./tools/perf/profile.sh compare [--queries CSV] [--top N] BASELINE_DIR EXPERIMENT_DIR [...]

  --label NAME              Human-readable experiment label (default: snapshot)
  --features CSV            Features to enable, e.g. num_all,std (default: none)
  --kind check|build        Profile actual Cargo operation (default: check)
  --runs N                  Number of independently cleaned runs (default: 3)
  --clean package|workspace|none   Clean policy before each run (default: package)
  --toolchain +NAME         Rust toolchain (default: +nightly)
  --default-features        Enable Cargo defaults (off by default)
  --native                  Pass --native to the project ./x launcher
  --target TRIPLE           Optional compilation target
  --cfg LIST                Pass --cfg LIST to ./x, e.g. 'foo,bar'
  --out DIR                 Output directory (default: /tmp/devela-profile-...)
  --keep-original           Preserve original .mm_profdata if an escape repair is needed
  --top N                   Number of self-profile summary rows to show (default: 15)
  -v, --verbose             Show diagnostic commands, compiler output and report headers

Compare options:
  --queries CSV             Self-profile query names to show (default: lint/macro hotspots)
  --top N                   Show the N largest changes in query self time (default: 10)
  -h, --help                Show this help

Examples:
  ./tools/perf/profile.sh --label baseline --runs 3
  ./tools/perf/profile.sh --label num-all --features num_all --runs 3
  ./tools/perf/profile.sh compare /tmp/profile-baseline /tmp/profile-no-tags
  ./tools/perf/profile.sh compare --queries early_lint_checks,drop_ast --top 15 \
    /tmp/profile-baseline /tmp/profile-no-tags

Outputs per run: rustc.log, devela-*.mm_profdata, summarize.txt,
                   phases.json, top.txt, patch-info.txt.
Outputs per battery: results.csv, metadata.txt, comparison.txt.
Compare reads each run's saved summarize.txt; it does not rebuild anything.

The known measureme JSON escape bug is repaired IN PLACE on generated profiles
by the same-length substitution \\u{b7} -> \\u00b7. This preserves byte offsets.
Use --keep-original to preserve an affected file before modifying it.

Unlike Hyperfine, this is a DIAGNOSTIC profiler: its timings include profiling
instrumentation overhead. With --clean none, Cargo may skip the build entirely.
Compare reports made with identical settings, and use
compile.sh/Hyperfine for the independent user-facing compilation-time baseline.
HELP
}

die() { printf 'error: %s\n' "$*" >&2; exit 2; }

# Compare previously collected batteries without recompiling.
# The first directory is the reference; differences are experimental, not
# necessarily causal unless toolchain, flags, and cache policy are matched.
if [[ ${1:-} == compare ]]; then
    shift
    compare_queries='early_lint_checks,shallow_lint_levels_on,expand_proc_macro,drop_ast,expand_crate,lower_to_hir'
    compare_top=10
    while (($#)); do
        case "$1" in
            --queries|--top)
                (($# >= 2)) || die "$1 requires a value"
                key=$1; val=$2; shift 2
                if [[ $key == --queries ]]; then compare_queries=$val; else compare_top=$val; fi;;
            --) shift; break;;
            -*) die "unknown compare option: $1";;
            *) break;;
        esac
    done
    (($# >= 2)) || die 'compare requires at least two result directories'
    [[ $compare_queries =~ ^[A-Za-z0-9_,]*$ ]] || die 'invalid --queries list'
    [[ $compare_top =~ ^[0-9]+$ ]] || die '--top must be nonnegative'
    python3 - "$compare_queries" "$compare_top" "$@" <<'PY_COMPARE'
from pathlib import Path
import csv
import re
import statistics
import sys

focused = [q for q in sys.argv[1].split(',') if q]
top = int(sys.argv[2])
paths = [Path(p) for p in sys.argv[3:]]
metrics = ['wall_s', 'rustc_total_s', 'expand_crate', 'type_check_crate',
           'MIR_borrow_checking', 'generate_crate_metadata', 'self_cpu_s']
units_ms = {'ns': 1e-6, 'µs': 1e-3, 'us': 1e-3, 'ms': 1.0, 's': 1e3}

def duration_ms(value):
    match = re.fullmatch(r'([0-9.]+)\s*(ns|µs|us|ms|s)', value)
    if not match:
        raise ValueError(f'invalid measureme duration: {value!r}')
    return float(match.group(1)) * units_ms[match.group(2)]

def summarize_queries(path):
    result = {}
    columns = None
    for line in path.read_text(errors='replace').splitlines():
        if not line.startswith('|'):
            continue
        cells = [cell.strip() for cell in line.strip('|').split('|')]
        if cells[0] == 'Item':
            if 'Self time' in cells and 'Item count' in cells:
                columns = cells.index('Self time'), cells.index('Item count')
            elif columns is not None:
                break  # Later tables (e.g. artifact sizes) are not query timings.
            continue
        if columns is None or len(cells) <= max(columns):
            continue
        name = re.sub(r'\s*\.{2,}\s*$', '', cells[0]).strip()
        if name:
            try:
                result[name] = (duration_ms(cells[columns[0]]),
                                int(cells[columns[1]].replace(',', '')))
            except ValueError:
                continue
    if columns is None:
        raise ValueError(f'{path}: missing query table header (expected Self time and Item count)')
    return result

def stats(numbers):
    if not numbers:
        return None, None
    return statistics.mean(numbers), statistics.stdev(numbers) if len(numbers) > 1 else 0.0

def meta(path):
    data = {}
    if (path / 'metadata.txt').is_file():
        for line in (path / 'metadata.txt').read_text().splitlines():
            if '=' in line:
                key, value = line.split('=', 1)
                data[key] = value
    return data

values = []
for path in paths:
    with (path / 'results.csv').open(newline='') as f:
        rows = list(csv.DictReader(f))
    if not rows:
        raise SystemExit(f'No results in {path}')
    runs = sorted(path.glob('run-*/summarize.txt'))
    if len(runs) != len(rows):
        raise SystemExit(f'{path}: found {len(runs)} self-profile summaries for {len(rows)} runs')
    record = {'name': rows[0]['label'], 'path': path, 'n': len(rows),
              'queries': [summarize_queries(p) for p in runs], 'meta': meta(path)}
    for key in metrics:
        numbers = [float(r[key]) for r in rows if r.get(key)]
        record[key] = statistics.mean(numbers) if numbers else None
    values.append(record)

ref = values[0]
print('Reference:', ref['name'], ref['path'])
print('CAUTION: profile self times include instrumentation; compare matching configurations.')
for other in values[1:]:
    check = ('kind', 'features', 'clean', 'toolchain', 'no_default', 'native',
             'target', 'cfg', 'rustc_version')
    mismatch = [k for k in check if k in ref['meta'] and k in other['meta']
                and ref['meta'][k] != other['meta'][k]]
    if mismatch:
        print(f"WARNING: {other['name']} differs in: {', '.join(mismatch)}")
print()
print(f"{'Experiment':<24}{'Runs':>5}  {'Wall s':>10}  {'Rustc s':>10}  {'Expand s':>10}  {'Delta rustc':>11}  {'Delta expand':>12}")
for record in values:
    def fmt(key):
        n = record[key]
        return f'{n:.3f}' if n is not None else 'n/a'
    def delta(key):
        n, base = record[key], ref[key]
        return f'{(n / base - 1) * 100:+.1f}%' if n is not None and base else 'n/a'
    print(f"{record['name']:<24}{record['n']:>5}  {fmt('wall_s'):>10}  "
          f"{fmt('rustc_total_s'):>10}  {fmt('expand_crate'):>10}  "
          f"{delta('rustc_total_s'):>11}  {delta('expand_crate'):>12}")

# Each summarize.txt is one run. Compare query self times, not inclusive times,
# so nested queries do not double-count. Missing queries count as zero invocations.
def query_stats(record, query):
    times = [run.get(query, (0.0, 0))[0] for run in record['queries']]
    counts = [run.get(query, (0.0, 0))[1] for run in record['queries']]
    mean, sd = stats(times)
    return mean, sd, statistics.mean(counts)

for record in values[1:]:
    print(f"\nSelf-profile queries: {ref['name']} -> {record['name']} (ms, mean ± SD)")
    print(f"{'Query':<40} {'Reference':>16} {'Experiment':>16} {'Delta':>11} {'Calls (ref->exp)':>20}")
    all_names = set().union(*(run.keys() for run in ref['queries'] + record['queries']))
    def change(query):
        return query_stats(record, query)[0] - query_stats(ref, query)[0]
    ranked = sorted(all_names, key=lambda name: abs(change(name)), reverse=True)
    shown = list(dict.fromkeys(focused + (ranked[:top] if top else [])))
    for query in shown:
        if query not in all_names:
            print(f'{query:<40} (not found in either profile)')
            continue
        base, base_sd, base_n = query_stats(ref, query)
        exp, exp_sd, exp_n = query_stats(record, query)
        print(f'{query:<40} {base:>6.2f} ± {base_sd:<6.2f} '
              f'{exp:>6.2f} ± {exp_sd:<6.2f} {exp-base:>+10.2f} '
              f'{base_n:>8.0f} -> {exp_n:<8.0f}')
    if top:
        print(f'  Focused queries followed by up to {top} largest absolute changes (deduplicated).')
PY_COMPARE
    exit
fi

label='snapshot'
features=''
kind='check'
runs=3
clean='package'
toolchain='+nightly'
no_default=1
native=0
target=''
cfg=''
out=''
keep_original=0
top=15
verbose=0

while (($#)); do
    case "$1" in
        --label|--features|--kind|--runs|--clean|--toolchain|--target|--cfg|--out|--top)
            (($# >= 2)) || die "$1 requires a value"
            key=$1; val=$2; shift 2
            case $key in
                --label) label=$val;;
                --features) features=$val;;
                --kind) kind=$val;;
                --runs) runs=$val;;
                --clean) clean=$val;;
                --toolchain) toolchain=$val;;
                --target) target=$val;;
                --cfg) cfg=$val;;
                --out) out=$val;;
                --top) top=$val;;
            esac;;
        --default-features) no_default=0; shift;;
        --native) native=1; shift;;
        --keep-original) keep_original=1; shift;;
        -v|--verbose) verbose=1; shift;;
        -h|--help) usage; exit 0;;
        *) die "unknown option: $1";;
    esac
done

[[ $label =~ ^[A-Za-z0-9_.-]+$ ]] || die 'label may contain only letters, digits, _, -, .'
[[ -z $features || $features =~ ^[A-Za-z0-9_]+(,[A-Za-z0-9_]+)*$ ]] || die 'features must be a comma-separated list'
[[ $kind == check || $kind == build ]] || die 'kind must be check or build'
[[ $clean == package || $clean == workspace || $clean == none ]] || die 'invalid clean policy'
[[ $runs =~ ^[1-9][0-9]*$ ]] || die 'runs must be positive'
[[ $clean != none || $runs == 1 ]] || die '--clean none only supports one run (Cargo may reuse compiled output)'
[[ $top =~ ^[1-9][0-9]*$ ]] || die 'top must be positive'
[[ $toolchain =~ ^\+[A-Za-z0-9_.-]+$ ]] || die 'toolchain must be +NAME'
[[ -z $target || $target =~ ^[A-Za-z0-9_.-]+$ ]] || die 'invalid target triple'
[[ -z $cfg || $cfg =~ ^[A-Za-z0-9_,=-]+$ ]] || die 'invalid --cfg list'
[[ -z ${RUSTC_WRAPPER:-} ]] || die 'RUSTC_WRAPPER already set: cannot transparently chain with another wrapper'
[[ -z ${RUSTC_WORKSPACE_WRAPPER:-} ]] || die 'RUSTC_WORKSPACE_WRAPPER already set: unset it for this test'
command -v python3 >/dev/null || die 'python3 not found'
command -v summarize >/dev/null || die 'measureme summarize not found in PATH'

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
script_path="$script_dir/$(basename -- "${BASH_SOURCE[0]}")"
workspace=$(cd -- "$script_dir/../.." && pwd)
cd "$workspace"
[[ -x ./x ]] || die "expected executable ./x in workspace root: $workspace"

if [[ -z $out ]]; then
    out=$(mktemp -d "/tmp/devela-profile-${label}-XXXXXXXX")
else
    [[ ! -e $out ]] || die "output path already exists: $out"
    mkdir -p -- "$out"
fi
out=$(cd -- "$out" && pwd)

x_base=(./x "$toolchain")
((native == 0)) || x_base+=(--native)
[[ -z $cfg ]] || x_base+=(--cfg "$cfg")

cargo_args=(-p devela --lib --locked)
((no_default == 0)) || cargo_args+=(--no-default-features)
[[ -z $features ]] || cargo_args+=(-F "$features")
[[ -z $target ]] || cargo_args+=(--target "$target")

if [[ $kind == check ]]; then operation=c; else operation=b; fi
cmd=("${x_base[@]}" "$operation" "${cargo_args[@]}")

{
    printf 'label=%s\nkind=%s\nfeatures=%s\nruns=%s\nclean=%s\ntoolchain=%s\nnative=%s\n' \
        "$label" "$kind" "${features:--}" "$runs" "$clean" "$toolchain" "$native"
    printf 'no_default=%s\ntarget=%s\ncfg=%s\n' "$no_default" "${target:--}" "${cfg:--}"
    printf 'git_commit=%s\n' "$(git rev-parse HEAD 2>/dev/null || echo unknown)"
    printf 'git_dirty=%s\n' "$(if [[ -n $(git status --porcelain 2>/dev/null) ]]; then echo yes; else echo no; fi)"
    printf 'rustc_version='; rustc "$toolchain" --version
    printf 'command='; printf '%q ' "${cmd[@]}"; printf '\n'
} > "$out/metadata.txt"

printf 'Saving profiling battery to: %s\n' "$out"
printf 'Command: '; printf '%q ' "${cmd[@]}"; printf '\n'
if ((verbose)); then
    printf 'Compiler: '; rustc "$toolchain" --version
    printf 'Summarize: %s\n' "$(command -v summarize)"
fi

for ((i=1; i<=runs; i++)); do
    printf -v num '%02d' "$i"
    run_dir="$out/run-$num"
    mkdir -p -- "$run_dir"
    printf '\n[%d/%d] cleaning (%s)...\n' "$i" "$runs" "$clean"
    case $clean in
        package) "${x_base[@]}" clean -p devela > "$run_dir/clean.log" 2>&1 || {
            cat "$run_dir/clean.log" >&2; die 'package clean failed';
        };;
        workspace) "${x_base[@]}" clean > "$run_dir/clean.log" 2>&1 || {
            cat "$run_dir/clean.log" >&2; die 'workspace clean failed';
        };;
        none) : > "$run_dir/clean.log";;
    esac
    if ((verbose)); then
        printf '  Clean log: %s\n' "$run_dir/clean.log"
        cat "$run_dir/clean.log"
    fi

    printf '[%d/%d] profiling devela...\n' "$i" "$runs"
    if ((verbose)); then
        printf '  Compiler log: %s (shown after the timed run)\n' "$run_dir/rustc.log"
    fi
    start_ns=$(date +%s%N)
    if DEVELA_PROFILE_WRAPPER=1 DEVELA_PROFILE_DIR="$run_dir" \
        RUSTC_WRAPPER="$script_path" "${cmd[@]}" > "$run_dir/rustc.log" 2>&1; then
        :
    else
        rc=$?
        tail -n 60 "$run_dir/rustc.log" >&2
        die "compilation failed (exit $rc); see $run_dir/rustc.log"
    fi
    end_ns=$(date +%s%N)
    printf '%s\n' "$((end_ns-start_ns))" > "$run_dir/wall-ns.txt"
    if ((verbose)); then
        cat "$run_dir/rustc.log"
    fi

    # Repair only the known six-byte invalid JSON escape. mmap performs the
    # same-length in-place substitution without loading large files into RAM.
    python3 - "$run_dir" "$keep_original" <<'PY'
import mmap
import pathlib
import shutil
import sys

folder = pathlib.Path(sys.argv[1])
keep = sys.argv[2] == '1'
old = br'\u{b7}'
new = br'\u00b7'
assert len(old) == len(new) == 6
profiles = sorted(folder.glob('devela-*.mm_profdata'))
if not profiles:
    raise SystemExit(f'No devela self-profile found in {folder}; inspect rustc.log')
with (folder / 'patch-info.txt').open('w') as report:
    for path in profiles:
        n = 0
        with path.open('r+b') as file:
            if path.stat().st_size == 0:
                raise SystemExit(f'Empty profiler data: {path}')
            with mmap.mmap(file.fileno(), 0) as view:
                pos = view.find(old)
                if pos >= 0 and keep:
                    shutil.copy2(path, path.with_suffix('.mm_profdata.orig'))
                while pos >= 0:
                    view[pos:pos + len(old)] = new
                    n += 1
                    pos = view.find(old, pos + len(new))
                if n:
                    view.flush()
        print(f'{path.name}: {n} fixed escape(s)', file=report)
PY
    if ((verbose)); then
        printf '  Escape repair: '; cat "$run_dir/patch-info.txt"
    fi

    profiles=("$run_dir"/devela-*.mm_profdata)
    [[ -f ${profiles[0]} ]] || die "no devela self-profile generated in $run_dir"
    ((${#profiles[@]} == 1)) || die "expected one devela profile, found ${#profiles[@]} in $run_dir"
    if ((verbose)); then
        printf '  Summarize command: summarize summarize %q\n' "${profiles[0]}"
    fi
    summarize summarize "${profiles[0]}" > "$run_dir/summarize.txt" 2> "$run_dir/summarize.err" || {
        cat "$run_dir/summarize.err" >&2
        die "summarize failed; inspect $run_dir"
    }
    if ((verbose)); then
        printf '  Report: %s\n' "$run_dir/summarize.txt"
        grep -E '^\|[[:space:]]*Item[[:space:]]*\|' "$run_dir/summarize.txt" || true
    fi

    python3 - "$run_dir" "$top" <<'PY'
from pathlib import Path
import json
import re
import sys

folder = Path(sys.argv[1])
top = int(sys.argv[2])
log = (folder / 'rustc.log').read_text(errors='replace')
summary = (folder / 'summarize.txt').read_text(errors='replace')
phase_names = ['total', 'expand_crate', 'macro_expand_crate', 'type_check_crate',
               'MIR_borrow_checking', 'coherence_checking', 'generate_crate_metadata',
               'codegen_crate', 'LLVM_passes']
phases = {}
for line in log.splitlines():
    match = re.search(r'^\s*time:\s*([0-9.]+);.*?\s([A-Za-z_][A-Za-z_0-9]*)\s*$', line)
    if match and match.group(2) in phase_names:
        phases[match.group(2)] = float(match.group(1))

cpu = re.search(r'Total cpu time:\s*([\d.]+)(ns|µs|ms|s)', summary)
unit = {'ns': 1e-9, 'µs': 1e-6, 'ms': 1e-3, 's': 1}
cpu_s = float(cpu.group(1)) * unit[cpu.group(2)] if cpu else None
query_counts = {}
query_column = None
for line in summary.splitlines():
    if not line.startswith('|'):
        continue
    cells = [cell.strip() for cell in line.strip('|').split('|')]
    if cells[0] == 'Item':
        if 'Self time' in cells and 'Item count' in cells:
            query_column = cells.index('Item count')
        elif query_column is not None:
            break  # Ignore the later artifact-size table.
        continue
    if query_column is None or len(cells) <= query_column:
        continue
    name = re.sub(r'\s*\.{2,}\s*$', '', cells[0]).strip()
    if name in ('typeck_root', 'mir_borrowck', 'expand_proc_macro', 'mir_built'):
        query_counts[name] = int(cells[query_column].replace(',', ''))
if query_column is None:
    raise SystemExit(f'{folder / "summarize.txt"}: missing query table header '
                     '(expected Self time and Item count)')
wall = int((folder / 'wall-ns.txt').read_text().strip()) / 1e9
result = {'run': folder.name, 'wall_s': wall, 'rustc_total_s': phases.get('total'),
          'self_cpu_s': cpu_s, 'phases_s': phases, 'query_counts': query_counts}
(folder / 'phases.json').write_text(json.dumps(result, indent=2) + '\n')
(folder / 'top.txt').write_text('\n'.join(summary.splitlines()[:top + 4]) + '\n')
print('  wall={:.3f}s rustc={}s expand={}s self_cpu={}s'.format(
    wall, phases.get('total', 'n/a'), phases.get('expand_crate', 'n/a'),
    f'{cpu_s:.3f}' if cpu_s is not None else 'n/a'))
PY

done

python3 - "$out" "$label" "$kind" "$features" <<'PY'
from pathlib import Path
import csv
import json
import statistics
import sys

out = Path(sys.argv[1])
label, kind, features = sys.argv[2:]
data = [json.loads(f.read_text()) for f in sorted(out.glob('run-*/phases.json'))]
keys = ['wall_s', 'rustc_total_s', 'self_cpu_s', 'expand_crate',
        'type_check_crate', 'MIR_borrow_checking', 'generate_crate_metadata',
        'expand_proc_macro_count', 'typeck_root_count', 'mir_borrowck_count']
records = []
for d in data:
    row = {'label': label, 'kind': kind, 'features': features or '-', 'run': d['run'],
           'wall_s': d['wall_s'], 'rustc_total_s': d['rustc_total_s'],
           'self_cpu_s': d['self_cpu_s']}
    for name in keys[3:7]:
        row[name] = d['phases_s'].get(name)
    for name in keys[7:]:
        row[name] = d['query_counts'].get(name.removesuffix('_count'))
    records.append(row)
with (out / 'results.csv').open('w', newline='') as file:
    writer = csv.DictWriter(file, fieldnames=list(records[0]))
    writer.writeheader()
    writer.writerows(records)
lines = [f'Experiment: {label} ({kind}, features={features or "none"})',
         f'Runs: {len(records)}', '']
for key in keys:
    values = [row[key] for row in records if row.get(key) is not None]
    if not values:
        continue
    mean = statistics.mean(values)
    sd = statistics.stdev(values) if len(values) > 1 else 0
    unit = 'count' if key.endswith('_count') else 's'
    lines.append(f'{key:<28} {mean:>9.3f} ± {sd:>7.3f} {unit}')
text = '\n'.join(lines) + '\n'
(out / 'comparison.txt').write_text(text)
print('\n' + text)
print(f'Reports saved in: {out}')
PY
