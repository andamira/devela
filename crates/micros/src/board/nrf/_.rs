//
#![doc = crate::_DOC_BOARD_NRF!()]
#![doc = crate::_doc!(modules: crate::board; nrf)]
#![doc = crate::_doc!(flat:"board")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg(feature = "nice_nano")]
    mod_ nrf52840;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "nice_nano")]
        pub use super::nrf52840::_all::*;
    }
}
