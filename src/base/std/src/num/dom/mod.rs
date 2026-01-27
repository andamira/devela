// devela_base_std::num::dom
//
#![doc = crate::_DOC_NUM_DOM!()]
//

mod real; // Real-valued numeric domains and representations.

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            real::_all::*,
        };
    }
}
