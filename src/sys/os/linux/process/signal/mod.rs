// devela/src/sys/os/linux/process/signal/mod.rs
//
//! Signal handling ABI definitions.
//!
//! Provides Linux signal-related structures, constants, and control
//! interfaces used to configure signal delivery, handlers, and masks
//! at the process and thread level.
//

mod _raw; // (LINUX_SIGACTION, LINUX_SIGNAL)

mod action; // LinuxSigaction
mod flags; // LinuxSigactionFlags
mod handler; // (LinuxSigactionHandler), LINUX_SIG[INFO]_HANDLERS
mod info; // LinuxSiginfo, (LinuxSigval)
mod set; // LinuxSignal, LinuxSignalSet, LinuxSigset

#[crate::macro_apply(crate::_unsafe_syscall_not_miri)]
crate::items! {
    mod restorer;
}

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            action::*,
            flags::*,
            info::*,
            set::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            _raw::_crate_internals::*,
            handler::*,
        };
    }
}
