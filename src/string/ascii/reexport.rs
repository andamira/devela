// devela::string::ascii::reexport
//
//! Reexport `const-str` crate macros related to ASCII.
//!
//! Reexport the `const-str` crate macros related to ASCII,
//! prefixed with `ascii_` and with a new first line of documentation.
//

use crate::codegen::reexport;

reexport! { "const-str" | const_str , features: "string",
    doc: "Returns [`true`] if all codes in this
        ([`&str`] | [`&[u8]`](slice) | [`&[u8; N]`](array)) are ASCII.",
    @is_ascii as ascii_check
}
reexport! { "const-str" | const_str , features: "string",
    doc: "Converts a [`&str`] to a specified case. Non-ASCII characters are not affected.",
    @convert_ascii_case as convert_ascii_case
}
reexport! { "const-str" | const_str , features: "string",
    doc: "Returns [`true`] if two [`&str`] are an ASCII *case-insensitive* match.",
    @eq_ignore_ascii_case as eq_ignore_ascii_case
}
reexport! { "const-str" | const_str , features: "string",
    doc: "Splits a [`&str`] by ASCII whitespaces, and joins the parts with a single space.",
    @squish as squish
}
