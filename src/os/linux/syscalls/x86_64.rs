// devela::io::linux::x86_64
//
//!
//

use super::SysTimeSpec;
use core::{
    arch::asm,
    ffi::{c_int, c_ulong},
};

#[doc = include_str!("./doc/Sys_exit.md")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub unsafe fn sys_exit(status: c_int) -> ! {
    const SYS_EXIT: isize = 60;
    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_EXIT,
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
    const SYS_READ: isize = 0;
    let r0;
    asm!(
        "syscall",
        inlateout("rax") SYS_READ => r0,
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
    const SYS_WRITE: isize = 1;
    let r0;
    asm!(
        "syscall",
        inlateout("rax") SYS_WRITE => r0,
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
    const SYS_NANOSLEEP: isize = 35;
    let r0;
    asm!(
        "syscall",
        inlateout("rax") SYS_NANOSLEEP => r0,
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
    const SYS_IOCTL: isize = 16;
    let r0;
    asm!(
        "syscall",
        inlateout("rax") SYS_IOCTL => r0,
        in("rdi") fd,
        in("rsi") request,
        in("rdx") argp,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}
