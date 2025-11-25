// devela_base_core::code::util::debug

/// Emits a compile-time warning with a provided message.
///
/// This implemented through an existing `dead_code` warning,
/// thus the output for the following example:
///
/// ```
/// # use devela_base_core::compile_warn;
/// compile_warn!(sample_user_defined_warning);
/// ```
///
/// may look as follows:
/// ```text
/// warning: constant `user_defined_warning` is never used
///   --> src/lib.rs:7:9
///   |
/// 7 |  compile_warn!(user_defined_warning);
///   |                ^^^^^^^^^^^^^^^^^^^^
/// ```
///
/// Once [`proc_macro_diagnostics`] feature is stabilized, this macro
/// could be replaced with a proper proc-macro-based implementation.
///
/// This macro is intended to be used in the development process, as an alternative
/// to the [`unimplemented`] macro which doesn't cause code to panic.
///
/// [`std::compile_error`]: https://doc.rust-lang.org/std/macro.compile_error.html
/// [`proc_macro_diagnostics`]: https://github.com/rust-lang/rust/issues/54140
/// [`unimplemented`]: https://doc.rust-lang.org/std/macro.unimplemented.html
#[doc = crate::_doc!(vendor: "stdext")]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! compile_warn {
    ($const:ident) => {
        #[warn(dead_code)]
        #[allow(non_upper_case_globals)]
        const $const: &str = "";
    };
}
#[doc(inline)]
pub use compile_warn;

/// This macro returns the name of the enclosing function.
///
/// As the internal implementation is based on [`type_name`],
/// this macro derives all the limitations of this function.
#[doc = crate::doclink!(custom devela "[`type_name`]" "code/trait.AnyExt.html#method.type_name")]
///
/// ## Examples
/// ```
/// mod bar {
///     pub fn sample_function() {
///         # use devela_base_core::fn_name;
///         assert!(fn_name!().ends_with("bar::sample_function"));
///     }
/// }
/// bar::sample_function();
/// ```
#[doc = crate::_doc!(vendor: "stdext")]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! fn_name {
    () => {{
        // Okay, this is ugly, I get it. However, this is the best we can get on a stable rust.
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            core::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3] // `3` is the length of the `::f`.
    }};
}
#[doc(inline)]
pub use fn_name;
