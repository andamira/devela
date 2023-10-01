// devela::io::linux::fns::syscalls::riscv
//
//! Both for riscv32 and riscv64
//

use crate::os::linux::{LinuxSigaction, LinuxTimespec, LINUX_SYS_RISCV as SYS};
use core::{
    arch::asm,
    ffi::{c_int, c_uint, c_ulong},
};

#[doc = include_str!("./doc/Sys_exit.md")]
pub unsafe fn linux_sys_exit(status: c_int) -> ! {
    unsafe {
        asm!(
            "ecall",
            in("a7") SYS::EXIT,
            in("a0") status,
            options(noreturn)
        );
    }
}

#[doc = include_str!("./doc/Sys_read.md")]
#[must_use]
pub unsafe fn linux_sys_read(fd: c_int, buf: *mut u8, count: usize) -> isize {
    let r0;
    asm!(
        "ecall",
        inlateout("a7") SYS::READ => r0,
        in("a0") fd,
        in("a1") buf,
        in("a2") count,
        options(nostack, preserves_flags)
    );
    r0
}

#[doc = include_str!("./doc/Sys_write.md")]
#[must_use]
pub unsafe fn linux_sys_write(fd: c_int, buf: *const u8, count: usize) -> isize {
    let r0;
    asm!(
        "ecall",
        inlateout("a7") SYS::WRITE => r0,
        in("a0") fd,
        in("a1") buf,
        in("a2") count,
        options(nostack, preserves_flags)
    );
    r0
}

#[doc = include_str!("./doc/Sys_nanosleep.md")]
#[must_use]
pub unsafe fn linux_sys_nanosleep(req: *const LinuxTimespec, rem: *mut LinuxTimespec) -> isize {
    let r0;
    asm!(
        "ecall",
        inlateout("a7") SYS::NANOSLEEP => r0,
        in("a0") fd,
        in("a1") buf,
        lateout("a2") _,
        options(nostack, preserves_flags)
    );
    r0
}

#[doc = include_str!("./doc/Sys_ioctl.md")]
#[must_use]
pub unsafe fn linux_sys_ioctl(fd: c_int, request: c_ulong, argp: *mut u8) -> isize {
    let r0;
    asm!(
        "ecall",
        inlateout("a7") SYS::IOCTL => r0,
        in("a0") fd,
        in("a1") request,
        in("a2") argp,
        options(nostack, preserves_flags)
    );
    r0
}

#[doc = include_str!("./doc/Sys_getrandom.md")]
#[must_use]
pub unsafe fn linux_sys_getrandom(buffer: *mut u8, size: usize, flags: c_uint) -> isize {
    let r0;
    asm!(
        "ecall",
        inlateout("a7") SYS::GETRANDOM => r0,
        in("a0") buffer,
        in("a1") size,
        in("a2") flags,
        options(nostack, preserves_flags)
    );
    r0
}

#[doc = include_str!("./doc/Sys_getpid.md")]
#[must_use]
pub unsafe fn linux_sys_getpid() -> i32 {
    let r0: isize;
    asm!(
        "ecall",
        inlateout("a7") SYS::GETPID => r0,
        options(nostack, preserves_flags)
    );
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
    asm!(
        "ecall",
        inlateout("a7") SYS::RT_SIGACTION => r0,
        in("a0") sig,
        in("a1") act,
        in("a2") oact,
        in("a3") sigsetsize,
        options(nostack, preserves_flags)
    );
    r0
}
