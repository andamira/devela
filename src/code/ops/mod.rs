// devela::code::ops
//
#![doc = crate::_DOC_CODE_OPS!()] // public
#![doc = crate::_doc!(modules: crate::code; ops)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: ops)]
//

mod _reexport_core;

mod call; // Call[Semantics|BindTime|Context|Dispatch|Openness|Storage]
mod punroll; // punroll!

// #[cfg(feature = "std")]
// pub mod _wip_fns; // WIP
// mod _wip_closure; // WIP

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            call::*,
            punroll::*,
        };
        // #[cfg(feature = "std")]
        // pub use super::_wip_fns::*;
        // pub use super::_wip_closure::*;
    }
    _reexports {
        #[doc(inline)]
        pub use super::_reexport_core::*;
    }
}
