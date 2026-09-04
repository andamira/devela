// devela/src/code/util/debug/fn_name.rs
//
//! Defines [`fn_name`].
//

#[doc = crate::_tags!(code debug)]
/// Returns a best-effort name of the enclosing function.
#[doc = crate::_doc_meta!{
    location("code/util/debug", macro fn_name),
}]
/// This is a diagnostic helper based on [`core::any::type_name_of_val`].
/// The exact format is not guaranteed by Rust, so this macro must not be
/// used for semantic program behavior.
///
/// ## Examples
/// ```
/// mod bar {
///     use devela::fn_name;
///
///     pub fn sample_function() {
///         assert!(fn_name!().ends_with("bar::sample_function"));
///     }
/// }
/// bar::sample_function();
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! fn_name· {
    () => {{
        fn f() {}
        let name = ::core::any::type_name_of_val(&f);
        name.strip_suffix("::f").unwrap_or(name)
    }};
}
#[doc(inline)]
pub use fn_name· as fn_name;
