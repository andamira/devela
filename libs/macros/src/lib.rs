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
//! the [`cfg!`][3] macro, except they use *compilation predicates*.
//!
//! There is also the [`#[compile_doc]`][compile_doc()] macro to conditionally
//! compile documentation blocks depending on predicates.
//!
//! ### Compilation predicates
//!
//! The following compilation predicates are supported:
//!
//! - unary:
//!   - A bare predicate returns `true` only if it is the **`true`** literal
//!   - `not()`: returns `true` only if the predicate does **not** evaluate to **`true`**.
//!
//! - binary:
//!   - `equal()`: returns `true` if both predicates are evaluated as **equal**.
//!   - `xor()`: returns `true` if **only one** predicate **is `true`**, but **not both**.
//!
//!   - `eq()`: returns `true` if both predicates are **number literals** and Left **==** Right.
//!   - `ne()`: returns `true` if both predicates are **number literals** and Left **!=** Right.
//!   - `ge()`: returns `true` if both predicates are **number literals** and Left **>=** Right.
//!   - `gt()`: returns `true` if both predicates are **number literals** and Left **>** Right.
//!   - `le()`: returns `true` if both predicates are **number literals** and Left **<=** Right.
//!   - `lt()`: returns `true` if both predicates are **number literals** and Left **<** Right.
//!
//! - non-binary:
//!   - `any()`: returns `true` if **any** predicate **is `true`**.
//!   - `all()`: returns `true` if **all** predicates **are `true`**.
//!   - `none()`: returns `true` if there is **no given** predicate.
//!   - `some()`: returns `true` if there is **some given** predicate.
//!   - `diff()`: returns `true` if **any** predicate has a **different text**.
//!   - `same()`: returns `true` if **all** the predicates have the **same text**.
//!   - `xany()`: returns `true` if there are **any `true`** predicates, but **not all**.
//!   - `xodd()`: returns `true` if there is an **odd number** of `true` predicates.
//!   - `xone()`: returns `true` if there is just **one `true`** predicate, but **no more**.
//!
//! When more than 1 predicate is supported, they are separated by commas.
//!
//! [1]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute
//! [2]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg_attr-attribute
//! [3]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-macro
//

// warnings
#![warn(clippy::all)]
// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{
    format,
    string::{String, ToString},
};

extern crate proc_macro;
use proc_macro::{TokenStream, TokenTree};

#[cfg(feature = "alloc")]
#[cfg(test)]
mod tests;

mod common;
#[cfg(feature = "alloc")]
use common::{compile_eval, deindent, split_args, split_compile_doc_tuple};

/// Conditionally compiles the thing it is attached to based on the
/// [compilation predicate](https://docs.rs/devela_macros/#compilation-predicates).
///
/// # Examples
/// ```
#[doc = include_str!("../examples/compile.rs")]
/// ```
#[proc_macro_attribute]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn compile(argument: TokenStream, input: TokenStream) -> TokenStream {
    if compile_eval(argument.to_string()) {
        input
    } else {
        proc_macro::TokenStream::new()
    }
}

/// Conditionally compiles the given attributes based on the
/// [compilation predicate](https://docs.rs/devela_macros/#compilation-predicates).
///
/// # Examples
/// ```
#[doc = include_str!("../examples/compile_attr.rs")]
/// ```
#[proc_macro_attribute]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
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

/// Conditionally compiles each doc comment based on the
/// [compilation predicate](https://docs.rs/devela_macros/#compilation-predicates).
/// # Examples
/// ```
#[doc = include_str!("../examples/compile_doc.rs")]
/// ```
#[proc_macro_attribute]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn compile_doc(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = args.to_string();
    let doc_conditions = split_args(&args);
    let mut result = String::new();

    for doc_condition in doc_conditions {
        if doc_condition.is_empty() {
            break;
        }
        let (condition, comment) = split_compile_doc_tuple(&doc_condition);
        if compile_eval(condition) {
            result.push_str(&format!("#[doc = \"{}\n\"]", deindent(&comment)));
        }
    }
    // Append the original item
    result.push_str(&input.to_string());
    result.parse().unwrap()
}

/// Evaluates to either a `true` of `false` literal based on the
/// [compilation predicate](https://docs.rs/devela_macros/#compilation-predicates).
///
/// # Examples
/// ```
#[doc = include_str!("../examples/cif.rs")]
/// ```
#[proc_macro]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn cif(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let result = compile_eval(input);
    result.to_string().parse().unwrap()
}

/// Returns the first non-empty argument.
///
/// If all arguments are empty, the macro returns nothing.
///
/// This macro is inspired by the SQL `COALESCE` function, which returns the first
/// non-null value from a list of arguments, or null if all the arguments are null.
///
/// # Examples
/// ```
#[doc = include_str!("../examples/coalesce.rs")]
/// ```
#[proc_macro]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
pub fn coalesce(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let args = split_args(&input);

    let first_non_empty_arg = args
        .into_iter()
        .find(|arg| !arg.is_empty())
        .unwrap_or_default();
    // .unwrap_or_else(|| "".to_string());
    // .expect("No non-empty arguments found");

    first_non_empty_arg
        .parse()
        .expect("Failed to parse TokenStream")
}

/// Returns the total number of [identifiers] in its input.
///
/// [identifiers]: https://doc.rust-lang.org/reference/identifiers.html
///
/// This macro does not differentiate between different kinds of identifiers
/// nor check their validity in context. It simply counts all identifier,
/// discarding the rest.
///
/// See also [`ident_unique!`], [`ident_total_unique!`].
///
/// # Examples
/// ```
/// # use devela_macros::ident_total;
/// assert_eq![ident_total!(a, a 東 r#true; a3 != 3a), 5];
/// ```
#[proc_macro]
pub fn ident_total(input: TokenStream) -> TokenStream {
    let mut count = 0;
    for token in input {
        #[allow(clippy::single_match)]
        match token {
            TokenTree::Ident(_) => count += 1,
            _ => {}
        }
    }
    let result = format!("{}", count);
    result.parse().unwrap()
}

/// Returns the number of *unique* [identifiers] in its input.
///
/// [identifiers]: https://doc.rust-lang.org/reference/identifiers.html
///
/// This macro does not differentiate between different kinds of identifiers
/// nor check their validity in context. It simply counts all unique identifiers,
/// discarding the rest.
///
/// See also [`ident_total!`], [`ident_total_unique!`].
///
/// # Examples
/// ```
/// # use devela_macros::ident_unique;
/// assert_eq![ident_unique!(a, a 東 r#true; a3 != 3a), 4];
/// ```
#[proc_macro]
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub fn ident_unique(input: TokenStream) -> TokenStream {
    let mut unique = std::collections::HashSet::new();
    for token in input {
        #[allow(clippy::single_match)]
        match token {
            TokenTree::Ident(i) => {
                unique.insert(i.to_string());
            }
            _ => {}
        }
    }
    let result = format!("{}", unique.len());
    result.parse().unwrap()
}

/// Returns the numbers of both *total* and *unique* [identifiers] in its input.
///
/// [identifiers]: https://doc.rust-lang.org/reference/identifiers.html
///
/// This macro does not differentiate between different kinds of identifiers
/// nor check their validity in context. It simply counts all identifiers,
/// discarding the rest, and returns an array with both the total and unique count.
///
/// See also [`ident_total!`], [`ident_unique!`].
///
/// # Examples
/// ```
/// # use devela_macros::ident_total_unique;
/// assert_eq![ident_total_unique!(a, a 東 r#true; a3 != 3a), [5, 4]];
/// ```
#[proc_macro]
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub fn ident_total_unique(input: TokenStream) -> TokenStream {
    let mut unique = std::collections::HashSet::new();
    let mut total = 0;
    for token in input {
        #[allow(clippy::single_match)]
        match token {
            TokenTree::Ident(i) => {
                total += 1;
                unique.insert(i.to_string());
            }
            _ => {}
        }
    }
    let result = format!("[{}, {}]", total, unique.len());
    result.parse().unwrap()
}
