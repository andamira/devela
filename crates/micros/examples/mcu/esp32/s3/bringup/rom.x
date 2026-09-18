/* devela/examples/hw/mcu/esp32/s3_bringup/rom.x */

/* ESP32-S3 ROM functions used during minimal startup. */

PROVIDE(rom_Cache_Suspend_DCache = 0x400018b4);
PROVIDE(Cache_Resume_DCache = 0x400018c0);

PROVIDE(rom_config_instruction_cache_mode = 0x40001a1c);
PROVIDE(rom_config_data_cache_mode = 0x40001a28);
