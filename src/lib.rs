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

/* imports */

#[cfg(feature = "alloc")]
extern crate alloc;

extern crate proc_macro;
#[cfg(feature = "alloc")]
use proc_macro::TokenStream;

#[cfg(feature = "dep_hashbrown")]
use hashbrown::HashSet;
#[cfg(all(not(feature = "dep_hashbrown"), feature = "std"))]
use std::collections::HashSet;

mod bodies;
#[cfg_attr(not(feature = "alloc"), allow(unused_imports))]
use bodies::*;

/* inner helpers */

// Allows a group of items to share the same cfg options.
#[allow(unused_macros)]
macro_rules! items { ( $($item:item)* ) => { $($item)* }; }
#[allow(unused_imports)]
use items;

/* macros: compile */

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

/* macros: ident */

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
#[cfg(any(feature = "dep_hashbrown", feature = "std"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
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
#[cfg(any(feature = "dep_hashbrown", feature = "std"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
pub fn ident_unique(input: TokenStream) -> TokenStream {
    body_ident_unique(input)
}

/* macros: niche */

/// Generates a unit-only enum with variants associated to a specified range.
///
/// This macro generates an enum with integer variants named `P#` for positive
/// vales and `N#` for negative values.
///
/// It allows to represent integers with valid range of values, and where the
/// invalid values can be used by the compiler for memory niche optimization.
///
/// It only supports 8-bit and 16-bit representations to avoid excessive time
/// and memory usage during compilation.
///
/// # Usage
/// ```
/// # use devela_macros::enumint;
/// // [visibility] name, repr, start, end
/// enumint![pub MyEnum, i8, -10, 10];
/// ```
///
/// # Parameters
/// - `visibility`: Optional visibility indicator. E.g. `pub(crate)`.
/// - `name`: The name of the enum to be created.
/// - `repr`: the data representation. E.g `u8`.
/// - `start`: The starting value for the range of variants (inclusive).
/// - `end`: The ending value for the range of variants (inclusive).
///
/// # Panics
/// - Panics if any given value is not of the kind expected.
/// - Panics if `start` or `end` are outside the `repr` representable range.
/// - Panics if `start` is greater than `end`.
///
/// # Example
/// ```
#[doc = include_str!("../examples/enumint.rs")]
/// ```
#[proc_macro]
#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub fn enumint(input: TokenStream) -> TokenStream {
    body_enumint(input)
}
