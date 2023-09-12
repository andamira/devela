// devela::io::linux::riscv
//
//! Both for riscv32 and riscv64
//

// use super::SysTimeSpec;
use core::{
    arch::asm,
    ffi::{c_int, c_ulong},
};

#[doc = include_str!("./doc/Sys_exit.md")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub unsafe fn sys_exit(status: c_int) -> ! {
    const SYS_EXIT: isize = 93;
    unsafe {
        asm!(
            "ecall",
            in("a7") SYS_EXIT,
            in("a0") status,
            options(noreturn)
        );
    }
}

#[doc = include_str!("./doc/Sys_read.md")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub unsafe fn sys_read(fd: c_int, buf: *mut u8, count: usize) -> isize {
    const SYS_READ: isize = 63;
    let r0;
    asm!(
        "ecall",
        inlateout("a7") SYS_READ => r0,
        in("a0") fd,
        in("a1") buf,
        in("a2") count,
        options(nostack, preserves_flags)
    );
    r0
}

#[doc = include_str!("./doc/Sys_write.md")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub unsafe fn sys_write(fd: c_int, buf: *const u8, count: usize) -> isize {
    const SYS_WRITE: isize = 64;
    let r0;
    asm!(
        "ecall",
        inlateout("a7") SYS_WRITE => r0,
        in("a0") fd,
        in("a1") buf,
        in("a2") count,
        options(nostack, preserves_flags)
    );
    r0
}

// #[doc = include_str!("./doc/Sys_nanosleep.md")]
// #[cfg_attr(
//     feature = "nightly",
//     doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
// )]
// pub unsafe fn sys_nanosleep(req: *const SysTimeSpec, rem: *mut SysTimeSpec) -> isize {
//     // const SYS_NANOSLEEP: isize = ???; // TODO
//     let r0;
//     asm!(
//         "ecall",
//         inlateout("a7") SYS_NANOSLEEP => r0,
//         in("a0") fd,
//         in("a1") buf,
//         lateout("a2") _,
//         options(nostack, preserves_flags)
//     );
//     r0
// }

#[doc = include_str!("./doc/Sys_ioctl.md")]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(all(target_os = "linux", feature = "unsafe_os")))
)]
pub unsafe fn sys_ioctl(fd: c_int, request: c_ulong, argp: *mut u8) -> isize {
    const SYS_IOCTL: isize = 54;
    let r0;
    asm!(
        "ecall",
        inlateout("a7") SYS_IOCTL => r0,
        in("a0") fd,
        in("a1") request,
        in("a2") argp,
        options(nostack, preserves_flags)
    );
    r0
}
