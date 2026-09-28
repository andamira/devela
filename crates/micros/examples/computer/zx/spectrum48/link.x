ENTRY(_start)

MEMORY
{
    RAM (rwx) : ORIGIN = 0x8000, LENGTH = 0x8000
}

SECTIONS
{
    .text :
    {
        KEEP(*(.text._start))
        *(.text .text.*)
    } > RAM

    .rodata :
    {
        *(.rodata .rodata.*)
    } > RAM

    .data :
    {
        *(.data .data.*)
    } > RAM

    .bss (NOLOAD) :
    {
        *(.bss .bss.*)
        *(COMMON)
    } > RAM

    /DISCARD/ :
    {
        *(.eh_frame*)
        *(.comment*)
    }
}
