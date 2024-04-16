## Features

Features are grouped in the following categories:
- *Miscellaneous*
- *Environment*
- *Module*
- *Safety*
- *Nightly*
- *Capability*
- *Dependency*

Only the `_default` capabilities are enabled by default.

Features from different categories are designed to be mostly independent from
each other, and composable, except from the miscellaneous features.

### Miscellaneous features

In this category there are features with varied purposes mostly for internal use.



### Environment features

By default the crate is `no_std` compatible without allocation.

- `std`: disables `no_std` compatibility and enables `std` functionality.
- `alloc`: enables `alloc` functionality.
- `no_std`: enables functionality incompatible with or substitute of `std`:
  - enables the `libm` optional dependency.


### Module features

Modules can be enabled independently of *environment*, *dependency* or *safety*.

- `all`: enables all the root modules and extra submodules:

Single modules:
- [`code`]
- [`data`]
  - `data_bit`: enables `BitOps`, `Biting`, `bitfield!`.
  - `data_collections`:
- [`exec`]: enables `exec` functionality.
- [`lex`]: enables `Char*`, `Egc`, `Nonul`.
- [`mem`]
  - `mem_bit`: `BitSize`.
- [`num`]: enables all of the `num` sub-features:
    - `num_float`: `Float`, `ExtFloat`.
    - `num_geom`: enables geometric types and operations.
    - `num_int`: `Divisor`, `Frac`, `Int`, `NumInt`, `NumRefInt`.
    - `num_niche_range`: `Range*` and `NonRange*` niche types.
- [`rend`]
    - `rend_audio`
    - `rend_color`
    - `rend_font`
    - `rend_image`
    - `rend_video`
- [`sys`]
- [`time`]
- [`ui`]: enables all of the `ui` sub-features:
    - `ui_events`: extra support for events.
    - `ui_term`: enables the terminal functionality.

[`code`]: crate::code
[`data`]: crate::data
[`exec`]: crate::exec
[`lex`]: crate::lex
[`num`]: crate::num
[`rend`]: crate::rend
[`niche`]: crate::num::niche
[`sys`]: crate::sys
[`time`]: crate::time
[`ui`]: crate::ui
[`ui_term`]: crate::ui_term


### Safety features

In order to use any unsafe functionality:
1. enable the corresponding `unsafe` feature.
2. don't enable the `safe` feature for that module.

- `safe`: forbids `unsafe` (and overrides unsafe features), includes:
  - `safe_code`
  - `safe_data`
  - `safe_lex`
  - `safe_num`
  - `safe_rend`
  - `safe_sys`
  - `safe_time`
  - `safe_ui`, `safe_ui_term`
  - `safe_work`

- `unsafe`: enables `unsafe` (as long as it isn't forbidden in the module), includes:
	- `unsafe_array` faster array initialization.
	- `unsafe_async` custom task waker, coroutine impls.
	- `unsafe_const` extra const methods.
	- `unsafe_dyn` DSTs in the stack, `no_std` Error dyn impls, `ExtAny::downcast*`.
	- `unsafe_niche` unchecked niche constructors.
	- `unsafe_ptr` pop methods without `Clone`.
	- `unsafe_slice` extra slice methods, avoid bound checks.
	- `unsafe_str` unchecked utf-8 char and &str conversions.
	- `unsafe_thread` `Logging::set_logger_racy`.


### Nightly features

- `nightly`: enables nightly features.
  - `nightly_coro`: enables `coroutines`, `coroutine_trait`, `iter_from_coroutine`.
  - `nightly_doc`: enables `doc_cfg`.


### Capability features

These semi-hidden features allows to fine-tune extra capabilities.
Enabling them will likely worsen compilation times.

- `_default`: enables default capabilities.
- `_max`: enables the maximum capabilities.
- `_tuple_arity_[31|63|96|127]`: increased arity support for [`ExtTuple`].

Enable specific implementations for [`Int`], [`Float`], [`Frac`], [`Divisor`],
[`Angle`], [`Point`], [`Vector`]:
- `_num_all`:
  - `_float_all`:
    - `_float_f32`, `_float_f64`.
  - `_int_all`:
    - `_int_all_i`:
      - `_int_i8`, `_int_i16`, `_int_i32`, `_int_i64`, `_int_i128`, `_int_isize`.
    - `_int_all_u`:
      - `_int_u8`, `_int_u16`, `_int_u32`, `_int_u64`, `_int_u128`, `_int_usize`.

[`ExtTuple`]: crate::data::collections::ExtTuple
[`Float`]: crate::num::Float
[`Frac`]: crate::num::Frac
[`Int`]: crate::num::Int
[`Divisor`]: crate::num::Divisor
[`Angle`]: crate::num::Angle
[`Point`]: crate::num::Point
[`Vector`]: crate::num::Vector

Enable specific implementations for [`Bitwise`], [`bitfield`], [`enumset`]:
- `_bit_all`:
  - `_bit_all_i`:
    - `_bit_i8`, `_bit_i16`, `_bit_i32`,
      `_bit_i64`, `_bit_i128`, `_bit_isize`.
  - `_bit_all_u`:
    - `_bit_u8`, `_bit_u16`, `_bit_u32`,
      `_bit_u64`, `_bit_u128`, `_bit_usize`.

[`Bitwise`]: crate::num::Bitwise
[`bitfield`]: crate::num::bitfield
[`enumset`]: crate::num::enumset

 Documentation capabilities:
- `_docsrs`: enables the most complete version of the documentation for [docs.rs](https://docs.rs).
- `_docsrs_max`: like `_docsrs` but also enables `capability_max`.
- `_docsrs_stable`: like `_docsrs` but without enabling `nightly`.


### Dependency features

- `dep_all`: enables all the optional dependencies
  - `dep_exec`: enables `atomic`, `portable-atomic`.
  - `dep_lex`: enables: `const-str`, `memchr`, `unicode-segmentation`, `unicode-width`.
  - `dep_ui_term`: enables `const-str`, `crossterm`.
