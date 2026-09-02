// devela/src/lang/hum/i18n/_.rs
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_LANG_HUM_I18N!()] // public
#![doc = crate::_doc!(modules: crate::lang::hum; i18n)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//! Utilities for translating and adapting software to different languages and regions.
//

crate::mods_in! {
    // mod_ data; // registries, tables, normative datasets
    mod_ locale; // locale matching & fallback
    mod_ msg; // message selection languages
    mod_ select; // selection logic (plural, gender, variants, fallback)
    mod_ tag; // language tags
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // data::_all::*,
            locale::_all::*,
            msg::_all::*,
            select::_all::*,
            tag::_all::*,
        };
    }
}
