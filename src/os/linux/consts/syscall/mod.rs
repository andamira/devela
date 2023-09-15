// devela::os::linux::syscall::consts
//
//! Defines the syscall numbers for each target.
//
// - https://marcin.juszkiewicz.com.pl/download/tables/syscalls.html

#![allow(non_camel_case_types)]

mod aarch64;
mod arm;
mod riscv;
mod x86;
mod x86_64;
pub use {aarch64::SYS_AARCH64, arm::SYS_ARM, riscv::SYS_RISCV, x86::SYS_X86, x86_64::SYS_X86_64};

/// Linux `sys/syscall.h` constants for the current target.
#[cfg(target_arch = "x86")]
pub type SYS_TARGET = SYS_X86;
/// Linux `sys/syscall.h` constants for the current target.
#[cfg(target_arch = "x86_64")]
pub type SYS_TARGET = SYS_X86_64;
/// Linux `sys/syscall.h` constants for the current target.
#[cfg(target_arch = "arm")]
pub type SYS_TARGET = SYS_ARM;
/// Linux `sys/syscall.h` constants for the current target.
#[cfg(target_arch = "aarch64")]
pub type SYS_TARGET = SYS_AARCH64;
/// Linux `sys/syscall.h` constants for the current target.
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub type SYS_TARGET = SYS_RISCV;
