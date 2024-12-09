// devela::data::bit::field
//
//!
//

mod bitfield;

#[cfg(all(test, feature = "_bit_u8"))]
mod tests;

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::bitfield::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
