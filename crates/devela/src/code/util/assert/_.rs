//
#![doc = crate::_DOC_CODE_UTIL_ASSERT!()] // public
#![doc = crate::_doc!(modules: crate::code::util; token)]
#![doc = crate::_doc!(flat:"code")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    mod dynamic; // assert_eq_all, assert_approx_eq_all
    mod_ r#static; // const_assert!
    mod_ test_size_of; // test_size_of! (__test_size_of_report)
}
crate::mods_out! { // _mods, _reexports, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            dynamic::{assert_approx_eq_all, assert_eq_all},
            r#static::_all::*,
            test_size_of::_all::test_size_of,
        };
    }
    _reexports {
        pub use crate::boot::rust::code::util_assert::*;
    }
    _hidden {
        pub use super::{
            test_size_of::_hidden::*,
        };
    }
}
