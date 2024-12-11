// devela::data::collections::array::d1
//
//! 1-dimensional array
//

mod impl_traits;
mod methods;

mod definitions; // Array

#[cfg(feature = "unsafe_array")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_array")))]
mod uninit; // ArrayUninit

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::definitions::*;
        #[cfg(feature = "unsafe_array")]
        pub use super::uninit::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
