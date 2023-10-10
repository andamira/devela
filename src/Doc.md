## Features

Features are grouped in 6 categories, mostly independent from each other:
- *Environment*
- *Modules*
- *Platform*
- *Dependency*
- *Safety* 
- *Nightly*

All features are disabled by default.


### Environment features

By default the crate is `no_std` compatible without allocation.

- `std`: disables `no_std` compatibility and enables `std` functionality.
- `alloc`: enables `alloc` functionality.
- `no_std`: enables functionality incompatible with `std` (unused).


### Module features

Modules can be enabled independently of *environment*, *platform* *dependency*
or *safety*, unless specified.

For example, the `_unsafe` suffix also enables the corresponding unsafe feature
for that module.

When the `depend` feature is enabled, modules will also enable their associated
optional dependencies.

- `full`, `full_unsafe`: enables all the modules, and the **platforms**.
- `fullest`: enables all the modules, recursively (unused).

Single modules: 
- `ascii`, `ascii_unsafe:` enables the [`ascii`] module.
- `char`, `char_unsafe`: enables the [`mod@char`] module.
- `cmp`, `cmp_unsafe`: enables the [`cmp`] module.
- `codegen`, `codegen_unsafe`: enables the [`codegen`] module,
  and the [`devela_macros`] optional dependency.
- `convert`, `convert_unsafe`: enables the [`convert`] module,
  and the [`az`] optional dependency.
- `fmt`, `fmt_unsafe`: enables the [`fmt`] module.
- `future`, `fmt_unsafe`: enables the [`future`] module.
- `mem`, `mem_unsafe`: enables the [`mem`] module,
  and the [`bytemuck`] optional dependency.
- `num`, `num_unsafe`: enables the [`num`] module.
- `ops`, `ops_unsafe`: enables the [`ops`] module,
- `option`, `option_unsafe`: enables the [`option`] module.
- `path`, `path_unsafe`: enables the [`path`] module.
- `result`, `result_unsafe`: enables the [`result`] module.
- `slice`, `slice_unsafe`: enables the [`mod@slice`] module.
- `str`, `str_unsafe`: enables the [`mod@str`] module,
  and the [`const-str`] optional dependency.
- `string`, `string_unsafe`: enables the [`string`] module.
  and the [`unicode-segmentation`] optional dependency.
- `sync`, `sync_unsafe`: enables the [`sync`] module,
  and the [`atomic`] and [`portable_atomic`] optional dependencies.
- `task`, `task_unsafe`: enables the [`task`] module.
- `thread`, `thread_unsafe`: enables the [`thread`] module.

Module groups:
- `async`, `async_unsafe`: enables [`future`] and [`task`] modules.
- `texts`, `texts_unsafe`: enables [`ascii`], [`mod@char`], [`fmt`], [`mod@str`]
  and [`string`] modules.


### Platform features

Platform-specific functionality is not automatically enabled since *OS* detection
depends on `std`, and we want to use it also from `no_std`.

Platform features are `os` submodules that have to be explicitly enabled:
- `linux`, `linux_unsafe`: enables [`os::linux`] functionality.
- `term`, `term_unsafe:` enables [`os::term`] functionality.


### Dependency features

- `depend`: allows modules to automatically enable their defined dependencies.

Dependencies can also be enabled individually:
- `atomic` is used in `sync`.
- `az` is used in `convert`.
- `bytemuck` is used in `mem`, `linux`, `num`.
- `devela_macros` is used in `codegen`.
- `const-str` is used in `str`, `ascii`, `option`, `os`, `linux`.
- `portable-atomic` is used in `sync`.
- `unicode-segmentation` is used in `string`.


### Safety features

By default the crate doesn't use unsafe.

They offer a gradient of safety.

- `unsafest`: enables unsafe recursively (unused).
- `unsafe`: enables all the unsafe sub-features:
  - `unsafe_ascii`: enables unchecked conversions in [`ascii`],
  - `unsafe_char`: enables unchecked conversions in [`mod@char`],
  - `unsafe_cmp`: enables const floating-point comparison in [`cmp`],
       using [`transmute`] for constant access to the bits.
  - `unsafe_codegen`: *(unused)*.
  - `unsafe_convert`: enables using [`MaybeUninit`] for [`slice_into_array`]
      initialization in [`convert`].
  - `unsafe_fmt`: provides [`IntBuf`] and [`IntBufable`] in [`fmt`].
  - `unsafe_future`: *(unused)*.
  - `unsafe_mem`: provides [`as_bytes`], [`as_bytes_mut`] and [`as_bytes_sized`]
      in [`mem`].
  - `unsafe_num`: enables `new_unchecked` constructors and implements `bytemuck` traits.
  - `unsafe_ops`: *(unused)*.
  - `unsafe_option`: *(unused)*.
  - `unsafe_os`: enables all the unsafe *platform* sub-features:
    - `unsafe_linux`: provides functionality depending on linux syscalls and
         implements `bytemuck` traits.
    - `unsafe_term`: *(unused)*.
  - `unsafe_path`: *(unused)*.
  - `unsafe_result`: *(unused)*.
  - `unsafe_slice`: *(unused)*.
  - `unsafe_str`: enables unsafe use in [`str`][mod@str].
  - `unsafe_string`: enables unsafe use in [`string`].
  - `unsafe_sync`: *(unused)*.
  - `unsafe_task`: provides a minimal implementation of stackless
    [coroutines][task::coroutine].
  - `unsafe_thread`: *(unused)*.
- `safe`: forbids unsafe at the crate level.
- `safest`: forbids unsafe recursively (unused).


### Nightly features

They are currently only used for generating improved documentation.

- `nightly`: enables nightly features.
- `nightly_docs`: enables features for docs.rs

[`IntBuf`]: fmt::IntBuf
[`IntBufable`]: fmt::IntBufAble
[`slice_into_array`]: convert::collection::slice_into_array
[`MaybeUninit`]: core::mem::MaybeUninit
[`transmute`]: core::mem::transmute
[`as_bytes`]: mem::as_bytes
[`as_bytes_mut`]: mem::as_bytes_mut
[`as_bytes_sized`]: mem::as_bytes_sized

[`atomic`]: depend::atomic
[`az`]: depend::az
[`bytemuck`]: depend::bytemuck
[`devela_macros`]: depend::devela_macros
[`const-str`]: depend::const_str
[`portable_atomic`]: depend::portable_atomic
[`unicode-segmentation`]: depend::unicode_segmentation
