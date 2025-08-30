// devela::sys::fs
//
#![doc = crate::_DOC_SYS_FS!()]
//!
#![doc = crate::_doc!(extends: fs, path)]
#![doc = crate::_doc!(newline)]
//

crate::mod_path!(std _s "../../../libs/base_std/src/sys/fs/reexports.rs");

mod path; // ExtPath, Path*, sys::path::*

#[cfg(feature = "std")]
crate::items! {
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    mod namespace; // Fs
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    mod fs_path; // FsPath
}

// WIP ZONE
// mod ext; // ExtFile

crate::structural_mods! { // _mods
    _mods {
        pub use super::{path::_all::*};

        #[cfg(feature = "std")]
        pub use super::{_s::*, namespace::*, fs_path::*};
        // WIPZONE
        // pub use super::ext::*;
    }
}
