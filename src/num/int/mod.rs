// devela::num::int
//
#![doc = crate::_DOC_NUM_INT!()]
//

mod fns; // prime_number_teorem

mod num_trait; // NumInt, NumRefInt

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            fns::*,
            num_trait::*,
        };
        #[doc(inline)]
        pub use devela_base_core::{
            isize_down, isize_up, usize_down, usize_up,
            Divisor,
            GcdReturn,
        };

        #[doc(inline)]
        pub use devela_base_num::Int;

        #[doc(inline)]
        #[cfg(feature = "alloc")]
        pub use devela_base_alloc::IntAlloc;
    }
}
