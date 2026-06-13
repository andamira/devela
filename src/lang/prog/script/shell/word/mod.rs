// devela/src/lang/prog/script/shell/word/mod.rs
//
//! Shell word parsing and quoting.
//

#[cfg(test)]
mod tests;

mod error;
mod lex;
mod quote;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            error::*,
            lex::*,
            quote::*,
        };
    }
}
