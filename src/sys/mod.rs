// devela::sys
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
    crate::_doc!(modules: crate; sys: arch, device, env, fs, io, log, mem, net, os); // hw
}

mod hw; // {audio, …} WIP

pub mod arch; // Arch, *asm, detect_*, m128* m256*
pub mod device; //
pub mod env; // App*, Arg*, Env
pub mod fs; // Fs, FsPath, PathExt
pub mod io; // Io*
pub mod log; // Log*
pub mod mem; // Mem,
pub mod net; // Ip*, Socket*, Tcp*, Udp*
pub mod os; // Linux,

// #[cfg(feature = "std")]
// mod bench; // WIP

crate::structural_mods! { // _mods, _pub_mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            hw::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            arch::_all::*,
            device::_all::*,
            env::_all::*,
            fs::_all::*,
            io::_all::*,
            log::_all::*,
            mem::_all::*,
            net::_all::*,
            os::_all::*,
        };
        // #[cfg(feature = "std")]
        // pub use super::bench::_all::*; // WIP
    }
    _crate_internals {
        pub(crate) use super::_DOC_SYS_MODULES;
        pub(crate) use super::device::_crate_internals::*;
    }
    _hidden {
        pub use super::mem::_hidden::*;
    }
}
