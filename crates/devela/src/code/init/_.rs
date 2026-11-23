//
#![doc = crate::_DOC_CODE_INIT!()] // public
#![doc = crate::_doc!(modules: crate::code; init)]
#![doc = crate::_doc!(flat: "code")]
#![doc = crate::_doc!(extends: default)]
//!
//! Const-friendly initialization separates the ability to construct a valid
//! compile-time value from the semantic baseline expressed by [`Default`].
//!
//! [`ConstInit`] and its elemental implementation foundation are always available.
//! The `init` feature extends that foundation with broader implementations for
//! Rust library and dependency types.
//

crate::mods_in! {
    mod r#const;
}
crate::mods_out! { // _mods, _reexports, _crate_internals
    _mods {
        pub use super::r#const::ConstInit;
    }
    _reexports {
        pub use crate::boot::rust::code::init::*;
    }
    _crate_internals {
        pub(crate) use super::r#const::_impl_init;
    }
}
