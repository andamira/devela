// devela_base_alloc::data::list::array
//
#![doc = crate::_DOC_DATA_LIST_ARRAY!()]
//

// mod reexports;
mod vec;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        // pub use super::reexports::*;
        pub use super::vec::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
