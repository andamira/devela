// devela/src/sys/fs/path/mod.rs
//
#![doc = crate::_DOC_SYS_FS_PATH!()] // private
#![doc = crate::_doc!(modules: crate::sys::fs; path)]
#![doc = crate::_doc!(extends: path)]
//

#[cfg(feature = "std")]
mod _reexport_std;

#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod ext; // PathExt

#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod fs_path; // FsPath

crate::structural_mods! { // _mods, _reexports
    _mods {
        #[cfg(feature = "std")]
        pub use super::{
            ext::*,
            fs_path::*,
        };
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
