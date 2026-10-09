/* Adafruit UF2 + Nordic S140 v6 application (e.g. nice!nano).
 * This is NOT a standalone image: the MBR/SoftDevice and bootloader remain intact.
 * Application may occupy [0x26000, 0xEC000). Keep the app-data and bootloader
 * regions above that untouched. The lower 16 KiB of RAM is reserved.
 * Verify the connected bootloader/SoftDevice version before flashing.
 */
ENTRY(__devela_nrf52840_reset)
EXTERN(__devela_nrf52840_vectors)

MEMORY
{
    FLASH (rx)  : ORIGIN = 0x00026000, LENGTH = 0x000C6000
    RAM   (rwx) : ORIGIN = 0x20004000, LENGTH = 0x0003C000
}

__stack_top = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
    .vectors ORIGIN(FLASH) :
    {
        . = ALIGN(256);
        KEEP(*(.vectors))
    } > FLASH

    .text :
    {
        . = ALIGN(4);
        *(.text .text.*)
        *(.rodata .rodata.*)
        . = ALIGN(4);
    } > FLASH

    /DISCARD/ :
    {
        *(.ARM.exidx .ARM.exidx.*)
        *(.ARM.extab .ARM.extab.*)
    }

    .data :
    {
        . = ALIGN(4);
        __sdata = .;
        *(.data .data.*)
        . = ALIGN(4);
        __edata = .;
    } > RAM AT > FLASH
    __sidata = LOADADDR(.data);

    .bss (NOLOAD) :
    {
        . = ALIGN(4);
        __sbss = .;
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4);
        __ebss = .;
    } > RAM
}
ASSERT(SIZEOF(.vectors) == 256, "nRF52840 vector table must have 64 entries")
