// devela_base_alloc::num::dom
//
#![doc = crate::_DOC_NUM_DOM!()] // public
#![doc = crate::_doc!(modules: crate::num; dom)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

pub mod int; // IntAlloc

crate::structural_mods! { // _pub_mods
    _pub_mods {
        #[doc(inline)]
        pub use super::{
            int::_all::*,
        };
    }
}
