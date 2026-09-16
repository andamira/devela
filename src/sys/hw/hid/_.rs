//
//! Human interface devices.
//

crate::mods_in! {
    mod_ evdev; // WIP
    // mod_ gamepad;
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            evdev::_all::*,
            // gamepad::_all::*,
        };
    }
}
