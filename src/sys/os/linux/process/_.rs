// devela/src/sys/os/linux/process/_.rs
//
//! Linux-specific extensions to [`std::process`].
//

crate::mods_in! {
    #[cfg(all(feature = "unsafe_syscall", not(miri)))]
    mod entry; // linux_entry!
    mod_ signal; // LinuxSigaction, LinuxSiginfo, LinuxSigset, (LINUX_[SIGACTION|SIGNAL])
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        #[crate::macro_apply(crate::_unsafe_syscall_not_miri)]
        pub use super::entry::linux_entry;
        pub use super::signal::_all::*;
    }
    _crate_internals {
        pub(crate) use super::signal::_crate_internals::*;
    }
}
