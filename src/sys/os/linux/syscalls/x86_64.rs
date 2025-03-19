// devela::io::linux::syscalls:x86_64
//
//! Implements linux syscalls for x86_64.
//
//- https://x64.syscall.sh/

use super::shared_docs::*;
use crate::{
    asm, c_int, c_uint, c_ulong, Linux, LinuxSigaction, LinuxTimespec, LINUX_SYS_X86_64 as SYS,
};

/// # System calls.
impl Linux {
    #[doc = SYS_EXIT!()]
    pub unsafe fn sys_exit(status: c_int) -> ! {
        unsafe {
            asm!(
                "mov rax, {EXIT}",
                "syscall",
                EXIT = const SYS::EXIT,
                in("rdi") status,
                options(noreturn)
            );
        }
    }

    #[doc = SYS_READ!()]
    #[must_use]
    pub unsafe fn sys_read(fd: c_int, buf: *mut u8, count: usize) -> isize {
        let result;
        unsafe {
            asm!(
                "mov rax, {READ}",
                "syscall",
                READ = const SYS::READ,
                in("rdi") fd,
                in("rsi") buf,
                in("rdx") count,
                lateout("rax") result,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        result
    }

    #[doc = SYS_WRITE!()]
    #[must_use]
    pub unsafe fn sys_write(fd: c_int, buf: *const u8, count: usize) -> isize {
        let result;
        unsafe {
            asm!(
                "mov rax, {WRITE}",
                "syscall",
                WRITE = const SYS::WRITE,
                in("rdi") fd,
                in("rsi") buf,
                in("rdx") count,
                lateout("rax") result,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        result
    }

    #[doc = SYS_NANOSLEEP!()]
    #[must_use]
    pub unsafe fn sys_nanosleep(req: *const LinuxTimespec, rem: *mut LinuxTimespec) -> isize {
        let result;
        unsafe {
            asm!(
                "mov rax, {NANOSLEEP}",
                "syscall",
                NANOSLEEP = const SYS::NANOSLEEP,
                in("rdi") req,
                in("rsi") rem,
                lateout("rax") result,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        result
    }

    #[doc = SYS_IOCTL!()]
    #[must_use]
    pub unsafe fn sys_ioctl(fd: c_int, request: c_ulong, argp: *mut u8) -> isize {
        let result;
        unsafe {
            asm!(
                "mov rax, {IOCTL}",
                "syscall",
                IOCTL = const SYS::IOCTL,
                in("rdi") fd,
                in("rsi") request,
                in("rdx") argp,
                lateout("rax") result,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        result
    }

    #[doc = SYS_GETRANDOM!()]
    #[must_use]
    pub unsafe fn sys_getrandom(buffer: *mut u8, size: usize, flags: c_uint) -> isize {
        let result;
        unsafe {
            asm!(
                "mov rax, {GETRANDOM}",
                "syscall",
                GETRANDOM = const SYS::GETRANDOM,
                in("rdi") buffer,
                in("rsi") size,
                in("rdx") flags,
                lateout("rax") result,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        result
    }

    #[doc = SYS_GETPID!()]
    #[must_use]
    pub unsafe fn sys_getpid() -> i32 {
        let result: isize;
        unsafe {
            asm!(
                "mov rax, {GETPID}",
                "syscall",
                GETPID = const SYS::GETPID,
                lateout("rax") result,
                options(nostack)
            );
        }
        result as i32
    }

    #[doc = SYS_RT_SIGACTION!()]
    #[must_use]
    pub unsafe fn sys_rt_sigaction(
        sig: c_int,
        act: *const LinuxSigaction,
        oact: *mut LinuxSigaction,
        sigsetsize: usize,
    ) -> isize {
        let result;
        unsafe {
            asm!(
                "mov rax, {RT_SIGACTION}",
                "syscall",
                RT_SIGACTION = const SYS::RT_SIGACTION,
                in("rdi") sig,
                in("rsi") act,
                in("rdx") oact,
                in("r10") sigsetsize,
                lateout("rax") result,
                lateout("rcx") _,
                lateout("r11") _,
                options(nostack)
            );
        }
        result
    }
}
