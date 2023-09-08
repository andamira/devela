## Features
Features from different groups (*Capability, Environment, Safety* and *Nightly*)
are independent from each other.

### Capability features
- `default`: no features.
- `full`: enables optional capabilities:
  - `bytemuck`: implements several [`bytemuck`] traits for [`num`] types
    if the `unsafe_num` feature is also enabled.

### Environment features
By default the crate is `no_std` compatible.
- `std`: disables `no_std` compatibility and enables `std` functionality.
- `alloc`: enables `alloc` functionality.
- `no_std`: enables functionality incompatible with `std` (unused).

### Safety features
A gradient of safety. By default nothing is enabled.
- `unsafest`: enables unsafe recursively (unused).
- `unsafe`: enables unsafe features:
  - `unsafe_cmp`: enables const floating-point comparison in [`cmp`],
     using [`transmute`] for constant access to the bits.
  - `unsafe_convert`: enables using [`MaybeUninit`] for [`slice_into_array`]
    initialization in [`convert`].
  - `unsafe_fmt`: provides [`IntBuf`] and [`IntBufable`] in [`fmt`].
  - `unsafe_num`: enables `new_unchecked` constructors and implements
    [`bytemuck`] traits for types defined in [`num`].
  - `unsafe_os`: provides functionality that depends on linux syscalls.
- `safe`: forbids unsafe at the crate level.
- `safest`: forbids unsafe recursively (unused).

### Nightly features
Currently only used for improved documentation.
- `nightly`: enables nightly features.
- `nightly_docs`: enables features for docs.rs

[`bytemuck`]: mem::bytemuck
[`IntBuf`]: fmt::IntBuf
[`IntBufable`]: fmt::IntBufAble
[`slice_into_array`]: convert::collection::slice_into_array
[`MaybeUninit`]: core::mem::MaybeUninit
[`transmute`]: core::mem::transmute
