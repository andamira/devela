// devela/sys/hw/mcu/esp32/c3/_.rs
//
//! ESP32-C3 microcontroller.
//

crate::mods_in! {
    mod direct_boot;
    mod namespace;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            direct_boot::esp32_c3_direct_boot,
            namespace::McuEsp32C3,
        };
    }
}
