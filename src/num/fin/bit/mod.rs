// devela::num::fin::bit
//
#![doc = crate::_DOC_NUM_FIN_BIT!()]
//

#[cfg(test)]
mod tests;

crate::structural_mods! { // _reexports
    _reexports {
        #[doc(inline)]
        pub use devela_base_core::{ // bit
            BitOps, Bitwise
        };
    }
}
