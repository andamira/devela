// devela_base_core::code::util::doclink
//
//! Defines [`doclink!`].
//
// TOC
// - doclink! (*internet*)
// - doclink! (*local*)
//
// NOTE: duplicated (not symlinked) in devela_base_macros/src/core_bridge/)
// WAIT [missing cross-crate docs](https://github.com/rust-lang/rust/issues/120927)

/// Custom domain used for the [`doclink!`] macro.
#[doc(hidden)] #[macro_export] #[rustfmt::skip]
macro_rules! _DOCLINK_CUSTOM_DOMAIN { () => { "https://andamira.github.io/" }; }
#[doc(hidden)] #[rustfmt::skip] pub use _DOCLINK_CUSTOM_DOMAIN as DOCLINK_CUSTOM_DOMAIN;

crate::CONST! { hidden macro_export,
    // DOCLINK_CUSTOM_DOMAIN = "https://andamira.github.io"; // doesn't work

    /// Documentation for the `doclink!` macro.
    _DOC_DOCLINK = "\n\nReturns the constructed URL to the linked doc element.

The `custom` arms depend on defining a macro that returns a string literal
containing the custom domain to use for the docs, following the same structure as docs.rs. E.g.:
```
#[macro_export] #[doc(hidden)]
macro_rules! DOCLINK_CUSTOM_DOMAIN { () => { \"https://docs.rs/\" } } // it must end in `/`
```

# Features
If the `__publish` feature is enabled it links to the published documentation on the internet,
otherwise it links to the local path.

The local version needs the `CARGO_TARGET_DIR` env var always defined.
For *devela* it is defined in `/build/main/environment.rs`.

The `current_crate` versions needs a macro defined named `__crate_name!`
that returns a string literal with the name of the current crate.

# Examples
```
# use devela_base_core::doclink;
/// Example links to [`AnotherExample`] and [some module].
#[doc = doclink!(crate_name \"[`AnotherExample`]\" \"path/to/struct.AnotherExample.html\")]
#[doc = doclink!(crate_name \"[some module]\" \"path/to/some_module\" @mod)]
///
/// This doclink to [`Item`] links to the current crate being compiled.
#[doc = doclink!(crate \"[`Item`]\" \"path/to/struct.Item.html\")]
pub struct Example;
```
    ";
}

#[doc = crate::_TAG_CODE!()]
/// Helps doc-linking items in downstream crates.
/// <!-- (*internet* version) -->
#[doc = crate::_doc_location!("code/util")]
#[doc = _DOC_DOCLINK!()]
#[macro_export]
#[cfg(feature = "__publish")]
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[allow(clippy::crate_in_macro_def, reason = "crate::__crate_name! is intended")]
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
     custom_current_crate $item_path:expr, $(@mod$($_m:lifetime)?)?) => {
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

#[doc = crate::_TAG_CODE!()]
/// Helps doc-linking items in downstream crates.
/// (*local* version)
#[doc = crate::_doc_location!("code/util")]
#[doc = _DOC_DOCLINK!()]
#[macro_export]
#[cfg(not(feature = "__publish"))]
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[allow(clippy::crate_in_macro_def, reason = "crate::__crate_name! is intended")]
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
     custom_current_crate $item_path:expr, $(@mod$($_m:lifetime)?)?) => {
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

#[doc(inline)]
pub use _doclink as doclink;
