//
//! AVR Atmega2560 boards.
//

crate::mods_in! {
    #[cfg(feature = "arduino_mega2560")]
    mod arduino_mega2560;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "arduino_mega2560")]
        pub use super::arduino_mega2560::BoardArduinoMega2560;
    }
}
