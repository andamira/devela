//
#![doc = crate::_DOC_SYS_HW_PIN!()] // public
#![doc = crate::_doc!(modules: crate::sys::hw; pin)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod_ gpio; // gpio  = direct pin control
    mod_ i2c; // I²C two-wire bus primitives.
    // mod_ spi; // SPI pin-level synchronous bus
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // gpio::_all::*,
            i2c::_all::*,
            // spi::_all::*,
        };
    }
}
