// devela::run::time::driver
//
//!
//

mod driver; // RunDriver
mod error; // RunDriverError, RunDriverFrameError

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            driver::*,
            error::*,
        };
    }
}
