//
#![doc = crate::_DOC_BOARD_ESP32!()] // public
#![doc = crate::_doc!(modules: crate::board; esp32)]
#![doc = crate::_doc!(flat:"board")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg_attr(not(nightly_doc), cfg(feature = "esp32c3"))]
    mod_ c3;
    #[cfg_attr(not(nightly_doc), cfg(feature = "esp32c6"))]
    mod_ c6;
    #[cfg_attr(not(nightly_doc), cfg(feature = "esp32s3"))]
    mod_ s3;
}
crate::mods_out! { // _mods, _reexports
    _mods {
        #[cfg_attr(not(nightly_doc), cfg(feature = "esp32c3"))]
        pub use super::c3::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "esp32c6"))]
        pub use super::c6::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "esp32s3"))]
        pub use super::s3::_all::*;
    }
    _reexports {
        #[cfg(feature = "supermini_oled042")]
        pub use super::c3::_all::BoardSuperMiniOled042;
        // #[cfg(feature = "c6_touch_lcd147")]
        // pub use super::c6::_all::BoardWaveshareC6TouchLcd147;
    }
}
