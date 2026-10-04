use crate::{_reexport, _tags};

/* traits */

// logic ops
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait BitAnd, tag: _tags!(logic),
    doc: "The bitwise AND operator `&`.", BitAnd
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait BitAndAssign, tag: _tags!(logic),
    doc: "The bitwise AND assignment operator `&=`.", BitAndAssign
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait BitOr, tag: _tags!(logic),
    doc: "The bitwise OR operator `|`.", BitOr
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait BitOrAssign, tag: _tags!(logic),
    doc: "The bitwise OR assignment operator `|=`.", BitOrAssign
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait BitXor, tag: _tags!(logic),
    doc: "The bitwise XOR operator `^`.", BitXor
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait BitXorAssign, tag: _tags!(logic),
    doc: "The bitwise XOR assignment operator `^=`.", BitXorAssign
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait Not, tag: _tags!(logic),
    doc: "The unary logical negation operator `!`.", Not
}

// num ops
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait Add, tag: _tags!(num),
    doc: "The addition operator `+`.", Add
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait AddAssign, tag: _tags!(num),
    doc: "The addition assignment operator `+=`.", AddAssign
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait Div, tag: _tags!(num),
    doc: "The division operator `/`.", Div
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait DivAssign, tag: _tags!(num),
    doc: "The division assignment operator `/=`.", DivAssign
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait Mul, tag: _tags!(num),
    doc: "The multiplication operator `*`.", Mul
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait MulAssign, tag: _tags!(num),
    doc: "The multiplication assignment operator `*=`.", MulAssign
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait Rem, tag: _tags!(num),
    doc: "The remainder operator `%`.", Rem
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait RemAssign, tag: _tags!(num),
    doc: "The remainder assignment operator `%=`.", RemAssign
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait Sub, tag: _tags!(num),
    doc: "The subtraction operator `-`.", Sub
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait SubAssign, tag: _tags!(num),
    doc: "The subtraction assignment operator `-=`.", SubAssign
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait Neg, tag: _tags!(num),
    doc: "The unary negation operator `-`.", Neg
}

_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait Deref, tag: _tags!(lifetime),
    doc: "Used for immutable dereferencing operations, like `*v`.", Deref
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait DerefMut, tag: _tags!(lifetime),
    doc: "Used for mutable dereferencing operations, like in `*v = 1;`.", DerefMut
}

_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait Drop, tag: _tags!(guard),
    doc: "Custom code within the destructor.", Drop
}

// NOTE: Fn* traits re-exported from code/ops/call

_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait Index, tag: _tags!(value lifetime),
    doc: "Used for indexing operations (`container[index]`) in immutable contexts.", Index
}
_reexport! { rust: core::ops,
    location: "code/ops/overload" => trait IndexMut, tag: _tags!(value lifetime),
    doc: "Used for indexing operations (`container[index]`) in mutable contexts.", IndexMut
}
