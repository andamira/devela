## Features

Features are grouped in 5 categories, mostly independent from each other:
- *Environment*
- *Module*
- *Safety*
- *Nightly*
- *Dependency*

All features are disabled by default.


### Environment features

By default the crate is `no_std` compatible without allocation.

- `std`: disables `no_std` compatibility and enables `std` functionality.
- `alloc`: enables `alloc` functionality.
- `no_std`: enables functionality incompatible with `std`:
  - enables the `libm` optional dependency.


### Module features

Modules can be enabled independently of *environment*, *dependency* or *safety*.

When the `dep` feature is enabled, modules will also enable their associated
optional dependencies.

- `all`: enables all the root modules:

Single modules:
- `code`: enables the [`code`] module,
- `data`: enables the [`data`] module,
- `gfx`: enables the [`gfx`] module.
- `io`: enables the [`io`] module.
- `mem`: enables the [`mem`] module,
  and the [`bytemuck`] optional dependency.
  and the [`devela_macros`] optional dependency.
- `num`: enables the [`num`] module.
- `os`: enables all the [`os`] specific functionality.
- `result`: enables the [`result`] module.
- `text`: enables the [`text`] module,
  and the [`const-str`], [`unicode-segmentation`] and [`unicode-width`] optional dependencies.
  and the [`atomic`] and [`portable_atomic`] optional dependencies.
- `time`: enables the [`time`] module.
- `ui`: enables the [`ui`] module.
  - `ui_term`: enables the terminal functionality,
    and the [`const-str`] optional dependency.
- `work`: enables the [`work`] module.


### Safety features

In order to use any unsafe functionality:
1. enable the corresponding `unsafe` feature.
2. don't enable `safe` feature in the module.

- `safe`: forbids `unsafe` (and overrides unsafe features)
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

- `unsafe`: enables unsafe (as long as it isn't forbidden in the module)
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
  - `nightly_coro`: enables `coroutines`, `coroutine_trait`.
  - `nightly_doc`: enables `doc_cfg`.

- `docsrs`: enables compiling the complete documentation for [docs.rs](https://docs.rs).


### Dependency features

- `dep`: allows modules to automatically enable their defined dependencies.

Dependencies can also be enabled individually:
- `atomic` is used in `work`.
- `bytemuck` is used in `mem`, `num`.
- `const-str` is used in `result`, `text`, `ui_term`.
- `devela_macros` is used in `meta`.
- `libm` is used in `num`.
- `portable-atomic` is used in `work`.
- `unicode-segmentation` is used in `text`.
- `unicode-width` is used in `text`.

[`slice_into_array`]: data::convert::collection::slice_into_array
[`MaybeUninit`]: core::mem::MaybeUninit
[`transmute`]: core::mem::transmute
[`mem_as_bytes`]: mem::mem_as_bytes
[`mem_as_bytes_mut`]: mem::mem_as_bytes_mut
[`mem_as_bytes_sized`]: mem::mem_as_bytes_sized

[`atomic`]: dep::atomic
[`bytemuck`]: dep::bytemuck
[`devela_macros`]: dep::devela_macros
[`const-str`]: dep::const_str
[`portable_atomic`]: dep::portable_atomic
[`unicode-segmentation`]: dep::unicode_segmentation
[`unicode-width`]: dep::unicode_width
