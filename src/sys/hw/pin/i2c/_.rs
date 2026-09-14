// devela/sys/hw/pin/i2c/_.rs
//
//! I²C two-wire bus primitives.
//

crate::mods_in! {
    mod addr;
    // mod bus;
    mod error;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            addr::I2cAddr7,
            // bus::I2cBus,
            error::I2cError,
        };
    }
}
