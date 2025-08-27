// devela::code::panic
//
#![doc = crate::_DOC_CODE_PANIC!()]
//!
#![doc = crate::_doc!(extends: panic)]
//

mod namespace; // Panic
mod reexports; // ::core::panic::*
mod set; // set_panic_handler!

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{namespace::*, reexports::*, set::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
