/* ESP32-C6 direct-boot memory layout. */

ENTRY(_start)

MEMORY
{
    /*
     * External flash mapping used by ROM direct boot.
     *
     * Keep the generic MCU image inside the first 4 MiB. Concrete C6 variants
     * may provide more flash; board-specific storage support can expose it
     * independently of this minimal executable mapping.
     */
    ROM (rx) : ORIGIN = 0x42000000, LENGTH = 0x400000

    /*
     * 512 KiB HP SRAM, with the first 32 KiB reserved for the flash cache.
     * ESP32-C6 maps instruction and data access through the same HP SRAM range.
     */
    RAM (rw) : ORIGIN = 0x40808000, LENGTH = 0x78000
}

SECTIONS
{
    /*
     * ESP32-C6 direct-boot magic.
     *
     * ROM recognizes these two words at flash offset zero and starts executing
     * the mapped image immediately after the 8-byte header.
     */
    .header ORIGIN(ROM) : AT(0)
    {
        LONG(0xaedb041d)
        LONG(0xaedb041d)
    } > ROM

    .text ORIGIN(ROM) + 8 : AT(8)
    {
        KEEP(*(.text.entry))
        *(.text .text.*)

        . = ALIGN(16);
    } > ROM

    __flash_after_text = LOADADDR(.text) + SIZEOF(.text);

    .rodata ORIGIN(ROM) + __flash_after_text : AT(__flash_after_text)
    {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        *(.data.rel.ro .data.rel.ro.*)

        . = ALIGN(16);
    } > ROM

    __flash_after_rodata = LOADADDR(.rodata) + SIZEOF(.rodata);

    .data ORIGIN(RAM) : AT(__flash_after_rodata)
    {
        __data_start = .;

        *(.data .data.*)

        __sdata_start = .;
        *(.sdata .sdata.*)

        *(.got .got.*)

        __data_end = .;
    } > RAM

    __data_load = ORIGIN(ROM) + LOADADDR(.data);
    __flash_after_data = LOADADDR(.data) + SIZEOF(.data);

    .bss (NOLOAD) :
    {
        __bss_start = .;

        *(.sbss .sbss.*)
        *(.bss .bss.*)
        *(COMMON)

        __bss_end = .;
    } > RAM

    /* RISC-V ABI global pointer. */
    __global_pointer$ = MIN(__sdata_start + 0x800, MAX(__data_start + 0x800, __bss_end - 0x800));

    /* The RISC-V ABI requires a suitably aligned stack; RAM ends aligned. */
    __stack_top = ORIGIN(RAM) + LENGTH(RAM);

    ASSERT(__flash_after_data <= LENGTH(ROM), "direct-boot image exceeds mapped flash")

    ASSERT(__bss_end + 0x4000 <= __stack_top, "less than 16 KiB remain for stack")

    /DISCARD/ :
    {
        *(.eh_frame .eh_frame.*)
        *(.eh_frame_hdr)
        *(.comment)
        *(.note .note.*)
    }
}
