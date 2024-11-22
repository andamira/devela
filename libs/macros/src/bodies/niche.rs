// devela_macros::bodies::niche
//
//! Bodies related to niche optimizations.
//
// TOC
// - enumint

use super::shared::parse_vis_ident;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use alloc::{format, vec::Vec, string::ToString};

#[inline(always)]
#[cfg(feature = "alloc")]
pub(crate) fn body_enumint(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let parts: Vec<&str> = input_str.split(',').collect();
    if parts.len() != 4 {
        panic!("Expected format: [visibility] enum_name, repr, start, end");
    }

    let enum_name_str = parts[0].trim();
    let (visibility, enum_name) = parse_vis_ident(enum_name_str);
    let repr_str = parts[1].trim();

    let start: i128 = parts[2].trim().parse().expect("Invalid start value");
    let end: i128 = parts[3].trim().parse().expect("Invalid end value");
    if start > end {
        panic!("Start value must be less than or equal to end value");
    }

    // Validate the provided representation against the range length
    let range_length = end - start + 1;
    let repr = match repr_str {
        // unsigned reprs
        "u8" => {
            if range_length > u8::MAX as i128 {
                panic!("u8 cannot represent the range [{start}, {end}]")
            }
            quote! { u8 }
        }
        "u16" => {
            if range_length > u16::MAX as i128 {
                panic!("u16 cannot represent the range [{start}, {end}]")
            }
            quote! { u16 }
        }
        // signed reprs
        "i8" => {
            if start < i8::MIN as i128 || end > i8::MAX as i128 {
                panic!("i8 cannot represent the range [{start}, {end}]")
            }
            quote! { i8 }
        }
        "i16" => {
            if start < i16::MIN as i128 || end > i16::MAX as i128 {
                panic!("i16 cannot represent the range [{start}, {end}]")
            }
            quote! { i16 }
        }
        _ => panic!("Invalid representation type: {}", repr_str),
    };
    let unsigned_repr = match repr_str {
        "i8" => quote! { u8 },
        "i16" => quote! { u16 },
        _ => repr.clone(), // For unsigned types, use the same repr
    };

    // Generate the enum variants, handling negative and positive values.
    let mut enum_variants = Vec::new();
    for i in start..=end {
        let variant = if i < 0 {
            Ident::new(&format!("N{}", i.abs()), Span::call_site())
        } else {
            Ident::new(&format!("P{}", i), Span::call_site())
        };
        enum_variants.push(quote! { #variant = #i as #repr });
    }

    // Generate the final output
    let enum_definition = quote! {
        /// An auto-generated enum for values between #start and #end.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(#repr)]
        #visibility enum #enum_name {
            #(
                /// .
                #enum_variants
            ),*
        }

        unsafe impl Send for #enum_name {}
        unsafe impl Sync for #enum_name {}

        impl #enum_name {
            /* constants */

            /// Returns the number of valid values, as an unsigned primitive.
            pub const VALID_VALUES: #unsigned_repr = #range_length as #unsigned_repr;

            /// Returns the number of invalid values, as an unsigned primitive.
            pub const NICHE_VALUES: #unsigned_repr = #unsigned_repr::MAX - Self::VALID_VALUES + 1;

            /// Returns the minimum possible value.
            pub const MIN: #repr = #start as #repr;

            /// Returns the maximum possible value.
            pub const MAX: #repr = #end as #repr;

            /* methods */

            /// Returns the appropriate variant from the given `value`.
            ///
            /// Returns `None` if it's out of range.
            #[inline]
            #[must_use]
            pub const fn new(value: #repr) -> Option<Self> {
                if value >= #start as #repr && value <= #end as #repr {
                    // SAFETY: The check ensures that `value` is within the valid range,
                    // so the `transmute` will always produce a valid enum variant.
                    Some(unsafe { core::mem::transmute(value) })
                } else {
                    None
                }
            }

            /// Returns the appropriate variant if the given `value` is within bounds.
            ///
            /// # Panics
            /// Panics in debug if `value < #start || value > #end`.
            /// # Safety
            /// The given `value` must always be `value >= #start && value <= #end`.
            #[must_use]
            pub const unsafe fn new_unchecked(value: #repr) -> Self {
                debug_assert!(value >= #start as #repr && value <= #end as #repr, "Value out of range");
                // SAFETY: caller must ensure safety
                unsafe { core::mem::transmute(value) }
            }

            /// Returns the appropriate variant from the given `value`,
            /// saturating at the type bounds.
            #[inline]
            #[must_use]
            pub const fn new_saturated(value: #repr) -> Self {
                // SAFETY: The `clamp` function ensures that the value is within the valid range,
                // so the `transmute` will always produce a valid enum variant.
                unsafe { core::mem::transmute(Self::clamp(value, #start as #repr, #end as #repr)) }
            }

            /// Returns the appropriate variant from the given `value`,
            /// wrapping around within the type bounds.
            #[inline]
            #[must_use]
            pub const fn new_wrapped(value: #repr) -> Self {
                let range_size = (#end - #start + 1) as #repr;
                let wrapped_value = if value >= #start as #repr {
                    (value - #start as #repr) % range_size + #start as #repr  // Upward wrapping
                } else {
                    let diff = #start as #repr - value;
                    #end as #repr - ((diff - 1) % range_size)  // Downward wrapping
                };
                // SAFETY: The `wrapped_value` is guaranteed to be within the valid range,
                // so `transmute` will always produce a valid enum variant.
                unsafe { core::mem::transmute(wrapped_value) }
            }

            /// Cast the enum to its underlying representation.
            #[inline]
            #[must_use]
            pub const fn get(self) -> #repr {
                self as #repr
            }

            /* helpers */

            #[inline]
            const fn clamp(v: #repr, min: #repr, max: #repr) -> #repr {
                if v < min { min } else if v > max { max } else { v }
            }
            // #[inline]
            // const fn max(a: {repr}) -> {repr} {{ if a > b {{ a }} else {{ b }} }}
            // #[inline]
            // const fn min(a: {repr}) -> {repr} {{ if a < b {{ a }} else {{ b }} }}
        }
    };
    enum_definition.into()
}
