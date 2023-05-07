// devela_macros
//
//!
//

extern crate proc_macro;
use proc_macro::TokenStream;

/// Compiles the associated code only if the provided literal is equal to `true`.
///
/// # Examples
/// ```
/// extern crate devela_macros;
/// use devela_macros::compile;
///
/// #[compile(true)]
/// fn compiled() {}
///
/// #[compile(no)]
/// fn not_compiled() {}
///
/// compiled()
/// // not_compiled() // fails to compile if uncommented
/// ```
#[proc_macro_attribute]
pub fn compile(argument: TokenStream, input: TokenStream) -> TokenStream {
    if argument.to_string() == "true" {
        input
    } else {
        proc_macro::TokenStream::new()
    }
}
