// devela::data::bit
//
//! Bit-focused items.
//

#[cfg(test)]
mod tests;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use devela_base_core::{
            BitOps, Bitwise, bitfield,
        };
    }
}
