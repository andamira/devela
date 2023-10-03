// devela::mem::always_fns
//
//! `mem` standalone functions.
//!
//! Always available for internal use
//

#![allow(unused)]

#[cfg(feature = "unsafe_mem")]
use core::{mem, slice};

/// View any `T: Sync + Unpin + ?Sized` as `&[u8]`.
///
/// This is a safer interface to [`slice::from_raw_parts`].
///
/// # Examples
/// ```
/// use devela::mem::as_bytes;
///
/// #[repr(C)]
/// struct Data(u32);
///
/// let data = Data(1234);
/// let bytes = as_bytes(&data);
///
/// if cfg!(target_endian = "little") {
///     assert!(bytes == &[210, 4, 0, 0]);
/// } else {
///     assert!(bytes == &[0, 0, 4, 210]);
/// }
/// ```
#[inline(always)]
#[must_use]
#[cfg(feature = "unsafe_mem")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_mem")))]
pub fn as_bytes<'t, T: Sync + Unpin + ?Sized + 't>(v: &T) -> &'t [u8] {
    unsafe { slice::from_raw_parts(v as *const _ as *const u8, mem::size_of_val(v)) }
}

/// View any `T: Sync + Unpin + ?Sized` as `&mut [u8]`.
///
/// This is a safer interface to [`slice::from_raw_parts_mut`].
///
/// # Examples
/// ```
/// use devela::mem::as_bytes_mut;
///
/// #[repr(C)]
/// struct Data(u32);
///
/// let mut data = Data(1234);
/// let bytes = as_bytes_mut(&mut data);
///
/// if cfg!(target_endian = "little") {
///     bytes[1] = 0;
///     assert!(bytes == &[210, 0, 0, 0] && data.0 == 210);
/// } else {
///     bytes[1] = 0;
///     assert!(bytes == &[0, 0, 0, 210] && data.0 == 210);
/// }
/// ```
#[inline(always)]
#[must_use]
#[cfg(feature = "unsafe_mem")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_mem")))]
pub fn as_bytes_mut<'t, T: Sync + Unpin + ?Sized + 't>(v: &mut T) -> &'t mut [u8] {
    unsafe { slice::from_raw_parts_mut(v as *mut _ as *mut u8, mem::size_of_val(v)) }
}

/// View any `T: Sync + Unpin + Sized` as `&[u8]` (const-compatible).
///
/// This is a safer interface to [`slice::from_raw_parts`], for `Sized` types.
///
/// # Examples
/// ```
/// use devela::mem::as_bytes_sized;
///
/// const DATA: u32 = 1234;
/// const BYTES: &[u8] = as_bytes_sized(&DATA);
///
/// if cfg!(target_endian = "little") {
///     assert_eq!(BYTES, &[210, 4, 0, 0]);
/// } else {
///     assert_eq!(BYTES, &[0, 0, 4, 210]);
/// }
/// ```
#[inline(always)]
#[must_use]
#[cfg(feature = "unsafe_mem")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe_mem")))]
pub const fn as_bytes_sized<T: Sync + Unpin>(v: &T) -> &[u8] {
    unsafe { slice::from_raw_parts(v as *const T as *const u8, mem::size_of::<T>()) }
}
