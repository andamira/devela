// devela::text::parse::scanner
//
//! Defines [`TextScanner`]
//

// impls
mod byte;
mod quote;
mod range;

mod define; // TextScanner

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            define::*,
        };
    }
}
