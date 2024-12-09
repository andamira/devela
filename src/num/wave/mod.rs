// devela::num::wave
//
//! Wavelets.
//

#[cfg(test)]
mod tests;

mod shared; // WaveletHaar, WaveletUnitRole

#[cfg(feature = "alloc")]
#[cfg(any(feature = "std", feature = "_float_f64"))] // NOTE: not documented
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
mod alloc;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::shared::*;
        #[cfg(feature = "alloc")]
        #[cfg(any(feature = "std", feature = "_float_f64"))]
        pub use super::alloc::*;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
