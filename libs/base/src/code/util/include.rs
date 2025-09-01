// devela_base::code::util::include
//
//! Defines the [`include_from!`], [`mod_from!`] and [`mod_path!`] macros.
//
// WAIT [include! fails with top-level attributes](https://github.com/rust-lang/rfcs/issues/752)
// WAIT [include! should allow inner attributes](https://github.com/rust-lang/rust/issues/66920)
// WAIT [inner attributes with include!](https://github.com/rust-lang/rust/issues/117464)

/// Includes a Rust source file relative to the project's directory.
///
/// The contents of the specified file are inserted into the current file
/// at the location of the macro invocation. This allows you to reuse code
/// from other files without creating separate modules.
///
/// # Usage
///
/// - To include a file relative to the project's directory:
/// ```ignore
/// include_from!("src/helper.rs");
/// ```
///
/// - To include a file using its module name (relative to the project's directory):
/// ```ignore
/// include_from!(helper); // Resolves to "helper.rs" in the project's root.
/// ```
///
/// - When using [`cargo-script`], the path is relative to the script's directory:
/// ```ignore
/// #!/usr/bin/env -S cargo +nightly -Zscript
/// include_from!(utils);
/// ```
/// [`cargo-script`]: https://github.com/rust-lang/cargo/issues/12207
///
/// See also [`mod_from!`].
///
/// # Issues
/// The `include!` macro fails if included file has top-level inner attributes.
/// See [#752](https://github.com/rust-lang/rfcs/issues/752).
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
macro_rules! _include_from {
    ($module_name:ident) => {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", stringify!($module_name), ".rs"));
    };
    ($module_path_str:literal) => {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $module_path_str));
    };
}
#[doc(inline)]
pub use _include_from as include_from;

/// Declares a module by including a Rust source file relative to the project's directory.
///
/// The macro generates a `mod` declaration and inserts the contents of the specified file
/// into the module. This is a more ergonomic alternative to manually wrapping `include!`
/// within a module declaration.
///
/// # Usage
///
/// - To declare a module using its name:
/// ```ignore
/// // Declares `pub(super) mod helper` with contents from "helper.rs":
/// mod_from!(pub(super) helper);
/// ```
///
/// - To declare a module with a specific path:
/// ```ignore
/// // Declares `mod helper` with contents from "src/helper.rs":
/// mod_from!(helper, "src/helper.rs");
/// ```
///
/// See also [`include_from!`], [`mod_path!`]
///
/// # Issues
/// The `include!` macro fails if included file has top-level inner attributes.
/// See [#752](https://github.com/rust-lang/rfcs/issues/752).
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _mod_from {
    ($vis:vis $module_name:ident) => {
        $vis mod $module_name { $crate::include_from!($module_name); }
    };
    ($vis:vis $module_name:ident, $module_path_str:literal) => {
        $vis mod $module_name { $crate::include_from!($module_path_str); }
    };
}
#[doc(inline)]
pub use _mod_from as mod_from;

/// A macro helper to define a module name and path.
///
/// This is used to include files re-exported by lower-level workspace crates,
/// which add documentation on top of the original one. But there's an [issue][0]
/// where cross-crate import chains mises docs from middle crates.
///
// WAIT: missing cross-crate docs:
/// [0]: https://github.com/rust-lang/rust/issues/120927
///
/// There's another [issue][1] with the `include!` macro not supporting module-level attributes.
///
// WAIT: include! fails with top-level attributes:
/// [1]: https://github.com/rust-lang/rfcs/issues/752
///
/// so we have to resort to hardcoding the `path` attribute for the module.
/// (hardcoding is necessary since it only supports a whole literal. See [issue][2].
///
// WAIT: Module path attribute override with nested macros doesn't work:
/// [2]: https://github.com/rust-lang/rust/issues/87681
//
// Once the main issue (120927) is fixed, then this:
// mod_path!(alloc "../libs/base_alloc/src/reexports.rs", _ia);
//
// Could be replaced with this:
// #[cfg(feature = "alloc")] #[doc(inline)] #[rustfmt::skip]
// pub use devela_base_alloc::{String, ToString};
///
/// See also [`mod_from!`].
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _mod_path {
    (
     /* most common feature-gates */
     alloc $mod:ident $path:literal) => { $crate::mod_path![["alloc"] $mod $path]; };
    (std $mod:ident $path:literal) => { $crate::mod_path![["std"] $mod $path]; };
    (
    // an optional feature gate
    $([$feature:literal])? $mod:ident $path:literal) => {
        $(#[cfg(feature = $feature)])?
        #[path = $path] mod $mod;
    };
    (
     /* with automatic imports of all the items from the module */
     alloc +$vis:vis $mod:ident $path:literal) => {
        $crate::mod_path![["alloc"] +$vis $mod $path];
    };
    (std +$vis:vis $mod:ident $path:literal) => {
        $crate::mod_path![["std"] +$vis $mod $path];
    };
    ($([$feature:literal])? +$vis:vis $mod:ident $path:literal) => {
        $(#[cfg(feature = $feature)])?
        #[path = $path] mod $mod;
        $(#[cfg(feature = $feature)])?
        $vis use $mod::*;
    };
}
#[doc(inline)]
pub use _mod_path as mod_path;
