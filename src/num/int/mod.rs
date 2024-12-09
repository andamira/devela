// devela::num::int
//
//! Integer functionality.
//

mod alias; // [i|u]size_[down|up]
mod fns; // prime_number_teorem
mod gcd; // GcdReturn

#[cfg(_int_·)]
crate::items! {
    mod num_trait; // NumInt, NumRefInt
    mod wrapper; // Int
    mod divisor;
}

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline { #[allow(unused_imports, reason = "fns")]
        pub use super::{alias::*, fns::*, gcd::*};
        #[cfg(_int_·)]
        pub use super::{divisor::*, num_trait::*, wrapper::all::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
