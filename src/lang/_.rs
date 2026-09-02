// devela/src/lang/_.rs
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
    crate::_doc!(modules: crate; lang: prog, sem); // disc, gram, hum, repr
}

crate::mods_in! {
        mod_ disc; //  Discourse & expression WIP
        mod_ gram; // Grammar machinery WIP
        mod_ hum; // Human languages WIP
    pub mod_ prog; // Programming languages
        mod_ repr; // Representation languages WIP
    pub mod_ sem; // semantic relations
}
crate::mods_out! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            disc::_all::*,
            gram::_all::*,
            hum::_all::*,
            repr::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            prog::_all::*,
            sem::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_LANG_MODULES;
        pub(crate) use super::prog::_crate_internals::*;
    }
}
