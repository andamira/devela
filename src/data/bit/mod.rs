// devela::data::bit
//
//! Bit-focused items.
//

#[cfg(test)]
mod tests;

mod ops; // BitOps

crate::structural_mods! { // _mods
    _mods {
        pub use super::ops::*;

        #[doc(inline)]
        pub use devela_base_core::{Bitwise, bitfield};
    }
}
