//
//! AVR Atmega168 boards.
//

crate::mods_in! {
    #[cfg(feature = "arduino_diecimila")]
    mod arduino_diecimila;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "arduino_diecimila")]
        pub use super::arduino_diecimila::BoardArduinoDiecimila;
    }
}
