// devela::sys::os::linux::time
//
//! Timekeeping ABI definitions.
//!
//! Provides Linux time-related structures and identifiers used to
//! query clocks and represent time values at the kernel ABI level.
//

mod clock; // LinuxClock
mod timespec; // LinuxTimespec

#[crate::macro_apply(crate::_unsafe_syscall_not_miri)]
crate::items! {
    mod instant; // LinuxInstant, LinuxTime
}

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            clock::*,
            timespec::*,
        };
        #[crate::macro_apply(crate::_unsafe_syscall_not_miri)]
        pub use super::instant::*;
    }
}
