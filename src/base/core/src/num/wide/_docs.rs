// method's docs as consts
crate::CONST! { hidden macro_export,

/* method api docs */

_LANE_AUTO =
"This method is a thin wrapper over the best available alternative (simd→wide→plain)";
// MAYBE Refer to that method's documentation.

/* simd docs */

_ADD_ASSIGN_SIMD = "
Adds each lane of `rhs` to the corresponding lane in `self` using SIMD.\n\n
Performs elementwise addition with the semantics of the underlying element type.\n\n
Integer addition wraps on overflow; floating-point follows IEEE-754 rules.";
_SUB_ASSIGN_SIMD = "
Subtracts each lane of `rhs` from the corresponding lane in `self` using SIMD.
\n\nPerforms elementwise subtraction with the semantics of the underlying element type.\n\n
Integer subtraction wraps on overflow; floating-point follows IEEE-754 rules.
";
_MUL_ASSIGN_SIMD = "
Multiplies each lane of `self` by the corresponding lane of `rhs` using SIMD.
\n\nPerforms elementwise multiplication with the semantics of the underlying element type.\n\n
Integer multiplication wraps on overflow; floating-point follows IEEE-754 rules.";
// only floats
_DIV_ASSIGN_SIMD = "
Divides each lane of `self` by the corresponding lane of `rhs` using SIMD.\n\n
Applies elementwise division following IEEE-754 semantics.
Not available for integer types.";
// _REM_ASSIGN_SIMD = "Applies elementwise remainder using SIMD.\n\n
// Uses the remainder semantics of the underlying element type.
// Availability depends on SIMD backend support.";
_NEG_ASSIGN_SIMD = "
Negates each lane in `self` using SIMD.\n\n
Available for signed integer and floating-point element types.";
_ADD_SCALAR_ASSIGN_SIMD = "
Adds the scalar `rhs` to every lane in `self` using SIMD.\n\n
Integer addition wraps on overflow; floating-point follows IEEE-754 rules.";
_SUB_SCALAR_ASSIGN_SIMD = "
Subtracts the scalar `rhs` from every lane in `self` using SIMD.\n\n
Integer subtraction wraps on overflow; floating-point follows IEEE-754 rules.";
_MUL_SCALAR_ASSIGN_SIMD = "Multiplies every lane in `self` by the scalar `rhs` using SIMD.\n\n
Integer multiplication wraps on overflow; floating-point follows IEEE-754 rules.";
// only floats
_DIV_SCALAR_ASSIGN_SIMD = "
Divides every lane in `self` by the scalar `rhs` using SIMD.\n\n
Follows IEEE-754 division semantics.";

/* only ints */
_BITAND_ASSIGN_SIMD = "
Applies a bitwise AND between each lane of `self` and `rhs` using SIMD.";
_BITOR_ASSIGN_SIMD = "
Applies a bitwise OR between each lane of `self` and `rhs` using SIMD.";
_BITXOR_ASSIGN_SIMD = "
Applies a bitwise XOR between each lane of `self` and `rhs` using SIMD.";
_SHL_ASSIGN_SIMD = "
Shifts each lane in `self` left by the scalar amount `rhs` using SIMD.\n\n
Uses the shift semantics of the underlying integer type.";
_SHR_ASSIGN_SIMD = "
Shifts each lane in `self` right by the scalar amount `rhs` using SIMD.\n\n
Applies arithmetic right shift for signed integers and logical right shift for unsigned integers.";

/* other */
_SUM_SIMD = "
Returns the sum of all lanes using SIMD acceleration.\n\n
Follows the addition semantics of the underlying element type.
Integer sums wrap on overflow; floating-point follows IEEE-754 rules.";
_MIN_SIMD = "
Returns the minimum value across all lanes using SIMD acceleration.\n\n
Follows the comparison semantics of the underlying element type.";
_MAX_SIMD = "Returns the maximum value across all lanes using SIMD acceleration.\n\n
Follows the comparison semantics of the underlying element type.";
_CLAMP_ASSIGN_SIMD = "
Clamps each lane of `self` into the inclusive range `[lo, hi]` using SIMD.\n\n
Follows the comparison semantics of the underlying element type.";
}
