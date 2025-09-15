// devela::io::linux::syscalls::x86
//
//! Implements linux syscalls for x86.
//
// - https://x86.syscall.sh/
// - https://syscalls.mebeim.net/?table=x86/32/ia32/latest

use super::{LinuxOffset, shared_docs::*};
use crate::{
    LINUX_SYS as SYS, Linux, LinuxClock, LinuxSigaction, LinuxStat, LinuxTimespec, asm, c_char,
    c_int, c_uchar, c_uint, c_ulong,
};

/// # Syscalls: File descriptors.
impl Linux {
    #[must_use]
    #[doc = SYS_READ!()]
    pub unsafe fn sys_read(fd: c_int, buf: *mut c_uchar, count: usize) -> isize {
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
    #[must_use]
    #[doc = SYS_WRITE!()]
    pub unsafe fn sys_write(fd: c_int, buf: *const c_uchar, count: usize) -> isize {
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
    #[must_use]
    #[doc = SYS_OPEN!()]
    pub unsafe fn sys_open(path: *const c_char, flags: c_int, mode: c_uint) -> c_int {
        let result: c_int;
        unsafe {
            asm!(
                "int 0x80",
                in("eax") SYS::OPEN,
                in("ebx") path,
                in("ecx") flags,
                in("edx") mode,
                lateout("eax") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = SYS_CLOSE!()]
    pub unsafe fn sys_close(fd: c_int) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "int 0x80",
                in("eax") SYS::CLOSE,
                in("ebx") fd,
                lateout("eax") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = SYS_LSEEK!()]
    pub unsafe fn sys_lseek(fd: c_int, offset: LinuxOffset, whence: c_int) -> LinuxOffset {
        let result_lo: u32;
        let result_hi: u32;
        unsafe {
            asm!(
                "push ebx",
                "mov ebx, {fd}", // fd in ebx (arg1)
                "mov eax, {LSEEK}",
                "int 0x80",
                "pop ebx",
                fd = in(reg) fd,
                LSEEK = const SYS::LSEEK as u32,
                in("ecx") offset as u32,          // Low 32 bits (arg2)
                in("edx") (offset >> 32) as u32,  // High 32 bits (arg3)
                in("edi") whence,                 // (arg4)
                lateout("eax") result_lo,
                lateout("edx") result_hi,
                options(nostack, preserves_flags)
            );
        }
        ((result_hi as i64) << 32) | (result_lo as i64)
    }
    #[must_use]
    #[doc = SYS_DUP!()]
    pub unsafe fn sys_dup(oldfd: c_int) -> c_int {
        let result: c_int;
        unsafe {
            asm!(
                "int 0x80",
                in("eax") SYS::DUP,
                in("ebx") oldfd,
                lateout("eax") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = SYS_DUP2!()]
    pub unsafe fn sys_dup2(oldfd: c_int, newfd: c_int) -> c_int {
        let result: c_int;
        unsafe {
            asm!(
                "int 0x80",
                in("eax") SYS::DUP2,
                in("ebx") oldfd,
                in("ecx") newfd,
                lateout("eax") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = SYS_FCNTL!()]
    pub unsafe fn sys_fcntl(fd: c_int, cmd: c_int, arg: c_ulong) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "int 0x80",
                in("eax") SYS::FCNTL,
                in("ebx") fd,
                in("ecx") cmd,
                in("edx") arg,
                lateout("eax") result,
                options(nostack)
            );
        }
        result
    }
}

/// # Syscalls: Filesystem.
impl Linux {
    #[must_use]
    #[doc = SYS_STAT!()]
    pub unsafe fn sys_stat(path: *const c_char, statbuf: *mut LinuxStat) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "int 0x80",
                in("eax") SYS::STAT,
                in("ebx") path,
                in("ecx") statbuf,
                lateout("eax") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = SYS_FSTAT!()]
    pub unsafe fn sys_fstat(fd: c_int, statbuf: *mut LinuxStat) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "int 0x80",
                in("eax") SYS::FSTAT,
                in("ebx") fd,
                in("ecx") statbuf,
                lateout("eax") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = SYS_GETDENTS!()]
    pub unsafe fn sys_getdents(fd: c_int, dirp: *mut c_uchar, count: usize) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "int 0x80",
                in("eax") SYS::GETDENTS,
                in("ebx") fd,
                in("ecx") dirp,
                in("edx") count,
                lateout("eax") result,
                options(nostack)
            );
        }
        result
    }
}

/// # Syscalls: Device and special I/O.
impl Linux {
    #[must_use]
    #[doc = SYS_IOCTL!()]
    pub unsafe fn sys_ioctl(fd: c_int, request: c_ulong, argp: *mut c_uchar) -> isize {
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
}

/// # Syscalls: IPC.
impl Linux {
    #[must_use]
    #[doc = SYS_PIPE!()]
    pub unsafe fn sys_pipe(pipefd: *mut c_int) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "int 0x80",
                in("eax") SYS::PIPE,
                in("ebx") pipefd,
                lateout("eax") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = SYS_PIPE2!()]
    pub unsafe fn sys_pipe2(pipefd: *mut c_int, flags: c_int) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "int 0x80",
                in("eax") SYS::PIPE2,
                in("ebx") pipefd,
                in("ecx") flags,
                lateout("eax") result,
                options(nostack)
            );
        }
        result
    }
}

/// # Syscalls: Process control.
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
    #[must_use]
    #[doc = SYS_GETPID!()]
    pub unsafe fn sys_getpid() -> c_int {
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
        result as c_int
    }
    #[must_use]
    #[doc = SYS_GETRANDOM!()]
    pub unsafe fn sys_getrandom(buffer: *mut c_uchar, size: usize, flags: c_uint) -> isize {
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
}

/// # Syscalls: Timing and signal handling.
impl Linux {
    #[must_use]
    #[doc = SYS_CLOCK_GETRES!()]
    pub unsafe fn sys_clock_getres(clock_id: LinuxClock, res: *mut LinuxTimespec) -> isize {
        let result;
        unsafe {
            asm!(
                "mov r7, {CLOCK_GETRES}",
                "svc 0",
                CLOCK_GETRES = const SYS::CLOCK_GETRES,
                in("r0") clock_id.as_raw(),
                in("r1") res,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }

    #[must_use]
    #[doc = SYS_CLOCK_GETTIME!()]
    pub unsafe fn sys_clock_gettime(clock_id: LinuxClock, tp: *mut LinuxTimespec) -> isize {
        let result;
        unsafe {
            asm!(
                "mov eax, {CLOCK_GETTIME}",
                "int 0x80",
                CLOCK_GETTIME = const SYS::CLOCK_GETTIME,
                in("ebx") clock_id.as_raw(),
                in("ecx") tp,
                lateout("eax") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = SYS_NANOSLEEP!()]
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
    #[must_use]
    #[doc = SYS_RT_SIGACTION!()]
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
