// devela/src/yard/_doc/location.rs
//
//! Defines [`_doc_location!`].
//

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
#[doc = crate::_doc_meta!{
    location("yard", macro _doc_location),
}]
/// Location is usually the nearest publicly documented parent,
/// instead of the conceptual private source module of origin.
///
/// The annotation renders a small location marker (`📍`) followed by the
/// public API path under `devela`.
///
/// An exact item may optionally be supplied as `kind Item`. In that form,
/// the module path and item name are linked independently.
///
/// # Forms
/// - `"path"`: links to the containing module.
/// - `"path", kind Item`: also links to the exact item.
/// - `proc "path"`: location for an item defined by the proc-macro crate.
/// - `proc "path", kind Item`: proc-macro location including the exact item.
/// - `re-exported "path"`: location for an item re-exported from another crate.
///
/// Supported item kinds are:
/// `struct`, `enum`, `union`, `trait`, `type`, `fn`, `const`, `static`,
/// `macro`, `attr`, and `derive`.
///
/// The path must not begin with `/`.
///
/// NOTE: It's important NOT to pass a leading slash in `$path` for the URL to work.
// NOTE: duplicated (not symlinked) in /crates/devela_macros/src/core_bridge/_doc_location.rs)
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(not(feature = "__docs_internal"), doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "__docs_internal")))]
#[macro_export]
#[allow(clippy::crate_in_macro_def, reason = "to invoke __crate_name from crate of invocation")]
macro_rules! _doc_location {
    /* direct forms --------------------------------------------------------- */

    // for an item defined in devela.
    ($path:literal) => {
        concat!(
            "\n\n---\n\n", // TEMP
            $crate::_doc_location!(%from_meta $path),
            "\n\n---\n\n" // TEMP
        )
    };
    // for a specific item defined in devela.
    ($path:literal, $kind:ident $item:ident) => {
        concat!(
            "\n\n---\n\n", // TEMP
            $crate::_doc_location!(%from_meta $path, $kind $item),
            "\n\n---\n\n" // TEMP
        )
    };
    // for an item defined in the proc-macro workspace crate
    // and aggregated in devela.
    (proc $path:literal) => {
        concat!(
            "\n\n---\n\n", // TEMP
            $crate::_doc_location!(%from_meta proc $path),
            "\n\n---\n\n" // TEMP
        )
    };
    // for a specific item defined in the proc-macro workspace crate
    // and aggregated in devela.
    (proc $path:literal, $kind:ident $item:ident) => {
        concat!(
            "\n\n---\n\n", // TEMP
            $crate::_doc_location!(%from_meta proc $path, $kind $item),
            "\n\n---\n\n" // TEMP
        )
    };
    // for items re-exported from another crate.
    // Called from `_reexport!`; deliberately does not end with the closing separator.
    (re-exported $path:literal) => {
        concat!(
            "\n\n---\n\n", // TEMP
            $crate::_doc_location!(%from_meta re-exported $path)
        )
    };
    // same, with an exact item.
    (re-exported $path:literal, $kind:ident $item:ident) => {
        concat!(
            "\n\n---\n\n", // TEMP
            $crate::_doc_location!(%from_meta re-exported $path, $kind $item)
        )
    };
    // re-export location fragment.
    (%from_meta re-exported $path:literal) => {
        concat!(
            "<sup title='re-exported from `", crate::__crate_name!(),
            "`'>[`📍`](",
            $crate::doclink·![custom_current_crate $path, @mod],
            ")</sup>",
            "<sup class='_doc_location' title='location in `devela`'><b>",
            "[`", $path, "`](",
            $crate::doclink·![custom devela $path @mod],
            ")</b></sup>",
        )
    };
    // exact re-export location fragment.
    (%from_meta re-exported $path:literal, $kind:ident $item:ident) => {
        concat!(
            "<sup title='re-exported from `", crate::__crate_name!(),
            "`'>[`📍`](",
            $crate::doclink·![custom_current_crate $path, @item $kind $item],
            ")</sup>",
            "<sup class='_doc_location' title='location in `devela`'><b>",
            "[`", $path, "`](",
            $crate::doclink·![custom devela $path @mod],
            ")::[`", ::core::stringify!($item), "`](",
            $crate::doclink·![custom devela $path @item $kind $item],
            ")</b></sup>",
        )
    };

    /* `_doc_meta!` fragments ---------------------------------------------- */

    // module location in devela.
    (%from_meta $path:literal) => {
        concat!(
            "<sup class='_doc_location' title='location in `devela`'>",
            "📍 [`", $path, "`](",
            // NOTE: Use the exported definition directly here: resolving the `doclink!`
            // re-export can get stuck during early `#[doc = ...]` expansion.
            $crate::doclink·![custom devela $path @mod],
            ")</sup>"
        )
    };
    // exact item location in devela.
    (%from_meta $path:literal, $kind:ident $item:ident) => {
        concat!(
            "<sup class='_doc_location' title='location in `devela`'>",
            "📍 [`", $path, "`](",
            $crate::doclink·![custom devela $path @mod],
            ")::[`", ::core::stringify!($item), "`](",
            $crate::doclink·![custom devela $path @item $kind $item],
            ")</sup>"
        )
    };
    // proc-macro module location in devela.
    (%from_meta proc $path:literal) => {
        concat!(
            "<sup class='_doc_location' title='procedural macro location in `devela`'>",
            "📍 [`", $path, "`](",
            $crate::doclink·![custom devela $path @mod],
            ")</sup>"
        )
    };
    // exact proc-macro item location in devela.
    (%from_meta proc $path:literal, $kind:ident $item:ident) => {
        concat!(
            "<sup class='_doc_location' title='procedural macro location in `devela`'>",
            "📍 [`", $path, "`](",
            $crate::doclink·![custom devela $path @mod],
            ")::[`", ::core::stringify!($item), "`](",
            $crate::doclink·![custom devela $path @item $kind $item],
            ")</sup>"
        )
    };
    // re-export location fragment.
    (%from_meta re-exported $path:literal) => {
        concat!(
            "<sup title='re-exported from `", crate::__crate_name!(),
            "`'>[`📍`](",
            $crate::doclink·![custom_current_crate $path, @mod],
            ")</sup>",
            "<sup class='_doc_location' title='location in `devela`'><b>",
            "[`", $path, "`](",
            $crate::doclink·![custom devela $path @mod],
            ")</b></sup>",
        )
    };
}
#[doc(inline)]
pub use _doc_location;
