// devela/src/num/grain/cast/mod.rs
//
//! Helpers for converting between primitives.
//

#[cfg(test)]
mod _test;

mod macros; // cast!
mod namespace; // Cast
mod traits; // PrimCast, PrimJoin, PrimSplit

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            macros::cast,
            namespace::Cast,
            traits::_all::*,
        };
    }
}
