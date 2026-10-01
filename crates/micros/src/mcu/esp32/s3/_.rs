// devela/mcu/esp32/s3/_.rs
//
//! ESP32-S3 microcontroller.
//

crate::mods_in! {
    mod mcu;
    mod pin;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            mcu::McuEsp32S3,
            pin::Esp32S3Pin,
        };
    }
}
