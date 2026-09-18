//
//! Arduino boards.
//

crate::mods_in! {
    #[cfg(feature = "arduino_nano")]
    mod nano;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "arduino_nano")]
        pub use super::nano::BoardArduinoNano;
    }
}
