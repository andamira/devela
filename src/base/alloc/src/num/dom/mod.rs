// devela_base_alloc::num::dom
//
#![doc = crate::_DOC_NUM_DOM!()]
//

mod int; // Int alloc methods

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            int::_all::*,
        };
    }
}
