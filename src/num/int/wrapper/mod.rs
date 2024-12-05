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

// structural access
crate::items! {
    mod doc_inline {
        pub use super::namespace::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(crate) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
