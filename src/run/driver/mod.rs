// devela::run::time::driver
//
//!
//

mod driver; // RunDriver
mod error; // RunDriverError, RunDriverFrameError

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            driver::RunDriver,
            error::*,
        };
    }
    _crate_internals {
        pub(crate) use super::driver::_run_driver_step_frame_body;
    }
}
