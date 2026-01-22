// devela_base_core::num::bit::_docs
//
//! Defines constants for shared documentation on
//! [`BitOps`][crate::BitOps] and [`Bitwise`][crate::Bitwise].
//

crate::CONST! { pub(super),

/* mask */

_DOC_BIT_MASK_RANGE = r#"
Returns a bitmask of ones from the `[start..=end]` range.

Sets the rest of the bits to 0.
# Panics
Panics in debug mode if `start >= BITS` || `end >= BITS` || `start > end`.
"#;
_DOC_BIT_MASK_RANGE_CHECKED = r#"
Returns a bitmask of ones from the `[start..=end]` checked range.

Sets the rest of the bits to 0.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS` || `end >= BITS`
and [`MismatchedIndices`] if `start > end`.
"#;
_DOC_BIT_IS_SET_MASK = r#"
Returns `true` if any bit selected by `mask` is 1 in `self`.

Bits not included in `mask` are ignored.
"#;
_DOC_BIT_SET_MASK = r#"
Sets all bits selected by `mask` to 1.

Bits not included in `mask` are left unchanged.
"#;
_DOC_BIT_IS_UNSET_MASK = r#"
Returns `true` if all bits selected by `mask` are 0 in `self`.

Bits not included in `mask` are ignored.
"#;
_DOC_BIT_UNSET_MASK = r#"
Sets all bits selected by `mask` to 0.

Bits not included in `mask` are left unchanged.
"#;

/* get */

_DOC_BIT_GET_RANGE = r#"
Gets the bits in `self` from the `[start..=end]` range.

Sets the rest of the bits to 0.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_GET_CHECKED_RANGE = r#"
Gets the bits in `self` from the `[start..=end]` checked range.

Sets the rest of the bits to 0.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
[`MismatchedIndices`] if `start > end`.
"#;

/* get value */

_DOC_BIT_GET_VALUE_RANGE = r#"
Gets the value of the bits in `self` from the `[start..=end]` range.

Sets the rest of the bits to 0.

The bits in the specified range are shifted rightwards so that the least
significant bit (LSB) aligns with the units place, forming the integer value.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_GET_VALUE_CHECKED_RANGE = r#"
Gets the value of the bits in `self` from the `[start..=end]` checked range.

Sets the rest of the bits to 0.

The bits in the specified range are shifted rightwards so that the least
significant bit (LSB) aligns with the units place, forming the integer value.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
[`MismatchedIndices`] if `start > end`.
"#;

/* set */

_DOC_BIT_IS_SET = r#"
Returns true if the `nth` bit is 1.
# Panics
Panics in debug mode if `nth >= BITS`.
"#;
_DOC_BIT_IS_SET_CHECKED = r#"
Returns true if the `nth` bit is 1.
# Errors
Returns [`IndexOutOfBounds`] if `nth >= BITS`.
"#;
_DOC_BIT_SET = r#"
Sets the nth bit.
# Panics
Panics in debug mode if `nth >= BITS
"#;
_DOC_BIT_SET_CHECKED = r#"
Sets the `nth` bit.
# Errors
Returns [`IndexOutOfBounds`] if `nth >= BITS`.
"#;
_DOC_BIT_IS_SET_RANGE = r#"
Returns `true` if all bits in the `[start..=end]` range are 1.

Bits outside the range are ignored.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_IS_SET_CHECKED_RANGE = r#"
Returns `true` if all bits in the checked `[start..=end]` range are 1.

Bits outside the range are ignored.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS`
and [`MismatchedIndices`] if `start > end`.
"#;
_DOC_BIT_SET_RANGE = r#"
Sets the bits in `self` to 1, from the `[start..=end]` range.

Leaves the rest of the bits unchanged.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_SET_CHECKED_RANGE = r#"
Sets the bits in `self` to 1, from the `[start..=end]` checked range.

Leaves the rest of the bits unchanged.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS`
and [`MismatchedIndices`] if `start > end`.
"#;
_DOC_BIT_SET_ALL = r#"Sets all the bits to 1."#;

/* set value */

_DOC_BIT_SET_VALUE_RANGE = r#"
Sets the given `value` into the bits from the `[start..=end]` range.

Leaves the rest of the bits unchanged.

The value is first masked to fit the size of the range, and then
it is inserted into the specified bit range of `self`, replacing
the existing bits in that range. The rest of the bits in `self` remain unchanged.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_SET_VALUE_CHECKED_RANGE = r#"
Sets the given `value` into the bits from the `[start..=end]` checked range.

Leaves the rest of the bits unchanged.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS`
and [`MismatchedIndices`] if `start > end`.
"#;
_DOC_BIT_SET_CHECKED_VALUE_CHECKED_RANGE = r#"
Sets the given checked `value` into the bits from the `[start..=end]` checked range.

Leaves the rest of the bits unchanged.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS`,
[`MismatchedIndices`] if `start > end`, and
[`MismatchedCapacity`] if `value` does not fit within the specified bit range.
"#;

/* unset */

_DOC_BIT_IS_UNSET = r#"
Returns true if the nth bit is 0.
# Panics
Panics in debug mode if `nth >= BITS
"#;
_DOC_BIT_IS_UNSET_CHECKED = r#"
Returns true if the `nth` bit is unset to 0.
# Errors
Returns [`IndexOutOfBounds`] if `nth >= BITS`.
"#;
_DOC_BIT_UNSET = r#"
Unsets the nth bit.
# Panics
Panics in debug mode if `nth >= BITS
"#;
_DOC_BIT_UNSET_CHECKED = r#"
Unsets the `nth` bit.
# Errors
Returns [`IndexOutOfBounds`] if `nth >= BITS`.
"#;
_DOC_BIT_IS_UNSET_RANGE = r#"
Returns `true` if all bits in the `[start..=end]` range are 0.

Bits outside the range are ignored.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_IS_UNSET_CHECKED_RANGE = r#"
Returns `true` if all bits in the checked `[start..=end]` range are 0.

Bits outside the range are ignored.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS`
and [`MismatchedIndices`] if `start > end`.
"#;
_DOC_BIT_UNSET_RANGE = r#"
Unsets the bits in `self` to 0, from the `[start..=end]` range.

Leaves the rest of the bits unchanged.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_UNSET_CHECKED_RANGE = r#"
Unsets the bits in `self` to 0, from the `[start..=end]` checked range.

Leaves the rest of the bits unchanged.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
[`MismatchedIndices`] if `start > end`.
"#;
_DOC_BIT_UNSET_ALL = r#"Unsets all the bits to 0."#;

/* flip */

_DOC_BIT_FLIP = r#"Flips the nth bit.
# Panics
Panics in debug mode if `nth >= BITS.
"#;
_DOC_BIT_FLIP_CHECKED = r#"Flips the nth bit.
# Errors
Returns [`IndexOutOfBounds`] if `nth >= BITS`.
"#;
_DOC_BIT_FLIP_RANGE = r#"
Flips the bits in `self` from the `[start..=end]` range.

Leaves the rest of the bits unchanged.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_FLIP_CHECKED_RANGE = r#"
Flips the bits in `self` from the `[start..=end]` checked range.

Leaves the rest of the bits unchanged.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
[`MismatchedIndices`] if `start > end`.
"#;
_DOC_BIT_FLIP_RANGE_IF = r#"
Conditionally flips bits in a range if `cond` is true.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_FLIP_CHECKED_RANGE_IF = r#"
Conditionally flips bits in a range if `cond` is true.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
[`MismatchedIndices`] if `start > end`.
"#;

/* reverse */

_DOC_BIT_REVERSE_RANGE = r#"
Reverses the order of the bits in `self` from the `[start..=end]` range.

Leaves the rest of the bits unchanged.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_REVERSE_CHECKED_RANGE = r#"
Reverses the order of the bits in `self` from the `[start..=end]` checked range.

Leaves the rest of the bits unchanged.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
[`MismatchedIndices`] if `start > end`.
"#;

/* count */

_DOC_BIT_COUNT_ONES_RANGE = r#"
Counts the number of 1s in `self` from the `[start..=end]` range.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_COUNT_ONES_CHECKED_RANGE = r#"
Counts the number of 1s in `self` from the `[start..=end]` checked range.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
[`MismatchedIndices`] if `start > end`.
"#;
_DOC_BIT_COUNT_ZEROS_RANGE = r#"
Counts the number of 0s in `self` from the `[start..=end]` range.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_COUNT_ZEROS_CHECKED_RANGE = r#"
Counts the number of 0s in `self` from the `[start..=end]` checked range.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
[`MismatchedIndices`] if `start > end`.
"#;

/* find first */

_DOC_BIT_FIND_FIRST_ONE_RANGE = r#"
Finds the index of the first 1 in `self` from the `[start..=end]` range.

Returns `None` if there are no bits set.

The index is relative to the entire sequence of `self`, not to the given `start`.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_FIND_FIRST_ONE_CHECKED_RANGE = r#"
Finds the index of the first 1 in `self` from the `[start..=end]` checked range.

Returns `None` if there are no bits set.

The index is relative to the entire sequence of `self`, not to the given `start`.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
[`MismatchedIndices`] if `start > end`.
"#;
_DOC_BIT_FIND_FIRST_ZERO_RANGE = r#"
Finds the index of the first 0 in `self` from the `[start..=end]` range.

Returns `None` if there are no bits unset.

The index is relative to the entire sequence of `self`, not to the given `start`.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_FIND_FIRST_ZERO_CHECKED_RANGE = r#"
Finds the index of the first 0 in `self` from the `[start..=end]` checked range.

Returns `None` if there are no bits unset.

The index is relative to the entire sequence of `self`, not to the given `start`.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
[`MismatchedIndices`] if `start > end`.
"#;

/* find last */

_DOC_BIT_FIND_LAST_ONE_RANGE = r#"
Finds the index of the last 1 in `self` from the `[start..=end]` range.

Returns `None` if there are no bits set.

The index is relative to the entire sequence of `self`, not to the given `start`.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_FIND_LAST_ONE_CHECKED_RANGE = r#"
Finds the index of the last 1 in `self` from the `[start..=end]` checked range.

Returns `None` if there are no bits set.

The index is relative to the entire sequence of `self`, not to the given `start`.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
[`MismatchedIndices`] if `start > end`.
"#;
_DOC_BIT_FIND_LAST_ZERO_RANGE = r#"
Finds the index of the last 0 in `self` from the `[start..=end]` range.

Returns `None` if there are no bits set.

The index is relative to the entire sequence of `self`, not to the given `start`.
# Panics
Panics in debug mode if `start >= BITS || end >= BITS || start > end`.
"#;
_DOC_BIT_FIND_LAST_ZERO_CHECKED_RANGE = r#"
Finds the index of the last 0 in `self` from the `[start..=end]` checked range.

Returns `None` if there are no bits set.

The index is relative to the entire sequence of `self`, not to the given `start`.
# Errors
Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
[`MismatchedIndices`] if `start > end`.
"#;
}
