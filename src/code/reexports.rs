// devela::code::reexports
//
//! Reexported items.
//

#![allow(unused_imports)]

use super::reexport;

/// <span class="stab portability" title="re-exported from `devela_macros`
/// crate">`devela_macros`</span>
pub use devela_macros::{
    cif, coalesce, compile, compile_attr, compile_doc, enumint, ident_total, ident_total_unique,
    ident_unique,
};

/* `core` re-exports */

// source code
reexport! { rust: core,
doc: "Expands to the column number at which it was invoked.", @column as code_column }
reexport! { rust: core,
doc: "Expands to the line number at which it was invoked.", @line as code_line }
reexport! { rust: core,
doc: "Expands to the file name at which it was invoked.", @file as code_file }
reexport! { rust: core,
doc: "Expands to a string representing the current module path.", @module_path as code_module }

// assert
reexport! { rust: core,
doc: "Asserts that a boolean expression is true at runtime.", assert }
reexport! { rust: core,
doc: "Asserts that two expressions are equal to each other.", assert_eq }
reexport! { rust: core,
doc: "Asserts that two expressions are not equal to each other.", assert_ne }
reexport! { rust: core,
doc: "Asserts that a boolean expression is true at runtime.", debug_assert }
reexport! { rust: core,
doc: "Asserts that two expressions are equal to each other.", debug_assert_eq }
reexport! { rust: core,
doc: "Asserts that two expressions are not equal to each other.", debug_assert_ne }

// cfg
reexport! { rust: core,
doc: "Evaluates boolean combinations of configuration flags at compile-time.", cfg }

// include
reexport! { rust: core,
doc: "Parses a file as an expression or an item according to the context.", include }
reexport! { rust: core,
doc: "Includes a file as a reference to a byte array.", include_bytes }
reexport! { rust: core,
doc: "Includes a UTF-8 encoded file as a string.", include_str }

// concatenating
reexport! { rust: core, doc: "Concatenates literals into a static string slice.", concat }
reexport! { rust: core, doc: "Stringifies its arguments.", stringify }
// WAIT: [concat_idents](https://github.com/rust-lang/rust/issues/29599)
// reexport! { rust: core, doc: "Concatenates identifiers into one identifier.", concat_idents }
// WAIT: [concat_bytes](https://github.com/rust-lang/rust/issues/87555)
// reexport! { rust: core, doc: "Concatenates literals into a byte slice.", concat_bytes }

/* `core::clone` re-exports */

// NOTE: the trait and the derive macro have the same name
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::clone::Clone;

/* `core::convert` re-exports */

// enums
reexport! { rust: core::convert,
doc: "The error type for errors that can never happen.", Infallible }

// traits
reexport! { rust: core::convert,
doc: "Used to do a cheap mutable-to-mutable reference conversion.", AsMut }
reexport! { rust: core::convert,
doc: "Used to do a cheap reference-to-reference conversion.", AsRef }
reexport! { rust: core::convert,
doc: "Used to do value-to-value conversions while consuming the input value.", From }
reexport! { rust: core::convert,
doc: "A value-to-value conversion that consumes the input value.", Into }
reexport! { rust: core::convert,
doc: "Simple and safe type conversions that may fail in a controlled way.", TryFrom }
reexport! { rust: core::convert,
doc: "An attempted conversion that consumes self, which may or may not be expensive.",
    TryInto }

// functions
reexport! { rust: core::convert,
doc: "The identity function.", identity }

/* `core::default` re-exports */

// NOTE: the trait and the derive macro have the same name
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::default::Default;

/* `core::hint` re-exports */

reexport! { rust: core::hint,
doc: "Makes a *soundness* promise to the compiler that `cond` holds.", assert_unchecked }
reexport! { rust: core::hint,
doc: "Hints the compiler to be maximally pessimistic about what black_box could do.", black_box }
reexport! { rust: core::hint,
doc: "Signals the processor that it is running in a busy-wait spin-loop.", spin_loop }
reexport! { rust: core::hint,
doc: "Informs the compiler that the current calling site is not reachable.", unreachable_unchecked }

/* `core::marker` re-exports */

/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
// <https://doc.rust-lang.org/nomicon/phantom-data.html#table-of-phantomdata-patterns>
pub use core::marker::PhantomData;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::PhantomPinned;

// NOTE: the trait and the derive macro have the same name
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Copy;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Send;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Sized;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Sync;
/// <span class='stab portability' title='re-exported from rust&#39;s `core`'>`core`</span>
pub use core::marker::Unpin;

/* `core::ops` re-exports */

// enums
reexport! { rust: core::ops, doc: "An endpoint of a range of keys.", Bound }
reexport! { rust: core::ops,
doc: "Used to tell an operation whether it should exit early or go on as usual.", ControlFlow }

// structs
reexport! { rust: core::ops,
doc: "A range bounded inclusively below and exclusively above (`start..end`).", Range }
reexport! { rust: core::ops, doc: "A range only bounded inclusively below (`start..`).",
RangeFrom }
reexport! { rust: core::ops, doc: "An unbounded range (`..`).",
RangeFull }
reexport! { rust: core::ops, doc: "A range bounded inclusively below and above (`start..=end`).",
RangeInclusive }
reexport! { rust: core::ops, doc: "A range only bounded exclusively above (`..end`).",
RangeTo }
reexport! { rust: core::ops, doc: "A range only bounded inclusively above (`..=end`).",
RangeToInclusive }

// traits
reexport! { rust: core::ops, doc: "The addition operator `+`.", Add }
reexport! { rust: core::ops, doc: "The addition assignment operator `+=`.", AddAssign }
reexport! { rust: core::ops, doc: "The bitwise AND operator `&`.", BitAnd }
reexport! { rust: core::ops, doc: "The bitwise AND assignment operator `&=`.", BitAndAssign }
reexport! { rust: core::ops, doc: "The bitwise OR operator `|`.", BitOr }
reexport! { rust: core::ops, doc: "The bitwise OR assignment operator `|=`.", BitOrAssign }
reexport! { rust: core::ops, doc: "The bitwise XOR operator `^`.", BitXor }
reexport! { rust: core::ops, doc: "The bitwise XOR assignment operator `^=`.", BitXorAssign }
reexport! { rust: core::ops, doc: "Used for immutable dereferencing operations, like `*v`.", Deref }
reexport! { rust: core::ops, doc: "Used for mutable dereferencing operations, like in `*v = 1;`.",
DerefMut }
reexport! { rust: core::ops, doc: "The division operator `/`.", Div }
reexport! { rust: core::ops, doc: "The division assignment operator `/=`.", DivAssign }
reexport! { rust: core::ops, doc: "Custom code within the destructor.", Drop }
reexport! { rust: core::ops,
doc: "The version of the call operator that takes an immutable receiver.", Fn }
reexport! { rust: core::ops,
doc: "The version of the call operator that takes a mutable receiver.", FnMut }
reexport! { rust: core::ops,
doc: "The version of the call operator that takes a by-value receiver.", FnOnce }
reexport! { rust: core::ops,
doc: "Used for indexing operations (`container[index]`) in immutable contexts.", Index }
reexport! { rust: core::ops,
doc: "Used for indexing operations (`container[index]`) in mutable contexts.", IndexMut }
reexport! { rust: core::ops, doc: "The multiplication operator `*`.", Mul }
reexport! { rust: core::ops, doc: "The multiplication assignment operator `*=`.", MulAssign }
reexport! { rust: core::ops, doc: "The unary negation operator `-`.", Neg }
reexport! { rust: core::ops, doc: "The unary logical negation operator `!`.", Not }
reexport! { rust: core::ops, doc: "Implemented by Rust’s built-in range types", RangeBounds }
