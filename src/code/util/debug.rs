// devela/src/code/util/debug.rs
//
//! Defines [`const_warn!`], [`fn_name`].
//

#[doc = crate::_tags!(code debug)]
/// Emits a compile-time warning with a provided message.
#[doc = crate::_doc_meta!{location("code/util")}]
///
/// # Example
/// ```
/// # use devela::const_warn;
/// fn main() {
///     const_warn!("hello warning!");
///     const_warn!(if size_of::<u8>() == 1, "byte alert!!");
/// }
/// ```
/// That may look as follows:
/// ```text
/// warning: use of deprecated associated constant `main::_::__::<true>::WARNING`: hello warning!
///  --> src/main.rs:3:5
///   |
/// 3 |     const_warn!("hello warning!");
///   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///   |
///   = note: `#[warn(deprecated)]` on by default
///   = note: this warning originates in the macro `$crate::const_warn` which comes from the expansion of the macro `const_warn` (in Nightly builds, run with -Z macro-backtrace for more info)
///
/// warning: use of deprecated associated constant `main::_::__::<true>::WARNING`: byte alert!!
///  --> src/main.rs:4:5
///   |
/// 4 |     const_warn!(if size_of::<u8>() == 1, "byte alert!!");
///   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
///   ...
/// ```
// IMPROVE: use a proc-macro leveraging nightly feature proc_macro_diagnostics
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! const_warn {
    ($msg:literal $(,)?) => {
        $crate::const_warn!(if true, $msg);
    };
    (if $cond:expr, $msg:literal $(,)?) => {
        $crate::const_warn!(%if $cond, WARNING, $msg);
    };
    (%if $cond:expr, $name:ident, $msg:literal $(,)?) => {
        #[allow(dead_code)]
        const _: () = {
            struct __<const B: bool>;
            impl __<false> {
                const $name: () = ();
            }
            impl __<true> {
                #[deprecated(note = $msg)]
                const $name: () = ();
            }
            let _ = __::<{ $cond }>::$name;
        };
    };
}
#[doc(inline)]
pub use const_warn;

#[doc = crate::_tags!(code debug)]
/// Returns a best-effort name of the enclosing function.
#[doc = crate::_doc_meta!{location("code/util")}]
///
/// This is a diagnostic helper based on [`core::any::type_name_of_val`].
/// The exact format is not guaranteed by Rust, so this macro must not be
/// used for semantic program behavior.
///
/// ## Examples
/// ```
/// mod bar {
///     pub fn sample_function() {
///         # use devela::fn_name;
///         assert!(fn_name!().ends_with("bar::sample_function"));
///     }
/// }
/// bar::sample_function();
/// ```
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! fn_name {
    () => {{
        fn f() {}
        let name = ::core::any::type_name_of_val(&f);
        name.strip_suffix("::f").unwrap_or(name)
    }};
}
#[doc(inline)]
pub use fn_name;
