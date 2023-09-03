// devela::io::linux::riscv
//
//! Both for riscv32 and riscv64
//

use core::arch::asm;

pub unsafe fn exit(status: i32) -> ! {
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

pub unsafe fn read(fd: i32, buf: *mut u8, count: usize) -> isize {
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

pub unsafe fn write(fd: i32, buf: *const u8, count: usize) -> isize {
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

// pub unsafe fn nanosleep(req: *const timespec, rem: *mut timespec) -> isize {
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
