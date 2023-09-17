// devela::str::reexport_const_str
//
//! Reexport the `const-str` crate macros related to string slices,
//! prefixed with `str_` and with a new first line of documentation.
//

#[doc = "Compares two [`&str`] lexicographically.\n\n---"]
pub use const_str::compare as str_compare;

#[doc = "Concatenates values ([`&str`] | [`char`] | [`bool`] | `u*` | `i*`)"]
#[doc = " into a [`&str`].\n\n---"]
pub use const_str::concat as str_concat;

#[doc = "Concatenates values ([`&str`] | [`u8`] | [`&[u8]`](slice) | [`[u8; N]`](array)"]
#[doc = " | [`&[u8; N]`](array)) into a [`&[u8; _]`](array).\n\n---"]
pub use const_str::concat_bytes as str_concat_bytes;

#[doc = "Returns [`true`] if the given pattern ([`&str`] | [`char`])"]
#[doc = " matches a sub-slice of this `&str`.\n\n---"]
pub use const_str::contains as str_contains;

#[doc = "Converts a [`&str`] to [`&CStr`](core::ffi::CStr).\n\n---"]
pub use const_str::cstr as str_cstr;

#[doc = "Encodes a [`&str`] with the specified (`utf8` | `utf16`) encoding.\n\n---"]
pub use const_str::encode as str_encode;

#[doc = "Encodes a [`&str`] with the specified (`utf8` | `utf16`) encoding"]
#[doc = " and append a nul character.\n\n---"]
pub use const_str::encode_z as str_encode_z;

#[doc = "Returns [`true`] if the given pattern ([`&str`] | [`char`])"]
#[doc = " matches a suffix of this [`&str`].\n\n---"]
pub use const_str::ends_with as str_ends_with;

#[doc = "Returns [`true`] if two [`&str`] are equal.\n\n---"]
pub use const_str::equal as str_equal;

#[doc = "Returns a [`&str`] from a [`&[u8]`](slice).\n\n"]
#[doc = "#Panics\nPanics if it's not valid utf8.\n\n---"]
pub use const_str::from_utf8 as str_from_utf8;

#[doc = "Converts a [`&str`] with hexadecimals (`0-9` | `A-F` | `a-f`)"]
#[doc = " into a [`[u8; _]`](array).\n\n---"]
pub use const_str::hex as str_hex;

#[doc = "Parses a [`&str`] into a value ([`&str`] | [`char`] | [`bool`] | `u*` | `i*`).\n\n---"]
pub use const_str::parse as str_parse;

#[doc = "Converts a [`&str`] into a [`*const c_char`](core::ffi::c_char).\n\n---"]
pub use const_str::raw_cstr as str_raw_cstr;

#[doc = "Creates a new [`&str`] by repeating a [`&str`] n times.\n\n---"]
pub use const_str::repeat as str_repeat;

#[doc = "Replaces all matches of a pattern ([`&str`] | [`char`]) with another [`&str`].\n\n---"]
pub use const_str::replace as str_replace;

#[doc = "Sorts multiple [`&str`] ([`&[&str]`](slice) | [`[&str; N]`](array) |"]
#[doc = " [`&[&str; N]`](array)) and returns a [`[&str; _]`](array).\n\n---"]
pub use const_str::sorted as str_sorted;

#[doc = "Splits a [`&str`] by a separator pattern ([`&str`] | [`char`])"]
#[doc = " and returns a [`[&str; _]`](array).\n\n---"]
pub use const_str::split as str_split;

#[doc = "Returns [`true`] if the given pattern ([`&str`] | [`char`]) matches"]
#[doc = " a prefix of this [`&str`].\n\n---"]
pub use const_str::starts_with as str_starts_with;

#[doc = "Returns a [`&str`] with the prefix removed.\n\n---"]
pub use const_str::strip_prefix as str_strip_prefix;

#[doc = "Returns a [`&str`] with the suffix removed.\n\n---"]
pub use const_str::strip_suffix as str_strip_suffix;

#[doc = "Converts a [`&str`] or [`&[u8]`](slice) into a [`[u8; _]`](array).\n\n---"]
pub use const_str::to_byte_array as str_to_byte_array;

#[doc = "Converts a [`&str`] into a [`[char; _]`](array).\n\n---"]
pub use const_str::to_char_array as str_to_char_array;

#[doc = "Returns a [`&str`] from a value ([`&str`] | [`char`] | [`bool`] | `u*` | `i*`).\n\n---"]
pub use const_str::to_str as str_from;
