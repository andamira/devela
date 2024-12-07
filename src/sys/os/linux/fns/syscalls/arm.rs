// devela::io::linux::fns::syscalls::arm
//
//!
//
// https://github.com/rust-lang/rust/issues/85056 can't use r7 register

use crate::{asm, c_int, c_uint, c_ulong, LinuxSigaction, LinuxTimespec, LINUX_SYS_ARM as SYS};

#[doc = include_str!("./doc/Sys_exit.md")]
pub unsafe fn linux_sys_exit(status: c_int) -> ! {
    unsafe {
        asm!(
            "mov rax, {EXIT}",
            "swi 0",
            EXIT = const SYS::EXIT,
            in("r0") status,
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
            "mov rax, {READ}",
            "swi 0",
            READ = const SYS::READ,
            in("r0") fd,
            in("r1") buf,
            in("r2") count,
            lateout("r7") result,
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
            "mov rax, {WRITE}",
            "swi 0",
            WRITE = const SYS::WRITE,
            in("r0") fd,
            in("r1") buf,
            in("r2") count,
            lateout("r7") result,
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
            "mov rax, {NANOSLEEP}",
            "swi 0",
            NANOSLEEP = const SYS::NANOSLEEP,
            in("r0") req,
            in("r1") rem,
            lateout("r2") _,
            lateout("r7") result,
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
            "mov rax, {IOCTL}",
            "swi 0",
            IOCTL = const SYS::IOCTL,
            in("r0") fd,
            in("r1") request,
            in("r2") argp,
            lateout("r7") result,
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
            "mov rax, {GETRANDOM}",
            "swi 0",
            GETRANDOM = const SYS::GETRANDOM,
            in("r0") buffer,
            in("r1") size,
            in("r2") flags,
            lateout("r7") result,
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
            "mov rax, {GETPID}",
            "swi 0",
            GETPID = const SYS::GETPID,
            lateout("r7") result,
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
            "mov rax, {RT_SIGACTION}",
            "swi 0",
            RT_SIGACTION = const SYS::RT_SIGACTION,
            in("r0") sig,
            in("r1") act,
            in("r2") oact,
            in("r3") sigsetsize,
            lateout("r7") result,
            options(nostack)
        );
    }
    result
}
