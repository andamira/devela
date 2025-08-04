// devela::lang::ffi::js::value
//
//! Defines [`JsValue`].
//
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Numbers_and_strings

#[cfg(feature = "alloc")]
use devela::String;
use devela::{js_bool, js_number, js_uint32};

/// A JavaScript value for FFI communication.
///
/// # Variants
/// - `Null`: JS `null`
/// - `Undefined`: JS `undefined`  
/// - `Boolean(bool)`: JS boolean (`true`/`false`)
/// - `Object(u32)`: Handle to JS object (registry index)
/// - `Str(&'static str)`: Static string (no allocation)
/// - `String(String)`: Heap-allocated string (requires `alloc` feature)
/// - `Number(f64)`: JS number (always 64-bit float)
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum JsValue {
    /* misc. */
    Null,
    Undefined,
    Boolean(js_bool),
    Object(js_uint32),

    /* strings */
    Str(&'static str),
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    String(String),

    /* numbers */
    Number(js_number),
}

impl JsValue {
    /// Returns the number tag associated with the current variant.
    #[inline(always)]
    pub const fn type_tag(&self) -> u8 {
        match self {
            Self::Null => 0,
            Self::Undefined => 1,
            Self::Boolean(_) => 2,
            Self::Object(_) => 3,
            //
            Self::Str(_) => 4,
            #[cfg(feature = "alloc")]
            Self::String(_) => 5,
            //
            Self::Number(_) => 6,
        }
    }

    /// Encodes the value into a byte buffer for WASM FFI.
    ///
    /// Returns the total bytes written.
    ///
    /// # Layout
    /// ```text
    /// [1B: type tag][payload...]
    /// ```
    /// # Panics
    /// If buffer is too small (debug builds only).
    ///
    pub fn encode_into(&self, buf: &mut [u8]) -> usize {
        let mut offset = 0;
        buf[offset] = self.type_tag(); // 1 Byte
        offset += 1;

        match self {
            Self::Boolean(b) => {
                buf[offset] = *b as u8;
                offset += 1;
            }
            Self::Object(handle) => {
                buf[offset..offset + 4].copy_from_slice(&handle.to_le_bytes());
                offset += 4;
            }
            Self::Str(s) => {
                offset += self.encode_str(s, &mut buf[offset..]);
            }
            #[cfg(feature = "alloc")]
            Self::String(s) => {
                offset += self.encode_str(s.as_str(), &mut buf[offset..]);
            }
            Self::Number(n) => {
                buf[offset..offset + 8].copy_from_slice(&n.to_le_bytes());
                offset += 8;
            }
            Self::Null | Self::Undefined => (),
        }
        offset
    }

    #[inline(always)]
    fn encode_str(&self, s: &str, buf: &mut [u8]) -> usize {
        let (bytes, len) = (s.as_bytes(), s.len());
        buf[..4].copy_from_slice(&(len as u32).to_le_bytes()); // encode length prefix
        buf[4..4 + len].copy_from_slice(bytes); // encode string byte content
        4 + len // return total bytes written
    }
}
