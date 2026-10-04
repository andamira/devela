//
//! Defines the [`Linux`] namespace.
//

crate::mods_in! {
    mod define; // Linux

    /* impls (syscalls are implemented in ../syscalls) */
    #[crate::macro_apply(crate::_linux_syscall)]
    mod r#in;
    #[crate::macro_apply(crate::_linux_syscall)]
    mod out;
    #[crate::macro_apply(crate::_linux_syscall)]
    mod file;
    #[cfg(feature = "term")]
    #[crate::macro_apply(crate::_linux_syscall)]
    mod term; // (LinuxTermModeGuard)
    #[crate::macro_apply(crate::_linux_syscall)]
    mod thread; // thread, time
    #[crate::macro_apply(crate::_linux_syscall)]
    mod signal;
    #[crate::macro_apply(crate::_linux_syscall)]
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
