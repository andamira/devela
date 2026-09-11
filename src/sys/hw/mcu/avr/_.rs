// devela/sys/hw/mcu/avr/_.rs
//
#![doc = crate::_DOC_SYS_HW_MCU_AVR!()] // public
#![doc = crate::_doc!(modules: crate::sys::hw::mcu; avr)]
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
//! # Timer/counter vocabulary
//!
//! - **clock** — the source of timing ticks.
//! - **prescaler** — divides the source clock before it reaches the counter.
//! - **tick** — one counter-clock event.
//! - **counter** — the value advanced by timer ticks.
//! - **BOTTOM** — the lowest value in the counting sequence, normally zero.
//! - **MAX** — the largest value representable by the physical counter.
//! - **TOP** — the highest value reached in the current counting mode.
//! - **compare** — detects when the counter equals a configured compare value.
//! - **compare match** — the event produced by a successful comparison.
//! - **event flag** — a register bit recording that a hardware event occurred.
//! - **overflow** — the event associated with the end of a counting cycle;
//!   its exact timing depends on the counting mode.
//! - **CTC** — *Clear Timer on Compare Match*: a compare value supplies TOP,
//!   and the timer starts again from BOTTOM after reaching it.
//! - **PWM** — uses periodic counting and compare points to control output duty.
//!
//! A hardware event and an interrupt are separate concepts. An event can
//! set a flag that software polls, request an interrupt when enabled,
//! affect a hardware output, or some combination of these.
//

crate::mods_in! {
    mod atmega328p;
    mod pin;
    mod port;
    mod register;
    mod_ timer;
    mod usart;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            atmega328p::Atmega328p,
            pin::AvrPin,
            port::AvrPort,
            register::AvrReg8,
            timer::_all::AvrTimer0,
            usart::AvrUsart,
        };
    }
}
