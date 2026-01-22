// devela::num::int
//
#![doc = crate::_DOC_NUM_INT!()]
//

mod fns; // prime_number_teorem

#[cfg(feature = "int")]
mod num_trait; // NumInt, NumRefInt

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            fns::*,
        };
        #[cfg(feature = "int")]
        pub use super::num_trait::*;

        #[doc(inline)]
        pub use devela_base_core::{
            isize_down, isize_up, usize_down, usize_up,
            define_divisor,
            GcdReturn,
        };
        #[cfg(feature = "_docs_min")]
        pub use devela_base_core::num::DivisorExample;
        #[cfg(feature = "int")]
        pub use devela_base_core::num::Int;

        #[cfg(feature = "alloc")]
        pub use devela_base_alloc::IntAlloc;
    }
}
