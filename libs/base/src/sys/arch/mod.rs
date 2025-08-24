// devela_base::sys::arch
//
#![doc = crate::_DOC_SYS_ARCH!()]
//

mod reexports;
mod wasm;

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{
            reexports::*,
            wasm::*,
        };
    }
    mod _pub_mods { #![allow(unused)]
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
