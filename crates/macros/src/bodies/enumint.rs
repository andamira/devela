// devela_macros::bodies::enumint
//
//! The body of [`enumint!`][crate::enumint].
//

use super::shared::{
    deny_tokens, error_tokens, expect_punct, int_lit, macro_errors_are_denied, parse_int,
    parse_visibility, warn_tokens,
};
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Delimiter, Ident, Span, TokenStream as TokenStream2, TokenTree};
use quote::quote;

pub(crate) fn body_enumint(input: TokenStream1) -> TokenStream1 {
    let input: TokenStream2 = input.into();
    let tokens: Vec<TokenTree> = input.into_iter().collect();

    // Process the tokens to extract: attrs, vis, enum_name, repr, start, end
    // Expected format: (attrs)* (vis)? enum_name, repr, start, end
    let mut iter = tokens.into_iter().peekable();

    let mut attrs = Vec::<TokenStream2>::new();
    loop {
        let mut clone = iter.clone();
        match (clone.next(), clone.next()) {
            (Some(TokenTree::Punct(p)), Some(TokenTree::Group(g)))
                if p.as_char() == '#' && g.delimiter() == Delimiter::Bracket =>
            {
                // consume from real iterator
                let hash = iter.next().unwrap();
                let group = iter.next().unwrap();
                attrs.push(quote! { #hash #group });
            }
            _ => break,
        }
    }

    let vis = parse_visibility(&mut iter);

    let enum_name_raw = match iter.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(tok) => return error_tokens(tok.span(), "enumint expected enum name").into(),
        None => return error_tokens(Span::call_site(), "enumint expected enum name").into(),
    };
    expect_punct(&mut iter, ',');

    let repr_ident = match iter.next() {
        Some(TokenTree::Ident(ident)) => ident,
        Some(tok) => {
            return error_tokens(tok.span(), "enumint expected representation type").into();
        }
        None => {
            return error_tokens(enum_name_raw.span(), "enumint expected representation type")
                .into();
        }
    };
    let repr = repr_ident.to_string();
    let repr_str = repr.as_str();

    expect_punct(&mut iter, ',');
    let start = parse_int(&mut iter);
    expect_punct(&mut iter, ',');
    let end = parse_int(&mut iter);

    // Ensure no more tokens
    if let Some(tok) = iter.next() {
        return error_tokens(tok.span(), "enumint unexpected token after end value").into();
    }

    // Validate the provided representation against the range length
    if start > end {
        return error_tokens(
            enum_name_raw.span(),
            &format!("enumint range start `{start}` must be less than or equal to end `{end}`"),
        )
        .into();
    }
    let (repr, repr_min, repr_max, capacity): (TokenStream2, i64, i64, i64) = match repr_str {
        "u8" => (quote!(u8), u8::MIN as i64, u8::MAX as i64, 256),
        "i8" => (quote!(i8), i8::MIN as i64, i8::MAX as i64, 256),
        "u16" => (quote!(u16), u16::MIN as i64, u16::MAX as i64, 65_536),
        "i16" => (quote!(i16), i16::MIN as i64, i16::MAX as i64, 65_536),
        _ => {
            return error_tokens(
            repr_ident.span(),
            &format!(
                "enumint unsupported representation `{repr_str}`; expected one of: u8, i8, u16, i16"
            ),
        )
        .into();
        }
    };
    if start < repr_min || end > repr_max {
        return error_tokens(
            repr_ident.span(),
            &format!(
                "enumint range `{start}..={end}` is outside `{repr_str}` bounds `{repr_min}..={repr_max}`"
            ),
        )
        .into();
    }

    let range_length = end - start + 1;
    let vis = vis.unwrap_or_default();
    let enum_name = Ident::new(&enum_name_raw.to_string(), enum_name_raw.span());

    /* safety guards */

    let unsafe_backend = cfg!(all(not(feature = "safe"), feature = "unsafe_layout"));
    let safe_backend = cfg!(any(feature = "safe", not(feature = "unsafe_layout")));
    let safe_too_large = safe_backend && range_length > SAFE_ERROR_VALUES;
    let unsafe_too_large = unsafe_backend && range_length > UNSAFE_ERROR_VALUES;

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
    let err_safe_large = deny_tokens(
        safe_too_large,
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
             large ranges can significantly increase compile time and memory use even with the unsafe backend"
        ),
    );
    let err_unsafe_large = deny_tokens(
        unsafe_too_large,
        enum_name.span(),
        &format!(
            "enumint refuses to generate {range_length} variants;\n \
             large contiguous numeric domains should usually be represented by a checked newtype"
        ),
    );
    if macro_errors_are_denied() && (safe_too_large || unsafe_too_large) {
        return quote! {
            #err_safe_large
            #err_unsafe_large
        }
        .into();
    }

    // Generate the enum variants, handling negative and positive values.
    let (mut enum_variants, mut new_arms, mut valid_arms) = (vec![], vec![], vec![]);
    for i in start..=end {
        let variant = if i < 0 {
            Ident::new(&format!("N{}", i.abs()), Span::call_site())
        } else {
            Ident::new(&format!("P{}", i), Span::call_site())
        };
        let lit = int_lit(i);
        enum_variants.push(quote! { #variant = #lit });
        if safe_backend {
            new_arms.push(quote! { #lit => Some(Self::#variant) });
            valid_arms.push(quote! { #lit => Self::#variant });
        }
    }

    /* generate constructors' bodies */

    let new_body = if safe_backend {
        quote! {
            match value { #( #new_arms, )* _ => None, }
        }
    } else {
        quote! {
            if value >= Self::MIN && value <= Self::MAX {
                // SAFETY: The range check ensures `value` is a valid discriminant.
                Some(unsafe { core::mem::transmute(value) })
            } else {
                None
            }
        }
    };
    let saturated_body = if safe_backend {
        quote! { Self::_new_valid(Self::clamp(value, Self::MIN, Self::MAX)) }
    } else {
        quote! {
            // SAFETY: `clamp` returns a value inside the represented range.
            unsafe { core::mem::transmute(Self::clamp(value, Self::MIN, Self::MAX)) }
        }
    };
    let wrapped_body = if safe_backend {
        quote! {
            Self::_new_valid(wrapped)
        }
    } else {
        quote! {
            // SAFETY: `wrapped` is computed inside the represented range.
            unsafe { core::mem::transmute(wrapped) }
        }
    };
    let new_valid_helper = if safe_backend {
        quote! {
            const fn _new_valid(value: #repr) -> Self {
                match value {
                    #( #valid_arms, )*
                    _ => panic!("enumint value out of range"),
                }
            }
        }
    } else {
        TokenStream2::new()
    };

    // Generate the final output
    let enum_definition = quote! {
        #warn_safe_large
        #err_safe_large
        #warn_unsafe_large
        #err_unsafe_large

        #( #attrs )*
        #[allow(missing_docs)] // reason = "undocumented variants"
        #[repr(#repr)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #vis enum #enum_name { #( #enum_variants ),* }

        impl #enum_name {
            /* constants */

            /// The number of represented values.
            pub const VALUES: usize = #range_length as usize;
            /// The number of unused representation values available as niches.
            pub const NICHES: usize = #capacity as usize - Self::VALUES;

            /// The lowest represented integer.
            pub const MIN: #repr = #start as #repr;
            /// The highest represented integer.
            pub const MAX: #repr = #end as #repr;

            /* constructors */

            /// Returns the variant for `value`, or `None` if it is outside the range.
            #[must_use]
            pub const fn new(value: #repr) -> Option<Self> { #new_body }

            /// Returns the variant for `value`, clamped to the represented range.
            #[must_use]
            pub const fn new_saturated(value: #repr) -> Self { #saturated_body }

            /// Returns the variant for `value`, wrapped inside the represented range.
            #[must_use]
            pub const fn new_wrapped(value: #repr) -> Self {
                let (start, end) = (#start as i64, #end as i64);
                let len = end - start + 1;
                let offset = (value as i64 - start).rem_euclid(len);
                let wrapped = (start + offset) as #repr;
                #wrapped_body
            }

            /* getters */

            /// Returns the underlying integer representation.
            #[must_use]
            pub const fn get(self) -> #repr { self as #repr }

            /* helpers */

            #new_valid_helper

            const fn clamp(v: #repr, min: #repr, max: #repr) -> #repr {
                if v < min { min } else if v > max { max } else { v }
            }
        }
    };
    enum_definition.into()
}
