// devela::io::linux::fns::syscalls::arm
//
//! Implements linux syscalls for arm.
//
// - https://arm.syscall.sh/
//
// - WAIT: [can't use r7 register](https://github.com/rust-lang/rust/issues/85056)

use super::shared_docs::*;
use crate::{asm, c_int, c_uint, c_ulong, Linux, LinuxSigaction, LinuxTimespec, LINUX_SYS as SYS};

/// System calls.
impl Linux {
    #[doc = SYS_EXIT!()]
    pub unsafe fn sys_exit(status: c_int) -> ! {
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

    #[doc = SYS_READ!()]
    #[must_use]
    pub unsafe fn sys_read(fd: c_int, buf: *mut u8, count: usize) -> isize {
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

    #[doc = SYS_WRITE!()]
    #[must_use]
    pub unsafe fn sys_write(fd: c_int, buf: *const u8, count: usize) -> isize {
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

    #[doc = SYS_NANOSLEEP!()]
    #[must_use]
    pub unsafe fn sys_nanosleep(req: *const LinuxTimespec, rem: *mut LinuxTimespec) -> isize {
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

    #[doc = SYS_IOCTL!()]
    #[must_use]
    pub unsafe fn sys_ioctl(fd: c_int, request: c_ulong, argp: *mut u8) -> isize {
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

    #[doc = SYS_GETRANDOM!()]
    #[must_use]
    pub unsafe fn sys_getrandom(buffer: *mut u8, size: usize, flags: c_uint) -> isize {
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

    #[doc = SYS_GETPID!()]
    #[must_use]
    pub unsafe fn sys_getpid() -> i32 {
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
}
