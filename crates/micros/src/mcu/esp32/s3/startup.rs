//
//! ESP32-S3 startup support for `xtensa-lx-rt`.
//

#[doc = crate::_tags!(hw code)]
/// Defines the ESP32-S3 startup hooks used with `xtensa-lx-rt`.
#[doc = crate::_doc_meta!{
    location("mcu/esp32/s3", macro esp32_s3_startup),
}]
/// This packages the startup support required by devela's current ESP32-S3
/// linker layout while leaving reset-vector and exception handling to `xtensa-lx-rt`.
///
/// Invoke it once in the application crate
/// together with an `xtensa_lx_rt::entry` entry point.
///
/// It defines:
///
/// - `__init_data`, preventing the runtime from reloading program data,
/// - `__pre_init`, configuring the ESP32-S3 instruction and data caches,
/// - `esp_app_desc`, the application descriptor expected by the ESP boot image.
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
#[cfg(feature = "unsafe_mmio")]
macro_rules! esp32_s3_startup· {
    () => {
        #[cfg(not(target_arch = "xtensa"))]
        compile_error!("ESP32-S3 startup requires an Xtensa target");

        #[cfg(target_arch = "xtensa")]
        #[unsafe(export_name = "__init_data")]
        extern "C" fn __devela_esp32_s3_init_data() -> bool {
            false
        }

        #[cfg(target_arch = "xtensa")]
        #[unsafe(export_name = "__pre_init")]
        #[unsafe(link_section = ".rwtext")]
        unsafe extern "C" fn __devela_esp32_s3_pre_init() {
            #[allow(non_snake_case)]
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

        #[cfg(target_arch = "xtensa")]
        #[repr(C)]
        #[allow(dead_code)]
        struct __DevelaEsp32S3AppDesc {
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

        #[cfg(target_arch = "xtensa")]
        #[unsafe(export_name = "esp_app_desc")]
        #[unsafe(link_section = ".flash.appdesc")]
        #[used]
        static __DEVELA_ESP32_S3_APP_DESC: __DevelaEsp32S3AppDesc = __DevelaEsp32S3AppDesc {
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
    };
}

#[cfg(feature = "unsafe_mmio")]
#[doc(inline)]
pub use esp32_s3_startup· as esp32_s3_startup;
