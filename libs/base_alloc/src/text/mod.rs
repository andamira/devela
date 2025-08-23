// devela_base_alloc::text
//
//! Text types and operations, text processing.
//

pub mod str;

crate::items! { // structural access: _mods, _pub_mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods {
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{
            // fmt::_all::*,
            // parse::_all::*,
            str::_all::*,
        };
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
