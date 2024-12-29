// devela::text::str::macro

/// [`&str`] compile-time operations, namespaced from the [const-str][::const_str] crate.
///
/// The name of each operation links to the original macro documentation.
///
/// # Operations
/// - [`compare:`][::const_str::compare]
///   Compares two [`&str`] lexicographically.
/// - [`concat:`][::const_str::concat]
///   Concatenates ([`&str`] | [`char`] | [`bool`] | `u*` | `i*`) into a [`&str`].
/// - [`concat_bytes:`][::const_str::concat_bytes] Concatenates ([`&str`] | [`u8`]
///   | [`&[u8]`](slice) | [`[u8; N]`](array) | [`&[u8; N]`](array)) to [`&[u8; _]`](array).
/// - [`contains:`][::const_str::contains]
///   Returns [`true`] if the given pattern ([`&str`] | [`char`]) matches a sub-[`&str`].
/// - [`cstr:`][::const_str::cstr]
///   Converts a [`&str`] to [`&CStr`](core::ffi::CStr).
/// - [`encode:`][::const_str::encode]
///   Encodes a [`&str`] with an encoding (`utf8` | `utf16`).
/// - [`encode_z:`][::const_str::encode_z]
///   Encodes a [`&str`] with an encoding (`utf8` | `utf16`) and append a NUL char.
/// - [`ends_with:`][::const_str::ends_with]
///   Returns `true` if the given pattern matches a suffix of this [`&str`].
/// - [`equal:`][::const_str::equal]
///   Returns [`true`] if two [`&str`] are equal.
/// - [`from_utf8:`][::const_str::from_utf8]
///   Returns a [`&str`] from a [`&[u8]`](slice). Panics if it's not valid utf8.
/// - [`hex:`][::const_str::hex]
///   Converts a [`&str`] with hexadecimals (`0-9` | `A-F` | `a-f`) into a [`[u8; _]`](array).
/// - [`join:`][::const_str::join]
///   Concatenates multiple [`&str`] into a [&str] separated by the given separator.
/// - [`parse:`][::const_str::parse]
///   Parses a [`&str`] into a value ([`&str`] | [`char`] | [`bool`] | `u*` | `i*`).
/// - [`raw_cstr:`][::const_str::raw_cstr]
///   Converts a [`&str`] into a [`*const c_char`](core::ffi::c_char).
/// - [`repeat:`][::const_str::repeat]
///   Creates a [`&str`] by repeating a [`&str`] n times.
/// - [`replace:`][::const_str::replace]
///   Replaces all matches of a pattern ([`&str`] | [`char`]) with another [`&str`].
/// - [`sorted:`][::const_str::sorted]
///   Sorts multiple ([`&[&str]`](slice) | [`[&str; N]`](array) |
///   [`&[&str; N]`](array)) into a [`[&str; _]`](array).
/// - [`split:`][::const_str::split]
///   Splits a [`&str`] by a separator pattern ([`&str`] | [`char`])
///   returning [`[&str; _]`](array).
/// - [`starts_with:`][::const_str::starts_with]
///   Returns [`true`] if the given pattern ([`&str`] | [`char`]) matches a prefix of [`&str`].
/// - [`strip_prefix:`][::const_str::strip_prefix]
///   Returns a [`&str`] with the prefix removed.
/// - [`strip_suffix:`][::const_str::strip_suffix]
///   Returns a [`&str`] with the suffix removed.
/// - [`to_byte_array:`][::const_str::to_byte_array]
///   Converts a [`&str`] or [`&[u8]`](slice) into a [`[u8; _]`](array).
/// - [`to_char_array:`][::const_str::to_char_array]
///   Converts a [`&str`] into a [`[char; _]`](array).
/// - [`to_str:`][::const_str::to_str]
///   Returns a [`&str`] from a value ([`&str`] | [`char`] | [`bool`] | `u*` | `i*`).
/// - [`unwrap:`][::const_str::unwrap] Unwraps a container, returns the content
///   (see also the [`unwrap!`][crate::unwrap] macro).
///
/// Ascii related:
/// - [`convert_ascii_case:`][::const_str::convert_ascii_case]
///   Converts a [`&str`] to a specified case. Non-ASCII characters are not affected.
/// - [`eq_ignore_ascii_case:`][::const_str::eq_ignore_ascii_case]
///   Returns [`true`] if two [`&str`] are an ASCII *case-insensitive* match.
/// - [`is_ascii:`][::const_str::is_ascii]
///   Returns [`true`] if all codes in this
///   ([`&str`] | [`&[u8]`](slice) | [`&[u8; N]`](array)) are ASCII.
/// - [`squish:`][::const_str::squish]
///   Splits a [`&str`] by ASCII whitespaces, and joins the parts with a single space.
#[macro_export]
#[doc(hidden)]
macro_rules! _str { // 29 arms
    (compare: $($t:tt)*) => {$crate::_dep::const_str::compare!{$($t)*} };
    (concat: $($t:tt)*) => {$crate::_dep::const_str::concat!{$($t)*} };
    (concat_bytes: $($t:tt)*) => {$crate::_dep::const_str::concat_bytes!{$($t)*} };
    (contains: $($t:tt)*) => { $crate::_dep::const_str::contains!{$($t)*} };
    (cstr: $($t:tt)*) => {$crate::_dep::const_str::cstr!{$($t)*} };
    (encode: $($t:tt)*) => {$crate::_dep::const_str::encode!{$($t)*} };
    (encode_z: $($t:tt)*) => {$crate::_dep::const_str::encode_z!{$($t)*} };
    (ends_with: $($t:tt)*) => {$crate::_dep::const_str::ends_with!{$($t)*} };
    (equal: $($t:tt)*) => {$crate::_dep::const_str::equal!{$($t)*} };
    (from_utf8: $($t:tt)*) => {$crate::_dep::const_str::from_utf8!{$($t)*} };
    (hex: $($t:tt)*) => {$crate::_dep::const_str::hex!{$($t)*} };
    (join: $($t:tt)*) => {$crate::_dep::const_str::join!{$($t)*} };
    (parse: $($t:tt)*) => {$crate::_dep::const_str::parse!{$($t)*} };
    (raw_cstr: $($t:tt)*) => {$crate::_dep::const_str::raw_cstr!{$($t)*} };
    (repeat: $($t:tt)*) => {$crate::_dep::const_str::repeat!{$($t)*} };
    (replace: $($t:tt)*) => {$crate::_dep::const_str::replace!{$($t)*} };
    (sorted: $($t:tt)*) => {$crate::_dep::const_str::sorted!{$($t)*} };
    (split: $($t:tt)*) => {$crate::_dep::const_str::split!{$($t)*} };
    (starts_with: $($t:tt)*) => {$crate::_dep::const_str::starts_with!{$($t)*} };
    (strip_prefix: $($t:tt)*) => {$crate::_dep::const_str::strip_prefix!{$($t)*} };
    (strip_suffix: $($t:tt)*) => {$crate::_dep::const_str::strip_suffix!{$($t)*} };
    (to_byte_array: $($t:tt)*) => {$crate::_dep::const_str::to_byte_array!{$($t)*} };
    (to_char_array: $($t:tt)*) => {$crate::_dep::const_str::to_char_array!{$($t)*} };
    (to_str: $($t:tt)*) => {$crate::_dep::const_str::to_str!{$($t)*} };
    (
     is_ascii: $($t:tt)*) => {$crate::_dep::const_str::is_ascii!{$($t)*} };
    (convert_ascii_case: $($t:tt)*) => {$crate::_dep::const_str::convert_ascii_case!{$($t)*} };
    (eq_ignore_ascii_case: $($t:tt)*) => {$crate::_dep::const_str::eq_ignore_ascii_case!{$($t)*} };
    (squish: $($t:tt)*) => {$crate::_dep::const_str::squish!{$($t)*} };
    (unwrap: $($t:tt)*) => {$crate::_dep::const_str::unwrap!{$($t)*} };
}
#[doc(inline)]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_const_str")))]
pub use _str as str;
