// devela::data::list::array
//
#![doc = crate::_DOC_DATA_LIST_ARRAY!()]
//!
#![doc = crate::_doc!(extends: array, vec)]
//!
//! They enable efficient iterable storage over a sequence of the same type.
//

crate::mod_path!(_c "../../../../libs/base/src/data/list/array/reexports.rs");

mod adt; // DataArray
mod d1; // 1-dimensional Array
mod d2; // 2-dimensional Array2d
mod ext; // ExtArray, ArrayFmt
mod from; // ArrayFrom
mod init; // array_init!

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod vec;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{_c::*, adt::*, d1::_all::*, d2::_all::*, ext::*, from::*, init::*};

        #[cfg(feature = "alloc")]
        pub use super::vec::_all::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        #[cfg(feature = "alloc")]
        pub use super::vec::_always::*;
    }
}
