// devela/sys/hw/mcu/board/_.rs
//
#![doc = crate::_DOC_SYS_HW_MCU_BOARD!()] // public
#![doc = crate::_doc!(modules: crate::sys::hw::mcu; board)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//!
//! Board definitions compose a concrete MCU with board-level facts such as
//! clocking, connector pin mappings, onboard devices, and fixed wiring.
//!
//! Silicon-specific peripherals and register layouts belong
//! to the MCU family modules, such as [`sys::hw::mcu::avr`][super::avr].
//!
//! # Boards
//!
//! | Board                   | MCU           |       Clock | Onboard I/O | Display    | Host / serial          |
//! | ----------------------- | ------------- | ----------: | ----------- | ---------- | ---------------------- |
//! | `BoardArduinoNano`      | McuAtmega328p |      16 MHz | D13 LED     | —          | USB–UART               |
//! | `BoardSuperMiniOled042` | McuEsp32C3    | 40 MHz XTAL | GPIO8 LED   | 72×40 OLED | USB Serial/JTAG, UART0 |
//!
//! ```txt
//! ◇  hardware capability not yet exposed by devela
//! —  not present / not applicable
//! ```
//

crate::mods_in! {
    mod_ arduino;
    mod_ generic;
    // mod_ waveshare;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            arduino::_all::BoardArduinoNano,
            generic::_all::BoardSuperMiniOled042,
            // waveshare::_all::BoardWaveshareC6TouchLcd147,
        };
    }
}
