// devela/src/sys/os/term/cap/mod.rs
//
//! Terminal capabilities.
//

mod field; // TermCaps
mod list; // TermCap

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            field::*,
            list::*,
        };
    }
}
