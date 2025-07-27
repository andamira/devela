// devela::lang::ffi::js::str
//
//! Defines the internal fns: [`js_str`], [`js_string`], [`js_string_with_capacity`].
//
// IMPROVE: TODO namespace

use devela::{Str, js_int32, js_uint32};
#[cfg(feature = "alloc")]
use devela::{String, Vec};

/// Calls a JS-backed FFI function to fill the given buffer, returning a valid UTF-8 `&str`.
///
/// - Truncates if the buffer is too small.
/// - Panics if the result is not valid UTF-8.
#[inline(always)]
pub(crate) fn js_str(
    buffer: &mut [u8],
    mut write_fn: impl FnMut(*mut u8, js_uint32) -> js_int32,
) -> &str {
    let ptr = buffer.as_mut_ptr();
    let cap = buffer.len() as js_uint32;
    let len = write_fn(ptr, cap) as usize;
    Str::from_utf8(&buffer[..len]).expect("Valid UTF-8") // IMPROVE
}

/// Allocates a `String` by calling a JS-backed FFI fn that writes a UTF-8 string into WASM memory.
///
/// - Uses a dynamic buffer, starting with 128 bytes of `capacity`.
/// - Retries with exact required capacity if truncation is detected.
#[cfg(all(feature = "alloc", unsafe··))]
pub(crate) fn js_string(write_fn: impl FnMut(*mut u8, js_uint32) -> js_int32) -> String {
    js_string_with_capacity(128, false, write_fn)
}

/// Allocates a `String` by calling a JS-backed FFI fn that writes a UTF-8 string into WASM memory.
///
/// - Uses a dynamic buffer, starting with the given `capacity`.
/// - Retries with exact required capacity if truncation is detected (unless `truncate = true`).
/// - Assumes the FFI fn returns `js_int32`: positive = bytes written, negative = required size.
#[cfg(all(feature = "alloc", unsafe··))]
pub(crate) fn js_string_with_capacity(
    mut cap: js_uint32,
    truncate: bool,
    mut write_fn: impl FnMut(*mut u8, js_uint32) -> js_int32,
) -> String {
    loop {
        let mut vec = Vec::with_capacity(cap as usize);
        let ptr = vec.as_mut_ptr();
        let result = write_fn(ptr, cap);
        if !truncate && result < 0 {
            cap = (-result) as js_uint32;
            continue;
        }
        unsafe {
            vec.set_len(result as usize);
            return String::from_utf8_unchecked(vec);
        }
    }
}
