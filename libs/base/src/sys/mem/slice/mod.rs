// devela_base::sys::mem::slice
//
//! Slices of memory.
//

mod join; // const_join!
mod namespace; // Slice

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            namespace::*,
            join::*,
        };
    }
}
