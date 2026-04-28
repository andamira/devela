// devela::data::codec::bit
//
#![doc = crate::_DOC_DATA_CODEC_BIT!()] // private
#![doc = crate::_doc!(modules: crate::data::codec; bit)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

// mod budget; // BitBudget
mod field; // bitfield!
// mod manifest; // BitManifest
// mod recipe; // BitRecipe
// mod view; // BitView // WIP
mod set; // set!

crate::structural_mods! { // _mods
    _mods {
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
