// devela::text::ascii::digits::_docs

use crate::CONST;

CONST! { pub(super),
_DOC_DIGIT_AT_POWER_10 =
"Extracts the ASCII byte for a specific digit position using a power-of-10 divisor.\n\n
# Arguments\n\n
* `divisor` - A power of 10 that selects the digit position (1, 10, 100, …)";
_DOC_DIGIT_AT_POWER_16 =
"Extracts the ASCII byte for a specific digit position using a power-of-16 divisor.\n\n
# Arguments\n\n
* `divisor` - A power of 16 that selects the digit position (1, F, FF, …)";

_DOC_COUNT_DIGITS_10 = "Counts the number of decimal digits in the number.\n\n
Returns 1 for zero, and the base-10 logarithm plus one for other numbers.\n\n";
_DOC_COUNT_DIGITS_16 = "Counts the number of hexadecimal digits in the number.\n\n
Returns 1 for zero, and the base-16 logarithm plus one for other numbers.\n\n";

_DOC_DIGITS_STR = "Returns a static string with zero-padded digits with minimum `width`.\n\n
The given `width` will be clamped betweeen the actual number of digits
and the maximum number of digits.
# Features
- Makes use of the `unsafe_str` feature if enabled.\n\n";
}

#[rustfmt::skip] macro_rules! _DOC_WRITE_DIGITS_10 { ($MAX:literal) => { concat![
"Writes 1..=", $MAX, " decimal digits without leading zeros starting at `offset`,
returning the number of bytes written.\n\n",
"Writes `0` when the value is zero.\n\n",
"This method checks the exact number of bytes required before writing.
It can write into buffers with fewer than ", $MAX, " bytes remaining when
the value itself needs fewer digits. Returns 0 and writes nothing if
insufficient space remains.\n\n"
]}; }
#[rustfmt::skip] macro_rules! _DOC_WRITE_DIGITS_10_OMIT0 { ($MAX:literal) => { concat![
"Writes 0..=", $MAX, " decimal digits without leading zeros starting at `offset`,
returning the number of bytes written.\n\n",
"Returns 0 and writes nothing when the value is zero.\n\n",
"This method checks the exact number of bytes required before writing.
It can write into buffers with fewer than ", $MAX, " bytes remaining when
the value itself needs fewer digits. Returns 0 and writes nothing if
insufficient space remains.\n\n"
]}; }
#[rustfmt::skip] macro_rules! _DOC_WRITE_DIGITS_10_FAST { ($MAX:literal) => { concat![
"Writes 1..=", $MAX, " decimal digits without leading zeros starting at `offset`,
returning the number of bytes written.\n\n",
"Writes `0` when the value is zero.\n\n",
"This method assumes a maximum-width workspace: ", $MAX, " bytes must be
available starting at `offset`. It avoids the exact pre-counting step used by
[`write_digits10`][Self::write_digits10]. Returns 0 and writes nothing if
that workspace is not available.\n\n"
]}; }
#[rustfmt::skip] macro_rules! _DOC_WRITE_DIGITS_10_FAST_OMIT0 { ($MAX:literal) => { concat![
"Writes 0..=", $MAX, " decimal digits without leading zeros starting at `offset`,
returning the number of bytes written.\n\n",
"Returns 0 and writes nothing when the value is zero.\n\n",
"This method assumes a maximum-width workspace: ", $MAX, " bytes must be
available starting at `offset`. It avoids the exact pre-counting step used by
[`write_digits10`][Self::write_digits10]. Returns 0 and writes nothing if
that workspace is not available.\n\n"
]}; }
pub(super) use {
    _DOC_WRITE_DIGITS_10, _DOC_WRITE_DIGITS_10_FAST, _DOC_WRITE_DIGITS_10_FAST_OMIT0,
    _DOC_WRITE_DIGITS_10_OMIT0,
};
