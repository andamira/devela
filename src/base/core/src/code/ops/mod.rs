// devela_base_core::code::ops
//
#![doc = crate::_DOC_CODE_OPS!()] // public
#![doc = crate::_doc!(modules: crate::code; ops)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: ops)]
//

mod _reexport; // SYMLINK from /src/code/ops/_reexport_core.rs

mod punroll; // punroll!

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            punroll::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
