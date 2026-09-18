//
//! ESP32-C3 microcontroller.
//

crate::mods_in! {
    mod direct_boot;
    mod namespace;
    mod pin;
    mod uart;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            namespace::McuEsp32C3,
            pin::Esp32C3Pin,
            uart::Esp32C3Uart,
        };
        #[cfg(feature = "unsafe_mmio")]
        pub use super::direct_boot::esp32_c3_direct_boot;
    }
}
