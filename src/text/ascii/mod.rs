// devela::text::ascii
//
//! ASCII strings and characters.
#![doc = crate::doc_!(extends: ascii)]
#![doc = crate::doc_!(modules: crate::text; ascii)]
#![doc = crate::doc_!(newline)]
//!
//

mod wrapper;

#[cfg(feature = "ascii")]
mod char;

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::wrapper::Ascii;

        #[cfg(feature = "ascii")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "ascii")))]
        pub use super::char::AsciiChar;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
