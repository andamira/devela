// devela::num::int
//
//! Integer functionality.
//

mod fns; // prime_number_teorem
mod gcd; // GcdReturn
mod num_trait; // NumInt, NumRefInt
mod wrapper; // Int

#[cfg(_int_·)]
mod divisor;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{fns::*, gcd::*, num_trait::*, wrapper::all::*};
        #[cfg(_int_·)]
        pub use super::divisor::*;
    }
    pub(crate) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
