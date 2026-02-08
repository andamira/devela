// devela_base_std::geom
//
#![doc = crate::_tags!(wip)] // public, root
#![doc = crate::_DOC_GEOM!()]
#![doc = crate::_DOC_GEOM_MODULES!()]
#![doc = crate::_doc!(flat:"geom")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_geom", forbid(unsafe_geom))]
// docs
crate::CONST! { pub(crate) _DOC_GEOM_MODULES =
    crate::_doc!(modules: crate; geom); // metric, shape, field
}

crate::structural_mods! { // _crate_internals
    _crate_internals {
        pub(crate) use super::_DOC_GEOM_MODULES;
    }
}
