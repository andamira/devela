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
- [`exec`]: enables `exec` functionality.
- [`lex`]: enables `Char*`, `Egc`, `Nonul`.
- [`mix`]: enables audio, colors, images…
- [`num`]: enables all of the `num` sub-features:
    - `num_float`: `Float`, `ExtFloat`.
    - `num_geom`: `NumVector`, `Vector`, `VecVector`.
    - `num_int`: `Frac`, `Int`, `NumInt`, `NumRefInt`.
    - `num_niche_range`: `Range*` and `NonRange*` niche types.
- [`sys`]
- [`time`]
- [`ui`]: enables all of the `ui` sub-features:
    - `ui_events`: extra support for events.
    - `ui_term`: enables the terminal functionality.

[`code`]: crate::code
[`data`]: crate::data
[`exec`]: crate::exec
[`lex`]: crate::lex
[`mix`]: crate::mix
[`num`]: crate::num
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
  - `safe_mix`
  - `safe_num`
  - `safe_sys`
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

The following is a tree of features that allows fine-grained control over
for which primitive types several generic implementations will compile.
That's the case for example of [`Float`], [`Int`], or the [`niche`] types.
- `_nums:` enables all the numeric types implementations.
  - `_floats:`
    - `_f32`, `_f64`.
  - `_ints:`
    - `_sints:`
      - `_i8`, `_i16`, `_i32`, `_i64`, `_i128`, `_isize`.
    - `_uints:`
      - `_u8`, `_u16`, `_u32`, `_u64`, `_u128`, `_usize`.

[`ExtTuple`]: crate::data::collections::ExtTuple
[`Float`]: crate::num::Float
[`Int`]: crate::num::Int


### Dependency features

- `dep_all`: enables all the optional dependencies
  - `dep_exec`: enables `atomic`, `portable-atomic`.
  - `dep_lex`: enables: `const-str`, `memchr`, `unicode-segmentation`, `unicode-width`.
  - `dep_ui_term`: enables `const-str`, `crossterm`.
