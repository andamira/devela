// devela::io::linux::fns::syscalls::riscv
//
//! Both for riscv32 and riscv64
//

use crate::sys::os::linux::{LinuxSigaction, LinuxTimespec, LINUX_SYS_RISCV as SYS};
use core::{
    arch::asm,
    ffi::{c_int, c_uint, c_ulong},
};

#[doc = include_str!("./doc/Sys_exit.md")]
pub unsafe fn linux_sys_exit(status: c_int) -> ! {
    unsafe {
        asm!(
            "li a7, {EXIT}",
            "ecall",
            EXIT = const SYS::EXIT,
            in("a0") status,
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
            "li a7, {READ}",
            "ecall",
            READ = const SYS::READ,
            in("a0") fd,
            in("a1") buf,
            in("a2") count,
            lateout("a7") result,
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
            "li a7, {WRITE}",
            "ecall",
            WRITE = const SYS::WRITE,
            in("a0") fd,
            in("a1") buf,
            in("a2") count,
            lateout("a7") result,
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
            "li a7, {NANOSLEEP}",
            "ecall",
            NANOSLEEP = const SYS::NANOSLEEP,
            in("a0") req,
            in("a1") rem,
            lateout("a7") result,
            lateout("a2") _,
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
            "li a7, {IOCTL}",
            "ecall",
            IOCTL = const SYS::IOCTL,
            in("a0") fd,
            in("a1") request,
            in("a2") argp,
            lateout("a7") result,
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
            "li a7, {GETRANDOM}",
            "ecall",
            GETRANDOM = const SYS::GETRANDOM,
            in("a0") buffer,
            in("a1") size,
            in("a2") flags,
            lateout("a7") result,
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
            "li a7, {GETPID}",
            "ecall",
            GETPID = const SYS::GETPID,
            lateout("a7") result,
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
            "li a7, {RT_SIGACTION}",
            "ecall",
            RT_SIGACTION = const SYS::RT_SIGACTION,
            in("a0") sig,
            in("a1") act,
            in("a2") oact,
            in("a3") sigsetsize,
            lateout("a7") result,
            options(nostack)
        );
    }
    result
}
