// devela_base_alloc::data::list
//
#![doc = crate::_DOC_DATA_LIST!()]
//

mod array;
// mod queue;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::array::_all::*;
        // pub use super::queue::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
