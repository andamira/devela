// devela_base::text::str
//
#![doc = crate::_DOC_TEXT_STR!()]
//

mod reexports;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::reexports::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
