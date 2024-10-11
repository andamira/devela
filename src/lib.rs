// devela_macros
//
//!
#![doc = include_str!("./Lib.md")]
//
// TOC
// - compile:
//   - cif
//   - compile
//   - compile_attr
//   - compile_doc (hidden)
// - ident:
//   - coalesce
//   - ident_total
//   - ident_total_unique
//   - ident_unique
// - niche:
//   - enumint

// warnings
#![warn(clippy::all)]
// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly_doc", feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate proc_macro;
use proc_macro::TokenStream;

mod bodies;
use bodies::*;

/* compile */

/// Evaluates to either a `true` of `false` literal based on the
/// [compilation predicate](https://docs.rs/devela_macros/#compilation-predicates).
///
/// # Examples
/// ```
#[doc = include_str!("../examples/cif.rs")]
/// ```
#[proc_macro]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub fn cif(input: TokenStream) -> TokenStream {
    body_cif(input)
}

/// Conditionally compiles the thing it is attached to based on the
/// [compilation predicate](https://docs.rs/devela_macros/#compilation-predicates).
///
/// # Examples
/// ```
#[doc = include_str!("../examples/compile.rs")]
/// ```
#[proc_macro_attribute]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub fn compile(args: TokenStream, input: TokenStream) -> TokenStream {
    body_compile(args, input)
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
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub fn compile_attr(args: TokenStream, input: TokenStream) -> TokenStream {
    body_compile_attr(args, input)
}

/// Conditionally compiles each doc comment based on the
/// [compilation predicate](https://docs.rs/devela_macros/#compilation-predicates).
/// # Examples
/// ```
#[doc = include_str!("../examples/compile_doc.rs")]
/// ```
#[proc_macro_attribute]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
#[doc(hidden)]
pub fn compile_doc(args: TokenStream, input: TokenStream) -> TokenStream {
    body_compile_doc(args, input)
}

/* ident */

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
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub fn coalesce(input: TokenStream) -> TokenStream {
    body_coalesce(input)
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
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub fn ident_total(input: TokenStream) -> TokenStream {
    body_ident_total(input)
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
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
pub fn ident_total_unique(input: TokenStream) -> TokenStream {
    body_ident_total_unique(input)
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
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
pub fn ident_unique(input: TokenStream) -> TokenStream {
    body_ident_unique(input)
}

/* niche */

/// Generates an enum with variants corresponding to values within a specified range.
///
/// This macro generates an enum with integer variants named `_` followed by the value.
/// The enum is automatically assigned an appropriate `#[repr(u8 | u16 | u32 | u64)]`
/// based on the size of the range.
///
/// # Parameters
/// - `enum_name`: The name of the enum to be created.
/// - `start`: The starting value for the range of variants (inclusive).
/// - `end`: The ending value for the range of variants (inclusive).
///
/// # Panics
/// - Panics if the provided enum name is not a valid Rust identifier.
/// - Panics if `start` is greater than `end`.
/// - Panics if the `start` or `end` values are invalid integers.
///
/// # Example
/// ```rust
/// # use devela_macros::enumint;
/// enumint!(MyEnum, 2, 5);
/// assert_eq![2, MyEnum::_2 as u8];
/// ```
/// This will generate the following enum:
/// ```ignore
/// #[repr(u8)]
/// #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
/// enum MyEnum {
///     _2 = 2,
///     _3 = 3,
///     _4 = 4,
///     _5 = 5,
/// }
/// ```
#[proc_macro]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub fn enumint(input: TokenStream) -> TokenStream {
    body_enumint(input)
}
