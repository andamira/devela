#![no_std]
#![no_main]

use devela::{Arch, set_panic_handler};
use devela_micros::{McuEsp32S3 as Mcu, devela};
use xtensa_lx_rt::entry;

set_panic_handler! { loop }

#[entry]
fn main() -> ! {
    unsafe {
        // Flashing and console access use the same USB connection.
        // Wait until we know the host has re-opened the endpoint.
        while !Mcu::USB_SERIAL_JTAG.rx_ready() {}
        Mcu::USB_SERIAL_JTAG.write_bytes_blocking(b"hello from devela on esp32-s3\r\n");
    }
    loop {
        Arch::relax();
    }
}

#[unsafe(no_mangle)]
extern "C" fn __init_data() -> bool {
    false
}

#[unsafe(export_name = "__pre_init")]
#[unsafe(link_section = ".rwtext")]
unsafe extern "C" fn s3_pre_init() {
    unsafe extern "C" {
        fn rom_Cache_Suspend_DCache() -> u32;
        fn Cache_Resume_DCache(param: u32);

        fn rom_config_instruction_cache_mode(cache_size: u32, ways: u8, line_size: u8);

        fn rom_config_data_cache_mode(cache_size: u32, ways: u8, line_size: u8);
    }

    unsafe {
        // 32 KiB, 8-way, 32-byte lines.
        rom_config_instruction_cache_mode(32 * 1024, 8, 32);

        let _ = rom_Cache_Suspend_DCache();

        // 32 KiB, 8-way, 32-byte lines.
        rom_config_data_cache_mode(32 * 1024, 8, 32);

        Cache_Resume_DCache(0);
    }
}

#[repr(C)]
struct EspAppDesc {
    magic_word: u32,
    secure_version: u32,
    reserved1: [u32; 2],

    version: [u8; 32],
    project_name: [u8; 32],
    time: [u8; 16],
    date: [u8; 16],
    idf_ver: [u8; 32],

    app_elf_sha256: [u8; 32],

    min_efuse_blk_rev_full: u16,
    max_efuse_blk_rev_full: u16,
    mmu_page_size: u8,

    reserved3: [u8; 3],
    reserved2: [u32; 18],
}

#[unsafe(export_name = "esp_app_desc")]
#[unsafe(link_section = ".flash.appdesc")]
#[used]
static ESP_APP_DESC: EspAppDesc = EspAppDesc {
    magic_word: 0xABCD_5432,
    secure_version: 0,
    reserved1: [0; 2],

    version: [0; 32],
    project_name: [0; 32],
    time: [0; 16],
    date: [0; 16],
    idf_ver: [0; 32],

    app_elf_sha256: [0; 32],

    min_efuse_blk_rev_full: 0,
    max_efuse_blk_rev_full: u16::MAX,

    // log2(64 KiB)
    mmu_page_size: 16,

    reserved3: [0; 3],
    reserved2: [0; 18],
};
