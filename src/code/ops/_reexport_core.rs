// devela::code::ops::_reexport_core

use crate::{_reexport, _tags};

/* enums */

_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(quant),
doc: "An endpoint of a range of keys.",
+doc: "See also `num::`[`Interval`][crate::Interval].", Bound }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(code),
doc: "Used to tell an operation whether it should exit early or go on as usual.", ControlFlow }

/* structs */

_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(quant),
doc: "A range bounded inclusively below and exclusively above (`start..end`).", Range }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(quant),
doc: "A range only bounded inclusively below (`start..`).", RangeFrom }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(quant),
doc: "An unbounded range (`..`).", RangeFull }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(quant),
doc: "A range bounded inclusively below and above (`start..=end`).", RangeInclusive }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(quant),
doc: "A range only bounded exclusively above (`..end`).", RangeTo }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(quant),
doc: "A range only bounded inclusively above (`..=end`).", RangeToInclusive }

/* traits */

_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(quant),
doc: "Implemented by Rust's built-in range types", RangeBounds }

// logic ops
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(logic),
doc: "The bitwise AND operator `&`.", BitAnd }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(logic),
doc: "The bitwise AND assignment operator `&=`.", BitAndAssign }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(logic),
doc: "The bitwise OR operator `|`.", BitOr }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(logic),
doc: "The bitwise OR assignment operator `|=`.", BitOrAssign }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(logic),
doc: "The bitwise XOR operator `^`.", BitXor }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(logic),
doc: "The bitwise XOR assignment operator `^=`.", BitXorAssign }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(logic),
doc: "The unary logical negation operator `!`.", Not }

// num ops
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(num),
doc: "The addition operator `+`.", Add }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(num),
doc: "The addition assignment operator `+=`.", AddAssign }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(num),
doc: "The division operator `/`.", Div }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(num),
doc: "The division assignment operator `/=`.", DivAssign }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(num),
doc: "The multiplication operator `*`.", Mul }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(num),
doc: "The multiplication assignment operator `*=`.", MulAssign }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(num),
doc: "The remainder operator `%`.", Rem }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(num),
doc: "The remainder assignment operator `%=`.", RemAssign }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(num),
doc: "The subtraction operator `-`.", Sub }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(num),
doc: "The subtraction assignment operator `-=`.", SubAssign }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(num),
doc: "The unary negation operator `-`.", Neg }

_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(lifetime),
doc: "Used for immutable dereferencing operations, like `*v`.", Deref }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(lifetime),
doc: "Used for mutable dereferencing operations, like in `*v = 1;`.", DerefMut }

_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(guard),
doc: "Custom code within the destructor.", Drop }

_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(code),
doc: "The version of the call operator that takes an immutable receiver.", Fn }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(code),
doc: "The version of the call operator that takes a mutable receiver.", FnMut }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(code),
doc: "The version of the call operator that takes a by-value receiver.", FnOnce }

_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(value lifetime),
doc: "Used for indexing operations (`container[index]`) in immutable contexts.", Index }
_reexport! { rust: core::ops, location: "code/ops", tag: _tags!(value lifetime),
doc: "Used for indexing operations (`container[index]`) in mutable contexts.", IndexMut }
