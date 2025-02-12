// devela::num::ord
//
//! Comparing and ordering values.
// #![doc = crate::doc_!(extends: cmp)]
// #![doc = crate::doc_!(modules: crate::num; cmp)]
// #![doc = crate::doc_!(newline)]
//

mod compare; // `Compare`
mod reexports;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{compare::*, reexports::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{reexports::*, compare::*};
    }
}
