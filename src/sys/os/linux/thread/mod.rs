// devela::sys::os::linux::thread
//
//! Linux-specific extensions to [`std::thread`].
//

mod time; // LinuxClock, LinuxInstant, LinuxTime, LinuxTimespec

crate::structural_mods! { // _mods
    _mods {
        pub use super::time::*;
    }
}
