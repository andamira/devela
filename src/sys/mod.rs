// devela/src/sys/mod.rs
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

crate::mods_in! {
    pub mod_ arch; // Architecture-specific intrinsics
    pub mod_ mem; // Memory primitives, layout contracts, and safe access foundations

    // #[cfg(feature = "std")]
    // mod bench; // WIP
}
// WIP:
pub mod device; // Live system device interfaces {alsa, x11}
pub mod env; // Process environment inspection and manipulation
pub mod fs; // Filesystem abstractions
mod hw; // Low-level hardware and driver-facing system interfaces WIP
pub mod io; // I/O primitives and stream interfaces
pub mod log; // Execution timing, measurement, and benchmark instrumentation

pub mod net; // Networking functionality
pub mod os; // Operating systems and supervisors

crate::mods_out! { // _mods, _pub_mods, _crate_internals, _hidden
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
        pub(crate) use super::{
            arch::_crate_internals::*,
            device::_crate_internals::*,
            net::_crate_internals::*,
            os::_crate_internals::*,
        };
    }
    _hidden {
        pub use super::{
            mem::_hidden::*,
        };
    }
}
