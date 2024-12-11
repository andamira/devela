// devela::data::bit
//
//! Bit-focused types and traits.
//

mod r#trait; // BitOps

#[cfg(_bit_·)]
crate::items! {
    mod field; // bitfield
    mod wrapper; // Bitwise
}

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::r#trait::*;
        #[cfg(_bit_·)]
        pub use super::{field::all::*, wrapper::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
