// devela::sys::os::linux::process::signal::restorer
//
//! Configures the global assembly for target-architecture-specific signal restorers.
//!
//! This module defines the low-level assembly routines required for restoring the execution state
//! after a signal handler is invoked. These routines are architecture-specific and are used to
//! properly return from signal handling to the interrupted program context.
//!
//! The implementation is inspired by
//! [musl libc](https://git.musl-libc.org/cgit/musl/tree/src/signal).
//!
//
// source: [musl](https://git.musl-libc.org/cgit/musl/tree/src/signal)

use crate::{LINUX_SYS as SYS, global_asm};

// https://git.musl-libc.org/cgit/musl/tree/src/signal/x86_64/restore.s
#[cfg(target_arch = "x86_64")]
global_asm! {
    "nop",
    ".global __devela_linux_restore_rt",
    ".hidden __devela_linux_restore_rt",
    ".type __devela_linux_restore_rt,@function",
    "__devela_linux_restore_rt:",
        "mov rax, {RT_SIGRETURN}",
        "syscall",
    RT_SIGRETURN = const SYS::RT_SIGRETURN,
}

// https://git.musl-libc.org/cgit/musl/blame/src/signal/i386/restore.s
#[cfg(target_arch = "x86")]
global_asm!(
    ".global __devela_linux_restore_rt",
    ".hidden __devela_linux_restore_rt",
    ".type __devela_linux_restore_rt,@function",
    "__devela_linux_restore_rt:",
        "mov eax, {RT_SIGRETURN}",
        "int 0x80",
    RT_SIGRETURN = const SYS::RT_SIGRETURN,
);

// https://git.musl-libc.org/cgit/musl/tree/src/signal/arm/restore.s
#[cfg(target_arch = "arm")]
global_asm!(
    ".syntax unified", // Use unified syntax for ARM/Thumb
    ".global __devela_linux_restore_rt",
    ".hidden __devela_linux_restore_rt",
    ".type __devela_linux_restore_rt,%function",
    "__devela_linux_restore_rt:",
        "mov r7, #{RT_SIGRETURN}",
        "swi 0x0",
    RT_SIGRETURN = const SYS::RT_SIGRETURN,
);

// https://git.musl-libc.org/cgit/musl/tree/src/signal/aarch64/restore.s
#[cfg(target_arch = "aarch64")]
global_asm!(
    ".global __devela_linux_restore_rt",
    ".hidden __devela_linux_restore_rt",
    ".type __devela_linux_restore_rt,%function",
    "__devela_linux_restore_rt:",
        "mov x8, #{RT_SIGRETURN}",
        "svc 0",
    RT_SIGRETURN = const SYS::RT_SIGRETURN,
);

// https://git.musl-libc.org/cgit/musl/tree/src/signal/riscv32/restore.s
// https://git.musl-libc.org/cgit/musl/tree/src/signal/riscv64/restore.s
#[cfg(any_target_arch_riscv)]
global_asm!(
    ".global __devela_linux_restore_rt",
    ".type __devela_linux_restore_rt, %function",
    "__devela_linux_restore_rt:",
        "li a7, {RT_SIGRETURN}",
        "ecall",
    RT_SIGRETURN = const SYS::RT_SIGRETURN,
);
