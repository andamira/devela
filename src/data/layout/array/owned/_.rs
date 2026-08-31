// devela/src/data/layout/array/owned/_.rs
//
//! Owning array containers and storage-specific utilities.
//

crate::mods_in! {
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    mod_ vec;
}
crate::mods_out! {
    _mods {
        #[cfg(feature = "alloc")]
        pub use super::vec::_all::*;
    }
}
