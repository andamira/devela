// devela_base_alloc::sys::mem::alloc
//
#![doc = crate::_DOC_SYS_MEM_ALLOC!()]
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
