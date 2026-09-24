//
//! I²C two-wire bus primitives.
//

crate::mods_in! {
    mod addr;
    #[cfg(all(feature = "unsafe_mmio", not(feature = "safe_sys")))]
    mod control;
    mod cmd_data;
    mod error;
    mod target;
    mod write;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            addr::I2cAddr7,
            cmd_data::I2cCmdData,
            error::I2cError,
            target::I2cTarget,
            write::I2cBusWrite,
        };
        #[cfg(all(feature = "unsafe_mmio", not(feature = "safe_sys")))]
        pub use super::control::{I2cControl, I2cController};
    }
}
