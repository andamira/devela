// devela::sys::os::linux
//
#![doc = crate::_DOC_SYS_OS_LINUX!()] // public
#![doc = crate::_doc!(modules: crate::sys::os; linux)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: sys)]
//!
//! # Supported architectures
//! Most of Linux functionality is feature-gated for the following target architectures:
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

// mod container; // WIP
mod error; // LinuxError, LinuxResult, LINUX_<ERRNO|EXIT>
pub mod io; // LinuxStat, LinuxTermios, LINUX_<FILENO|IOCTL|TERMIOS_*|…>-
mod namespace; // Linux
pub mod process; // LinuxSig<action|set|siginfo>, LINUX_SIG<ACTION|AIGNAL>
crate::_unsafe_syscall_not_miri! {
mod syscalls; } // LINUX_SYS
pub mod thread; // Linux<Clock|Instant|Time|Timespec>

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            // container::*,
            error::*,
            namespace::Linux,
        };
        #[crate::macro_apply(crate::_unsafe_syscall_not_miri)]
        pub use super::syscalls::_all::*;
    }
    _pub_mods {
        pub use super::{
            io::_all::*,
            process::_all::*,
            thread::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            io::_crate_internals::*,
            namespace::_crate_internals::*,
        };
    }
}
