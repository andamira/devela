// devela/src/sys/hw/mod.rs
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_SYS_HW!()] // public
#![doc = crate::_doc!(modules: crate::sys; hw)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(hr)]
//

// mod block; // Block storage
// mod capture; // Image/surface capture hardware
// mod hid; // Human interface devices WIP Evdev*
// mod link; // Communication links
// mod sensor; // Measurement sensors
// mod usb; // USB bus/devices

crate::structural_mods! { // _mods
    _mods {
        // pub use super::{
        //     block::_all::*,
        //     capture::_all::*,
        //     hid::_all::*,
        //     link::_all::*,
        //     sensor::_all::*,
        //     usb::_all::*,
        // };
    }
}
