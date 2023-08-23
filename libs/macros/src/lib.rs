// devela_macros
//
//!
//! ## Conditional compilation
//!
//! Each form of conditional compilation takes a compilation predicate that
//! evaluates to `true` or `false`.
//!
//! These are the [`#[compile]`][compile()] and [`#[compile_attr]`][compile_attr()]
//! attributes and the [`cif!`][cif()] macro.
//!
//! They are similar to the [`#[cfg]`][1] and [`#[cfg_attr]`][2] attributes and
//! the [`cfg!`][3] macro, except for using *compilation predicates*.
//!
//! ### Compilation predicates
//!
//! The following compilation predicates are supported:
//!
//! - unary:
//!   - A bare predicate returns `true` only if it is the word `true`.
//!   - `not()`: returns `true` only if the single predicate is **not** `true`.
//! - binary:
//!   - `eq()`: returns `true` if both predicates evaluate as equal.
//!   - `ne()`: returns `true` if both predicates does **not** evaluate as equal.
//!   - `xor()`: returns `true` only if **one** predicate is `true`, but not both.
//! - non-binary:
//!   - `any()`: returns `true` if any predicate is `true`.
//!   - `all()`: returns `true` if all of the predicates are `true`.
//!   - `xodd()`: returns `true` if there is an **odd number** of `true` predicates.
//!   - `xome()`: returns `true` if there is **some `true`** predicates, but not all.
//!   - `same()`: returns `true` if all the predicates have the **same text**.
//!   - `diff()`: returns `true` if any predicate has a **different text**.
//!   - `some()`: returns `true` if there is **some given** predicate.
//!   - `none()`: returns `true` if there is **no given** predicate.
//!
//! When more than 1 predicate is supported, they are separated by commas.
//!
//! [1]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute
//! [2]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute
//! [3]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-macro
//

extern crate proc_macro;
use proc_macro::TokenStream;

#[cfg(test)]
mod tests;

mod common;
use common::{compile_eval, split_args};

/// Conditionally compiles the thing it is attached to based on the
/// [compilation predicate][crate#compilation-predicates].
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

/// Conditionally compiles the given attributes based on the
/// [compilation predicate][crate#compilation-predicates].
///
/// # Examples
/// ```
#[doc = include_str!("../examples/compile_attr.rs")]
/// ```
#[proc_macro_attribute]
pub fn compile_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.to_string();
    let mut args = split_args(&args);

    if args.is_empty() {
        panic!("The compile_attr macro requires at least one argument");
    }
    let condition = args.remove(0);

    if compile_eval(condition) {
        let mut expanded = input.to_string();
        for attribute in args {
            expanded = format!("#[{}] {}", attribute, expanded);
        }
        expanded.parse().expect("Couldn't parse as a TokenStream")
    } else {
        input
    }
}

/// Evaluates to either a `true` of `false` literal based on the
/// [compilation predicate][crate#compilation-predicates].
///
/// # Examples
/// ```
/// use devela_macros::cif;
///
/// let the_answer_is = if cif!(none(some)) {
///   "one"
/// } else if cif!(any(false, diff(this, that))) {
///   "two"
/// } else {
///   "three"
/// };
///
/// assert_eq!(the_answer_is, "two");
/// ```
#[proc_macro]
pub fn cif(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let result = compile_eval(input);
    result.to_string().parse().unwrap()
}
