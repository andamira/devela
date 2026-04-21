// devela::lang::prog::ffi::js::time
//
//!
//

mod instant; // JsInstant
mod timeout; // JsTimeout

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            instant::*,
            timeout::*,
        };
    }
}
