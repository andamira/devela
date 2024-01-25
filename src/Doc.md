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

By default the crate doesn't use unsafe.

They offer a gradient of safety.

- `safe`: forbids unsafe at the crate level.
- `unsafe`: enables all the unsafe sub-features:
  - `unsafe_mem`: provides [`mem_as_bytes`], [`mem_as_bytes_mut`] and [`mem_as_bytes_sized`].
  - `unsafe_num`: enables `new_unchecked` constructors, implements `bytemuck` traits,
    enables using [`MaybeUninit`] for [`slice_into_array`] initialization and
    const floating-point comparison  using [`transmute`] for constant access to the bits.
  - `unsafe_work`: provides a minimal implementation of stackless
    [coroutines][work::async::coroutine].
  - `unsafe_text`: enables use of unsafe in [`text`].


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
