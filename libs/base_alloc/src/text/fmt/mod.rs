// devela_base_alloc::text::fmt
//
#![doc = crate::_DOC_TEXT_FMT!()]
// #![doc = crate::doc_!(extends: fmt)]
//

mod reexports;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{
            reexports::*,
        };
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
