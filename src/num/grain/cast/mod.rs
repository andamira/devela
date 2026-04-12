// devela::num::grain::cast
//
//! Helpers for converting between primitives.
//

mod macros; // cast!
mod namespace; // Cast
mod traits; // PrimitiveCast, PrimitiveJoin, PrimitiveSplit

#[cfg(test)]
mod tests;

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
