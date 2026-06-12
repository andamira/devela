// devela::sys::fs
//
#![doc = crate::_DOC_SYS_FS!()] // public
#![doc = crate::_doc!(modules: crate::sys; fs)]
#![doc = crate::_doc!(flat:"sys")]
#![doc = crate::_doc!(extends: fs, path)]
//

#[cfg(feature = "std")]
mod _reexport_std;

// pub mod app; // WIP
pub mod path; // PathExt, Path*, sys::path::*

#[cfg(feature = "std")]
crate::items! {
    #[cfg_attr(nightly_doc, doc(cfg(feature = "std")))]
    mod namespace; // Fs
}

// mod ext; // FileExt // WIP

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        #[cfg(feature = "std")]
        pub use super::{
            namespace::*,
        };
        // pub use super::ext::*; // WIP
    }
    _pub_mods {
        pub use super::{
            // app::all_::*,
            path::_all::*,
        };
    }
    _reexports {
        #[cfg(feature = "std")]
        pub use super::_reexport_std::*;
    }
}
