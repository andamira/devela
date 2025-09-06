// devela_base_core::num::int
//
#![doc = crate::_DOC_NUM_INT!()]
//

mod alias; // [i|u]size_[down|up]

crate::structural_mods! { // _mods
    _mods {
        pub use super::alias::*;
    }
}
