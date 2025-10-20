// devela_base_macros::lib
//
#![doc = include_str!("./Lib.md")]
//!
//
// TOC
// - helpers
// - compile:
//   - cif
//   - compile
//   - compile_attr
//   - compile_doc (hidden)
// - ident:
//   - ident_total
//   - ident_total_unique
//   - ident_unique
// - misc:
//   - coalesce
//   - field_of
//   - repeat
//
// WAIT: [proc_macro_hygiene](https://github.com/rust-lang/rust/issues/54727#issuecomment-485181171)

#![forbid(unsafe_code)]
#![cfg_attr(nightly_doc, feature(doc_cfg))]

extern crate self as devela_base_macros;
use proc_macro::TokenStream as TS;
use std::collections::HashSet;

mod bodies;
use bodies::*;

/* helpers */

// Allows a group of items to share the same cfg options.
#[allow(unused_macros)] #[rustfmt::skip] macro_rules! items { ($($item:item)*) => { $($item)* }; }
items! { #[allow(unused_imports)] use items; }

/* macros: compile */

/// Evaluates to either a `true` of `false` literal based on the [predicate].
///
/// [predicate]: https://andamira.github.io/devela/latest/devela/_info/macros/#compilation-predicates
#[doc = concat!("# Example\n```\n", include_str!("../examples/cif.rs"), "\n```")]
#[proc_macro] #[rustfmt::skip]
pub fn cif(input: TS) -> TS { body_cif(input) }

/// Conditionally compiles the thing it is attached to based on the [predicate].
///
/// [predicate]: https://andamira.github.io/devela/latest/devela/_info/macros/#compilation-predicates
#[doc = concat!("# Example\n```\n", include_str!("../examples/compile.rs"), "\n```")]
#[proc_macro_attribute] #[rustfmt::skip]
pub fn compile(args: TS, input: TS) -> TS { body_compile(args, input) }

/// Conditionally compiles the given attributes based on the [predicate].
///
/// [predicate]: https://andamira.github.io/devela/latest/devela/_info/macros/#compilation-predicates
///
#[doc = concat!("# Example\n```\n", include_str!("../examples/compile_attr.rs"), "\n```")]
#[proc_macro_attribute] #[rustfmt::skip]
pub fn compile_attr(args: TS, input: TS) -> TS { body_compile_attr(args, input) }

#[doc(hidden)]
/// Conditionally compiles each doc comment based on the [predicate].
///
/// [predicate]: https://andamira.github.io/devela/latest/devela/_info/macros/#compilation-predicates
///
#[doc = concat!("# Example\n```\n", include_str!("../examples/compile_doc.rs"), "\n```")]
#[proc_macro_attribute] #[rustfmt::skip]
pub fn compile_doc(args: TS, input: TS) -> TS { body_compile_doc(args, input) }

/* macros: ident */

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
/// # Example
/// ```
/// # use devela_base_macros::ident_total;
/// assert_eq![ident_total!(a, a 東 r#true; a3 != 3a), 5];
/// ```
#[proc_macro] #[rustfmt::skip]
pub fn ident_total(input: TS) -> TS { body_ident_total(input) }

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
/// # Example
/// ```
/// # use devela_base_macros::ident_total_unique;
/// assert_eq![ident_total_unique!(a, a 東 r#true; a3 != 3a), [5, 4]];
/// ```
#[proc_macro] #[rustfmt::skip]
pub fn ident_total_unique(input: TS) -> TS { body_ident_total_unique(input) }

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
/// # Example
/// ```
/// # use devela_base_macros::ident_unique;
/// assert_eq![ident_unique!(a, a 東 r#true; a3 != 3a), 4];
/// ```
#[proc_macro] #[rustfmt::skip]
pub fn ident_unique(input: TS) -> TS { body_ident_unique(input) }

/* macros: misc. */

/// Returns the first non-empty argument.
///
/// If all arguments are empty, the macro returns nothing.
///
/// This macro is inspired by the SQL `COALESCE` function, which returns the first
/// non-null value from a list of arguments, or null if all the arguments are null.
///
#[doc = concat!("# Example\n```\n", include_str!("../examples/coalesce.rs"), "\n```")]
#[proc_macro] #[rustfmt::skip]
pub fn coalesce(input: TS) -> TS { body_coalesce(input) }

/// Generates an expression for accessing a field of a tuple or struct.
///
/// Constructs an expression in the form `<value>.<field>`, where `<value>`
/// is a tuple or struct, and `<field>` is the field to access, either
/// a zero-based index (for tuples) or a named field (for structs).
///
/// # Example
/// ```
/// # use devela_base_macros::field_of;
/// let my_tuple = (42, 100, 300);
/// let value = field_of!(my_tuple, 1); // expands to `my_tuple.1`
/// assert_eq!(value, 100);
///
/// struct MyStruct { x: i32, y: i32 }
/// let my_struct = MyStruct { x: 10, y: 20 };
/// let value = field_of!(my_struct, y); // expands to `my_struct.y`
/// assert_eq!(value, 20);
/// ```
#[proc_macro] #[rustfmt::skip]
pub fn field_of(input: TS) -> TS { body_field_of(input) }

/// Repeats an expression the given number of times, as duplicated code with no loops.
///
/// # Example
/// ```
/// # use devela_base_macros::repeat;
/// let mut a = 0;
/// repeat![3, a += 1];
/// assert_eq![a, 3];
/// ```
#[proc_macro] #[rustfmt::skip]
pub fn repeat(input: TS) -> TS { body_repeat(input) }
