// devela::num::bit
//
#![doc = crate::_DOC_NUM_BIT!()]
//

#[cfg(test)]
mod tests;

crate::structural_mods! { // _reexports
    _reexports {
        #[doc(inline)]
        pub use devela_base_core::{
            BitOps, Bitwise
        };
    }
}
