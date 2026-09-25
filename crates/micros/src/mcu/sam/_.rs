//
#![doc = crate::_DOC_MCU_SAM!()] // public
#![doc = crate::_doc!(modules: crate::mcu; sam)]
#![doc = crate::_doc!(flat:"mcu")]
#![doc = crate::_doc!(hr)]
//!
//! SAM identifies the silicon family; supported devices retain their exact
//! device identity. The SAM3X8E uses an Arm Cortex-M3 processor core.
//

crate::mods_in! {
    #[cfg(feature = "sam3x8e")]
    mod_ sam3x8e;

    #[cfg(feature = "sam")]
    mod pin;
    #[cfg(feature = "sam")]
    mod port;
    #[cfg(feature = "sam")]
    mod register;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "sam")]
        pub use super::{
            pin::SamPin,
            port::SamPort,
            register::SamReg32,
        };
        #[cfg(feature = "sam3x8e")]
        pub use super::sam3x8e::_all::*;
    }
}
