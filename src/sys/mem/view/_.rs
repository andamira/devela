// devela/src/sys/mem/view/_.rs
//
#![doc = crate::_DOC_SYS_MEM_VIEW!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; view)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: borrow, slice)]
//

crate::mods_in! {
    mod_ borrow; // Borrowed data and ownership-relaxed views.
    mod byte; // MaybeByte
    mod_ replica; // MemReplicaError, MemReplicaSlice
    mod_ slice; // Slice, SliceExt
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            borrow::_all::*,
            byte::MaybeByte,
            replica::_all::*,
            slice::_all::*,
        };
    }
}
