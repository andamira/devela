// devela::ascii::reexport_const_str
//
//! Reexport `const-str` crate macros related to ASCII.
//!
//! Reexport the `const-str` crate macros related to ASCII,
//! prefixed with `ascii_` and with a new first line of documentation.
//

#[doc = "Returns [`true`] if all the characters in this"]
#[doc = " ([`&str`] | [`&[u8]`](slice) | [`&[u8; N]`](array)) are ASCII.\n\n"]
#[doc = "*Reexported from the [`const-str`](https://docs.rs/const-str)* crate.\n\n---"]
#[cfg(feature = "const-str")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "str", feature = "const-str")))
)]
pub use const_str::is_ascii as ascii_check;

#[doc = "Converts a [`&str`] to a specified case."]
#[doc = " Non-ASCII characters are not affected.\n\n"]
#[doc = "*Reexported from the [`const-str`](https://docs.rs/const-str)* crate.\n\n---"]
#[cfg(feature = "const-str")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "str", feature = "const-str")))
)]
pub use const_str::convert_ascii_case as ascii_convert_case;

#[doc = "Returns [`true`] if two [`&str`] are an ASCII *case-insensitive* match.\n\n"]
#[doc = "*Reexported from the [`const-str`](https://docs.rs/const-str)* crate.\n\n---"]
#[cfg(feature = "const-str")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "str", feature = "const-str")))
)]
pub use const_str::eq_ignore_ascii_case as ascii_eq_ignore_case;

#[doc = "Splits a [`&str`] by ASCII whitespaces,"]
#[doc = " and joins the parts with a single space.\n\n"]
#[doc = "*Reexported from the [`const-str`](https://docs.rs/const-str)* crate.\n\n---"]
#[cfg(feature = "const-str")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "str", feature = "const-str")))
)]
pub use const_str::squish as ascii_squish;
