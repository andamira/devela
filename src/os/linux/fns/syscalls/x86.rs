// devela::io::linux::fns::syscalls::x86
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
    const SYS_EXIT: isize = 1;
    unsafe {
        asm!(
            "int 0x80",
            in("eax") SYS_EXIT,
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
    const SYS_READ: isize = 3;
    let r0;
    asm!(
        "int 0x80",
        inlateout("eax") SYS_READ => r0,
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
    const SYS_WRITE: isize = 4;
    let r0;
    asm!(
        "int 0x80",
        inlateout("eax") SYS_WRITE => r0,
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
pub unsafe fn sys_nanosleep(req: *const SysTimeSpec, rem: *mut SysTimeSpec) -> isize {
    const SYS_NANOSLEEP: isize = 162;
    let r0;
    asm!(
        "int 0x80",
        inlateout("eax") SYS_NANOSLEEP => r0,
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
    const SYS_IOCTL: isize = 54;
    let r0;
    asm!(
        "int 0x80",
        inlateout("eax") SYS_IOCTL => r0,
        in("ebx") fd,
        in("ecx") request,
        in("edx") argp,
        options(nostack, preserves_flags)
    );
    r0
}
