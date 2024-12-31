// devela::num::int
//
//! Integer functionality.
//

mod alias; // [i|u]size_[down|up]
mod fns; // prime_number_teorem
mod gcd; // GcdReturn

#[cfg(_int··)]
crate::items! {
    mod divisor;
    mod num_trait; // NumInt, NumRefInt
    mod shared_docs; // FORMULA_*!()
    mod wrapper; // Int
}

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods {
        #[allow(unused, reason = "fns")]
        pub use super::{alias::*, fns::*, gcd::*};

        #[cfg(_int··)]
        pub use super::{divisor::*, num_trait::*, wrapper::_all::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::alias::*;
    }
}
