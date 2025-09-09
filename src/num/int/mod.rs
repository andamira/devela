// devela::num::int
//
#![doc = crate::_DOC_NUM_INT!()]
//

pub(crate) mod shared_docs; // FORMULA_*!()

mod fns; // prime_number_teorem
mod gcd; // GcdReturn

crate::items! {
    mod divisor;
    mod num_trait; // NumInt, NumRefInt
    mod wrapper; // Int
}

crate::structural_mods! { // _mods
    _mods {
        pub use super::{fns::*, gcd::*};

        pub use super::{divisor::*, num_trait::*, wrapper::_all::*};

        #[doc(inline)]
        pub use devela_base_core::{isize_down, isize_up, usize_down, usize_up};
    }
}
