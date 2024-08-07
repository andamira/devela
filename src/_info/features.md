## Features

Features are grouped in the following categories:
- *Miscellaneous*
- *Environment*
- *Modules*
- *Safety*
- *Nightly*
- *Capability*
- *Dependencies*

Only the `_default` capabilities are enabled by default.

Features from different categories are designed to be mostly independent from
each other, and composable, except from the miscellaneous features.

### Miscellaneous features

In this category there are features with varied purposes mostly for internal use.

- `__dbg`: for debugging purposes, shows enabled features and reflection flags.
- `__no_test`: allows excluding examples from being tested.


### Environment features

By default the crate is `no_std` compatible without allocation.

- `std`: disables `no_std` compatibility and enables `std` functionality.
- `alloc`: enables `alloc` functionality.
- `no_std`: enables functionality incompatible with or substitute of `std`:
  - enables the `libm` optional dependency.


### Modules features

Modules can be enabled independently of *environment*, *dependencies* or *safety*.

- `all`: enables all the root modules and extra submodules:

Single modules:
- [`code`]
- [`data`]
- [`exec`]: enables `exec` functionality.
- [`mem`]
  - `mem_bit`: `BitSize`.
- [`num`]: enables all of the `num` sub-features:
    - [`num_geom`]: enables geometric types and operations.
    - [`num_rand`]: enables random number generators.
- [`sys`]
- [`text`]: enables `Char*`, `Egc`, `Nonul`.
- [`time`]

Enabling `mem`, `num`, or their submodules, sets the corresponding flags:
`_some_mem`, `_some_num`.

[`code`]: crate::code
[`data`]: crate::data
[`exec`]: crate::exec
[`mem`]: crate::mem
[`num`]: crate::num
[`num_geom`]: crate::num::geom
[`num_rand`]: crate::num::rand
[`sys`]: crate::sys
[`text`]: crate::text
[`time`]: crate::time


### Safety features

In order to use any unsafe functionality:
1. enable the corresponding `unsafe` feature.
2. don't enable the `safe` feature for that module.

- `safe`: forbids `unsafe` (and overrides unsafe features), includes:
  - `safe_code`
  - `safe_data`
  - `safe_num`
  - `safe_sys`
  - `safe_text`
  - `safe_time`
  - `safe_work`

- `unsafe`: enables `unsafe` (as long as it isn't forbidden in the module), includes:
	- `unsafe_array`: faster array initialization.
	- `unsafe_async`: custom task waker, coroutine impls.
	- `unsafe_const`: extra const methods.
	- `unsafe_dyn`: DSTs in the stack, `no_std` Error dyn impls, `ExtAny::downcast*`.
	- `unsafe_niche`: unchecked niche constructors.
	- `unsafe_ptr`: `Pinned`, pop methods without `Clone`.
	- `unsafe_slice`: extra slice methods, avoid bound checks.
	- `unsafe_str`: unchecked utf-8 char and &str conversions.
	- `unsafe_thread`: `Logging::set_logger_racy`.


### Nightly features

Enabling any of them sets the `_some_nightly` flag.

- `nightly`: enables the nightly features.
  - `nightly_coro`: enables `coroutines`, `coroutine_trait`, `iter_from_coroutine`.
  - `nightly_doc`: enables `doc_cfg`.
  - `nightly_simd`: enables `portable_simd`.
  - `nightly_stabilized`: enables stabilized features to be released soon.

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
`_some_bit`.

[`Bitwise`]: crate::data::Bitwise
[`bitfield`]: crate::data::bitfield
[`enumset`]: crate::code::enumset

Enable specific implementations of data collections
[`Destaque`], [`GraphU*`]+, [`NodeU*`]+, [`Stack`]:
- `_collections_all`:
	- `_destaque_all`:
    - `_destaque_u8`, `_destaque_u16`, `_destaque_u32`, `_destaque_usize`.
	- `_graph_all`:
    - `_graph_u8`, `_graph_u16`, `_graph_u32`, `_graph_usize`.
	- `_node_all`:
    - `_node_u8`, `_node_u16`, `_node_u32`, `_node_usize`.
	`_stack_all`:
		`_stack_u8`, `_stack_u16`, `_stack_u32`, `_stack_usize`.

They also set the corresponding flags:
`_some_destaque`, `_some_graph`, `_some_node`, `_some_stack`.

Enable specific implementations for [`Sort`].
`_sort_all`:
  `_sort_u8`, `_sort_u16`, `_sort_u32`, `_sort_u64`, `_sort_u128`, `_sort_usize`,
  `_sort_i8`, `_sort_i16`, `_sort_i32`, `_sort_i64`, `_sort_i128`, `_sort_isize`,
  `_sort_f32`, `_sort_f64`.

They also set the corresponding flags:
`_some_sort`, `_some_sort_int`, `_some_sort_float`.

Implement the [`Tuple`] trait for some maximum arity (12 by default).
- `_tuple[_24|_36|_48|_72]`.

[`Destaque`]: crate::data::Destaque
[`Stack`]: crate::data::Stack
[`GraphU*`]: crate::data::GraphU8
[`NodeU*`]: crate::data::NodeU8
[`Sort`]: crate::data::Sort
[`Tuple`]: crate::data::Tuple

#### `num` capabilities

Enable specific implementations for [`Compare`]:
- `_cmp_all`:
  - `_cmp_f32`, `_cmp_f64`.
  - `_cmp_i8`, `_cmp_i16`, `_cmp_i32`, `_cmp_i64`, `_cmp_i128`, `_cmp_isize`.
  - `_cmp_u8`, `_cmp_u16`, `_cmp_u32`, `_cmp_u64`, `_cmp_u128`, `_cmp_usize`.

They also set the corresponding flag:
`_some_cmp`.

Enable specific implementations for [`Int`], [`Float`], [`Frac`], [`Divisor`],
[`Angle`], [`Point`], [`Vector`]:
- `_num_all`:
  - `_float_all`:
    - `_float_f32`, `_float_f64`.
  - `_int_all`:
    - `_int_iall`:
      - `_int_i8`, `_int_i16`, `_int_i32`, `_int_i64`, `_int_i128`, `_int_isize`.
    - `_int_uall`:
      - `_int_u8`, `_int_u16`, `_int_u32`, `_int_u64`, `_int_u128`, `_int_usize`.

They also set the corresponding flags:
`_some_nums`, `_some_float`, `_some_int`, `_some_int_i`, `_some_int_u`.

Enable specific implementations for niche [`NonValue*`], [`NonRange*`], [`InRange*`].
- `_niche_all`:
  - `_non_value_all`:
    - `_non_value_i8`, `_non_value_i16`, `_non_value_i32`,
      `_non_value_i64`, `_non_value_i128`, `_non_value_isize`.
    - `_non_value_u8`, `_non_value_u16`, `_non_value_u32`,
      `_non_value_u64`, `_non_value_u128`, `_non_value_usize`.
  - `_non_range_all`:
    - `_non_range_i8`, `_non_range_i16`, `_non_range_i32`,
      `_non_range_i64`, `_non_range_i128`, `_non_range_isize`.
    - `_non_range_u8`, `_non_range_u16`, `_non_range_u32`,
      `_non_range_u64`, `_non_range_u128`, `_non_range_usize`.
  - `_in_range_all`:
    - `_in_range_i8`, `_in_range_i16`, `_in_range_i32`,
      `_in_range_i64`, `_in_range_i128`, `_in_range_isize`.
    - `_in_range_u8`, `_in_range_u16`, `_in_range_u32`,
      `_in_range_u64`, `_in_range_u128`, `_in_range_usize`.

They also set the corresponding flags:
`_some_niche`, `_some_non_value`, `_some_non_value_i`, `_some_non_value_u`,
`_some_non_range`, `_some_in_range`.

[`Compare`]: crate::num::Compare
[`Float`]: crate::num::Float
[`Frac`]: crate::num::Frac
[`Int`]: crate::num::Int
[`Divisor`]: crate::num::Divisor
[`Angle`]: crate::num::Angle
[`Point`]: crate::num::Point
[`Vector`]: crate::num::Vector
[`InRange*`]: crate::num::InRangeU8
[`NonRange*`]: crate::num::NonRangeU8
[`NonValue*`]: crate::num::NonValueU8

#### `text` capabilities

Enable specific implementations for [`StringU*`]*, [`StringNonul`]:
- `_string_all`:
  - `_string_uall`:
    - `_string_u8`, `_string_u16`, `_string_u32`, `_string_usize`.
  - `_string_nonul`.

They also set the corresponding flags:
`_some_string`, `_some_string_u`.

[`StringU*`]: crate::text::StringU8
[`StringNonul`]: crate::text::StringNonul


### Dependencies features

Enabling any of them sets the `_some_dep` flag.

- `dep_all`: enables all the optional dependencies
  - `dep_exec`: enables `atomic`, `portable-atomic`.
  - `dep_text`: enables: `const-str`, `memchr`, `unicode-segmentation`, `unicode-width`.
