// devela::num::fin::bit
//
#![doc = crate::_DOC_NUM_FIN_BIT!()] // private
#![doc = crate::_doc!(modules: crate::num; bit)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
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
