// devela::io::linux::fns::syscalls::aarch64
//
//!
//

use crate::os::linux::{LinuxSigaction, LinuxTimespec, LINUX_SYS_AARCH64 as SYS};
use core::{
    arch::asm,
    ffi::{c_int, c_uint, c_ulong},
};

#[doc = include_str!("./doc/Sys_exit.md")]
pub unsafe fn linux_sys_exit(status: c_int) -> ! {
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
pub unsafe fn linux_sys_read(fd: c_int, buf: *mut u8, count: usize) -> isize {
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
pub unsafe fn linux_sys_write(fd: c_int, buf: *const u8, count: usize) -> isize {
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
pub unsafe fn linux_sys_nanosleep(req: *const LinuxTimespec, rem: *mut LinuxTimespec) -> isize {
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
pub unsafe fn linux_sys_ioctl(fd: c_int, request: c_ulong, argp: *mut u8) -> isize {
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
pub unsafe fn linux_sys_getrandom(buffer: *mut u8, size: usize, flags: c_uint) -> isize {
    let r0;
    asm!(
        "svc 0",
        inlateout("x8") SYS::GETRANDOM => r0,
        in("x0") buffer,
        in("x1") size,
        in("x2") flags,
        // lateout("x16") _,
        options(nostack, preserves_flags)
    );
    r0
}

#[doc = include_str!("./doc/Sys_getpid.md")]
pub unsafe fn linux_sys_getpid() -> i32 {
    let r0: isize;
    asm!(
        "svc 0",
        inlateout("x8") SYS::GETPID => r0,
        options(nostack, preserves_flags)
    );
    r0 as i32
}

#[doc = include_str!("./doc/Sys_rt_sigaction.md")]
pub unsafe fn linux_sys_rt_sigaction(
    sig: c_int,
    act: *const LinuxSigaction,
    oact: *mut LinuxSigaction,
    sigsetsize: usize,
) -> isize {
    let r0;
    asm!(
        "svc 0",
        inlateout("x8") SYS::GETRANDOM => r0,
        in("x0") sig,
        in("x1") act,
        in("x2") oact,
        in("x3") sigsetsize,
        options(nostack, preserves_flags)
    );
    r0
}
