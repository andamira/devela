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

mod consts;
mod structs;

#[cfg(all(feature = "unsafe_syscall", not(miri)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_syscall")))]
mod fns;

#[cfg(all(feature = "unsafe_syscall", not(miri)))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_syscall")))]
mod terminal;

/// Linux-specific extensions to [`std::io`].
pub mod io {
    #[allow(unused)]
    pub use _all::*;
    pub(super) mod _all {
        #[cfg(all(
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "arm",
                target_arch = "aarch64",
                target_arch = "riscv32",
                target_arch = "riscv64"
            ),
            feature = "unsafe_syscall",
            not(miri),
        ))]
        pub use super::super::{
            consts::termios::*,
            fns::{
                linux_eprint, linux_eprintln, linux_get_byte, linux_get_char, linux_get_dirty_char,
                linux_get_line, linux_get_str, linux_get_utf8_bytes, linux_pause_until_char,
                linux_print, linux_print_bytes, linux_println, linux_prompt, linux_random_bytes,
                linux_random_u128, linux_random_u16, linux_random_u32, linux_random_u64,
                linux_random_u8, linux_sys_getrandom, linux_sys_ioctl, linux_sys_read,
                linux_sys_write,
            },
            structs::{LinuxTerminalSize, LinuxTermios},
        };
    }
}

/// Linux-specific extensions to [`std::process`].
pub mod process {
    #[allow(unused)]
    pub use _all::*;
    pub(super) mod _all {
        #[cfg(all(
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "arm",
                target_arch = "aarch64",
                target_arch = "riscv32",
                target_arch = "riscv64"
            ),
            feature = "unsafe_syscall",
            not(miri),
        ))]
        pub use super::super::{
            consts::signal::*,
            fns::{linux_sig_handler_no_return, linux_sys_exit, linux_sys_rt_sigaction},
            structs::{LinuxSigaction, LinuxSigset},
        };
    }
}

/// Linux-specific extensions to [`std::thread`].
pub mod thread {
    #[allow(unused)]
    pub use _all::*;
    pub(super) mod _all {
        #[cfg(all(
            any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "arm",
                target_arch = "aarch64",
                target_arch = "riscv32",
                target_arch = "riscv64"
            ),
            feature = "unsafe_syscall",
            not(miri),
        ))]
        pub use super::super::{
            fns::{linux_getpid, linux_sleep, linux_sys_getpid, linux_sys_nanosleep},
            structs::LinuxTimespec,
        };
    }
}

crate::items! { // structural access: _mods, _pub_mods, _all
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _mods {
        #[cfg(all(feature = "unsafe_syscall", not(miri)))]
        pub use super::terminal::*;
    }
    mod _pub_mods { #![allow(unused)]
        pub use super::{consts::_all::*, io::_all::*, process::_all::*, thread::_all::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
}
