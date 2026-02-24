// devela::lang
//
#![doc = crate::_DOC_LANG!()] // public, root
#![doc = crate::_DOC_LANG_MODULES!()]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(extends: ffi)]
//
// LINKS
// - https://en.wikipedia.org/wiki/Language | https://es.wikipedia.org/wiki/Lenguaje
//
// safety
#![cfg_attr(feature = "safe_lang", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_LANG_MODULES =
    crate::_doc!(modules: crate; lang: prog); // disc, gram, hum, repr, sem
}

mod disc; //  discourse & expression WIP
mod gram; // grammar machinery WIP
mod hum; // human languages WIP
mod repr; // representation languages WIP
mod sem; // semantic relations WIP

pub mod prog; // programming languages

crate::structural_mods! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            disc::_all::*,
            // gram::_all::*,
            hum::_all::*,
            repr::_all::*,
            sem::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            prog::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_LANG_MODULES;
        pub(crate) use super::prog::_crate_internals::*;
    }
}
