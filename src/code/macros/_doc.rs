// devela::code::macros::_doc
//
//! private doc meta helpers
//
// TOC
// - doc_!
// - doc_availability!
// - doc_miri_warn!
//

#[doc = doc_private!()]
/// Generates a formatted meta-documentation string.
macro_rules! doc_ {
    (@meta_start) => {
        "<br/><i style='margin-left:0em;'></i><span style='font-size:90%;word-spacing:0px'>"
    };
    (@meta_start_nobr) => {
        concat!(
            "<i style='margin-left:0em;margin-top:-2em;'></i>",
            "<span style='font-size:90%;word-spacing:0px'>",
        )
    };
    (@meta_end) => { "</span>" };
    (@meta_end_hr) => { "</span><hr/>" };
    // (newline) => { "<br/><br/>" };
    (newline) => { "<br/><br style='display:block;content:\"\";margin-top:10px;' />" };

    /* list of submodules */

    // IMPROVE: pass optional feature-gate arg for each module
    (modules: $path:path; $self:ident) => {
        concat!(
            $crate::doc_!(@meta_start),
            "[", stringify!($self), "][mod@", stringify!($path), "::", stringify!($self), "]",
            $crate::doc_!(@meta_end),
        )
    };
    (modules: $path:path; $self:ident: $($mod:ident),+ $(,)?) => {
        concat!(
            $crate::doc_!(@meta_start),
            "[", stringify!($self), "][mod@", stringify!($path), "::", stringify!($self), "]::{",
            $crate::doc_!(@modules: $path; $self: $($mod),+), "}",
            $crate::doc_!(@meta_end),
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
            $crate::doc_!(@meta_start_nobr), "Extends: ",
            "std::{", $crate::doc_!(@extends: $($mod),+), "}",
            $crate::doc_!(@meta_end_hr),
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

#[doc = doc_private!()]
/// Generates a formatted documentation string for conditional availability.
///
/// It's intended to be used like this:
/// ```txt
/// #[doc = crate::doc_availability!(feature = "one")]
/// #[doc = crate::doc_availability!(all(feature = "one", feature = "two")]
/// #[doc = crate::doc_availability!(any(feature = "one", feature = "two")]
/// ```
#[allow(unused_macros)]
macro_rules! doc_availability {
    (feature = $feat:literal) => {
        $crate::doc_availability!{@wrap
            "Available on <strong>crate feature ",
            $crate::doc_availability!{@code $feat},
            "</strong> only."
        }
    };

    // Handle one or more required features
    ( all( $(feature = $feat:literal),+ ) ) => {
        $crate::doc_availability!{@wrap
            "Available on <strong>crate features ",
            $crate::doc_availability!{@join_features_and $($feat),+},
            "</strong> only."
        }
    };
    // Handle one or more alternative features
    ( any( $(feature = $feat:literal),+ ) ) => {
        $crate::doc_availability!{@wrap
            "Available on <strong>crate features ",
            $crate::doc_availability!{@join_features_or $($feat),+},
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
            $crate::doc_availability!{@code $first}
            $(
                , " and ", $crate::doc_availability!{@code $rest}
            )*
        )
    };
    // single arm for joining features with "or"
    (@join_features_or $first:literal $(, $rest:literal)*) => {
        concat!(
            $crate::doc_availability!{@code $first}
            $(
                , " or ", $crate::doc_availability!{@code $rest}
            )*
        )
    };
}
#[allow(unused_imports)]
pub(crate) use doc_availability;

#[doc = doc_private!()]
/// Generates a formatted documentation string for a miri warning.
#[allow(unused_macros)]
macro_rules! doc_miri_warn {
    (tag) => {
        concat!(
            "<span class='stab portability' ",
            "title='Fails to compile with Miri.'>",
            "<code>⚠️</code></span>"
        )
    };
    (body $(, url: $url:literal)?) => {
        concat!(
            "<div class='warning'>",
            "Fails to compile with Miri.",
            $( "<p><em>See <a href = '", $url, "'>", $url, "</a>.</em></p>", )?
            "</div>"
        )
    };
}
#[allow(unused_imports)]
pub(crate) use doc_miri_warn;

/// Generates a doc tag for private items.
#[allow(unused_macros)]
macro_rules! doc_private {
    () => {
        concat!("<span class='stab portability' ", "title='Private item'>", "🔒</span>")
    };
}
#[allow(unused_imports)]
pub(crate) use doc_private;

/// Generates a doc tag for Rust's primitive items.
#[allow(unused_macros)]
macro_rules! doc_primitive {
    () => {
        concat!("<span class='stab portability' ", "title='Rust primitive'>", "⚙️</span>")
    };
}
#[allow(unused_imports)]
pub(crate) use doc_primitive;
