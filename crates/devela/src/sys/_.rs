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
    crate::_doc!(modules: crate; sys: arch, device, env, fs, hw, io, log, mem, net, os, service);
}

crate::mods_in! {
    pub mod_ arch; // Architecture-specific intrinsics
        // #[cfg(feature = "std")]
        // mod bench; // WIP
    pub mod_ device; // Live system device interfaces {alsa, x11}
    pub mod_ env; // Process environment inspection and manipulation
    pub mod_ fs; // Filesystem abstractions
    pub mod_ io; // I/O primitives and stream interfaces
    pub mod_ hw; // Low-level hardware and driver-facing system interfaces
    pub mod_ log; // Execution timing, measurement, and benchmark instrumentation
    pub mod_ mem; // Memory primitives, layout contracts, and safe access foundations
    pub mod_ net; // Network connectivity, addressing, routing, and transport
    pub mod_ os; // Operating systems and supervisors
    pub mod_ service; // Application-level protocols for networked services
}

crate::mods_out! { // _pub_mods, _reexports, _crate_internals, _hidden
    _pub_mods {
        pub use super::{
            arch::_all::*,
            device::_all::*,
            env::_all::*,
            fs::_all::*,
            hw::_all::*,
            io::_all::*,
            log::_all::*,
            mem::_all::*,
            net::_all::*,
            os::_all::*,
            service::_all::*,
        };
        // #[cfg(feature = "std")]
        // pub use super::bench::_all::*; // WIP
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            arch::Arch,
            io::TextOut,
            log::DiagOut,
            mem::Slice,
        };
        #[doc(inline)] #[cfg(feature = "std")]
        pub use super::fs::Fs;

        #[doc(inline)] #[cfg(feature = "term")]
        pub use super::os::TermGrid;

        #[doc(inline)] #[cfg(feature = "_linux_abi")]
        pub use super::os::Linux;

        #[doc(inline)] #[cfg(all(feature = "web", not(windows), not(feature = "safe_sys")))]
        pub use super::os::Web;

        #[doc(inline)] #[cfg(feature = "hw")]
        pub use super::hw::{I2cBusRead, SpiBusWrite};

        #[doc(inline)] #[cfg(all(unix, feature = "x11", not(feature = "safe_sys")))]
        pub use super::device::XFrontend;
    }
    _crate_internals {
        pub(crate) use super::_DOC_SYS_MODULES;
        pub(crate) use super::{
            arch::_crate_internals::*,
            device::_crate_internals::*,
            // net::_crate_internals::*,
            os::_crate_internals::*,
            service::_crate_internals::*,
        };
    }
    _hidden {
        pub use super::{
            log::_hidden::*,
            mem::_hidden::*,
        };
    }
}
