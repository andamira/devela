// devela::sys::os::linux::time
//
//! Timekeeping ABI definitions.
//!
//! Provides Linux time-related structures and identifiers used to
//! query clocks and represent time values at the kernel ABI level.
//

mod clock; // LinuxClock
mod timespec; // LinuxTimespec

#[cfg(all(feature = "unsafe_syscall", not(miri)))]
mod instant; // LinuxInstant, LinuxTime

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            clock::*,
            timespec::*,
        };
        #[cfg(all(feature = "unsafe_syscall", not(miri)))]
        pub use super::instant::*;
    }
}
