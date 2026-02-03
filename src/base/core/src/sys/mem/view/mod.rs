// devela_base_core::sys::mem::view
//
#![doc = crate::_DOC_SYS_MEM_VIEW!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; view)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: borrow, slice)]
//

mod borrow; // Borrowed data and ownership-relaxed views.
mod byte; // MaybeByte
mod slice; // Slice

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            borrow::_all::*,
            byte::*,
            slice::_all::*,
        };
    }
}
