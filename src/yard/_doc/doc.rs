// devela::yard::_doc::_doc
//
//! Defines private doc meta helpers.
//
// TOC
// - _doc!
// - _doc_miri_warn!
//
// MAYBE: try to use paste! instead of concat!, since it appears to be faster.

#[doc = crate::_tags!(internal)]
/// Generates a formatted meta-documentation string.
#[doc = crate::_doc_meta!{location("yard")}]
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
// #[allow(clippy::crate_in_macro_def, reason = "to invoke _std_core from crate of invocation")]
macro_rules! _doc· {
    (@meta_start_br) => {
        "<br/><i style='margin-left:0em;'></i><span style='font-size:90%;word-spacing:0px'>"
    };
    (@meta_start_lf) => {
        "\n\n<i style='margin-left:0em;margin-top:-2em;'></i><span style='font-size:90%;word-spacing:0px'>"
    };
    (@meta_start_nobr) => {
        "<i style='margin-left:0em;margin-top:-2em;'></i><span style='font-size:90%;word-spacing:0px'>"
    };
    (@meta_end) => { "</span>" };
    (@meta_end_hr) => { "</span><hr/>" };

    // (newline) => { "<br/><br/>" };
    (newline) => { "<br/><br style='display:block;content:\"\";margin-top:10px;' />" }; // DEPRECATE
    (lf) => { "\n\n" };
    (br) => { "<br/><br style='display:block;content:\"\";margin-top:10px;' />" };
    (br+lf) => { "<br/><br style='display:block;content:\"\";margin-top:10px;' />\n\n" };
    (br+hr) => { "<br/><hr/>" };
    (hr) => { "<hr/>" };

    // link to zall_::* associated module
    (flat: $mod:literal) => {
        concat!["<a title='See the flat view of the `", $mod, "` module' href='",
            $crate::doclink![custom_current_crate concat!["zall_/_", $mod], @mod], "'>◉</a>"]//◉◦•
    };
    // link to the root * associated module
    (root: $mod:literal) => {
        concat!["<a title='See the hierarchical view of the `", $mod, "` module' href='",
            $crate::doclink![custom_current_crate $mod, @mod], "'>▽</a>"]//▽▾⧩◬◌⊙⊜◎◎
    };

    /* list of submodules */

    // IMPROVE:MAYBE pass optional feature-gate arg for each module
    ( // no submodules:
        modules: $path:path; $self:ident) => {
        concat!(
            $crate::_doc!(@meta_start_lf),
            "[", stringify!($self), "][mod@", stringify!($path), "::", stringify!($self), "]",
            $crate::_doc!(@meta_end),
        )
    };
    ( // without submodules, but showing the module line.
        modules: $path:path; $self:ident: _ $(,)?) => {
        concat!(
            $crate::_doc!(@meta_start_br),
            "[", stringify!($self), "][mod@", stringify!($path), "::", stringify!($self), "]",
            // $crate::_doc!(@modules: $path; $self: $($mod),+), "}",
            $crate::_doc!(@meta_end),
        )
    };
    ( // with submodules:
        modules: $path:path; $self:ident: $($mod:ident),+ $(,)?) => {
        concat!(
            $crate::_doc!(@meta_start_br),
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
            $crate::_doc!(@meta_start_lf), "Extends: ",
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
}
#[doc(inline)]
pub use _doc· as _doc;

#[doc = crate::_tags!(internal)]
/// Generates a formatted documentation string for a miri warning.
#[doc = crate::_doc_meta!{location("yard")}]
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
macro_rules! _doc_miri_warn· {
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
#[doc(inline)]
pub use _doc_miri_warn· as _doc_miri_warn;
