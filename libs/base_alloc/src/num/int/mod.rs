// devela_base_alloc::num::int
//
#![doc = crate::_DOC_NUM_INT!()]
//

mod wrapper; // Int alloc methods

crate::structural_mods! { // _mods
    _mods {
        pub use super::wrapper::_all::*;
    }
}
