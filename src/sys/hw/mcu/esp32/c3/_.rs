// devela/sys/hw/mcu/esp32/c3/_.rs
//
//! ESP32-C3 microcontroller.
//

crate::mods_in! {
    mod direct_boot;
    mod namespace;
    mod pin;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            namespace::McuEsp32C3,
            pin::Esp32C3Pin,
        };
        #[crate::macro_apply(crate::__cfg_item_unsafe_show("safe_sys", "unsafe_mmio"))]
        pub use super::direct_boot::esp32_c3_direct_boot;
    }
}
