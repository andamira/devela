// devela_macros::bodies::paste::segment
//
//! Grammar and modifier evaluation.
//

use super::{PasteError, PasteResult};
use core::iter::Peekable;
use proc_macro::{Delimiter, Group, Ident, Literal, Span, TokenTree, token_stream};

pub(crate) enum PasteSegment {
    String(LitStr),
    Apostrophe(Span),
    Env(LitStr),
    Modifier(Colon, Ident),
    Replace(Colon, Group),
}

pub(crate) struct LitStr {
    pub value: String,
    pub span: Span,
}
pub(crate) struct Colon {
    pub span: Span,
}

impl PasteSegment {
    pub(crate) fn parse(
        tokens: &mut Peekable<token_stream::IntoIter>,
    ) -> PasteResult<Vec<PasteSegment>> {
        let mut segments = Vec::new();
        while match tokens.peek() {
            None => false,
            Some(TokenTree::Punct(punct)) => punct.as_char() != '>',
            Some(_) => true,
        } {
            match tokens.next().unwrap() {
                TokenTree::Ident(ident) => {
                    let mut fragment = ident.to_string();
                    if fragment.starts_with("r#") {
                        fragment = fragment.split_off(2);
                    }
                    if fragment == "env"
                        && match tokens.peek() {
                            Some(TokenTree::Punct(punct)) => punct.as_char() == '!',
                            _ => false,
                        }
                    {
                        let bang = tokens.next().unwrap(); // `!`
                        let expect_group = tokens.next();
                        let parenthesized = match &expect_group {
                            Some(TokenTree::Group(group))
                                if group.delimiter() == Delimiter::Parenthesis =>
                            {
                                group
                            }
                            Some(wrong) => {
                                return Err(PasteError::new(wrong.span(), "expected `(`"));
                            }
                            None => {
                                return Err(PasteError::new2(
                                    ident.span(),
                                    bang.span(),
                                    "expected `(` after `env!`",
                                ));
                            }
                        };
                        let mut inner = parenthesized.stream().into_iter();
                        let lit = match inner.next() {
                            Some(TokenTree::Literal(lit)) => lit,
                            Some(wrong) => {
                                return Err(PasteError::new(
                                    wrong.span(),
                                    "expected string literal",
                                ));
                            }
                            None => {
                                return Err(PasteError::new2(
                                    ident.span(),
                                    parenthesized.span(),
                                    "expected string literal as argument to env! macro",
                                ));
                            }
                        };

                        segments.push(PasteSegment::Env(LitStr {
                            value: get_literal_string_value(&lit, false, false)?,
                            span: lit.span(),
                        }));

                        if let Some(unexpected) = inner.next() {
                            return Err(PasteError::new(
                                unexpected.span(),
                                "unexpected token in env! macro",
                            ));
                        }
                    } else {
                        segments.push(PasteSegment::String(LitStr {
                            value: fragment,
                            span: ident.span(),
                        }));
                    }
                }
                TokenTree::Literal(lit) => {
                    segments.push(PasteSegment::String(LitStr {
                        value: lit.to_string(),
                        span: lit.span(),
                    }));
                }
                // Note: `_` is always tokenized as an Ident, never a Punct, so there
                // is no `'_'` arm here, it would be unreachable dead code.
                TokenTree::Punct(punct) => match punct.as_char() {
                    '\'' => segments.push(PasteSegment::Apostrophe(punct.span())),
                    ':' => {
                        let colon_span = punct.span();
                        let colon = Colon { span: colon_span };
                        let ident = match tokens.next() {
                            Some(TokenTree::Ident(ident)) => ident,
                            wrong => {
                                let span = wrong.as_ref().map_or(colon_span, TokenTree::span);
                                return Err(PasteError::new(span, "expected identifier after `:`"));
                            }
                        };

                        if ident.to_string().as_str() == "replace" {
                            let replace = tokens.next();

                            match replace {
                                Some(TokenTree::Group(group))
                                    if group.delimiter() == Delimiter::Parenthesis =>
                                {
                                    segments.push(PasteSegment::Replace(colon, group));
                                }
                                _ => {
                                    return Err(PasteError::new2(
                                        colon.span,
                                        ident.span(),
                                        "expected `(` after replace modifier",
                                    ));
                                }
                            }
                        } else {
                            segments.push(PasteSegment::Modifier(colon, ident));
                        }
                    }
                    '#' => segments.push(PasteSegment::String(LitStr {
                        value: "#".to_string(),
                        span: punct.span(),
                    })),
                    _ => return Err(PasteError::new(punct.span(), "unexpected punct")),
                },
                TokenTree::Group(group) => {
                    if group.delimiter() == Delimiter::None {
                        let mut inner = group.stream().into_iter().peekable();
                        let nested = Self::parse(&mut inner)?;
                        if let Some(unexpected) = inner.next() {
                            return Err(PasteError::new(unexpected.span(), "unexpected token"));
                        }
                        segments.extend(nested);
                    } else {
                        return Err(PasteError::new(group.span(), "unexpected token"));
                    }
                }
            }
        }
        Ok(segments)
    }

    pub(crate) fn paste(segments: &[PasteSegment]) -> PasteResult<String> {
        let mut evaluated = Vec::new();
        let mut is_lifetime = false;

        for (i, segment) in segments.iter().enumerate() {
            match segment {
                PasteSegment::String(segment) => {
                    if segment.value.as_str() == "#" {
                        if i == 0 {
                            // Enable Raw mode
                            evaluated.push(String::from("r#"));
                            continue;
                        }
                        return Err(PasteError::new(
                            segment.span,
                            "`#` is reserved keyword and it enables the raw mode \
                                (i.e. generate Raw Identifiers) and it is only allowed in \
                                the beginning like `[< # ... >]`",
                        ));
                    }
                    evaluated.push(segment.value.clone());
                }
                PasteSegment::Apostrophe(span) => {
                    if is_lifetime {
                        return Err(PasteError::new(*span, "unexpected lifetime"));
                    }
                    is_lifetime = true;
                }
                PasteSegment::Env(var) => {
                    let Ok(resolved) = std::env::var(&var.value) else {
                        return Err(PasteError::new(
                            var.span,
                            &format!("no such env var: {:?}", var.value),
                        ));
                    };
                    let resolved = resolved.replace('-', "_");
                    evaluated.push(resolved);
                }
                PasteSegment::Modifier(colon, ident) => {
                    let Some(last) = evaluated.pop() else {
                        return Err(PasteError::new2(
                            colon.span,
                            ident.span(),
                            "unexpected modifier",
                        ));
                    };
                    match ident.to_string().as_str() {
                        "lower" => {
                            evaluated.push(last.to_lowercase());
                        }
                        "upper" => {
                            evaluated.push(last.to_uppercase());
                        }
                        "snake" => {
                            let mut acc = String::new();
                            let mut prev = '_';
                            for ch in last.chars() {
                                if ch.is_uppercase() && prev != '_' {
                                    acc.push('_');
                                }
                                acc.push(ch);
                                prev = ch;
                            }
                            evaluated.push(acc.to_lowercase());
                        }
                        "camel" | "upper_camel" | "lower_camel" => {
                            let mut is_lower_camel = ident.to_string().as_str() == "lower_camel";
                            let mut acc = String::new();
                            let mut prev = '_';
                            for ch in last.chars() {
                                if ch != '_' {
                                    if prev == '_' {
                                        if is_lower_camel {
                                            for chl in ch.to_lowercase() {
                                                acc.push(chl);
                                            }
                                            is_lower_camel = false;
                                        } else {
                                            for chu in ch.to_uppercase() {
                                                acc.push(chu);
                                            }
                                        }
                                    } else if prev.is_uppercase() {
                                        for chl in ch.to_lowercase() {
                                            acc.push(chl);
                                        }
                                    } else {
                                        acc.push(ch);
                                    }
                                }
                                prev = ch;
                            }
                            evaluated.push(acc);
                        }
                        "camel_edge" => {
                            let mut acc = String::new();
                            let mut prev = '_';
                            for ch in last.chars() {
                                if ch != '_' {
                                    if prev == '_' {
                                        for chu in ch.to_uppercase() {
                                            acc.push(chu);
                                        }
                                    } else if prev.is_uppercase() {
                                        for chl in ch.to_lowercase() {
                                            acc.push(chl);
                                        }
                                    } else {
                                        acc.push(ch);
                                    }
                                } else if prev == '_' {
                                    acc.push(ch);
                                }
                                prev = ch;
                            }
                            evaluated.push(acc);
                        }
                        _ => {
                            return Err(PasteError::new2(
                                colon.span,
                                ident.span(),
                                "unsupported modifier",
                            ));
                        }
                    }
                }
                PasteSegment::Replace(colon, group) => {
                    let mut inner_stream = group.stream().into_iter();
                    let from = inner_stream.next();
                    let punct = inner_stream.next();
                    let to = inner_stream.next();

                    if let Some(unexpected_token) = inner_stream.next() {
                        return Err(PasteError::new(unexpected_token.span(), "expected `)`"));
                    }

                    match (from, punct, to) {
                        (Some(from), Some(TokenTree::Punct(punct)), Some(to))
                            if punct.as_char() == ',' =>
                        {
                            let Some(last) = evaluated.pop() else {
                                return Err(PasteError::new2(
                                    colon.span,
                                    group.span(),
                                    "replace modifier requires a preceding value to operate on.",
                                ));
                            };
                            let from_str = get_token_tree_string_value(&from)?;
                            let to_str = get_token_tree_string_value(&to)?;

                            let new_ident = last.replace(&from_str, &to_str);

                            evaluated.push(new_ident);
                        }
                        _ => {
                            return Err(PasteError::new(
                                group.span(),
                                "expected replace modifier format: `:replace(\"from\", \"to\")`",
                            ));
                        }
                    }
                }
            }
        }

        let mut pasted = evaluated.into_iter().collect::<String>();
        if is_lifetime {
            pasted.insert(0, '\'');
        }
        Ok(pasted)
    }
}

// The three defensive sub-conditions below can never evaluate to false when called
// from get_literal_string_value with a real proc_macro::Literal, because-
//
//   ends_with('"')   — the Rust tokenizer rejects unterminated string literals before
//                      the macro receives them, so starts_with('"') implies ends_with('"').
//   ends_with('\'')  — same: the tokenizer rejects unterminated char literals.
//   len() >= 2       — impossible once both prefix and suffix chars are confirmed present.
//
// These checks are kept for correctness in case this function is called with an
// arbitrary &str in the future.  The false branches are exercised by the
// unit tests below, which call is_quoted_string_or_char directly
// with raw &str values (e.g. "\"abc", "'abc", "\"").
// proc_macro::Literal has no constructor that produces such malformed tokens, so
// the test_helpers.rs (with paste_test macro) path cannot cover these branches.
fn is_quoted_string_or_char(l_str: &str, parse_char: bool) -> bool {
    ((l_str.starts_with('"') && l_str.ends_with('"'))
        || (parse_char && l_str.starts_with('\'') && l_str.ends_with('\'')))
        && l_str.len() >= 2
}

fn get_literal_string_value(
    l: &Literal,
    parse_char: bool,
    parse_numbers: bool,
) -> PasteResult<String> {
    let l_str = l.to_string();

    if is_quoted_string_or_char(&l_str, parse_char) {
        // TODO: maybe handle escape sequences in the string if someone has a use case.
        Ok(String::from(&l_str[1..l_str.len() - 1]))
    } else if parse_numbers {
        Ok(l_str)
    } else {
        Err(PasteError::new(l.span(), "expected string literal"))
    }
}

pub(crate) fn get_token_tree_string_value(t: &TokenTree) -> PasteResult<String> {
    match t {
        TokenTree::Ident(ident) => Ok(ident.to_string()),
        TokenTree::Literal(literal) => get_literal_string_value(literal, true, true),
        TokenTree::Group(group) if group.delimiter() == Delimiter::None => {
            // Handle interpolated tokens from macro_rules (common in older Rust versions)
            let mut inner = group.stream().into_iter();
            if let Some(first) = inner.next() {
                if inner.next().is_none() {
                    // Single token in the group, recursively process it
                    return get_token_tree_string_value(&first);
                }
            }
            Err(PasteError::new(t.span(), "Expected either Ident, or Literal."))
        }
        _ => Err(PasteError::new(t.span(), "Expected either Ident, or Literal.")),
    }
}
