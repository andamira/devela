// devela/src/lang/prog/script/form/shell/word/mod.rs
//
//! Shell word parsing and quoting.
//

#[cfg(test)]
mod _test;

mod error;
mod lex;
mod quote;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            error::ShellWordError,
            lex::ShellLex,
            quote::ShellQuote,
        };
    }
}
