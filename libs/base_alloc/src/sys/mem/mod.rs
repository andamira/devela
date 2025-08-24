// devela_base_alloc::sys::mem
//
#![doc = crate::_DOC_SYS_MEM!()]
//


mod alloc;
mod borrow;
// mod pin;
// mod ptr;
mod reexports;
// pub mod cell;

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{
            alloc::_all::*,
            borrow::_all::*,
            // pin::_all::*,
            // ptr::_all::*,
            reexports::*,
        };
    }
    mod _pub_mods { #![allow(unused)]
        // pub use super::{
        //     cell::_all::*,
        // };
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
