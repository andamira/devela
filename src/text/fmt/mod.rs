// devela::text::fmt
//
#![doc = crate::_DOC_TEXT_FMT!()] // public
#![doc = crate::_doc!(modules: crate::text; fmt)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: fmt)]
//

mod _reexport_core; // SYMLINK to /src/base/core/src/text/fmt/_reexport.rs
#[cfg(feature = "alloc")]
mod _reexport_alloc; // SYMLINK to /src/base/alloc/src/text/fmt/_reexport.rs

mod namespace; // Fmt

// WIPZONE
// mod case;
// mod table;

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::namespace::*;

        // WIPZONE
        // pub use super::case::*;
        // pub use super::table::*;
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;

        #[doc(inline)]
        pub use devela_base_core::text::fmt::{
            DebugExt,
            FmtNum, FmtNumConf, FmtNumShape, FmtNumSign,
            FmtWriter, fmtcat, format_buf,
        };
    }
}
