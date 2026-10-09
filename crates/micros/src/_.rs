//
//! Microcontroller, microcomputer, board, and embedded hardware support.
//

// environment
#![no_std]
// safety
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
// nightly
#![cfg_attr(nightly_doc, doc(test(attr(feature(doc_cfg)))))] // enable for all doctests
#![cfg_attr(nightly_doc, feature(doc_cfg, doc_notable_trait))]
#![cfg_attr(all(target_arch = "avr", feature = "unsafe_hint"), feature(asm_experimental_arch))]
#![cfg_attr(all(target_arch = "z80", feature = "unsafe_hint"), feature(asm_experimental_arch))]
//

/* imports */

extern crate self as devela_micros;

use ::devela::__hidden::*;
use ::devela::all::*;

crate::mods_in! {
    #[cfg_attr(not(nightly_doc), cfg(feature = "board"))]
    pub mod_ board;
    #[cfg_attr(not(nightly_doc), cfg(feature = "computer"))]
    pub mod_ computer;
    #[cfg_attr(not(nightly_doc), cfg(feature = "device"))]
    pub mod_ device;
    #[cfg_attr(not(nightly_doc), cfg(feature = "mcu"))]
    pub mod_ mcu;
    #[cfg_attr(not(nightly_doc), cfg(feature = "processor"))]
    pub mod_ processor;

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
    #[cfg_attr(not(nightly_doc), cfg(feature = "computer"))]
    #[doc = concat![crate::_DOC_COMPUTER!(), _COMMON_DOC!("computer")]]
    pub mod _computer {
        #[allow(unused_imports)]
        pub use crate::computer::_all::*;
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
    #[cfg_attr(not(nightly_doc), cfg(feature = "processor"))]
    #[doc = concat![crate::_DOC_PROCESSOR!(), _COMMON_DOC!("processor")]]
    pub mod _processor {
        #[allow(unused_imports)]
        pub use crate::processor::_all::*;
    }
}
#[doc = crate::_DOC_ALL!()]
pub mod all {
    #[allow(unused_imports)]
    pub use crate::_all::*;
}

#[doc(hidden)]
pub use _devela as devela;
/// Integrated devela vocabulary available through this crate.
///
/// Combines this crate's public items
/// with the corresponding `devela` vocabulary in one namespace.
pub mod _devela {
    #[allow(unused_imports)]
    pub use crate::all::*;
    pub use ::devela::all::*;
}

crate::mods_out! { // _pub_mods, _reexports, _crate_internals
    _pub_mods {
        #[cfg_attr(not(nightly_doc), cfg(feature = "board"))]
        pub use super::board::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "computer"))]
        pub use super::computer::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "device"))]
        pub use super::device::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "mcu"))]
        pub use super::mcu::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "processor"))]
        pub use super::processor::_all::*;
    }
    _reexports {
        #[doc(inline)] #[cfg(feature = "arduino_nano")]
        pub use super::board::BoardArduinoNano;
        //
        #[doc(inline)] #[cfg(feature = "spectrum48")]
        pub use super::computer::ComputerSpectrum48;
        //
        #[doc(inline)] #[cfg(feature = "ssd13xx")]
        pub use super::device::Ssd13xx;
        //
        #[doc(inline)] #[cfg(feature = "atmega328p")]
        pub use super::mcu::McuAtmega328p;
        #[doc(inline)] #[cfg(feature = "esp32c3")]
        pub use super::mcu::McuEsp32C3;
        #[doc(inline)] #[cfg(feature = "nrf52840")]
        pub use super::mcu::McuNrf52840;
        //
        #[doc(inline)] #[cfg(feature = "z80")]
        pub use super::processor::ProcessorZ80;
    }
    _crate_internals {
        pub use super::{
            yard::_crate_internals::*,
        };
    }
}
