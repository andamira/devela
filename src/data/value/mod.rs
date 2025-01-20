// devela::data::value
//
//! Enumerated data values and types, classified by size.
//

mod macros; // internal macros

mod build;
mod traits; // DataValue(Copy), DataType(Copy), DataRaw(Copy)

crate::items! { // structural access: _mods, _internals, _all
    #[allow(unused)]
    pub use {_mods::*, _internals::*};

    mod _mods { #![allow(unused)]
        pub use super::{build::*, traits::*};
    }
    pub(super) mod _internals { #![allow(unused)]
        pub(crate) use super::macros::_internals::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
