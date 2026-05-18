// devela::sys::os::linux::process
//
//! Linux-specific extensions to [`std::process`].
//

crate::_unsafe_syscall_not_miri! {
mod entry; } // linux_entry!
mod signal; // LinuxSigaction, LinuxSiginfo, LinuxSigset, LINUX_[SIGACTION|SIGNAL]

crate::structural_mods! { // _mods
    _mods {
        #[crate::macro_apply(crate::_unsafe_syscall_not_miri)]
        pub use super::entry::*;
        pub use super::signal::*;
    }
}
