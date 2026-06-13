// devela/src/num/grain/cast/traits/mod.rs
//
//! Helpers for converting between primitives.
//

mod cast; // PrimCast
mod join; // PrimJoin
mod split; // PrimSplit

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            cast::*,
            join::*,
            split::*,
        };
    }
}
