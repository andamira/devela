// devela/src/sys/fs/path/_.rs
//
#![doc = crate::_DOC_SYS_FS_PATH!()] // private
#![doc = crate::_doc!(modules: crate::sys::fs; path)]
#![doc = crate::_doc!(extends: path)]
//

crate::mods_in! {
    #[cfg(feature = "std")]
    mod _reexport_std;

    #[cfg(feature = "std")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    mod ext; // PathExt

    #[cfg(all(feature = "std", not(miri)))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    mod fs_path; // FsPath
}
crate::mods_out! { // _mods, _reexports
    _mods {
        #[cfg(feature = "std")]
        pub use super::ext::*;
        #[cfg(all(feature = "std", not(miri)))]
        pub use super::fs_path::*;
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
