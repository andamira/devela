## Features

Features are grouped in 6 categories, mostly independent from each other:
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

- `full`: enables every root module:

Single modules:
- `code`: enables the [`code`] module,
- `error`: enables the [`error`] module.
- `io`: enables the [`io`] module.
- `mem`: enables the [`mem`] module,
  and the [`bytemuck`] optional dependency.
  and the [`devela_macros`] optional dependency.
- `num`: enables the [`num`] module.
- `os`: enables all the [`os`] specific functionality.
- `render`: enables the [`render`] module.
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
  - `safe_data`
  - `safe_error`
  - `safe_mem`
  - `safe_num`
  - `safe_os`
  - `safe_text`
  - `safe_ui`
  - `safe_work`

- `unsafe`: enables unsafe (as long as it isn't forbidden in the module)
	- `unsafe_array` enables: faster array initialization.
	- `unsafe_async` enables: custom task waker, coroutine impls.
	- `unsafe_const` enables: extra const methods.
	- `unsafe_dyn` enables: DSTs in the stack, `no_std` Error dyn impls.
	- `unsafe_niche` enables: unchecked niche constructors.
	- `unsafe_slice` enables: extra slice methods, avoid bound checks.
	- `unsafe_str` enables: unchecked utf-8 char and &str conversions.


### Nightly features

- `nightly`: enables nightly features.
- `docsrs`: enables compiling the complete documentation.


### Dependency features

- `dep`: allows modules to automatically enable their defined dependencies.

Dependencies can also be enabled individually:
- `atomic` is used in `work`.
- `bytemuck` is used in `mem`, `num`.
- `const-str` is used in `error`, `text`, `ui_term`.
- `devela_macros` is used in `meta`.
- `libm` is used in `num`.
- `portable-atomic` is used in `work`.
- `unicode-segmentation` is used in `text`.
- `unicode-width` is used in `text`.

[`IntBuf`]: text::IntBuf
[`IntBufable`]: text::IntBufAble
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
