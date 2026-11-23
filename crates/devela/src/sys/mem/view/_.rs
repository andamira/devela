//
#![doc = crate::_DOC_SYS_MEM_VIEW!()] // private
#![doc = crate::_doc!(modules: crate::sys::mem; view)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: borrow, slice)]
//

crate::mods_in! {
    mod_ borrow; // Borrowed data and ownership-relaxed views.
    mod byte; // MaybeByte

    #[cfg(feature = "replica")]
    mod_ replica; // MemReplicaError, MemReplicaSlice
    mod_ slice; // Slice, SliceExt
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            borrow::_all::*,
            byte::MaybeByte,
            slice::_all::*,
        };
        #[cfg(feature = "replica")]
        pub use super::replica::_all::*;
    }
}
