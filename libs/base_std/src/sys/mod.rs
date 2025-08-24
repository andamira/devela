// devela_base_std::sys
//
#![doc = crate::_DOC_SYS!()]
//

pub mod arch;
pub mod env;
pub mod fs;
pub mod mem;
pub mod net;

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods { #![allow(unused)]
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{
            arch::_all::*,
            env::_all::*,
            fs::_all::*,
            mem::_all::*,
            net::_all::*,
        };
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
