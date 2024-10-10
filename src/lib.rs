// devela_macros
//
//!
#![doc = include_str!("./Lib.md")]
//

// warnings
#![warn(clippy::all)]
// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly_doc", feature(doc_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::{
    format,
    string::{String, ToString},
};

extern crate proc_macro;
use core::fmt::Write;
use proc_macro::{Ident, Span, TokenStream, TokenTree};

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
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
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
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
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
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
#[doc(hidden)]
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
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
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
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub fn coalesce(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let args = split_args(&input);

    let first_non_empty_arg = args.into_iter().find(|arg| !arg.is_empty()).unwrap_or_default();
    // .unwrap_or_else(|| "".to_string());
    // .expect("No non-empty arguments found");

    first_non_empty_arg.parse().expect("Failed to parse TokenStream")
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
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
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
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "std")))]
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
    let input_str = input.to_string();

    let parts: Vec<&str> = input_str.split(',').collect();
    if parts.len() != 3 {
        panic!("Expected format: enum_name, start, end");
    }

    let enum_name_str = parts[0].trim();
    let enum_name = Ident::new(enum_name_str, Span::call_site()); // will panic if invalid
    let start: u64 = parts[1].trim().parse().expect("Invalid start value");
    let end: u64 = parts[2].trim().parse().expect("Invalid end value");
    if start > end {
        panic!("Start value must be less than or equal to end value");
    }

    let range_length = end - start + 1;
    let repr = if range_length <= u8::MAX as u64 {
        "u8"
    } else if range_length <= u16::MAX as u64 {
        "u16"
    } else if range_length <= u32::MAX as u64 {
        "u32"
    } else {
        "u64"
    };

    let mut enum_variants = String::new();
    for i in start..=end {
        write!(enum_variants, "_{} = {},", i, i).unwrap();
    }

    let enum_definition = format!(
        "/// An auto-generated enum for values between {start} and {end}.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr({repr})]
        enum {enum_name} {{ {enum_variants} }}

        impl {enum_name} {{
            /* constants */

            /// Returns the number of valid values.
            pub const VALID_VALUES: {repr} = {range_length};

            /// Returns the number of invalid values.
            pub const NICHE_VALUES: {repr} = {repr}::MAX - Self::VALID_VALUES + 1;

            /// Returns the minimum possible value.
            pub const MIN: {repr} = {start};

            /// Returns the maximum possible value.
            pub const MAX: {repr} = {end};

            /* methods */

            /// Try to create an enum from the underlying representation.
            #[inline]
            pub const fn new(value: {repr}) -> Option<Self> {{
                if value >= {start} && value <= {end} {{
                    Some(unsafe {{ core::mem::transmute(value) }})
                }} else {{
                    None
                }}
            }}

            /// Cast the enum to its underlying representation.
            #[inline]
            #[must_use]
            pub const fn get(self) -> {repr} {{
                self as {repr}
            }}

            /* arithmetic */

            /// Checked addition. Returns `None` if overflow occurs.
            #[inline] #[must_use]
            pub const fn checked_add(self, other: Self) -> Option<Self> {{
                let result = self.get().checked_add(other.get());
                if let Some(result) = result {{
                    Self::new(result)
                }} else {{
                    None
                }}
            }}

            /// Saturating addition. Returns the maximum value if overflow occurs.
            #[inline] #[must_use]
            pub const fn saturating_add(self, other: Self) -> Self {{
                let result = self.get().saturating_add(other.get());
                match Self::new(result) {{
                    Some(value) => value,
                    None => match Self::new(Self::_{end}.get()) {{
                        Some(max) => max,
                        None => Self::_{end}, // Fallback to end (shouldn't happen)
                    }},
                }}
            }}

            /// Wrapping (modular) addition. Wraps around on overflow.
            #[inline] #[must_use]
            pub const fn wrapping_add(self, other: Self) -> Self {{
                let range_size = Self::_{end}.get() - Self::_{start}.get() + 1;
                let result = (self.get() - Self::_{start}.get() + other.get())
                                % range_size + Self::_{start}.get();
                match Self::new(result) {{
                    Some(value) => value,
                    None => Self::_{start},  // Fallback to start (shouldn't happen)
                }}
            }}
        }}"
    );
    enum_definition.parse().unwrap()
}
