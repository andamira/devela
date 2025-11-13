// devela::data::list::array::d1
//
//! 1-dimensional array
//

mod impl_traits;
mod methods;

mod definitions; // Array

#[cfg(all(not(feature = "safe_data"), feature = "unsafe_array"))]
mod uninit; // ArrayUninit

crate::structural_mods! { // _mods
    _mods {
        pub use super::definitions::*;
        #[cfg(all(not(feature = "safe_data"), feature = "unsafe_array"))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_array")))]
        pub use super::uninit::*;
    }
}
