// devela::io::linux::fns::syscalls::arm
//
//! Implements linux syscalls for arm.
//
// - https://arm.syscall.sh/
// - https://syscalls.mebeim.net/?table=arm/32/eabi/latest
//
// - WAIT: [use of r7 register](https://github.com/rust-lang/rust/issues/85056)

use super::{LinuxOffset, shared_docs::*};
use crate::{
    LINUX_SYS as SYS, Linux, LinuxClock, LinuxSigaction, LinuxStat, LinuxTimespec, asm, c_char,
    c_int, c_uchar, c_uint, c_ulong,
};

/// # Syscalls: File descriptors.
impl Linux {
    #[must_use]
    #[doc = _DOC_SYS_READ!()]
    pub unsafe fn sys_read(fd: c_int, buf: *mut c_uchar, count: usize) -> isize {
        let result;
        unsafe {
            asm!(
                "mov r7, {READ}",
                "svc 0",
                READ = const SYS::READ,
                in("r0") fd,
                in("r1") buf,
                in("r2") count,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = _DOC_SYS_WRITE!()]
    pub unsafe fn sys_write(fd: c_int, buf: *const c_uchar, count: usize) -> isize {
        let result;
        unsafe {
            asm!(
                "mov r7, {WRITE}",
                "svc 0",
                WRITE = const SYS::WRITE,
                in("r0") fd,
                in("r1") buf,
                in("r2") count,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = _DOC_SYS_OPEN!()]
    pub unsafe fn sys_open(path: *const c_char, flags: c_int, mode: c_uint) -> c_int {
        let result: c_int;
        unsafe {
            asm!(
                "mov r7, {OPEN}",
                "svc 0",
                OPEN = const SYS::OPEN,
                in("r0") path,
                in("r1") flags,
                in("r2") mode,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = _DOC_SYS_CLOSE!()]
    pub unsafe fn sys_close(fd: c_int) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "mov r7, {CLOSE}",
                "svc 0",
                CLOSE = const SYS::CLOSE,
                in("r0") fd,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = _DOC_SYS_LSEEK!()]
    pub unsafe fn sys_lseek(fd: c_int, offset: LinuxOffset, whence: c_int) -> LinuxOffset {
        let result_lo: u32;
        let result_hi: u32;
        unsafe {
            asm!(
                "mov r7, {LSEEK}",
                "svc 0",
                LSEEK = const SYS::LSEEK as u32,
                in("r0") fd,
                in("r1") offset as u32,          // Low 32 bits
                in("r2") (offset >> 32) as u32,  // High 32 bits
                in("r3") whence,
                lateout("r0") result_lo,
                lateout("r1") result_hi,
                options(nostack)
            );
        }
        ((result_hi as i64) << 32) | (result_lo as i64)
    }
    #[must_use]
    #[doc = _DOC_SYS_DUP!()]
    pub unsafe fn sys_dup(oldfd: c_int) -> c_int {
        let result: c_int;
        unsafe {
            asm!(
                "mov r7, {DUP}",
                "svc 0",
                DUP = const SYS::DUP,
                in("r0") oldfd,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = _DOC_SYS_DUP2!()]
    pub unsafe fn sys_dup2(oldfd: c_int, newfd: c_int) -> c_int {
        let result: c_int;
        unsafe {
            asm!(
                "mov r7, {DUP2}",
                "svc 0",
                DUP2 = const SYS::DUP2,
                in("r0") oldfd,
                in("r1") newfd,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = _DOC_SYS_FCNTL!()]
    pub unsafe fn sys_fcntl(fd: c_int, cmd: c_int, arg: c_ulong) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "mov r7, {FCNTL}",
                "svc 0",
                FCNTL = const SYS::FCNTL,
                in("r0") fd,
                in("r1") cmd,
                in("r2") arg,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
}

/// # Syscalls: Filesystem.
impl Linux {
    #[must_use]
    #[doc = _DOC_SYS_STAT!()]
    pub unsafe fn sys_stat(path: *const c_char, statbuf: *mut LinuxStat) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "mov r7, {STAT}",
                "svc 0",
                STAT = const SYS::STAT,
                in("r0") path,
                in("r1") statbuf,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = _DOC_SYS_FSTAT!()]
    pub unsafe fn sys_fstat(fd: c_int, statbuf: *mut LinuxStat) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "mov r7, {FSTAT}",
                "svc 0",
                FSTAT = const SYS::FSTAT,
                in("r0") fd,
                in("r1") statbuf,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = _DOC_SYS_GETDENTS!()]
    pub unsafe fn sys_getdents(fd: c_int, dirp: *mut c_uchar, count: usize) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "mov r7, {GETDENTS64}",
                "svc 0",
                GETDENTS64 = const SYS::GETDENTS64,
                in("r0") fd,
                in("r1") dirp,
                in("r2") count,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
}

/// # Syscalls: Device and special I/O.
impl Linux {
    #[must_use]
    #[doc = _DOC_SYS_IOCTL!()]
    pub unsafe fn sys_ioctl(fd: c_int, request: c_ulong, argp: *mut c_uchar) -> isize {
        let result;
        unsafe {
            asm!(
                "mov r7, {IOCTL}",
                "svc 0",
                IOCTL = const SYS::IOCTL,
                in("r0") fd,
                in("r1") request,
                in("r2") argp,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
}

/// # Syscalls: IPC.
impl Linux {
    #[must_use]
    #[doc = _DOC_SYS_PIPE!()]
    pub unsafe fn sys_pipe(pipefd: *mut c_int) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "mov r7, {PIPE}",
                "svc 0",
                PIPE = const SYS::PIPE,
                in("r0") pipefd,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = _DOC_SYS_PIPE2!()]
    pub unsafe fn sys_pipe2(pipefd: *mut c_int, flags: c_int) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "mov r7, {PIPE2}",
                "svc 0",
                PIPE2 = const SYS::PIPE2,
                in("r0") pipefd,
                in("r1") flags,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
}

/// # Syscalls: Process control.
impl Linux {
    #[doc = _DOC_SYS_EXIT!()]
    pub unsafe fn sys_exit(status: c_int) -> ! {
        unsafe {
            asm!(
                "mov r7, {EXIT}",
                "svc 0",
                EXIT = const SYS::EXIT,
                in("r0") status,
                options(noreturn)
            );
        }
    }
    #[must_use]
    #[doc = _DOC_SYS_GETPID!()]
    pub unsafe fn sys_getpid() -> c_int {
        let result: isize;
        unsafe {
            asm!(
                "mov r7, {GETPID}",
                "svc 0",
                GETPID = const SYS::GETPID,
                lateout("r0") result,
                options(nostack)
            );
        }
        result as c_int
    }
    #[must_use]
    #[doc = _DOC_SYS_GETRANDOM!()]
    pub unsafe fn sys_getrandom(buffer: *mut c_uchar, size: usize, flags: c_uint) -> isize {
        let result;
        unsafe {
            asm!(
                "mov r7, {GETRANDOM}",
                "svc 0",
                GETRANDOM = const SYS::GETRANDOM,
                in("r0") buffer,
                in("r1") size,
                in("r2") flags,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
}

/// # Syscalls: Timing and signal handling.
impl Linux {
    #[must_use]
    #[doc = _DOC_SYS_CLOCK_GETRES!()]
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
    #[doc = _DOC_SYS_CLOCK_GETTIME!()]
    pub unsafe fn sys_clock_gettime(clock_id: LinuxClock, tp: *mut LinuxTimespec) -> isize {
        let result;
        unsafe {
            asm!(
                "mov r7, {CLOCK_GETTIME}",
                "svc 0",
                CLOCK_GETTIME = const SYS::CLOCK_GETTIME,
                in("r0") clock_id.as_raw(),
                in("r1") tp,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = _DOC_SYS_NANOSLEEP!()]
    pub unsafe fn sys_nanosleep(req: *const LinuxTimespec, rem: *mut LinuxTimespec) -> isize {
        let result;
        unsafe {
            asm!(
                "mov r7, {NANOSLEEP}",
                "svc 0",
                NANOSLEEP = const SYS::NANOSLEEP,
                in("r0") req,
                in("r1") rem,
                lateout("r2") _,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = _DOC_SYS_RT_SIGACTION!()]
    pub unsafe fn sys_rt_sigaction(
        sig: c_int,
        act: *const LinuxSigaction,
        oact: *mut LinuxSigaction,
        sigsetsize: usize,
    ) -> isize {
        let result;
        unsafe {
            asm!(
                "mov r7, {RT_SIGACTION}",
                "svc 0",
                RT_SIGACTION = const SYS::RT_SIGACTION,
                in("r0") sig,
                in("r1") act,
                in("r2") oact,
                in("r3") sigsetsize,
                lateout("r0") result,
                options(nostack)
            );
        }
        result
    }
}
