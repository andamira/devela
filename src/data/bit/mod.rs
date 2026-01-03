// devela::data::bit
//
#![doc = crate::_DOC_DATA_BIT!()]
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
