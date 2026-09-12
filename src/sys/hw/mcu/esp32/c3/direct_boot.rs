// devela/src/sys/hw/mcu/esp32/c3/direct_boot.rs
//
//! ESP32-C3 direct-boot startup support.
//

#[doc = crate::_tags!(hw code)]
/// Defines the minimal ESP32-C3 ROM direct-boot entry point.
#[doc = crate::_doc_meta!{
    location("sys/hw/mcu/esp32", macro esp32_c3_direct_boot),
}]
/// Use this together with devela's `esp32_c3_direct_boot.x` linker script.
/// The script is made available to the linker when targeting `riscv32imc-unknown-none-elf`.
///
/// Select it with:
/// `-C link-arg=-Tesp32_c3_direct_boot.x`.
///
/// The script provides the memory layout and symbols required by the startup
/// sequence. The supplied Rust function is entered after the stack and global
/// pointer are established, `.data` is copied to RAM, and `.bss` is cleared.
#[macro_export]
macro_rules! esp32_c3_direct_boot· {
    ($main:ident) => {
        #[cfg(not(target_arch = "riscv32"))]
        compile_error!("ESP32-C3 direct boot requires a RISC-V 32-bit target");

        #[cfg(target_arch = "riscv32")]
        $crate::global_asm!(
            r#"
            .section .text.entry,"ax",@progbits
            .global _start
            .type _start, @function

        _start:
            .option push
            .option norelax
            la      gp, __global_pointer$
            .option pop

            la      sp, __stack_top

            /* Copy initialized .data from flash into RAM. */
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

            /* Clear .bss. */
        2:
            la      t0, __bss_start
            la      t1, __bss_end
        3:
            beq     t0, t1, 4f
            sb      zero, 0(t0)
            addi    t0, t0, 1
            j       3b

        4:
            call    {main}

        5:
            j       5b

            .size _start, . - _start
            "#,
            main = sym $main,
        );
    };
}
#[doc(inline)]
pub use esp32_c3_direct_boot· as esp32_c3_direct_boot;
