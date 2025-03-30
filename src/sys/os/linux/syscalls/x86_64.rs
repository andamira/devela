// devela::io::linux::syscalls:x86_64
//
//! Implements linux syscalls for x86_64.
//
// - https://x64.syscall.sh/
// - https://syscalls.mebeim.net/?table=x86/64/x64/latest
// - https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/
//
// Syscalls: 19

use super::{LinuxOffset, shared_docs::*};
use crate::{
    LINUX_SYS as SYS, Linux, LinuxSigaction, LinuxStat, LinuxTimespec, asm, c_char, c_int, c_uint,
    c_ulong,
};

/// # Syscalls: File descriptors.
impl Linux {
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
    #[doc = SYS_OPEN!()]
    #[must_use]
    pub unsafe fn sys_open(path: *const c_char, flags: c_int, mode: c_uint) -> c_int {
        let result: c_int;
        unsafe {
            asm!(
                "mov rax, {OPEN}",
                "syscall",
                OPEN = const SYS::OPEN,
                in("rdi") path,
                in("rsi") flags,
                in("rdx") mode,
                lateout("rax") result,
                options(nostack)
            );
        }
        result
    }
    #[doc = SYS_CLOSE!()]
    #[must_use]
    pub unsafe fn sys_close(fd: c_int) -> isize {
        let result;
        unsafe {
            asm!(
                "mov rax, {CLOSE}",
                "syscall",
                CLOSE = const SYS::CLOSE,
                in("rdi") fd,
                lateout("rax") result,
                options(nostack)
            );
        }
        result
    }
    #[doc = SYS_LSEEK!()]
    #[must_use]
    pub unsafe fn sys_lseek(fd: c_int, offset: LinuxOffset, whence: c_int) -> LinuxOffset {
        let result;
        unsafe {
            asm!(
                "mov rax, {LSEEK}",
                "syscall",
                LSEEK = const SYS::LSEEK,
                in("rdi") fd,
                in("rsi") offset,
                in("rdx") whence,
                lateout("rax") result,
                options(nostack)
            );
        }
        result
    }
    #[doc = SYS_DUP!()]
    #[must_use]
    pub unsafe fn sys_dup(oldfd: c_int) -> c_int {
        let result: c_int;
        unsafe {
            asm!(
                "mov rax, {DUP}",
                "syscall",
                DUP = const SYS::DUP,
                in("rdi") oldfd,
                lateout("rax") result,
                options(nostack)
            );
        }
        result
    }
    #[doc = SYS_DUP2!()]
    #[must_use]
    pub unsafe fn sys_dup2(oldfd: c_int, newfd: c_int) -> c_int {
        let result: c_int;
        unsafe {
            asm!(
                "mov rax, {DUP2}",
                "syscall",
                DUP2 = const SYS::DUP2,
                in("rdi") oldfd,
                in("rsi") newfd,
                lateout("rax") result,
                options(nostack)
            );
        }
        result
    }
    #[doc = SYS_FCNTL!()]
    #[must_use]
    pub unsafe fn sys_fcntl(fd: c_int, cmd: c_int, arg: c_ulong) -> isize {
        let result;
        unsafe {
            asm!(
                "mov rax, {FCNTL}",
                "syscall",
                FCNTL = const SYS::FCNTL,
                in("rdi") fd,
                in("rsi") cmd,
                in("rdx") arg,
                lateout("rax") result,
                options(nostack)
            );
        }
        result
    }
}

/// # Syscalls: Filesystem.
impl Linux {
    #[doc = SYS_STAT!()]
    #[must_use]
    pub unsafe fn sys_stat(path: *const c_char, statbuf: *mut LinuxStat) -> isize {
        let result;
        unsafe {
            asm!(
                "mov rax, {STAT}",
                "syscall",
                STAT = const SYS::STAT,
                in("rdi") path,
                in("rsi") statbuf,
                lateout("rax") result,
                options(nostack)
            );
        }
        result
    }
    #[doc = SYS_FSTAT!()]
    #[must_use]
    pub unsafe fn sys_fstat(fd: c_int, statbuf: *mut LinuxStat) -> isize {
        let result;
        unsafe {
            asm!(
                "mov rax, {FSTAT}",
                "syscall",
                FSTAT = const SYS::FSTAT,
                in("rdi") fd,
                in("rsi") statbuf,
                lateout("rax") result,
                options(nostack)
            );
        }
        result
    }
    #[doc = SYS_GETDENTS!()]
    #[must_use]
    pub unsafe fn sys_getdents(fd: c_int, dirp: *mut u8, count: usize) -> isize {
        let result;
        unsafe {
            asm!(
                "mov rax, {GETDENTS}",
                "syscall",
                GETDENTS = const SYS::GETDENTS64,
                in("rdi") fd,
                in("rsi") dirp,
                in("rdx") count,
                lateout("rax") result,
                options(nostack)
            );
        }
        result
    }
}

/// # Syscalls: Device and special I/O.
// TODO
// - poll/epoll
// - inotify
// - signalfd
impl Linux {
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
}

/// # Syscalls: IPC.
impl Linux {
    #[doc = SYS_PIPE!()]
    #[must_use]
    pub unsafe fn sys_pipe(pipefd: *mut c_int) -> isize {
        let result;
        unsafe {
            asm!(
                "mov rax, {PIPE}",
                "syscall",
                PIPE = const SYS::PIPE,
                in("rdi") pipefd,
                lateout("rax") result,
                options(nostack)
            );
        }
        result
    }
    #[doc = SYS_PIPE2!()]
    #[must_use]
    pub unsafe fn sys_pipe2(pipefd: *mut c_int, flags: c_int) -> isize {
        let result;
        unsafe {
            asm!(
                "mov rax, {PIPE2}",
                "syscall",
                PIPE2 = const SYS::PIPE2,
                in("rdi") pipefd,
                in("rsi") flags,
                lateout("rax") result,
                options(nostack)
            );
        }
        result
    }
}

/// # Syscalls: Process control.
// TODO
// - getppid
impl Linux {
    #[doc = SYS_EXIT!()]
    pub unsafe fn sys_exit(status: c_int) -> ! {
        unsafe {
            asm!(
                "mov rax, {EXIT}",
                "syscall",
                EXIT = const SYS::EXIT,
                in("rdi") status,
                // MAYBE: UNDERSTAND
                // lateout("rcx") _,
                // lateout("r11") _,
                options(noreturn)
            );
        }
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
}

/// # Syscalls: Timing and signal handling.
impl Linux {
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
