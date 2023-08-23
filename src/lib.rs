// devela_macros
//
//!
//

extern crate proc_macro;
use proc_macro::TokenStream;

#[cfg(test)]
mod tests;

mod common;
use common::compile_eval;

/// Compiles the associated code only if the inner predicate evaluates to `true`.
///
/// Anything other than a `true` result is considered as `false`
/// and the code wont be compiled.
///
/// The following combinable predicate modifiers are supported:
///
/// - unary:
///   - returns `true` if the single predicate is `true`.
///   - `not()`: returns `true` if the predicate is **not** `true`.
/// - binary:
///   - `eq()`: returns `true` if both predicates evaluate equally.
///   - `ne()`: returns `true` if both predicates does **not** evaluate equally.
///   - `xor()`: returns `true` only if **one** predicate is `true`, but not both.
/// - non-binary:
///   - `any()`: returns `true` if any predicate is `true`.
///   - `all()`: returns `true` if all of the predicates are `true`.
///   - `xodd()`: returns `true` if there is an **odd number** of `true` predicates.
///   - `xome()`: returns `true` if there is **some `true`** predicates, but not all.
///   - `same()`: returns `true` if all the predicates have the **same text**.
///   - `diff()`: returns `true` if any predicate has a **different text**.
///   - `some()`: returns `true` if there is **some given** predicate.
///   - `none()`: returns `true` if there is **no given** predicate.
///
/// # Examples
/// ```
#[doc = include_str!("../examples/compile.rs")]
/// ```
#[proc_macro_attribute]
pub fn compile(argument: TokenStream, input: TokenStream) -> TokenStream {
    if compile_eval(argument.to_string()) {
        input
    } else {
        proc_macro::TokenStream::new()
    }
}
