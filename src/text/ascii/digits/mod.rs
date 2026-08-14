// devela/src/text/ascii/digits/mod.rs
//
//! Defines [`Digits`].
//

#[cfg(test)]
mod _test;

mod _docs; // DOC_*
use _docs::*;

mod _helper; // ascii_digit_*

mod define; // Digits
mod u8;
mod u16;
mod u32;
mod u64;
mod u128;
mod usize;

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            define::Digits,
        };
    }
    _crate_internals {
        pub(crate) use super::_helper::*;
    }
}
