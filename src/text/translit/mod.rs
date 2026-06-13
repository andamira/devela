// devela/src/text/translit/mod.rs
//
#![doc = crate::_DOC_TEXT_TRANSLIT!()] // public, root
#![doc = crate::_doc!(modules: crate::text; translit)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(hr)]

#[cfg(feature = "translit")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "translit")))]
mod ascii; // Unicode scalar/string → ASCII approximation
// mod fallback; // replacement policy: '?', "", named, escaped
// mod latin; // Greek/Cyrillic/etc. → Latin-ish forms
// mod marks; // strip/keep/approximate diacritics
mod namespace; // Translit
// mod slug; // identifier/url/file-safe text simplification

crate::structural_mods! { // _mods
    _mods {
        pub use super::namespace::*;
        #[cfg(feature = "translit")]
        pub use super::{
            ascii::*
        };
    }
}
