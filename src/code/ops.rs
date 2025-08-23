// devela::code::ops
//
//! Overloadable operators.
//!
#![doc = crate::doc_!(extends: ops)]
//

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::reexports::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::reexports::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}

mod reexports {
    use crate::{_reexport, TAG_QUANT};

    // enums
    _reexport! { rust: core::ops, tag: TAG_QUANT!(),
    doc: "An endpoint of a range of keys.\n\n See also `num::`[`Interval`][crate::Interval].",
    Bound }
    _reexport! { rust: core::ops,
    doc: "Used to tell an operation whether it should exit early or go on as usual.", ControlFlow }

    // structs
    _reexport! { rust: core::ops, tag: TAG_QUANT!(),
    doc: "A range bounded inclusively below and exclusively above (`start..end`).", Range }
    _reexport! { rust: core::ops, tag: TAG_QUANT!(),
    doc: "A range only bounded inclusively below (`start..`).", RangeFrom }
    _reexport! { rust: core::ops, tag: TAG_QUANT!(), doc: "An unbounded range (`..`).", RangeFull }
    _reexport! { rust: core::ops, tag: TAG_QUANT!(),
    doc: "A range bounded inclusively below and above (`start..=end`).", RangeInclusive }
    _reexport! { rust: core::ops, tag: TAG_QUANT!(),
    doc: "A range only bounded exclusively above (`..end`).", RangeTo }
    _reexport! { rust: core::ops, tag: TAG_QUANT!(),
    doc: "A range only bounded inclusively above (`..=end`).", RangeToInclusive }

    // traits
    _reexport! { rust: core::ops, doc: "The addition operator `+`.", Add }
    _reexport! { rust: core::ops, doc: "The addition assignment operator `+=`.", AddAssign }
    _reexport! { rust: core::ops, doc: "The bitwise AND operator `&`.", BitAnd }
    _reexport! { rust: core::ops, doc: "The bitwise AND assignment operator `&=`.", BitAndAssign }
    _reexport! { rust: core::ops, doc: "The bitwise OR operator `|`.", BitOr }
    _reexport! { rust: core::ops, doc: "The bitwise OR assignment operator `|=`.", BitOrAssign }
    _reexport! { rust: core::ops, doc: "The bitwise XOR operator `^`.", BitXor }
    _reexport! { rust: core::ops, doc: "The bitwise XOR assignment operator `^=`.", BitXorAssign }
    _reexport! { rust: core::ops,
    doc: "Used for immutable dereferencing operations, like `*v`.", Deref }
    _reexport! { rust: core::ops,
    doc: "Used for mutable dereferencing operations, like in `*v = 1;`.", DerefMut }
    _reexport! { rust: core::ops, doc: "The division operator `/`.", Div }
    _reexport! { rust: core::ops, doc: "The division assignment operator `/=`.", DivAssign }
    _reexport! { rust: core::ops, doc: "Custom code within the destructor.", Drop }
    _reexport! { rust: core::ops,
    doc: "The version of the call operator that takes an immutable receiver.", Fn }
    _reexport! { rust: core::ops,
    doc: "The version of the call operator that takes a mutable receiver.", FnMut }
    _reexport! { rust: core::ops,
    doc: "The version of the call operator that takes a by-value receiver.", FnOnce }
    _reexport! { rust: core::ops,
    doc: "Used for indexing operations (`container[index]`) in immutable contexts.", Index }
    _reexport! { rust: core::ops,
    doc: "Used for indexing operations (`container[index]`) in mutable contexts.", IndexMut }
    _reexport! { rust: core::ops, doc: "The multiplication operator `*`.", Mul }
    _reexport! { rust: core::ops, doc: "The multiplication assignment operator `*=`.", MulAssign }
    _reexport! { rust: core::ops, doc: "The unary negation operator `-`.", Neg }
    _reexport! { rust: core::ops, doc: "The unary logical negation operator `!`.", Not }
    _reexport! { rust: core::ops, doc: "Implemented by Rust’s built-in range types", RangeBounds }
    _reexport! { rust: core::ops, doc: "The remainder operator `%`.", Rem }
    _reexport! { rust: core::ops, doc: "The remainder assignment operator `%=`.", RemAssign }
    _reexport! { rust: core::ops, doc: "The subtraction operator `-`.", Sub }
    _reexport! { rust: core::ops, doc: "The subtraction assignment operator `-=`.", SubAssign }
}
