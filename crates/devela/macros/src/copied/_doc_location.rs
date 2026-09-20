//
//! Recreates devela's `_doc_location!` macro without `#[macro_export]`.
//
// NOTE: remove all '·' chars when coyping it from /crates/devela/src/yard/_doc/location.rs

/// Emits a location annotation for documentation.
///
/// The annotation links both to the defining workspace crate and to the
/// corresponding public location under `devela`.
///
/// An exact item may optionally be supplied as `kind Item`.
///
/// # Forms
/// - `"path"`: links to the containing module.
/// - `"path", kind Item`: also links to the exact item.
/// - `proc "path"`: location for an item defined by this proc-macro crate.
/// - `proc "path", kind Item`: also links to the exact proc-macro item.
/// - `re-exported "path"`: location for an item re-exported from another crate.
///
/// Supported item kinds are:
/// `struct`, `enum`, `union`, `trait`, `type`, `fn`, `const`, `static`,
/// `macro`, `attr`, and `derive`.
///
/// The path must not begin with `/`.
macro_rules! _doc_location {
    /* direct forms --------------------------------------------------------- */

    // Ordinary item whose public location is in the current crate.
    ($path:literal) => {
        concat!(
            "\n\n---\n\n",
            $crate::_doc_location!(%from_meta $path),
            "\n\n---\n\n"
        )
    };
    ($path:literal, $kind:ident $item:ident) => {
        concat!(
            "\n\n---\n\n",
            $crate::_doc_location!(%from_meta $path, $kind $item),
            "\n\n---\n\n"
        )
    };

    // Procedural macro whose public location is in the current crate.
    (proc $path:literal) => {
        concat!(
            "\n\n---\n\n",
            $crate::_doc_location!(%from_meta proc $path),
            "\n\n---\n\n"
        )
    };
    (proc $path:literal, $kind:ident $item:ident) => {
        concat!(
            "\n\n---\n\n",
            $crate::_doc_location!(%from_meta proc $path, $kind $item),
            "\n\n---\n\n"
        )
    };

    // Procedural macro defined at the root of the current proc-macro crate,
    // with its public location in another crate.
    (proc in $target:ident $path:literal) => {
        concat!(
            "\n\n---\n\n",
            $crate::_doc_location!(%from_meta proc in $target $path),
            "\n\n---\n\n"
        )
    };
    (proc in $target:ident $path:literal, $kind:ident $item:ident) => {
        concat!(
            "\n\n---\n\n",
            $crate::_doc_location!(%from_meta proc in $target $path, $kind $item),
            "\n\n---\n\n"
        )
    };

    // Item re-exported from another crate.
    // Called from `_reexport!`; deliberately leaves the closing separator
    // to the surrounding metadata machinery.
    (re-exported $path:literal) => {
        concat!(
            "\n\n---\n\n",
            $crate::_doc_location!(%from_meta re-exported $path)
        )
    };
    (re-exported $path:literal, $kind:ident $item:ident) => {
        concat!(
            "\n\n---\n\n",
            $crate::_doc_location!(%from_meta re-exported $path, $kind $item)
        )
    };

    /* `_doc_meta!` fragments ---------------------------------------------- */

    // Ordinary module location in the current crate.
    (%from_meta $path:literal) => {
        concat!(
            "<sup class='_doc_location' title='location in `",
            env!("CARGO_PKG_NAME"), "`'>", "📍 [`", $path, "`](",
            $crate::doclink![custom_current_crate $path, @mod],
            ")</sup>"
        )
    };

    // Exact ordinary item location in the current crate.
    (%from_meta $path:literal, $kind:ident $item:ident) => {
        concat!(
            "<sup class='_doc_location' title='location in `",
            env!("CARGO_PKG_NAME"), "`'>", "📍 [`", $path, "`](",
            $crate::doclink![custom_current_crate $path, @mod],
            ")::[`", ::core::stringify!($item), "`](",
            $crate::doclink![custom_current_crate $path, @item $kind $item],
            ")</sup>"
        )
    };

    // Procedural macro whose public location is in the current crate.
    (%from_meta proc $path:literal) => {
        concat!(
            "<sup class='_doc_location' title='procedural macro location in `",
            env!("CARGO_PKG_NAME"), "`'>", "📍 [`", $path, "`](",
            $crate::doclink![custom_current_crate $path, @mod],
            ")</sup>"
        )
    };

    (%from_meta proc $path:literal, $kind:ident $item:ident) => {
        concat!(
            "<sup class='_doc_location' title='procedural macro location in `",
            env!("CARGO_PKG_NAME"), "`'>", "📍 [`", $path, "`](",
            $crate::doclink![custom_current_crate $path, @mod],
            ")::[`", ::core::stringify!($item), "`](",
            $crate::doclink![custom_current_crate $path, @item $kind $item],
            ")</sup>"
        )
    };

    // Procedural macro defined at the root of the current proc-macro crate,
    // with its public location in another crate.
    (%from_meta proc in $target:ident $path:literal) => {
        concat!(
            "<sup title='defined in `", env!("CARGO_PKG_NAME"), "`'>",
            "📍 [`", env!("CARGO_PKG_NAME"), "`](",
            $crate::doclink![custom_current_proc_crate @mod],
            ")</sup>",

            "<sup> → </sup>",

            "<sup class='_doc_location' title='public location in `",
            ::core::stringify!($target), "`'><b>", "[`", $path, "`](",
            $crate::doclink![custom $target $path @mod],
            ")</b></sup>"
        )
    };
    (%from_meta proc in $target:ident $path:literal, $kind:ident $item:ident) => {
        concat!(
            // Definition in the current proc-macro crate.
            "<sup title='defined in `", env!("CARGO_PKG_NAME"), "`'>",
            "📍 [`", env!("CARGO_PKG_NAME"), "`](",
            $crate::doclink![custom_current_proc_crate @mod],
            ")::[`", ::core::stringify!($item), "`](",
            $crate::doclink![custom_current_proc_crate @item $kind $item],
            ")</sup>",

            "<sup> → </sup>",

            // Public location in the target crate.
            "<sup class='_doc_location' title='public location in `",
            ::core::stringify!($target), "`'><b>", "[`", $path, "`](",
            $crate::doclink![custom $target $path @mod],
            ")::[`", ::core::stringify!($item), "`](",
            $crate::doclink![custom $target $path @item $kind $item],
            ")</b></sup>"
        )
    };

    // Re-export location in the current crate.
    (%from_meta re-exported $path:literal) => {
        concat!(
            "<sup title='re-exported in `", env!("CARGO_PKG_NAME"), "`'>[`📍`](",
            $crate::doclink![custom_current_crate $path, @mod],
            ")</sup>",
            "<sup class='_doc_location' title='location in `", env!("CARGO_PKG_NAME"),
            "`'><b>", "[`", $path, "`](",
            $crate::doclink![custom_current_crate $path, @mod],
            ")</b></sup>"
        )
    };

    (%from_meta re-exported $path:literal, $kind:ident $item:ident) => {
        concat!(
            "<sup title='re-exported in `", env!("CARGO_PKG_NAME"), "`'>[`📍`](",
            $crate::doclink![custom_current_crate $path, @item $kind $item],
            ")</sup>",
            "<sup class='_doc_location' title='location in `", env!("CARGO_PKG_NAME"),
            "`'><b>", "[`", $path, "`](",
            $crate::doclink![custom_current_crate $path, @mod],
            ")::[`", ::core::stringify!($item), "`](",
            $crate::doclink![custom_current_crate $path, @item $kind $item],
            ")</b></sup>"
        )
    };
}
pub(crate) use _doc_location;
