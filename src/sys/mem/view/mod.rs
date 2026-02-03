// devela::sys::mem::view
//
#![doc = crate::_DOC_SYS_MEM_VIEW!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; view)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: borrow, slice)]
//

mod borrow; // Borrowed data and ownership-relaxed views.
mod slice; // Slice, SliceExt

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            borrow::_all::*,
            slice::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use devela_base_core::sys::mem::{
            MaybeByte,
        };
    }
}
