// devela::io::linux::arm
//
//!
//

use core::arch::asm;

pub unsafe fn exit(status: i32) -> ! {
    const SYS_EXIT: isize = 1;
    unsafe {
        asm!(
            "swi 0",
            in("r7") SYS_EXIT,
            in("r0") status,
            options(noreturn)
        );
    }
}

pub unsafe fn read(fd: i32, buf: *mut u8, count: usize) -> isize {
    const SYS_READ: isize = 3;
    let r0;
    asm!(
        "swi 0",
        inlateout("r7") SYS_READ => r0,
        in("r0") fd,
        in("r1") buf,
        in("r2") count,
        options(nostack, preserves_flags)
    );
    r0
}

pub unsafe fn write(fd: i32, buf: *const u8, count: usize) -> isize {
    const SYS_WRITE: isize = 4;
    let r0;
    asm!(
        "swi 0",
        inlateout("r7") SYS_WRITE => r0,
        in("r0") fd,
        in("r1") buf,
        in("r2") count,
        options(nostack, preserves_flags)
    );
    r0
}
