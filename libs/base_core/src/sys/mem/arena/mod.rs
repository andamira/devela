// devela_base_core::sys::mem::arena
//
//!
//

mod bytes; // ArenaBytes
mod handle; // ArenaHandle

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            bytes::*,
            handle::*,
        };
    }
}
