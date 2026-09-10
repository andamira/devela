// devela/sys/hw/mcu/_.rs
//
#![doc = crate::_DOC_SYS_HW_MCU!()] // public
#![doc = crate::_doc!(modules: crate::sys::hw; mcu: avr, board)] // esp32
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//!
//! This module contains chip-specific foundations
//! for programs that execute directly on microcontrollers.
//!
//! MCU support distinguishes:
//! - processor architecture from concrete silicon,
//! - silicon peripherals from development-board wiring,
//! - target/compiler support from runtime support,
//! - host-side build and flashing tools from firmware dependencies.
//

crate::mods_in! {
    pub mod_ avr;
    pub mod_ board;
    // pub mod_ esp32;
}
crate::mods_out! { // _pub_mods, _reexports
    _pub_mods {
        pub use super::{
            avr::_all::*,
            board::_all::*,
            // esp32::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            avr::{Atmega328p, AvrPort, AvrReg8, AvrUsart},
            // board::{},
            // esp32::{Esp32C3, EspReg32},
        };
    }
}
