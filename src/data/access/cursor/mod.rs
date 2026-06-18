// devela/src/data/access/cursor/mod.rs
//
#![doc = crate::_DOC_DATA_ACCESS_CURSOR!()] // private
#![doc = crate::_doc!(modules: crate::data::access; cursor)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod byte; // ByteCursor, ByteCursorError
// mod traits;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            byte::_all::*,
            // traits::*,
        };
    }
}
