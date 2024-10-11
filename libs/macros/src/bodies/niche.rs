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

            /// Try to create an enum from the underlying representation.
            #[inline]
            pub const fn new(value: {repr}) -> Option<Self> {{
                if value >= {start} && value <= {end} {{
                    Some(unsafe {{ core::mem::transmute(value) }})
                }} else {{
                    None
                }}
            }}

            /// Cast the enum to its underlying representation.
            #[inline]
            #[must_use]
            pub const fn get(self) -> {repr} {{
                self as {repr}
            }}

            /* arithmetic */

            /// Checked addition. Returns `None` if overflow occurs.
            #[inline] #[must_use]
            pub const fn checked_add(self, other: Self) -> Option<Self> {{
                let result = self.get().checked_add(other.get());
                if let Some(result) = result {{
                    Self::new(result)
                }} else {{
                    None
                }}
            }}

            /// Saturating addition. Returns the maximum value if overflow occurs.
            #[inline] #[must_use]
            pub const fn saturating_add(self, other: Self) -> Self {{
                let result = self.get().saturating_add(other.get());
                match Self::new(result) {{
                    Some(value) => value,
                    None => match Self::new(Self::_{end}.get()) {{
                        Some(max) => max,
                        None => Self::_{end}, // Fallback to end (shouldn't happen)
                    }},
                }}
            }}

            /// Wrapping (modular) addition. Wraps around on overflow.
            #[inline] #[must_use]
            pub const fn wrapping_add(self, other: Self) -> Self {{
                let range_size = Self::_{end}.get() - Self::_{start}.get() + 1;
                let result = (self.get() - Self::_{start}.get() + other.get())
                                % range_size + Self::_{start}.get();
                match Self::new(result) {{
                    Some(value) => value,
                    None => Self::_{start},  // Fallback to start (shouldn't happen)
                }}
            }}
        }}"
    );
    enum_definition.parse().unwrap()
}
