// devela::data::collections::array::d2
//
//! 2-dimensional array
//

mod impl_traits;
mod methods;
#[cfg(test)]
mod tests;

mod definitions; // Array2d

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::definitions::*;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
