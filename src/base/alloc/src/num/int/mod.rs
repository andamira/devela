// devela_base_alloc::num::int
//
#![doc = crate::_DOC_NUM_INT!()]
//

#[cfg(feature = "int")]
mod wrapper; // Int alloc methods

crate::structural_mods! { // _mods
    _mods {
        #[cfg(feature = "int")]
        pub use super::wrapper::_all::*;
    }
}
