//
#![doc = crate::_DOC_BOARD_SAM!()] // public
#![doc = crate::_doc!(modules: crate::board; sam)]
#![doc = crate::_doc!(flat:"board")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    #[cfg_attr(not(nightly_doc), cfg(feature = "sam3x8e"))]
    mod_ sam3x8e;
}
crate::mods_out! { // _mods, _reexports
    _mods {
        #[cfg_attr(not(nightly_doc), cfg(feature = "sam3x8e"))]
        pub use super::sam3x8e::_all::*;
    }
    _reexports {
        #[doc(inline)]
        #[cfg(feature = "arduino_due")]
        pub use super::sam3x8e::BoardArduinoDue;
    }
}
