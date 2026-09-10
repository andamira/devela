// devela/sys/hw/mcu/_.rs
//
//! Microcontroller units and bare-metal execution substrates.
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
    // pub mod_ esp32;
}
crate::mods_out! { // _pub_mods, _reexports
    _pub_mods {
        pub use super::{
            avr::_all::*,
            // esp32::_all::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            avr::{AvrPort, AvrReg8}, // Atmega328p
            // esp32::{},
        };
    }
}
