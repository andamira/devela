// devela::num::int::wrapper
//
//! Integer wrapper struct.
//

mod namespace; // Int

#[cfg(_int_·)]
crate::items! {
    mod impl_base;
    mod impl_combinatorics;
    mod impl_core;
    mod impl_div;
    mod impl_factors;
    mod impl_modulo;
    mod impl_prime;
    mod impl_root;
}

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::namespace::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
