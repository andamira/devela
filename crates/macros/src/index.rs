// devela_macros::index
//
// NOTE: proc. macro crates can only export procedural macros.
//
//! A development substrate of coherence.
//!
//! Procedural macros for devela.
//!
#![doc = include_str!("./docs/compile.md")]
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
// - macro:
//   - macro_apply
//   - macro_derive
//   - macro_derive_with
// - misc:
//   - coalesce
//   - enumint
//   - field_of
//   - paste
//   - repeat
//
// WAIT: [proc_macro_hygiene](https://github.com/rust-lang/rust/issues/54727#issuecomment-485181171)
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(nightly_doc, feature(doc_cfg))]
// ----------------------------
// `nightly_stable_later`: 1.?? core, alloc, std, not(miri)…
#![cfg_attr(nightly_stable_later, feature(proc_macro_diagnostic, proc_macro_value))]

/* crate safeguards */

// safety
#[cfg(all(feature = "safe", any(feature = "unsafe", feature = "unsafe_layout")))]
compile_error!("You can't enable `safe` and any `unsafe*` features at the same time.");

extern crate self as devela_macros;
macro_rules! __crate_name {
    () => {
        "devela_macros"
    };
}
pub(crate) use __crate_name;

use proc_macro::TokenStream as TS;
use std::collections::HashSet;

mod bodies;
mod copied;
use {bodies::*, copied::*};

// mod _doc;
// mod yard;

/* helpers */

// Allows a group of items to share the same cfg options.
#[allow(unused_macros)] #[rustfmt::skip] macro_rules! items { ($($item:item)*) => { $($item)* }; }
items! { #[allow(unused_imports)] use items; }

/* compile */

/// Evaluates to either a `true` of `false` literal based on the [predicate].
#[doc = crate::_doc_location!(proc "code/util")]
///
#[doc = doclink!(devela "[predicate]" "_doc/macros" @mod "#compilation-predicates")]
#[doc = concat!("# Example\n```\n", include_str!("../examples/cif.rs"), "\n```")]
#[proc_macro] #[rustfmt::skip]
pub fn cif(input: TS) -> TS { body_cif(input) }

/// Conditionally compiles the thing it is attached to based on the [predicate].
#[doc = crate::_doc_location!(proc "code/util")]
///
#[doc = doclink!(devela "[predicate]" "_doc/macros" @mod "#compilation-predicates")]
#[doc = concat!("# Example\n```\n", include_str!("../examples/compile.rs"), "\n```")]
#[proc_macro_attribute] #[rustfmt::skip]
pub fn compile(args: TS, input: TS) -> TS { body_compile(args, input) }

/// Conditionally compiles the given attributes based on the [predicate].
#[doc = crate::_doc_location!(proc "code/util")]
///
#[doc = doclink!(devela "[predicate]" "_doc/macros" @mod "#compilation-predicates")]
///
#[doc = concat!("# Example\n```\n", include_str!("../examples/compile_attr.rs"), "\n```")]
#[proc_macro_attribute] #[rustfmt::skip]
pub fn compile_attr(args: TS, input: TS) -> TS { body_compile_attr(args, input) }

#[doc(hidden)]
/// Conditionally compiles each doc comment based on the [predicate].
#[doc = crate::_doc_location!(proc "code/util")]
///
#[doc = doclink!(devela "[predicate]" "_doc/macros" @mod "#compilation-predicates")]
///
#[doc = concat!("# Example\n```\n", include_str!("../examples/compile_doc.rs"), "\n```")]
#[proc_macro_attribute] #[rustfmt::skip]
pub fn compile_doc(args: TS, input: TS) -> TS { body_compile_doc(args, input) }

/* ident */

/// Returns the total number of [identifiers] in its input.
#[doc = crate::_doc_location!(proc "code/util")]
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
/// # use devela_macros::ident_total;
/// assert_eq![ident_total!(a, a 東 r#true; a3 != 3a), 5];
/// ```
#[proc_macro] #[rustfmt::skip]
pub fn ident_total(input: TS) -> TS { body_ident_total(input) }

/// Returns the numbers of both *total* and *unique* [identifiers] in its input.
#[doc = crate::_doc_location!(proc "code/util")]
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
/// # use devela_macros::ident_total_unique;
/// assert_eq![ident_total_unique!(a, a 東 r#true; a3 != 3a), [5, 4]];
/// ```
#[proc_macro] #[rustfmt::skip]
pub fn ident_total_unique(input: TS) -> TS { body_ident_total_unique(input) }

/// Returns the number of *unique* [identifiers] in its input.
#[doc = crate::_doc_location!(proc "code/util")]
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
/// # use devela_macros::ident_unique;
/// assert_eq![ident_unique!(a, a 東 r#true; a3 != 3a), 4];
/// ```
#[proc_macro] #[rustfmt::skip]
pub fn ident_unique(input: TS) -> TS { body_ident_unique(input) }

/* derive */

/// Applies a declarative macro to the decorated item.
#[doc = crate::_doc_location!(proc "code/util")]
///
/// Expands `#[macro_apply(m)] item` as `m! { item }`.
///
/// If arguments are provided, `#[macro_apply(m(args...))] item`
/// expands as `m! { (args...) item }`.
///
/// The macro receives ownership of the item and must re-emit it if it should remain.
///
/// This attribute cannot be used on out-of-line file modules such as
/// `mod name;`, because file modules in proc-macro input are unstable.
/// Use the generated alias macro directly for those cases.
///
/// # Examples
/// ```ignore
#[doc = include_str!("./docs/macro_apply_examples.rs")]
/// ```
#[doc = crate::_doc_vendor!("macro_rules_attribute")]
#[proc_macro_attribute] #[rustfmt::skip]
pub fn macro_apply(args: TS, input: TS) -> TS { body_macro_apply(args, input) }

/// Runs classic derives and declarative derives from one list.
#[doc = crate::_doc_location!(proc "code/util")]
///
/// Entries ending in `!` are called as declarative macros.
/// Other entries are forwarded to Rust's built-in `derive`.
///
/// Declarative derives may receive call-local arguments:
/// `Name!(args...)` expands as `Name! { (args...) item }`.
///
/// The optional helper attribute `#[macro_derive_args(...)]`
/// may be used to pass item-local arguments to declarative derives.
///
/// # Examples
/// ```ignore
#[doc = include_str!("./docs/macro_derive_examples.rs")]
/// ```
#[doc = crate::_doc_vendor!("macro_rules_attribute")]
#[proc_macro_attribute] #[rustfmt::skip]
pub fn macro_derive(args: TS, input: TS) -> TS { body_macro_derive(args, input) }

/// Runs declarative derive-like macros over the decorated item.
#[doc = crate::_doc_location!(proc "code/util")]
///
/// Each macro receives a copy of the item and may emit impls or side-items.
/// The original item is preserved.
///
/// A macro may receive call-local arguments:
/// `m(args...)` expands as `m! { (args...) item }`.
///
/// The optional helper attribute `#[macro_derive_args(...)]`
/// may be used to pass item-local arguments to those declarative derives.
///
/// # Examples
/// ```ignore
#[doc = include_str!("./docs/macro_derive_with_examples.rs")]
/// ```
#[doc = crate::_doc_vendor!("macro_rules_attribute")]
#[proc_macro_attribute] #[rustfmt::skip]
pub fn macro_derive_with(args: TS, input: TS) -> TS { body_macro_derive_with(args, input) }

#[doc(hidden)] #[rustfmt::skip]
/// No-op derive used by [`macro_derive`] and [`macro_derive_with`].
///
/// It admits helper attributes such as `#[macro_derive_args(...)]`
/// on items inspected by declarative derive macros.
#[proc_macro_derive(__macro_derive_helpers, attributes(macro_derive_args))]
pub fn __macro_derive_helpers(_: TS) -> TS { TS::new() }

/* misc. */

/// Returns the first non-empty argument.
#[doc = crate::_doc_location!(proc "code/util")]
///
/// If all arguments are empty, the macro returns nothing.
///
/// This macro is inspired by the SQL `COALESCE` function, which returns the first
/// non-null value from a list of arguments, or null if all the arguments are null.
///
#[doc = concat!("# Example\n```\n", include_str!("../examples/coalesce.rs"), "\n```")]
#[proc_macro] #[rustfmt::skip]
pub fn coalesce(input: TS) -> TS { body_coalesce(input) }

// #[doc = base::_tags!(construction niche procedural_macro)]
/// Defines a compact enum over a contiguous integer interval.
#[doc = crate::_doc_location!(proc "code/util")]
///
#[doc = include_str!("docs/enumint.md")]
// #[doc = concat!("# Example\n```\n", include_str!("../examples/enumint.rs"), "\n```")]
#[proc_macro] #[rustfmt::skip]
pub fn enumint(input: TS) -> TS { body_enumint(input) }

/// Generates an expression for accessing a field of a tuple or struct.
#[doc = crate::_doc_location!(proc "code/util")]
///
/// Constructs an expression in the form `<value>.<field>`, where `<value>`
/// is a tuple or struct, and `<field>` is the field to access, either
/// a zero-based index (for tuples) or a named field (for structs).
///
/// # Example
/// ```
/// # use devela_macros::field_of;
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

/// Allows to paste identifiers together.
#[doc = crate::_doc_location!("code/util")]
///
#[doc = include_str!("docs/paste.md")]
#[doc = crate::_doc_vendor!("pastey")]
#[proc_macro] #[rustfmt::skip]
pub fn paste(input: TS) -> TS { body_paste(input) }

/// Repeats an expression the given number of times, as duplicated code with no loops.
#[doc = crate::_doc_location!(proc "code/util")]
///
/// # Example
/// ```
/// # use devela_macros::repeat;
/// let mut a = 0;
/// repeat![3, a += 1];
/// assert_eq![a, 3];
/// ```
#[proc_macro] #[rustfmt::skip]
pub fn repeat(input: TS) -> TS { body_repeat(input) }
