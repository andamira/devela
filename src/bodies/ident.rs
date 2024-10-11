// devela_macros::bodies::ident
//
//! Bodies related to identifiers.
//
// TOC
// - coalesce
// - ident_total
// - ident_total_unique
// - ident_unique

use super::common::split_args;
use proc_macro::{TokenStream, TokenTree};

#[inline(always)]
#[cfg(feature = "alloc")]
pub(crate) fn body_coalesce(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let args = split_args(&input);

    let first_non_empty_arg = args.into_iter().find(|arg| !arg.is_empty()).unwrap_or_default();
    // .unwrap_or_else(|| "".to_string());
    // .expect("No non-empty arguments found");

    first_non_empty_arg.parse().expect("Failed to parse TokenStream")
}

#[inline(always)]
#[cfg(feature = "alloc")]
pub(crate) fn body_ident_total(input: TokenStream) -> TokenStream {
    let mut count = 0;
    for token in input {
        #[allow(clippy::single_match)]
        match token {
            TokenTree::Ident(_) => count += 1,
            _ => {}
        }
    }
    let result = format!("{}", count);
    result.parse().unwrap()
}

#[inline(always)]
#[cfg(feature = "alloc")]
pub(crate) fn body_ident_total_unique(input: TokenStream) -> TokenStream {
    let mut unique = std::collections::HashSet::new();
    let mut total = 0;
    for token in input {
        #[allow(clippy::single_match)]
        match token {
            TokenTree::Ident(i) => {
                total += 1;
                unique.insert(i.to_string());
            }
            _ => {}
        }
    }
    let result = format!("[{}, {}]", total, unique.len());
    result.parse().unwrap()
}

#[inline(always)]
#[cfg(feature = "alloc")]
pub(crate) fn body_ident_unique(input: TokenStream) -> TokenStream {
    let mut unique = std::collections::HashSet::new();
    for token in input {
        #[allow(clippy::single_match)]
        match token {
            TokenTree::Ident(i) => {
                unique.insert(i.to_string());
            }
            _ => {}
        }
    }
    let result = format!("{}", unique.len());
    result.parse().unwrap()
}
