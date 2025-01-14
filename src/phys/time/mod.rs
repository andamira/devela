// devela::phys::time
//
//! Time and calendar types and operations.
//!
#![doc = crate::doc_!(extends: time)]
//
// safety
#![cfg_attr(feature = "safe_time", forbid(unsafe_code))]

mod error;
mod reexports;

#[cfg(feature = "time")]
crate::items! {
    mod calendar;
    mod fmt;
    mod no;
    mod split;
    mod unix;
}

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        pub use super::{error::*, reexports::*};

        #[cfg(feature = "time")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "time")))]
        pub use super::{calendar::*, fmt::*, no::*, split::*, unix::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
