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

crate::structural_mods! { // _mods
    _mods {
        pub use super::{fns::*, gcd::*, reexports::*};

        #[cfg(_int··)]
        pub use super::{divisor::*, num_trait::*, wrapper::_all::*};
    }
}
