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
    pub mod_ call; // Callability and invocation semantics
    mod hook_morph; // Hook, Morph, hook!, morph!
    pub mod_ overload; // Overloadable operator traits
    pub mod_ range; // Range bounds, values, and traits

    // #[cfg(feature = "std")]
    // pub mod _wip_fns; // WIP
    // mod _wip_closure; // WIP
}
crate::mods_out! { // _mods, _pub_mods
    _mods {
        pub use super::hook_morph::{Hook, Morph, hook, morph};
        #[doc(inline)]
        pub use crate::boot::base::code::punroll;
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
        pub use crate::boot::rust::code::ops::*;
    }
}
