## Features

Features are grouped in the following categories:
- *Miscellaneous*
- *Environment*
- *Module*
- *Safety*
- *Nightly*
- *Dependency*

All features are disabled by default.

Features from different categories are designed to be mostly independent from
each other, and composable, except from the miscellaneous features.

### Miscellaneous features

- `docsrs`: enables the most complete version of the documentation for [docs.rs](https://docs.rs).
  - Enables: `std`, `all`, `unsafe`, `nightly`, `dep`, `libm`.


### Environment features

By default the crate is `no_std` compatible without allocation.

- `std`: disables `no_std` compatibility and enables `std` functionality.
- `alloc`: enables `alloc` functionality.
- `no_std`: enables functionality incompatible with or substitute of `std`:
  - enables the `libm` optional dependency.


### Module features

Modules can be enabled independently of *environment*, *dependency* or *safety*.

When the `dep` feature is enabled, modules will also enable their associated
optional dependencies.

- `all`: enables all the root modules and extra submodules:

Single modules:
- `code`: enables the [`code`] module,
- `data`: enables the [`data`] module,
- `gfx`: enables the [`gfx`] module.
- `io`: enables the [`io`] module.
- `mem`: enables the [`mem`] module,
  and the [`bytemuck`] optional dependency.
- `num`: enables the [`num`] module.
  - `num_all`: enables all the `num` extra submodules:
    - `num_niche_impls`: extra impls for [`niche`] types.
- `os`: enables all the [`os`] specific functionality.
- `result`: enables the [`result`] module.
- `text`: enables the [`text`] module,
  and the [`const-str`], [`unicode-segmentation`] and [`unicode-width`] optional dependencies.
  and the [`atomic`] and [`portable_atomic`] optional dependencies.
- `time`: enables the [`time`] module.
- `ui`: enables the [`ui`] module.
  - `ui_all`: enables all the `ui` extra submodules:
    - `ui_term`: enables the terminal functionality,
    and the [`const-str`] optional dependency.
- `work`: enables the [`work`] module.

[`code`]: crate::code
[`data`]: crate::data
[`gfx`]: crate::gfx
[`io`]: crate::io
[`mem`]: crate::mem
[`num`]: crate::num
[`niche`]: crate::num::niche
[`os`]: crate::os
[`result`]: crate::result
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
  - `safe_gfx`
  - `safe_io`
  - `safe_mem`
  - `safe_num`
  - `safe_os`
  - `safe_result`
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


### Dependency features

- `dep`: allows modules to automatically enable their defined dependencies.

Dependencies can also be enabled individually:
- `atomic` is used in `work`.
- `bytemuck` is used in `mem`, `num`.
- `const-str` is used in `result`, `text`, `ui_term`.
- `libm` is used in `num`.
- `hashbrown` is used in `data`.
- `portable-atomic` is used in `work`.
- `unicode-segmentation` is used in `text`.
- `unicode-width` is used in `text`.

[`atomic`]: dep::atomic
[`bytemuck`]: dep::bytemuck
[`const-str`]: dep::const_str
[`portable_atomic`]: dep::portable_atomic
[`unicode-segmentation`]: dep::unicode_segmentation
[`unicode-width`]: dep::unicode_width
