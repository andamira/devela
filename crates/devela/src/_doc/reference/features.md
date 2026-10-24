
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

The Rust module tree organizes related abstractions; Cargo features control
which capabilities and implementations are compiled. The two structures need
not form the same hierarchy.

**Capability features** such as `ratio`, `prob`, `prng`, `audio`, or `event`
are independently selectable, even when their implementations belong to nested
modules. They imply other features only when those capabilities are genuinely
required, not simply because of module ancestry.

**Module-family features** such as `data`, `media`, `num`, and `ui` are curated
entry points for their domains. Their base selections may be intentionally
small, initially empty, or expanded as useful low-cost bundles emerge.

**Aggregate features** ending in `_all` collect the ordinary public
capabilities of a thematic family, including its base feature where defined.

An implementation family may also keep a **local generation registry**:
its canonical variants are generated once under `#[cfg]` conditions matching
the needs of consuming modules, while a public feature (such as `ratio`)
enables its complete predefined set. The registry and its consumers keep
those conditions synchronized; no separate Cargo feature is needed
for every primitive width or generated variant.

Environment (`alloc`, `std`), safety (`unsafe_*`), scope, dependencies, and
platform/backend capabilities remain independently selectable, except where a
particular capability requires them. In particular, `sys_all` collects ordinary
portable system facilities without automatically enabling all operating-system,
device, or backend implementations.


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


`__select_modules` enables root-module selection during development. Normally
all root modules are present, regardless of their corresponding module-family
features. With `__select_modules`, only `code` and `yard` are always included;
other roots are compiled when their root feature is explicitly enabled.
The development feature enables `code` automatically because its macros
bootstrap the root module declarations.

`-F __select_modules,num` selects the `code`, `yard`, and `num` roots. Some root families
still have dependencies on others; this mode is intended to expose and refine
those dependencies, not to guarantee that every selection already compiles.
The WIP `error` and `vita` roots can be selected separately but are not members
of the public `all` capability aggregate.

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
