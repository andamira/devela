// devela/src/code/ops/_.rs
//
#![doc = crate::_DOC_CODE_OPS!()] // public
#![doc = crate::_doc!(modules: crate::code; ops: call, overload, range)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(extends: clone, ops, range)]
//!
//! Operations describe how values and computations act and compose rather than
//! the domain-specific algorithms they perform.
//!
//! [`call`] models invocation semantics, [`overload`] exposes Rust's overloadable
//! operators, and [`range`] describes bounded operational domains. The module
//! also contains small composition primitives such as [`Hook`] and [`Morph`].
//

crate::mods_in! {
    mod _reexport_core;

    pub mod_ call; // Callability and invocation semantics
    mod guard; // ScopeGuard
    mod hook_morph; // Hook, Morph, hook!, morph!
    pub mod_ overload; // Overloadable operator traits
    mod punroll; // punroll!
    pub mod_ range; // Range bounds, values, and traits

    // #[cfg(feature = "std")]
    // pub mod _wip_fns; // WIP
    // mod _wip_closure; // WIP
}
crate::mods_out! { // _mods, _pub_mods
    _mods {
        pub use super::{
            guard::ScopeGuard,
            hook_morph::{Hook, Morph, hook, morph},
            punroll::punroll,
        };
        // #[cfg(feature = "std")]
        // pub use super::_wip_fns::*;
        // pub use super::_wip_closure::*;
    }
    _pub_mods {
        pub use super::{
            call::_all::*,
            overload::_all::*,
            range::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::_reexport_core::*;
    }
}
