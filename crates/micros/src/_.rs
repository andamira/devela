//
//! Microcontroller, board, and embedded hardware support for devela.
//

// environment
#![no_std]
// safety
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
// nightly
#![cfg_attr(nightly_doc, doc(test(attr(feature(doc_cfg)))))] // enable for all doctests
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]

/* imports */

extern crate self as devela_micros;

use ::devela::__hidden::*;
use ::devela::all::*;

crate::mods_in! {
    #[cfg_attr(not(nightly_doc), cfg(feature = "board"))]
    pub mod_ board;
    #[cfg_attr(not(nightly_doc), cfg(feature = "device"))]
    pub mod_ device;
    #[cfg_attr(not(nightly_doc), cfg(feature = "mcu"))]
    pub mod_ mcu;

    // internal
    pub mod_ yard; // Scaffolding, taxonomy, and documentation support.
}

#[rustfmt::skip]
#[doc = crate::_DOC_ALL_!()]
#[doc = crate::_doc!(br+hr)] // gives way to the first root module
#[doc = crate::_DOC_ALL_PLUS_!()]
pub mod all_ {
    macro_rules! _COMMON_DOC { ($mod:literal) => { concat!(" ",
        crate::_doc!(root:$mod), "\n\nAll `", $mod, "` module's items flat re-exported.") }; }

    #[cfg_attr(not(nightly_doc), cfg(feature = "board"))]
    #[doc = concat![crate::_DOC_BOARD!(), _COMMON_DOC!("board")]]
    pub mod _board {
        #[allow(unused_imports)]
        pub use crate::board::_all::*;
    }
    #[cfg_attr(not(nightly_doc), cfg(feature = "device"))]
    #[doc = concat![crate::_DOC_DEVICE!(), _COMMON_DOC!("device")]]
    pub mod _device {
        #[allow(unused_imports)]
        pub use crate::device::_all::*;
    }
    #[cfg_attr(not(nightly_doc), cfg(feature = "mcu"))]
    #[doc = concat![crate::_DOC_MCU!(), _COMMON_DOC!("mcu")]]
    pub mod _mcu {
        #[allow(unused_imports)]
        pub use crate::mcu::_all::*;
    }
}
#[doc = crate::_DOC_ALL!()]
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
        #[cfg_attr(not(nightly_doc), cfg(feature = "board"))]
        pub use super::board::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "device"))]
        pub use super::device::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "mcu"))]
        pub use super::mcu::_all::*;
    }
    _crate_internals {
        pub use super::{
            yard::_crate_internals::*,
        };
    }
}
