// devela/src/sys/os/linux/thread/mod.rs
//
//! Linux-specific extensions to [`std::thread`].
//

#[cfg(feature = "time")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
mod time; // LinuxClock, LinuxInstant, LinuxTime, LinuxTimespec

crate::structural_mods! { // _mods
    _mods {
        #[cfg(feature = "time")]
        pub use super::time::*;
    }
}
