//
#![doc = crate::_DOC_BOARD_AVR!()] // public
#![doc = crate::_doc!(modules: crate::board; avr)]
#![doc = crate::_doc!(flat:"board")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg_attr(not(nightly_doc), cfg(feature = "atmega2560"))]
    mod_ atmega2560;
    #[cfg_attr(not(nightly_doc), cfg(feature = "atmega328p"))]
    mod_ atmega328p;
}
crate::mods_out! { // _mods, _reexports
    _mods {
        #[cfg_attr(not(nightly_doc), cfg(feature = "atmega2560"))]
        pub use super::atmega2560::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "atmega328p"))]
        pub use super::atmega328p::_all::*;
    }
    _reexports {
        #[doc(inline)]
        #[cfg(feature = "arduino_mega2560")]
        pub use super::atmega2560::BoardArduinoMega2560;
        #[doc(inline)]
        #[cfg(feature = "arduino_nano")]
        pub use super::atmega328p::BoardArduinoNano;
    }
}
