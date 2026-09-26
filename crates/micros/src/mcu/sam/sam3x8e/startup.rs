//
//! Minimal SAM3X8E startup.
//

use core::{arch::global_asm, ptr};

global_asm!(
    r#"
    .section .vectors, "a", %progbits
    .balign 256
    .global __devela_sam3x8e_vectors
    .type __devela_sam3x8e_vectors, %object

__devela_sam3x8e_vectors:
    .word __stack_top
    .word __devela_sam3x8e_reset

    /* 14 remaining Cortex-M vectors + 45 SAM3X8E IRQs. */
    .rept 59
    .word __devela_sam3x8e_default_handler
    .endr

    .size __devela_sam3x8e_vectors, . - __devela_sam3x8e_vectors
"#
);

unsafe extern "C" {
    static __sidata: u32;

    static mut __sdata: u32;
    static mut __edata: u32;

    static mut __sbss: u32;
    static mut __ebss: u32;

    fn main() -> !;
}

#[unsafe(no_mangle)]
unsafe extern "C" fn __devela_sam3x8e_reset() -> ! {
    // Disable the watchdog. It starts enabled after reset.
    unsafe {
        ptr::write_volatile(0x400E_1A54 as *mut u32, 1 << 15);
    }

    // Initialize .data.
    unsafe {
        let mut src = ptr::addr_of!(__sidata);
        let mut dst = ptr::addr_of_mut!(__sdata);
        let end = ptr::addr_of_mut!(__edata);

        // Use volatile accesses to keep these startup loops explicit.
        // Ordinary reads/writes may be lowered to memcpy/memset compiler builtins.
        while (dst as usize) < (end as usize) {
            ptr::write_volatile(dst, ptr::read_volatile(src));
            src = src.add(1);
            dst = dst.add(1);
        }
    }

    // Initialize .bss.
    unsafe {
        let mut dst = ptr::addr_of_mut!(__sbss);
        let end = ptr::addr_of_mut!(__ebss);

        while (dst as usize) < (end as usize) {
            ptr::write_volatile(dst, 0);
            dst = dst.add(1);
        }
    }

    // Point VTOR directly at the application vector table in Flash.
    unsafe {
        ptr::write_volatile(0xE000_ED08 as *mut u32, 0x0008_0000);
    }

    unsafe { main() }
}

#[unsafe(no_mangle)]
extern "C" fn __devela_sam3x8e_default_handler() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
