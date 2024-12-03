## Features

Features are grouped in the following categories:
- [*Development*][#develoment-features]
- [*Environment*][#environment-features]
- [*Module*][#module-features]
- [*Safety*][#safety-features]
- [*Nightly*][#nightly-features]
- [*Capability*][#capability-features]
- [*Dependency*][#dependency-features]

There are no features enabled by default.

Features from different categories are designed to be mostly independent from
each other, and composable.

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

Modules can be enabled independently of *environment*, *dependencies* or *safety*.

- `all`: enables all the root modules and extra submodules:

Single modules:
- [`code`]
- [`data`]
- [`error`]
- [`ffi`]
- [`mem`]
  - `bit`: `BitSize`.
- [`num`]: enables all of the `num` sub-features:
    - [`geom`]: geometry, linear algebra.
    - [`rand`]: random number generators.
    - [`wave`]: wavelets.
- [`rend`]
  - [`audio`]
  - [`color`]
  - [`draw`]
  - [`font`]
  - [`image`]
  - [`layout`]
- [`sys`]: enables all `sys` sub-features (except os-specific).
  - [`io`]
  - [`time`]
  - os:
    - `linux`: Linux functionality.
- [`text`]
- [`ui`]
- [`work`]

[`code`]: crate::code
[`data`]: crate::data
[`error`]: mod@crate::error
[`ffi`]: crate::ffi
[`rend`]: mod@crate::rend
  [`audio`]: crate::rend::audio
  [`color`]: crate::rend::color
  [`draw`]: crate::rend::draw
  [`font`]: crate::rend::font
  [`image`]: crate::rend::image
  [`layout`]: crate::rend::layout
[`mem`]: crate::mem
[`num`]: crate::num
  [`geom`]: crate::num::geom
  [`rand`]: crate::num::rand
  [`wave`]: crate::num::wave
[`sys`]: crate::sys
  [`time`]: crate::sys::time
[`text`]: crate::text
[`work`]: crate::work


### Safety features

- `unsafe_*` features enable the use of unsafe by *purpose*.
- `safe_*` features disable the use of unsafe per *module*.

In order to use any unsafe functionality:
1. enable the corresponding `unsafe` feature.
2. don't enable the `safe` feature for that module.

- `safe`: forbids `unsafe` (and overrides unsafe features), includes:
  - `safe_code`
  - `safe_data`
  - `safe_error`
  - `safe_ffi`
  - `safe_mem`
  - `safe_num`
  - `safe_rend`
    - `safe_audio`
		- `safe_color`
		- `safe_draw`
		- `safe_font`
		- `safe_image`
		- `safe_layout`
  - `safe_sys`
    - `safe_io`
    - `safe_time`
  - `safe_text`
  - `safe_work`

- `unsafe`: enables `unsafe` (as long as it isn't forbidden for that module), includes:
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

- `safest`: forbids `unsafe` even in dependencies (except the standard library).


### Nightly features

Enabling any of them sets the `_nightly_·` flag.

- `nightly`: enables the nightly features.
  - `nightly_coro`: enables [`coroutines`], `coroutine_trait`, `iter_from_coroutine`.
  - `nightly_doc`: enables [`doc_cfg`], [`doc_notable_trait`].
  - `nightly_float`: enables [`f16`, `f128`].
  - `nightly_simd`: enables [`portable_simd`].
  - `nightly_stabilized`: enables stabilized features to be released soon. See `Cargo.toml`.

[coroutines]: https://github.com/rust-lang/rust/issues/43122
[`doc_cfg]: https://github.com/rust-lang/rust/issues/43781
[`doc_notable_trait`]: https://github.com/rust-lang/rust/issues/45040
[`f16`, `f128`]: https://github.com/rust-lang/rust/issues/116909
[portable_simd]: https://github.com/rust-lang/rust/issues/86656

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
  - `_cmp_i8`, `_cmp_i16`, `_cmp_i32`, `_cmp_i64`, `_cmp_i128`, `_cmp_isize`.
  - `_cmp_u8`, `_cmp_u16`, `_cmp_u32`, `_cmp_u64`, `_cmp_u128`, `_cmp_usize`.

They also set the corresponding flag:
`_cmp_·`.

Enable specific implementations for [`Int`], [`Float`], [`Frac`], [`Divisor`], [`Vector`]:
- `_num_all`:
  - `_float_all`:
    - `_float_f32`, `_float_f64`.
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

Enable specific implementations for [`char*`]*:
- `_char7`, `_char8`, `_char16`, `_char24`.

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
