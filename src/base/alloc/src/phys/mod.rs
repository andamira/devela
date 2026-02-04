// devela_base_alloc::phys
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_PHYS!()] // public, root
#![doc = crate::_DOC_PHYS_MODULES!()]
#![doc = crate::_doc!(flat:"phys")]
#![doc = crate::_doc!(extends: time)]
//
// safety
#![cfg_attr(feature = "safe_phys", forbid(unsafe_phys))]
// docs
crate::CONST! { pub(crate) _DOC_PHYS_MODULES =
    crate::_doc!(modules: crate; phys); // astro, bio, chem, elec, mech, time, unit, wave
}

crate::structural_mods! { // _crate_internals
    _crate_internals {
        pub(crate) use super::_DOC_PHYS_MODULES;
    }
}
