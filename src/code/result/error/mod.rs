// devela::code::result::error
//
//! Error-related types and traits.
//!
//! It re-exports the error and result types defined in other modules.
//!
#![doc = crate::doc_!(extends: error)]
//

mod ext; // ExtError
mod all_error; // AllError, modular errors
mod reexports;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{all_error::*, ext::*, reexports::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{all_error::*, ext::*, reexports::*};
    }
}
