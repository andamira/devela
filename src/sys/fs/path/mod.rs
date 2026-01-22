// devela::sys::fs::path
//
#![doc = crate::_DOC_SYS_FS_PATH!()]
//!
#![doc = crate::_doc!(extends: path)]
#![doc = crate::_doc!(newline)]
//

#[cfg(feature = "std")]
mod _reexport_std; // SYMLINK to /src/base/std/src/sys/fs/path/_reexport.rs

#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod ext; // PathExt

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[cfg(feature = "std")]
        pub use super::ext::*;
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
