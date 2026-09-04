// devela/src/sys/mem/size/_.rs
//
//! Memory size functionality.
//

crate::mods_in! {
    mod_ bit; // BitSized
    mod byte; // ByteSized
    mod_ expr; // size_of_expr!
    // #[cfg(feature = "alloc")]
    // mod_ heap; // WIP
}
crate::mods_out! { // _mods, _reexports, _hidden
    _mods {
        #[doc(inline)]
        pub use super::{
            bit::_all::BitSized,
            byte::ByteSized,
            expr::size_of_expr,
        };
    }
    _reexports {
        #[doc(inline)]
        pub use crate::Sized;
    }
    _hidden {
        pub use super::{
            expr::_hidden::*,
        };
    }
}
