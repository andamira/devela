// devela/src/text/translit/_.rs
//
#![doc = crate::_DOC_TEXT_TRANSLIT!()] // public, root
#![doc = crate::_doc!(modules: crate::text; translit)]
#![doc = crate::_doc!(flat:"text")]
#![doc = crate::_doc!(hr)]

crate::mods_in! {
    mod namespace; // Translit

    /* impls */
    #[cfg(feature = "translit")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "translit")))]
    mod_ ascii; // Unicode scalar/string → ASCII approximation
    // mod fallback; // replacement policy: '?', "", named, escaped
    // mod latin; // Greek/Cyrillic/etc. → Latin-ish forms
    // mod marks; // strip/keep/approximate diacritics
    // mod slug; // identifier/url/file-safe text simplification
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            namespace::*,
        };
    }
}
