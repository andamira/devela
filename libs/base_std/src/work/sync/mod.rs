// devela_base_std::work::sync
//
#![doc = crate::_DOC_WORK_SYNC!()]
//

mod reexports;

pub mod mpsc;

crate::items! { // structural access: _mods, _pub_mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods {
        pub use super::{
            reexports::*,
        };
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{
            mpsc::*,
        };
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
