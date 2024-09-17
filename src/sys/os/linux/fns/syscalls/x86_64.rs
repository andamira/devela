// devela::io::linux::fns::syscalls:x86_64
//
//!
//
// - https://arm64.syscall.sh/

use crate::sys::os::linux::{LinuxSigaction, LinuxTimespec, LINUX_SYS_X86_64 as SYS};
use core::{
    arch::asm,
    ffi::{c_int, c_uint, c_ulong},
};

#[doc = include_str!("./doc/Sys_exit.md")]
pub unsafe fn linux_sys_exit(status: c_int) -> ! {
    unsafe {
        asm!(
            "syscall",
            in("rax") SYS::EXIT,
            in("rdi") status,
            options(noreturn)
        );
    }
}

#[doc = include_str!("./doc/Sys_read.md")]
#[must_use]
pub unsafe fn linux_sys_read(fd: c_int, buf: *mut u8, count: usize) -> isize {
    let r0;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") SYS::READ => r0,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
    }
    r0
}

#[doc = include_str!("./doc/Sys_write.md")]
#[must_use]
pub unsafe fn linux_sys_write(fd: c_int, buf: *const u8, count: usize) -> isize {
    let r0;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") SYS::WRITE => r0,
            in("rdi") fd,
            in("rsi") buf,
            in("rdx") count,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
    }
    r0
}

#[doc = include_str!("./doc/Sys_nanosleep.md")]
#[must_use]
pub unsafe fn linux_sys_nanosleep(req: *const LinuxTimespec, rem: *mut LinuxTimespec) -> isize {
    let r0;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") SYS::NANOSLEEP => r0,
            in("rdi") req,
            in("rsi") rem,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
    }
    r0
}

#[doc = include_str!("./doc/Sys_ioctl.md")]
#[must_use]
pub unsafe fn linux_sys_ioctl(fd: c_int, request: c_ulong, argp: *mut u8) -> isize {
    let r0;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") SYS::IOCTL => r0,
            in("rdi") fd,
            in("rsi") request,
            in("rdx") argp,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
    }
    r0
}

#[doc = include_str!("./doc/Sys_getrandom.md")]
#[must_use]
pub unsafe fn linux_sys_getrandom(buffer: *mut u8, size: usize, flags: c_uint) -> isize {
    let r0;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") SYS::GETRANDOM => r0,
            in("rdi") buffer,
            in("rsi") size,
            in("rdx") flags,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
    }
    r0
}

#[doc = include_str!("./doc/Sys_getpid.md")]
#[must_use]
pub unsafe fn linux_sys_getpid() -> i32 {
    let r0: isize;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") SYS::GETPID => r0,
            options(nostack, preserves_flags)
        );
    }
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
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") SYS::RT_SIGACTION => r0,
            in("rdi") sig,
            in("rsi") act,
            in("rdx") oact,
            in("r10") sigsetsize,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack, preserves_flags)
        );
    }
    r0
}