// devela_base_std::run
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_RUN!()] // public, root
#![doc = crate::_DOC_RUN_MODULES!()]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_run", forbid(unsafe_run))]
// docs
crate::CONST! { pub(crate) _DOC_RUN_MODULES =
    crate::_doc!(modules: crate; run); // frame, setup
}

crate::structural_mods! { // _crate_internals
    _crate_internals {
        pub(crate) use super::_DOC_RUN_MODULES;
    }
}
