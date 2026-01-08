// devela::lang::ffi::js::namespace
//
//! Defines the [`Js`] namespace.
//

use devela::{JsConsole, Str, js_int32, js_uint32};
#[cfg(all(feature = "alloc", unsafe··))]
use devela::{String, Vec};

#[doc = crate::_tags!(runtime namespace)]
/// Javascript-related operations.
#[doc = crate::_doc_location!("lang/ffi/js")]
///
/// See also: [`Web`][crate::Web].
#[derive(Debug)]
pub struct Js;

#[rustfmt::skip]
impl Js {
    /// Returns the [`JsConsole`] namespace struct.
    pub fn console() -> JsConsole { JsConsole }
}

/// Strings
impl Js {
    /// Reads a JS string into a Rust `&str` backed by the given `buffer`,
    /// truncating if the buffer is too small.
    ///
    /// # Panic
    /// Panics if the result is not valid UTF-8.
    #[inline(always)]
    pub fn read_str(
        buffer: &mut [u8],
        mut write_fn: impl FnMut(*mut u8, js_uint32) -> js_int32,
    ) -> &str {
        let ptr = buffer.as_mut_ptr();
        let cap = buffer.len() as js_uint32;
        let len = write_fn(ptr, cap) as usize;
        Str::from_utf8(&buffer[..len]).expect("Valid UTF-8") // IMPROVE
    }

    /// Allocates a `String` by calling a JS-backed FFI fn that writes a UTF-8 string into WASM memory.
    /// Allocates a new `String` from JS (auto-sizing).
    ///
    /// - Uses a dynamic buffer, starting with 128 bytes of `capacity`.
    /// - Retries with exact required capacity if truncation is detected.
    #[inline(always)]
    #[cfg(not(feature = "safe_lang"))]
    #[cfg(all(feature = "alloc", unsafe··))]
    #[cfg_attr(nightly_doc, doc(cfg(all(feature = "alloc", unsafe··))))]
    pub fn read_string(write_fn: impl FnMut(*mut u8, js_uint32) -> js_int32) -> String {
        Js::read_string_capped(128, false, write_fn)
    }

    /// Allocates a `String` by calling a JS-backed FFI fn that writes a UTF-8 string into WASM memory.
    ///
    /// - Uses a dynamic buffer, starting with the given `capacity`.
    /// - Retries with exact required capacity if truncation is detected (unless `truncate = true`).
    /// - Assumes the FFI fn returns `js_int32`: positive = bytes written, negative = required size.
    #[cfg(not(feature = "safe_lang"))]
    #[cfg(all(feature = "alloc", unsafe··))]
    #[cfg_attr(nightly_doc, doc(cfg(all(feature = "alloc", unsafe··))))]
    pub fn read_string_capped(
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
}
