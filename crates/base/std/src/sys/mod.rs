// devela_base_std::sys
//
#![doc = crate::_DOC_SYS!()] // public, root
#![doc = crate::_DOC_SYS_MODULES!()]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: alloc, arch, borrow, boxed, cell, env, fs, mem,
    io, net, os, path, pin, ptr, rc, slice)]
//
// safety
#![cfg_attr(feature = "safe_sys", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_SYS_MODULES =
    crate::_doc!(modules: crate; sys: arch, env, fs, mem, net, os); //  device, hw, io, log
}

pub mod arch;
pub mod env;
pub mod fs;
pub mod mem;
pub mod net;
pub mod os;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            arch::_all::*,
            env::_all::*,
            fs::_all::*,
            mem::_all::*,
            net::_all::*,
            os::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_SYS_MODULES;
    }
}
