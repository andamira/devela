// devela_base_std::phys::time
//
#![doc = crate::_DOC_PHYS_TIME!()] // public
#![doc = crate::_doc!(modules: crate::phys; time)]
#![doc = crate::_doc!(flat:"phys")]
#![doc = crate::_doc!(extends: time)]
//

mod _reexport; // SYMLINK from /src/phys/time/_reexport_std.rs

mod error;

pub mod source;

crate::structural_mods! { // _mods, _pub_mods, _reexports
    _mods {
        pub use super::error::*;
    }
    _pub_mods {
        pub use super::source::*;
    }
    _reexports {
        pub use super::_reexport::*;
    }
}
