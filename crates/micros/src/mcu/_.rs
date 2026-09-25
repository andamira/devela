//
#![doc = crate::_DOC_MCU!()] // public
#![doc = crate::_doc!(modules: crate; mcu: avr, esp32, sam)]
#![doc = crate::_doc!(flat:"mcu")]
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
//!
//! # Microcontrollers
//!
//! | MCU             | Core              | Memory                                 | GPIO | Timers              | Serial                       | Analog          | Radio        |
//! | --------------- | ----------------- | -------------------------------------- | ---: | ------------------- | ---------------------------- | --------------- | ------------ |
//! | `McuAtmega328p` | AVR 8-bit, 20 MHz | 32 KiB Flash, 2 KiB SRAM, 1 KiB EEPROM |   23 | T0/T1/T2            | USART0, ◇SPI, ◇TWI           | ◇10-bit ADC     | —            |
//! | `McuEsp32C3`    | RV32IMC, 160 MHz  | 384 KiB ROM, 400 KiB SRAM, ext. Flash  | ≤ 22 | ◇GPTimer, ◇SYSTIMER | UART0, I²C0, ◇UART1, ◇SPI    | ◇2x12-bit ADC   | ◇Wi-Fi, ◇BLE |
//! | `McuSam3x8e`    | Cortex-M3, 84 MHz | 512 KiB Flash, 96 KiB SRAM             |  103 | ◇TC0/TC1/TC2        | ◇UART, ◇USART0–3, ◇SPI, ◇TWI | ◇12-bit ADC/DAC | —            |
//!
//! ```txt
//! ◇  hardware capability not yet exposed by this crate
//! —  not present / not applicable
//! ```
//! The table is a compact orientation, not an exhaustive peripheral inventory.
//

crate::mods_in! {
    #[cfg_attr(not(nightly_doc), cfg(feature = "avr"))]
    pub mod_ avr;
    #[cfg_attr(not(nightly_doc), cfg(feature = "esp32"))]
    pub mod_ esp32;
    #[cfg_attr(not(nightly_doc), cfg(feature = "sam"))]
    pub mod_ sam;
}
crate::mods_out! { // _pub_mods, _reexports
    _pub_mods {
        #[cfg_attr(not(nightly_doc), cfg(feature = "avr"))]
        pub use super::avr::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "esp32"))]
        pub use super::esp32::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "sam"))]
        pub use super::sam::_all::*;
    }
    _reexports {
        #[doc(inline)]
        #[cfg(feature = "atmega328p")]
        pub use super::avr::McuAtmega328p;
        #[doc(inline)]
        #[cfg(feature = "esp32c3")]
        pub use super::esp32::McuEsp32C3;
        #[doc(inline)]
        #[cfg(feature = "sam3x8e")]
        pub use super::sam::McuSam3x8e;
    }
}
