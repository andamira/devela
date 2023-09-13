// devela::io::linux::aarch64
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
    const SYS_EXIT: isize = 93;
    unsafe {
        asm!(
            "svc 0",
            in("x8") SYS_EXIT,
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
    const SYS_READ: isize = 63;
    let r0;
    asm!(
        "svc 0",
        inlateout("x8") SYS_READ => r0,
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
    const SYS_WRITE: isize = 64;
    let r0;
    asm!(
        "svc 0",
        inlateout("x8") SYS_WRITE => r0,
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
    const SYS_NANOSLEEP: isize = 35;
    let r0;
    asm!(
        "svc 0",
        inlateout("x8") SYS_NANOSLEEP => r0,
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
    const SYS_IOCTL: isize = 16;
    let r0;
    asm!(
        "svc 0",
        inlateout("x8") SYS_IOCTL => r0,
        in("x0") fd,
        in("x1") request,
        in("x2") argp,
        options(nostack, preserves_flags)
    );
    r0
}
