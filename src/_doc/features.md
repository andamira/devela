## Features and Flags

Features are grouped in the following categories:
- [*Development*](#development-features) (`__*`)
- [*Environment*](#environment-features) (`alloc`, `std`, `no_std`)
- [*Workspace*](#workspace) (`devela_*`)
- [*Module*](#module-features) (`all`, `code`, `data`, `lang`, `media`, `num`, …)
- [*Safety*](#safety-features) (`safe*`, `unsafe*`)
- [*Capability*](#capability-features)  (`_*`)
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


### Workspace features

Enables optional workspace libraries.

- `devela_macros`


### Module features

Enabling a parent module enables all its sub-modules,
except for `os`.

They set automatic compile flags named `*··`, used for reflection.
For example, `num··` will be set if any num submodule feature is enabled.

- `all`: enables all the root modules and extra submodules.

<!-- NOTE some links only work with inlined notation -->
Root modules & public sub-modules features:
- [`code`]
- [`data`]
  - `codec`:
    - [`hash`]:
- [`geom`]: geometry.
  - [`affine`]:
  - [`dir`]:
  - [`fig`]:
  - [`metric`]:
  - [`rel`]:
  - [`space`]:
- [`lang`]:
  - `ffi`: glsl, js.
- [`media`]
  - [`audio`]:
  - [`color`]:
  - [`draw`]:
  - [`font`]:
  - [`image`]:
  - [`video`]:
- [`num`]:
  - [`dom`]:
  - [`fin`]:
  - [`grain`]:
  - [`lin`]:
  - [`prob`]:
  - [`quant`]:
  - [`symb`]:
- [`phys`]:
  - `bio`:
  - `chem`:
  - `elec`:
  - `mech`:
  - [`time`]
  - [`unit`]:
  - [`wave`][crate::phys::wave]: wavelets.
- [`run`]:
  - `frame`:
  - `setup`:
- [`sys`]: enables all `sys` sub-features (except for `os`).
  - [`device`]
  - [`io`]: no_std `io` implementations.
  - [`mem`]
  - [`os`]:
    - [`linux`]
    - `windows`
- [`text`]
- [`ui`]
  - [`layout`]
- [`work`]
  - `future`:
  - `process`:
  - `sync`:

[`code`]:         crate::code
  [`error`]:      crate::code::error
[`data`]:         crate::data
  [`hash`]:       crate::data::codec::hash
[`geom`]:         crate::geom
  [`affine`]:     crate::geom::affine
  [`dir`]:        crate::geom::dir
  [`fig`]:        crate::geom::fig
  [`metric`]:     crate::geom::metric
  [`rel`]:        crate::geom::rel
  [`space`]:      crate::geom::space
[`lang`]:         crate::lang
[`media`]:        crate::media
  [`audio`]:      crate::media::audio
  [`color`]:      crate::media::color
  [`draw`]:       crate::media::draw
  [`font`]:       crate::media::font
  [`image`]:      crate::media::image
  [`video`]:      crate::media::video
[`num`]:          crate::num
  [`dom`]:        crate::num::fin::logic
  [`fin`]:        crate::num::prob::fin
  [`lin`]:        crate::num::prob::lin
  [`prob`]:       crate::num::prob::prob
  [`quant`]:      crate::num::prob::quant
  [`symb`]:       crate::num::prob::symb
[`phys`]:         crate::phys
  [`time`]:       crate::phys::time
[`run`]:          crate::run
[`sys`]:          crate::sys
  [`device`]:     crate::sys::device
  [`io`]:         crate::sys::io
  [`mem`]:        crate::sys::mem
  [`os`]:         crate::sys::os
    [`linux`]:    crate::sys::os::linux
[`text`]:         crate::text
  [`str`]:        mod@crate::text::str
[`ui`]:           crate::ui
  [`layout`]:     crate::ui::layout
[`work`]:         crate::work
  [`future`]:     crate::work::future
  [`process`]:    crate::work::process
  [`sync`]:       crate::work::sync


### Safety features

They offer a convenient way to opt in and out of safety in a granular fashion.

- `unsafe_*` features enable the use of unsafe by *purpose*.
- `safe_*` features disable the use of unsafe per *module*.

Enabling any of them sets either the `safe··` or `unsafe··` flag.

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


### Capability features

These semi-hidden features allows to fine-tune extra capabilities.
Enabling them will likely worsen compilation times.

Documentation capabilities:
- `_docs[_min|_nodep]`: enables the most complete (or custom) version of the documentation.

#### `data` capabilities

Enable specific implementations of data collections
[`Destaque`], [`Stack`]:
- `_collections_all`:
  - `_destaque_all`:
    - `_destaque_u8`, `_destaque_u16`, `_destaque_u32`, `_destaque_usize`.
  `_stack_all`:
    `_stack_u8`, `_stack_u16`, `_stack_u32`, `_stack_usize`.

They also set the corresponding flags:
`_destaque··`, `_graph··`, `_node··`, `_stack··`.

Implement the [`Tuple`] trait for some maximum arity (12 by default).
- `_tuple[_24|_36|_48|_72]`.

[`Destaque`]: crate::data::Destaque
[`Stack`]: crate::data::Stack
[`Sort`]: crate::data::Sort
[`Tuple`]: crate::data::Tuple


### Dependency features

- Optional external dependencies.
- Re-exported from the [`_dep`][crate::_dep] root module.
- Can be enabled with the `dep_crate_name` feature in snake_case.
- Enabling any of them sets the `_dep··` flag.

- `dep_all`: enables all the optional dependencies.

There are also the following groups of dependencies:
- `alloc_deps`: enables: `alloc`, `dep_allocator_api2`.
- `linux_deps`: enables: `linux`, `dep_atomic`, `dep_bytemuck`, `dep_nc`, `dep_rustix`.
- `text_deps`: enables: `text`, `dep_const_str`, `dep_memchr`, `dep_regex_lite`, `dep_stringzilla`, `dep_unicode_segmentation`, `dep_unicode_width`.
- `work_deps`: enables `work`, `dep_atomic`, `dep_portable_atomic`, `dep_rayon`, `dep_tokio`.


### Nightly flags

Enabling any of them sets the `nightly··` reflection flag.

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
  - `nightly_stable`: enables stabilized features marked to be released *soon*:
    - `nightly_stable_{MSRV +1 | +2}`: the next 2 versions
    - `nightly_stable_later`: later than that but *soon enough*.

[`allocator_api`]: https://github.com/rust-lang/rust/issues/32838
[`autodiff`]: https://github.com/rust-lang/rust/issues/124509
[`coroutines`]: https://github.com/rust-lang/rust/issues/43122
[`doc_cfg`]: https://github.com/rust-lang/rust/issues/43781
[`doc_notable_trait`]: https://github.com/rust-lang/rust/issues/45040
[`f16`, `f128`]: https://github.com/rust-lang/rust/issues/116909
[`portable_simd`]: https://github.com/rust-lang/rust/issues/86656

