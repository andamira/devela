// devela_base_macros::core_bridge::_doc_location
//
//! Recreates devela_base_core's `doclink!` macro without `#[macro_export]`.
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
macro_rules! _doclink {
    (
     /* links to either a custom domain or a local URL */

     // [anchor]: https://…/crate/item_path
     // [anchor]: file://…/current_crate/item_path/index.html
     custom crate $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["\n\n", $anchor, ": ",
        $crate::doclink![custom crate $item_path $(@mod$($_m)?)? $($jump_link)?]]
    };
    ( // https://…/crate/item_path
      // file://…/current_crate/item_path/index.html
     custom crate $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![crate::_DOCLINK_CUSTOM_DOMAIN!(),
        crate::__crate_name!(), env!("CARGO_CRATE_NAME"),
        "/latest/", crate::__crate_name!(), "/", $item_path $(, $jump_link)?]
    };
    (
     // [anchor]: https://…/crate_name/item_path
     // [anchor]: file://…/crate_name/item_path/index.html
     custom $crate_name:ident $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["\n\n", $anchor, ": ",
        $crate::doclink![custom $crate_name $item_path $(@mod$($_m)?)? $($jump_link)?]]
    };
    ( // https://…/crate_name/item_path
      // file://…/crate_name/item_path/index.html
     custom $crate_name:ident $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![crate::_DOCLINK_CUSTOM_DOMAIN!(), ::core::stringify!($crate_name),
        "/latest/", ::core::stringify!($crate_name), "/", $item_path $(, $jump_link)?]
    };
    ( // https://…/{crate::__crate_name!()}/item_path
      // file://…/{crate::__crate_name!()}/item_path/index.html
     custom_current_crate $item_path:expr,
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat![crate::_DOCLINK_CUSTOM_DOMAIN!(), crate::__crate_name!(),
        "/latest/", crate::__crate_name!(), "/", $item_path $(, $jump_link)?]
    };
    ( // https://…/{crate::__crate_name!()}/
      // file://…/{crate::__crate_name!()}/index.html
     custom_current_proc_crate
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        concat![crate::_DOCLINK_CUSTOM_DOMAIN!(), crate::__crate_name!(),
        "/latest/", crate::__crate_name!(), "/" $(, $jump_link)?]
    };
    (
     /* links to either docs.rs or a local URL */

     // [anchor]: https://…/crate/item_path
     // [anchor]: file://…/current_crate/item_path/index.html
     crate $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["\n\n", $anchor, ": ",
        $crate::doclink![crate $item_path $(@mod$($_m)?)? $($jump_link)?]]
    };
    ( // https://…/crate/item_path
      // file://…/current_crate/item_path/index.html
     crate $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["https://docs.rs/",
        crate::__crate_name!(), env!("CARGO_CRATE_NAME"),
        "/latest/", crate::__crate_name!(), "/", $item_path $(, $jump_link)?]
    };
    (
     // [anchor]: https://…/crate_name/item_path
     // [anchor]: file://…/crate_name/item_path/index.html
     $crate_name:ident $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["\n\n", $anchor, ": ",
        $crate::doclink![$crate_name $item_path $(@mod$($_m)?)? $($jump_link)?]]
    };
    ( // https://…/crate_name/item_path
      // file://…/crate_name/item_path/index.html
     $crate_name:ident $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["https://docs.rs/", ::core::stringify!($crate_name),
        "/latest/", ::core::stringify!($crate_name), "/", $item_path $(, $jump_link)?]
    };
    ( // https://…/{crate::__crate_name!()}/item_path
      // file://…/{crate::__crate_name!()}/item_path/index.html
     current_crate $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["https://docs.rs/", crate::__crate_name!(),
        "/latest/", crate::__crate_name!(), "/", $item_path $(, $jump_link)?]
    };
    ( // https://…/{crate::__crate_name!()}/
      // file://…/{crate::__crate_name!()}/index.html
     current_proc_crate
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        concat!["https://docs.rs/", crate::__crate_name!(),
        "/latest/", crate::__crate_name!(), "/" $(, $jump_link)?]
    };
}

/// Helps doc-linking items in downstream crates.
/// (*local* version)
#[cfg(not(feature = "__publish"))]
#[allow(clippy::crate_in_macro_def, reason = "crate::__crate_name! is intended")]
macro_rules! _doclink {
    (
     /* links to either a custom domain or a local URL */

     // [anchor]: file://…/current_crate/item_path/index.html
     custom crate $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["\n\n", $anchor, ": ",
        $crate::doclink![custom crate $item_path $(@mod$($_m)?)? $($jump_link)?]]
    };
    ( // file://…/current_crate/item_path/index.html
     custom crate $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        env!("CARGO_CRATE_NAME"), "/", $item_path $(, "/index.html"$($_m)?)? $(, $jump_link)?]
    };
    (
     // [anchor]: file://…/crate_name/item_path/index.html
     custom $crate_name:ident $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["\n\n", $anchor, ": ",
        $crate::doclink![custom $crate_name $item_path $(@mod$($_m)?)? $(, $jump_link)?]]
    };
    ( // file://…/crate_name/item_path/index.html
     custom $crate_name:ident $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        ::core::stringify!($crate_name), "/", $item_path $(, "/index.html"$($_m)?)?
            $(, $jump_link)?]
    };
    ( // file://…/{crate::__crate_name!()}/item_path/index.html
     custom_current_crate $item_path:expr,
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        crate::__crate_name!(), "/", $item_path $(, "/index.html"$($_m)?)? $(, $jump_link)?]
    };
    ( // file://…/{crate::__crate_name!()}/index.html
     custom_current_proc_crate
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        crate::__crate_name!(), $("/index.html"$($_m)?)? $(, $jump_link)?]
    };
    (
     /* links to either docs.rs or a local URL */

     // [anchor]: file://…/current_crate/item_path/index.html
     crate $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["\n\n", $anchor, ": ",
        $crate::doclink![crate $item_path $(@mod$($_m)?)? $($jump_link)?]]
    };
    ( // file://…/current_crate/item_path/index.html
     crate $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        env!("CARGO_CRATE_NAME"), "/", $item_path $(, "/index.html"$($_m)?)? $(, $jump_link)?]
    };
    (
     // [anchor]: file://…/crate_name/item_path/index.html
     $crate_name:ident $anchor:literal $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["\n\n", $anchor, ": ",
        $crate::doclink![$crate_name $item_path $(@mod$($_m)?)? $($jump_link)?]]
    };
    ( // file://…/crate_name/item_path/index.html
     $crate_name:ident $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        ::core::stringify!($crate_name), "/", $item_path $(, "/index.html"$($_m)?)?
            $(, $jump_link)?]
    };
    ( // file://…/{crate::__crate_name!()}/item_path/index.html
     current_crate $item_path:literal
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        ::core::concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        crate::__crate_name!(), "/", $item_path $(, "/index.html"$($_m)?)? $(, $jump_link)?]
    };
    ( // file://…/{crate::__crate_name!()}/index.html
     current_proc_crate
     $(@mod$($_m:lifetime)?)? $($jump_link:literal)?) => {
        concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        crate::__crate_name!(), $("/index.html"$($_m)?)? $(, $jump_link)? $(, $jump_link)?]
    };
}

pub(crate) use _doclink as doclink;
