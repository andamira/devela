// devela::text::fmt
//
#![doc = crate::_DOC_TEXT_FMT!()]
//!
#![doc = crate::_doc!(extends: fmt)]
//

mod buf;
mod namespace; // Fmt
mod reexports;

#[cfg(feature = "fmt")]
mod num_to_str;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{buf::*, namespace::*, reexports::*};

        #[cfg(feature = "fmt")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "fmt")))]
        pub use super::num_to_str::*;
        // WIPZONE
        // pub use super::table::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
// WIPZONE
// mod table;
