//
//! ESP32-C6 boards.
//

crate::mods_in! {
    #[cfg(feature = "waveshare_c6_touch_lcd147")]
    mod touch_lcd147;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "waveshare_c6_touch_lcd147")]
        pub use super::touch_lcd147::BoardWaveshareC6TouchLcd147;
    }
}
