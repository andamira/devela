// devela::code::any
//
//! Dynamic typing and reflection.
// #![doc = crate::doc_!(extends: any)]
// #![doc = crate::doc_!(modules: crate::code; any)]
// #![doc = crate::doc_!(newline)]
//

mod ext;
mod reexports;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{ext::*, reexports::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{ext::*, reexports::*};
    }
}
