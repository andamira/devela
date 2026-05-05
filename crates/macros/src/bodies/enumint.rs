// devela_macros::bodies::enumint
//
//! The body of [`enumint!`][crate::enumint].
//

use super::shared::{expect_punct, parse_int, parse_visibility};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2, TokenTree};
use quote::quote;

pub(crate) fn body_enumint(input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.into();
    let tokens: Vec<TokenTree> = input.into_iter().collect();

    // Process the tokens to extract: visibility, enum_name, repr, start, end
    // Expected format: [visibility] enum_name, repr, start, end
    let mut iter = tokens.into_iter().peekable();

    let visibility = parse_visibility(&mut iter);

    let enum_name = match iter.next() {
        Some(TokenTree::Ident(ident)) => ident,
        other => panic!("Expected enum name, found {:?}", other),
    };
    expect_punct(&mut iter, ',');

    let repr = match iter.next() {
        Some(TokenTree::Ident(ident)) => ident.to_string(),
        other => panic!("Expected representation type, found {:?}", other),
    };
    let repr_str = repr.as_str();

    expect_punct(&mut iter, ',');
    let start = parse_int(&mut iter);
    expect_punct(&mut iter, ',');
    let end = parse_int(&mut iter);

    // Ensure no more tokens
    if let Some(tok) = iter.next() {
        panic!("Unexpected token after end value: {:?}", tok);
    }

    // Validate the provided representation against the range length
    if start > end {
        panic!("enumint start must be <= end");
    }
    let (repr, repr_min, repr_max, capacity): (TokenStream2, i64, i64, i64) = match repr_str {
        "u8" => (quote!(u8), u8::MIN as i64, u8::MAX as i64, 256),
        "u16" => (quote!(u16), u16::MIN as i64, u16::MAX as i64, 65_536),
        "i8" => (quote!(i8), i8::MIN as i64, i8::MAX as i64, 256),
        "i16" => (quote!(i16), i16::MIN as i64, i16::MAX as i64, 65_536),
        _ => panic!("Invalid representation type: {}", repr_str),
    };
    if start < repr_min || end > repr_max {
        panic!("{repr_str} cannot represent the range [{start}, {end}]");
    }
    let range_length = end - start + 1;
    let visibility = visibility.unwrap_or_default();
    let enum_name = Ident::new(&enum_name.to_string(), Span::call_site());

    // Generate the enum variants, handling negative and positive values.
    let mut enum_variants = Vec::new();
    #[allow(unused_mut)]
    let (mut new_arms, mut valid_arms): (Vec<TokenStream2>, Vec<TokenStream2>) = (vec![], vec![]);

    for i in start..=end {
        let variant = if i < 0 {
            Ident::new(&format!("N{}", i.abs()), Span::call_site())
        } else {
            Ident::new(&format!("P{}", i), Span::call_site())
        };
        enum_variants.push(quote! { #variant = #i as #repr });
        #[cfg(not(feature = "unsafe"))]
        {
            let lit = int_lit(i);
            new_arms.push(quote! { #lit => Some(Self::#variant) });
            valid_arms.push(quote! { #lit => Self::#variant });
        }
    }
    #[cfg(not(feature = "unsafe"))]
    fn int_lit(n: i64) -> TokenStream2 {
        n.to_string().parse().unwrap()
    }

    // Generate the final output
    let enum_definition = quote! {
        /// An auto-generated enum for values between #start and #end.
        #[allow(missing_docs)] // reason = "undocumented variants"
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(#repr)]
        #visibility enum #enum_name {
            #(
                #enum_variants
            ),*
        }

        impl #enum_name {
            /* constants */

            /// The number of valid values, as an unsigned primitive.
            pub const VALUES: usize = #range_length as usize;
            /// The number of invalid values, as an unsigned primitive.
            pub const NICHES: usize = #capacity as usize - Self::VALUES;

            /// Returns the minimum possible value.
            pub const MIN: #repr = #start as #repr;
            /// Returns the maximum possible value.
            pub const MAX: #repr = #end as #repr;

            /* methods */

            /// Returns the appropriate variant from the given `value`.
            ///
            /// Returns `None` if it's out of range.
            #[must_use]
            pub const fn new(value: #repr) -> Option<Self> {
                cfg_select! {
                    not(feature = "unsafe") => {
                        match value { #( #new_arms, )* _ => None }
                    }
                    _ => {
                        if #start > #end { panic!("enumint start must be <= end"); }
                        if value >= #start as #repr && value <= #end as #repr {
                            // SAFETY: The check ensures that `value` is within the valid range,
                            // so the `transmute` will always produce a valid enum variant.
                            Some(unsafe { core::mem::transmute(value) })
                        } else {
                            None
                        }
                    }
                }
            }

            /// Returns the appropriate variant if the given `value` is within bounds.
            ///
            /// # Panics
            /// Panics in debug if `value < #start || value > #end`.
            /// # Safety
            /// The given `value` must always be `value >= #start && value <= #end`.
            #[must_use]
            #[cfg(feature = "unsafe")]
            pub const unsafe fn new_unchecked(value: #repr) -> Self {
                debug_assert!(value >= #start as #repr && value <= #end as #repr,
                    "Value out of range");
                // SAFETY: caller must ensure safety
                unsafe { core::mem::transmute(value) }
            }

            /// Returns the appropriate variant from the given `value`,
            /// saturating at the type bounds.
            #[must_use]
            pub const fn new_saturated(value: #repr) -> Self {
                cfg_select! {
                    not(feature = "unsafe") => {
                        Self::_new_valid(Self::clamp(value, Self::MIN, Self::MAX))
                    }
                    _ => {
                        // SAFETY: The `clamp` fn ensures that the value is within the valid range,
                        // so the `transmute` will always produce a valid enum variant.
                        unsafe {
                            core::mem::transmute(Self::clamp(value, #start as #repr, #end as #repr))
                        }
                    }
                }
            }

            /// Returns the appropriate variant from the given `value`,
            /// wrapping around within the type bounds.
            #[must_use]
            pub const fn new_wrapped(value: #repr) -> Self {
                let (start, end) = (#start as i64, #end as i64);
                let len = end - start + 1;
                let offset = (value as i64 - start).rem_euclid(len);
                let wrapped = (start + offset) as #repr;
                cfg_select! {
                    not(feature = "unsafe") => Self::_new_valid(wrapped),
                    _ => {
                        // SAFETY: The `wrapped_value` is guaranteed to be within the valid range,
                        // so `transmute` will always produce a valid enum variant.
                        unsafe { core::mem::transmute(wrapped) }
                    }
                }
            }

            /// Cast the enum to its underlying representation.
            #[must_use]
            pub const fn get(self) -> #repr {
                self as #repr
            }

            /* helpers */

            const fn _new_valid(value: #repr) -> Self {
                match value {
                    #( #valid_arms,)*
                    _ => panic!("enumint value out of range"),
                }
            }
            const fn clamp(v: #repr, min: #repr, max: #repr) -> #repr {
                if v < min { min } else if v > max { max } else { v }
            }
        }
    };
    enum_definition.into()
}
