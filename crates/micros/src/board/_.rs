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
//! | Board                       | MCU            | MHz | Display      | Host / serial          | Onboard I/O       |
//! | --------------------------- | -------------- | --: | ------------ | ---------------------- | ----------------- |
//! | [`Arduino Diecimila`]       | [`ATmega168`]  |  16 | —            | USB–UART               | D13 LED           |
//! | [`Arduino Due`]             | [`SAM3X8E`]    |  84 | —            | USB–UART, native USB   | D13/L LED         |
//! | [`Arduino Mega 2560`]       | [`ATmega2560`] |  16 | —            | USB–UART, USART1–3     | D13 LED           |
//! | [`Arduino Nano`]            | [`ATmega328p`] |  16 | —            | USB–UART               | D13 LED           |
//! | [`Lilygo T-Deck`]           | [`ESP32-S3`]   | 240 | 320×240 LCD  | ◇USB Serial/JTAG       | ◇keyboard, ◇trackball, ◇touch, ◇TF |
//! | [`Lilygo T-Display-S3`]     | [`ESP32-S3`]   | 240 | 170×320 LCD  | USB Serial/JTAG        | buttons           |
//! | [`Lilygo T-Watch-S3`]       | [`ESP32-S3`]   | 240 | 240×240 LCD  | USB Serial/JTAG        | ◇touch, ◇accel, ◇audio, ◇LoRa |
//! | [`ESP32-C3-OLED-0.42`]      | [`ESP32-C3`]   | 160 | [72×40 OLED] | USB Serial/JTAG, UART0 | GPIO8 LED         |
//! | [`ESP32-C6-Touch-LCD-1.47`] | [`ESP32-C6`]   | 160 | 172×320 LCD  | ◇USB, ◇UART0           | ◇Touch, ◇IMU, ◇TF |
//!
//! ```txt
//! ◇  hardware capability not yet exposed by devela
//! —  not present / not applicable
//! ```
//!
//! [`Arduino Diecimila`]: crate::BoardArduinoDiecimila
//! [`Arduino Due`]: crate::BoardArduinoDue
//! [`Arduino Mega 2560`]: crate::BoardArduinoMega2560
//! [`Arduino Nano`]: crate::BoardArduinoNano
//! [`Lilygo T-Deck`]: crate::BoardLilygoTDeckS3
//! [`Lilygo T-Display-S3`]: crate::BoardLilygoTDisplayS3
//! [`Lilygo T-Watch-S3`]: crate::BoardLilygoTWatchS3
//! [`ESP32-C3-OLED-0.42`]: crate::BoardSuperMiniOled042
//! [`ESP32-C6-Touch-LCD-1.47`]: crate::BoardWaveshareC6TouchLcd147
//!
//! [`ATmega168`]: crate::McuAtmega168
//! [`ATmega2560`]: crate::McuAtmega2560
//! [`ATmega328p`]: crate::McuAtmega328p
//! [`ESP32-C3`]: crate::McuEsp32C3
//! [`ESP32-C6`]: crate::McuEsp32C6
//! [`ESP32-S3`]: crate::McuEsp32S3
//! [`SAM3X8E`]: crate::McuSam3x8e
//!
//! [72×40 OLED]: crate::Ssd13xx#associatedconstant.OLED_72X40
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
        #[doc(inline)] #[cfg(feature = "arduino_due")]
        pub use super::sam::_all::BoardArduinoDue;
        #[doc(inline)] #[cfg(feature = "arduino_mega2560")]
        pub use super::avr::_all::BoardArduinoMega2560;
        #[doc(inline)] #[cfg(feature = "arduino_nano")]
        pub use super::avr::_all::BoardArduinoNano;
        #[doc(inline)] #[cfg(feature = "lilygo_t_deck_s3")]
        pub use super::esp32::_all::BoardLilygoTDeckS3;
        #[doc(inline)] #[cfg(feature = "lilygo_t_display_s3")]
        pub use super::esp32::_all::BoardLilygoTDisplayS3;
        #[doc(inline)] #[cfg(feature = "lilygo_t_watch_s3")]
        pub use super::esp32::_all::BoardLilygoTWatchS3;
        #[doc(inline)] #[cfg(feature = "supermini_oled042")]
        pub use super::esp32::_all::BoardSuperMiniOled042;
        #[doc(inline)] #[cfg(feature = "waveshare_c6_touch_lcd147")]
        pub use super::esp32::_all::BoardWaveshareC6TouchLcd147;
    }
}
