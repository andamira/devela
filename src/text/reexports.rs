// devela::text::reexports
//
//! Reexport the *const-str* crate macros related to string slices,
//! prefixed with `str_` and with a new first line of documentation.
//

use crate::code::{impl_cdef, reexport};

impl_cdef!["" => &str];
// WAIT: [const_str_from_utf8_unchecked_mut](https://github.com/rust-lang/rust/issues/91005)
// #[cfg(feature = "unsafe_str")]
// #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_str")))]
// impl ConstDefault for &mut str {
//     // SAFETY: The empty string is valid UTF-8.
//     const DEFAULT: Self = unsafe { core::str::from_utf8_unchecked_mut(&mut []) };
// }
#[cfg(feature = "alloc")]
impl_cdef![Self::new() => String];

/* from other modules */

pub use crate::sys::ffi::CStr;
#[cfg(feature = "alloc")]
pub use crate::sys::ffi::CString;
#[cfg(feature = "std")]
crate::items! { pub use crate::sys::ffi::{OsStr, OsString}; }

/* core, alloc */

reexport! { rust: alloc::string,
    doc: "A UTF-8–encoded, growable string.",
    String
}
reexport! { rust: alloc::string,
    doc: "A trait for converting a value to a String.",
    ToString
}

/* const-str */

reexport! { "dep_const_str", "const-str", const_str,
    doc: "Compares two [`&str`] lexicographically.",
    @compare as str_compare
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Concatenates ([`&str`] | [`char`] | [`bool`] | `u*` | `i*`) into a [`&str`].",
    @concat as str_concat
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Concatenates ([`&str`] | [`u8`] | [`&[u8]`](slice) | [`[u8; N]`](array)
        | [`&[u8; N]`](array)) to [`&[u8; _]`](array).",
    @concat_bytes as str_concat_bytes
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Returns [`true`] if the given pattern ([`&str`] | [`char`]) matches a sub-[`&str`].",
    @contains as str_contains
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Converts a [`&str`] to [`&CStr`](core::ffi::CStr).",
    @cstr as str_cstr
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Encodes a [`&str`] with an encoding (`utf8` | `utf16`).",
    @encode as str_encode
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Encodes a [`&str`] with an encoding (`utf8` | `utf16`) and append a NUL char.",
    @encode_z as str_encode_z
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Returns `true` if the given pattern matches a suffix of this [`&str`].",
    @ends_with as str_ends_with
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Returns [`true`] if two [`&str`] are equal.",
    @equal as str_equal
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Returns a [`&str`] from a [`&[u8]`](slice).\n\n#Panics
        Panics if it's not valid utf8",
    @from_utf8 as str_from_utf8
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Converts a [`&str`] with hexadecimals (`0-9` | `A-F` | `a-f`)
        into a [`[u8; _]`](array).",
    @hex as str_hex
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Parses a [`&str`] into a value ([`&str`] | [`char`] | [`bool`] | `u*` | `i*`).",
    @parse as str_parse
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Converts a [`&str`] into a [`*const c_char`](core::ffi::c_char).",
    @raw_cstr as str_raw_cstr
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Creates a [`&str`] by repeating a [`&str`] n times.",
    @repeat as str_repeat
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Replaces all matches of a pattern ([`&str`] | [`char`]) with another [`&str`].",
    @replace as str_replace
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Sorts multiple ([`&[&str]`](slice) | [`[&str; N]`](array) |
        [`&[&str; N]`](array)) into a [`[&str; _]`](array).",
    @sorted as str_sorted
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Splits a [`&str`] by a separator pattern ([`&str`] |
        [`char`]) returning [`[&str; _]`](array).",
    @split as str_split
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Returns [`true`] if the given pattern ([`&str`] | [`char`])
        matches a prefix of [`&str`].",
    @starts_with as str_starts_with
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Returns a [`&str`] with the prefix removed.",
    @strip_prefix as str_strip_prefix
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Returns a [`&str`] with the suffix removed.",
    @strip_suffix as str_strip_suffix
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Converts a [`&str`] or [`&[u8]`](slice) into a [`[u8; _]`](array).",
    @to_byte_array as str_to_byte_array
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Converts a [`&str`] into a [`[char; _]`](array).",
    @to_char_array as str_to_char_array
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Returns a [`&str`] from a value ([`&str`] | [`char`] | [`bool`] | `u*` | `i*`).",
    @to_str as str_from
}

/* ascii related */

reexport! { "dep_const_str", "const-str", const_str,
    doc: "Returns [`true`] if all codes in this
        ([`&str`] | [`&[u8]`](slice) | [`&[u8; N]`](array)) are ASCII.",
    @is_ascii as str_is_ascii
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Converts a [`&str`] to a specified case. Non-ASCII characters are not affected.",
    @convert_ascii_case as str_convert_ascii_case
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Returns [`true`] if two [`&str`] are an ASCII *case-insensitive* match.",
    @eq_ignore_ascii_case as str_eq_ignore_ascii_case
}
reexport! { "dep_const_str", "const-str", const_str,
    doc: "Splits a [`&str`] by ASCII whitespaces, and joins the parts with a single space.",
    @squish as str_squish
}
