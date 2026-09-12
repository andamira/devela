// devela/sys/hw/mcu/avr/_.rs
//
#![doc = crate::_DOC_SYS_HW_MCU_AVR!()] // public
#![doc = crate::_doc!(modules: crate::sys::hw::mcu; avr: timer)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//!
//! AVR microcontrollers.
//!
//! This module models silicon-facing AVR concepts such as memory-mapped registers,
//! GPIO ports and pins, serial peripherals, and concrete devices. Board-specific
//! wiring and connector names belong under [`sys::hw::mcu::board`][super::board].
//!
//! # AVR model
//!
//! AVR is an 8-bit architecture: its general-purpose working registers and
//! natural integer datapath are primarily 8 bits wide. This is a natural
//! operation width, not a limit on Rust value sizes. Wider integers occupy
//! multiple bytes and may require multiple registers, instructions, or helper
//! routines.
//!
//! Rust's `avr-none` target uses 16-bit pointers, so `usize`, `isize`, and
//! ordinary thin pointers are 16 bits wide. Fixed-width integers retain their
//! usual Rust sizes: `u16` is two bytes, `u32` four bytes, and so on.
//!
//! The practical constraints on wider values are storage and operation cost,
//! rather than representability. Multi-byte accesses also should not be assumed
//! atomic, which matters especially when data is shared with interrupt handlers.
//!
//! AVR separates program and data memory spaces, and peripheral layouts vary
//! between AVR families. The abstractions here therefore distinguish general
//! AVR concepts from the layout of a particular device.
//!
//! # Timers
//!
//! AVR timer/counters combine clocked counters with compare, capture,
//! event, interrupt, and waveform-generation facilities.
//!
//! See the [`timer`] module for the timer model and its terminology.
//!
//! # Interrupt vocabulary
//!
//! - **interrupt source** — a hardware condition capable of requesting service.
//! - **interrupt flag** — records that the corresponding event occurred.
//! - **interrupt enable** — allows a particular source to request an interrupt.
//! - **global interrupt enable** — CPU-wide gate for interrupt handling.
//! - **vector** — the entry point associated with an interrupt source.
//! - **ISR** — *interrupt service routine*, the function executed for a vector.
//!
//! A peripheral interrupt normally requires both its local enable and the CPU's
//! global interrupt enable. Event flags may still be set while interrupts are disabled.
//

crate::mods_in! {
        mod atmega328p;
        mod pin;
        mod port;
        mod register;
    pub mod_ timer;
        mod usart;
}
crate::mods_out! { // _mods, _pub_mods, _reexports
    _mods {
        pub use super::{
            atmega328p::McuAtmega328p,
            pin::AvrPin,
            port::AvrPort,
            register::AvrReg8,
            usart::AvrUsart,
        };
    }
    _pub_mods {
        pub use super::{
            timer::_all::{AvrTimer0, AvrTimer1},
        };
    }
    _reexports {
        #[doc(inline)]
        pub use super::{
            timer::_all::{AvrTimer0, AvrTimer1},
        };
    }
}
