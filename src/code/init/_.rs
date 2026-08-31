// devela/src/code/init/_.rs
//
#![doc = crate::_DOC_CODE_INIT!()] // public
#![doc = crate::_doc!(modules: crate::code; init)]
#![doc = crate::_doc!(flat: "code")]
#![doc = crate::_doc!(extends: default)]
//

crate::mods_in! {
    mod _reexport_core;

    mod r#const;
}
crate::mods_out! { // _mods, _reexports, _crate_internals
    _mods {
        pub use super::r#const::ConstInit;
    }
    _reexports {
        pub use super::_reexport_core::Default;
    }
    _crate_internals {
        pub(crate) use super::r#const::_impl_init;
    }
}
