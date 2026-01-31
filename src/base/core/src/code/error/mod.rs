// devela_base_core::code::error
//
#![doc = crate::_DOC_CODE_ERROR!()] // public
#![doc = crate::_doc!(modules: crate::code; error)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: backtrace, error)]
//

mod _reexport;

// mod context; // ContextualError WIP
mod define_error; // define_error!
mod errors; // general errors

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            // context::*,
            define_error::*,
            errors::*,
        };
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
