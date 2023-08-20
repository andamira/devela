// devela_macros
//
//!
//

extern crate proc_macro;
use proc_macro::TokenStream;

/// Compiles the associated code only if the inner predicate equals `true`.
///
/// Anything other than `true` is considered `false` and the code wont compile.
///
/// It supports the following argument wrapper modifiers:
/// - `not()`: `true` become `false` and anything other than `true` becomes `true`.
/// - `some()`: `true` become `false` and anything other than `true` becomes `true`.
/// - `none()`: `true` become `false` and anything other than `true` becomes `true`.
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
///
/// #[compile(not(other_than_true))]
/// fn compiled2() {}
/// #[compile(not(true))]
/// fn not_compiled2() {}
///
/// // `some()` and `none()`:
///
/// #[compile(some(thing))]
/// fn compiled3() {}
/// #[compile(none())]
/// fn compiled4() {}
///
/// #[compile(some())]
/// fn not_compiled3() {}
/// #[compile(none(thing))]
/// fn not_compiled4() {}
///
/// compiled();
/// compiled2();
/// compiled3();
/// compiled4();
/// // not_compiled()
/// // not_compiled2();
/// // not_compiled3();
/// // not_compiled4();
/// ```
#[proc_macro_attribute]
pub fn compile(argument: TokenStream, input: TokenStream) -> TokenStream {
    let condition = match argument.to_string().split_whitespace().next() {
        Some(arg) if arg == "true" => true,

        // not(arg) returns true only if arg is false
        Some(arg) if arg.starts_with("not(") && arg.ends_with(')') => {
            let inner_arg = &arg[4..arg.len() - 1];
            inner_arg != "true"
        }

        // some(arg) returns true only if there's an arg
        Some(arg) if arg.starts_with("some(") && arg.ends_with(')') => {
            let inner_arg = &arg[5..arg.len() - 1];
            !inner_arg.is_empty()
        }

        // none(arg) returns true only if there's not an arg
        Some(arg) if arg.starts_with("none(") && arg.ends_with(')') => {
            let inner_arg = &arg[5..arg.len() - 1];
            inner_arg.is_empty()
        }
        _ => false,
    };

    if condition {
        input
    } else {
        proc_macro::TokenStream::new()
    }
}
