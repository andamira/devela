// devela_base_core::sys::mem::arena
//
//!
//

#[cfg(test)]
mod tests;

mod bytes; // ArenaBytes, ArenaMark
mod handle; // ArenaHandle

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            bytes::*,
            handle::*,
        };
    }
}
