// devela_base::code::util::_doc
//
//! Defines [`doclink!`].
//

/// Helps doc-linking to items in downstream crates. (*local* version)
///
/// Returns the constructed URL to the
///
/// # Features
/// If the `__publish` feature is enabled it links to the published documentation,
/// otherwise it links to the local path.
//
// CARGO_TARGET_DIR: in sync with /build/main/environment.rs
// WAIT [missing cross-crate docs](https://github.com/rust-lang/rust/issues/120927)
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
#[cfg(not(feature = "__publish"))]
macro_rules! _doclink {
    (
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
}
/// Helps doc-linking to items in downstream crates. (*internet* version)
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[macro_export]
#[cfg(feature = "__publish")]
macro_rules! __doclink {
    (
     // [anchor]: https://…/crate/item_path
     crate $anchor:literal $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["\n\n", $anchor, ": ", $crate::doclink![crate $item_path $(@mod$($_m)?)?]]
    };
    ( // https://…/crate/item_path
     crate $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["https://andamira.github.io/",
        stringify!($crate_name),env! ("CARGO_CRATE_NAME"),
        "/latest/", stringify!($crate_name), "/", $item_path]
    };
    (
     // [anchor]: https://…/crate_name/item_path
     $anchor:literal $crate_name:ident $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["\n\n", $anchor, ": ", $crate::doclink![$crate_name $item_path $(@mod$($_m)?)?]]
    };
    ( // https://…/crate_name/item_path
     $crate_name:ident $item_path:literal $(@mod$($_m:lifetime)?)?) => {
        concat!["https://andamira.github.io/", stringify!($crate_name),
        "/latest/", stringify!($crate_name), "/", $item_path]
    };
}
#[doc(inline)]
pub use _doclink as doclink;
