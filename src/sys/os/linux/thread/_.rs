// devela/src/sys/os/linux/thread/_.rs
//
//! Linux-specific extensions to [`std::thread`].
//

crate::mods_in! {
    #[cfg(feature = "time")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "time")))]
    mod_ time; // LinuxClock, LinuxInstant, LinuxTime, LinuxTimespec
}
crate::mods_out! { // _mods
    _mods {
        #[cfg(feature = "time")]
        pub use super::time::_all::*;
    }
}
