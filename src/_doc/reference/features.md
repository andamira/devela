<!-- devela/src/_doc/reference/features.md -->

## Features and flags

devela has no default Cargo features. Its baseline remains `no_std` compatible
and does not require allocation.

Features are grouped by purpose: environment, public modules and capabilities,
safety, implementation scope, optional dependencies, and development support.
Compiler configuration flags form a separate layer, chiefly for nightly Rust
and internal feature reflection.

These groups are meant to compose. One feature may imply another when the first
has little useful meaning without it; independently useful capabilities remain
separately selectable.

### Environment features

Without `std`, devela compiles as `no_std`.

`alloc` enables functionality requiring Rust's allocation facilities.
`std` implies `alloc` and enables functionality requiring the standard library.

The `no_std` feature has a narrower meaning: it enables functionality
that is specifically incompatible with, or acts as a substitute for, `std`.
It is not necessary merely to compile devela without the standard library.

`std` and `no_std` are mutually exclusive.

### Module and capability features

Public module-family features provide two possible breadth grades.

A bare family feature, such as `data`, `media`, `num`, or `ui`, enables a
lightweight coherent base for that domain. It is intended to remain useful on
its own and to avoid compiling unrelated capabilities merely because they share
the same thematic namespace.

Where a broader distinction is useful, the corresponding `*_all` feature
selects the full ordinary public family. It always includes the bare feature
and adds the public capabilities associated with that domain. For example,
`ui_all` currently enables `ui`, `event`, and `widget`.

Specific capability features remain independently selectable. They should imply
only the minimum other features required for their meaningful operation, and
cross-family dependencies should prefer the narrowest sufficient feature grade.

The module taxonomy and Cargo feature graph therefore need not coincide exactly.
A capability may belong to a module structurally while remaining outside that
module family's lightweight base.

The `*_all` suffix describes thematic breadth, not every feature that can affect
that module. Environment, safety, scope, optional dependencies, nightly
configuration, and platform- or backend-specific selections remain orthogonal,
except where a selected capability intrinsically requires them.

This distinction is especially visible in `sys_all`, which selects the ordinary
portable `sys` family while leaving target-, operating-system-, and
backend-specific capabilities independently selectable.

The root `all` feature enables the `*_all` grade of each feature-gated root
module. It is a broad thematic library selection, not a synonym for enabling
every Cargo feature.


### Safety features

Safety features follow two independent axes.

Features named `safe_*` forbid unsafe code within a module or module family.
For example, `safe_media` includes its audio, font, and visual safety scopes,
while `safe_sys` includes `safe_io` and `safe_mem`.

Features named `unsafe_*` enable unsafe implementation capabilities by purpose,
such as `unsafe_ffi`, `unsafe_layout`, or `unsafe_syscall`.

`safe` enables the library-wide module safety set. `unsafe` enables all
supported unsafe-purpose features. Enabling `safe` together with `unsafe`
or any `unsafe_*` feature is rejected.

Individual `safe_*` features may instead be combined with selected unsafe
purposes to forbid unsafe code only in particular parts of the library.

`safest` extends `safe` by forbidding unsafe code transitively in dependencies,
apart from Rust's `core`, `alloc`, and `std`.

When an `unsafe_*` feature changes the behavior or implementation of an
otherwise available API, the effect is described in that item's `# Features`
section. For example, [`RasterSamplePacked`] accepts any [`MemPod`] sample type
when `unsafe_layout` is enabled.

[`RasterSamplePacked`]: crate::media::visual::image::raster::RasterSamplePacked
[`MemPod`]: crate::sys::mem::MemPod

### Scope features

Features beginning with a single underscore, such as `_tuple`, `_unroll`, or
`_docs`, are semi-hidden scope controls. They expand generated implementations,
documentation coverage, or other internal scope.

They are not ordinary public capabilities and may noticeably increase compile time.
Examples include:

`_unroll*` selecting greater generated unrolling depth;
`_tuple*` selecting greater generated tuple arity;
`_data_all` expanding hidden data coverage;
`_max` and `_maxest` selecting broad internal coverage;
`_docs*` assembling documentation configurations;
and `_linux_abi` exposing additional Linux ABI scope.

Public features such as `data_all` or `ui_all` are not scope features despite
their similar suffix: they select public capability families.

### Dependency features

Optional external dependencies use the `dep_*` prefix,
with dashes converted to underscores where necessary.

For example, `dep_hashbrown` or `dep_rand_core` selects that external dependency
explicitly. `dep_all` enables the complete optional-dependency set.

A capability may enable a dependency feature transitively when that dependency
is intrinsic to the capability. Convenience groups such as `work_deps`
may also select a small related set together.

### Development features

Features beginning with `__` are reserved for development, maintenance, testing,
publishing, and documentation machinery rather than normal library capability.

Current examples include `__dbg`, `__std`, `__publish`, `__docs_internal`,
`__disable_native_libs`, `__exclude_test`, and `__force_miri_dst`.

Their exact set may change with project tooling
and should not be treated as a stable public capability surface.

### Nightly flags

Nightly configuration uses compiler `cfg` flags rather than Cargo features.

For example:

```sh
RUSTFLAGS="--cfg nightly_simd" cargo +nightly build
```

Specific flags enable selected unstable facilities, while versioned
`nightly_stable_*` flags cover functionality expected from upcoming stable Rust
releases. `nightly_stable_later` collects candidates farther from the current MSRV.

The maintained release-by-release inventory lives in `docs/nightly.md`.

### Reflection flags

devela's build machinery derives internal reflection flags
from enabled features and configuration.

These use a `··` suffix, such as `num··`, `dep··`, or `nightly··`, and answer
questions such as whether any feature in a broader family is active.

They are internal summary flags. Users normally enable the original Cargo
feature or `nightly_*` configuration flag rather than setting them directly.
