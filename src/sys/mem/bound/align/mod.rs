// devela/src/sys/mem/bound/align/mod.rs
//
//! Memory alignment bounds.
//

mod aligned; // Aligned
mod cache; // CacheAlign

crate::mods_out! { // _mods
    _mods {
        pub use super::{
            aligned::*,
            cache::*,
        };
    }
}
