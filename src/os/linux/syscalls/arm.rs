// devela::io::linux::arm
//
//!
//
// https://github.com/rust-lang/rust/issues/85056 can't use r7 register

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
    const SYS_EXIT: isize = 1;
    unsafe {
        asm!(
            "swi 0",
            in("r8") SYS_EXIT,
            in("r0") status,
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
    const SYS_READ: isize = 3;
    let r0;
    asm!(
        "swi 0",
        inlateout("r8") SYS_READ => r0,
        in("r0") fd,
        in("r1") buf,
        in("r2") count,
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
    const SYS_WRITE: isize = 4;
    let r0;
    asm!(
        "swi 0",
        inlateout("r8") SYS_WRITE => r0,
        in("r0") fd,
        in("r1") buf,
        in("r2") count,
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
        inlateout("r8") SYS_NANOSLEEP => r0,
        in("r0") req,
        in("r1") rem,
        lateout("r2") _,
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
    const SYS_IOCTL: isize = 54;
    let r0;
    asm!(
        "swi 0",
        inlateout("r8") SYS_IOCTL => r0,
        in("r0") fd,
        in("r1") request,
        in("r2") argp,
        options(nostack, preserves_flags)
    );
    r0
}
