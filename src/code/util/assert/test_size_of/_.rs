// devela/src/code/util/assert/test_size_of/_.rs
//
//!
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    mod define; // test_size_of!
    mod report; // (__test_size_of_report)
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
            report::__test_size_of_report,
        };
    }
}
