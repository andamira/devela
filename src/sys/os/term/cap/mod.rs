// devela/src/sys/os/term/cap/mod.rs
//
//! Terminal capabilities.
//

mod field; // TermCaps
mod list; // TermCap

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            field::*,
            list::*,
        };
    }
}
