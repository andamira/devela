// devela::data::id
//
#![doc = crate::_DOC_DATA_ID!()] // public
#![doc = crate::_doc!(modules: crate::data; id)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
#![doc = crate::_QUO_DATA_ID!()]
//

mod handle; // handle!, HandleExample
mod uid; // IdPin

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            handle::*,
            uid::_all::*,
        };
    }
}
