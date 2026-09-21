//
//! I²C two-wire bus primitives.
//

crate::mods_in! {
    mod addr;
    mod control;
    mod error;
    mod write;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            addr::I2cAddr7,
            control::{I2cControl, I2cController},
            error::I2cError,
            write::I2cWrite,
        };
    }
}
