// devela/examples/sys/hw/mcu/esp/c3_supermini_oled042/src/bin/led.rs
//
//! Turns on the board's GPIO8 LED using direct ESP32-C3 MMIO.
//

#![no_std]
#![no_main]

use devela::{BoardSuperMiniOled042, McuEsp32C3, global_asm, set_panic_handler};

/*
 * Entry from the ESP32-C3 ROM direct-boot path.
 *
 * At entry there is not yet a valid Rust execution environment:
 * in particular, Rust code must not run before SP has been established.
 */
global_asm!(
    r#"
    .section .text.entry,"ax",@progbits
    .global _start
    .type _start, @function

_start:
    /* Establish the RISC-V ABI global pointer without linker relaxation. */
    .option push
    .option norelax
    la      gp, __global_pointer$
    .option pop

    /* Establish the stack before entering any Rust function. */
    la      sp, __stack_top

    /* Copy initialized .data from its DROM flash mapping into RAM. */
    la      t0, __data_load
    la      t1, __data_start
    la      t2, __data_end
1:
    beq     t1, t2, 2f
    lbu     t3, 0(t0)
    sb      t3, 0(t1)
    addi    t0, t0, 1
    addi    t1, t1, 1
    j       1b

    /* Zero .bss. */
2:
    la      t0, __bss_start
    la      t1, __bss_end
3:
    beq     t0, t1, 4f
    sb      zero, 0(t0)
    addi    t0, t0, 1
    j       3b

    /* Rust may now run normally. */
4:
    call    main

    /* `main` is divergent, but do not fall through if that changes. */
5:
    j       5b

    .size _start, . - _start
"#
);

set_panic_handler! { loop }

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let led = BoardSuperMiniOled042::LED_MASK;

    unsafe {
        McuEsp32C3::GPIO_OUT_W1TC.write(led);
        McuEsp32C3::GPIO_ENABLE_W1TS.write(led);
    }

    loop {}
}
