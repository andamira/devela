// devela_base::data::list::link
//
#![doc = crate::_DOC_DATA_LIST_LINK!()]
//

mod r#const; // ConstList

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::r#const::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
