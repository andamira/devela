// devela/mcu/esp32/s3/_.rs
//
//! ESP32-S3 microcontroller.
//

crate::mods_in! {
    mod mcu;
    mod pin;
    mod startup;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            mcu::McuEsp32S3,
            pin::Esp32S3Pin,
        };
        #[cfg(feature = "unsafe_mmio")]
        pub use super::startup::esp32_s3_startup;
    }
}
