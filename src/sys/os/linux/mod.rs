// devela::sys::os::linux
//
//! Linux-specific definitions
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

mod error; // LinuxError, LinuxResult, LINUX_[ERRNO|EXIT]
mod file; // LinuxStat, LINUX_[F_CMD|FILENO|IOCTL|O_FLAGS|S_IFMT|SEEK]
mod namespace; // Linux
mod signal; // LinuxSigaction, LinuxSiginfo, LinuxSigset, LINUX_[SIGACTION|SIGNAL]
mod term; // LinuxTermios, LINUX_TERMIOS
mod time; // LinuxClock, LinuxInstant, LinuxTime, LinuxTimespec

#[cfg(all(feature = "unsafe_syscall", not(miri)))]
crate::items! {
    mod point_entry; // linux_entry!
    mod syscalls; // LINUX_SYS
}

/// Linux-specific extensions to [`std::io`].
pub mod io {
    crate::structural_mods! { // _mods
        _mods {
            pub use super::super::{
                file::*,
                term::*,
            };
        }
    }
}

/// Linux-specific extensions to [`std::process`].
pub mod process {
    crate::structural_mods! { // _mods
        _mods {
            pub use super::super::signal::*;
        }
    }
}

/// Linux-specific extensions to [`std::thread`].
pub mod thread {
    crate::structural_mods! { // _mods
        _mods {
            pub use super::super::time::*;
        }
    }
}

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{
            error::*,
            namespace::*,
        };

        #[cfg(all(feature = "unsafe_syscall", not(miri)))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_syscall")))]
        pub use super::{
            point_entry::*,
            syscalls::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            io::_all::*,
            process::_all::*,
            thread::_all::*,
        };
    }
}
