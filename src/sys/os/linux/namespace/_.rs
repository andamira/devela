// devela/src/sys/os/linux/namespace/_.rs
//
//! Defines the [`Linux`] namespace.
//

crate::mods_in! {
    mod define; // Linux

    /* impls (syscalls are implemented in ../syscalls) */
    #[cfg(all(feature = "unsafe_syscall", not(miri), linux_syscall_target))] // WAIT:1.99:apply
    mod r#in;
    #[cfg(all(feature = "unsafe_syscall", not(miri), linux_syscall_target))] // WAIT:1.99:apply
    mod out;
    #[cfg(all(feature = "unsafe_syscall", not(miri), linux_syscall_target))] // WAIT:1.99:apply
    mod file;
    #[cfg(feature = "term")]
    #[cfg(all(feature = "unsafe_syscall", not(miri), linux_syscall_target))] // WAIT:1.99:apply
    mod term; // (LinuxTermModeGuard)
    #[cfg(all(feature = "unsafe_syscall", not(miri), linux_syscall_target))] // WAIT:1.99:apply
    mod thread; // thread, time
    #[cfg(all(feature = "unsafe_syscall", not(miri), linux_syscall_target))] // WAIT:1.99:apply
    mod signal;
    #[cfg(all(feature = "unsafe_syscall", not(miri), linux_syscall_target))] // WAIT:1.99:apply
    mod random;
}
crate::mods_out! { // _mods, _crate_internals
    _mods {
        pub use super::{
            define::Linux,
        };
    }
    _crate_internals {
        #[cfg(feature = "term")]
        #[crate::macro_apply(crate::_linux_syscall)]
        pub(crate) use super::term::LinuxTermModeGuard;
    }
}
