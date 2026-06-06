// devela::code::util::asserts
//
//! Assertion utilities.
//

mod dynamic; // assert_eq_all, assert_approx_eq_all
mod r#static; // const_assert!
mod test_size_of; // test_size_of! (__test_size_of_report)

crate::structural_mods! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            dynamic::*,
            r#static::_all::*,
            test_size_of::test_size_of,
        };
    }
    _hidden {
        pub use super::{
            test_size_of::__test_size_of_report,
        };
    }
}
