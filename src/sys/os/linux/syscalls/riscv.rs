// devela::io::linux::fns::syscalls::riscv
//
//! Implements linux syscalls for both riscv32 and riscv64.
//
// - https://syscalls.mebeim.net/?table=riscv/32/rv32/latest
// - https://syscalls.mebeim.net/?table=riscv/64/rv64/latest

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
                "li a7, {READ}",
                "ecall",
                READ = const SYS::READ,
                in("a0") fd,
                in("a1") buf,
                in("a2") count,
                lateout("a7") result,
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
                "li a7, {WRITE}",
                "ecall",
                WRITE = const SYS::WRITE,
                in("a0") fd,
                in("a1") buf,
                in("a2") count,
                lateout("a7") result,
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
                "li a7, {OPENAT}",
                "ecall",
                OPENAT = const SYS::OPENAT,
                in("a0") AT_FDCWD,
                in("a1") path,
                in("a2") flags,
                in("a3") mode,
                lateout("a0") result,
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
                "li a7, {CLOSE}",
                "ecall",
                CLOSE = const SYS::CLOSE,
                in("a0") fd,
                lateout("a0") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = SYS_LSEEK!()]
    #[cfg(target_arch = "riscv32")]
    pub unsafe fn sys_lseek(fd: c_int, offset: LinuxOffset, whence: c_int) -> LinuxOffset {
        let result_lo: u32;
        let result_hi: u32;
        unsafe {
            asm!(
                "li a7, {LSEEK}",
                "mv a0, {fd}",
                "mv a1, {offset_lo}",
                "mv a2, {offset_hi}",
                "mv a3, {whence}",
                "ecall",
                LSEEK = const SYS::LSEEK,
                fd = in(reg) fd,
                offset_lo = in(reg) (offset as u32),
                offset_hi = in(reg) ((offset >> 32) as u32),
                whence = in(reg) whence,
                lateout("a0") result_lo,
                lateout("a1") result_hi,
                options(nostack)
            );
        }
        ((result_hi as i64) << 32) | (result_lo as i64)
    }
    #[must_use]
    #[doc = SYS_LSEEK!()]
    #[cfg(target_arch = "riscv64")]
    pub unsafe fn sys_lseek(fd: c_int, offset: LinuxOffset, whence: c_int) -> LinuxOffset {
        let result: LinuxOffset;
        unsafe {
            asm!(
                "li a7, {LSEEK}",
                "ecall",
                LSEEK = const SYS::LSEEK,
                in("a0") fd,
                in("a1") offset,
                in("a2") whence,
                lateout("a0") result,
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
                "li a7, {DUP}",
                "ecall",
                DUP = const SYS::DUP,
                in("a0") oldfd,
                lateout("a0") result,
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
                "li a7, {DUP3}",
                "ecall",
                DUP3 = const SYS::DUP3,
                in("a0") oldfd,
                in("a1") newfd,
                in("a2") 0,
                lateout("a0") result,
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
                "li a7, {FCNTL}",
                "ecall",
                FCNTL = const SYS::FCNTL,
                in("a0") fd,
                in("a1") cmd,
                in("a2") arg,
                lateout("a0") result,
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
                "li a7, {NEWFSTATAT}",
                "li a0, {AT_FDCWD}",
                "mv a1, {path}",
                "mv a2, {statbuf}",
                "li a3, 0",     // flags = 0
                "ecall",
                NEWFSTATAT = const SYS::NEWFSTATAT,
                AT_FDCWD = const AT_FDCWD,
                path = in(reg) path,
                statbuf = in(reg) statbuf,
                lateout("a0") result,
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
                "li a7, {FSTAT}",
                "ecall",
                FSTAT = const SYS::FSTAT,
                in("a0") fd,
                in("a1") statbuf,
                lateout("a0") result,
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
                "li a7, {GETDENTS64}",
                "ecall",
                GETDENTS64 = const SYS::GETDENTS64,
                in("a0") fd,
                in("a1") dirp,
                in("a2") count,
                lateout("a0") result,
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
                "li a7, {IOCTL}",
                "ecall",
                IOCTL = const SYS::IOCTL,
                in("a0") fd,
                in("a1") request,
                in("a2") argp,
                lateout("a7") result,
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
    //             "li a7, {PIPE}",
    //             "ecall",
    //             PIPE = const SYS::PIPE,
    //             in("a0") pipefd,
    //             lateout("a0") result,
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
                "li a7, {PIPE2}",
                "ecall",
                PIPE2 = const SYS::PIPE2,
                in("a0") pipefd,
                in("a1") flags,
                lateout("a0") result,
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
                "li a7, {EXIT}",
                "ecall",
                EXIT = const SYS::EXIT,
                in("a0") status,
                options(noreturn)
            );
        }
    }
    #[must_use]
    #[doc = SYS_GETRANDOM!()]
    pub unsafe fn sys_getrandom(buffer: *mut c_uchar, size: usize, flags: c_uint) -> isize {
        let result;
        unsafe {
            asm!(
                "li a7, {GETRANDOM}",
                "ecall",
                GETRANDOM = const SYS::GETRANDOM,
                in("a0") buffer,
                in("a1") size,
                in("a2") flags,
                lateout("a7") result,
                options(nostack)
            );
        }
        result
    }
    #[must_use]
    #[doc = SYS_GETPID!()]
    pub unsafe fn sys_getpid() -> c_int {
        let result: isize;
        unsafe {
            asm!(
                "li a7, {GETPID}",
                "ecall",
                GETPID = const SYS::GETPID,
                lateout("a7") result,
                options(nostack)
            );
        }
        result as c_int
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
                "li a7, {CLOCK_GETRES}",
                "ecall",
                CLOCK_GETRES = const SYS::CLOCK_GETRES,
                in("a0") clock_id,
                in("a1") res,
                lateout("a0") result,
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
                "li a7, {CLOCK_GETTIME}",
                "ecall",
                CLOCK_GETTIME = const SYS::CLOCK_GETTIME,
                in("a0") clock_id,
                in("a1") tp,
                lateout("a0") result,
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
                "li a7, {NANOSLEEP}",
                "ecall",
                NANOSLEEP = const SYS::NANOSLEEP,
                in("a0") req,
                in("a1") rem,
                lateout("a7") result,
                lateout("a2") _,
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
                "li a7, {RT_SIGACTION}",
                "ecall",
                RT_SIGACTION = const SYS::RT_SIGACTION,
                in("a0") sig,
                in("a1") act,
                in("a2") oact,
                in("a3") sigsetsize,
                lateout("a7") result,
                options(nostack)
            );
        }
        result
    }
}
