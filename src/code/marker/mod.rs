// devela::code::marker
//
//! Marker types, traits and macros.
//!
#![doc = crate::doc_!(extends: marker)]
//

mod reexports; // core::marker re-exports
mod type_marker; // zero-cost generic type markers
mod type_resource; // zero-cost type-safe resource markers

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{reexports::*, type_marker::*, type_resource::*};
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{reexports::*, type_marker::*, type_resource::*};
    }
}
