//
//! Formatting numbers.
//

crate::mods_in! {
    #[cfg(test)]
    mod _test;

    #[cfg(feature = "_docs_examples")]
    mod _example;

    // definitions
    mod define;
    mod_ impls;

    mod conf;
    mod group;
    mod shape;
}
crate::mods_out! { // _mods, _hidden
    _mods {
        pub use super::{
            conf::{FmtNumConf, FmtNumSign},
            define::fmt_num,
            group::FmtNumGroup,
            shape::FmtNumShape,
        };
        #[cfg(feature = "_docs_examples")]
        pub use super::_example::FmtNumExample;
    }
    _hidden {
        #[doc(hidden)]
        pub use super::{
            define::__fmt_num,
            impls::_hidden::*,
        };
    }
}
