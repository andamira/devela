// devela_macros::bodies::derive
//
//! Attribute-position adapters for `macro_rules!`.
//

use proc_macro::{Delimiter, Group, Punct, Spacing, TokenStream as TS, TokenTree as TT};

#[inline(always)]
pub(crate) fn body_macro_apply(args: TS, input: TS) -> TS {
    macro_call(args.into_iter().collect(), input)
}

#[inline(always)]
pub(crate) fn body_macro_derive(args: TS, input: TS) -> TS {
    let args = match split_top_commas(args.into_iter().collect()) {
        Ok(args) => args,
        Err(err) => return err,
    };
    let mut parsed = Vec::with_capacity(args.len());
    for arg in &args {
        let Ok(head) = parse_macro_head(arg.clone()) else {
            return error(
                "expected `path::to::Macro`, `path::to::Macro!`, or `path::to::Macro!(...)`",
            );
        };
        parsed.push(head);
    }
    // Pure classic derive path.
    if parsed.iter().all(|head| !head.had_bang) {
        if parsed.iter().any(|head| head.args.is_some()) {
            return error("macro derives with arguments must end in `!`");
        }
        let mut out = real_derive(join_commas(args));
        out.extend(input);
        return out;
    }
    let mut out = TS::new();
    let mut real_derives = Vec::new();
    for (arg, head) in args.into_iter().zip(parsed) {
        if head.had_bang {
            out.extend(macro_call_parsed(head, input.clone()));
        } else if head.args.is_some() {
            return error("macro derives with arguments must end in `!`");
        } else {
            real_derives.push(arg);
        }
    }
    let mut real_derives = join_commas(real_derives);
    if !real_derives.is_empty() {
        real_derives.extend([comma()]);
    }
    real_derives.extend("::devela::__macro_derive_helpers,".parse::<TS>().unwrap());
    out.extend(real_derive(real_derives));
    out.extend(input);
    out
}
#[inline(always)]
pub(crate) fn body_macro_derive_with(args: TS, input: TS) -> TS {
    let args = match split_top_commas(args.into_iter().collect()) {
        Ok(args) => args,
        Err(err) => return err,
    };
    let mut out = TS::new();
    for arg in args {
        out.extend(macro_call(arg, input.clone()));
    }
    // permits inert helper attrs such as `#[__custom(...)]` on the original item
    out.extend(macro_attrs_derive());
    out.extend(input);
    out
}

/* helpers */

struct MacroHead {
    path: Vec<TT>,
    args: Option<Group>,
    had_bang: bool,
}
// This accepts: m m! m("x") m!("x") path::to::m("x", "y") path::to::m!("x", "y")
fn parse_macro_head(mut tts: Vec<TT>) -> Result<MacroHead, ()> {
    strip_trailing_comma(&mut tts);
    let args = if matches!(
        tts.last(),
        Some(TT::Group(g)) if g.delimiter() == Delimiter::Parenthesis
    ) {
        match tts.pop().unwrap() {
            TT::Group(g) => Some(g),
            _ => unreachable!(),
        }
    } else {
        None
    };
    let had_bang = path_bang(&tts)?;
    if had_bang {
        strip_trailing_bang(&mut tts)?;
    }
    Ok(MacroHead { path: tts, args, had_bang })
}
fn macro_call(head: Vec<TT>, input: TS) -> TS {
    let Ok(head) = parse_macro_head(head) else {
        return error("expected `path::to::macro`, optionally followed by `(...)`");
    };
    macro_call_parsed(head, input)
}
fn macro_call_parsed(mut head: MacroHead, input: TS) -> TS {
    head.path.push(bang());
    let mut body = TS::new();
    if let Some(args) = head.args {
        body.extend([TT::Group(args)]);
    }
    body.extend(input);
    head.path.push(TT::Group(Group::new(Delimiter::Brace, body)));
    head.path.into_iter().collect()
}
fn real_derive(derives: TS) -> TS {
    let mut inner: TS = "::core::prelude::v1::derive".parse().unwrap();
    inner.extend([TT::Group(Group::new(Delimiter::Parenthesis, derives))]);

    TS::from_iter([
        TT::Punct(Punct::new('#', Spacing::Alone)),
        TT::Group(Group::new(Delimiter::Bracket, inner)),
    ])
}
fn path_bang(tts: &[TT]) -> Result<bool, ()> {
    let mut i = 0;
    eat_colons(tts, &mut i);
    loop {
        match tts.get(i) {
            Some(TT::Ident(_)) => i += 1,
            _ => return Err(()),
        }
        if i == tts.len() {
            return Ok(false);
        }
        if matches!(tts.get(i), Some(tt) if is_bang(tt)) {
            i += 1;
            return if i == tts.len() { Ok(true) } else { Err(()) };
        }
        if !eat_colons(tts, &mut i) {
            return Err(());
        }
    }
}
fn split_top_commas(tts: Vec<TT>) -> Result<Vec<Vec<TT>>, TS> {
    if tts.is_empty() {
        return Err(error("expected at least one path"));
    }
    let mut out = vec![Vec::new()];
    for tt in tts {
        if is_comma(&tt) {
            if out.last().unwrap().is_empty() {
                return Err(error("empty argument"));
            }
            out.push(Vec::new());
        } else {
            out.last_mut().unwrap().push(tt);
        }
    }
    // accept one trailing comma
    if out.last().is_some_and(Vec::is_empty) {
        out.pop();
    }
    Ok(out)
}

fn join_commas(parts: Vec<Vec<TT>>) -> TS {
    let mut out = TS::new();
    for (i, part) in parts.into_iter().enumerate() {
        if i != 0 {
            out.extend([comma()]);
        }
        out.extend(part);
    }

    out
}
fn comma() -> TT {
    TT::Punct(Punct::new(',', Spacing::Alone))
}
fn bang() -> TT {
    TT::Punct(Punct::new('!', Spacing::Alone))
}
fn is_comma(tt: &TT) -> bool {
    matches!(tt, TT::Punct(p) if p.as_char() == ',')
}
fn is_bang(tt: &TT) -> bool {
    matches!(tt, TT::Punct(p) if p.as_char() == '!')
}
fn macro_attrs_derive() -> TS {
    real_derive("::devela::__macro_derive_helpers,".parse().unwrap())
}
fn strip_trailing_bang(tts: &mut Vec<TT>) -> Result<(), ()> {
    match tts.pop() {
        Some(TT::Punct(p)) if p.as_char() == '!' => Ok(()),
        _ => Err(()),
    }
}
fn strip_trailing_comma(tts: &mut Vec<TT>) {
    if tts.last().is_some_and(is_comma) {
        tts.pop();
    }
}
fn error(msg: &str) -> TS {
    format!("::core::compile_error!({msg:?});").parse().unwrap()
}
fn eat_colons(tts: &[TT], i: &mut usize) -> bool {
    let ok = matches!(tts.get(*i),
            Some(TT::Punct(p)) if p.as_char() == ':' && p.spacing() == Spacing::Joint)
        && matches!(tts.get(*i + 1),
            Some(TT::Punct(p)) if p.as_char() == ':' && p.spacing() == Spacing::Alone);
    if ok {
        *i += 2;
    }
    ok
}
