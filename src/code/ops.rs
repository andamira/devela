// devela::code::ops
//
//! Overloadable operators.
//!
#![doc = crate::doc_!(extends: ops)]
//

// #![allow(unused_imports)] // DELETE

use crate::reexport;

// enums
reexport! { rust: core::ops, doc: "An endpoint of a range of keys.

See also `num::`[`Interval`][crate::Interval].",
Bound }
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
reexport! { rust: core::ops, doc: "The remainder operator `%`.", Rem }
reexport! { rust: core::ops, doc: "The remainder assignment operator `%=`.", RemAssign }
reexport! { rust: core::ops, doc: "The subtraction operator `-`.", Sub }
reexport! { rust: core::ops, doc: "The subtraction assignment operator `-=`.", SubAssign }
