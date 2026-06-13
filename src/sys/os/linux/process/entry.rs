// devela/src/sys/os/linux/process/entry.rs
//
//! Configures the global assembly for target-architecture-specific program entry points.
//!
//! This module defines the low-level assembly routines that serve as the initial
//! execution point for the program. These routines are architecture-specific and handle:
//! 1. Setting up the minimal execution environment
//! 2. Transferring control to the Rust entry point
//! 3. Properly exiting with the program's return status
//!
//! The implementations match the system's calling conventions and use architecture-specific
//! syscall instructions for process termination.

#[doc = crate::_tags!(linux code)]
/// Defines the program entry point and main fn translation layer for Linux systems.
#[doc = crate::_doc_meta!{location("sys/os/linux")}]
///
/// Handles architecture-specific entry point setup and Rust-to-C ABI translation.
#[doc(hidden)]
#[macro_export]
macro_rules! linux_entry {
    (
    // The main() function will return LinuxResult, and get converted to LINUX_EXIT
    linux_result) => {
        $crate::linux_entry![main_body:
            if let Err(e) = main() { e.to_exit_code() } else { $crate::LINUX_EXIT::OK }
        ];
    };
    (
    // $main_body: the body of the entry point function that receives the result of main()
    main_body: $main_body:expr) => {
        #[inline(never)]
        #[unsafe(no_mangle)]
        pub extern "C" fn __main() -> i32 { $main_body }
        $crate::linux_entry!(_start: __main);
    };
    (
    //
    _start: $main:ident) => {
        #[cfg(target_arch = "x86_64")]
        $crate::global_asm!(
            ".global _start",
            ".hidden _start",
            ".type _start,@function",
            "_start:",
                "call {main}",
                "mov rdi, rax",
                "mov rax, {EXIT}",
                "syscall",
                main = sym $main,
                EXIT = const $crate::LINUX_SYS::EXIT,
        );
        #[cfg(target_arch = "x86")]
        $crate::global_asm!(
            ".global _start",
            ".hidden _start",
            ".type _start,@function",
            "_start:",
                "call {main}",
                "mov ebx, eax",
                "mov eax, {EXIT}",
                "int 0x80",
                main = sym $main,
                EXIT = const $crate::LINUX_SYS::EXIT,
        );
        #[cfg(target_arch = "arm")]
        $crate::global_asm!(
            ".global _start",
            ".hidden _start",
            ".type _start,%function",
            "_start:",
                "bl {main}",
                "mov r7, #{EXIT}",
                "swi #0",
                main = sym $main,
                EXIT = const $crate::LINUX_SYS::EXIT,
        );
        #[cfg(target_arch = "aarch64")]
        $crate::global_asm!(
            ".global _start",
            ".hidden _start",
            ".type _start,%function",
            "_start:",
                "mov x29, #0", // clear frame pointer
                "mov x30, #0", // clear link register
                "and sp, sp, #~15", // align stack
                "bl {main}",
                "mov x8, #{EXIT}",
                "svc #0",
                main = sym $main,
                EXIT = const $crate::LINUX_SYS::EXIT,
        );
        // NOTE: can't use `any_target_arch_riscv` because this is resolved in calling code
        #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
        $crate::global_asm!(
            ".global _start",
            ".hidden _start",
            ".type _start,@function",
            "_start:",
                "call {main}",
                "li a7, {EXIT}",
                "ecall",
                main = sym $main,
                EXIT = const $crate::LINUX_SYS::EXIT,
        );
    };
}
#[doc(inline)]
pub use linux_entry;
