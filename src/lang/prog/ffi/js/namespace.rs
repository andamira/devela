// devela/src/lang/prog/ffi/js/namespace.rs
//
//! Defines the [`Js`] namespace.
//

use devela::{JsConsole, Str, js_int32, js_uint32};
#[cfg(feature = "alloc")]
use devela::{String, Vec};

#[doc = crate::_tags!(runtime namespace)]
/// Javascript-related operations.
#[doc = crate::_doc_meta!{location("lang/prog/ffi/js")}]
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
    /// Reads a JS string into a Rust `&str` backed by `buffer`.
    ///
    /// If the buffer is too small, returns the longest complete UTF-8 prefix.
    ///
    /// # Panics
    /// Panics if the writer violates its length contract or produces invalid
    /// UTF-8 other than an incomplete final code point caused by truncation.
    // FUTURE IMPROVE add unchecked specialization
    #[inline(always)]
    pub fn read_str(
        buffer: &mut [u8],
        mut write_fn: impl FnMut(*mut u8, js_uint32) -> js_int32,
    ) -> &str {
        let cap = js_uint32::try_from(buffer.len()).expect("JS string buffer exceeds js_uint32");
        let result = write_fn(buffer.as_mut_ptr(), cap);
        if result < 0 {
            let required = result.unsigned_abs();
            assert!(required > cap, "JS writer returned an invalid required size");
            Str::from_utf8_complete_prefix(buffer).expect("JS writer returned invalid UTF-8")
        } else {
            let written = result as usize;
            assert!(
                written <= buffer.len(),
                "JS writer returned more bytes than the provided capacity"
            );
            Str::from_utf8(&buffer[..written]).expect("JS writer returned invalid UTF-8")
        }
    }

    /// Allocates a `String` by calling a JS-backed FFI function
    /// that writes a UTF-8 string into WASM memory.
    ///
    /// - Uses a dynamic buffer, starting with 128 bytes of `capacity`.
    /// - Retries with exact required capacity if truncation is detected.
    ///
    /// # Features
    /// - `unsafe_ffi` enables writing directly into uninitialized allocation capacity.
    /// - `unsafe_str` enables unchecked UTF-8 conversion.
    #[inline(always)]
    #[cfg(feature = "alloc")]
    pub fn read_string(write_fn: impl FnMut(*mut u8, js_uint32) -> js_int32) -> String {
        Js::read_string_capped(128, false, write_fn)
    }

    /// Allocates a `String` by calling a JS-backed FFI function
    /// that writes UTF-8 bytes into WASM memory.
    ///
    /// A non-negative result is the number of bytes written.
    ///
    /// A negative result is the negated required size. In that case the writer
    /// must still initialize the entire provided buffer with the leading bytes
    /// of the encoded string.
    ///
    /// # Features
    /// - `unsafe_ffi` enables writing directly into uninitialized allocation capacity.
    /// - `unsafe_str` enables unchecked UTF-8 conversion.
    #[cfg(feature = "alloc")]
    pub fn read_string_capped(
        mut cap: js_uint32,
        truncate: bool,
        mut write_fn: impl FnMut(*mut u8, js_uint32) -> js_int32,
    ) -> String {
        loop {
            let capacity = cap as usize;
            let mut bytes = Vec::with_capacity(capacity);
            cfg_select! {
                all(feature = "unsafe_ffi", not(feature = "safe_lang")) => {}
                _ => bytes.resize(capacity, 0), // give the foreign writer initialized storage
            }
            let result = write_fn(bytes.as_mut_ptr(), cap);
            let truncated = result < 0;
            if truncated {
                let required = result.unsigned_abs();
                assert!(required > cap, "JS writer returned an invalid required size");
                if !truncate {
                    cap = required;
                    continue;
                }
            }
            // The writer contract says the entire buffer
            // was initialized with the leading encoded bytes.
            let written = if truncated {
                capacity
            } else {
                let written = result as usize;
                assert!(
                    written <= capacity,
                    "JS writer returned more bytes than the provided capacity"
                );
                written
            };
            cfg_select! {
                all(feature = "unsafe_ffi", not(feature = "safe_lang")) => {
                    // SAFETY: `written <= capacity` && the JS writer contract
                    // guarantees that every byte in `0..written` was initialized.
                    unsafe { bytes.set_len(written); }
                }
                _ => bytes.truncate(written),
            }
            if truncated {
                let valid_len = Str::from_utf8_complete_prefix(&bytes)
                    .expect("JS writer returned invalid UTF-8")
                    .len();
                bytes.truncate(valid_len);
                // This branch has already validated the relevant bytes.
                // Revalidation is the safe_lang-compatible finishing path.
                return String::from_utf8(bytes).expect("validated UTF-8 prefix");
            }
            cfg_select! {
                all(feature = "unsafe_str", not(feature = "safe_lang")) => {
                    // SAFETY: the JS writer contract guarantees valid UTF-8.
                    return unsafe { String::from_utf8_unchecked(bytes) };
                }
                _ => {
                    return String::from_utf8(bytes).expect("JS writer returned invalid UTF-8");
                }
            }
        }
    }
}
