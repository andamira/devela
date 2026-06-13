// devela/src/sys/os/linux/process/mod.rs
//
//! Linux-specific extensions to [`std::process`].
//

crate::_unsafe_syscall_not_miri! {
mod entry; } // linux_entry!
mod signal; // LinuxSigaction, LinuxSiginfo, LinuxSigset, LINUX_[SIGACTION|SIGNAL]

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        #[crate::macro_apply(crate::_unsafe_syscall_not_miri)]
        pub use super::entry::*;
        pub use super::signal::_all::*;
    }
    _crate_internals {
        pub(crate) use super::signal::_crate_internals::*;
    }
}
