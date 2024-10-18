// devela::io::linux::fns::syscalls::aarch64
//
//!
//

use crate::sys::os::linux::{LinuxSigaction, LinuxTimespec, LINUX_SYS_AARCH64 as SYS};
use core::{
    arch::asm,
    ffi::{c_int, c_uint, c_ulong},
};

#[doc = include_str!("./doc/Sys_exit.md")]
pub unsafe fn linux_sys_exit(status: c_int) -> ! {
    unsafe {
        asm!(
            "mov x8, {EXIT}",
            "svc 0",
            EXIT = const SYS::EXIT,
            in("x8") SYS::EXIT,
            in("x0") status,
            options(noreturn)
        );
    }
}

#[doc = include_str!("./doc/Sys_read.md")]
#[must_use]
pub unsafe fn linux_sys_read(fd: c_int, buf: *mut u8, count: usize) -> isize {
    let result;
    unsafe {
        asm!(
            "mov x8, {READ}",
            "svc 0",
            READ = const SYS::READ,
            in("x0") fd,
            in("x1") buf,
            in("x2") count,
            lateout("x8") result,
            options(nostack)
        );
    }
    result
}

#[doc = include_str!("./doc/Sys_write.md")]
#[must_use]
pub unsafe fn linux_sys_write(fd: c_int, buf: *const u8, count: usize) -> isize {
    let result;
    unsafe {
        asm!(
            "mov x8, {WRITE}",
            "svc 0",
            WRITE = const SYS::WRITE,
            in("x0") fd,
            in("x1") buf,
            in("x2") count,
            lateout("x8") result,
            options(nostack)
        );
    }
    result
}

#[doc = include_str!("./doc/Sys_nanosleep.md")]
#[must_use]
pub unsafe fn linux_sys_nanosleep(req: *const LinuxTimespec, rem: *mut LinuxTimespec) -> isize {
    let result;
    unsafe {
        asm!(
            "mov x8, {NANOSLEEP}",
            "svc 0",
            NANOSLEEP = const SYS::NANOSLEEP,
            in("x0") req,
            in("x1") rem,
            lateout("x2") _,
            lateout("x8") result,
            options(nostack)
        );
    }
    result
}

#[doc = include_str!("./doc/Sys_ioctl.md")]
#[must_use]
pub unsafe fn linux_sys_ioctl(fd: c_int, request: c_ulong, argp: *mut u8) -> isize {
    let result;
    unsafe {
        asm!(
            "mov x8, {IOCTL}",
            "svc 0",
            IOCTL = const SYS::IOCTL,
            in("x0") fd,
            in("x1") request,
            in("x2") argp,
            lateout("x8") result,
            options(nostack)
        );
    }
    result
}

#[doc = include_str!("./doc/Sys_getrandom.md")]
#[must_use]
pub unsafe fn linux_sys_getrandom(buffer: *mut u8, size: usize, flags: c_uint) -> isize {
    let result;
    unsafe {
        asm!(
            "mov x8, {GETRANDOM}",
            "svc 0",
            GETRANDOM = const SYS::GETRANDOM,
            in("x0") buffer,
            in("x1") size,
            in("x2") flags,
            lateout("x8") result,
            // lateout("x16") _,
            options(nostack)
        );
    }
    result
}

#[doc = include_str!("./doc/Sys_getpid.md")]
#[must_use]
pub unsafe fn linux_sys_getpid() -> i32 {
    let result: isize;
    unsafe {
        asm!(
            "mov x8, {GETPID}",
            "svc 0",
            GETPID = const SYS::GETPID,
            lateout("x8") result,
            options(nostack)
        );
    }
    result as i32
}

#[doc = include_str!("./doc/Sys_rt_sigaction.md")]
#[must_use]
pub unsafe fn linux_sys_rt_sigaction(
    sig: c_int,
    act: *const LinuxSigaction,
    oact: *mut LinuxSigaction,
    sigsetsize: usize,
) -> isize {
    let result;
    unsafe {
        asm!(
            "mov x8, {RT_SIGACTION}",
            "svc 0",
            RT_SIGACTION = const SYS::RT_SIGACTION,
            in("x0") sig,
            in("x1") act,
            in("x2") oact,
            in("x3") sigsetsize,
            lateout("x8") result,
            options(nostack)
        );
    }
    result
}
