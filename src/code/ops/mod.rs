// devela::code::ops
//
#![doc = crate::_DOC_CODE_OPS!()] // public
#![doc = crate::_doc!(modules: crate::code; ops)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: ops)]
//

mod _reexport_core; // SYMLINK to /crates/base/core/src/code/ops/_reexport.rs

// WIPZONE
// #[cfg(feature = "std")]
// pub mod _wip_fns;
// mod _wip_closure; // TODO

crate::structural_mods! { // _mods
    _mods {
        // WIPZONE
        // #[cfg(feature = "std")]
        // pub use super::_wip_fns::*;
        // pub use super::_wip_closure::*;
    }
    _reexports {
        #[doc(inline)]
        pub use super::_reexport_core::*;

        pub use devela_base_core::code::ops::{ // call
            CallSemantics,
            CallBindTime, CallContext, CallDispatch, CallOpenness, CallStorage,
        };
    }
}
