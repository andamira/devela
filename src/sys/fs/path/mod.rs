// devela::sys::fs::path
//
#![doc = crate::_DOC_SYS_FS_PATH!()]
//!
#![doc = crate::_doc!(extends: path)]
#![doc = crate::_doc!(newline)]
//

mod reexports;

#[cfg(feature = "std")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
mod ext; // ExtPath

crate::structural_mods! { // _mods
    _mods {
        pub use super::reexports::*;

        #[cfg(feature = "std")]
        pub use super::ext::*;
    }
}
