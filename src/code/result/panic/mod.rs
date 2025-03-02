// devela::code::result::panic
//
//! Panic support,
#![doc = crate::doc_!(extends: panic)]
#![doc = crate::doc_!(modules: crate::code::result; panic)]
#![doc = crate::doc_!(newline)]
//!
//

mod define; // define_panic_handler!
mod namespace; // Panic
mod reexports;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{define::*, namespace::*, reexports::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
