// devela::os::linux::syscall
//
//!
//
// INFO
// - https://doc.rust-lang.org/reference/inline-assembly.html
// - https://doc.rust-lang.org/nightly/rust-by-example/unsafe/asm.html

// documentation is add in the parent module
#![allow(clippy::missing_safety_doc)]

#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(
        target_os = "linux",
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
            target_arch = "riscv32",
            target_arch = "riscv64"
        ),
        feature = "unsafe_os",
    )))
)]
pub use sys::{sys_exit, sys_ioctl, sys_read, sys_write};

#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(
        target_os = "linux",
        any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "arm",
            target_arch = "aarch64",
        ),
        feature = "unsafe_os",
    )))
)]
pub use sys::sys_nanosleep;

mod structs;
pub use structs::{SysTermios, SysTimeSpec};

/* architecture-specific */

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
use x86_64 as sys;

#[cfg(target_arch = "x86")]
mod x86;
#[cfg(target_arch = "x86")]
use x86 as sys;

#[cfg(target_arch = "arm")]
mod arm;
#[cfg(target_arch = "arm")]
use arm as sys;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
use aarch64 as sys;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
mod riscv;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
use riscv as sys;
