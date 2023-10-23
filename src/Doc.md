## Features

Features are grouped in 6 categories, mostly independent from each other:
- *Environment*
- *Platform*
- *Module*
- *Safety* 
- *Nightly*
- *Dependency*

All features are disabled by default.


### Environment features

By default the crate is `no_std` compatible without allocation.

- `std`: disables `no_std` compatibility and enables `std` functionality.
- `alloc`: enables `alloc` functionality.
- `no_std`: enables functionality incompatible with `std` (unused).


### Platform specific features

Platform-specific functionality is not automatically enabled since *OS* detection
depends on `std`, and we want to use it from `no_std`.

Platform features are `os` submodules that have to be explicitly enabled:
- `linux`, `linux_unsafe`: enables [`os::linux`] functionality.
- `term`, `term_unsafe:` enables [`os::term`] functionality.


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
- `any`, `any_unsafe:` enables the [`any`] module.
- `cmp`, `cmp_unsafe`: enables the [`cmp`] module.
- `codegen`, `codegen_unsafe`: enables the [`codegen`] module,
  and the [`devela_macros`] optional dependency.
- `convert`, `convert_unsafe`: enables the [`convert`] module,
  and the [`az`] optional dependency.
- `future`, `future_unsafe`: enables the [`future`] module.
- `mem`, `mem_unsafe`: enables the [`mem`] module,
  and the [`bytemuck`] optional dependency.
- `num`, `num_unsafe`: enables the [`num`] module.
- `ops`, `ops_unsafe`: enables the [`ops`] module,
- `option`, `option_unsafe`: enables the [`option`] module.
- `path`, `path_unsafe`: enables the [`path`] module.
- `result`, `result_unsafe`: enables the [`result`] module.
- `slice`, `slice_unsafe`: enables the [`mod@slice`] module.
- `string`, `string_unsafe`: enables the [`string`] module.
  and the [`const-str`] and [`unicode-segmentation`] optional dependencies.
- `sync`, `sync_unsafe`: enables the [`sync`] module,
  and the [`atomic`] and [`portable_atomic`] optional dependencies.
- `task`, `task_unsafe`: enables the [`task`] module.
- `thread`, `thread_unsafe`: enables the [`thread`] module.
- `time`, `time_unsafe`: enables the [`time`] module.

Module groups:
- `async`, `async_unsafe`: enables [`future`] and [`task`] modules.


### Safety features

By default the crate doesn't use unsafe.

They offer a gradient of safety.

- `unsafest`: enables unsafe recursively (unused).
- `unsafe`: enables all the unsafe sub-features:
  - `unsafe_any`: *(unused)*.
  - `unsafe_cmp`: enables const floating-point comparison in [`cmp`],
       using [`transmute`] for constant access to the bits.
  - `unsafe_codegen`: *(unused)*.
  - `unsafe_convert`: enables using [`MaybeUninit`] for [`slice_into_array`]
      initialization in [`convert`].
  - `unsafe_future`: *(unused)*.
  - `unsafe_mem`: provides [`mem_as_bytes`], [`mem_as_bytes_mut`] and
    [`mem_as_bytes_sized`] in [`mem`].
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
  - `unsafe_string`: enables unsafe use in [`string`].
  - `unsafe_sync`: *(unused)*.
  - `unsafe_task`: provides a minimal implementation of stackless
    [coroutines][task::coroutine].
  - `unsafe_thread`: *(unused)*.
  - `unsafe_time`: *(unused)*.
- `safe`: forbids unsafe at the crate level.
- `safest`: forbids unsafe recursively (unused).


### Nightly features

They are currently only used for generating improved documentation.

- `nightly`: enables nightly features.
- `nightly_docs`: enables features for docs.rs


### Dependency features

- `depend`: allows modules to automatically enable their defined dependencies.

Dependencies can also be enabled individually:
- `atomic` is used in `sync`.
- `az` is used in `convert`.
- `bytemuck` is used in `mem`, `linux`, `num`.
- `devela_macros` is used in `codegen`.
- `const-str` is used in `string`, `option`, `os`, `linux`.
- `portable-atomic` is used in `sync`.
- `unicode-segmentation` is used in `string`.

[`IntBuf`]: string::IntBuf
[`IntBufable`]: string::IntBufAble
[`slice_into_array`]: convert::collection::slice_into_array
[`MaybeUninit`]: core::mem::MaybeUninit
[`transmute`]: core::mem::transmute
[`mem_as_bytes`]: mem::mem_as_bytes
[`mem_as_bytes_mut`]: mem::mem_as_bytes_mut
[`mem_as_bytes_sized`]: mem::mem_as_bytes_sized

[`atomic`]: depend::atomic
[`az`]: depend::az
[`bytemuck`]: depend::bytemuck
[`devela_macros`]: depend::devela_macros
[`const-str`]: depend::const_str
[`portable_atomic`]: depend::portable_atomic
[`unicode-segmentation`]: depend::unicode_segmentation
