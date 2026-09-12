// devela/sys/hw/mcu/esp32/_.rs
//
//! Espressif microcontrollers.
//

crate::mods_in! {
    mod_ c3;
    mod register;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            c3::_all::*,
            register::EspReg32,
        };
    }
}
