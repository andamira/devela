// devela::ascii::reexport
//
//! Reexport `const-str` crate macros related to ASCII.
//!
//! Reexport the `const-str` crate macros related to ASCII,
//! prefixed with `ascii_` and with a new first line of documentation.
//

/// <span class="stab portability" title="re-exported from `const-str`">`const-str`</span>
#[doc = "Returns [`true`] if all the characters in this"]
#[doc = " ([`&str`] | [`&[u8]`](slice) | [`&[u8; N]`](array)) are ASCII.\n\n"]
#[doc = "*Reexported from the [`const-str`](https://docs.rs/const-str)* crate.\n\n---"]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "ascii", feature = "depend", feature = "str")))
)]
#[cfg(any(feature = "const-str", all(feature = "str", feature = "depend")))]
pub use crate::depend::const_str::is_ascii as ascii_check;

/// <span class="stab portability" title="re-exported from `const-str`">`const-str`</span>
#[doc = "Converts a [`&str`] to a specified case."]
#[doc = " Non-ASCII characters are not affected.\n\n"]
#[doc = "*Reexported from the [`const-str`](https://docs.rs/const-str)* crate.\n\n---"]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "ascii", feature = "depend", feature = "str")))
)]
#[cfg(any(feature = "const-str", all(feature = "str", feature = "depend")))]
pub use crate::depend::const_str::convert_ascii_case as ascii_convert_case;

/// <span class="stab portability" title="re-exported from `const-str`">`const-str`</span>
#[doc = "Returns [`true`] if two [`&str`] are an ASCII *case-insensitive* match.\n\n"]
#[doc = "*Reexported from the [`const-str`](https://docs.rs/const-str)* crate.\n\n---"]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "ascii", feature = "depend", feature = "str")))
)]
#[cfg(any(feature = "const-str", all(feature = "str", feature = "depend")))]
pub use crate::depend::const_str::eq_ignore_ascii_case as ascii_eq_ignore_case;

/// <span class="stab portability" title="re-exported from `const-str`">`const-str`</span>
#[doc = "Splits a [`&str`] by ASCII whitespaces,"]
#[doc = " and joins the parts with a single space.\n\n"]
#[doc = "*Reexported from the [`const-str`](https://docs.rs/const-str)* crate.\n\n---"]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(feature = "ascii", feature = "depend", feature = "str")))
)]
#[cfg(any(feature = "const-str", all(feature = "str", feature = "depend")))]
pub use crate::depend::const_str::squish as ascii_squish;
