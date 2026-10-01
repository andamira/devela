/* devela/examples/hw/mcu/esp32/s3_bringup/memory.x */

/* Minimal ESP32-S3 memory map for initial devela bring-up.
 *
 * This is intentionally conservative rather than a final linker layout.
 */

MEMORY
{
    /*
     * ESP32-S3 internal instruction RAM.
     *
     * 0x40370000..0x40378000 is reserved here for the 32 KiB I-cache.
     * Xtensa exception vectors occupy the next 0x400 bytes.
     */
    vectors_seg (RX) : ORIGIN = 0x40378000, LENGTH = 0x400

    /*
     * Reserve a small, dedicated IRAM window for xtensa-lx-rt startup/runtime.
     * Ends at 0x40380000.
     */
    iram_seg (RX) : ORIGIN = 0x40378400, LENGTH = 0x7C00

    /*
     * The IRAM and DRAM views overlap physically on the S3.
     *
     * Correspondingly reserve 0x3FC88000..0x3FC90000 on the DRAM side,
     * so our .data/.bss cannot overlap vectors/.rwtext during bring-up.
     */
    dram_seg (RW) : ORIGIN = 0x3FC90000, LENGTH = 0x4B700

    /*
     * Flash-mapped read-only data.
     *
     * Keep the first mapping within one 64 KiB page for this tiny program.
     */
    drom_seg (R) : ORIGIN = 0x3C000020, LENGTH = 0xFFE0

    /*
     * Flash-mapped executable code.
     *
     * Start one 64 KiB mapping page after DROM so the two mappings cannot
     * collide in the application image while we keep the linker simple.
     */
    irom_seg (RX) : ORIGIN = 0x42010020, LENGTH = 0xFFFE0
}

REGION_ALIAS("ROTEXT", irom_seg);
REGION_ALIAS("RODATA", drom_seg);
REGION_ALIAS("RWTEXT", iram_seg);
REGION_ALIAS("RWDATA", dram_seg);

/* The ESP-IDF second-stage bootloader expects the application
   descriptor at the beginning of the first DROM segment. */
SECTIONS
{
    .flash.appdesc : ALIGN(4)
    {
        KEEP(*(.flash.appdesc));
        KEEP(*(.flash.appdesc.*));
    } > RODATA
}

/* xtensa-lx-rt loads this symbol into SP during Reset. */
PROVIDE(_stack_start_cpu0 = ORIGIN(dram_seg) + LENGTH(dram_seg));

INCLUDE rom.x
