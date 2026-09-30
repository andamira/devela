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
        __bss_start = .;

        *(.bss .bss.*)
        *(COMMON)

        __bss_end = .;
    } > RAM

    /DISCARD/ :
    {
        *(.eh_frame*)
        *(.comment*)
    }
}
