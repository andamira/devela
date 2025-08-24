// devela::num::int
//
#![doc = crate::_DOC_NUM_INT!()]
//

pub(crate) mod shared_docs; // FORMULA_*!()

mod fns; // prime_number_teorem
mod gcd; // GcdReturn
mod reexports;

#[cfg(_int··)]
crate::items! {
    mod divisor;
    mod num_trait; // NumInt, NumRefInt
    mod wrapper; // Int
}

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub use super::{fns::*, gcd::*, reexports::*};

        #[cfg(_int··)]
        pub use super::{divisor::*, num_trait::*, wrapper::_all::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
