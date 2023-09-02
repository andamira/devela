// devela::io::linux::x86
//
//!
//

use core::arch::asm;

pub unsafe fn exit(status: i32) -> ! {
    const SYS_EXIT: isize = 1;
    unsafe {
        asm!(
            "int 0x80",
            in("eax") SYS_EXIT,
            in("ebx") status,
            options(noreturn)
        );
    }
}

pub unsafe fn read(fd: i32, buf: *mut u8, count: usize) -> isize {
    const SYS_READ: isize = 3;
    let r0;
    asm!(
        "int 0x80",
        inlateout("eax") SYS_READ => r0,
        in("ebx") fd,
        in("ecx") buf,
        in("edx") count,
        options(nostack, preserves_flags)
    );
    r0
}

pub unsafe fn write(fd: i32, buf: *const u8, count: usize) -> isize {
    const SYS_WRITE: isize = 4;
    let r0;
    asm!(
        "int 0x80",
        inlateout("eax") SYS_WRITE => r0,
        in("ebx") fd,
        in("ecx") buf,
        in("edx") count,
        options(nostack, preserves_flags)
    );
    r0
}
