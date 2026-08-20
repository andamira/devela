// devela_macros::copied::_doc_location
//
//! Recreates devela's `_doc_location!` macro without `#[macro_export]`.
//

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
    // for items defined in a non-proc-macro workspace crate and aggregated in devela.
    ($path:literal) => {
        concat!(
            "\n\n---\n\n",
            "<sup title='defined in `", crate::__crate_name!(), "`'>",
            "[`📍`](",
            $crate::doclink![custom_current_crate $path, @mod],
            ")</sup>",
            "<sup class='_doc_location' title='location in `devela`'><b>",
            "[`", $path, "`](",
            $crate::doclink![custom devela $path @mod],
            ")</b></sup>\n\n",
        )
    };
    // for a specific item defined in a non-proc-macro workspace crate
    // and aggregated in devela.
    ($path:literal, $kind:ident $item:ident) => {
        concat!(
            "\n\n---\n\n",
            "<sup title='defined in `", crate::__crate_name!(), "`'>",
            "[`📍`](",
            $crate::doclink![
                custom_current_crate $path,
                @item $kind $item
            ],
            ")</sup>",
            "<sup class='_doc_location' title='location in `devela`'><b>",
            "[`", $path, "`](",
            $crate::doclink![custom devela $path @mod],
            ")::[`", ::core::stringify!($item), "`](",
            $crate::doclink![custom devela $path @item $kind $item],
            ")</b></sup>\n\n",
        )
    };
    // for items defined in this proc-macro crate and aggregated in devela.
    (proc $path:literal) => {
        concat!(
            "\n\n---\n\n",
            "<sup title='defined in `", crate::__crate_name!(), "`'>",
            "[`📍`](",
            $crate::doclink![custom_current_proc_crate @mod],
            ")</sup>",
            "<sup class='_doc_location' title='location in `devela`'><b>",
            "[`", $path, "`](",
            $crate::doclink![custom devela $path @mod],
            ")</b></sup>\n\n",
        )
    };
    // for a specific item defined in this proc-macro crate
    // and aggregated in devela.
    (proc $path:literal, $kind:ident $item:ident) => {
        concat!(
            "\n\n---\n\n",
            "<sup title='defined in `", crate::__crate_name!(), "`'>",
            "[`📍`](",
            $crate::doclink![
                custom_current_proc_crate
                @item $kind $item
            ],
            ")</sup>",
            "<sup class='_doc_location' title='location in `devela`'><b>",
            "[`", $path, "`](",
            $crate::doclink![custom devela $path @mod],
            ")::[`", ::core::stringify!($item), "`](",
            $crate::doclink![custom devela $path @item $kind $item],
            ")</b></sup>\n\n",
        )
    };
    // for items re-exported from another crate.
    // Called from `_reexport!`; deliberately does not end with `\n\n`.
    (re-exported $path:literal) => {
        concat!(
            "\n\n",
            "<sup title='re-exported from `", crate::__crate_name!(), "`'>",
            "[`📍`](",
            $crate::doclink![custom_current_crate $path, @mod],
            ")</sup>",
            "<sup class='_doc_location' title='location in `devela`'><b>",
            "[`", $path, "`](",
            $crate::doclink![custom devela $path @mod],
            ")</b></sup>",
        )
    };
}
pub(crate) use _doc_location;
