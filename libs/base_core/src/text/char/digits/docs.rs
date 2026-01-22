// devela_base_core::text::char::digits::docs

use crate::CONST;

CONST! { pub(super),
DOC_DIGIT_AT_POWER_10 =
"Extracts the ASCII byte for a specific digit position using a power-of-10 divisor.\n\n
# Arguments\n\n
* `divisor` - A power of 10 that selects the digit position (1, 10, 100, …)";
DOC_DIGIT_AT_POWER_16 =
"Extracts the ASCII byte for a specific digit position using a power-of-16 divisor.\n\n
# Arguments\n\n
* `divisor` - A power of 16 that selects the digit position (1, F, FF, …)";

DOC_COUNT_DIGITS_10 = "Counts the number of decimal digits in the number.\n\n
Returns 1 for zero, and the base-10 logarithm plus one for other numbers.\n\n";
DOC_COUNT_DIGITS_16 = "Counts the number of hexadecimal digits in the number.\n\n
Returns 1 for zero, and the base-16 logarithm plus one for other numbers.\n\n";

DOC_DIGITS_STR = "Returns a static string with zero-padded digits with minimum `width`.\n\n
The given `width` will be clamped betweeen the actual number of digits
and the maximum number of digits.
# Features
- Makes use of the `unsafe_str` feature if enabled.\n\n";
}

#[rustfmt::skip] macro_rules! DOC_WRITE_DIGITS_10 { ($MAX:literal) => { concat![
"Writes 1..=", $MAX, " decimal digits without leading zeros starting at `offset`,
returning the number of bytes written.\n\n
This method calculates the exact digit count first, allowing it to work with
buffers smaller than ", $MAX, " bytes. Returns 0 if insufficient space remains.\n\n"
]}; }
#[rustfmt::skip] macro_rules! DOC_WRITE_DIGITS_10_FAST { ($MAX:literal) => { concat![
"Writes 1..=", $MAX, " decimal digits without leading zeros starting at `offset`,
returning the number of bytes written.\n\n
This method uses a faster algorithm that avoids digit counting but requires
the buffer to have at least ", $MAX, " bytes available.
# Panics
Panics in debug mode if fewer than ", $MAX, " bytes remain starting at `offset`."
] }; }
pub(super) use {DOC_WRITE_DIGITS_10, DOC_WRITE_DIGITS_10_FAST};
