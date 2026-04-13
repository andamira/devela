// devela::text::fmt
//
#![doc = crate::_DOC_TEXT_FMT!()] // public
#![doc = crate::_doc!(modules: crate::text; fmt)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(extends: fmt)]
//

mod _reexport_core;
#[cfg(feature = "alloc")]
mod _reexport_alloc;

mod buf; // FmtWriter, format_buf!
mod cat; // fmtcat!
// mod case; // WIP
mod debug; // DebugExt
mod namespace; // Fmt
mod num; // FmtNum, FmtNumGroup, FmtNumShape, FmtNumConf, FmtNumSign, fmt_num! WIP
// mod table; // WIP

crate::structural_mods! { // _mods, _reexports, _hidden
    _mods {
        pub use super::namespace::*;
        pub use super::{
            buf::*,
            cat::*,
            // case::*,
            debug::*,
            num::_all::*,
            // table::*,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[cfg(feature = "alloc")]
        pub use super::_reexport_alloc::*;
    }
    _hidden {
        pub use super::num::_hidden::*;
    }
}
