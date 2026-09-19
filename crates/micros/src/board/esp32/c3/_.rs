//
//! ESP32-C3 boards.
//

crate::mods_in! {
    #[cfg(feature = "supermini_oled042")]
    mod supermini_oled042;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "supermini_oled042")]
        pub use super::supermini_oled042::BoardSuperMiniOled042;
    }
}
