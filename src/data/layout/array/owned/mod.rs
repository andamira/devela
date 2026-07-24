// devela/src/data/layout/array/owned/mod.rs
//
//! Owning array containers and storage-specific utilities.
//

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod vec;

crate::structural_mods! {
    _mods {
        #[cfg(feature = "alloc")]
        pub use super::vec::_all::*;
    }
}
