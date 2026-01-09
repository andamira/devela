// devela::run
//
#![doc = crate::_DOC_RUN!()]
#![doc = crate::_DOC_RUN_MODULES!()]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(newline)]
#![doc = crate::_QUOTE_RUN!()]
//!
//
// safety
#![cfg_attr(feature = "safe_run", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_RUN_MODULES =
    crate::_doc!(modules: crate; run: frame, setup);
}

// mod run; // Runtime
mod tick; // RuntimeTick

pub mod frame; // FramePacer // WIP
pub mod setup; // RunCap* WIP

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            // run::*,
            tick::*,
        };
    }
    _pub_mods {
        pub use super::{
            frame::_all::*,
            setup::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_RUN_MODULES;
    }
}
