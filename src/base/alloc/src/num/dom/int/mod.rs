// devela_base_alloc::num::dom::int
//
#![doc = crate::_DOC_NUM_DOM_INT!()] // public
#![doc = crate::_doc!(modules: crate::num::dom; int)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

#[cfg(feature = "int")]
mod wrapper; // Int alloc methods

crate::structural_mods! { // _mods
    _mods {
        #[cfg(feature = "int")]
        pub use super::wrapper::_all::*;
    }
}
