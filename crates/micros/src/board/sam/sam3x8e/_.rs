//
//! SAM3X8E boards.
//

crate::mods_in! {
    #[cfg(feature = "arduino_due")]
    mod arduino_due;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "arduino_due")]
        pub use super::arduino_due::BoardArduinoDue;
    }
}
