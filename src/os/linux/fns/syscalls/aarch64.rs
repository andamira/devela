// devela::io::linux::fns::syscalls::aarch64
//
//!
//

use crate::os::linux::{SysTimeSpec, SYS_AARCH64 as SYS};
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
            "svc 0",
            in("x8") SYS::EXIT,
            in("x0") status,
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
        "svc 0",
        inlateout("x8") SYS::READ => r0,
        in("x0") fd,
        in("x1") buf,
        in("x2") count,
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
        "svc 0",
        inlateout("x8") SYS::WRITE => r0,
        in("x0") fd,
        in("x1") buf,
        in("x2") count,
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
        "svc 0",
        inlateout("x8") SYS::NANOSLEEP => r0,
        in("x0") req,
        in("x1") rem,
        lateout("x2") _,
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
        "svc 0",
        inlateout("x8") SYS::IOCTL => r0,
        in("x0") fd,
        in("x1") request,
        in("x2") argp,
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
        "svc 0x0",
        inlateout("x8") SYS::GETRANDOM => r0,
        in("x0") buffer,
        in("x1") size,
        in("x2") flags,
        // lateout("x16") _,
        options(nostack, preserves_flags)
    );
    r0
}
