// devela::io::linux::syscalls::aarch64
//
//! Implements linux syscalls for aarch64.
//
// - https://arm64.syscall.sh/
// - https://syscalls.mebeim.net/?table=arm64/64/aarch64/latest

use super::{LinuxOffset, shared_docs::*};
use crate::{
    AT_FDCWD, LINUX_SYS as SYS, Linux, LinuxSigaction, LinuxStat, LinuxTimespec, asm, c_char,
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
    #[must_use]
    #[doc = SYS_WRITE!()]
    pub unsafe fn sys_write(fd: c_int, buf: *const c_uchar, count: usize) -> isize {
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
    #[must_use]
    #[doc = SYS_OPEN!()] // NOTE: OPENAT also in the other arches
    pub unsafe fn sys_open(path: *const c_char, flags: c_int, mode: c_uint) -> c_int {
        let result: c_int;
        unsafe {
            asm!(
                "mov x8, {OPENAT}",
                "svc 0",
                OPENAT = const SYS::OPENAT,
                in("x0") AT_FDCWD, // emulate open by using current working directory
                in("x1") path,
                in("x2") flags,
                in("x3") mode,
                lateout("x0") result,
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
                "mov x8, {CLOSE}",
                "svc 0",
                CLOSE = const SYS::CLOSE,
                in("x0") fd,
                lateout("x0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = SYS_LSEEK!()]
    pub unsafe fn sys_lseek(fd: c_int, offset: LinuxOffset, whence: c_int) -> LinuxOffset {
        let result: LinuxOffset;
        unsafe {
            asm!(
                "mov x8, {LSEEK}",
                "svc 0",
                LSEEK = const SYS::LSEEK,
                in("x0") fd,
                in("x1") offset,
                in("x2") whence,
                lateout("x0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = SYS_DUP!()]
    pub unsafe fn sys_dup(oldfd: c_int) -> c_int {
        let result: c_int;
        unsafe {
            asm!(
                "mov x8, {DUP}",
                "svc 0",
                DUP = const SYS::DUP,
                in("x0") oldfd,
                lateout("x0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = SYS_DUP2!()] // TODO: DUP3
    pub unsafe fn sys_dup2(oldfd: c_int, newfd: c_int) -> c_int {
        let result: c_int;
        unsafe {
            asm!(
                "mov x8, {DUP3}",
                "svc 0",
                DUP3 = const SYS::DUP3,
                in("x0") oldfd,
                in("x1") newfd,
                in("x2") 0,  // flags = 0 to mimic dup2 behavior
                lateout("x0") result,
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
                "mov x8, {FCNTL}",
                "svc 0",
                FCNTL = const SYS::FCNTL,
                in("x0") fd,
                in("x1") cmd,
                in("x2") arg,
                lateout("x0") result,
                options(nostack)
            );
        }
        result
    }
}

/// # Syscalls: Filesystem.
impl Linux {
    #[must_use]
    #[doc = SYS_STAT!()] // NOTE: there's NEWFSTATAT for x86_64 as well
    pub unsafe fn sys_stat(fd: c_int, statbuf: *mut LinuxStat) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "mov x8, {NEWFSTATAT}",
                "svc 0",
                NEWFSTATAT = const SYS::NEWFSTATAT,
                in("x0") AT_FDCWD, // emulate stat by using current working directory
                in("x1") fd,
                in("x2") statbuf,
                in("x3") 0, // flags = 0 to mimic stat behavior
                lateout("x0") result,
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
                "mov x8, {FSTAT}",
                "svc 0",
                FSTAT = const SYS::FSTAT,
                in("x0") fd,
                in("x1") statbuf,
                lateout("x0") result,
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
                "mov x8, {GETDENTS}",
                "svc 0",
                GETDENTS = const SYS::GETDENTS64,
                in("x0") fd,
                in("x1") dirp,
                in("x2") count,
                lateout("x0") result,
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
}

/// # Syscalls: IPC.
impl Linux {
    // #[must_use]
    // #[doc = SYS_PIPE!()]
    // pub unsafe fn sys_pipe(pipefd: *mut c_int) -> isize {
    //     let result: isize;
    //     unsafe {
    //         asm!(
    //             "mov x8, {PIPE}",
    //             "svc 0",
    //             PIPE = const SYS::PIPE,
    //             in("x0") pipefd,
    //             lateout("x0") result,
    //             options(nostack)
    //         );
    //     }
    //     result
    // }
    #[must_use]
    #[doc = SYS_PIPE2!()]
    pub unsafe fn sys_pipe2(pipefd: *mut c_int, flags: c_int) -> isize {
        let result: isize;
        unsafe {
            asm!(
                "mov x8, {PIPE2}",
                "svc 0",
                PIPE2 = const SYS::PIPE2,
                in("x0") pipefd,
                in("x1") flags,
                lateout("x0") result,
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
                "mov x8, {EXIT}",
                "svc 0",
                EXIT = const SYS::EXIT,
                in("x8") SYS::EXIT,
                in("x0") status,
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
                "mov x8, {GETPID}",
                "svc 0",
                GETPID = const SYS::GETPID,
                lateout("x8") result,
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
}

/// # Syscalls: Timing and signal handling.
impl Linux {
    #[must_use]
    #[doc = SYS_CLOCK_GETRES!()]
    pub unsafe fn sys_clock_getres(clock_id: c_int, res: *mut LinuxTimespec) -> isize {
        let result;
        unsafe {
            asm!(
                "mov x8, {CLOCK_GETRES}",
                "svc 0",
                CLOCK_GETRES = const SYS::CLOCK_GETRES,
                in("x0") clock_id,
                in("x1") res,
                lateout("x0") result,
                options(nostack)
            );
        }
        result
    }

    #[must_use]
    #[doc = SYS_CLOCK_GETTIME!()]
    pub unsafe fn sys_clock_gettime(clock_id: c_int, tp: *mut LinuxTimespec) -> isize {
        let result;
        unsafe {
            asm!(
                "mov x8, {CLOCK_GETTIME}",
                "svc 0",
                CLOCK_GETTIME = const SYS::CLOCK_GETTIME,
                in("x0") clock_id,
                in("x1") tp,
                lateout("x0") result,
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
