// devela_base_core::sys::os::fd
//
#![doc = crate::_DOC_SYS_OS_FD!()] // public
#![doc = crate::_doc!(modules: crate::sys::os; fd)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: os)]
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
