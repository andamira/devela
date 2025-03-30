// devela::sys::os::linux::restorer
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
    ".global __devela_linux_restore_rt",
    ".hidden __devela_linux_restore_rt",
    ".type __devela_linux_restore_rt,@function",
    "__devela_linux_restore_rt:",
        "mov rax, {RT_SIGRETURN}",
        "syscall",
    // ".size __devela_linux_restore_rt,.-__devela_linux_restore_rt",
    RT_SIGRETURN = const SYS::RT_SIGRETURN,
}

// https://git.musl-libc.org/cgit/musl/blame/src/signal/i386/restore.s
#[cfg(target_arch = "x86")]
global_asm!(
    ".global __devela_linux_restore",
    ".hidden __devela_linux_restore",
    ".type __devela_linux_restore,@function",
    "__devela_linux_restore:",
        "popl eax",
        "movl eax, {SIGRETURN}",
        "int 0x80",
    // ".size __devela_linux_restore,.-__devela_linux_restore",

    ".global __devela_linux_restore_rt",
    ".hidden __devela_linux_restore_rt",
    ".type __devela_linux_restore_rt,@function",
    "__devela_linux_restore_rt:",
        "movl eax, {RT_SIGRETURN}",
        "int 0x80",
    // ".size __devela_linux_restore_rt,.-__devela_linux_restore_rt",
    SIGRETURN = const SYS::SIGRETURN,
    RT_SIGRETURN = const SYS::RT_SIGRETURN,
);

// https://git.musl-libc.org/cgit/musl/tree/src/signal/arm/restore.s
#[cfg(target_arch = "arm")]
global_asm!(
    ".syntax unified", // Use unified syntax for ARM/Thumb

    ".global __devela_linux_restore",
    ".hidden __devela_linux_restore",
    ".type __devela_linux_restore,%function",
    "__devela_linux_restore:",
        "mov r7, {SIGRETURN}",
        "swi 0x0",
    // ".size __devela_linux_restore,.-__devela_linux_restore",

    ".global __devela_linux_restore_rt",
    ".hidden __devela_linux_restore_rt",
    ".type __devela_linux_restore_rt,%function",
    "__devela_linux_restore_rt:",
        "mov r7, {RT_SIGRETURN}",
        "swi 0x0",
    // ".size __devela_linux_restore_rt,.-__devela_linux_restore_rt",
    SIGRETURN = const SYS::SIGRETURN,
    RT_SIGRETURN = const SYS::RT_SIGRETURN,
);

// https://git.musl-libc.org/cgit/musl/tree/src/signal/aarch64/restore.s
#[cfg(target_arch = "aarch64")]
global_asm!(
    ".global __devela_linux_restore",
    ".hidden __devela_linux_restore",
    ".type __devela_linux_restore,%function",
    "__devela_linux_restore:",
        // no implementation provided
    ".global __devela_linux_restore_rt",
    ".hidden __devela_linux_restore_rt",
    ".type __devela_linux_restore_rt,%function",
    "__devela_linux_restore_rt:",
        "mov x8, {RT_SIGRETURN}",
        "svc 0",
    RT_SIGRETURN = const SYS::RT_SIGRETURN,
);

// https://git.musl-libc.org/cgit/musl/tree/src/signal/riscv32/restore.s
// https://git.musl-libc.org/cgit/musl/tree/src/signal/riscv64/restore.s
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
global_asm!(
    ".global __devela_linux_restore_rt",
    ".type __devela_linux_restore_rt, %function",
    "__devela_linux_restore_rt:",
        "li a7, {RT_SIGRETURN}",
        "ecall",
    RT_SIGRETURN = const SYS::RT_SIGRETURN,
);
