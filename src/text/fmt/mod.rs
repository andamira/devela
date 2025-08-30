// devela::text::fmt
//
#![doc = crate::_DOC_TEXT_FMT!()]
//!
#![doc = crate::_doc!(extends: fmt)]
//

crate::mod_path!(_c "../../../libs/base/src/text/fmt/reexports.rs");
crate::mod_path!(alloc _a "../../../libs/base_alloc/src/text/fmt/reexports.rs");

mod buf;
mod namespace; // Fmt

#[cfg(feature = "fmt")]
mod num_to_str;

// WIPZONE
// mod table;

crate::structural_mods! { // _mods, _always
    _mods {
        pub use super::{_c::*, buf::*, namespace::*};
        #[cfg(feature = "alloc")]
        pub use super::_a::*;

        #[cfg(feature = "fmt")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "fmt")))]
        pub use super::num_to_str::*;
        // WIPZONE
        // pub use super::table::*;
    }
    _always {
        pub use super::_c::*;
        #[cfg(feature = "alloc")]
        pub use super::_a::*;
    }
}
