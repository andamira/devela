// devela/src/data/layout/array/owned/mod.rs
//
//! Owning array containers and storage-specific utilities.
//

mod d1; // Array, ArrayUninit TEMPorary design
mod d2; // Array2d TEMPorary design

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod vec;

crate::structural_mods! {
    _mods {
        pub use super::{
            d1::_all::*,
            d2::_all::*,
        };
        #[cfg(feature = "alloc")]
        pub use super::vec::_all::*;
    }
}
