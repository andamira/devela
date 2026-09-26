ENTRY(__devela_sam3x8e_reset)
EXTERN(__devela_sam3x8e_vectors)

MEMORY
{
    FLASH (rx)  : ORIGIN = 0x00080000, LENGTH = 512K
    RAM   (rwx) : ORIGIN = 0x20070000, LENGTH = 96K
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

    /* Bare-metal firmware does not unwind; discard Arm unwind metadata. */
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
