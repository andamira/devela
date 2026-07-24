## Features and Flags

Features are grouped in the following categories:
- [*Development*](#development-features) (`__*`)
- [*Environment*](#environment-features) (`alloc`, `std`, `no_std`)
- [*Module*](#module-features) (`all`, `code`, `data`, `lang`, `media`, `num`, …)
- [*Safety*](#safety-features) (`safe*`, `unsafe*`)
- [*Scope*](#scope-features)  (`_*`)
- [*Dependency*](#dependency-features) (`dep_*`)

Flags are grouped in the following categories:
- [*Nightly*](#nightly-flags) (`nightly_*`)
- *reflection* (`*··`)

There are no features enabled by default.

Features from *different categories* are designed to be (for the most part)
*independent from each other*, to be orthogonally composable.

Note however that not all features are additive,
e.g. it's not possible to enable at the same time `std` and `no_std`,
nor `safe` and `unsafe`.


### Development features

Intended for development and internal purposes, like debugging and maintenance.

- `__dbg`    : for debugging purposes, shows enabled features and reflection flags.
- `__no_test`: its purpose is to exclude certain examples from being tested.
- `__publish`: for when publishing to crates.io or building online docs.


### Environment features

By default the crate is `no_std` compatible without allocation.

- `std`: disables `no_std` compatibility and enables `std` functionality.
- `alloc`: enables `alloc` functionality.
- `no_std`: enables functionality incompatible with or substitute of `std`.


### Module features

Visible module features are grouped by public meaning.

A bare root-module feature, such as `ui`, `media`, or `num`, enables the
canonical public substrate of that module. It should remain useful on its own
and should avoid pulling adjunct layers that are not always needed.

Each root module also has a public `*_all` feature, such as `ui_all`,
`media_all`, or `num_all`. This enables the complete public family associated
with that module, including its public adjunct features.

A public adjunct feature may live under a module namespace while remaining
independently gated when it represents a meaningful supported layer. Examples:
`event`, `widget`, `font`, `image`, `time`, `process`.

The root `all` feature enables the public `*_all` feature of each root module.

Module-family reflection flags named with the `··` suffix are set
when any feature in the corresponding public family is enabled.


### Safety features

They offer a convenient way to opt in and out of safety in a granular fashion.

- `unsafe_*` features enable the use of unsafe by *purpose*.
- `safe_*` features disable the use of unsafe per *module*.

To be able to use any unsafe functionality it's necessary to:
1. enable the corresponding `unsafe` feature.
2. don't enable that module's `safe` feature.

- `safe`: forbids `unsafe` (and overrides unsafe features), including
  - `safe_code`
  - `safe_data`
  - `safe_lang`
  - `safe_media`
    - `safe_audio`
    - `safe_color`
    - `safe_draw`
    - `safe_font`
    - `safe_image`
  - `safe_num`
  - `safe_phys`
    - `safe_time`
  - `safe_sys`
    - `safe_io`
    - `safe_mem`
  - `safe_text`
  - `safe_work`
  - `safe_ui`

- `unsafe`: enables `unsafe` (as long as it isn't forbidden for that module), including:
  - `unsafe_array`: faster array initialization, `UninitArray`.
  - `unsafe_ffi`: unsafe foreign function calls (WASM, OS, external C).
  - `unsafe_hint`: unreachable_unchecked, assert_unchecked.
  - `unsafe_layout`: `MemPod`, DSTs in the stack, `AnyExt::downcast*`.
  - `unsafe_niche`: unchecked niche constructors.
  - `unsafe_ptr`: `Pinned`, pop methods without `Clone`.
  - `unsafe_slice`: extra slice methods, avoid bound checks.
  - `unsafe_str`: unchecked utf-8 `char` and `&str` conversions.
  - `unsafe_sync`: `SpinLock`, implement `Send` and `Sync`.
  - `unsafe_syscall`: os syscalls.
  - `unsafe_thread`: `Logging::set_logger_racy`, `Env::{remove_var, set_var}`.

- `safest`: forbids `unsafe` even in dependencies (except for the standard library).


### Scope features

Scope features are semi-hidden features used to expand implementation coverage,
documentation coverage, generated code, or internal capability breadth.

They are usually prefixed with `_`. Enabling them may increase compilation time significantly.

Public `*_all` module features, such as `data_all`, are not scope features.
They enable complete public feature families. Hidden `_module_all` features,
such as `_data_all`, are internal scope expansions.

#### Documentation scope

- `_docs[_min|_nodep]`: enables the most complete or customized documentation
  configuration.

#### `code` scope

Implements the [`unroll!`] macro for a selected maximum recursion depth (64 by default).

- `_unroll[_128|_256|_512|_1024|_2048]`

#### `data` scope

Expands internal data-structure coverage and generated implementations.

- `_data_all`: enables all hidden data scope expansions.
- `_collections_all`: enables hidden collection implementation families.
- `_tuple[_24|_36|_48|_72]`: implements the [`Tuple`] trait for a selected
  maximum arity (12 by default).

[`unroll!`]: crate::code::util::unroll
[`Tuple`]: crate::data::value::Tuple


### Dependency features

- Optional external dependencies.
- Re-exported from the hidden [`devela::_dep`] root module.
- Can be enabled with the `dep_crate_name` feature in snake_case.

- `dep_all`: enables all the optional dependencies.

There are also the following groups of dependencies:
- `work_deps`: enables `work`, `dep_atomic`, `dep_portable_atomic`.


### Nightly flags

Usage example:
```sh
RUSTFLAGS="--cfg nightly_coro --cfg nightly_stable_next1" cargo +nightly build
```

- `nightly`: enables the nightly cfg flags:
  - `nightly_allocator`: enables [`allocator_api`].
  <!-- - `nightly_autodiff`: enables [`autodiff`]. FEATURE_DISABLED:nightly_autodiff -->
  - `nightly_coro`: enables [`coroutines`], `coroutine_trait`, `iter_from_coroutine`.
  - `nightly_doc`: enables [`doc_cfg`], [`doc_notable_trait`].
  - `nightly_float`: enables [`f16`, `f128`].
  - `nightly_simd`: enables [`portable_simd`].
  - `nightly_stable`: enables stabilized features marked to be released *soon*™:
    - `nightly_stable_{MSRV +1 | +2}`: the next 2 versions
    - `nightly_stable_later`: later than that but hopefully *soon enough*.

[`allocator_api`]: https://github.com/rust-lang/rust/issues/32838
[`autodiff`]: https://github.com/rust-lang/rust/issues/124509
[`coroutines`]: https://github.com/rust-lang/rust/issues/43122
[`doc_cfg`]: https://github.com/rust-lang/rust/issues/43781
[`doc_notable_trait`]: https://github.com/rust-lang/rust/issues/45040
[`f16`, `f128`]: https://github.com/rust-lang/rust/issues/116909
[`portable_simd`]: https://github.com/rust-lang/rust/issues/86656

