//
#![doc = crate::_DOC_DEVICE!()] // public
#![doc = crate::_doc!(modules: crate; device: display)] // …
#![doc = crate::_doc!(flat:"device")]
#![doc = crate::_doc!(hr)]
//!
//! A device may be built into a board or connected externally; its driver
//! belongs here when its protocol and behavior are independent of a particular
//! microcontroller.
//!
//! Physical transport is kept separate where practical, so the same device
//! driver can operate through different MCU- or board-specific interfaces.
//

crate::mods_in! {
    // #[cfg_attr(not(nightly_doc), cfg(feature = "audio"))]
    // pub mod_ audio;   // Audio converters, amplifiers and interface devices
    // #[cfg_attr(not(nightly_doc), cfg(feature = "comm"))]
    // pub mod_ comm;    // Communication transceivers, modems, and interface devices
    #[cfg_attr(not(nightly_doc), cfg(feature = "display"))]
    pub mod_ display; // Display controllers and display-device drivers
    // #[cfg_attr(not(nightly_doc), cfg(feature = "input"))]
    // pub mod_ input;   // Input controllers and human-interface devices
    // #[cfg_attr(not(nightly_doc), cfg(feature = "motion"))]
    // pub mod_ motion;  // Motor, actuator, and motion-control devices
    // #[cfg_attr(not(nightly_doc), cfg(feature = "power"))]
    // pub mod_ power;   // Power-management, conversion, and control devices
    // #[cfg_attr(not(nightly_doc), cfg(feature = "sensor"))]
    // pub mod_ sensor;  // Sensors and measurement devices
    // #[cfg_attr(not(nightly_doc), cfg(feature = "vision"))]
    // pub mod_ vision;  // Cameras, imagers, and vision-oriented devices
}
crate::mods_out! { // _pub_mods, _reexports
    _pub_mods {
        // #[cfg_attr(not(nightly_doc), cfg(feature = "audio"))]
        // pub use super::audio::_all::*;
        // #[cfg_attr(not(nightly_doc), cfg(feature = "comm"))]
        // pub use super::comm::_all::*;
        #[cfg_attr(not(nightly_doc), cfg(feature = "display"))]
        pub use super::display::_all::*;
        // #[cfg_attr(not(nightly_doc), cfg(feature = "input"))]
        // pub use super::input::_all::*;
        // #[cfg_attr(not(nightly_doc), cfg(feature = "motion"))]
        // pub use super::motion::_all::*;
        // #[cfg_attr(not(nightly_doc), cfg(feature = "power"))]
        // pub use super::power::_all::*;
        // #[cfg_attr(not(nightly_doc), cfg(feature = "sensor"))]
        // pub use super::sensor::_all::*;
        // #[cfg_attr(not(nightly_doc), cfg(feature = "vision"))]
        // pub use super::vision::_all::*;
    }
    _reexports {
        #[doc(inline)] #[cfg(feature = "ssd13xx")]
        pub use super::display::Ssd13xx;
    }
}
