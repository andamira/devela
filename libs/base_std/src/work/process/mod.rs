// devela_base_std::work
//
#![doc = crate::_DOC_WORK_PROCESS!()]
//

pub mod reexports;

crate::items! { // structural access: _mods, _pub_mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods {
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{
            reexports::*,
        };
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
