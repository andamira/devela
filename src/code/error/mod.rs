// devela::code::error
//
//! Errors, backtraces, structured handling.
//!
//! It re-exports the error and result types defined in other modules.
//!
#![doc = crate::doc_!(extends: backtrace, error)]
//

mod ext; // ExtError
mod all_error; // AllError, modular errors
mod reexports;

mod data; // data-related errors

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{all_error::*, data::*, ext::*, reexports::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{all_error::*, data::*, ext::*, reexports::*};
    }
}
