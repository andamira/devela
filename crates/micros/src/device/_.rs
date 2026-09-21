//
#![doc = crate::_DOC_DEVICE!()] // public
#![doc = crate::_doc!(modules: crate; device: display)] // …
#![doc = crate::_doc!(flat:"device")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod_ audio;   // Audio converters, amplifiers and interface devices
    // mod_ comm;    // Communication transceivers, modems, and interface devices
    // mod_ display; // Display controllers and display-device drivers
    // mod_ input;   // Input controllers and human-interface devices
    // mod_ motion;  // Motor, actuator, and motion-control devices
    // mod_ power;   // Power-management, conversion, and control devices
    // mod_ sensor;  // Sensors and measurement devices
    // mod_ vision;  // Cameras, imagers, and vision-oriented devices
}
crate::mods_out! { // _mods
    _mods {
        // pub use super::{
        //     // audio::_all::*,
        //     // comm::_all::*,
        //     // display::_all::*,
        //     // input::_all::*,
        //     // motion::_all::*,
        //     // power::_all::*,
        //     // sensor::_all::*,
        //     // vision::_all::*,
        // };
    }
}
