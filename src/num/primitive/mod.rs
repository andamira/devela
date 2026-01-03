// devela::num::primitive
//
//! Helpers for converting between primitives.
//

mod cast; // PrimitiveCast
mod join; // PrimitiveJoin
mod split; // PrimitiveSplit

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            cast::*,
            join::*,
            split::*,
        };
    }
}
