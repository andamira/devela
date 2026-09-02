// devela/src/lang/hum/_.rs
//
#![doc = crate::_DOC_LANG_HUM!()] // public
#![doc = crate::_doc!(modules: crate::lang; hum: art, i18n, nat)]
// denote, form, prag, syntax, vocal, write
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//! Structure and use of language as produced, interpreted, and shared by humans.
//

crate::mods_in! {
    pub mod_ art; // Artificial human languages
    // pub mod code; // Cultural/semiotic code systems
        mod_ denote; // Denotation, reference, semantics
        mod_ form; // Word formation (morphology)
    pub mod_ i18n; // Internationalization and localization support
    pub mod_ nat; // Natural human languages
        mod_ prag; // Pragmatics (meaning in use)
        mod_ rethoric; // Human-language realization of rhetorical structures
        mod_ syntax; // Sentence structure
        mod_ vocal; // Sound systems (phonetics, phonology, prosody)
        mod_ write; // Writing systems, ortopgraphy
}
crate::mods_out! { // _mods, _pub_mods
    _mods {
        pub use super::{
            denote::_all::*,
            form::_all::*,
            prag::_all::*,
            rethoric::_all::*,
            syntax::_all::*,
            vocal::_all::*,
            write::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            art::_all::*,
            // code::_all::*,
            i18n::_all::*,
            nat::_all::*,
        };
    }
}
