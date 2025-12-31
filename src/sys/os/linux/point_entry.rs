// devela::sys::os::linux::point_entry
//
//! Configures the global assembly for target-architecture-specific program entry points.
//!
//! This module defines the low-level assembly routines that serve as the initial execution point
//! for the program. These routines are architecture-specific and handle:
//! 1. Setting up the minimal execution environment
//! 2. Transferring control to the Rust entry point
//! 3. Properly exiting with the program's return status
//!
//! The implementations match the system's calling conventions and use architecture-specific
//! syscall instructions for process termination.

#[doc = crate::_TAG_CODE!()]
#[doc = crate::_TAG_LINUX!()]
/// Defines the program entry point and main fn translation layer for Linux systems.
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

// WIP

// /// Configures the Linux program entry point with customizable initialization
// #[macro_export]
// macro_rules! linux_entry {
//     // Basic version with default behavior
//     () => {
//         $crate::linux_entry!(main => |_| {});
//     };
//
//     // // Version with custom main wrapper
//     // (main => $wrapper:expr) => {
//     //     $crate::__linux_internal_entry! {
//     //         pre_main: || {},
//     //         post_main: || {},
//     //         main_wrapper: $wrapper,
//     //         panic_handler: || loop {}
//     //     }
//     // };
//     //
//     // // Full version with all customization points
//     // (
//     //     pre_main: $pre:expr,
//     //     post_main: $post:expr,
//     //     main_wrapper: $wrapper:expr,
//     //     panic_handler: $panic:expr
//     // ) => {
//     //     $crate::__linux_internal_entry! {
//     //         pre_main: $pre,
//     //         post_main: $post,
//     //         main_wrapper: $wrapper,
//     //         panic_handler: $panic
//     //     }
//     // };
// }

// /// Internal implementation macro
// #[doc(hidden)]
// #[macro_export]
// macro_rules! __linux_internal_entry {.
//     ($($config:tt)*) => {
//         // Architecture-specific entry point
//         $crate::global_asm!(
//             ".global _start",
//             ".hidden _start",
//             ".type _start,@function",
//             "_start:",
//             $crate::__arch_entry_asm!($($config)*)
//         );
//
//         // Rust-level components
//         $crate::__linux_entry_components!($($config)*);
//     };
// }
// pub use __linux_internal_entry;
//
// /// Architecture-specific ASM generation.
// #[doc(hidden)]
// #[macro_export]
// macro_rules! __arch_entry_asm {
//     (
//         pre_main: $pre:expr,
//         post_main: $post:expr,
//         main_wrapper: $wrapper:expr,
//         panic_handler: $panic:expr
//     ) => {
//         #[cfg(target_arch = "x86_64")]
//         {
//             concat!(
//                 "call {PRE_MAIN}\n",
//                 "call {WRAPPER_MAIN}\n",
//                 "mov rdi, rax\n",
//                 "mov rax, {EXIT}\n",
//                 "syscall",
//                 // PRE_MAIN = sym $pre,
//                 // WRAPPER_MAIN = sym $wrapper,
//                 EXIT = const $crate::LINUX_SYS::EXIT
//             )
//         }
//         // Other architectures would follow similar patterns...
//     };
// }
// pub use __arch_entry_asm;
