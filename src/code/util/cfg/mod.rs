// devela/src/code/util/cfg/mod.rs
//
//! Conditional compilation and configuration.
//

mod _reexport_core;

mod deprecate; // deprecate_feature!

crate::structural_mods! { // _mods, _reexports,
    _mods {
        #[doc(inline)]
        pub use super::{
            deprecate::deprecate_feature,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
        #[doc = crate::_tags!(code procedural_macro)]
        pub use devela_macros::{
            cif, compile, compile_attr, // compile_doc,
        };
    }
}
