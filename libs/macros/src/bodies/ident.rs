// devela_macros::bodies::ident
//
//! Bodies related to identifiers.
//
// TOC
// - coalesce
// - ident_total
// - ident_total_unique
// - ident_unique

use super::shared::split_args;
use alloc::{format, string::ToString};
use proc_macro::{TokenStream, TokenTree};

pub(crate) fn body_coalesce(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let args = split_args(&input);

    let first_non_empty_arg = args.into_iter().find(|arg| !arg.is_empty()).unwrap_or_default();

    first_non_empty_arg.parse().expect("Failed to parse TokenStream")
}

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

#[cfg(any(feature = "dep_hashbrown", feature = "std"))]
pub(crate) fn body_ident_total_unique(input: TokenStream) -> TokenStream {
    let mut unique = crate::HashSet::new();
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

#[cfg(any(feature = "dep_hashbrown", feature = "std"))]
pub(crate) fn body_ident_unique(input: TokenStream) -> TokenStream {
    let mut unique = crate::HashSet::new();
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

pub(crate) fn body_field_of(input: TokenStream) -> TokenStream {
    let input = input.to_string();

    let mut parts = input.split(',').map(|s| s.trim());
    let parts = [parts.next().unwrap_or(""), parts.next().unwrap_or("")];
    if parts[0].is_empty() || parts[1].is_empty() {
        panic!("Expected format: field_of!(value, field)");
    }
    let (value, field) = (parts[0], parts[1]);
    format!("{}.{}", value, field).parse().unwrap()
}
