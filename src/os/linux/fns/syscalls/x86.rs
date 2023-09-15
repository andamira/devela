// devela::io::linux::fns::syscalls::x86
//
//!
//

use crate::os::linux::{SysTimespec, SYS_X86 as SYS};
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
            "int 0x80",
            in("eax") SYS::EXIT,
            in("ebx") status,
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
        "int 0x80",
        inlateout("eax") SYS::READ => r0,
        in("ebx") fd,
        in("ecx") buf,
        in("edx") count,
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
        "int 0x80",
        inlateout("eax") SYS::WRITE => r0,
        in("ebx") fd,
        in("ecx") buf,
        in("edx") count,
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
        "int 0x80",
        inlateout("eax") SYS::NANOSLEEP => r0,
        in("ebx") req,
        in("ecx") rem,
        lateout("edx") _,
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
        "int 0x80",
        inlateout("eax") SYS::IOCTL => r0,
        in("ebx") fd,
        in("ecx") request,
        in("edx") argp,
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
        "int 0x80",
        inlateout("eax") SYS::GETRANDOM => r0,
        in("ebx") buffer,
        in("ecx") size,
        in("edx") flags,
        options(nostack, preserves_flags)
    );
    r0
}
