// devela/src/sys/os/linux/io/file/state/mod.rs
//
//!
//

mod _raw; // (LINUX_S_IFMT)
mod stat; // LinuxFileType, LinuxStat

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            stat::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_raw::LINUX_S_IFMT;
    }
}
