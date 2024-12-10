## Features

Features are grouped in the following categories:
- [*Development*](#development-features) (`__*`)
- [*Environment*](#environment-features) (`alloc`, `std`, `no_std`)
- [*Module*](#module-features) (`all`, `code`, `data`, `lang`, `media`, `num`, …)
- [*Safety*](#safety-features) (`safe*`, `unsafe*`)
- [*Nightly*](#nightly-features) (`nightly_*`)
- [*Capability*](#capability-features)  (`_*`)
- [*Dependency*](#dependency-features) (`dep_*`)

[*environment*]: #environment-features
[*dependency*]: #dependency-features
[*safety*]: #safety-features

There are no features enabled by default.

Features from different categories are designed to be mostly independent from
each other, to be composable.

Note that features are not all additive,
e.g. it's not possible to enable at the same time `std` and `no_std`,
nor `safe` and `unsafe`.


### Development features

Intended for development and internal purposes, like debugging and maintenance.

- `__dbg`: for debugging purposes, shows enabled features and reflection flags.
- `__no_test`: allows excluding examples from being tested.


### Environment features

By default the crate is `no_std` compatible without allocation.

- `std`: disables `no_std` compatibility and enables `std` functionality.
- `alloc`: enables `alloc` functionality.
- `no_std`: enables functionality incompatible with or substitute of `std`:
  - enables the `libm` optional dependency.


### Module features

Modules can be enabled independently of [*environment*], [*safety*] or [*dependency*] features.

- `all`: enables all the root modules and extra submodules:

Root modules & public sub-modules:
- [`code`]
- [`data`]
  - [`hash`]
- [`lang`]
- [`media`]
  - [`audio`]
  - [`color`]
  - [`draw`]
  - [`font`]
  - [`image`]
- [`num`]: enables all of the `num` sub-features:
  - [`geom`]: geometry, linear algebra.
  - [`rand`]: random number generators.
  - `unit`: unit prefixes.
  - [`wave`]: wavelets.
- [`phys`]: enables all `phys` sub-features.
    - [`time`]
- [`sys`]: enables all `sys` sub-features (except for `os`).
  - [`io`]
  - [`mem`]
    - `bit`: `BitSize`.
  - `os`:
    - [`linux`]: Linux functionality.
- [`text`]
  - `ascii`: `AsciiChar`.
  - [`fmt`]: `NumToStr`.
  - [`str`]: `Str`, `ExtStr`, `ExtString`.
- [`ui`]
  - [`layout`]
- [`work`]

[`code`]: crate::code
  [`result`]: mod@crate::result
[`data`]: crate::data
  [`hash`]: crate::data::hash
[`lang`]: crate::lang
[`media`]: mod@crate::media
  [`audio`]: crate::media::audio
  [`color`]: crate::media::color
  [`draw`]: crate::media::draw
  [`font`]: crate::media::font
  [`image`]: crate::media::image
[`num`]: crate::num
  [`geom`]: crate::num::geom
  [`rand`]: crate::num::rand
  [`wave`]: crate::num::wave
[`phys`]: crate::phys
  [`time`]: crate::phys::time
[`sys`]: crate::sys
  [`io`]: crate::sys::io
  [`mem`]: crate::sys::mem
    [`linux`]: crate::sys::os::linux
[`text`]: crate::text
  [`fmt`]: crate::text::fmt
  [`str`]: crate::text::str
[`work`]: crate::work
[`ui`]: crate::ui
  [`layout`]: crate::ui::layout


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
    - `safe_layout`

- `unsafe`: enables `unsafe` (as long as it isn't forbidden for that module), including:
  - `unsafe_array`: faster array initialization, `UninitArray`.
  - `unsafe_async`: task_waker_noop, `CoroRun`.
  - `unsafe_hint`: unreachable_unchecked, assert_unchecked.
  - `unsafe_layout`: `MemPod`, DSTs in the stack, `ExtAny::downcast*`.
  - `unsafe_niche`: unchecked niche constructors.
  - `unsafe_ptr`: `Pinned`, pop methods without `Clone`.
  - `unsafe_slice`: extra slice methods, avoid bound checks.
  - `unsafe_str`: unchecked utf-8 `char` and `&str` conversions.
  - `unsafe_sync`: implement `Send` and `Sync`.
  - `unsafe_syscall`: os syscalls.
  - `unsafe_thread`: `Logging::set_logger_racy`, `Env::{remove_var, set_var}`.

- `safest`: forbids `unsafe` even in dependencies (except for the standard library).


### Nightly features

Enabling any of them sets the `_nightly_·` flag.

- `nightly`: enables the nightly features:
  - `nightly_autodiff`: enables [`autodiff`].
  - `nightly_coro`: enables [`coroutines`], `coroutine_trait`, `iter_from_coroutine`.
  - `nightly_doc`: enables [`doc_cfg`], [`doc_notable_trait`].
  - `nightly_float`: enables [`f16`, `f128`].
  - `nightly_simd`: enables [`portable_simd`].
  - `nightly_stable`: enables stabilized features marked to be released *soon*:
    - `nightly_stable_next1`: in the next version.
    - `nightly_stable_next2`: in the version after that.
    - `nightly_stable_later`: later than that but *soon enough*.

[`autodiff`]: https://github.com/rust-lang/rust/issues/124509
[`coroutines`]: https://github.com/rust-lang/rust/issues/43122
[`doc_cfg]: https://github.com/rust-lang/rust/issues/43781
[`doc_notable_trait`]: https://github.com/rust-lang/rust/issues/45040
[`f16`, `f128`]: https://github.com/rust-lang/rust/issues/116909
[`portable_simd`]: https://github.com/rust-lang/rust/issues/86656


### Capability features

These semi-hidden features allows to fine-tune extra capabilities.
Enabling them will likely worsen compilation times.

- `_default`: enables default capabilities.
- `_max`: enables the maximum capabilities.

Documentation capabilities:
- `_docsrs`: enables the most complete version of the documentation for [docs.rs](https://docs.rs).
- `_docsrs_stable`: like `_docsrs` but without enabling `nightly`.

#### `data` capabilities

Enable specific implementations for [`Bitwise`], [`bitfield`], [`enumset`]:
- `_bit_all`:
    - `_bit_i8`, `_bit_i16`, `_bit_i32`, `_bit_i64`, `_bit_i128`, `_bit_isize`.
    - `_bit_u8`, `_bit_u16`, `_bit_u32`, `_bit_u64`, `_bit_u128`, `_bit_usize`.

They also set the corresponding flag:
`_bit_·`.

[`Bitwise`]: crate::data::Bitwise
[`bitfield`]: crate::data::bitfield
[`enumset`]: crate::code::enumset

Enable specific implementations of data collections
[`Destaque`], [`Stack`]:
- `_collections_all`:
  - `_destaque_all`:
    - `_destaque_u8`, `_destaque_u16`, `_destaque_u32`, `_destaque_usize`.
  `_stack_all`:
    `_stack_u8`, `_stack_u16`, `_stack_u32`, `_stack_usize`.

They also set the corresponding flags:
`_destaque_·`, `_graph_·`, `_node_·`, `_stack_·`.

Enable specific implementations for [`Sort`].
`_sort_all`:
  `_sort_u8`, `_sort_u16`, `_sort_u32`, `_sort_u64`, `_sort_u128`, `_sort_usize`,
  `_sort_i8`, `_sort_i16`, `_sort_i32`, `_sort_i64`, `_sort_i128`, `_sort_isize`,
  `_sort_f32`, `_sort_f64`.

They also set the corresponding flags:
`_sort_·`, `_sort_int_·`, `_sort_float_·`.

Implement the [`Tuple`] trait for some maximum arity (12 by default).
- `_tuple[_24|_36|_48|_72]`.

[`Destaque`]: crate::data::Destaque
[`Stack`]: crate::data::Stack
[`Sort`]: crate::data::Sort
[`Tuple`]: crate::data::Tuple

#### `num` capabilities

Enable specific implementations for [`Compare`]:
- `_cmp_all`:
  - `_cmp_f32`, `_cmp_f64`.
  - `_cmp_f16`, `_cmp_f128`. ←(needs `nightly_float`)
  - `_cmp_i8`, `_cmp_i16`, `_cmp_i32`, `_cmp_i64`, `_cmp_i128`, `_cmp_isize`.
  - `_cmp_u8`, `_cmp_u16`, `_cmp_u32`, `_cmp_u64`, `_cmp_u128`, `_cmp_usize`.

They also set the corresponding flag:
`_cmp_·`.

Enable specific implementations for [`Int`], [`Float`], [`Frac`], [`Divisor`], [`Vector`]:
- `_num_all`:
  - `_float_all`:
    - `_float_f32`, `_float_f64`.
    - `_float_f16`, `_float_f128`. ←(needs `nightly_float`)
  - `_int_all`:
    - `_int_iall`:
      - `_int_i8`, `_int_i16`, `_int_i32`, `_int_i64`, `_int_i128`, `_int_isize`.
    - `_int_uall`:
      - `_int_u8`, `_int_u16`, `_int_u32`, `_int_u64`, `_int_u128`, `_int_usize`.

They also set the corresponding flags:
`_nums_·`, `_float_·`, `_int_·`, `_int_i·`, `_int_u·`.

[`Compare`]: crate::num::Compare
[`Float`]: crate::num::Float
[`Frac`]: crate::num::Frac
[`Int`]: crate::num::Int
[`Divisor`]: crate::num::Divisor
[`Vector`]: crate::num::Vector

#### `text` capabilities

Enable specific implementations for [`char*`]:
- `_char7`, `_char8`, `_char16`.

Enable specific implementations for [`StringU*`]*, [`StringNonul`]:
- `_string_all`:
  - `_string_uall`:
    - `_string_u8`, `_string_u16`, `_string_u32`, `_string_usize`.
  - `_string_nonul`.

They also set the corresponding flags:
`_char·`, `_string_·`, `_string_u·`.

[`CharU*`]: crate::text::CharU8
[`StringU*`]: crate::text::StringU8
[`StringNonul`]: crate::text::StringNonul


### Dependency features

Enabling any of them sets the `_dep_·` flag.

- `dep_all`: enables all the optional dependencies
  - `linux_deps`: enables: `atomic`, `bytemuck`.
  - `text_deps`: enables: `const-str`, `memchr`, `unicode-segmentation`, `unicode-width`.
  - `work_deps`: enables `atomic`, `portable-atomic`.
