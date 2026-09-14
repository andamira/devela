// devela/sys/hw/mcu/esp32/_.rs
//
//! Espressif microcontrollers.
//

crate::mods_in! {
    mod_ c3;
    mod i2c;
    mod register;
    mod usb_serial_jtag;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            c3::_all::*,
            i2c::EspI2c,
            register::EspReg32,
            usb_serial_jtag::EspUsbSerialJtag,
        };
    }
}
