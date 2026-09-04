// devela/src/data/topol/ord/_.rs
//
#![doc = crate::_DOC_DATA_TOPOL_ORD!()] // public
#![doc = crate::_doc!(modules: crate::data::topol; ord)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//!
//! Ordered topology describes precedence, succession, and composition
//! independently of storage and geometric position.
//

crate::mods_in! {
    mod concat; // Concat
}
crate::mods_out! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            concat::Concat,
        };
    }
}
