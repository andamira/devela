// devela::io::linux::fns::syscalls:x86_64
//
//!
//
// - https://arm64.syscall.sh/

use crate::os::linux::{SysTimeSpec, SYS_X86_64 as SYS};
use core::{
    arch::asm,
    ffi::{c_int, c_uint, c_ulong},
};

#[doc = include_str!("./doc/Sys_exit.md")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub unsafe fn sys_exit(status: c_int) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") SYS::EXIT,
            in("rdi") status,
            options(noreturn)
        );
    }
}

#[doc = include_str!("./doc/Sys_read.md")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub unsafe fn sys_read(fd: c_int, buf: *mut u8, count: usize) -> isize {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") SYS::READ => r0,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") count,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

#[doc = include_str!("./doc/Sys_write.md")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub unsafe fn sys_write(fd: c_int, buf: *const u8, count: usize) -> isize {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") SYS::WRITE => r0,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") count,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

#[doc = include_str!("./doc/Sys_nanosleep.md")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub unsafe fn sys_nanosleep(req: *const SysTimeSpec, rem: *mut SysTimeSpec) -> isize {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") SYS::NANOSLEEP => r0,
        in("rdi") req,
        in("rsi") rem,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

#[doc = include_str!("./doc/Sys_ioctl.md")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub unsafe fn sys_ioctl(fd: c_int, request: c_ulong, argp: *mut u8) -> isize {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") SYS::IOCTL => r0,
        in("rdi") fd,
        in("rsi") request,
        in("rdx") argp,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

#[doc = include_str!("./doc/Sys_getrandom.md")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub unsafe fn sys_getrandom(buffer: *mut u8, size: usize, flags: c_uint) -> isize {
    let r0;
    asm!(
        "syscall",
        inlateout("rax") SYS::GETRANDOM => r0,
        in("rdi") buffer,
        in("rsi") size,
        in("rdx") flags,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}
