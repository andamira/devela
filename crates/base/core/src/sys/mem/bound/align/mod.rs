// devela_base_core::sys::mem::bound::align
//
//!
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
