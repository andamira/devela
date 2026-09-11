// devela/sys/hw/mcu/avr/_.rs
//
#![doc = crate::_DOC_SYS_HW_MCU_AVR!()] // public
#![doc = crate::_doc!(modules: crate::sys::hw::mcu; avr)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//!
//! This module models AVR silicon-facing concepts such as memory-mapped
//! registers, GPIO ports and pins, serial peripherals, and concrete devices.
//! Board-specific wiring and connector names belong under
//! [`crate::sys::hw::mcu::board`].
//

crate::mods_in! {
    mod_ atmega328p;
    mod pin;
    mod port;
    mod register;
    mod_ timer;
    mod usart;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            atmega328p::_all::Atmega328p,
            pin::AvrPin,
            port::AvrPort,
            register::AvrReg8,
            timer::_all::AvrTimer0,
            usart::AvrUsart,
        };
    }
}
