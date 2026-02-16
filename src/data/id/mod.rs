// devela::data::id
//
#![doc = crate::_DOC_DATA_ID!()] // public
#![doc = crate::_doc!(modules: crate::data; id: key)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
#![doc = crate::_QUO_DATA_ID!()]
//

mod uid; // IdPin

pub mod key; //

crate::structural_mods! { // _mods, _pub_mods, _reexports
    _mods {
        pub use super::{
            uid::_all::*,
        };
    }
    _pub_mods {
        pub use super::{
            key::_all::*,
        };
    }
    _reexports {
        // handle
        #[doc(inline)]
        pub use devela_base_core::data::id::define_handle;
        #[cfg(feature = "_docs_examples")]
        pub use devela_base_core::data::id::HandleExample;
    }
}
