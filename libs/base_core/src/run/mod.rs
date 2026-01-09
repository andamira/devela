// devela_base_core::run
//
#![doc = crate::_DOC_RUN!()]
#![doc = crate::_DOC_RUN_MODULES!()]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(newline)]
//
// safety
#![cfg_attr(base_safe_run, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_RUN_MODULES =
    crate::_doc!(modules: crate; run: frame); // setup
}

crate::structural_mods! { // _mods, _crate_internals
    _mods {
    }
    _crate_internals {
        pub(crate) use super::_DOC_RUN_MODULES;
    }
}
