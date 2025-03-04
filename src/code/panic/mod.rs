// devela::code::panic
//
//! Panic hooks, unwinding, abort strategies.
//!
#![doc = crate::doc_!(extends: panic)]
//

mod define; // define_panic_handler!
mod namespace; // Panic
mod reexports; // ::core::panic::*

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{define::*, namespace::*, reexports::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
