// devela/src/sys/hw/_.rs
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_SYS_HW!()] // public
#![doc = crate::_doc!(modules: crate::sys; hw)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod_ block; // Block storage
    // mod_ capture; // Image/surface capture hardware
    // mod_ hid; // Human interface devices WIP Evdev*
    // mod_ link; // Communication links
    // mod_ mcu; // Microcontrollers
    // mod_ pin; // Pin-level hardware interfaces
    // mod_ sensor; // Measurement sensors
    // mod_ usb; // USB bus/devices
}
crate::mods_out! { // _mods
    _mods {
        // pub use super::{
        //     block::_all::*,
        //     capture::_all::*,
        //     hid::_all::*,
        //     link::_all::*,
        //     mcu::_all::*,
        //     pin::_all::*,
        //     sensor::_all::*,
        //     usb::_all::*,
        // };
    }
}
