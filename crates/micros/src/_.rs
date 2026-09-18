//
//! Microcontroller, board, and embedded hardware support for devela.
//

#![no_std]
#![cfg_attr(nightly_doc, feature(doc_cfg))]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]

/* imports */

extern crate self as devela_micros;
macro_rules! __crate_name {
    () => {
        "devela_micros"
    };
}
#[allow(unused_imports)]
pub(crate) use __crate_name;

use ::devela::__hidden::*;
use ::devela::all::*;

crate::mods_in! {
    #[cfg(feature = "board")]
    pub mod_ board;
    #[cfg(feature = "device")]
    mod_ device;
    #[cfg(feature = "mcu")]
    pub mod_ mcu;

    // internal
    pub mod_ yard; // Scaffolding, taxonomy, and documentation support.
}

/// All public devela_micros items in one flat namespace.
pub mod all {
    #[allow(unused_imports)]
    pub use crate::_all::*;
}
#[doc(hidden)]
/// Integrated devela vocabulary available through this crate.
pub mod devela {
    #[allow(unused_imports)]
    pub use crate::all::*;
    pub use ::devela::all::*;
}

crate::mods_out! { // _pub_mods, _crate_internals
    _pub_mods {
        #[cfg(feature = "board")]
        pub use super::board::_all::*;
        #[cfg(feature = "device")]
        pub use super::device::_all::*;
        #[cfg(feature = "mcu")]
        pub use super::mcu::_all::*;
    }
    _crate_internals {
        pub use super::{
            yard::_crate_internals::*,
        };
    }
}
