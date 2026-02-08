// devela_base_std::ui
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_UI!()] // public, root
#![doc = crate::_DOC_UI_MODULES!()]
#![doc = crate::_doc!(flat:"ui")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_ui", forbid(unsafe_ui))]
// docs
crate::CONST! { pub(crate) _DOC_UI_MODULES =
    crate::_doc!(modules: crate; ui); // back, front, layout
}

crate::structural_mods! { // _crate_internals
    _crate_internals {
        pub(crate) use super::_DOC_UI_MODULES;
    }
}
