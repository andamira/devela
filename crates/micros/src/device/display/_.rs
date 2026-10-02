//
#![doc = crate::_DOC_DEVICE_DISPLAY!()] // public
#![doc = crate::_doc!(modules: crate::device; display)]
#![doc = crate::_doc!(flat:"device")]
#![doc = crate::_doc!(hr)]
//!
//! [`Jd9853`] models color LCD geometry and controller RAM access; its current
//! profile targets the Waveshare 172×320 panel.
//!
//! [`Ssd13xx`] supports page-packed monochrome OLEDs, currently including the
//! 72×40 profile used by the ESP32-C3 OLED board.
//!
//! Both operate through [`CmdDataWrite`][crate::CmdDataWrite],
//! leaving the physical I²C or SPI transport to the caller.
//!
//! [`Tm1638`] covers the TM1638 LED/display and key-scan controller together
//! with its display-RAM frame model and [`Tm1638Bus`] transaction interface.
//

crate::mods_in! {
    #[cfg(feature = "jd9853")]
    mod jd9853;
    #[cfg(feature = "ssd13xx")]
    mod ssd13xx;
    #[cfg(feature = "tm1638")]
    mod_ tm1638;
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "jd9853")]
        pub use super::jd9853::Jd9853;
        #[cfg(feature = "ssd13xx")]
        pub use super::ssd13xx::Ssd13xx;
        #[cfg(feature = "tm1638")]
        pub use super::tm1638::_all::*;
    }
}
