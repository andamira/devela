// devela::os::linux::fns::syscalls
//
//!
//
// - https://doc.rust-lang.org/reference/inline-assembly.html
// - https://doc.rust-lang.org/nightly/rust-by-example/unsafe/asm.html

// documentation is add in the parent module
#![allow(clippy::missing_safety_doc)]

pub use sys::*;

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
