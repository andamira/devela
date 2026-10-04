//
//! Timekeeping ABI definitions.
//!
//! Provides Linux time-related structures and identifiers used to
//! query clocks and represent time values at the kernel ABI level.
//

crate::mods_in! {
    mod clock; // LinuxClock
    mod timespec; // LinuxTimespec

    #[crate::macro_apply(crate::_linux_syscall)]
    mod instant; // LinuxInstant, LinuxTime
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            clock::*,
            timespec::*,
        };
        #[crate::macro_apply(crate::_linux_syscall)]
        pub use super::instant::*;
    }
}
