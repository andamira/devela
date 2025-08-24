// devela_base_alloc::data
//
#![doc = crate::_DOC_DATA!()]
//

pub mod list;
pub mod key;

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods { #![allow(unused)]
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{
            list::_all::*,
            key::_all::*,
        };
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
