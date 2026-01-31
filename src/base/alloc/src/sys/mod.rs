// devela_base_alloc::sys
//
#![doc = crate::_DOC_SYS!()] // public, root
#![doc = crate::_DOC_SYS_MODULES!()]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: alloc, arch, borrow, boxed, cell, env, fs, mem,
    io, net, os, path, pin, ptr, rc, slice)]
//
// safety
#![cfg_attr(base_safe_sys, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_SYS_MODULES =
    crate::_doc!(modules: crate; sys: mem); //  arch, device, env, fs, hw, io, log, net, os
}

pub mod mem;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            mem::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_SYS_MODULES;
    }
}
