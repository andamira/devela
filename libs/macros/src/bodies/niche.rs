// devela_macros::bodies::niche
//
//! Bodies related to niche optimizations.
//
// TOC
// - enumint

use core::fmt::Write;
use proc_macro::{Ident, Span, TokenStream};

#[inline(always)]
#[cfg(feature = "alloc")]
pub(crate) fn body_enumint(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let parts: Vec<&str> = input_str.split(',').collect();
    if parts.len() != 3 {
        panic!("Expected format: enum_name, start, end");
    }

    let enum_name_str = parts[0].trim();
    let enum_name = Ident::new(enum_name_str, Span::call_site()); // will panic if invalid
    let start: u64 = parts[1].trim().parse().expect("Invalid start value");
    let end: u64 = parts[2].trim().parse().expect("Invalid end value");
    if start > end {
        panic!("Start value must be less than or equal to end value");
    }

    let range_length = end - start + 1;
    let repr = if range_length <= u8::MAX as u64 {
        "u8"
    } else if range_length <= u16::MAX as u64 {
        "u16"
    } else if range_length <= u32::MAX as u64 {
        "u32"
    } else {
        "u64"
    };

    let mut enum_variants = String::new();
    for i in start..=end {
        write!(enum_variants, "_{} = {},", i, i).unwrap();
    }

    let enum_definition = format!(
        "/// An auto-generated enum for values between {start} and {end}.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr({repr})]
        enum {enum_name} {{ {enum_variants} }}

        unsafe impl Send for {enum_name} {{}}
        unsafe impl Sync for {enum_name} {{}}

        impl {enum_name} {{
            /* constants */

            /// Returns the number of valid values.
            pub const VALID_VALUES: {repr} = {range_length};

            /// Returns the number of invalid values.
            pub const NICHE_VALUES: {repr} = {repr}::MAX - Self::VALID_VALUES + 1;

            /// Returns the minimum possible value.
            pub const MIN: {repr} = {start};

            /// Returns the maximum possible value.
            pub const MAX: {repr} = {end};

            /* methods */

            /// Returns the appropriate variant from the given `value`.
            ///
            /// Returns `None` if it's out of range.
            #[inline] #[must_use]
            pub const fn new(value: {repr}) -> Option<Self> {{
                if value >= {start} && value <= {end} {{
                    // SAFETY: The check ensures that `value` is within the valid range,
                    // so the `transmute` will always produce a valid enum variant.
                    Some(unsafe {{ core::mem::transmute(value) }})
                }} else {{
                    None
                }}
            }}

            /// Returns the appropriate variant if the given `value` is within bounds.
            ///
            /// # Panics
            /// Panics in debug if `value < {start} || value > {end}`.
            /// # Safety
            /// The given `value` must always be `value >= {start} && value <= {end}`.
            #[must_use]
            pub const unsafe fn new_unchecked(value: {repr}) -> Self {{
                debug_assert!(value >= {start} && value <= {end}, \"Value out of range\");
                // SAFETY: caller must ensure safety
                unsafe {{ core::mem::transmute(value) }}
            }}

            /// Returns the appropriate variant from the given `value`,
            /// saturating at the type bounds.
            #[inline] #[must_use]
            pub const fn new_saturated(value: {repr}) -> Self {{
                // SAFETY: The `clamp` function ensures that the value is within the valid range,
                // so the `transmute` will always produce a valid enum variant.
                unsafe {{ core::mem::transmute(Self::clamp(value, {start}, {end})) }}
            }}

            /// Returns the appropriate variant from the given `value`,
            /// wrapping around within the type bounds.
            #[inline] #[must_use]
            pub const fn new_wrapped(value: {repr}) -> Self {{
                let range_size = {end} - {start} + 1;
                let wrapped_value = if value >= {start} {{
                    (value - {start}) % range_size + {start}  // Upward wrapping
                }} else {{
                    let diff = {start} - value;
                    {end} - ((diff - 1) % range_size)  // Downward wrapping
                }};
                // SAFETY: The `wrapped_value` is guaranteed to be within the valid range,
                // so `transmute` will always produce a valid enum variant.
                unsafe {{ core::mem::transmute(wrapped_value) }}
            }}

            /// Cast the enum to its underlying representation.
            #[inline] #[must_use]
            pub const fn get(self) -> {repr} {{
                self as {repr}
            }}

            /* helpers */

            // #[inline]
            // const fn min(a: {repr}) -> {repr} {{ if a < b {{ a }} else {{ b }} }}
            // #[inline]
            // const fn max(a: {repr}) -> {repr} {{ if a > b {{ a }} else {{ b }} }}
            #[inline]
            const fn clamp(v: {repr}, min: {repr}, max: {repr}) -> {repr} {{
                if v < min {{ min }} else if v > max {{ max }} else {{ v }}
            }}
        }}"
    );
    enum_definition.parse().unwrap()
}
