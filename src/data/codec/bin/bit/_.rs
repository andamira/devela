// devela/src/data/codec/bin/bit/_.rs
//
#![doc = crate::_DOC_DATA_CODEC_BIN_BIT!()] // public
#![doc = crate::_doc!(modules: crate::data::codec::bin; bit)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//
//! > If a named item is a thing that can be present, use `set!`.
//! > If a named item is a slot that can hold a value, use `bitfield!`.
//

crate::mods_in! {
    // mod budget; // BitBudget
    mod_ enumset; // enumset!
    mod_ field; // bitfield!
    // mod manifest; // BitManifest
    // mod recipe; // BitRecipe
    // mod view; // BitView // WIP
    mod_ set; // set!
}
crate::mods_out! { // _mods, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            // budget::*,
            enumset::_all::*,
            field::bitfield,
            // manifest::*,
            // recipe::*,
            // view::*,
            set::set,
        };

        #[cfg(feature = "_docs_examples")]
        pub use super::field::BitfieldExample;
    }
    _hidden {
        pub use super::enumset::_hidden::*;
    }
}
