//
//! Minimal nRF52840 startup for standalone and bootloader-resident firmware.
//

use core::{
    arch::{asm, global_asm},
    ptr,
};

global_asm!(
    r#"
    .section .vectors, "a", %progbits
    .balign 256
    .global __devela_nrf52840_vectors
    .type __devela_nrf52840_vectors, %object

__devela_nrf52840_vectors:
    .word __stack_top
    .word __devela_nrf52840_reset

    /* Remaining 14 core vectors + IRQ 0..47 = 62 entries. */
    .rept 62
    .word __devela_nrf52840_default_handler
    .endr

    .size __devela_nrf52840_vectors, . - __devela_nrf52840_vectors
"#
);

unsafe extern "C" {
    static __devela_nrf52840_vectors: u32;
    static __sidata: u32;
    static mut __sdata: u32;
    static mut __edata: u32;
    static mut __sbss: u32;
    static mut __ebss: u32;
    fn main() -> !;
}

#[unsafe(no_mangle)]
unsafe extern "C" fn __devela_nrf52840_reset() -> ! {
    // The hard-float target needs CP10/CP11 enabled before user code uses FP.
    unsafe {
        let cpacr = 0xE000_ED88 as *mut u32;
        ptr::write_volatile(cpacr, ptr::read_volatile(cpacr) | (0b1111 << 20));
        asm!("dsb", "isb", options(nostack, preserves_flags));

        // Initialize .data, without calls to compiler-provided memcpy.
        let mut src = ptr::addr_of!(__sidata);
        let mut dst = ptr::addr_of_mut!(__sdata);
        let data_end = ptr::addr_of_mut!(__edata);
        while (dst as usize) < (data_end as usize) {
            ptr::write_volatile(dst, ptr::read_volatile(src));
            src = src.add(1);
            dst = dst.add(1);
        }

        // Initialize .bss, without calls to compiler-provided memset.
        let mut dst = ptr::addr_of_mut!(__sbss);
        let bss_end = ptr::addr_of_mut!(__ebss);
        while (dst as usize) < (bss_end as usize) {
            ptr::write_volatile(dst, 0);
            dst = dst.add(1);
        }

        // VTOR must follow the linked image: 0 for standalone, 0x26000 for UF2.
        ptr::write_volatile(
            0xE000_ED08 as *mut u32,
            ptr::addr_of!(__devela_nrf52840_vectors) as u32,
        );
        asm!("dsb", "isb", options(nostack, preserves_flags));
        main()
    }
}

#[unsafe(no_mangle)]
extern "C" fn __devela_nrf52840_default_handler() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
