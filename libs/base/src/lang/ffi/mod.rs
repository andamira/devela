// devela_base::lang::ffi
//
#![doc = crate::_DOC_LANG_FFI!()]
//

pub mod c;

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods { #![allow(unused)]
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{
            c::_all::*,
        };
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
