// devela::text::fmt
//
#![doc = crate::_DOC_TEXT_FMT!()]
//!
#![doc = crate::_doc!(extends: fmt)]
//

crate::mod_path!(_c "../../../libs/base_core/src/text/fmt/reexports.rs");
crate::mod_path!(alloc _a "../../../libs/base_alloc/src/text/fmt/reexports.rs");

mod namespace; // Fmt

// WIPZONE
// mod case;
// mod table;

crate::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;

        // WIPZONE
        // pub use super::case::*;
        // pub use super::table::*;

        /* re-exports */

        pub use super::_c::*;
        #[cfg(feature = "alloc")]
        pub use super::_a::*;

        #[doc(inline)]
        pub use devela_base_core::text::fmt::{
            FmtNum, FmtWriter, NumToStr, format_buf,
        };
    }
}
