// devela_base::sys::mem::cell
//
#![doc = crate::_DOC_SYS_MEM_CELL!()]
//

mod reexports;

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{
            reexports::*,
        };
    }
    mod _pub_mods { #![allow(unused)]
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
