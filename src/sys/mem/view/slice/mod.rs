// devela::sys::mem::view::slice
//
#![doc = crate::_DOC_SYS_MEM_VIEW_SLICE!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; slice)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: slice)]

#[cfg(test)]
mod tests;

mod ext; // SliceExt

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::ext::*;
    }
    _reexports {
        #[doc(inline)]
        pub use devela_base_core::sys::mem::{ // view::slice
            Slice, const_join, slice,
        };
    }
}
