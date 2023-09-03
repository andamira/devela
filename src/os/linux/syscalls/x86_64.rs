// devela::io::linux::x86_64
//
//!
//

use super::timespec;
use core::arch::asm;

pub unsafe fn exit(status: i32) -> ! {
    const SYS_EXIT: isize = 60;
    unsafe {
        asm!(
            "syscall",
            in("rax") SYS_EXIT,
            in("rdi") status,
            options(noreturn)
        );
    }
}

pub unsafe fn read(fd: i32, buf: *mut u8, count: usize) -> isize {
    const SYS_READ: isize = 0;
    let r0;
    asm!(
        "syscall",
        inlateout("rax") SYS_READ => r0,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") count,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

pub unsafe fn write(fd: i32, buf: *const u8, count: usize) -> isize {
    const SYS_WRITE: isize = 1;
    let r0;
    asm!(
        "syscall",
        inlateout("rax") SYS_WRITE => r0,
        in("rdi") fd,
        in("rsi") buf,
        in("rdx") count,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}

pub unsafe fn nanosleep(req: *const timespec, rem: *mut timespec) -> isize {
    const SYS_NANOSLEEP: isize = 35;
    let r0;
    asm!(
        "syscall",
        inlateout("rax") SYS_NANOSLEEP => r0,
        in("rdi") req,
        in("rsi") rem,
        lateout("rcx") _,
        lateout("r11") _,
        options(nostack, preserves_flags)
    );
    r0
}
