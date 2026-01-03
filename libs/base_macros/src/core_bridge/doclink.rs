// devela_base_macros::core_bridge::_doc_location
//
//! Recreates devela_base_core's `doclink!` macro without #[macro_export].
//

#![allow(unused)]

/// Custom domain used for the [`doclink!`] macro.
#[rustfmt::skip]
macro_rules! _DOCLINK_CUSTOM_DOMAIN { () => { "https://andamira.github.io/" }; }
pub(crate) use _DOCLINK_CUSTOM_DOMAIN as DOCLINK_CUSTOM_DOMAIN;

/// Helps doc-linking items in downstream crates.
/// <!-- (*internet* version) -->
#[doc = crate::_doc_location!("code/util")]
#[cfg(feature = "__publish")]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _doclink {
    (
     /* links to either a custom domain or a local URL */

     // [anchor]: https://…/crate/item_path
     // [anchor]: file://…/current_crate/item_path/index.html
     custom crate $anchor:literal $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["\n\n", $anchor, ": ",
        $crate::doclink![custom crate $item_path $(@mod$($_m)?)?]]
    };
    ( // https://…/crate/item_path
      // file://…/current_crate/item_path/index.html
     custom crate $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat![crate::DOCLINK_CUSTOM_DOMAIN!(),
        stringify!($crate_name),env! ("CARGO_CRATE_NAME"),
        "/latest/", stringify!($crate_name), "/", $item_path]
    };
    (
     // [anchor]: https://…/crate_name/item_path
     // [anchor]: file://…/crate_name/item_path/index.html
     custom $crate_name:ident $anchor:literal $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["\n\n", $anchor, ": ",
        $crate::doclink![custom $crate_name $item_path $(@mod$($_m)?)?]]
    };
    ( // https://…/crate_name/item_path
      // file://…/crate_name/item_path/index.html
     custom $crate_name:ident $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat![crate::DOCLINK_CUSTOM_DOMAIN!(),
        stringify!($crate_name), "/latest/", stringify!($crate_name), "/", $item_path]
    };
    ( // https://…/{crate::__crate_name!()}/item_path
      // file://…/{crate::__crate_name!()}/item_path/index.html
     custom_current_crate $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat![crate::DOCLINK_CUSTOM_DOMAIN!(),
        crate::__crate_name!(),"/latest/", stringify!($crate_name), "/", $item_path]
    };
    ( // https://…/{crate::__crate_name!()}/
      // file://…/{crate::__crate_name!()}/index.html
     custom_current_proc_crate $(@mod$($_m:lifetime)?)?) => {
        concat![crate::DOCLINK_CUSTOM_DOMAIN!(),
        crate::__crate_name!(),"/latest/", stringify!($crate_name), "/"]
    };
    (
     /* links to either docs.rs or a local URL */

     // [anchor]: https://…/crate/item_path
     // [anchor]: file://…/current_crate/item_path/index.html
     crate $anchor:literal $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["\n\n", $anchor, ": ", $crate::doclink![crate $item_path $(@mod$($_m)?)?]]
    };
    ( // https://…/crate/item_path
      // file://…/current_crate/item_path/index.html
     crate $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["https://docs.rs/",
        stringify!($crate_name),env! ("CARGO_CRATE_NAME"),
        "/latest/", stringify!($crate_name), "/", $item_path]
    };
    (
     // [anchor]: https://…/crate_name/item_path
     // [anchor]: file://…/crate_name/item_path/index.html
     $crate_name:ident $anchor:literal $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["\n\n", $anchor, ": ", $crate::doclink![$crate_name $item_path $(@mod$($_m)?)?]]
    };
    ( // https://…/crate_name/item_path
      // file://…/crate_name/item_path/index.html
     $crate_name:ident $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["https://docs.rs/",
        stringify!($crate_name), "/latest/", stringify!($crate_name), "/", $item_path]
    };
    ( // https://…/{crate::__crate_name!()}/item_path
      // file://…/{crate::__crate_name!()}/item_path/index.html
     current_crate $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["https://docs.rs/",
        crate::__crate_name!(),"/latest/", stringify!($crate_name), "/", $item_path]
    };
    ( // https://…/{crate::__crate_name!()}/
      // file://…/{crate::__crate_name!()}/index.html
     current_proc_crate $(@mod$($_m:lifetime)?)?) => {
        concat!["https://docs.rs/",
        crate::__crate_name!(),"/latest/", stringify!($crate_name), "/"]
    };
}

/// Helps doc-linking items in downstream crates.
/// (*local* version)
#[doc = crate::_doc_location!("code/util")]
#[cfg(not(feature = "__publish"))]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _doclink {
    (
     /* links to either a custom domain or a local URL */

     // [anchor]: file://…/current_crate/item_path/index.html
     custom crate $anchor:literal $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["\n\n", $anchor, ": ",
        $crate::doclink![custom crate $item_path $(@mod$($_m)?)?]]
    };
    ( // file://…/current_crate/item_path/index.html
     custom crate $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        env!("CARGO_CRATE_NAME"), "/", $item_path $(, "/index.html"$($_m)?)?]
    };
    (
     // [anchor]: file://…/crate_name/item_path/index.html
     custom $crate_name:ident $anchor:literal $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["\n\n", $anchor, ": ",
        $crate::doclink![custom $crate_name $item_path $(@mod$($_m)?)?]]
    };
    ( // file://…/crate_name/item_path/index.html
     custom $crate_name:ident $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        stringify!($crate_name), "/", $item_path $(, "/index.html"$($_m)?)?]
    };
    ( // file://…/{crate::__crate_name!()}/item_path/index.html
     custom_current_crate $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        crate::__crate_name!(), "/", $item_path $(, "/index.html"$($_m)?)?]
    };
    ( // file://…/{crate::__crate_name!()}/index.html
     custom_current_proc_crate $(@mod$($_m:lifetime)?)?) => {
        concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        crate::__crate_name!(), $("/index.html"$($_m)?)?]
    };
    (
     /* links to either docs.rs or a local URL */

     // [anchor]: file://…/current_crate/item_path/index.html
     crate $anchor:literal $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["\n\n", $anchor, ": ", $crate::doclink![crate $item_path $(@mod$($_m)?)?]]
    };
    ( // file://…/current_crate/item_path/index.html
     crate $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        env!("CARGO_CRATE_NAME"), "/", $item_path $(, "/index.html"$($_m)?)?]
    };
    (
     // [anchor]: file://…/crate_name/item_path/index.html
     $crate_name:ident $anchor:literal $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["\n\n", $anchor, ": ", $crate::doclink![$crate_name $item_path $(@mod$($_m)?)?]]
    };
    ( // file://…/crate_name/item_path/index.html
     $crate_name:ident $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        stringify!($crate_name), "/", $item_path $(, "/index.html"$($_m)?)?]
    };
    ( // file://…/{crate::__crate_name!()}/item_path/index.html
     current_crate $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        crate::__crate_name!(), "/", $item_path $(, "/index.html"$($_m)?)?]
    };
    ( // file://…/{crate::__crate_name!()}/index.html
     current_proc_crate $(@mod$($_m:lifetime)?)?) => {
        concat!["file://", env!("CARGO_TARGET_DIR"), "doc/",
        crate::__crate_name!(), $("/index.html"$($_m)?)?]
    };
}

pub(crate) use _doclink as doclink;
