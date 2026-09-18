//
#![doc = crate::_DOC_MCU_ESP32!()] // public
#![doc = crate::_doc!(modules: crate::mcu; esp32)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg(feature = "esp32c3")]
    mod_ c3;
    #[cfg(feature = "esp32s3")]
    mod_ s3;

    mod i2c;
    mod register;
    mod usb_serial_jtag;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            i2c::EspI2c,
            register::EspReg32,
            usb_serial_jtag::EspUsbSerialJtag,
        };
        #[cfg(feature = "esp32c3")]
        pub use super::c3::_all::*;
        #[cfg(feature = "esp32s3")]
        pub use super::s3::_all::*;
    }
}
