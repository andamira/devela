// devela_base_core::data::bit
//
#![doc = crate::_DOC_DATA_BIT!()]
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
