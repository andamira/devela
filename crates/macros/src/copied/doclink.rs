// devela_macros/src/copied/doclink.rs
//
//! Recreates devela's `doclink!` macro without `#[macro_export]`.
//

#![allow(unused)]

/// Custom domain used for the [`doclink!`] macro.
macro_rules! _DOCLINK_CUSTOM_DOMAIN {
    () => {
        "https://docs.rs/"
        // "https://andamira.github.io/"
    };
}
pub(crate) use _DOCLINK_CUSTOM_DOMAIN;

/// Helps doc-linking items in downstream crates.
/// <!-- (*internet* version) -->
#[cfg(feature = "__publish")]
#[allow(clippy::crate_in_macro_def, reason = "crate::__crate_name! is intended")]
macro_rules! doclink· {
    /* Rustdoc item filenames */

    (@item_file struct $item:ident) => {
        ::core::concat!("struct.", ::core::stringify!($item), ".html")
    };
    (@item_file enum $item:ident) => {
        ::core::concat!("enum.", ::core::stringify!($item), ".html")
    };
    (@item_file union $item:ident) => {
        ::core::concat!("union.", ::core::stringify!($item), ".html")
    };
    (@item_file trait $item:ident) => {
        ::core::concat!("trait.", ::core::stringify!($item), ".html")
    };
    (@item_file type $item:ident) => {
        ::core::concat!("type.", ::core::stringify!($item), ".html")
    };
    (@item_file fn $item:ident) => {
        ::core::concat!("fn.", ::core::stringify!($item), ".html")
    };
    (@item_file const $item:ident) => {
        ::core::concat!("constant.", ::core::stringify!($item), ".html")
    };
    (@item_file static $item:ident) => {
        ::core::concat!("static.", ::core::stringify!($item), ".html")
    };
    (@item_file macro $item:ident) => {
        ::core::concat!("macro.", ::core::stringify!($item), ".html")
    };
    (@item_file attr $item:ident) => {
        ::core::concat!("attr.", ::core::stringify!($item), ".html")
    };
    (@item_file derive $item:ident) => {
        ::core::concat!("derive.", ::core::stringify!($item), ".html")
    };
    /* item links */
    (
     /* links to either a custom domain or a local URL */

     // [anchor]: https://…/crate/module_path/struct.Item.html
     custom crate $anchor:literal $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            custom crate $module_path @item $kind $item $($jump_link)? ]
        ]
    };
    (
     // https://…/crate/module_path/struct.Item.html
     custom crate $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ $crate::doclink![custom crate $module_path], "/",
            $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    (
     // [anchor]: https://…/crate_name/module_path/struct.Item.html
     custom $crate_name:ident $anchor:literal $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            custom $crate_name $module_path @item $kind $item $($jump_link)? ] ]
    };
    (
     // https://…/crate_name/module_path/struct.Item.html
     custom $crate_name:ident $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ $crate::doclink![custom $crate_name $module_path],
            "/", $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    (
     // https://…/{crate::__crate_name!()}/module_path/struct.Item.html
     custom_current_crate $module_path:expr,
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ $crate::doclink![custom_current_crate $module_path,],
            "/", $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    (
     // https://…/{crate::__crate_name!()}/struct.Item.html
     custom_current_proc_crate
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ crate::_DOCLINK_CUSTOM_DOMAIN!(),
            crate::__crate_name!(), "/latest/", crate::__crate_name!(), "/",
            $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    /* existing module/path links */
    (
     // [anchor]: https://…/crate/item_path
     // [anchor]: file://…/current_crate/item_path/index.html
     custom crate $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            custom crate $item_path $(@mod$($_m)?)? $($jump_link)? ] ]
    };
    (
     // https://…/crate/item_path
     // file://…/current_crate/item_path/index.html
     custom crate $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ crate::_DOCLINK_CUSTOM_DOMAIN!(), crate::__crate_name!(),
            env!("CARGO_CRATE_NAME"), "/latest/", crate::__crate_name!(), "/",
            $item_path $(, $jump_link)? ]
    };
    (
     // [anchor]: https://…/crate_name/item_path
     // [anchor]: file://…/crate_name/item_path/index.html
     custom $crate_name:ident $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            custom $crate_name $item_path $(@mod$($_m)?)? $($jump_link)? ] ]
    };
    (
     // https://…/crate_name/item_path
     // file://…/crate_name/item_path/index.html
     custom $crate_name:ident $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ crate::_DOCLINK_CUSTOM_DOMAIN!(), ::core::stringify!($crate_name),
            "/latest/", ::core::stringify!($crate_name), "/", $item_path $(, $jump_link)? ]
    };
    (
     // https://…/{crate::__crate_name!()}/item_path
     // file://…/{crate::__crate_name!()}/item_path/index.html
     custom_current_crate $item_path:expr,
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ crate::_DOCLINK_CUSTOM_DOMAIN!(), crate::__crate_name!(),
            "/latest/", crate::__crate_name!(), "/", $item_path $(, $jump_link)? ]
    };
    (
     // https://…/{crate::__crate_name!()}/
     // file://…/{crate::__crate_name!()}/index.html
     custom_current_proc_crate
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ crate::_DOCLINK_CUSTOM_DOMAIN!(), crate::__crate_name!(),
            "/latest/", crate::__crate_name!(), "/" $(, $jump_link)? ]
    };
    /* item links */
    (
    /* links to either docs.rs or a local URL */

     // [anchor]: https://docs.rs/…/crate/module_path/struct.Item.html
     crate $anchor:literal $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            crate $module_path @item $kind $item $($jump_link)? ] ]
    };
    (
     // https://docs.rs/…/crate/module_path/struct.Item.html
     crate $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ $crate::doclink![crate $module_path], "/",
            $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    (
     // [anchor]: https://docs.rs/…/crate_name/module_path/struct.Item.html
     $crate_name:ident $anchor:literal $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            $crate_name $module_path @item $kind $item $($jump_link)? ] ]
    };
    (
     // https://docs.rs/…/crate_name/module_path/struct.Item.html
     $crate_name:ident $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ $crate::doclink![$crate_name $module_path], "/",
            $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    (
     // https://docs.rs/…/{crate::__crate_name!()}/module_path/struct.Item.html
     current_crate $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ $crate::doclink![current_crate $module_path], "/",
            $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    (
     // https://docs.rs/…/{crate::__crate_name!()}/struct.Item.html
     current_proc_crate
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ "https://docs.rs/", crate::__crate_name!(), "/latest/",
            crate::__crate_name!(), "/", $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    /* existing module/path links */
    (
     // [anchor]: https://…/crate/item_path
     // [anchor]: file://…/current_crate/item_path/index.html
     crate $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            crate $item_path $(@mod$($_m)?)? $($jump_link)? ] ]
    };
    (
     // https://…/crate/item_path
     // file://…/current_crate/item_path/index.html
     crate $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "https://docs.rs/", crate::__crate_name!(), env!("CARGO_CRATE_NAME"),
            "/latest/", crate::__crate_name!(), "/", $item_path $(, $jump_link)? ]
    };
    (
     // [anchor]: https://…/crate_name/item_path
     // [anchor]: file://…/crate_name/item_path/index.html
     $crate_name:ident $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            $crate_name $item_path $(@mod$($_m)?)? $($jump_link)? ] ]
    };
    (
     // https://…/crate_name/item_path
     // file://…/crate_name/item_path/index.html
     $crate_name:ident $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "https://docs.rs/", ::core::stringify!($crate_name), "/latest/",
            ::core::stringify!($crate_name), "/", $item_path $(, $jump_link)? ]
    };
    (
     // https://…/{crate::__crate_name!()}/item_path
     // file://…/{crate::__crate_name!()}/item_path/index.html
     current_crate $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "https://docs.rs/", crate::__crate_name!(), "/latest/",
            crate::__crate_name!(), "/", $item_path $(, $jump_link)? ]
    };
    (
     // https://…/{crate::__crate_name!()}/
     // file://…/{crate::__crate_name!()}/index.html
     current_proc_crate
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "https://docs.rs/", crate::__crate_name!(), "/latest/",
            crate::__crate_name!(), "/" $(, $jump_link)? ]
    };
}

/// Helps doc-linking items in downstream crates.
/// (*local* version)
#[cfg(not(feature = "__publish"))]
#[allow(clippy::crate_in_macro_def, reason = "crate::__crate_name! is intended")]
macro_rules! doclink· {
    /* Rustdoc item filenames */

    (@item_file struct $item:ident) => {
        ::core::concat!("struct.", ::core::stringify!($item), ".html")
    };
    (@item_file enum $item:ident) => {
        ::core::concat!("enum.", ::core::stringify!($item), ".html")
    };
    (@item_file union $item:ident) => {
        ::core::concat!("union.", ::core::stringify!($item), ".html")
    };
    (@item_file trait $item:ident) => {
        ::core::concat!("trait.", ::core::stringify!($item), ".html")
    };
    (@item_file type $item:ident) => {
        ::core::concat!("type.", ::core::stringify!($item), ".html")
    };
    (@item_file fn $item:ident) => {
        ::core::concat!("fn.", ::core::stringify!($item), ".html")
    };
    (@item_file const $item:ident) => {
        ::core::concat!("constant.", ::core::stringify!($item), ".html")
    };
    (@item_file static $item:ident) => {
        ::core::concat!("static.", ::core::stringify!($item), ".html")
    };
    (@item_file macro $item:ident) => {
        ::core::concat!("macro.", ::core::stringify!($item), ".html")
    };
    (@item_file attr $item:ident) => {
        ::core::concat!("attr.", ::core::stringify!($item), ".html")
    };
    (@item_file derive $item:ident) => {
        ::core::concat!("derive.", ::core::stringify!($item), ".html")
    };
    /* item links */
    (
     /* links to either a custom domain or a local URL */

     custom crate $anchor:literal $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            custom crate $module_path @item $kind $item $($jump_link)? ] ]
    };
    (
     custom crate $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ $crate::doclink![custom crate $module_path], "/",
            $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    (
     custom $crate_name:ident $anchor:literal $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            custom $crate_name $module_path @item $kind $item $($jump_link)? ] ]
    };
    (
     custom $crate_name:ident $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ $crate::doclink![custom $crate_name $module_path], "/",
            $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    (
     custom_current_crate $module_path:expr,
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ $crate::doclink![custom_current_crate $module_path,],
            "/", $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    (
     custom_current_proc_crate
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ "file://", env!("CARGO_TARGET_DIR"), "doc/", crate::__crate_name!(),
            "/", $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    /* existing module/path links */
    (
     // [anchor]: file://…/current_crate/item_path/index.html
     custom crate $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            custom crate $item_path $(@mod$($_m)?)? $($jump_link)? ] ]
    };
    (
     // file://…/current_crate/item_path/index.html
     custom crate $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "file://", env!("CARGO_TARGET_DIR"), "doc/", env!("CARGO_CRATE_NAME"),
            "/", $item_path $(, "/index.html"$($_m)?)? $(, $jump_link)? ]
    };
    (
     // [anchor]: file://…/crate_name/item_path/index.html
     custom $crate_name:ident $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            custom $crate_name $item_path $(@mod$($_m)?)? $($jump_link)? ] ]
    };
    (
     // file://…/crate_name/item_path/index.html
     custom $crate_name:ident $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "file://", env!("CARGO_TARGET_DIR"), "doc/",
        ::core::stringify!($crate_name), "/", $item_path
            $(, "/index.html"$($_m)?)? $(, $jump_link)? ]
    };
    (
     // file://…/{crate::__crate_name!()}/item_path/index.html
     custom_current_crate $item_path:expr,
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "file://", env!("CARGO_TARGET_DIR"), "doc/", crate::__crate_name!(),
            "/", $item_path $(, "/index.html"$($_m)?)? $(, $jump_link)? ]
    };
    (
     // file://…/{crate::__crate_name!()}/index.html
     custom_current_proc_crate
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "file://", env!("CARGO_TARGET_DIR"), "doc/", crate::__crate_name!()
            $(, "/index.html"$($_m)?)? $(, $jump_link)? ]
    };
    /* item links */
    (
     /* links to either docs.rs or a local URL */

     crate $anchor:literal $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            crate $module_path @item $kind $item $($jump_link)? ] ]
    };
    (
     crate $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ $crate::doclink![crate $module_path], "/",
            $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    (
     $crate_name:ident $anchor:literal $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            $crate_name $module_path @item $kind $item $($jump_link)? ] ]
    };
    (
     $crate_name:ident $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ $crate::doclink![$crate_name $module_path], "/",
            $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    (
     current_crate $module_path:literal
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ $crate::doclink![current_crate $module_path], "/",
            $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    (
     current_proc_crate
     @item $kind:ident $item:ident $($jump_link:literal)?) => {
        ::core::concat![ "file://", env!("CARGO_TARGET_DIR"), "doc/", crate::__crate_name!(), "/",
            $crate::doclink![@item_file $kind $item] $(, $jump_link)? ]
    };
    /* existing module/path links */
    (
     // [anchor]: file://…/current_crate/item_path/index.html
     crate $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            crate $item_path $(@mod$($_m)?)? $($jump_link)? ] ]
    };
    (
     // file://…/current_crate/item_path/index.html
     crate $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "file://", env!("CARGO_TARGET_DIR"), "doc/", env!("CARGO_CRATE_NAME"),
            "/", $item_path $(, "/index.html"$($_m)?)? $(, $jump_link)? ]
    };
    (
     // [anchor]: file://…/crate_name/item_path/index.html
     $crate_name:ident $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "\n\n", $anchor, ": ", $crate::doclink![
            $crate_name $item_path $(@mod$($_m)?)? $($jump_link)? ] ]
    };
    (
     // file://…/crate_name/item_path/index.html
     $crate_name:ident $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "file://", env!("CARGO_TARGET_DIR"), "doc/",
        ::core::stringify!($crate_name), "/", $item_path
            $(, "/index.html"$($_m)?)? $(, $jump_link)? ]
    };
    (
     // file://…/{crate::__crate_name!()}/item_path/index.html
     current_crate $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "file://", env!("CARGO_TARGET_DIR"), "doc/", crate::__crate_name!(),
            "/", $item_path $(, "/index.html"$($_m)?)? $(, $jump_link)? ]
    };
    (
     // file://…/{crate::__crate_name!()}/index.html
     current_proc_crate
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![ "file://", env!("CARGO_TARGET_DIR"), "doc/", crate::__crate_name!()
            $(, "/index.html"$($_m)?)? $(, $jump_link)? ] };
}

pub(crate) use doclink· as doclink;
