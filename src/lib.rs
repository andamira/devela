// devela_macros
//
//!
//

extern crate proc_macro;
use proc_macro::TokenStream;

/// Includes the block attached to this attribute macro, only if `argument`
/// is equal to `true`.
#[proc_macro_attribute]
pub fn include_block(argument: TokenStream, input: TokenStream) -> TokenStream {
    if argument.to_string() == "true" {
        input
    } else {
        proc_macro::TokenStream::new()
    }
}
