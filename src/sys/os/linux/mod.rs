// devela::sys::os::linux
//
//! Linux-specific definitions
//!
//! # Supported architectures
//!
//! Linux functionality will only be compiled in the following target architectures:
//! - `x86-64`, `x86`, `ARM`, `AArch64`, `RISC-V RV32`, `RISC-V RV64`.
//!
//! Apart from that, this module will compile without checking whether the
//! current OS is `linux`, since that would require enabling `std` and the
//! whole point is to be able to use all of this functionality from `no_std`.
//
// NOTE: doc cfg attributes for target_arch are hidden from reexports
// in order to be have a more concise documentation in the libera crate.
// This is achieved by attaching a brief version to the item itself,
// and attaching a complete version to the module that reexports them.
//
// This is so both for syscalls and safe syscall wrappers. And when more
// platforms are supported they will all need to be updated accordingly.
//
// TOC
// - private modules
// - public modules
//   - io
//   - process
//   - thread
// - structural_mods

mod consts;
mod error;
mod namespace;
mod structs;

#[cfg(all(feature = "unsafe_syscall", not(miri)))]
crate::items! {
    mod point_entry; // linux_entry!
    #[cfg_attr(not(feature = "__force_test_no_mangle"), cfg(not(test)))]
    mod restorer;
    // #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_syscall")))]
    mod syscalls; // LINUX_SYS
}

/// Linux-specific extensions to [`std::io`].
pub mod io {
    #[allow(unused)]
    pub use _all::*;
    pub(super) mod _all {
        #[cfg(all(any_supported_arch, feature = "unsafe_syscall", not(miri)))]
        crate::items! {
            pub use super::super::{consts::termios::*, structs::LinuxStat};
            #[cfg(feature = "term")]
            pub use super::super::structs::LinuxTermios;
        }
    }
}

/// Linux-specific extensions to [`std::process`].
pub mod process {
    #[allow(unused)]
    pub use _all::*;
    pub(super) mod _all {
        #[cfg(all(any_supported_arch, feature = "unsafe_syscall", not(miri)))]
        pub use super::super::{
            consts::signal::*,
            structs::{LinuxSigaction, LinuxSiginfo, LinuxSigset},
        };
    }
}

/// Linux-specific extensions to [`std::thread`].
pub mod thread {
    #[allow(unused)]
    pub use _all::*;
    pub(super) mod _all {
        #[cfg(all(any_supported_arch, feature = "unsafe_syscall", not(miri)))]
        pub use super::super::structs::LinuxTimespec;
    }
}

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{error::*, namespace::*};

        #[cfg(all(feature = "unsafe_syscall", not(miri)))]
        pub use super::{point_entry::*, syscalls::_all::*};
    }
    _pub_mods {
        pub use super::{consts::_all::*, io::_all::*, process::_all::*, thread::_all::*};
    }
}
