//
//!
//

crate::mods_in! {
    mod mcu;
    #[cfg(feature = "time")]
    mod time;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::mcu::McuAtmega328p;
        #[cfg(feature = "time")]
        pub use super::time::{Atmega328pTimer1Clock, Atmega328pTimer1ClockCfg};
    }
}
