// devela/src/code/util/assert/mod.rs
//
#![doc = crate::_DOC_CODE_UTIL_ASSERT!()] // public
#![doc = crate::_doc!(modules: crate::code::util; token)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(hr)]
//

mod _reexport_core;

mod dynamic; // assert_eq_all, assert_approx_eq_all
mod r#static; // const_assert!
mod test_size_of; // test_size_of! (__test_size_of_report)

crate::structural_mods! { // _mods, _reexports, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            dynamic::*,
            r#static::_all::*,
            test_size_of::test_size_of,
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
    _hidden {
        pub use super::{
            test_size_of::_hidden::*,
        };
    }
}
