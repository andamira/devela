//
//! Linux-specific extensions to [`std::process`].
//

crate::mods_in! {
    #[crate::macro_apply(crate::_linux_syscall)]
    mod entry; // linux_entry!
    mod_ signal; // LinuxSigaction, LinuxSiginfo, LinuxSigset, (LINUX_[SIGACTION|SIGNAL])
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        #[crate::macro_apply(crate::_linux_syscall)]
        pub use super::entry::linux_entry;
        pub use super::signal::_all::*;
    }
    _crate_internals {
        pub(crate) use super::signal::_crate_internals::*;
    }
}
