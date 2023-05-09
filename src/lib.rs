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
/// It supports the `not()` option that makes `true` become `false`
/// and anything other than `true` become `true`.
///
/// # Examples
/// ```
/// # extern crate devela_macros;
/// # use devela_macros::compile;
/// #[compile(true)]
/// fn compiled() {}
/// #[compile(no)]
/// fn not_compiled() {}
///
/// compiled();
/// // not_compiled() // fails
///
/// /* using `not()` */
/// #[compile(not(false))]
/// fn compiled2() {}
/// #[compile(not(true))]
/// fn not_compiled2() {}
///
/// compiled2();
/// // not_compiled2(); // fails
/// ```
#[proc_macro_attribute]
pub fn compile(argument: TokenStream, input: TokenStream) -> TokenStream {
    let condition = match argument.to_string().split_whitespace().next() {
        Some(arg) if arg == "true" => true,
        Some(arg) if arg.starts_with("not(") && arg.ends_with(')') => {
            let inner_arg = &arg[4..arg.len() - 1];
            inner_arg != "true"
        }
        _ => false,
    };

    if condition {
        input
    } else {
        proc_macro::TokenStream::new()
    }
}
