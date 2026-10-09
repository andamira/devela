//
#![doc = crate::_DOC_MCU_NRF!()]
#![doc = crate::_doc!(modules: crate::mcu; nrf)]
#![doc = crate::_doc!(flat:"mcu")]
#![doc = crate::_doc!(hr)]
//!
//! Nordic nRF silicon foundations. Board wiring belongs in `board::nrf`.
//

crate::mods_in! {
    #[cfg(feature = "nrf52840")]
    mod_ nrf52840;

    #[cfg(feature = "nrf")]
    mod pin;
    #[cfg(feature = "nrf")]
    mod port;
    #[cfg(feature = "nrf")]
    mod register;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "nrf")]
        pub use super::{
            pin::NrfPin,
            port::NrfPort,
            register::NrfReg32,
        };
        #[cfg(feature = "nrf52840")]
        pub use super::nrf52840::_all::*;
    }
}
