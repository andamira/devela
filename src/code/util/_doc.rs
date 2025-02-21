// devela::code::util::_doc
//
//! private doc meta helpers
//
// TOC
// - doc_!
// - doc_availability!
// - doc_miri_warn!
// - std_core!
// - EMOJI_*!, TAG_*!
//
// TODO: try to use paste! instead of concat!, since it's faster.

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

    // IMPROVE:MAYBE pass optional feature-gate arg for each module
    ( // no submodules:
        modules: $path:path; $self:ident) => {
        concat!(
            $crate::doc_!(@meta_start),
            "[", stringify!($self), "][mod@", stringify!($path), "::", stringify!($self), "]",
            $crate::doc_!(@meta_end),
        )
    };
    ( // with submodules:
        modules: $path:path; $self:ident: $($mod:ident),+ $(,)?) => {
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
            "[", stringify!($first), "](mod@",
                stringify!($path), "::", stringify!($self), "::",
                stringify!($first), ")",
            $(
                ", [", stringify!($rest), "](mod@", stringify!($path), "::",
                stringify!($self), "::", stringify!($rest), ")"
            ),*
        )
    };

    /* list of std modules */

    (extends: $($mod:ident),+ $(,)?) => {
        concat!(
            $crate::doc_!(@meta_start_nobr), "Extends: ",
            $crate::std_core!(), "::{", $crate::doc_!(@extends: $($mod),+), "}",
            $crate::doc_!(@meta_end_hr),
        )
    };
    // Handles the list of modules ensuring commas are only added between elements.
    (@extends: $first:ident $(, $rest:ident)*) => {
        concat!(
            "[", stringify!($first), "](mod@", $crate::std_core!(), "::", stringify!($first), ")",
            $( ", [", stringify!($rest), "](mod@", $crate::std_core!(), "::", stringify!($rest), ")" ),*
        )
    };
    (
    // Shows the `Vendored` doc section and links to the info line.
    //
    // $crate_id: the crate's name and html id anchor on the docs.
    vendor: $crate_id:literal) => { concat!(
        "\n\n# Vendored\n\nThis is adapted work from [",
        $crate_id, "][crate::_info::vendored#", $crate_id, "].\n\n"
    )};
    (
    // Shows the `Vendored` doc section and links to the *modifications* module.
    vendor: $crate_id:literal, module:$mod_id:ident) => { concat!(
        "\n\n# Vendored\n\nThis is adapted work from [",
        $crate_id, "][crate::_info::vendored::", $mod_id, "].\n\n"
    )};
    (
    // Assumes the path is in current directory. Used in `_info/vendored`.
    //
    // $crate_id:  the crate's name and html id anchor on the docs.
    // $text_path: the path to the text file to include, explaining the modifications.
    //
    // MAYBE: link to crate
    // MAYBE: add more information
    vendor_mod: $crate_id:literal, $mod_id:ident) => {
        #[doc = concat!(
            "# `", $crate_id,
            "` modifications\n\n[*(↑)*][crate::_info::vendored#", $crate_id, "] ",
            include_str!(concat!("./", $crate_id, ".md"))
        )]
        pub mod $mod_id {}
    };
    // (
    // // Does not assume the path. TEMP: unused
    //
    // // $crate_id:  the crate's name and html id anchor on the docs.
    // // $text_path: the path to the text file to include, explaining the modifications.
    // vendor_mod: $crate_id:literal, $text_path:literal) => { concat!(
    //     "# ", $crate_id, "\n\n[*(↑)*][crate::_info::vendored#", $crate_id, "] ",
    //     include_str!($text_path),
    // )};
}
pub(crate) use doc_;

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

/// Returns the string literal "std" if `std` is enabled, or "core" otherwise.
#[cfg(feature = "std")]
macro_rules! std_core {
    () => {
        "std"
    };
}
#[cfg(not(feature = "std"))]
macro_rules! std_core {
    () => {
        "core"
    };
}
pub(crate) use std_core;

/// Private tags definitions for visual type categorization in documentation.
mod tags {
    #![allow(unused)]
    crate::CONST! { pub(crate),
        EMOJI_ATOMIC = "⚛️"; // ⚛️,🔬,🌐
        EMOJI_DATA_STRUCTURE = "📦"; // 📦,🧩,🗂️,
        EMOJI_ERROR = "🚩"; // ❌,🚫,📛,🚧,📉,🚩,
        // EMOJI_COMPOSITE = "+"; // 📎,🧩,📦,🖇️,🗂️,
        EMOJI_ITERATOR = "🔄"; // 🔄,
        EMOJI_NAMESPACE = "🌐"; // 🌐,📛,
        EMOJI_PRIMITIVE = "⚙️"; // ⚙️,
        EMOJI_RESULT = "⚖️"; // ⚖️,↔️,✅,🗳,
        //
        TAG_ATOMIC = concat!("<span class='stab portability' title='Atomic type'>",
            crate::EMOJI_ATOMIC!(), "</span>");
        TAG_DATA_STRUCTURE =
            concat!("<span class='stab portability' title='A generic data structure'>",
            crate::EMOJI_DATA_STRUCTURE!(), "</span>");
        TAG_ERROR = concat!("<span class='stab portability' title='Individual error type'>",
            crate::EMOJI_ERROR!(), "</span>");
        TAG_ERROR_COMPOSITE =
            concat!("<span class='stab portability' title='Composite error type'>",
            crate::EMOJI_ERROR!(), "+</span>");
        TAG_ITERATOR = concat!("<span class='stab portability' title='Iterator-related item'>",
            crate::EMOJI_ITERATOR!(), "</span>");
        TAG_NAMESPACE = concat!("<span class='stab portability' title='Namespaced functionality'>",
            crate::EMOJI_NAMESPACE!(), "</span>");
        TAG_PRIMITIVE = concat!("<span class='stab portability' title='Rust primitive'>",
            crate::EMOJI_PRIMITIVE!(), "</span>");
        TAG_RESULT = concat!("<span class='stab portability' title='Result type'>",
            crate::EMOJI_RESULT!() ,"</span>");

        TAG_MAYBE_STD = "<span class='stab portability'
    title='re-exported from rust&#39;s `std` or recreated if `not(std)`'>`?std`</span>";

        /* optional dependencies */

        // used in: work::sync::atomic
        TAG_ATOMIC_CORE_PORTABLE = concat!("<span class='stab portability' ",
            "title='re-exported either from `core` or from the `portable-atomic` crate'>",
            "`?core`</span>");
        DOC_ATOMIC_CORE_PORTABLE = concat!("*Re-exported either from `core` or from the ",
            "[`portable-atomic`](https://docs.rs/portable-atomic)* crate.\n\n---");

        // used in: work::sync::re-exports and work::future::re-exports
        TAG_ATOMIC_ALLOC_PORTABLE_UTIL = concat!("<span class='stab portability' ",
            "title='re-exported either from `alloc` or from the `portable-atomic-util` crate'>",
            "`?alloc`</span>");
        DOC_ATOMIC_ALLOC_PORTABLE_UTIL = concat!("*Re-exported either from `alloc` or from the ",
            "[`portable-atomic-util`](https://docs.rs/portable-atomic-util)* crate.\n\n---");
    }
}
pub(crate) use tags::*;
