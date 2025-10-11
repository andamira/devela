// devela_base_core::text::char::ascii::digits::docs

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
Returns 1 for zero, and the base-10 logarithm plus one for other numbers.\n\n
For more advanced needs check the [`Int`] *base* methods.\n\n";
DOC_COUNT_DIGITS_16 = "Counts the number of hexadecimal digits in the number.\n\n
Returns 1 for zero, and the base-16 logarithm plus one for other numbers.\n\n
For more advanced needs check the [`Int`] *base* methods.\n\n";

DOC_DIGITS_STR = "Returns a static string with zero-padded digits with minimum `width`.\n\n
The given `width` will be clamped betweeen the actual number of digits
and the maximum number of digits.
# Features
- Makes use of the `unsafe_str` feature if enabled.\n\n";
}
