// devela::sys::os::fd
//
#![doc = crate::_DOC_SYS_OS_FD!()] // public
#![doc = crate::_doc!(modules: crate::sys::os; fd)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: os)]
//

#[cfg(feature = "std")]
mod _reexport_std;

pub mod raw; // FdRaw

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[doc(inline)]
        pub use super::raw::FdRaw;
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
