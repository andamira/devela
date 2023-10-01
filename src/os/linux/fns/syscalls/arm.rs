// devela::io::linux::fns::syscalls::arm
//
//!
//
// https://github.com/rust-lang/rust/issues/85056 can't use r7 register

use crate::os::linux::{LinuxSigaction, LinuxTimespec, LINUX_SYS_ARM as SYS};
use core::{
    arch::asm,
    ffi::{c_int, c_uint, c_ulong},
};

#[doc = include_str!("./doc/Sys_exit.md")]
pub unsafe fn linux_sys_exit(status: c_int) -> ! {
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
#[must_use]
pub unsafe fn linux_sys_read(fd: c_int, buf: *mut u8, count: usize) -> isize {
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
#[must_use]
pub unsafe fn linux_sys_write(fd: c_int, buf: *const u8, count: usize) -> isize {
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
#[must_use]
pub unsafe fn linux_sys_nanosleep(req: *const LinuxTimespec, rem: *mut LinuxTimespec) -> isize {
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
#[must_use]
pub unsafe fn linux_sys_ioctl(fd: c_int, request: c_ulong, argp: *mut u8) -> isize {
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
#[must_use]
pub unsafe fn linux_sys_getrandom(buffer: *mut u8, size: usize, flags: c_uint) -> isize {
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

#[doc = include_str!("./doc/Sys_getpid.md")]
#[must_use]
pub unsafe fn linux_sys_getpid() -> i32 {
    let r0: isize;
    asm!(
        "swi 0",
        inlateout("r8") SYS::GETPID => r0,
        options(nostack, preserves_flags)
    );
    r0 as i32
}

#[doc = include_str!("./doc/Sys_rt_sigaction.md")]
#[must_use]
pub unsafe fn linux_sys_rt_sigaction(
    sig: c_int,
    act: *const LinuxSigaction,
    oact: *mut LinuxSigaction,
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
