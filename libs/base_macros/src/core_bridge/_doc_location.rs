// devela_base_macros::core_bridge::_doc_location
//
//! Recreates devela_base_core's `_doc_location!` macro without #[macro_export].
//

/// Emits a location annotation for documentation.
///
/// This macro renders a small location marker (`📍`) followed by the public
/// API path under `devela`, and optionally the crate where the item is defined.
///
/// Two forms are supported:
/// - `= path` marks items defined directly in `devela`
/// - `path` marks items defined in another crate and re-exported by `devela`
///
/// NOTE: It's important NOT to pass a leading slash in `$path` for the URL to work.
macro_rules! _doc_location {
    // for items defined in a non-proc-macro workspace crate and aggregated in devela.
    ($path:literal) => {
        concat!(
            "\n\n---\n\n<sup title='defined in ", __crate_name!(),
            "`'>[`📍`](", $crate::doclink![custom_current_crate $path @mod], ")</sup>",
            "<sup title='location in `devela`'><b>[`", $path,
            "`](", $crate::doclink![custom devela $path @mod], ")</b></sup>\n\n",
        )
    };
    // for items defined in a proc-macro workspace crate and aggregated in devela.
    // NOTE: this macro and doclink! has to be copied there without #[macro_export].
    (proc $path:literal) => {
        concat!(
            "\n\n---\n\n<sup title='defined in ", __crate_name!(),
            "`'>[`📍`](", $crate::doclink![custom_current_proc_crate @mod], ")</sup>",
            "<sup title='location in `devela`'><b>[`", $path,
            "`](", $crate::doclink![custom devela $path @mod], ")</b></sup>\n\n",
        )
    };
    // for items re-exported from another crate.
    // called from the _reexport! macro, does not end with \n\n
    (re-exported $path:literal) => {
        concat!(
            "\n\n<sup title='re-exported from ", __crate_name!(),
            "`'>[`📍`](", $crate::doclink![custom_current_crate $path @mod], ")</sup>",
            "<sup title='location in `devela`'><b>[`", $path,
            "`](", $crate::doclink![custom devela $path @mod], ")</b></sup>",
        )
    };
}
pub(crate) use _doc_location;
