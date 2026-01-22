// devela_base_core::sys::os::fd
//
#![doc = crate::_DOC_SYS_OS_FD!()]
//

use crate::c_int;

#[doc = crate::_tags!(fs uid)]
/// Raw file descriptors.
#[doc = crate::_doc_location!("sys/os/fd")]
pub type FdRaw = c_int;

crate::structural_mods! { //_mods
    _mods {
        #[doc(inline)]
        pub use super::FdRaw;
    }
}
