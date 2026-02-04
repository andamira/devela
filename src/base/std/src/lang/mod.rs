// devela_base_std::lang
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_LANG!()] // public, root
#![doc = crate::_DOC_LANG_MODULES!()]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(extends: ffi)]
//
// safety
#![cfg_attr(feature = "safe_lang", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_LANG_MODULES =
    crate::_doc!(modules: crate; lang); // gram, hum, prog, repr, sem
}

crate::structural_mods! { // _crate_internals
    _crate_internals {
        pub(crate) use super::_DOC_LANG_MODULES;
    }
}
