// devela::data::codec::bin::bit
//
#![doc = crate::_DOC_DATA_CODEC_BIN_BIT!()] // private
#![doc = crate::_doc!(modules: crate::data::codec::bin; bit)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//
//! > If a named item is a thing that can be present, use `set!`.
//! > If a named item is a slot that can hold a value, use `bitfield!`.
//

// mod budget; // BitBudget
mod field; // bitfield!
// mod manifest; // BitManifest
// mod recipe; // BitRecipe
// mod view; // BitView // WIP
mod set; // set!

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            // budget::*,
            field::bitfield,
            // manifest::*,
            // recipe::*,
            // view::*,
            set::set,
        };

        #[cfg(feature = "_docs_examples")]
        pub use super::field::BitfieldExample;
    }
}
