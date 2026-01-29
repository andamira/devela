// devela_base_alloc::num::dom
//
#![doc = crate::_DOC_NUM_DOM!()]
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
