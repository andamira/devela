// devela/src/code/source/mod.rs
//
#![doc = crate::_DOC_CODE_SOURCE!()] // public
#![doc = crate::_doc!(modules: crate::code; source)]
#![doc = crate::_doc!(flat: "code")]
#![doc = crate::_doc!(hr)]
//

mod _reexport_core;

mod include; // include_from!, mod_from!, mod_path!
mod location; // CodeLocation
mod span; // CodeSpan
mod version; // Version, VersionFull

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            include::{include_from, mod_from, mod_path},
            location::CodeLocation,
            span::CodeSpan,
            version::{Version, VersionFull},
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
