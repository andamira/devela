OUTPUT_ARCH(riscv)
ENTRY(_start)

MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 16K
    RAM   : ORIGIN = 0x20000000, LENGTH = 2K
}

_stack_top = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
    .init :
    {
        KEEP(*(.init))
    } > FLASH

    .text :
    {
        *(.text .text.*)
    } > FLASH

    .rodata :
    {
        *(.rodata .rodata.*)
    } > FLASH

    .data :
    {
        *(.sdata .sdata.*)
        *(.data .data.*)
    } > RAM AT > FLASH

    .bss (NOLOAD) :
    {
        *(.sbss .sbss.*)
        *(.bss .bss.*)
        *(COMMON)
    } > RAM
}
