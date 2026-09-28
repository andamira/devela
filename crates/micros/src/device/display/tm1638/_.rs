//
//! TM1638 LED display and key-scan controller support.
//!
//! Includes the controller and display-RAM model, the common 8-digit
//! "LED & KEY" module profile, and an AVR bit-banged interface.
//

crate::mods_in! {
    mod bus; // IMPROVE: generalize transaction I/O
    mod device;
    mod frame;
    #[cfg(all(feature = "avr", target_arch = "avr"))]
    mod avr;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            bus::Tm1638Bus,
            device::{Tm1638, Tm1638Brightness},
            frame::{Tm1638Frame, Tm1638LedKey8},
        };
        #[cfg(all(feature = "avr", target_arch = "avr"))]
        pub use super::avr::Tm1638AvrBus;
    }
}
