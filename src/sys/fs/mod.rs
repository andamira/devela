// devela::sys::fs
//
#![doc = crate::_DOC_SYS_FS!()]
//!
#![doc = crate::_doc!(extends: fs, path)]
#![doc = crate::_doc!(newline)]
//

mod path; // ExtPath, Path*, sys::path::*
mod reexports; // sys::fs::*

#[cfg(feature = "std")]
crate::items! {
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    mod namespace; // Fs
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    mod fs_path; // FsPath
}

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{path::_all::*, reexports::*};

        #[cfg(feature = "std")]
        pub use super::{namespace::*, fs_path::*};
        // WIPZONE
        // pub use super::ext::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
// WIP ZONE
// mod ext; // ExtFile
