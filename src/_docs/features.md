## Features

Features are grouped in the following categories:
- *Miscellaneous*
- *Environment*
- *Module*
- *Safety*
- *Nightly*
- *Capability*
- *Dependency*

All features are disabled by default.

Features from different categories are designed to be mostly independent from
each other, and composable, except from the miscellaneous features.

### Miscellaneous features

In this category there are features with varied purposes mostly for internal use.

- `_docsrs`: enables the most complete version of the documentation for [docs.rs](https://docs.rs).
  - Enables: `std`, `all`, `unsafe`, `nightly`, `dep_all`, `libm`.
- `_docsrs_max`: like `_docsrs` but also enables `capability_max`.
- `_docsrs_stable`: like `_docsrs` but without enabling `nightly`.


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
- [`fig`]
- [`gfx`]
- [`io`]: enables the `io` functionality.
- [`mem`]
- [`num`]: enables all of the `num` sub-features:
    - `num_float`: `Float`, `ExtFloat`.
    - `num_int`: `Frac`, `Int`, `NumInt`, `NumRefInt`.
    - `num_niche_range`: `Range*` and `NonRange*` niche types.
- [`os`]
- [`text`]: enables `Char*`, `Egc`, `Nonul`.
- [`time`]
- [`ui`]: enables all of the `ui` sub-features:
    - `ui_events`: extra support for events.
    - `ui_term`: enables the terminal functionality.
- [`work`]: enables `work` functionality.

[`code`]: crate::code
[`data`]: crate::data
[`fig`]: crate::fig
[`gfx`]: crate::gfx
[`io`]: crate::io
[`mem`]: crate::mem
[`num`]: crate::num
[`niche`]: crate::num::niche
[`os`]: crate::os
[`text`]: crate::text
[`time`]: crate::time
[`ui`]: crate::ui
[`ui_term`]: crate::ui_term
[`work`]: crate::work


### Safety features

In order to use any unsafe functionality:
1. enable the corresponding `unsafe` feature.
2. don't enable the `safe` feature for that module.

- `safe`: forbids `unsafe` (and overrides unsafe features), includes:
  - `safe_code`
  - `safe_data`
  - `safe_fig`
  - `safe_gfx`
  - `safe_io`
  - `safe_mem`
  - `safe_num`
  - `safe_os`
  - `safe_text`
  - `safe_time`
  - `safe_ui`
  - `safe_work`

- `unsafe`: enables `unsafe` (as long as it isn't forbidden in the module), includes:
	- `unsafe_array` faster array initialization.
	- `unsafe_async` custom task waker, coroutine impls.
	- `unsafe_const` extra const methods.
	- `unsafe_dyn` DSTs in the stack, `no_std` Error dyn impls.
	- `unsafe_niche` unchecked niche constructors.
	- `unsafe_ptr` pop methods without `Clone`.
	- `unsafe_slice` extra slice methods, avoid bound checks.
	- `unsafe_str` unchecked utf-8 char and &str conversions.


### Nightly features

- `nightly`: enables nightly features.
  - `nightly_coro`: enables `coroutines`, `coroutine_trait`, `iter_from_coroutine`.
  - `nightly_doc`: enables `doc_cfg`.


### Capability features

These semi-hidden features allows to fine-tune extra capabilities.
Enabling them will likely worsen compilation times.

- `capability_max`: enables the maximum capabilities.
- `_tuple_arity_[31|63|96|127]`: increased arity support for [`ExtTuple`].

The following is a tree of features that allows fine-grained control over
for which primitive types several generic implementations will compile.
That's the case for example of [`Float`], [`Int`], or the [`niche`] types.
- `cap_nums:` enables all the numeric types implementations.
  - `cap_floats:` e
    - `f32`, `f64`.
  - `cap_ints:`
    - `cap_signed_ints:`
      - `i8`, `i16`, `i32`, `i64`, `i128`, `isize`.
    - `cap_unsigned_ints:`
      - `u8`, `u16`, `u32`, `u64`, `u128`, `usize`.

[`ExtTuple`]: crate::data::collections::ExtTuple
[`Float`]: crate::num::Float
[`Int`]: crate::num::Int


### Dependency features

- `dep_all`: enables all the optional dependencies
  - `dep_text`: enables: `const-str`, `memchr`, `unicode-segmentation`, `unicode-width`.
  - `dep_ui_term`: enables `const-str`, `crossterm`.
  - `dep_work`: enables `atomic`, `portable-atomic`.
