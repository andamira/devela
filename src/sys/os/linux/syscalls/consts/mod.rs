// devela::sys::os::linux::syscall::consts
//
//! Defines the syscall numbers for each target.
//
// - https://marcin.juszkiewicz.com.pl/download/tables/syscalls.html

#![allow(non_camel_case_types, clippy::upper_case_acronyms)]

crate::CONST! { _DOC_SYS = "Linux `sys/syscall.h` constants for the current compilation target."; }

crate::items! {
    #[cfg(target_arch = "aarch64")] mod aarch64;
    #[cfg(target_arch = "arm")] mod arm;
    #[cfg(any_target_arch_riscv)] mod riscv;
    #[cfg(target_arch = "x86")] mod x86;
    #[cfg(target_arch = "x86_64")] mod x86_64;
}

#[doc = _DOC_SYS!()]
#[cfg(target_arch = "aarch64")]
pub type LINUX_SYS = aarch64::LINUX_SYS_AARCH64;
#[doc = _DOC_SYS!()]
#[cfg(target_arch = "arm")]
pub type LINUX_SYS = arm::LINUX_SYS_ARM;
#[doc = _DOC_SYS!()]
#[cfg(any_target_arch_riscv)]
pub type LINUX_SYS = riscv::LINUX_SYS_RISCV;
#[doc = _DOC_SYS!()]
#[cfg(target_arch = "x86")]
pub type LINUX_SYS = x86::LINUX_SYS_X86;
#[doc = _DOC_SYS!()]
#[cfg(target_arch = "x86_64")]
pub type LINUX_SYS = x86_64::LINUX_SYS_X86_64;

/// File offset type (always 64-bit in modern Linux, even on 32-bit architectures)
///
/// Note: While libc's `off_t` may be 32-bit on some 32-bit systems without LFS,
/// the Linux kernel ABI always uses 64-bit offsets for syscalls. This matches the
/// behavior of `-D_FILE_OFFSET_BITS=64` in userspace.
pub(crate) type LinuxOffset = i64;

#[allow(dead_code, reason = "only used in aarch64 for now")]
/// Special value for openat, newopenatat.
///
/// Makes pathname be interpreted relative to the current directory.
pub(crate) const AT_FDCWD: crate::c_int = -100;
