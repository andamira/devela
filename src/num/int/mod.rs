// devela::num::int
//
//! Integer functionality.
//

mod alias; // [i|u]size_[down|up]
mod fns; // prime_number_teorem
mod gcd; // GcdReturn

#[cfg(_int_·)]
crate::items! {
    mod divisor;
    mod num_trait; // NumInt, NumRefInt
    mod shared_docs; // FORMULA_*!()
    mod wrapper; // Int
}

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        #[allow(unused, reason = "fns")]
        pub use super::{alias::*, fns::*, gcd::*};

        #[cfg(_int_·)]
        pub use super::{divisor::*, num_trait::*, wrapper::all::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::alias::*;
    }
}
