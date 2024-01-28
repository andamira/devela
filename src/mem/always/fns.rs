// devela::mem::always::fns
//
//!
//

#![allow(unused)]

#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
use core::{mem, slice};

/// View any `T: Sync + Unpin + ?Sized` as `&[u8]`.
///
/// This is a safer interface to [`slice::from_raw_parts`].
/// # Examples
/// ```
/// # use devela::mem::mem_as_bytes;
/// #[repr(C)]
/// struct Data(u32);
///
/// let data = Data(1234);
/// let bytes = mem_as_bytes(&data);
///
/// if cfg!(target_endian = "little") {
///     assert!(bytes == &[210, 4, 0, 0]);
/// } else {
///     assert!(bytes == &[0, 0, 4, 210]);
/// }
/// ```
#[must_use]
#[inline]
#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
pub fn mem_as_bytes<'t, T: Sync + Unpin + ?Sized + 't>(v: &T) -> &'t [u8] {
    unsafe { slice::from_raw_parts(v as *const _ as *const u8, mem::size_of_val(v)) }
}

/// View any `T: Sync + Unpin + ?Sized` as `&mut [u8]`.
///
/// This is a safer interface to [`slice::from_raw_parts_mut`].
/// # Examples
/// ```
/// # use devela::mem::mem_as_bytes_mut;
/// #[repr(C)]
/// struct Data(u32);
///
/// let mut data = Data(1234);
/// let bytes = mem_as_bytes_mut(&mut data);
///
/// if cfg!(target_endian = "little") {
///     bytes[1] = 0;
///     assert!(bytes == &[210, 0, 0, 0] && data.0 == 210);
/// } else {
///     bytes[1] = 0;
///     assert!(bytes == &[0, 0, 0, 210] && data.0 == 210);
/// }
/// ```
#[must_use]
#[inline]
#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
pub fn mem_as_bytes_mut<'t, T: Sync + Unpin + ?Sized + 't>(v: &mut T) -> &'t mut [u8] {
    unsafe { slice::from_raw_parts_mut(v as *mut _ as *mut u8, mem::size_of_val(v)) }
}

/// View any `T: Sync + Unpin + Sized` as `&[u8]` (const-compatible).
///
/// This is a safer interface to [`slice::from_raw_parts`], for `Sized` types.
/// # Examples
/// ```
/// # use devela::mem::mem_as_bytes_sized;
/// const DATA: u32 = 1234;
/// const BYTES: &[u8] = mem_as_bytes_sized(&DATA);
///
/// if cfg!(target_endian = "little") {
///     assert_eq!(BYTES, &[210, 4, 0, 0]);
/// } else {
///     assert_eq!(BYTES, &[0, 0, 4, 210]);
/// }
/// ```
#[must_use]
#[inline]
#[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
pub const fn mem_as_bytes_sized<T: Sync + Unpin>(v: &T) -> &[u8] {
    unsafe { slice::from_raw_parts(v as *const T as *const u8, mem::size_of::<T>()) }
}
