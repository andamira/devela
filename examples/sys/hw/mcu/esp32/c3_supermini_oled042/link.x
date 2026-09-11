/* devela/examples/sys/hw/mcu/esp32/c3_supermini_oled042/link.x */

/* ESP32-C3 direct-boot memory layout. */

ENTRY(_start)

MEMORY
{
    IROM (rx)  : ORIGIN = 0x42000000, LENGTH = 0x400000
    DROM (r)   : ORIGIN = 0x3c000000, LENGTH = 0x400000
    RAM  (rw)  : ORIGIN = 0x3fc80000, LENGTH = 0x50000
}

SECTIONS
{
    /*
     * ESP32-C3 direct-boot magic.
     *
     * The ROM maps flash at IROM/DROM and jumps to IROM + 8 when
     * these two words are found at flash offset zero.
     */
    .header ORIGIN(IROM) : AT(0)
    {
        LONG(0xaedb041d)
        LONG(0xaedb041d)
    } > IROM

    /* Execution starts immediately after the 8-byte header. */
    .text ORIGIN(IROM) + 8 : AT(8)
    {
        KEEP(*(.text.entry))
        *(.text .text.*)

        . = ALIGN(16);
    } > IROM

    __flash_after_text = LOADADDR(.text) + SIZEOF(.text);

    /*
     * Flash is simultaneously mapped through DROM for data reads.
     * Preserve the same physical flash offset as the preceding text.
     */
    .rodata ORIGIN(DROM) + __flash_after_text : AT(__flash_after_text)
    {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        *(.data.rel.ro .data.rel.ro.*)

        . = ALIGN(16);
    } > DROM

    __flash_after_rodata = LOADADDR(.rodata) + SIZEOF(.rodata);

    /*
     * Writable initialized data lives in RAM, with its initial bytes
     * stored at the next physical flash offset.
     */
    .data ORIGIN(RAM) : AT(__flash_after_rodata)
    {
        __data_start = .;

        *(.data .data.*)

        __sdata_start = .;
        *(.sdata .sdata.*)

        *(.got .got.*)

        __data_end = .;
    } > RAM

    __data_load = ORIGIN(DROM) + LOADADDR(.data);
    __flash_after_data = LOADADDR(.data) + SIZEOF(.data);

    .bss (NOLOAD) :
    {
        __bss_start = .;

        *(.sbss .sbss.*)
        *(.bss .bss.*)
        *(COMMON)

        __bss_end = .;
    } > RAM

    /*
     * RISC-V ABI global pointer. This follows the arrangement used by
     * Espressif's direct-boot reference linker script.
     */
    __global_pointer$ =
        MIN(__sdata_start + 0x800,
            MAX(__data_start + 0x800, __bss_end - 0x800));

    /* The RISC-V ABI requires a suitably aligned stack; RAM ends aligned. */
    __stack_top = ORIGIN(RAM) + LENGTH(RAM);

    ASSERT(__flash_after_data <= LENGTH(IROM),
        "direct-boot image exceeds mapped flash")

    ASSERT(__bss_end + 0x4000 <= __stack_top,
        "less than 16 KiB remain for stack")

    /DISCARD/ :
    {
        *(.eh_frame .eh_frame.*)
        *(.eh_frame_hdr)
        *(.comment)
        *(.note .note.*)
    }
}
