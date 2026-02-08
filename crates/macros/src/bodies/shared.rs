// devela_macros::bodies::shared
//
//! Shared functionality for procedural macros.
//
// TOC
// - split_compile_doc_tuple
// - deindent
// - compile_eval
// - parse_vis_ident

use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use std::iter::{Peekable, once};

/// Expects a specific punctuation.
pub(crate) fn expect_punct(iter: &mut Peekable<impl Iterator<Item = TokenTree>>, expected: char) {
    if let Some(TokenTree::Punct(punct)) = iter.next() {
        if punct.as_char() != expected {
            panic!("Expected '{}', found '{}'", expected, punct.as_char());
        }
    } else {
        panic!("Expected '{}'", expected);
    }
}

/// Parses an integer from the token stream.
pub(crate) fn parse_int(iter: &mut Peekable<impl Iterator<Item = TokenTree>>) -> i128 {
    let mut is_negative = false;
    if let Some(TokenTree::Punct(punct)) = iter.peek() {
        if punct.as_char() == '-' {
            is_negative = true;
            iter.next(); // Consume '-'
        }
    }

    let value = match iter.next() {
        Some(TokenTree::Literal(lit)) => {
            let s = lit.to_string();
            // Remove any underscores from the literal
            let s = s.replace('_', "");
            s.parse::<i128>().expect("Invalid integer literal")
        }
        other => panic!("Expected integer literal, found {:?}", other),
    };

    if is_negative { -value } else { value }
}

/// Parses visibility.
pub(crate) fn parse_visibility(
    iter: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Option<TokenStream2> {
    if let Some(TokenTree::Ident(ident)) = iter.peek() {
        if *ident == "pub" {
            let mut vis_stream = TokenStream2::new();
            let Some(TokenTree::Ident(ident)) = iter.next() else { unreachable!() };
            vis_stream.extend(once(TokenTree::Ident(ident.clone())));

            // Check for optional group (e.g., (crate), (super), (self), (in path))
            if let Some(TokenTree::Group(group)) = iter.peek() {
                if group.delimiter() == proc_macro2::Delimiter::Parenthesis {
                    let Some(TokenTree::Group(group)) = iter.next() else { unreachable!() };
                    vis_stream.extend(once(TokenTree::Group(group)));
                }
            }
            Some(vis_stream)
        } else {
            None
        }
    } else {
        None
    }
}
