#!/usr/bin/env python3
"""devela LLVM IR module-tree explorer.

Purpose: group cargo-llvm-lines entries by source-like module paths and compare
         IR line counts and function copies across reports.
Requires: Python 3.10+ (standard library); cargo-llvm-lines to produce input.
Use:     python3 tools/perf/llvm-tree.py llvm.txt --depth 2
         python3 tools/perf/llvm-tree.py llvm.txt --focus num --depth 5
         python3 tools/perf/llvm-tree.py after.txt --compare before.txt
Output:  tree to stdout; optional --csv FILE. See --help.
Note:    attribution from demangled symbols is approximate; IR lines are not
         compilation-time measurements.
"""
import argparse
import csv
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
import re
import sys

ROW = re.compile(r'^\s*(\d+)\s+\([^)]*\)\s+(\d+)\s+\([^)]*\)\s+(.+?)\s*$')
CRATE = re.compile(r'devela\[[0-9a-f]+\]::')
SCOPE = re.compile(r'((?:r#)?[\w·]+)::', re.UNICODE)


@dataclass
class Counts:
    lines: int = 0
    copies: int = 0
    functions: int = 0

    def add(self, other):
        self.lines += other.lines
        self.copies += other.copies
        self.functions += other.functions


@dataclass
class Report:
    direct: dict
    inclusive: dict
    children: dict
    reported_lines: int | None
    reported_copies: int | None
    rows: int
    dropped: int


def module_path(symbol):
    match = CRATE.search(symbol)
    if match is None:
        return ('[other crates / unattributed]',)
    rest = symbol[match.end():]
    segments = []
    while match := SCOPE.match(rest):
        segments.append(match.group(1))
        rest = rest[match.end():]
    return tuple(segments) if segments else ('[crate-level items]',)


def read_report(path):
    direct = defaultdict(Counts)
    reported_lines = reported_copies = None
    rows = dropped = 0
    with open(path, encoding='utf-8', errors='replace') as fh:
        for line in fh:
            match = ROW.match(line)
            if not match:
                continue
            lines, copies = int(match.group(1)), int(match.group(2))
            symbol = match.group(3)
            if symbol == '(TOTAL)':
                reported_lines, reported_copies = lines, copies
                continue
            path = module_path(symbol)
            direct[path].add(Counts(lines, copies, 1))
            rows += 1
    inclusive = defaultdict(Counts)
    children = defaultdict(set)
    for path, own in direct.items():
        for depth in range(len(path) + 1):
            prefix = path[:depth]
            inclusive[prefix].add(own)
            if depth < len(path):
                children[prefix].add(path[:depth + 1])
    if reported_lines is not None:
        dropped = reported_lines - inclusive[()].lines
    return Report(dict(direct), dict(inclusive), dict(children),
                  reported_lines, reported_copies, rows, dropped)


def fmt(value):
    return f'{value:,}'


def proportion(n, denom):
    return f'{100*n/denom:5.1f}%' if denom else '    - '


def iterate(report, old, base, end_depth, metric, top, min_pct):
    all_inclusive = set(report.inclusive) | (set(old.inclusive) if old else set())
    if base and base not in all_inclusive:
        raise ValueError(f"Module '{'::'.join(base)}' not found")

    def metric_value(path):
        new = report.inclusive.get(path, Counts())
        prior = old.inclusive.get(path, Counts()) if old else Counts()
        if metric == 'delta-lines':
            return abs(new.lines - prior.lines)
        if metric == 'delta-copies':
            return abs(new.copies - prior.copies)
        return getattr(new, metric)

    total_metric = metric.split('-', 1)[-1]
    new_denom = getattr(report.inclusive.get(base, Counts()), total_metric)
    old_denom = getattr(old.inclusive.get(base, Counts()), total_metric) if old else 0
    denom = max(new_denom, old_denom)

    items = [(base, 0)]
    while items:
        node, indent = items.pop()
        yield node, indent
        if len(node) >= end_depth:
            continue
        descendants = set(report.children.get(node, ()))
        if old:
            descendants |= old.children.get(node, set())
        descendants = sorted(descendants, key=lambda p: (-metric_value(p), p))
        if min_pct:
            descendants = [p for p in descendants if denom and
                           100 * max(getattr(report.inclusive.get(p, Counts()), total_metric),
                                     getattr(old.inclusive.get(p, Counts()), total_metric) if old else 0)
                           / denom >= min_pct]
        if top > 0:
            descendants = descendants[:top]
        items.extend((p, indent + 1) for p in reversed(descendants))


def render(report, old, opts):
    base = tuple(s for s in opts.focus.replace('.', '::').split('::') if s)
    if base not in report.inclusive and not (old and base in old.inclusive):
        raise ValueError(f"Unknown focus '{opts.focus}'")
    if opts.depth < len(base):
        raise ValueError('--depth must be >= the number of components in --focus')
    total = report.inclusive[()]
    focus = report.inclusive.get(base, Counts())
    print(f'Source: {opts.report}')
    print(f'Total: {fmt(total.lines)} lines; {fmt(total.copies)} copies; '
          f'{fmt(total.functions)} rows')
    if report.reported_lines is not None:
        print(f'Input TOTAL: {fmt(report.reported_lines)} lines, '
              f'{fmt(report.reported_copies)} copies; '
              f'unattributed difference: {report.dropped:+,} lines')
    print(f'Focus: {"::".join(base) if base else "devela"}  |  '
          f'sort: {opts.sort}  |  max absolute module depth: {opts.depth}')
    if old:
        print(f'Compared with: {opts.compare}')
    print()
    if old:
        print(f'{"Module":<57} {"Lines":>9} {"%total":>7} {"%parent":>8} '
              f'{"Direct":>9} {"Copies":>8} {"Funcs":>7} {"ΔLines":>10} {"ΔCopies":>9}')
    else:
        print(f'{"Module":<57} {"Lines":>9} {"%total":>7} {"%parent":>8} '
              f'{"Direct":>9} {"Copies":>8} {"Funcs":>7}')
    csv_rows = []
    for node, indent in iterate(report, old, base, opts.depth, opts.sort, opts.top, opts.min_pct):
        cur = report.inclusive.get(node, Counts())
        own = report.direct.get(node, Counts())
        parent = report.inclusive.get(node[:-1], Counts()) if node != base else focus
        name = ('devela' if node == () else node[-1])
        label = ('  ' * indent) + name
        if len(label) > 57:
            label = label[:54] + '...'
        old_counts = old.inclusive.get(node, Counts()) if old else Counts()
        delta_lines = cur.lines - old_counts.lines
        delta_copies = cur.copies - old_counts.copies
        result = (f'{label:<57} {cur.lines:>9,} '
                  f'{proportion(cur.lines, total.lines):>7} '
                  f'{proportion(cur.lines, parent.lines):>8} '
                  f'{own.lines:>9,} {cur.copies:>8,} {cur.functions:>7,}')
        if old:
            result += f' {delta_lines:>+10,} {delta_copies:>+9,}'
        print(result)
        csv_rows.append({
            'module': '::'.join(node), 'level': len(node),
            'lines': cur.lines, 'copies': cur.copies, 'functions': cur.functions,
            'direct_lines': own.lines, 'direct_copies': own.copies,
            'percent_crate_lines': round(100*cur.lines/total.lines, 3) if total.lines else 0,
            'percent_parent_lines': round(100*cur.lines/parent.lines, 3) if parent.lines else 0,
            'delta_lines': delta_lines if old else '',
            'delta_copies': delta_copies if old else '',
        })
    if opts.csv:
        outfile = Path(opts.csv)
        outfile.parent.mkdir(parents=True, exist_ok=True)
        with outfile.open('w', encoding='utf-8', newline='') as fh:
            writer = csv.DictWriter(fh, fieldnames=list(csv_rows[0].keys()))
            writer.writeheader()
            writer.writerows(csv_rows)
        print(f'\nCSV: {outfile}')
    print('\nNote: inclusive rows contain all descendant function IR; "Direct" is only'
          ' IR attributed exactly to that module path, not children.')
    print('Attribution uses demangled function paths; trait impls and synthesized symbols '
          'may not map to their source location.')
    print('LLVM lines are NOT the same as cargo check time.')


def main():
    p = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    p.add_argument('report', help='Output from cargo llvm-lines')
    p.add_argument('--depth', type=int, default=2, help='Maximum absolute module depth (default 2)')
    p.add_argument('--focus', default='', help='Subtree e.g. num, num::grain, text.layout')
    p.add_argument('--sort', choices=['lines','copies','functions','delta-lines','delta-copies'], default='lines',
                   help='Sort by current values, or absolute changes when comparing')
    p.add_argument('--top', type=int, default=0, help='Show at most N children per node (0=all)')
    p.add_argument('--min-pct', type=float, default=0, help='Minimum share of focus total to show a node')
    p.add_argument('--compare', help='Older llvm-lines report for per-subtree differences')
    p.add_argument('--csv', help='Export displayed tree as CSV')
    args = p.parse_args()
    if args.sort.startswith('delta-') and not args.compare:
        p.error('--sort delta-* requires --compare')
    try:
        report = read_report(args.report)
        if not report.rows:
            p.error('No recognizable function rows. Please supply the complete llvm-lines table.')
        old = read_report(args.compare) if args.compare else None
        render(report, old, args)
    except (OSError, ValueError) as err:
        p.error(str(err))


if __name__ == '__main__':
    main()
