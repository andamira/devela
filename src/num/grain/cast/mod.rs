// devela::num::grain::cast
//
//! Helpers for converting between primitives.
//

mod cast; // PrimitiveCast
mod join; // PrimitiveJoin
mod split; // PrimitiveSplit

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            cast::*,
            join::*,
            split::*,
        };
    }
    _reexports {
        pub use devela_base_core::num::grain::{ // cast
            Cast,
        };
    }
}
