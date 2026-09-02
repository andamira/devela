// devela/src/lang/prog/script/form/shell/word/_.rs
//
//! Shell word parsing and quoting.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod error;
    mod lex;
    mod quote;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            error::ShellWordError,
            lex::ShellLex,
            quote::ShellQuote,
        };
    }
}
