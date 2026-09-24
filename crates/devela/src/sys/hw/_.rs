//
#![doc = crate::_DOC_SYS_HW!()] // public
#![doc = crate::_doc!(modules: crate::sys; hw: pin)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//!
//! Hardware-facing interfaces and device foundations.
//!
//! This module describes physical hardware and the low-level mechanisms used
//! to interact with it, from microcontroller peripherals and signal lines to
//! buses, sensors, storage, and host-connected devices.
//!
//! Hardware-specific facts belong here when they describe the machine itself,
//! independently of a hosted operating system or higher-level interface.
//! Processor architecture belongs under [`sys::arch`](crate::sys::arch),
//! while hosted environments belong under [`sys::os`](crate::sys::os).
//

crate::mods_in! {
    // pub mod_ block; // Block storage
    // pub mod_ capture; // Image/surface capture hardware
        #[cfg(feature = "hw")]
        mod cmd_data;
    // pub mod_ hid; // Human interface devices WIP Evdev*
    // pub mod_ link; // Communication links
    #[cfg(feature = "hw")]
    pub mod_ pin; // Pin-level hardware interfaces and signal buses.
    // pub mod_ sensor; // Measurement sensors
    // pub mod_ usb; // USB bus/devices
}
crate::mods_out! { // _mods, _pub_mods
    _mods {
        #[cfg(feature = "hw")]
        pub use super::{
            cmd_data::CmdDataWrite,
        };
    }
    _pub_mods {
        // pub use super::{
        //     // block::_all::*,
        //     // capture::_all::*,
        //     // hid::_all::*,
        //     // link::_all::*,
        //     // sensor::_all::*,
        //     // usb::_all::*,
        // };
        #[cfg(feature = "hw")]
        pub use super::{
            pin::_all::*,
        };
    }
}
