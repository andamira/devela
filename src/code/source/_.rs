// devela/src/code/source/_.rs
//
#![doc = crate::_DOC_CODE_SOURCE!()] // public
#![doc = crate::_doc!(modules: crate::code; source)]
#![doc = crate::_doc!(flat: "code")]
#![doc = crate::_doc!(hr)]
//!
//! Source provenance is kept separate from the semantics of the code itself.
//! [`CodeLocation`] and [`CodeSpan`] identify where code originates,
//! [`Version`] describes revision state, and the inclusion macros connect source
//! files and modules at compile time.
//!
//! Token manipulation and generated Rust structure belong to
//! [`util::token`][crate::code::util::token] and
//! [`util::synth`][crate::code::util::synth], respectively.
//

crate::mods_in! {
    mod _reexport_core;

    mod include; // include_from!, mod_from!, mod_path!
    mod location; // CodeLocation
    mod span; // CodeSpan
    mod version; // Version, VersionFull
}
crate::mods_out! { // _mods, _reexports
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
