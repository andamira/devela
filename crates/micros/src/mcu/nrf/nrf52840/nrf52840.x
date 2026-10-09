/* Standalone nRF52840 firmware, without SoftDevice or resident bootloader. */
ENTRY(__devela_nrf52840_reset)
EXTERN(__devela_nrf52840_vectors)

MEMORY
{
    FLASH (rx)  : ORIGIN = 0x00000000, LENGTH = 1024K
    RAM   (rwx) : ORIGIN = 0x20000000, LENGTH = 256K
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
