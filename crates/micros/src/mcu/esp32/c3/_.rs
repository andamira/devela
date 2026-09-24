//
//! ESP32-C3 microcontroller.
//

crate::mods_in! {
    mod direct_boot;
    mod mcu;
    mod pin;
    mod random;
    mod systimer;
    mod uart;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            mcu::McuEsp32C3,
            pin::Esp32C3Pin,
            random::Esp32C3Rng, // Esp32C3Entropy
            systimer::Esp32C3SystemTimer,
            uart::Esp32C3Uart,
        };
        #[cfg(feature = "unsafe_mmio")]
        pub use super::direct_boot::esp32_c3_direct_boot;
    }
}
