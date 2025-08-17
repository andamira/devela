// devela_base::code::util::include
//
//! Defines the [`include_from!`] and [`mod_from!`] macros.
//

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
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! _include_from {
    ($module_name:ident) => {
        include!(concat!(std::env!("CARGO_MANIFEST_DIR"), "/", stringify!($module_name), ".rs"));
    };
    ($module_path_str:literal) => {
        include!(concat!(std::env!("CARGO_MANIFEST_DIR"), "/", $module_path_str));
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
/// See also [`include_from!`].
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
