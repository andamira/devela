// devela_macros::bodies::enumint
//
//! The body of [`enumint!`][crate::enumint].
//

use super::shared::{expect_punct, parse_int, parse_visibility};
use crate::{error_tokens, warn_tokens};
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
        "i8" => (quote!(i8), i8::MIN as i64, i8::MAX as i64, 256),
        "u16" => (quote!(u16), u16::MIN as i64, u16::MAX as i64, 65_536),
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
    // Appends the integer type to `n`.
    #[cfg(any(feature = "safe", not(feature = "unsafe")))]
    fn int_lit(n: i64) -> TokenStream2 {
        n.to_string().parse().unwrap()
    }

    /* safety guards */

    let unsafe_backend = cfg!(all(not(feature = "safe"), feature = "unsafe_layout"));
    let safe_backend = cfg!(any(feature = "safe", not(feature = "unsafe_layout")));

    const SAFE_WARN_VALUES: i64 = 256;
    const SAFE_ERROR_VALUES: i64 = 1024;
    const UNSAFE_WARN_VALUES: i64 = 4096;
    const UNSAFE_ERROR_VALUES: i64 = 32768;

    let warn_safe_large = warn_tokens(
        safe_backend && range_length > SAFE_WARN_VALUES && range_length <= SAFE_ERROR_VALUES,
        enum_name.span(),
        &format!(
            "enumint generated {range_length} variants using the safe backend;\n \
             this also generates one match arm per value and may noticeably increase compile time"
        ),
    );
    let err_safe_large = error_tokens(
        safe_backend && range_length > SAFE_ERROR_VALUES,
        enum_name.span(),
        &format!(
            "enumint safe backend refuses to generate {range_length} values;\n \
             reduce the range to {SAFE_ERROR_VALUES} values or fewer, \
             or enable the `unsafe_layout` feature to use the compact checked-conversion backend"
        ),
    );
    let warn_unsafe_large = warn_tokens(
        unsafe_backend && range_length > UNSAFE_WARN_VALUES && range_length <= UNSAFE_ERROR_VALUES,
        enum_name.span(),
        &format!(
            "enumint generated {range_length} variants;\n \
             large ranges can significantly increase \
             compile time and memory use even with the unsafe backend"
        ),
    );
    let err_unsafe_large = error_tokens(
        unsafe_backend && range_length > UNSAFE_ERROR_VALUES,
        enum_name.span(),
        &format!(
            "enumint refuses to generate {range_length} variants;\n \
             large contiguous numeric domains should usually be represented by a checked newtype"
        ),
    );

    let (start_doc, end_doc, repr_doc) = (start.to_string(), end.to_string(), repr_str.to_string());

    // Generate the final output
    let enum_definition = quote! {
        #warn_safe_large
        #err_safe_large
        #warn_unsafe_large
        #err_unsafe_large

        #[doc = concat!("A compact `", #repr_doc, "` enum over the contiguous integer range `",
            #start_doc, "..=", #end_doc, "`.")]
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

            /// The number of represented values.
            pub const VALUES: usize = #range_length as usize;
            /// The number of invalid representation left available as niches.
            pub const NICHES: usize = #capacity as usize - Self::VALUES;

            /// The lowest represented integer.
            pub const MIN: #repr = #start as #repr;
            /// The highest represented integer.
            pub const MAX: #repr = #end as #repr;

            /* constructors */

            /// Returns the variant for `value`, or `None` if it is outside the range.
            #[must_use]
            pub const fn new(value: #repr) -> Option<Self> {
                cfg_select! {
                    any(feature = "safe", not(feature = "unsafe_layout")) => {
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

            /// Returns the variant for `value`, clamped to the represented range.
            #[must_use]
            pub const fn new_saturated(value: #repr) -> Self {
                cfg_select! {
                    any(feature = "safe", not(feature = "unsafe_layout")) => {
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

            /// Returns the variant for `value`, wrapped inside the represented range.
            #[must_use]
            pub const fn new_wrapped(value: #repr) -> Self {
                let (start, end) = (#start as i64, #end as i64);
                let len = end - start + 1;
                let offset = (value as i64 - start).rem_euclid(len);
                let wrapped = (start + offset) as #repr;
                cfg_select! {
                    any(feature = "safe", not(feature = "unsafe_layout")) => {
                        Self::_new_valid(wrapped)
                    }
                    _ => {
                        // SAFETY: The `wrapped_value` is guaranteed to be within the valid range,
                        // so `transmute` will always produce a valid enum variant.
                        unsafe { core::mem::transmute(wrapped) }
                    }
                }
            }

            /* getters */

            /// Returns the underlying integer representation.
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
