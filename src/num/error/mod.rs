// devela::num::error
//
#![doc = crate::_DOC_NUM_ERROR!()] // public
#![doc = crate::_doc!(modules: crate::num; error)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

mod highest; // RETHINK
mod definitions;

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            highest::*,
            definitions::*,
        };
    }
}
