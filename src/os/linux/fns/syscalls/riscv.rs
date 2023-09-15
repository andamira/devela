// devela::io::linux::fns::syscalls::riscv
//
//! Both for riscv32 and riscv64
//

use crate::os::linux::{SysTimespec, SYS_RISCV as SYS};
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
            "ecall",
            in("a7") SYS::EXIT,
            in("a0") status,
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
        "ecall",
        inlateout("a7") SYS::READ => r0,
        in("a0") fd,
        in("a1") buf,
        in("a2") count,
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
        "ecall",
        inlateout("a7") SYS::WRITE => r0,
        in("a0") fd,
        in("a1") buf,
        in("a2") count,
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
        "ecall",
        inlateout("a7") SYS::NANOSLEEP => r0,
        in("a0") fd,
        in("a1") buf,
        lateout("a2") _,
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
        "ecall",
        inlateout("a7") SYS::IOCTL => r0,
        in("a0") fd,
        in("a1") request,
        in("a2") argp,
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
        "ecall",
        inlateout("a7") SYS::GETRANDOM => r0,
        in("a0") buffer,
        in("a1") size,
        in("a2") flags,
        options(nostack, preserves_flags)
    );
    r0
}
