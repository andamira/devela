// devela::io::linux::syscalls::x86
//
//! Implements linux syscalls for x86.
//
// - https://x86.syscall.sh/

use super::shared_docs::*;
use crate::{
    asm, c_int, c_uint, c_ulong, Linux, LinuxSigaction, LinuxTimespec, LINUX_SYS_X86 as SYS,
};

/// System calls.
impl Linux {
    #[doc = SYS_EXIT!()]
    pub unsafe fn sys_exit(status: c_int) -> ! {
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

    #[doc = SYS_READ!()]
    #[must_use]
    pub unsafe fn sys_read(fd: c_int, buf: *mut u8, count: usize) -> isize {
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

    #[doc = SYS_WRITE!()]
    #[must_use]
    pub unsafe fn sys_write(fd: c_int, buf: *const u8, count: usize) -> isize {
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

    #[doc = SYS_NANOSLEEP!()]
    #[must_use]
    pub unsafe fn sys_nanosleep(req: *const LinuxTimespec, rem: *mut LinuxTimespec) -> isize {
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

    #[doc = SYS_IOCTL!()]
    #[must_use]
    pub unsafe fn sys_ioctl(fd: c_int, request: c_ulong, argp: *mut u8) -> isize {
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

    #[doc = SYS_GETRANDOM!()]
    #[must_use]
    pub unsafe fn sys_getrandom(buffer: *mut u8, size: usize, flags: c_uint) -> isize {
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

    #[doc = SYS_GETPID!()]
    #[must_use]
    pub unsafe fn sys_getpid() -> i32 {
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
}
