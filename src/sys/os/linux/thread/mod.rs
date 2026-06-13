// devela/src/sys/os/linux/thread/mod.rs
//
//! Linux-specific extensions to [`std::thread`].
//

mod time; // LinuxClock, LinuxInstant, LinuxTime, LinuxTimespec

crate::structural_mods! { // _mods
    _mods {
        pub use super::time::*;
    }
}
