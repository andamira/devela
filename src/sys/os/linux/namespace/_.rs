// devela/src/sys/os/linux/namespace/_.rs
//
//! Defines the [`Linux`] namespace.
//

crate::mods_in! {
    mod define; // Linux

    /* impls (syscalls are implemented in ../syscalls) */
    #[cfg(all(feature = "unsafe_syscall", not(miri)))]
    mod r#in;
    #[cfg(all(feature = "unsafe_syscall", not(miri)))]
    mod out;
    #[cfg(all(feature = "unsafe_syscall", not(miri)))]
    mod file;
    #[cfg(all(feature = "unsafe_syscall", not(miri)))]
    #[cfg(feature = "term")]
    mod term; // (LinuxTermModeGuard)
    #[cfg(all(feature = "unsafe_syscall", not(miri)))]
    mod thread; // thread, time
    #[cfg(all(feature = "unsafe_syscall", not(miri)))]
    mod signal;
    #[cfg(all(feature = "unsafe_syscall", not(miri)))]
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
        #[crate::macro_apply(crate::_unsafe_syscall_not_miri)]
        pub(crate) use super::term::LinuxTermModeGuard;
    }
}
