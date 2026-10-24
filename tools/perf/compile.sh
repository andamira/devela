#!/usr/bin/env bash
# devela compilation benchmarks (Hyperfine)
#
# Purpose: measure clean Cargo check/build/doc times across selected features.
# Requires: Bash, Rust/Cargo, the workspace ./x launcher, and Hyperfine to run.
# Use:     ./tools/perf/compile.sh [--features ratio | --modules num --levels both]
#          ./tools/perf/compile.sh --list       (preview; no Hyperfine needed)
# Defaults: check devela without default features; clean package before each
#           run (dependencies cached); 1 warmup and 5 measured runs.
# Output:  timestamped /tmp/devela-compile-* directory, or --out DIR;
#          results.{json,csv}, scenarios.tsv, metadata.txt.
# Note:    Hyperfine times exclude the clean preparation step. See --help.

set -euo pipefail

usage() {
    cat <<'HELP'
Usage: ./tools/perf/compile.sh [OPTIONS]

Quick start:
  ./tools/perf/compile.sh
  ./tools/perf/compile.sh --features ratio
  ./tools/perf/compile.sh --modules num --levels both
  ./tools/perf/compile.sh --modules all --levels both --list

Common options:
  --features CSV        Additional Cargo features for every scenario
  --modules LIST        Root modules (comma-separated), all, or none (default: none)
  --levels base|all|both  Test root features, *_all, or both (default: base)
  --runs N              Measured runs per scenario (default: 5)
  --warmup N            Warmups per scenario (default: 1)
  --out DIR             Results directory (default: timestamped under /tmp)
  --list                Preview the scenarios and total planned runs; do not build

Scenario options:
  --combined            Also benchmark the union of the selected root modules
  --no-baseline         Omit the reference scenario (included by default)
  --env base|alloc|std|all  Environment feature variations (default: base)
  --unsafe off|on|both  Add the aggregate unsafe feature (default: off)
  --sys-leaves CSV      Separate sys probes: linux,term,hw,x11,alsa,web,android
  --docs none|min|nodep|full|max|all  Documentation scopes (default: none)

Build options:
  --kind check|build|doc  Cargo operation (default: check; doc for docs-only runs)
  --clean auto|package|docs|workspace|none  Pre-run clean (default: auto)
  --default-features    Enable Cargo default features (disabled by default)
  --toolchain +NAME     Rust toolchain (default: ./x selection; nightly for doc)
  --target TRIPLE       Target triple (default: host)
  --native              Pass --native to ./x
  -h, --help            Show this help

Notes:
  * --features applies to all scenarios; --modules none --features CSV tests
    that one feature set against no additional module selections.
  * --modules all tests 13 root modules separately; --combined adds their union.
    --levels all uses *_all; the all-root combined case uses Cargo feature all.
  * sys leaves are separate probes, not implied by sys_all or all.
  * --unsafe off does not forbid unsafe code: other features may enable it.
  * docs min/nodep/full/max correspond to _docs_min/_docs_nodep/_docs/_docs_max.
  * package clean rebuilds devela with dependencies cached; workspace clean
    rebuilds dependencies too. No clean mode flushes the OS filesystem cache.
  * Hyperfine excludes clean preparation from measured times. Keep toolchain,
    target, features, and clean policy identical when comparing changes.
HELP
}

die() { printf 'error: %s\n' "$*" >&2; exit 2; }

MODULES='none'
LEVELS='base'
COMBINED=0
BASELINE=1
NO_DEFAULT=1
SYS_LEAVES='none'
DOCS='none'
KIND=''
ENV_MODE='base'
UNSAFE_MODE='off'
FEATURES=''
TOOLCHAIN=''
TARGET=''
NATIVE=0
CLEAN='auto'
RUNS=5
WARMUP=1
OUT=''
LIST_ONLY=0

while (($#)); do
    case "$1" in
        --modules|--levels|--sys-leaves|--docs|--kind|--env|--unsafe|--features|--extra|--toolchain|--target|--clean|--runs|--warmup|--out)
            (($# >= 2)) || die "$1 requires a value"
            key=$1; val=$2; shift 2
            case "$key" in
                --modules) MODULES=$val;;
                --levels) LEVELS=$val;;
                --sys-leaves) SYS_LEAVES=$val;;
                --docs) DOCS=$val;;
                --kind) KIND=$val;;
                --env) ENV_MODE=$val;;
                --unsafe) UNSAFE_MODE=$val;;
                --features|--extra) FEATURES=$val;; # --extra: legacy WIP alias
                --toolchain) TOOLCHAIN=$val;;
                --target) TARGET=$val;;
                --clean) CLEAN=$val;;
                --runs) RUNS=$val;;
                --warmup) WARMUP=$val;;
                --out) OUT=$val;;
            esac;;
        --combined) COMBINED=1; shift;;
        --no-baseline) BASELINE=0; shift;;
        --default-features) NO_DEFAULT=0; shift;;
        --native) NATIVE=1; shift;;
        --list) LIST_ONLY=1; shift;;
        -h|--help) usage; exit 0;;
        *) die "unknown option: $1";;
    esac
done

[[ $LEVELS == base || $LEVELS == all || $LEVELS == both ]] || die 'invalid --levels'
[[ $ENV_MODE == base || $ENV_MODE == alloc || $ENV_MODE == std || $ENV_MODE == all ]] || die 'invalid --env'
[[ $UNSAFE_MODE == off || $UNSAFE_MODE == on || $UNSAFE_MODE == both ]] || die 'invalid --unsafe'
[[ $DOCS == none || $DOCS == min || $DOCS == nodep || $DOCS == full || $DOCS == max || $DOCS == all ]] || die 'invalid --docs'
[[ $CLEAN == auto || $CLEAN == package || $CLEAN == docs || $CLEAN == workspace || $CLEAN == none ]] || die 'invalid --clean'
[[ -z $KIND || $KIND == check || $KIND == build || $KIND == doc ]] || die 'invalid --kind'
[[ $RUNS =~ ^[1-9][0-9]*$ && ( $WARMUP == 0 || $WARMUP =~ ^[1-9][0-9]*$ ) ]] || die 'runs must be positive; warmup nonnegative (no leading zeros)'
[[ -z $TOOLCHAIN || $TOOLCHAIN =~ ^\+[A-Za-z0-9_.-]+$ ]] || die '--toolchain should be +NAME'
[[ -z $TARGET || $TARGET =~ ^[A-Za-z0-9_.-]+$ ]] || die 'invalid --target triple'
[[ $MODULES == all || $MODULES == none || $MODULES =~ ^[A-Za-z0-9_]+(,[A-Za-z0-9_]+)*$ ]] || die 'invalid --modules list'
[[ $SYS_LEAVES == none || $SYS_LEAVES =~ ^[A-Za-z0-9_]+(,[A-Za-z0-9_]+)*$ ]] || die 'invalid --sys-leaves list'
[[ -z $FEATURES || $FEATURES =~ ^[A-Za-z0-9_]+(,[A-Za-z0-9_]+)*$ ]] || die 'invalid --features list'

# A docs-only invocation should use rustdoc even when --modules was omitted.
# Explicit --kind check can instead benchmark checking the documentation scopes.
if [[ -z $KIND ]]; then
    if [[ $MODULES == none && $SYS_LEAVES == none && $DOCS != none ]]; then KIND=doc
    else KIND=check
    fi
fi
if [[ -z $TOOLCHAIN && $KIND == doc ]]; then TOOLCHAIN='+nightly'; fi

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
[[ -x ./x ]] || die "expected executable ./x at workspace root: $ROOT"

ROOT_MODULES=(code data geom lang media num org phys run sys text ui work)
SYS_CHOICES=(linux term hw x11 alsa web android)

module_valid() {
    local m=$1 candidate
    for candidate in "${ROOT_MODULES[@]}"; do [[ $m == "$candidate" ]] && return 0; done
    return 1
}
leaf_valid() {
    local m=$1 candidate
    for candidate in "${SYS_CHOICES[@]}"; do [[ $m == "$candidate" ]] && return 0; done
    return 1
}

selected=()
if [[ $MODULES == all ]]; then
    selected=("${ROOT_MODULES[@]}")
elif [[ $MODULES != none ]]; then
    IFS=',' read -r -a selected <<< "$MODULES"
    for m in "${selected[@]}"; do module_valid "$m" || die "unknown root module: $m"; done
fi
leaves=()
if [[ $SYS_LEAVES != none ]]; then
    IFS=',' read -r -a leaves <<< "$SYS_LEAVES"
    for m in "${leaves[@]}"; do leaf_valid "$m" || die "unknown sys leaf: $m"; done
fi

levels=()
case $LEVELS in
    base) levels=(base);;
    all) levels=(all);;
    both) levels=(base all);;
esac
envs=()
case $ENV_MODE in
    base|alloc|std) envs=("$ENV_MODE");;
    all) envs=(base alloc std);;
esac
safeties=()
case $UNSAFE_MODE in
    off|on) safeties=("$UNSAFE_MODE");;
    both) safeties=(off on);;
esac

# Store each fully quoted Cargo command as one Hyperfine scenario.
names=()
features=()
commands=()
x_args=(./x)
[[ -z $TOOLCHAIN ]] || x_args+=("$TOOLCHAIN")
((NATIVE == 0)) || x_args+=(--native)
case $KIND in
    check) cargo_op=c;;
    build) cargo_op=b;;
    doc) cargo_op=d;;
esac

make_command() {
    local joined=$1
    local -a cmd=("${x_args[@]}" "$cargo_op" -p devela --lib --locked)
    ((NO_DEFAULT == 0)) || cmd+=(--no-default-features)
    [[ -z $joined ]] || cmd+=(-F "$joined")
    [[ -z $TARGET ]] || cmd+=(--target "$TARGET")
    local part text=''
    for part in "${cmd[@]}"; do printf -v part '%q' "$part"; text+="${part} "; done
    REPLY="${text% }"
}

add_case() {
    local label=$1 base_features=$2 env safety joined
    for env in "${envs[@]}"; do
        for safety in "${safeties[@]}"; do
            joined=$base_features
            [[ $env == base ]] || joined="${joined:+$joined,}$env"
            [[ $safety == off ]] || joined="${joined:+$joined,}unsafe"
            [[ -z $FEATURES ]] || joined="${joined:+$joined,}$FEATURES"
            make_command "$joined"
            names+=("$label [env=$env unsafe=$safety]")
            features+=("${joined:--}")
            commands+=("$REPLY")
        done
    done
}

if ((BASELINE)); then
    # With --features the reference case is no longer feature-less.
    if [[ -n $FEATURES ]]; then add_case 'features' ''
    else add_case 'baseline' ''
    fi
fi
for level in "${levels[@]}"; do
    combined=''
    for m in "${selected[@]}"; do
        if [[ $level == base ]]; then f=$m; else f="$m,${m}_all"; fi
        add_case "$m/$level" "$f"
        combined="${combined:+$combined,}$f"
    done
    if ((COMBINED)) && ((${#selected[@]})); then
        if [[ $MODULES == all && $level == all ]]; then
            add_case 'combined/all' 'all'
        else
            add_case "combined/$level" "$combined"
        fi
    fi
done
for leaf in "${leaves[@]}"; do
    add_case "sys/$leaf" "sys,$leaf"
done
case $DOCS in
    none) ;;
    min) add_case 'docs/min' '_docs_min';;
    nodep) add_case 'docs/nodep' '_docs_nodep';;
    full) add_case 'docs/full' '_docs';;
    max) add_case 'docs/max' '_docs_max';;
    all)
        add_case 'docs/min' '_docs_min'
        add_case 'docs/nodep' '_docs_nodep'
        add_case 'docs/full' '_docs'
        add_case 'docs/max' '_docs_max';;
esac
((${#commands[@]})) || die 'empty scenario selection'

case $CLEAN in
    auto)
        if [[ $KIND == doc ]]; then CLEAN=docs; else CLEAN=package; fi;;
esac
prepare=()
case $CLEAN in
    package) prepare=(--prepare './x clean -p devela');;
    docs) prepare=(--prepare './x clean --doc');;
    workspace) prepare=(--prepare './x clean');;
    none) ;;
esac

scenario_count=${#commands[@]}
planned_runs=$((scenario_count * (10#$RUNS + 10#$WARMUP)))
printf 'Scenarios: %d | planned invocations: %d (warmups=%d, measured=%d each)\n' \
    "$scenario_count" "$planned_runs" "$WARMUP" "$RUNS"
printf 'kind=%s toolchain=%s target=%s clean=%s\n' \
    "$KIND" "${TOOLCHAIN:-workspace default}" "${TARGET:-host}" "$CLEAN"
for i in "${!commands[@]}"; do
    printf '%3d  %-37s features=%s\n' "$((i+1))" "${names[i]}" "${features[i]}"
done
if ((LIST_ONLY)); then exit 0; fi
command -v hyperfine >/dev/null || die 'hyperfine not found in PATH'

[[ -n $OUT ]] || OUT="${TMPDIR:-/tmp}/devela-compile-$(date +%Y%m%d-%H%M%S)"
# Never silently replace a previous report in a reused output directory.
if [[ -e $OUT/metadata.txt || -e $OUT/results.json || -e $OUT/results.csv || -e $OUT/scenarios.tsv ]]; then
    die "results already exist in output directory: $OUT"
fi
mkdir -p "$OUT"
OUT="$(cd "$OUT" && pwd)"

{
    printf 'date=%s\n' "$(date --iso-8601=seconds)"
    printf 'workspace=%s\n' "$ROOT"
    printf 'git_revision=%s\n' "$(git rev-parse HEAD 2>/dev/null || echo unknown)"
    printf 'git_dirty=%s\n' "$(if [[ -n $(git status --porcelain 2>/dev/null) ]]; then echo yes; else echo no; fi)"
    printf 'kind=%s\nclean=%s\ntoolchain=%s\ntarget=%s\n' "$KIND" "$CLEAN" "${TOOLCHAIN:-default}" "${TARGET:-host}"
    printf 'runs=%s\nwarmup=%s\nno_default_features=%s\n' "$RUNS" "$WARMUP" "$NO_DEFAULT"
    printf 'features=%s\n' "$FEATURES"
    printf 'modules=%s\nlevels=%s\ncombined=%s\nbaseline=%s\n' \
        "$MODULES" "$LEVELS" "$COMBINED" "$BASELINE"
    printf 'sys_leaves=%s\ndocs=%s\n' "$SYS_LEAVES" "$DOCS"
    printf 'env=%s\nunsafe=%s\nnative=%s\n' "$ENV_MODE" "$UNSAFE_MODE" "$NATIVE"
    printf 'scenarios=%s\nplanned_invocations=%s\n' "$scenario_count" "$planned_runs"
} > "$OUT/metadata.txt"
{
    printf 'label\tfeatures\tcommand\n'
    for i in "${!commands[@]}"; do
        printf '%s\t%s\t%s\n' "${names[i]}" "${features[i]}" "${commands[i]}"
    done
} > "$OUT/scenarios.tsv"

start_time=$(date +%s.%N)
finish() {
    local status=$?
    trap - EXIT
    local end_time total
    end_time=$(date +%s.%N)
    total=$(awk -v start="$start_time" -v end="$end_time" 'BEGIN { printf "%.2f", end-start }')
    printf '\nBattery wall time: %s seconds (includes cleaning, warmups and measured runs).\n' "$total"
    printf 'status=%s\ntotal_wall_seconds=%s\n' "$status" "$total" >> "$OUT/metadata.txt"
    printf 'Results: %s\n' "$OUT"
    exit "$status"
}
trap finish EXIT

hyperfine --warmup "$WARMUP" --runs "$RUNS" \
    "${prepare[@]}" \
    --export-json "$OUT/results.json" \
    --export-csv "$OUT/results.csv" \
    "${commands[@]}"
