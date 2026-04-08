// devela::num::dom::int
//
#![doc = crate::_DOC_NUM_DOM_INT!()] // public
#![doc = crate::_doc!(modules: crate::num::dom; int)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//

mod fns; // prime_number_teorem

#[cfg(all(feature = "int", feature = "alloc"))]
mod wrapper_alloc; // IntAlloc TEMP

#[cfg(feature = "int")]
mod num_trait; // NumInt, NumRefInt

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            fns::*,
        };
        #[cfg(feature = "int")]
        pub use super::{
            num_trait::*,
        };
        #[cfg(all(feature = "int", feature = "alloc"))]
        pub use super::{
            wrapper_alloc::*,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use devela_base_core::num::dom::{ // int
            isize_down, isize_up, usize_down, usize_up,
            define_divisor,
            GcdReturn,
        };
        #[cfg(feature = "_docs_examples")]
        pub use devela_base_core::num::dom::int::{
            DivisorExample,
        };
        #[cfg(feature = "int")]
        pub use devela_base_core::num::dom::int::{
            Int,
        };
    }
}
