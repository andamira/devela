// devela::io::linux::fns::syscalls::arm
//
//!
//
// https://github.com/rust-lang/rust/issues/85056 can't use r7 register

use crate::os::linux::{SysTimespec, SYS_ARM as SYS};
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
            "swi 0",
            in("r8") SYS::EXIT,
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
    let r0;
    asm!(
        "swi 0",
        inlateout("r8") SYS::READ => r0,
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
    let r0;
    asm!(
        "swi 0",
        inlateout("r8") SYS::WRITE => r0,
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
pub unsafe fn sys_nanosleep(req: *const SysTimespec, rem: *mut SysTimespec) -> isize {
    let r0;
    asm!(
        "swi 0",
        inlateout("r8") SYS::NANOSLEEP => r0,
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
    let r0;
    asm!(
        "swi 0",
        inlateout("r8") SYS::IOCTL => r0,
        in("r0") fd,
        in("r1") request,
        in("r2") argp,
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
        "swi 0",
        inlateout("r8") SYS::GETRANDOM => r0,
        in("r0") buffer,
        in("r1") size,
        in("r2") flags,
        options(nostack, preserves_flags)
    );
    r0
}

#[doc = include_str!("./doc/Sys_rt_sigaction.md")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub unsafe fn sys_rt_sigaction(
    sig: c_int,
    act: *const SysSigaction,
    oact: *mut SysSigaction,
    sigsetsize: usize,
) -> isize {
    let r0;
    asm!(
        "swi 0",
        inlateout("r8") SYS::RT_SIGACTION => r0,
        in("r0") sig,
        in("r1") act,
        in("r2") oact,
        in("r3") sigsetsize,
        options(nostack, preserves_flags)
    );
    r0
}
