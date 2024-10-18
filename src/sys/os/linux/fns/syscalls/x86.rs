// devela::io::linux::fns::syscalls::x86
//
//!
//

use crate::sys::os::linux::{LinuxSigaction, LinuxTimespec, LINUX_SYS_X86 as SYS};
use core::{
    arch::asm,
    ffi::{c_int, c_uint, c_ulong},
};

#[doc = include_str!("./doc/Sys_exit.md")]
pub unsafe fn linux_sys_exit(status: c_int) -> ! {
    unsafe {
        asm!(
            "mov eax, {EXIT}",
            "int 0x80",
            EXIT = const SYS::EXIT,
            in("ebx") status,
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
            "mov eax, {READ}",
            "int 0x80",
            READ = const SYS::READ,
            in("ebx") fd,
            in("ecx") buf,
            in("edx") count,
            lateout("eax") result,
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
            "mov eax, {WRITE}",
            "int 0x80",
            WRITE = const SYS::WRITE,
            in("ebx") fd,
            in("ecx") buf,
            in("edx") count,
            lateout("eax") result,
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
            "mov eax, {NANOSLEEP}",
            "int 0x80",
            NANOSLEEP = const SYS::NANOSLEEP,
            in("ebx") req,
            in("ecx") rem,
            lateout("edx") _,
            lateout("eax") result,
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
            "mov eax, {IOCTL}",
            "int 0x80",
            IOCTL = const SYS::IOCTL,
            in("ebx") fd,
            in("ecx") request,
            in("edx") argp,
            lateout("eax") result,
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
            "mov eax, {GETRANDOM}",
            "int 0x80",
            GETRANDOM = const SYS::GETRANDOM,
            in("ebx") buffer,
            in("ecx") size,
            in("edx") flags,
            lateout("eax") result,
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
            "mov eax, {GETPID}",
            "int0x80",
            GETPID = const SYS::GETPID,
            lateout("eax") result,
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
            "mov eax, {RT_SIGACTION}",
            "int 0x80",
            RT_SIGACTION = const SYS::RT_SIGACTION,
            in("ebx") sig,
            in("ecx") act,
            in("edx") oact,
            in("edi") sigsetsize,
            lateout("eax") result,
            options(nostack)
        );
    }
    result
}
