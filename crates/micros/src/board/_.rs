//
#![doc = crate::_DOC_BOARD!()] // public
#![doc = crate::_doc!(modules: crate; board: avr, esp32, sam)]
#![doc = crate::_doc!(flat:"board")]
#![doc = crate::_doc!(hr)]
//!
//! Board definitions compose a concrete MCU with board-level facts such as
//! clocking, connector pin mappings, onboard devices, and fixed wiring.
//!
//! Silicon-specific peripherals and register layouts belong to the MCU module.
//!
//! # Boards
//!
//! | Board                   | MCU              |       Clock | Onboard I/O | Display    | Host / serial          |
//! | ----------------------- | ---------------- | ----------: | ----------- | ---------- | ---------------------- |
//! | `BoardArduinoDue`       | McuSam3x8e       |      84 MHz | D13/L LED   | —          | USB–UART, native USB   |
//! | `BoardArduinoMega2560`  | McuAtmega2560    |      16 MHz | D13 LED     | —          | USB–UART, USART1–3     |
//! | `BoardArduinoNano`      | McuAtmega328p    |      16 MHz | D13 LED     | —          | USB–UART               |
//! | `BoardSuperMiniOled042` | McuEsp32C3       | 40 MHz XTAL | GPIO8 LED   | 72×40 OLED | USB Serial/JTAG, UART0 |
//!
// ◇  hardware capability not yet exposed by devela
//! ```txt
//! —  not present / not applicable
//! ```
//

crate::mods_in! {
    #[cfg_attr(not(nightly_doc), cfg(feature = "board_avr"))]
    pub mod_ avr;
    #[cfg_attr(not(nightly_doc), cfg(feature = "board_esp32"))]
    pub mod_ esp32;
    #[cfg_attr(not(nightly_doc), cfg(feature = "board_sam"))]
    pub mod_ sam;
}
crate::mods_out! { // _pub_mods, _reexports
    _pub_mods {
        #[cfg_attr(not(nightly_doc), cfg(feature = "board_avr"))]
        pub use super::avr::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "board_esp32"))]
        pub use super::esp32::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "board_sam"))]
        pub use super::sam::_all::*;
    }
    _reexports {
        #[doc(inline)]
        #[cfg(feature = "arduino_due")]
        pub use super::sam::_all::BoardArduinoDue;
        #[cfg(feature = "arduino_mega2560")]
        pub use super::avr::_all::BoardArduinoMega2560;
        #[cfg(feature = "arduino_nano")]
        pub use super::avr::_all::BoardArduinoNano;
        #[doc(inline)]
        #[cfg(feature = "supermini_oled042")]
        pub use super::esp32::_all::BoardSuperMiniOled042;
        // #[doc(inline)]
        // #[cfg(feature = "waveshare_c6_touch_lcd147")]
        // pub use super::esp32::_all::BoardWaveshareC6TouchLcd147;
    }
}
