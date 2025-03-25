// devela::sys::os::linux::syscall::consts
//
//! Defines the syscall numbers for each target.
//
// - https://marcin.juszkiewicz.com.pl/download/tables/syscalls.html

#![allow(non_camel_case_types)]

crate::CONST! { SYS_DOC = "Linux `sys/syscall.h` constants for the current compilation target."; }

crate::items! {
    #[cfg(target_arch = "aarch64")] mod aarch64;
    #[cfg(target_arch = "arm")] mod arm;
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))] mod riscv;
    #[cfg(target_arch = "x86")] mod x86;
    #[cfg(target_arch = "x86_64")] mod x86_64;
}

#[doc = SYS_DOC!()]
#[cfg(target_arch = "aarch64")]
pub type LINUX_SYS = aarch64::LINUX_SYS_AARCH64;
#[doc = SYS_DOC!()]
#[cfg(target_arch = "arm")]
pub type LINUX_SYS = arm::LINUX_SYS_ARM;
#[doc = SYS_DOC!()]
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub type LINUX_SYS = riscv::LINUX_SYS_RISCV;
#[doc = SYS_DOC!()]
#[cfg(target_arch = "x86")]
pub type LINUX_SYS = x86::LINUX_SYS_X86;
#[doc = SYS_DOC!()]
#[cfg(target_arch = "x86_64")]
pub type LINUX_SYS = x86_64::LINUX_SYS_X86_64;
