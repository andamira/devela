// devela::os::linux::syscall::consts
//
//! Defines the syscall numbers for each target.
//
// - https://marcin.juszkiewicz.com.pl/download/tables/syscalls.html

#![allow(dead_code)]

mod aarch64;
mod arm;
mod riscv;
mod x86;
mod x86_64;
pub use {aarch64::SYS_AARCH64, arm::SYS_ARM, riscv::SYS_RISCV, x86::SYS_X86, x86_64::SYS_X86_64};
