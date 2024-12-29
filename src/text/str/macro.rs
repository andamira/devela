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

#[cfg(test)]
mod tests {
    #![allow(unused)]

    use crate::{const_assert, str, unwrap, CStr, Slice};

    const ONE: &str = "1";
    const TWO: &str = "2";
    const TEN: &str = "10";
    const LANGS: &str = "hello你好";

    #[test]
    const fn compare() {
        const_assert!(str!(compare: <, ONE, TEN));
        const_assert!(str!(compare: >=, TWO, ONE));
        const_assert!(!str!(compare: <, TWO, ONE));
    }
    #[test]
    const fn concat() {
        const MESSAGE: &str = str!(concat: TWO, " > ", ONE);
        const_assert!(str!(compare: ==, MESSAGE, "2 > 1"));
    }
    #[test]
    const fn concat_bytes() {
        const S1: &[u8; 7] = str!(concat_bytes: b'A', b"BC", [68, b'E', 70], "G");
        const S2: &[u8; 12] = str!(concat_bytes: S1, "/123", 0u8);
        const_assert!(str!(compare: ==, S1, b"ABCDEFG"));
        const_assert!(str!(compare: ==, S2, b"ABCDEFG/123\x00"));
    }
    #[test]
    const fn contains() {
        const_assert!(str!(contains: TEN, "1"));
        const_assert!(!str!(contains: TEN, "2"));
    }
    #[test]
    const fn cstr() {
        const CSTR: &CStr = str!(cstr: "%d\n");
        const BYTES: &[u8; 4] = unwrap!(some CSTR.to_bytes_with_nul().first_chunk::<4>());
        const_assert!(str!(compare: ==, BYTES, b"%d\n\0"));
    }
    #[test]
    const fn encode() {
        const LANGS_UTF8: &[u8] = str!(encode: utf8, LANGS);
        const LANGS_UTF16: &[u16] = str!(encode: utf16, LANGS);
        const_assert!(eq_buf LANGS_UTF8, &[104, 101, 108, 108, 111, 228, 189, 160, 229, 165, 189]);
        const_assert!(Slice::<u16>::eq(LANGS_UTF16, &[104, 101, 108, 108, 111, 20320, 22909]));
    }
    #[test]
    const fn encode_z() {
        const LANGS_UTF8: &[u8] = str!(encode_z: utf8, LANGS);
        const LANGS_UTF16: &[u16] = str!(encode_z: utf16, LANGS);
        const_assert!(eq_buf LANGS_UTF8,
            &[104, 101, 108, 108, 111, 228, 189, 160, 229, 165, 189, 0]
        );
        const_assert!(Slice::<u16>::eq(LANGS_UTF16, &[104, 101, 108, 108, 111, 20320, 22909, 0]));
    }
    #[test]
    const fn ends_with() {
        const_assert!(str!(ends_with: TEN, "0"));
        const_assert!(!str!(ends_with: TEN, "1"));
        const_assert!(str!(ends_with: LANGS, "好"));
    }
    #[test]
    const fn equal() {}
    #[test]
    const fn from_utf8() {
        const BYTE_PATH: &[u8] = b"/tmp/file";
        const PATH: &str = str!(from_utf8: BYTE_PATH);
        const_assert!(eq_str PATH, "/tmp/file");
    }
    #[test]
    const fn hex() {
        const HEX: [u8; 4] = str!(hex: "01020304");
        const_assert!(eq_buf & str!(hex: "01020304"), &[1, 2, 3, 4]);
        const_assert!(eq_buf & str!(hex: "a1 b2 C3 D4"), &[0xA1, 0xB2, 0xc3, 0xd4]);
        const_assert!(eq_buf & str!(hex: ["0a0B", "0C0d"]), &[10, 11, 12, 13]);
    }
    #[test]
    const fn join() {
        const_assert!(eq_str str!(join: &[ONE, TWO, TEN], ","), "1,2,10");
        const_assert!(eq_str str!(join: &[ONE, TWO, TEN], ""), "1210");
    }
    #[test]
    const fn parse() {
        const_assert!(eq str!(parse: "true", bool), true);
        const_assert!(eq str!(parse: "false", bool), false);
        const_assert!(eq str!(parse: "16723", usize), 16723);
        const_assert!(eq str!(parse: "-100", i8), -100);
        const_assert!(eq str!(parse: "€", char), '€');
    }
    #[test]
    const fn raw_cstr() {
        const CCHAR: *const crate::c_char = str!(raw_cstr: "%d\n");
    }
    #[test]
    const fn repeat() {
        const_assert!(eq_str str!(repeat: TEN, 3), "101010");
    }
    #[test]
    const fn replace() {
        const_assert!(eq_str str!(replace: "original", "gin", "tonic"), "oritonical");
        const_assert!(eq_str str!(replace: "original", 'g', "G"), "oriGinal");
    }
    #[test]
    const fn sorted() {
        const SORTED: &[&str] = &str!(sorted: ["one", "two", "three"]);
        const_assert!(Slice::<&[&str]>::eq(SORTED, &["one", "three", "two"]));
        const_assert!(Slice::<&[&str]>::eq(&str!(sorted: ["1", "2", "10"]), &["1", "10", "2"]));
    }
    #[test]
    const fn split() {
        const TEXT: &str = "apple, kiwi, banana";
        const_assert!(Slice::<&[&str]>::eq(&str!(split: TEXT, ", "), &["apple", "kiwi", "banana"]));
    }
    #[test]
    const fn starts_with() {
        const_assert!(str!(starts_with: "banana", 'b'));
        const_assert!(str!(starts_with: "banana", "ban"));
        const_assert!(!str!(starts_with: "banana", "a"));
    }
    #[test]
    const fn strip_prefix() {
        const_assert!(eq_str unwrap![some str!(strip_prefix: "banana", "ban")], "ana");
        const_assert!(str!(strip_prefix: "banana", "a").is_none());
    }
    #[test]
    const fn strip_suffix() {
        const_assert!(eq_str unwrap![some str!(strip_suffix: "banana", "ana")], "ban");
        const_assert!(str!(strip_suffix: "banana", "b").is_none());
    }
    #[test]
    const fn to_byte_array() {
        const ARRAY: [u8; 5] = str![to_byte_array: "hello"];
        const_assert!(eq_buf & ARRAY, &[b'h', b'e', b'l', b'l', b'o']);
    }
    #[test]
    const fn to_char_array() {
        const ARRAY: [char; 5] = str![to_char_array: "hello"];
        const_assert!(Slice::<char>::eq(&ARRAY, &['h', 'e', 'l', 'l', 'o']));
    }
    #[test]
    const fn to_str() {
        const_assert!(eq_str str!(to_str: "string"), "string");
        const_assert!(eq_str str!(to_str: '€'), "€");
        const_assert!(eq_str str!(to_str: false), "false");
        const_assert!(eq_str str!(to_str: 50u32 - 3), "47");
        const_assert!(eq_str str!(to_str: 5i8 - 9), "-4");
    }
    #[test]
    const fn convert_ascii_case() {
        const_assert!(eq_str str!(convert_ascii_case:lower, "Lower Case"), "lower case");
        const_assert!(eq_str str!(convert_ascii_case:upper, "Upper Case"), "UPPER CASE");
        const_assert!(eq_str str!(convert_ascii_case:lower_camel, "lower camel"), "lowerCamel");
        const_assert!(eq_str str!(convert_ascii_case:upper_camel, "upper camel"), "UpperCamel");
        const_assert!(eq_str str!(convert_ascii_case:upper_camel, "upper camel"), "UpperCamel");
        const_assert!(eq_str str!(convert_ascii_case:snake, "snake case"), "snake_case");
        const_assert!(eq_str str!(convert_ascii_case:kebab, "kebab case"), "kebab-case");
        const_assert!(eq_str str!(convert_ascii_case:shouty_snake, "shouty snake"), "SHOUTY_SNAKE");
        const_assert!(eq_str str!(convert_ascii_case:shouty_kebab, "shouty kebab"), "SHOUTY-KEBAB");
    }
    #[test]
    const fn eq_ignore_ascii_case() {
        const_assert!(str!(eq_ignore_ascii_case: "Ferris", "FERRIS"));
        const_assert!(str!(eq_ignore_ascii_case: "Ferrös", "FERRöS"));
        const_assert!(!str!(eq_ignore_ascii_case: "Ferrös", "FERRÖS"));
    }
    #[test]
    const fn is_ascii() {
        const_assert!(str!(is_ascii: "hello\n"));
        const_assert!(!str!(is_ascii: LANGS));
    }
    #[test]
    const fn squish() {
        const_assert!(eq_str str!(squish: "   SQUISH  \t THAT  \t CAT!    "), "SQUISH THAT CAT!");
    }
    #[test]
    const fn unwrap() {
        struct NonCopy;
        let a: u8 = str!(unwrap: Some(23));
        // let b: NonCopy = str!(unwrap: Some(NonCopy)); // FAIL: only works with Copy types
        let a: NonCopy = unwrap!(some Some(NonCopy)); // but our unwrap macro can do :)
    }
}
