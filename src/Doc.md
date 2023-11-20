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

Modules can be enabled independently of *environment*, *dependency* or *safety*,
unless specified.

For example, the `_unsafe` suffix also enables the corresponding unsafe feature
for that module.

When the `depend` feature is enabled, modules will also enable their associated
optional dependencies.

- `full`, `full_unsafe`: enables all the modules.
- `fullest`: enables all the modules, recursively (unused).

Single modules: 
- `any`, `any_unsafe:` enables the [`any`] module.
- `mem`, `mem_unsafe`: enables the [`mem`] module,
  and the [`bytemuck`] optional dependency.
- `meta`, `meta_unsafe`: enables the [`meta`] module,
  and the [`devela_macros`] optional dependency.
- `num`, `num_unsafe`: enables the [`num`] module.
- `path`, `path_unsafe`: enables the [`path`] module.
- `result`, `result_unsafe`: enables the [`result`] module.
- `text`, `text_unsafe`: enables the [`text`] module.
  and the [`const-str`] and [`unicode-segmentation`] optional dependencies.
  and the [`atomic`] and [`portable_atomic`] optional dependencies.
- `task`, `task_unsafe`: enables the [`task`] module.
- `time`, `time_unsafe`: enables the [`time`] module.


### Safety features

By default the crate doesn't use unsafe.

They offer a gradient of safety.

- `unsafest`: enables unsafe recursively (unused).
- `unsafe`: enables all the unsafe sub-features:
  - `unsafe_any`: *(unused)*.
  - `unsafe_mem`: provides [`mem_as_bytes`], [`mem_as_bytes_mut`] and [`mem_as_bytes_sized`].
  - `unsafe_meta`: *(unused)*.
  - `unsafe_num`: enables `new_unchecked` constructors, implements `bytemuck` traits,
    enables using [`MaybeUninit`] for [`slice_into_array`] initialization and
    const floating-point comparison  using [`transmute`] for constant access to the bits.
  - `unsafe_path`: *(unused)*.
  - `unsafe_result`: *(unused)*.
  - `unsafe_text`: enables use of unsafe in [`text`].
  - `unsafe_task`: provides a minimal implementation of stackless
    [coroutines][task::async::coroutine].
  - `unsafe_time`: *(unused)*.
- `safe`: forbids unsafe at the crate level.
- `safest`: forbids unsafe recursively (unused).


### Nightly features

They are currently only used for generating improved documentation.

- `nightly`: enables nightly features.


### Dependency features

- `dep`: allows modules to automatically enable their defined dependencies.

Dependencies can also be enabled individually:
- `atomic` is used in `task`.
- `bytemuck` is used in `mem`, `num`.
- `const-str` is used in `text`, `result`.
- `devela_macros` is used in `meta`.
- `libm` is used in `color`, `num`.
- `portable-atomic` is used in `task`.
- `unicode-segmentation` is used in `text`.

[`IntBuf`]: text::IntBuf
[`IntBufable`]: text::IntBufAble
[`slice_into_array`]: num::convert::collection::slice_into_array
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
