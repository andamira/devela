// devela::os::linux
//
//! Linux-specific definitions
//
// NOTE: doc cfg attributes for target_arch are hidden from reexports
// in order to be have a more concise documentation in the libera crate.
// This is achieved by attaching a brief version to the item itself,
// and attaching a complete version to the module that reexports them.
//
// This is so both for syscalls and safe syscall wrappers. And when more
// platforms are supported they will all need to be updated accordingly.

mod consts;
#[cfg(all(feature = "unsafe_os", not(miri)))]
mod fns;
#[cfg(all(feature = "unsafe_os", not(miri)))]
mod syscalls;

pub use all::*;
pub(super) mod all {
    #[doc(inline)]
    pub use super::{consts::all::*, io::*, process::*, thread::*};
}

/* public modules */

/// Linux-specific extensions to [`std::io`].
pub mod io {
    #[cfg(all(
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "riscv32",
            target_arch = "riscv64"
        ),
        feature = "unsafe_os",
        not(miri),
    ))]
    pub use super::fns::{
        disable_raw_mode, enable_raw_mode, get_byte, is_terminal, print, print_bytes, sys_ioctl,
        sys_read, sys_write, SysTermios,
    };
}

/// Linux-specific extensions to [`std::process`].
pub mod process {
    #[cfg(all(
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "riscv32",
            target_arch = "riscv64"
        ),
        feature = "unsafe_os",
        not(miri),
    ))]
    pub use super::fns::sys_exit;
}

/// Linux-specific extensions to [`std::thread`].
pub mod thread {
    #[cfg(all(
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
        ),
        feature = "unsafe_os",
        not(miri),
    ))]
    pub use super::fns::{sleep, sys_nanosleep, SysTimeSpec};
}
