//
//! AVR Atmega328p boards.
//

crate::mods_in! {
    #[cfg(feature = "arduino_nano")]
    mod arduino_nano;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "arduino_nano")]
        pub use super::arduino_nano::BoardArduinoNano;
    }
}
