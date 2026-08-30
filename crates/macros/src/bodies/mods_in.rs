// devela_macros/src/bodies/mods_in.rs
//
//! Body of `mods_in!`.
//

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing};
use proc_macro::{TokenStream as TS, TokenTree as TT};

// Procedural because `mod_ name;` must synthesize the string literal
// `#[path = "name/_.rs"]`, which `macro_rules!` can't construct from an ident.
pub(crate) fn body_mods_in(input: TS) -> TS {
    let mut input = input.into_iter().peekable();
    let mut out = TS::new();
    while input.peek().is_some() {
        let mut attrs = TS::new();
        let mut vis = TS::new();
        // Preserve outer attributes.
        while matches!(input.peek(), Some(TT::Punct(p)) if p.as_char() == '#') {
            attrs.extend([input.next().unwrap()]);
            let Some(TT::Group(group)) = input.next() else {
                return error("expected attribute after `#`");
            };
            if group.delimiter() != Delimiter::Bracket {
                return error("expected attribute after `#`");
            }
            attrs.extend([TT::Group(group)]);
        }
        // Preserve visibility, including restricted forms.
        if matches!(input.peek(), Some(TT::Ident(i)) if i.to_string() == "pub") {
            vis.extend([input.next().unwrap()]);
            if matches!(input.peek(), Some(TT::Group(g)) if g.delimiter() == Delimiter::Parenthesis) {
                vis.extend([input.next().unwrap()]);
            }
        }
        let Some(TT::Ident(kind)) = input.next() else {
            return error("expected `mod` or `mod_`");
        };
        // `mod_` is a pseudo-keyword understood only by this macro.
        // Both forms ultimately emit a real Rust `mod` item.
        let directory = match kind.to_string().as_str() {
            "mod" => false,
            "mod_" => true,
            _ => return error("expected `mod` or `mod_`"),
        };
        let Some(TT::Ident(name)) = input.next() else {
            return error("expected module name");
        };
        let Some(TT::Punct(semi)) = input.next() else {
            return error("expected `;` after module name");
        };
        if semi.as_char() != ';' {
            return error("expected `;` after module name");
        }
        out.extend(attrs);
        // Directory modules use `_.rs` as their root source file.
        if directory {
            // Raw identifiers use their unescaped spelling in filesystem paths:
            // `r#type` resolves to `type/_.rs`.
            let name_string = name.to_string();
            let file_name = name_string.strip_prefix("r#").unwrap_or(&name_string);
            let path = format!("{file_name}/_.rs");
            let literal = Literal::string(&path);
            // #[path = "name/_.rs"]
            let mut attr = TS::new();
            attr.extend([
                TT::Ident(Ident::new("path", name.span())),
                TT::Punct(Punct::new('=', Spacing::Alone)),
                TT::Literal(literal),
            ]);
            out.extend([
                TT::Punct(Punct::new('#', Spacing::Alone)),
                TT::Group(Group::new(Delimiter::Bracket, attr)),
            ]);
        }
        out.extend(vis);
        out.extend([
            TT::Ident(Ident::new("mod", kind.span())),
            TT::Ident(name),
            TT::Punct(Punct::new(';', Spacing::Alone)),
        ]);
    }
    out
}
fn error(message: &str) -> TS {
    format!("compile_error!({message:?});").parse().unwrap()
}
