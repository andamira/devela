// devela::yard::_doc::_doc
//
//! Defines private doc meta helpers.
//
// TOC
// - _doc!
// - _doc_meta!
// - _doc_availability!
// - _doc_location!
// - _doc_miri_warn!
// - _doc_size_of!
//
// MAYBE: try to use paste! instead of concat!, since it appears to be faster.

#[doc = crate::_tags!(internal)]
/// Generates a formatted meta-documentation string.
#[doc = crate::_doc_location!("yard")]
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
/// Composes a small rustdoc metadata section for an item.
#[doc = crate::_doc_meta!{location("yard")}]
///
/// This macro centralizes the ad-hoc metadata band used near the top of item
/// documentation. It wraps supported metadata fragments between horizontal
/// rules and dispatches each section to the corresponding helper.
///
/// # Sections
/// - `location(...)`: emits an item location fragment through [`_doc_location!`].
/// - `size_of(...)`: emits checked type-size metadata through [`_doc_size_of!`].
/// - `origin(...)`: emits re-export origin metadata for Rust or dependency items.
///
/// # Examples
/// ```ignore
/// #[doc = crate::_doc_meta! { location("media/audio"), size_of(PcmSpec = 8) }]
/// #[doc = crate::_doc_meta! { location(re-exported "code/any"), origin(rust core::any) }]
/// #[doc = crate::_doc_meta! { location(re-exported "code"), origin(crate "hashbrown") }]
/// ```
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
macro_rules! _doc_meta· {
    /* internal dispatcher: end */
    (@items) => { "" };
    (@items ,) => { "" };
    /* public section: location */
    (@items location($($args:tt)*) $(, $($rest:tt)*)?) => {
        concat!(
            $crate::_doc_location!(%from_meta $($args)*),
            $crate::_doc_meta!(@items $($($rest)*)?)
        )
    };
    /* public section: size_of */
    (@items size_of($ty:ty = $bytes:literal) $(, $($rest:tt)*)?) => {
        concat!(
            $crate::_doc_size_of!($ty = $bytes),
            $crate::_doc_meta!(@items $($($rest)*)?)
        )
    };
    (@items size_of($name:ident : $ty:ty = $bytes:literal) $(, $($rest:tt)*)?) => {
        concat!(
            $crate::_doc_size_of!($name : $ty = $bytes),
            $crate::_doc_meta!(@items $($($rest)*)?)
        )
    };
    (@items size_of(abs $name:ident : $ty:ty = $bytes:literal) $(, $($rest:tt)*)?) => {
        concat!(
            $crate::_doc_size_of!(abs $name : $ty = $bytes),
            $crate::_doc_meta!(@items $($($rest)*)?)
        )
    };

    /* public section: origin from Rust core/alloc/std */
    (@items origin(rust $root:ident $(:: $path:ident)* $(;
        renamed($($old:ident as $new:ident),* $(,)?))?) $(, $($rest:tt)*)?) => {
        concat!(" ",
            $crate::_doc_meta!(@emit_origin_rust $root $(:: $path)* ; $($($old as $new),*)?),
            $crate::_doc_meta!(@items $($($rest)*)?)
        )
    };

    /* public section: origin from external crate */
    (@items origin(crate $dep:literal
        $(; renamed($($old:ident as $new:ident),* $(,)?))?) $(, $($rest:tt)*)?) => {
        concat!(" ",
            $crate::_doc_meta!(@emit_origin_crate $dep; $($($old as $new),*)?),
            $crate::_doc_meta!(@items $($($rest)*)?)
        )
    };

    /* optional: display name differs from docs.rs crate slug */
    (@items origin(crate $shown:literal => $docs:literal
        $(; renamed($($old:ident as $new:ident),* $(,)?))?) $(, $($rest:tt)*)?) => {
        concat!(" ",
            $crate::_doc_meta!(@emit_origin_crate_as $shown => $docs; $($($old as $new),*)?),
            $crate::_doc_meta!(@items $($($rest)*)?)
        )
    };

    /* internal emitters */
    (@emit_origin_rust $root:ident $(:: $path:ident)* ; $($renamed:tt)*) => {
        concat!(
            "<sup>re-exported from <a title='location in `", ::core::stringify!($root),
            "`' href=\"https://doc.rust-lang.org/", ::core::stringify!($root), "/",
            $( ::core::stringify!($path), "/", )* "\">", ::core::stringify!($root),
            $("::", ::core::stringify!($path),)* "</a>",
            $crate::_doc_meta!(@emit_renamed $($renamed)*), "</sup>"
        )
    };
    (@emit_origin_crate $dep:literal; $($renamed:tt)*) => {
        concat!(
            "<sup>re-exported from the <a title='docs for `", $dep,
            "`' href=\"https://docs.rs/", $dep, "\">", $dep, "</a> crate",
            $crate::_doc_meta!(@emit_renamed $($renamed)*), ".</sup>"
        )
    };

    (@emit_origin_crate_as $shown:literal => $docs:literal; $($renamed:tt)*) => {
        concat!(
            "<sup>re-exported from the <a title='docs for `", $shown,
            "`' href=\"https://docs.rs/", $docs, "\">", $shown, "</a> crate",
            $crate::_doc_meta!(@emit_renamed $($renamed)*), ".</sup>"
        )
    };
    (@emit_renamed) => { "" };
    (@emit_renamed $($old:ident as $new:ident),+ $(,)?) => {
        concat!( " ", $( "`::", stringify!($old), "` as `", stringify!($new), "` " ),+)
    };

    /* diagnostics */
    (@items $bad:ident($($args:tt)*) $(, $($rest:tt)*)?) => {
        compile_error!(concat!( "unknown _doc_meta! section: `", stringify!($bad), "`"))
    };

    /* public entrypoints last */
    () => { "" };
    ($($rest:tt)+) => {
        concat!(
            "\n\n---\n\n",
            $crate::_doc_meta!(@items $($rest)+),
            "\n\n---\n\n",
        )
    };
}
#[doc(inline)]
pub use _doc_meta· as _doc_meta;

#[doc = crate::_tags!(internal)]
/// Generates a formatted documentation string for conditional availability.
#[doc = crate::_doc_location!("yard")]
///
/// It's intended to be used like this:
/// ```txt
/// #[doc = crate::doc_availability!(feature = "one")]
/// #[doc = crate::doc_availability!(all(feature = "one", feature = "two")]
/// #[doc = crate::doc_availability!(any(feature = "one", feature = "two")]
/// ```
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
macro_rules! _doc_availability· {
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
#[doc(inline)]
pub use _doc_availability· as _doc_availability;

// WAIT: doctests cannot see items/macros behind cfg(doctest) in dependencies:
// WAIT: [cfg(doctest)](https://github.com/rust-lang/rust/issues/67295)
// no-op stub for doctests
// #[cfg(doctest)]
// #[doc(hidden)]
// #[macro_export]
// macro_rules! _doc_location {
//     ($($tt:tt)*) => { "" }
// }

// #[cfg(not(doctest))]
#[doc = crate::_tags!(internal)]
/// Emits a location annotation for documentation.
#[doc = crate::_doc_location!("yard")]
///
/// This macro renders a small location marker (`📍`) followed by the public
/// API path under `devela`, and optionally the crate where the item is defined.
///
/// Two forms are supported:
/// - `= path` marks items defined directly in `devela`
/// - `path` marks items defined in another crate and re-exported by `devela`
///
/// NOTE: It's important NOT to pass a leading slash in `$path` for the URL to work.
// NOTE: duplicated (not symlinked) in /crates/devela_macros/src/core_bridge/_doc_location.rs)
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
#[allow(clippy::crate_in_macro_def, reason = "to invoke __crate_name from crate of invocation")]
macro_rules! _doc_location {
    // TEMP VERSION WAIT until crate _doc_meta refactors are finished
    ($path:literal) => {
        concat!(
            "\n\n---\n\n", // TEMP
            "<sup title='location in `devela`'>",
            "📍 [`", $path, "`](",
            $crate::doclink![custom devela $path @mod],
            ")</sup>",
            "\n\n---\n\n" // TEMP
        )
    };
    // WAIT for crate refactors
    // for items defined in a workspace crate and aggregated in devela.
    (%from_meta $path:literal) => {
        // VERSION more useful for all definitions in a single crate
        concat!(
            "<sup title='location in `devela`'>",
            "📍 [`", $path, "`](",
            $crate::doclink![custom devela $path @mod],
            ")</sup>"
        )
    };
    // for items defined in a proc-macro workspace crate and aggregated in devela.
    // NOTE: this macro and doclink! has to be copied there without #[macro_export].
    (proc $path:literal) => {
        concat!(
            "<sup title='procedural macro location in `devela`'>",
            "📍 [`", $path, "`](",
            $crate::doclink![custom devela $path @mod],
            ")</sup>"
        )
    };
    // for items re-exported from another crate.
    // called from the _reexport! macro, does not end with \n\n
    (re-exported $path:literal) => {
        concat!(
            "\n\n---\n\n", // TEMP
            "<sup title='re-exported from `", crate::__crate_name!(),
            "`'>[`📍`](", $crate::doclink![custom_current_crate $path, @mod], ")</sup>",
            "<sup title='location in `devela`'><b>[`", $path,
            "`](", $crate::doclink![custom devela $path @mod], ")</b></sup>",
        )
    };
    (%from_meta re-exported $path:literal) => {
        concat!(
            "<sup title='re-exported from `", crate::__crate_name!(),
            "`'>[`📍`](", $crate::doclink![custom_current_crate $path, @mod], ")</sup>",
            "<sup title='location in `devela`'><b>[`", $path,
            "`](", $crate::doclink![custom devela $path @mod], ")</b></sup>",
        )
    };
}
#[doc(inline)]
pub use _doc_location;

#[doc = crate::_tags!(internal)]
/// Generates a formatted documentation string for a miri warning.
#[doc = crate::_doc_location!("yard")]
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

#[doc = crate::_tags!(internal)]
/// Shows the `Vendored` doc section and links to the info line.
#[doc = crate::_doc_location!("yard")]
///
/// See the documentation for [vendored work].
#[doc = crate::doclink!(custom devela "[vendored work]" "_doc/vendored" @mod)]
///
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
macro_rules! _doc_vendor· {
    (
    // Shows the `Vendored` doc section and links to the info line.
    //
    // $crate_id: the crate's name and html id anchor on the docs.
    $crate_id:literal) => {
        concat!("\n\n# Vendored\n\nThis is adapted work from [", $crate_id, "](",
            $crate::doclink![custom devela "_doc/vendored" @mod],
            "#", $crate_id, ").\n\n"
        )
    };
    // MAYBE
    // (
    // // Shows the `Vendored` doc section and links to the *modifications* module.
    // $crate_id:literal, module:$mod_id:ident) => { concat!(
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
    mod: $crate_id:literal, $mod_id:ident) => {
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
    // mod: $crate_id:literal, $text_path:literal) => { concat!(
    //     "# ", $crate_id, "\n\n[*(↑)*][crate::_doc::vendored#", $crate_id, "] ",
    //     include_str!($text_path),
    // )};
}
#[doc(inline)]
pub use _doc_vendor· as _doc_vendor;

#[doc = crate::_tags!(internal)]
/// Emits compact rustdoc metadata for a type's checked stack size.
#[doc = crate::_doc_meta!{location("yard")}]
///
/// The visible fragment shows the expected `size_of::<T>()` value.
/// A hidden doctest calls [`test_size_of!`][crate::test_size_of]
/// to verify the size during doctests.
///
/// # Forms
/// - `Type = N`: checks `devela::Type`.
/// - `Label: Type = N`: shows `Type` but uses `Label` as the metadata label.
/// - `abs Label: path::Type = N`: uses the type path exactly as written.
///
/// # Examples
/// ```ignore
/// #[doc = crate::_doc_size_of!(RasterFormat = 4)]
/// #[doc = crate::_doc_size_of!(PcmRawBuf_Slice: PcmRawBuf<&[u8]> = 24)]
/// #[doc = crate::_doc_size_of!(abs Local: crate::Local = 8)]
/// ```
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
macro_rules! _doc_size_of· {
    // Named public-root type:
    //
    // #[doc = _doc_size_of!(PcmRawBuf_Slice: PcmRawBuf<&[u8]> = 24)]
    ($ty:ty = $bytes:literal $(,)?) => {
        $crate::_doc_size_of!(@doc stringify!($ty),
            concat!("devela::", stringify!($ty)), stringify!($ty), stringify!($bytes))
    };
    // Unnamed public-root type:
    //
    // #[doc = _doc_size_of!(RasterFormat = 4)]
    ($name:ident : $ty:ty = $bytes:literal $(,)?) => {
        $crate::_doc_size_of!(@doc stringify!($name),
            concat!("devela::", stringify!($ty)), stringify!($ty), stringify!($bytes))
    };
    // Explicit path escape hatch:
    //
    // #[doc = _doc_size_of!(abs PcmRawBuf_Slice: crate::PcmRawBuf<&[u8]> = 24)]
    // #[doc = _doc_size_of!(abs PcmRawBuf_Slice: ::devela::PcmRawBuf<&[u8]> = 24)]
    (abs $name:ident : $ty:ty = $bytes:literal $(,)?) => {
        $crate::_doc_size_of!(@doc stringify!($name),
            stringify!($ty), stringify!($ty), stringify!($bytes))
    };
    // Core doc emitter.
    (@doc $label:expr, $test_ty:expr, $shown_ty:expr, $bytes:expr) => {
        concat!(
            " <sup title='stack size, checked by hidden doctest'>",
            // "📦 `", $label, ": size_of::<", $shown_ty, ">() == ", $bytes, "` bytes",
            "📦 `size_of::<", $shown_ty, ">() == ", $bytes, "` bytes", "</sup>\n\n",
            "<div hidden class='devela-hide-next'></div>\n\n",
            "```rust\n",
            "# devela::test_size_of!(assert ", $test_ty, " = ", $bytes, ");\n",
            "```\n",
        )
    };
}
#[doc(inline)]
pub use _doc_size_of· as _doc_size_of;
