// devela_base_core::sys::os::fd
//
#![doc = crate::_DOC_SYS_OS_FD!()]
//

use crate::c_int;

/// Raw file descriptors.
pub type FdRaw = c_int;

crate::structural_mods! { //_mods
    _mods {
        pub use super::FdRaw;
    }
}
