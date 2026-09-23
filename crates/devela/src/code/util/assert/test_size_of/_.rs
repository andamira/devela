//
//!
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // test_size_of!
    mod _internal; // __test_size_of!, __test_size_of_report
}
crate::mods_out! { // _mods
    _mods {
        #[doc(inline)]
        pub use super::{
            define::test_size_of,
        };
    }
    _hidden {
        pub use super::{
            _internal::{__test_size_of, __test_size_of_report},
        };
    }
}
