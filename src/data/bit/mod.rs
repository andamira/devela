// devela::data::bit
//
#![doc = crate::_DOC_DATA_BIT!()]
//!
#![doc = crate::_doc!(modules: crate::code; bit)]
#![doc = crate::_doc!(flat:"data")]
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
