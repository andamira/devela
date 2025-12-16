// devela_base_core::sys::os::fd
//
//! Unix-like file descriptors.
//

use crate::c_int;

/// Raw file descriptors.
pub type FdRaw = c_int;

crate::structural_mods! { //_mods
    _mods {
        pub use super::FdRaw;
    }
}
