// devela_macros
//
//!
//

extern crate proc_macro;
use proc_macro::TokenStream;

mod common;
use common::{compile_eval};

/// Compiles the associated code only if the inner predicate is `true`.
///
/// Anything other than `true` is considered `false` and the code wont compile.
///
/// The following combinable predicate modifiers are supported:
///
/// - `not()`: returns `true` if the predicate is `false`.
/// - `some()`: returns `true` if there **is** a predicate.
/// - `none()`: returns `true` if there is **no** predicate.
/// - `all()`: returns `true` if all the predicates are `true`.
/// - `any()`: returns `true` if any of the predicates is `true`.
///
/// # Examples
/// ```
/// # use devela_macros::compile;
/// #[compile(true)]
/// fn compiled() {}
/// #[compile(other_than_true)]
/// fn not_compiled() {}
///
/// // `not()`:
/// #[compile(not(other_than_true))]
/// fn compiled_not() {}
/// #[compile(not(true))]
/// fn not_compiled_not() {}
///
/// // `some()`
/// #[compile(some(thing))]
/// fn compiled_some() {}
/// #[compile(some())]
/// fn not_compiled_some() {}
///
/// // `none()`
/// #[compile(none())]
/// fn compiled_none() {}
/// #[compile(none(thing))]
/// fn not_compiled_none() {}
///
/// // `all()`:
/// #[compile(all(true, true, none(), some(thing), not(none(thing))))]
/// fn compiled_all() {}
/// #[compile(all(true, false))]
/// fn not_compiled_all() {}
///
/// // `any()`:
/// #[compile(any(true, false))]
/// fn compiled_any() {}
/// #[compile(any(false, false))]
/// fn not_compiled_any() {}
///
/// // nested:
/// #[compile(all(true, not(any(some(), none(thing), not(not(false))))))]
/// fn compiled_nested() {}
/// #[compile(all(true, not(any(some(), none(thing), true))))]
/// fn not_compiled_nested() {}
///
/// compiled();
/// compiled_not();
/// compiled_some();
/// compiled_none();
/// compiled_all();
/// compiled_any();
/// compiled_nested();
///
/// // not_compiled()
/// // not_compiled_not();
/// // not_compiled_some();
/// // not_compiled_none();
/// // not_compiled_all();
/// // not_compiled_any();
/// // not_compiled_nested();
/// ```
#[proc_macro_attribute]
pub fn compile(argument: TokenStream, input: TokenStream) -> TokenStream {
    if compile_eval(argument.to_string()) {
        input
    } else {
        proc_macro::TokenStream::new()
    }
}
