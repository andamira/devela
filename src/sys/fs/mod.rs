// devela::sys::fs
//
#![doc = crate::_DOC_SYS_FS!()]
//!
#![doc = crate::_doc!(extends: fs, path)]
#![doc = crate::_doc!(newline)]
//

#[cfg(feature = "std")]
mod _reexport_std; // SYMLINK to /src/base/std/src/sys/fs/_reexport.rs

mod path; // PathExt, Path*, sys::path::*

#[cfg(feature = "std")]
crate::items! {
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    mod namespace; // Fs
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    mod fs_path; // FsPath
}

// mod ext; // FileExt // WIP

crate::structural_mods! { // _mods
    _mods {
        pub use super::{path::_all::*};

        #[cfg(feature = "std")]
        pub use super::{namespace::*, fs_path::*};

        // pub use super::ext::*; // WIP
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
