// devela/src/text/ascii/digits/mod.rs
//
//! Defines [`Digits`].
//

#[cfg(test)]
mod _test;

mod _docs; // DOC_*
use _docs::*;

mod define; // Digits
mod u8;
mod u16;
mod u32;
mod u64;
mod u128;
mod usize;

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            define::Digits,
        };
    }
}
