// devela::yard::_doc::location
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
