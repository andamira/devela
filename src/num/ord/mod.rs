// devela::num::ord
//
#![doc = crate::_DOC_NUM_ORD!()]
// #![doc = crate::_doc!(extends: cmp)]
// #![doc = crate::_doc!(modules: crate::num; cmp)]
// #![doc = crate::_doc!(newline)]
//

mod reexports;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::reexports::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
