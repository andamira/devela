// devela/src/code/util/doclink.rs
//
//! Defines [`doclink!`].
//
// TOC
// - doclink! (*internet*)
// - doclink! (*local*)
//
// NOTE: duplicated (not symlinked) in /crates/macros/src/core_bridge/doclink.rs)
// WAIT [missing cross-crate docs](https://github.com/rust-lang/rust/issues/120927)

/// Custom domain used for the [`doclink!`] macro.
#[doc(hidden)]
#[macro_export]
macro_rules! _DOCLINK_CUSTOM_DOMAIN {
    () => {
        "https://docs.rs/"
        // "https://andamira.github.io/"
    };
}
#[doc(hidden)]
pub use _DOCLINK_CUSTOM_DOMAIN;

crate::CONST! { hidden macro_export,
    /// Documentation for the `doclink!` macro.
    _DOC_DOCLINK = "\n\nConstructs a URL to Rust documentation.

Module links use `@mod`. Exact item links use `@item kind Item`, allowing
`doclink!` to construct the rustdoc filename without requiring callers to
encode names such as `struct.Item.html` manually.

Supported item kinds are:
`struct`, `enum`, `union`, `trait`, `type`, `fn`, `const`, `static`,
`macro`, `attr`, and `derive`.

The `custom` forms use `_DOCLINK_CUSTOM_DOMAIN!` as their documentation root.
That macro must expand to a string literal ending in `/`, for example:
```
#[macro_export] #[doc(hidden)]
macro_rules! _DOCLINK_CUSTOM_DOMAIN { () => { \"https://docs.rs/\" } } // it must end in `/`
```

# Features
With the `__publish` feature enabled, links target published documentation.
Otherwise they target the locally generated rustdoc tree.

The local forms require `CARGO_TARGET_DIR` to be defined.
In devela this is provided by `/build/main/environment.rs`.

The `current_crate` and `current_proc_crate` forms require `__crate_name!`
to expand to the current crate name.

# Examples
```
# use devela::doclink;
/// Links to [`AnotherExample`] and [some module].
#[doc = doclink!(crate_name \"[`AnotherExample`]\" \"path/to/module\" @item struct AnotherExample)]
#[doc = doclink!(crate_name \"[some module]\" \"path/to/some_module\" @mod)]
///
/// [`Item`] is linked in the crate currently being compiled.
#[doc = doclink!(crate \"[`Item`]\" \"path/to/module\" @item struct Item)]
pub struct Example;
```
    ";
}

#[doc = crate::_tags!(code)] //_
/// Helps doc-linking items in downstream crates.
/// <!-- (*internet* version) -->
#[doc = crate::_doc_meta!{location("code/util", macro doclink)}]
#[doc = _DOC_DOCLINK!()] //
#[cfg(feature = "__publish")]
#[macro_export] //
#[cfg_attr(cargo_primary_package, doc(hidden))] //
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

#[doc = crate::_tags!(code)]
/// Helps doc-linking items in downstream crates.
/// (*local* version)
#[doc = crate::_doc_meta!{location("code/util", macro doclink)}]
#[doc = _DOC_DOCLINK!()]
#[cfg(not(feature = "__publish"))]
#[macro_export] //
#[cfg_attr(cargo_primary_package, doc(hidden))] //
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

#[doc(inline)]
pub use doclink· as doclink;
