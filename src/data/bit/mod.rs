// devela::data::bit
//
#![doc = crate::_DOC_DATA_BIT!()] // private
#![doc = crate::_doc!(modules: crate::data; bit)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

#[cfg(test)]
mod tests;

crate::structural_mods! { // _reexports
    _reexports {
        #[doc(inline)]
        pub use devela_base_core::{
            bitfield,
        };
    }
}
