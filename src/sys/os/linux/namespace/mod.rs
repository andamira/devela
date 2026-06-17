// devela/src/sys/os/linux/namespace/mod.rs
//
//! Defines the [`Linux`] namespace.
//

mod definition; // Linux

// impls
#[crate::macro_apply(crate::_unsafe_syscall_not_miri)]
crate::items! {
    mod r#in;
    mod out;
    mod file;
    #[cfg(feature = "term")]
    mod term; // (LinuxTermModeGuard)
    mod thread; // thread, time
    mod signal;
    mod random;
    // syscalls are implemented in ../syscalls/
}

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            definition::Linux,
        };
    }
    _crate_internals {
        #[cfg(feature = "term")]
        pub(crate) use super::term::LinuxTermModeGuard;
    }
}
