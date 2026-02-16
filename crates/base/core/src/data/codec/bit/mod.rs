// devela_base_core::data::codec::bit
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

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // budget::*,
            field::_all::*,
            // manifest::*,
            // recipe::*,
            // view::*,
        };
    }
}
