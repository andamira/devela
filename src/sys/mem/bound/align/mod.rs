// devela::sys::mem::bound::align
//
//! Memory alignment bounds.
//

mod aligned; // Aligned
mod cache; // CacheAlign

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            aligned::*,
            cache::*,
        };
    }
}
