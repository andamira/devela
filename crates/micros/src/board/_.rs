//
//! Microcontroller development boards.
//!
//! Board definitions compose a concrete MCU with board-level facts such as
//! clocking, connector pin mappings, onboard devices, and fixed wiring.
//!
//! Silicon-specific peripherals and register layouts belong to the MCU module.
//!
//! # Boards
//!
//! | Board                   | MCU           |       Clock | Onboard I/O | Display    | Host / serial          |
//! | ----------------------- | ------------- | ----------: | ----------- | ---------- | ---------------------- |
//! | `BoardArduinoNano`      | McuAtmega328p |      16 MHz | D13 LED     | —          | USB–UART               |
//! | `BoardSuperMiniOled042` | McuEsp32C3    | 40 MHz XTAL | GPIO8 LED   | 72×40 OLED | USB Serial/JTAG, UART0 |
//!
// ◇  hardware capability not yet exposed by devela
//! ```txt
//! —  not present / not applicable
//! ```
//

crate::mods_in! {
    #[cfg(feature = "arduino")]
    mod_ arduino;
    #[cfg(feature = "supermini_oled042")]
    mod_ generic;

    // mod_ waveshare;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "arduino_nano")]
        pub use super::arduino::_all::BoardArduinoNano;
        #[cfg(feature = "supermini_oled042")]
        pub use super::generic::_all::BoardSuperMiniOled042;

        // pub use super::waveshare::_all::BoardWaveshareC6TouchLcd147;
    }
}
