// devela_base_core::code::util::_doc
//
//! Defines private doc meta helpers.
//
// TOC
// - _doc!
// - _doc_availability!
// - _doc_miri_warn!
//
// TODO: try to use paste! instead of concat!, since it's faster.

/// Generates a formatted meta-documentation string.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
// #[allow(clippy::crate_in_macro_def, reason = "to invoke _std_core from crate of invocation")]
macro_rules! __doc {
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
            $crate::_doc!(@meta_start),
            "[", stringify!($self), "][mod@", stringify!($path), "::", stringify!($self), "]",
            $crate::_doc!(@meta_end),
        )
    };
    ( // with submodules:
        modules: $path:path; $self:ident: $($mod:ident),+ $(,)?) => {
        concat!(
            $crate::_doc!(@meta_start),
            "[", stringify!($self), "][mod@", stringify!($path), "::", stringify!($self), "]::{",
            $crate::_doc!(@modules: $path; $self: $($mod),+), "}",
            $crate::_doc!(@meta_end),
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

    // NOTE: it needs `_std_core!` to be defined in the crate where this is invoked.
    (extends: $($mod:ident),+ $(,)?) => {
        concat!(
            $crate::_doc!(@meta_start_nobr), "Extends: ",
            // crate::_std_core!(), "::{", $crate::_doc!(@extends: $($mod),+), "}",
            "std::{", $crate::_doc!(@extends: $($mod),+), "}",
            $crate::_doc!(@meta_end_hr),
        )
    };
    // Handles the list of modules ensuring commas are only added between elements.
    (@extends: $first:ident $(, $rest:ident)*) => {
        concat!(
            // "[", stringify!($first), "](mod@", crate::_std_core!(), "::", stringify!($first), ")",
            // $(
            // ", [", stringify!($rest), "](mod@", crate::_std_core!(), "::", stringify!($rest), ")"
            // ),*
            "[", stringify!($first), "](https://doc.rust-lang.org/std/", stringify!($first), ")",
            $(
            ", [", stringify!($rest), "](https://doc.rust-lang.org/std/", stringify!($rest), ")"
            ),*
        )
    };
    // -------------------------------------------------------------------------
    (
    // Links to original location in the workspace
    // (NOTE: do NOT pass a leading slash)
    //
    // Links to the module where this item was defined, in the devela crate.
    // USAGE: #[doc = crate::_doc!(location: "data/iter")]
    location: $path:literal) => {
        concat!($crate::_doc!(%location_prefix),
            "<sup title='canonical workspace location'>[`[", $path, "]`](",
            $crate::doclink![custom devela $path @mod], ")</sup>\n\n") };
    ( // the same, but without ending in a new line. used in _reexport!
    location_inline: $path:literal) => {
        concat!($crate::_doc!(%location_prefix_inline),
            "<sup title='canonical workspace location'>[`[", $path, "]`](",
            $crate::doclink![custom devela $path @mod], ")</sup>") };
    // (
    // // Links to the item in the module where it was defined, in the devela crate.
    // // USAGE: #[doc = crate::_doc!(location_item: "text/char/struct.Char.html")]
    // location_item: $path:literal) => {
    //     concat!($crate::_doc!(%location_prefix),
    //         "<sup title='canonical workspace location'><em>[`", $path, "`](",
    //         $crate::doclink![custom devela $path], ")</em></sup>\n\n") };
    (
    // Links to the module where this item was defined, in the given crate.
    // USAGE: #[doc = crate::_doc!(location: devela, "data/iter")]
    location: $crate_id:ident, $path:literal) => {
        concat!($crate::_doc!(%location_prefix),
            "<small title='canonical workspace location '>[`📍", $path, "`](",
            $crate::doclink![custom $crate_id $path @mod], ")</small>\n\n") };
    (%location_prefix) => { "\n\n---\n\n<sup>`📍`</sup>" /* 🎅DEBUG*/ };
    (%location_prefix_inline) => { "\n\n<sup>`📍`</sup>" };
    // -------------------------------------------------------------------------
    (
    // Shows the `Vendored` doc section and links to the info line.
    //
    // $crate_id: the crate's name and html id anchor on the docs.
    vendor: $crate_id:literal) => {
        concat!("\n\n# Vendored\n\nThis is adapted work from [", $crate_id, "](",
            $crate::doclink![custom devela "_doc/vendored" @mod],
            "#", $crate_id, ").\n\n"
        )
    };
    // MAYBE
    // (
    // // Shows the `Vendored` doc section and links to the *modifications* module.
    // vendor: $crate_id:literal, module:$mod_id:ident) => { concat!(
    //     "\n\n# Vendored\n\nThis is adapted work from [",
    //     $crate_id, "][crate::_doc::vendored::", $mod_id, "].\n\n"
    // )};

    (
    // Assumes the path is in current directory. Used in `_doc/vendored`.
    //
    // $crate_id:  the crate's name and html id anchor on the docs.
    // $text_path: the path to the text file to include, explaining the modifications.
    //
    // MAYBE: link to crate
    // MAYBE: add more information
    vendor_mod: $crate_id:literal, $mod_id:ident) => {
        #[doc = concat!(
            "# `", $crate_id,
            "` modifications\n\n[*(↑)*][crate::_doc::vendored#", $crate_id, "] ",
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
    //     "# ", $crate_id, "\n\n[*(↑)*][crate::_doc::vendored#", $crate_id, "] ",
    //     include_str!($text_path),
    // )};
}
#[doc(hidden)]
pub use __doc as _doc;

/// Generates a formatted documentation string for conditional availability.
///
/// It's intended to be used like this:
/// ```txt
/// #[doc = crate::doc_availability!(feature = "one")]
/// #[doc = crate::doc_availability!(all(feature = "one", feature = "two")]
/// #[doc = crate::doc_availability!(any(feature = "one", feature = "two")]
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! __doc_availability {
    (feature = $feat:literal) => {
        $crate::_doc_availability!{@wrap
            "Available on <strong>crate feature ",
            $crate::_doc_availability!{@code $feat},
            "</strong> only."
        }
    };

    // Handle one or more required features
    ( all( $(feature = $feat:literal),+ ) ) => {
        $crate::_doc_availability!{@wrap
            "Available on <strong>crate features ",
            $crate::_doc_availability!{@join_features_and $($feat),+},
            "</strong> only."
        }
    };
    // Handle one or more alternative features
    ( any( $(feature = $feat:literal),+ ) ) => {
        $crate::_doc_availability!{@wrap
            "Available on <strong>crate features ",
            $crate::_doc_availability!{@join_features_or $($feat),+},
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
            $crate::_doc_availability!{@code $first}
            $(
                , " and ", $crate::_doc_availability!{@code $rest}
            )*
        )
    };
    // single arm for joining features with "or"
    (@join_features_or $first:literal $(, $rest:literal)*) => {
        concat!(
            $crate::_doc_availability!{@code $first}
            $(
                , " or ", $crate::_doc_availability!{@code $rest}
            )*
        )
    };
}
#[doc(hidden)]
pub use __doc_availability as _doc_availability;

/// Generates a formatted documentation string for a miri warning.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! __doc_miri_warn {
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
#[doc(hidden)]
pub use __doc_miri_warn as _doc_miri_warn;
