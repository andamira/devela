// devela_base::code::util::_doc
//
//! Defines private doc meta helpers.
//
// TOC
// - _doc!
// - _doc_availability!
// - _doc_miri_warn!
// - _std_core!
//
// TODO: try to use paste! instead of concat!, since it's faster.

/// Generates a formatted meta-documentation string.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[expect(clippy::crate_in_macro_def, reason = "to invoke _std_core from crate of invocation")]
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
            crate::_std_core!(), "::{", $crate::_doc!(@extends: $($mod),+), "}",
            $crate::_doc!(@meta_end_hr),
        )
    };
    // Handles the list of modules ensuring commas are only added between elements.
    (@extends: $first:ident $(, $rest:ident)*) => {
        concat!(
            "[", stringify!($first), "](mod@", crate::_std_core!(), "::", stringify!($first), ")",
            $(
            ", [", stringify!($rest), "](mod@", crate::_std_core!(), "::", stringify!($rest), ")"
            ),*
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

// WAIT [missing cross-crate docs](https://github.com/rust-lang/rust/issues/120927)
// IMPROVE: further differentiate between publish with docs.rs docs or alternative.
/// Helps doc-linking to items in downstream crates. (local version)
///
/// Returns the constructed URL to the
///
/// # Features
/// If the `__publish` feature is enabled it links to the published documentation,
/// otherwise it links to the local path.
///
// Usage examples:
// - devela_base::data::bit::field::bitfield
// - devela::code::util::enumset
#[doc(hidden)] #[macro_export] #[cfg(not(feature = "__publish"))] #[rustfmt::skip]
macro_rules! __doclink {
    ( // [anchor]: file://…/crate_name/item_path/index.html
     $anchor:literal $crate_name:ident $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["\n\n", $anchor, ": ", $crate::_doclink![$crate_name $item_path $(@mod$($_m)?)?]]
    };
    ( // file://…/crate_name/item_path/index.html
     $crate_name:ident $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        // NOTE: in sync with ../../../build/environment.rs & /.cargo/config.toml#[env]
        concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        stringify!($crate_name), "/", $item_path $(, "/index.html"$($_m)?)?]
    };
}
/// Helps doc-linking to items in downstream crates. (internet version)
#[doc(hidden)] #[macro_export] #[cfg(feature = "__publish")] #[rustfmt::skip]
macro_rules! __doclink {
    ( // [anchor]: https://…/crate_name/item_path
     $anchor:literal $crate_name:ident $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["\n\n", $anchor, ": ", $crate::_doclink![$crate_name $item_path $(@mod$($_m)?)?]]
    };
    ( // https://…/crate_name/item_path
     $crate_name:ident $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["https://andamira.github.io/", stringify!($crate_name),
        "/latest/", stringify!($crate_name), "/", $item_path]
    };
}
#[doc(hidden)]
pub use __doclink as _doclink;

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

crate::sf! {
    /// Should return the string literal "std" if `std` is enabled, or "core" otherwise.
    macro_rules! _std_core { () => { "core" }; }
    #[allow(unused_imports)] pub(crate) use _std_core;
}
