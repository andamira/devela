// devela::io::linux::syscalls::aarch64
//
//! Implements linux syscalls for aarch64.
//
//- https://x64.syscall.sh/

use super::shared_docs::*;
use crate::{
    asm, c_int, c_uint, c_ulong, Linux, LinuxSigaction, LinuxTimespec, LINUX_SYS_AARCH64 as SYS,
};

/// System calls.
impl Linux {
    #[doc = SYS_EXIT!()]
    pub unsafe fn sys_exit(status: c_int) -> ! {
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

    #[doc = SYS_READ!()]
    #[must_use]
    pub unsafe fn sys_read(fd: c_int, buf: *mut u8, count: usize) -> isize {
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

    #[doc = SYS_WRITE!()]
    #[must_use]
    pub unsafe fn sys_write(fd: c_int, buf: *const u8, count: usize) -> isize {
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

    #[doc = SYS_NANOSLEEP!()]
    #[must_use]
    pub unsafe fn sys_nanosleep(req: *const LinuxTimespec, rem: *mut LinuxTimespec) -> isize {
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

    #[doc = SYS_IOCTL!()]
    #[must_use]
    pub unsafe fn sys_ioctl(fd: c_int, request: c_ulong, argp: *mut u8) -> isize {
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

    #[doc = SYS_GETRANDOM!()]
    #[must_use]
    pub unsafe fn sys_getrandom(buffer: *mut u8, size: usize, flags: c_uint) -> isize {
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

    #[doc = SYS_GETPID!()]
    #[must_use]
    pub unsafe fn sys_getpid() -> i32 {
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
}
