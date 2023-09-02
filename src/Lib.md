
# Features

- `std` (default): enables functionality that depends on the standard library.
  Disabling it makes the crate `no_std` compatible.
- `alloc`: enables functionality that depends on allocation. Included in `std`.
- `no_std`: enables functionality incompatible with `std` (unused).
---
- `safe`: forbids unsafe at the crate level.
- `safest`: forbids unsafe recursively (unused).
- `unsafe`: enables all the unsafe features:
  - `unsafe_cmp`: enables const floating-point comparison in [`cmp`],
     using [`transmute`] for constant access to the bits.
  - `unsafe_convert`: enables using [`MaybeUninit`] for [`slice_into_array`]
    initialization in [`convert`].
  - `unsafe_fmt`: provides [`IntBuf`] and [`IntBufable`] in [`fmt`].
  - `unsafe_num`: enables `new_unchecked` and implements
    [`bytemuck`] traits for new types defined in [`num`].
  - `unsafe_os`: provides functionality that depends on linux syscalls.
- `unsafest`: enables unsafe recursively (unused).
---
- `bytemuck`: implements several unsafe [`bytemuck`] traits for [`num`] types.

[`IntBuf`]: fmt::IntBuf
[`IntBufable`]: fmt::IntBufAble
[`slice_into_array`]: convert::collection::slice_into_array
[`MaybeUninit`]: core::mem::MaybeUninit
[`transmute`]: core::mem::transmute
