// devela/src/text/str/array/mod.rs
//
//!
//

mod nonul; // StringNonul
mod u; // StringU8, StringU16

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            nonul::*,
            u::*,
        };
    }
}
