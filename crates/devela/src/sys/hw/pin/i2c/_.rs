//
//! I²C two-wire bus primitives.
//

crate::mods_in! {
    mod addr;
    #[cfg(all(feature = "unsafe_mmio", not(feature = "safe_sys")))] // FUTURE IMPROVE feat gate
    mod control;
    mod error;
    mod write;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            addr::I2cAddr7,
            error::I2cError,
            write::I2cWrite,
        };
        #[cfg(all(feature = "unsafe_mmio", not(feature = "safe_sys")))]
        pub use super::control::{I2cControl, I2cController};
    }
}
