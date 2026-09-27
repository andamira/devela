//
//! ESP32-C6 microcontroller.
//

crate::mods_in! {
    mod direct_boot;
    mod mcu;
    mod pin;
    #[cfg(feature = "unsafe_mmio")]
    mod spi_cmd_data;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            mcu::McuEsp32C6,
            pin::Esp32C6Pin,
        };
        #[cfg(feature = "unsafe_mmio")]
        pub use super::spi_cmd_data::Esp32C6SpiCmdData;
        #[cfg(feature = "unsafe_mmio")]
        pub use super::direct_boot::esp32_c6_direct_boot;
    }
}
