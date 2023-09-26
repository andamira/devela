## Features
Features from different groups (*Capability, Environment, Safety* and *Nightly*)
are mostly independent from each other.

### Capability features
- `default`: no features.
- `full`: enables full capabilities (unused).

### Environment features
By default the crate is `no_std` compatible.
- `std`: disables `no_std` compatibility and enables `std` functionality.
- `alloc`: enables `alloc` functionality.
- `no_std`: enables functionality incompatible with `std` (unused).

### Safety features
A gradient of safety. By default nothing is enabled.
- `unsafest`: enables unsafe recursively (unused).
- `unsafe`: enables unsafe features:
  - `unsafe_char`: enables unchecked conversions in [`mod@char`],
  - `unsafe_cmp`: enables const floating-point comparison in [`cmp`],
     using [`transmute`] for constant access to the bits.
  - `unsafe_convert`: enables using [`MaybeUninit`] for [`slice_into_array`]
    initialization in [`convert`].
  - `unsafe_fmt`: provides [`IntBuf`] and [`IntBufable`] in [`fmt`].
  - `unsafe_mem`: provides [`as_bytes`], [`as_bytes_mut`] and [`as_bytes_sized`]
    in [`mem`].
  - `unsafe_num`: enables `new_unchecked` constructors and implements `bytemuck`
    traits.
  - `unsafe_os`: provides functionality that depends on linux syscalls and
    implements `bytemuck` traits.
  - `unsafe_str`: enables unsafe use in [`str`][mod@str].
- `safe`: forbids unsafe at the crate level.
- `safest`: forbids unsafe recursively (unused).

### Nightly features
Currently only used for improved documentation.
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
