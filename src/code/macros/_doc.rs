// devela::code::macros::doc
//
//! private meta helpers
//
// TOC
// - doc_!
// - doc_availability!
// - doc_miri_warn!

// Generates a formatted meta-documentation string.
macro_rules! doc_ {
    // (@meta_start) => { "<br/><i style='margin-left:0.618em;'></i><small style='color:#777'>" };
    // (@meta_end) => { "</small>" };
    (@meta_start) => {
        "<br/><i style='margin-left:0.618em;'></i><span style='color:#777;font-size:90%;word-spacing:0px'>"
    };
    (@meta_end) => { "</span>" };
    // (newline) => { "<br/><br/>" };
    (newline) => { "<br/><br style='display:block;content:\"\";margin-top:10px;' />" };

    /* list of submodules */

    (modules: $path:path; $self:ident) => {
        concat!(
            crate::code::doc_!(@meta_start), "Modules: ",
            "[", stringify!($self), "][mod@", stringify!($path), "::", stringify!($self), "]",
            crate::code::doc_!(@meta_end),
        )
    };
    (modules: $path:path; $self:ident: $($mod:ident),+ $(,)?) => {
        concat!(
            crate::code::doc_!(@meta_start), "Modules: ",
            "[", stringify!($self), "][mod@", stringify!($path), "::", stringify!($self), "]::{",
            crate::code::doc_!(@modules: $path; $self: $($mod),+), "}",
            crate::code::doc_!(@meta_end),
        )
    };
    // Handles the list of modules ensuring commas are only added between elements.
    (@modules: $path:path; $self:ident: $first:ident $(, $rest:ident)*) => {
        concat!(
            "[", stringify!($first), "](",
                stringify!($path), "::", stringify!($self), "::",
                stringify!($first), ")",
            $(
                ", [", stringify!($rest), "](", stringify!($path), "::",
                stringify!($self), "::", stringify!($rest), ")"
            ),*
        )
    };

    /* list of std modules */

    (extends: $($mod:ident),+ $(,)?) => {
        concat!(
            crate::code::doc_!(@meta_start), "Extends: ",
            "std::{", crate::code::doc_!(@extends: $($mod),+), "}",
            crate::code::doc_!(@meta_end),
        )
    };
    // Handles the list of modules ensuring commas are only added between elements.
    (@extends: $first:ident $(, $rest:ident)*) => {
        concat!(
            "[", stringify!($first), "](mod@std::", stringify!($first), ")",
            $( ", [", stringify!($rest), "](mod@std::", stringify!($rest), ")" ),*
        )
    };
}
pub(crate) use doc_;

// Generates a formatted documentation string for conditional availability.
//
// It's intended to be used like this:
// ```
// #[doc = crate::code::doc_availability!(feature = "one"]
// #[doc = crate::code::doc_availability!(all(feature = "one", feature = "two")]
// #[doc = crate::code::doc_availability!(any(feature = "one", feature = "two")]
// ```
#[allow(unused_macros)]
macro_rules! doc_availability {
    (feature = $feat:literal) => {
        $crate::code::doc_availability!{@wrap
            "Available on <strong>crate feature ",
            $crate::code::doc_availability!{@code $feat},
            "</strong> only."
        }
    };

    // Handle one or more required features
    ( all( $(feature = $feat:literal),+ ) ) => {
        $crate::code::doc_availability!{@wrap
            "Available on <strong>crate features ",
            $crate::code::doc_availability!{@join_features_and $($feat),+},
            "</strong> only."
        }
    };
    // Handle one or more alternative features
    ( any( $(feature = $feat:literal),+ ) ) => {
        $crate::code::doc_availability!{@wrap
            "Available on <strong>crate features ",
            $crate::code::doc_availability!{@join_features_or $($feat),+},
            "</strong> only."
        }
    };

    /* helper arms */

    // Outer wrap
    (@wrap $($strings:tt)+) => {
        concat!(
            "<div class='item-info' style='margin-left:0;'>",
            "<div class='stab portability'>",
            $($strings)+,
            "</div></div>"
        )
    };

    // code-formatted literal
    (@code $string:literal) => {
        concat!("<code style='background:none'>", $string, "</code>")
    };

    // single arm for joining features with "and"
    (@join_features_and $first:literal $(, $rest:literal)*) => {
        concat!(
            $crate::code::doc_availability!{@code $first}
            $(
                , " and ", $crate::code::doc_availability!{@code $rest}
            )*
        )
    };
    // single arm for joining features with "or"
    (@join_features_or $first:literal $(, $rest:literal)*) => {
        concat!(
            $crate::code::doc_availability!{@code $first}
            $(
                , " or ", $crate::code::doc_availability!{@code $rest}
            )*
        )
    };
}
#[allow(unused_imports)]
pub(crate) use doc_availability;

// Generates a formatted documentation string for a miri warning
#[allow(unused_macros)]
macro_rules! doc_miri_warn {
    (tag) => {
        concat!(
            "<span class='stab portability' ",
            "title='Fails to compile under Miri due to potential undefined behavior'>",
            "<code>⚠️</code></span>"
        )
    };
    (body $(, url: $url:literal)?) => {
        concat!(
            "<div class='warning'>",
            "Fails to compile under Miri due to potential undefined behavior.",
            $( "<p><em>See <a href = '", $url, "'>", $url, "</a>.</em></p>", )?
            "</div>"
        )
    };
}
#[allow(unused_imports)]
pub(crate) use doc_miri_warn;
